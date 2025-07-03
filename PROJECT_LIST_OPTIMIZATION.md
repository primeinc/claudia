# Project List Performance Optimization

This document describes the optimizations implemented to make the Claudia projects list load instantly (under 500ms).

## Problem Statement

The project listing was slow because it:
1. Read every project directory sequentially
2. Made multiple file system calls per project
3. Read JSONL files to extract metadata
4. Had no caching mechanism

## Implemented Solutions

### 1. Caching Layer (`src-tauri/src/commands/projects_cache.rs`)

- **Memory Cache**: Projects are cached in memory with a 1-minute TTL
- **Smart Invalidation**: Cache is invalidated when new sessions are created
- **Incremental Updates**: Support for partial cache updates (future enhancement)

### 2. Parallel Processing

- **Rayon Integration**: Projects are processed in parallel using Rust's rayon library
- **Batch Operations**: File system operations are batched where possible
- **Optimized File Finding**: Uses native `find` command for faster file discovery

### 3. Lazy Loading Strategy

- **Basic Info First**: Load project ID and path immediately
- **Session Details on Demand**: Session count and details loaded when needed
- **Configurable Loading**: `should_load_session_details()` and `should_load_todo_data()` control what's loaded

### 4. Frontend Optimizations

- **Preloading Hook**: `useProjectsCache()` preloads the cache on app start
- **Performance Monitoring**: `performanceMonitor` tracks load times
- **Background Refresh**: Cache can be refreshed without blocking UI

## Usage

### Frontend

```typescript
import { useProjectsCache, refreshProjectsCache } from '@/hooks/useProjectsCache';
import { perfMonitor } from '@/utils/performanceMonitor';

// In your App component
function App() {
  useProjectsCache(); // Preload cache on app start
  
  // When loading projects
  const loadProjects = async () => {
    const projects = await perfMonitor.measure('Load Projects', async () => {
      return await invoke('list_projects');
    });
  };
  
  // Force refresh when needed
  const handleRefresh = async () => {
    await refreshProjectsCache();
  };
}
```

### Backend Commands

- `list_projects` - Get cached projects list (fast)
- `refresh_projects_cache` - Force refresh the cache
- `get_projects_cache_stats` - Get cache statistics
- `clear_projects_cache` - Clear the cache

## Performance Targets

- Initial load: < 500ms (achieved through caching)
- Subsequent loads: < 50ms (from memory cache)
- Cache refresh: < 2s (background operation)

## Cache Invalidation

The cache is automatically invalidated when:
1. A new Claude session is created
2. The cache TTL expires (1 minute)
3. Manual refresh is triggered

## Future Enhancements

1. **Persistent Cache**: Store cache in SQLite for faster cold starts
2. **Incremental Updates**: Only update changed projects
3. **WebSocket Updates**: Real-time updates when sessions change
4. **Compression**: Compress cached data for larger project lists

## Monitoring

Use the cache stats command to monitor performance:

```rust
let stats = get_projects_cache_stats();
// Returns: "Projects cache: 42 projects, 156 total sessions, last updated: 15 seconds ago, is stale: false"
```