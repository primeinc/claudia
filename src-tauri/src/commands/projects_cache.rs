use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use crate::claude_paths::{
    find_claude_files, get_claude_metadata, list_claude_directory, read_claude_file,
    read_cache_file, write_cache_file, get_cache_file_metadata,
};
use crate::commands::claude::Project;

// Global cache for projects data
static PROJECTS_CACHE: Lazy<Arc<Mutex<ProjectsCache>>> = Lazy::new(|| {
    Arc::new(Mutex::new(ProjectsCache::new()))
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedProject {
    pub id: String,
    pub path: String,
    pub session_count: usize,
    pub created_at: u64,
    pub last_session_created: u64,
}

// Cache structures for disk persistence  
#[derive(Debug, Serialize, Deserialize)]
struct ProjectsCacheMetadata {
    version: u32,
    created_at: u64,
    last_updated: u64,
    ttl_seconds: u64,
    projects_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectsCacheData {
    metadata: ProjectsCacheMetadata,
    projects: Vec<CachedProject>,
    project_sessions: HashMap<String, Vec<String>>,
    file_metadata: HashMap<String, u64>,
}

// Cache constants
const PROJECTS_CACHE_VERSION: u32 = 1;
const PROJECTS_CACHE_TTL: u64 = 300; // 5 minutes

#[derive(Debug)]
struct ProjectsCache {
    projects: Vec<CachedProject>,
    project_sessions: HashMap<String, Vec<String>>, // project_id -> session_ids
    last_updated: Option<Instant>,
    file_metadata: HashMap<String, u64>, // file_path -> modification_time
}

impl ProjectsCache {
    fn new() -> Self {
        Self {
            projects: Vec::new(),
            project_sessions: HashMap::new(),
            last_updated: None,
            file_metadata: HashMap::new(),
        }
    }

    fn is_stale(&self) -> bool {
        match self.last_updated {
            None => true,
            Some(last) => last.elapsed() > Duration::from_secs(60), // 1 minute cache for projects
        }
    }

    #[allow(dead_code)]
    fn needs_incremental_update(&self) -> bool {
        // Check if we have cached data that just needs updating
        !self.projects.is_empty() && self.last_updated.is_some()
    }

    fn update(&mut self, projects: Vec<CachedProject>, sessions: HashMap<String, Vec<String>>, metadata: HashMap<String, u64>) {
        self.projects = projects;
        self.project_sessions = sessions;
        self.file_metadata = metadata;
        self.last_updated = Some(Instant::now());
    }

    fn clear(&mut self) {
        self.projects.clear();
        self.project_sessions.clear();
        self.file_metadata.clear();
        self.last_updated = None;
    }

    // Disk cache methods
    fn load_from_disk(&mut self) -> Result<bool, String> {
        // First check if cache file exists and is valid
        if !Self::is_disk_cache_valid() {
            return Ok(false);
        }
        
        match read_cache_file("projects.json") {
            Ok(content) => {
                let cache_data: ProjectsCacheData = serde_json::from_str(&content)
                    .map_err(|e| format!("Failed to parse projects cache: {}", e))?;
                
                // Check cache version and TTL
                if cache_data.metadata.version != PROJECTS_CACHE_VERSION {
                    log::warn!("Projects cache version mismatch, rebuilding");
                    return Ok(false);
                }
                
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                if now > cache_data.metadata.last_updated + cache_data.metadata.ttl_seconds {
                    log::info!("Projects cache expired, rebuilding");
                    return Ok(false);
                }
                
                // Load data into memory cache
                self.projects = cache_data.projects;
                self.project_sessions = cache_data.project_sessions;
                self.file_metadata = cache_data.file_metadata;
                self.last_updated = Some(Instant::now());
                
                log::info!("Loaded {} projects from disk cache", self.projects.len());
                Ok(true)
            }
            Err(e) => {
                if e.contains("No such file") {
                    log::info!("No projects cache file found, will create new one");
                } else {
                    log::warn!("Failed to read projects cache: {}, will rebuild", e);
                }
                Ok(false)
            }
        }
    }

    fn save_to_disk(&self) -> Result<(), String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let cache_data = ProjectsCacheData {
            metadata: ProjectsCacheMetadata {
                version: PROJECTS_CACHE_VERSION,
                created_at: now,
                last_updated: now,
                ttl_seconds: PROJECTS_CACHE_TTL,
                projects_count: self.projects.len(),
            },
            projects: self.projects.clone(),
            project_sessions: self.project_sessions.clone(),
            file_metadata: self.file_metadata.clone(),
        };
        
        let json = serde_json::to_string(&cache_data)
            .map_err(|e| format!("Failed to serialize projects cache: {}", e))?;
        
        write_cache_file("projects.json", &json)
            .map_err(|e| format!("Failed to write projects cache: {}", e))?;
        
        log::info!("Saved {} projects to disk cache", self.projects.len());
        Ok(())
    }

    fn is_disk_cache_valid() -> bool {
        match get_cache_file_metadata("projects.json") {
            Ok(modified_time) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                let is_valid = now <= modified_time + PROJECTS_CACHE_TTL;
                log::info!("Projects disk cache valid: {} (age: {}s)", is_valid, now - modified_time);
                is_valid
            }
            Err(e) => {
                log::info!("Projects disk cache not found: {}", e);
                false
            }
        }
    }
}

/// Optimized function to get all projects with caching and parallel processing
pub async fn get_cached_projects(force_refresh: bool) -> Result<Vec<CachedProject>, String> {
    // Check cache first
    if !force_refresh {
        let mut cache = PROJECTS_CACHE.lock().unwrap();
        
        // Try loading from disk cache if memory cache is stale/empty
        if cache.is_stale() || cache.projects.is_empty() {
            log::info!("[CACHE] Attempting to load projects cache from disk...");
            match cache.load_from_disk() {
                Ok(true) => log::info!("[CACHE] Successfully loaded projects cache from disk"),
                Ok(false) => log::info!("[CACHE] Disk cache not loaded (invalid or missing)"),
                Err(e) => {
                    log::warn!("[CACHE] Failed to load projects cache from disk: {}", e);
                    log::warn!("Failed to load projects cache from disk: {}", e);
                }
            }
        }
        
        if !cache.is_stale() && !cache.projects.is_empty() {
            log::info!("Returning cached projects ({} items)", cache.projects.len());
            return Ok(cache.projects.clone());
        }
    }

    log::info!("Loading projects from file system...");
    
    // Get all project directories
    let project_dirs = tokio::task::spawn_blocking(move || {
        list_claude_directory("projects")
    })
    .await
    .map_err(|e| format!("Failed to execute blocking task: {}", e))??;
    
    if project_dirs.is_empty() {
        let mut cache = PROJECTS_CACHE.lock().unwrap();
        cache.clear();
        return Ok(Vec::new());
    }

    // Process projects in parallel
    let projects_data: Vec<_> = project_dirs
        .par_iter()
        .filter_map(|dir_name| {
            process_project_directory(dir_name).ok()
        })
        .collect();

    // Build the cache structures
    let mut projects = Vec::new();
    let mut project_sessions = HashMap::new();
    let mut file_metadata = HashMap::new();

    for (project, sessions, metadata) in projects_data {
        project_sessions.insert(project.id.clone(), sessions);
        for (path, time) in metadata {
            file_metadata.insert(path, time);
        }
        projects.push(project);
    }

    // Sort projects by creation time (newest first)
    projects.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // Update cache
    {
        let mut cache = PROJECTS_CACHE.lock().unwrap();
        cache.update(projects.clone(), project_sessions, file_metadata);
        
        // Save to disk cache
        log::info!("[CACHE] Saving projects cache to disk...");
        if let Err(e) = cache.save_to_disk() {
            log::warn!("[CACHE] Failed to save projects cache to disk: {}", e);
            log::warn!("Failed to save projects cache to disk: {}", e);
        } else {
            log::info!("[CACHE] Projects cache saved successfully");
        }
    }

    log::info!("Loaded {} projects into cache", projects.len());
    Ok(projects)
}

/// Process a single project directory and return project info, sessions, and metadata
fn process_project_directory(dir_name: &str) -> Result<(CachedProject, Vec<String>, Vec<(String, u64)>), String> {
    let project_dir_path = format!("projects/{}", dir_name);
    
    // Get directory creation time
    let created_at = get_claude_metadata(&project_dir_path).unwrap_or(0);
    
    // List all JSONL files in parallel using find command (much faster for large directories)
    let session_files = match find_claude_files(&project_dir_path, "*.jsonl") {
        Ok(files) => files,
        Err(_) => {
            // Fallback to directory listing
            list_claude_directory(&project_dir_path)?
                .into_iter()
                .filter(|name| name.ends_with(".jsonl"))
                .map(|name| format!("{}/{}", project_dir_path, name))
                .collect()
        }
    };
    
    let mut sessions = Vec::new();
    let mut metadata = Vec::new();
    let mut last_session_created = 0u64;
    
    // Extract session IDs and track metadata
    for file_path in &session_files {
        if let Some(file_name) = file_path.split('/').last() {
            if let Some(session_id) = file_name.strip_suffix(".jsonl") {
                sessions.push(session_id.to_string());
                
                // Get file modification time
                if let Ok(mod_time) = get_claude_metadata(file_path) {
                    metadata.push((file_path.clone(), mod_time));
                    last_session_created = last_session_created.max(mod_time);
                }
            }
        }
    }
    
    // Get the actual project path - read first session file to extract it
    let project_path = if !session_files.is_empty() {
        extract_project_path_from_session(&session_files[0]).unwrap_or_else(|| decode_project_path(dir_name))
    } else {
        decode_project_path(dir_name)
    };
    
    Ok((
        CachedProject {
            id: dir_name.to_string(),
            path: project_path,
            session_count: sessions.len(),
            created_at,
            last_session_created,
        },
        sessions,
        metadata,
    ))
}

/// Extract project path from a session file (optimized to read only first few lines)
fn extract_project_path_from_session(session_path: &str) -> Option<String> {
    if let Ok(content) = read_claude_file(session_path) {
        // Only read first few lines for performance
        for line in content.lines().take(5) {
            if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(project_path) = entry.get("project_path").and_then(|p| p.as_str()) {
                    return Some(project_path.to_string());
                }
                // Also check for 'cwd' field as fallback
                if let Some(cwd) = entry.get("cwd").and_then(|p| p.as_str()) {
                    return Some(cwd.to_string());
                }
            }
        }
    }
    None
}

