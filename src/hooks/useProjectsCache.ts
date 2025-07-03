import { invoke } from '@tauri-apps/api/core';
import { useEffect } from 'react';

/**
 * Hook to manage projects cache preloading
 */
export function useProjectsCache() {
  useEffect(() => {
    // Preload projects cache on app start
    const preloadCache = async () => {
      try {
        console.log('Preloading projects cache...');
        await invoke('list_projects');
        console.log('Projects cache loaded');
      } catch (error) {
        console.error('Failed to preload projects cache:', error);
      }
    };

    // Preload after a short delay to not block initial UI
    const timer = setTimeout(preloadCache, 500);

    return () => clearTimeout(timer);
  }, []);
}

/**
 * Force refresh the projects cache
 */
export async function refreshProjectsCache(): Promise<void> {
  try {
    await invoke('refresh_projects_cache');
  } catch (error) {
    console.error('Failed to refresh projects cache:', error);
    throw error;
  }
}

/**
 * Get projects cache statistics
 */
export async function getProjectsCacheStats(): Promise<string> {
  try {
    return await invoke('get_projects_cache_stats');
  } catch (error) {
    console.error('Failed to get cache stats:', error);
    throw error;
  }
}

/**
 * Clear the projects cache
 */
export async function clearProjectsCache(): Promise<void> {
  try {
    await invoke('clear_projects_cache');
  } catch (error) {
    console.error('Failed to clear cache:', error);
    throw error;
  }
}