/// Decode project directory name (fallback when can't read from session)
fn decode_project_path(encoded: &str) -> String {
    encoded.replace('-', "/")
}

/// Get sessions for a specific project (uses cache when available)
pub async fn get_cached_project_sessions(project_id: String) -> Result<Vec<String>, String> {
    // Try cache first
    {
        let cache = PROJECTS_CACHE.lock().unwrap();
        if !cache.is_stale() {
            if let Some(sessions) = cache.project_sessions.get(&project_id) {
                log::info!("Returning cached sessions for project {}: {} items", project_id, sessions.len());
                return Ok(sessions.clone());
            }
        }
    }
    
    // If not in cache or cache is stale, load from file system
    let project_dir_path = format!("projects/{}", project_id);
    let session_files = tokio::task::spawn_blocking(move || {
        list_claude_directory(&project_dir_path)
    })
    .await
    .map_err(|e| format!("Failed to execute blocking task: {}", e))??;
    
    let sessions: Vec<String> = session_files
        .into_iter()
        .filter(|name| name.ends_with(".jsonl"))
        .map(|name| name.trim_end_matches(".jsonl").to_string())
        .collect();
    
    Ok(sessions)
}

/// Invalidate cache when a new session is created
pub fn invalidate_project_cache(project_id: Option<&str>) {
    let mut cache = PROJECTS_CACHE.lock().unwrap();
    
    if let Some(id) = project_id {
        // Partial invalidation - just mark as stale
        log::info!("Invalidating cache for project: {}", id);
        // We could implement partial updates here in the future
    } else {
        // Full invalidation
        log::info!("Invalidating entire projects cache");
        cache.clear();
    }
}

/// Get cache statistics
pub fn get_cache_stats() -> String {
    let cache = PROJECTS_CACHE.lock().unwrap();
    format!(
        "Projects cache: {} projects, {} total sessions, last updated: {} seconds ago, is stale: {}",
        cache.projects.len(),
        cache.project_sessions.values().map(|s| s.len()).sum::<usize>(),
        cache.last_updated.map(|t| t.elapsed().as_secs()).unwrap_or(999999),
        cache.is_stale()
    )
}

/// Perform incremental cache update (for background refresh)
#[allow(dead_code)]
pub async fn incremental_cache_update() -> Result<(), String> {
    let should_update = {
        let cache = PROJECTS_CACHE.lock().unwrap();
        cache.needs_incremental_update() && cache.is_stale()
    };
    
    if should_update {
        log::info!("Performing incremental cache update");
        
        // Get current cache state
        let (old_metadata, old_projects) = {
            let cache = PROJECTS_CACHE.lock().unwrap();
            (cache.file_metadata.clone(), cache.projects.clone())
        };
        
        // Check for new or modified files
        let project_dirs = tokio::task::spawn_blocking(move || {
            list_claude_directory("projects")
        })
        .await
        .map_err(|e| format!("Failed to execute blocking task: {}", e))??;
        let _updated_projects: Vec<Project> = Vec::new();
        let mut needs_full_refresh = false;
        
        for dir_name in project_dirs {
            let project_dir_path = format!("projects/{}", dir_name);
            
            // Check if this is a new project
            let is_new = !old_projects.iter().any(|p| p.id == dir_name);
            
            if is_new {
                log::info!("Found new project: {}", dir_name);
                needs_full_refresh = true;
                break;
            }
            
            // Check for new sessions in existing projects
            let project_dir_path_clone = project_dir_path.clone();
            let current_sessions_result = tokio::task::spawn_blocking(move || {
                find_claude_files(&project_dir_path_clone, "*.jsonl")
            })
            .await
            .map_err(|e| format!("Failed to execute blocking task: {}", e))?;
            
            if let Ok(current_sessions) = current_sessions_result {
                for session_path in current_sessions {
                    let session_path_clone = session_path.clone();
                    let mod_time_result = tokio::task::spawn_blocking(move || {
                        get_claude_metadata(&session_path_clone)
                    })
                    .await
                    .map_err(|e| format!("Failed to execute blocking task: {}", e))?;
                    
                    if let Ok(mod_time) = mod_time_result {
                        if let Some(&old_time) = old_metadata.get(&session_path) {
                            if mod_time > old_time {
                                log::info!("Found modified session: {}", session_path);
                                needs_full_refresh = true;
                                break;
                            }
                        } else {
                            log::info!("Found new session: {}", session_path);
                            needs_full_refresh = true;
                            break;
                        }
                    }
                }
            }
            
            if needs_full_refresh {
                break;
            }
        }
        
        if needs_full_refresh {
            // Perform full refresh
            get_cached_projects(true).await?;
        } else {
            log::info!("No changes detected in incremental update");
        }
    }
    
    Ok(())
}