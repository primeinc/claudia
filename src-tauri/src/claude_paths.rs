#[cfg(not(target_os = "windows"))]
use std::fs;
use std::path::PathBuf;
#[cfg(not(target_os = "windows"))]
use std::time::SystemTime;

#[cfg(target_os = "windows")]
use once_cell::sync::Lazy;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
use walkdir::WalkDir;
#[cfg(target_os = "windows")]
use glob::Pattern;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(target_os = "windows")]
static CLAUDE_UNC_PATH: Lazy<Result<PathBuf, String>> = Lazy::new(|| {
    // Run once at startup to get the UNC path to ~/.claude
    let output = std::process::Command::new("wsl")
        .args(&["--", "sh", "-c", "cd ~/.claude && wslpath -w ."])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to get Claude UNC path: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr)
            .replace('\r', "");
        return Err(format!("Failed to get Claude UNC path: {}", stderr.trim()));
    }
    
    let path_str = String::from_utf8_lossy(&output.stdout)
        .replace('\r', "")
        .trim()
        .to_string();
    
    log::info!("[UNC_PATH] wslpath returned: {}", path_str);
    
    Ok(PathBuf::from(path_str))
});

/// Get the full UNC path for a relative path within ~/.claude
#[cfg(target_os = "windows")]
fn get_claude_unc_path(relative_path: &str) -> Result<PathBuf, String> {
    let base = CLAUDE_UNC_PATH.as_ref().map_err(|e| e.clone())?;
    Ok(base.join(relative_path))
}

/// Gets the Claude configuration directory path
/// Returns ~/.claude on Unix-like systems
#[cfg(not(target_os = "windows"))]
pub fn get_claude_dir() -> Result<PathBuf, anyhow::Error> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))?;
    Ok(home.join(".claude"))
}

/// On Windows, we don't use this function directly since we access files via WSL
#[cfg(target_os = "windows")]
pub fn get_claude_dir() -> Result<PathBuf, anyhow::Error> {
    Err(anyhow::anyhow!("Direct path access not supported on Windows. Use read_claude_file/write_claude_file instead."))
}

/// Read a file from the Claude directory
#[cfg(target_os = "windows")]
pub fn read_claude_file(relative_path: &str) -> Result<String, String> {
    let file_path = get_claude_unc_path(relative_path)?;
    log::debug!("[READ_FILE] Attempting to read: {}", file_path.display());
    
    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            log::debug!("[READ_FILE] Successfully read {} bytes from {}", content.len(), relative_path);
            Ok(content)
        }
        Err(e) => {
            log::error!("[READ_FILE] Failed to read {}: {} (full path: {})", relative_path, e, file_path.display());
            Err(format!("Failed to read file {}: {}", relative_path, e))
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn read_claude_file(relative_path: &str) -> Result<String, String> {
    let claude_dir = get_claude_dir().map_err(|e| e.to_string())?;
    let file_path = claude_dir.join(relative_path);
    fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file {}: {}", relative_path, e))
}

/// Write a file to the Claude directory
#[cfg(target_os = "windows")]
pub fn write_claude_file(relative_path: &str, content: &str) -> Result<(), String> {
    let file_path = get_claude_unc_path(relative_path)?;
    
    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
    }
    
    std::fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write file {}: {}", relative_path, e))
}

#[cfg(not(target_os = "windows"))]
pub fn write_claude_file(relative_path: &str, content: &str) -> Result<(), String> {
    let claude_dir = get_claude_dir().map_err(|e| e.to_string())?;
    let file_path = claude_dir.join(relative_path);
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write file {}: {}", relative_path, e))
}

/// List files in a Claude directory
#[cfg(target_os = "windows")]
pub fn list_claude_directory(relative_path: &str) -> Result<Vec<String>, String> {
    let dir_path = get_claude_unc_path(relative_path)?;
    
    if !dir_path.exists() {
        return Ok(Vec::new());
    }
    
    let entries = std::fs::read_dir(&dir_path)
        .map_err(|e| format!("Failed to read directory {}: {}", relative_path, e))?;
    
    let mut names = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_string());
            }
        }
    }
    
    Ok(names)
}

#[cfg(not(target_os = "windows"))]
pub fn list_claude_directory(relative_path: &str) -> Result<Vec<String>, String> {
    let claude_dir = get_claude_dir().map_err(|e| e.to_string())?;
    let dir_path = claude_dir.join(relative_path);
    
    if !dir_path.exists() {
        return Ok(Vec::new());
    }
    
    let entries = fs::read_dir(&dir_path)
        .map_err(|e| format!("Failed to read directory {}: {}", relative_path, e))?;
    
    let mut names = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_string());
            }
        }
    }
    
    Ok(names)
}

/// Check if a file exists in the Claude directory
#[cfg(target_os = "windows")]
pub fn claude_file_exists(relative_path: &str) -> bool {
    get_claude_unc_path(relative_path)
        .ok()
        .map(|p| p.exists())
        .unwrap_or(false)
}

#[cfg(not(target_os = "windows"))]
pub fn claude_file_exists(relative_path: &str) -> bool {
    if let Ok(claude_dir) = get_claude_dir() {
        claude_dir.join(relative_path).exists()
    } else {
        false
    }
}

/// Get file/directory metadata (modification time)
#[cfg(target_os = "windows")]
pub fn get_claude_metadata(relative_path: &str) -> Result<u64, String> {
    let path = get_claude_unc_path(relative_path)?;
    
    let metadata = std::fs::metadata(&path)
        .map_err(|e| format!("Failed to read metadata: {}", e))?;
    
    let timestamp = metadata.modified()
        .map_err(|e| format!("Failed to get modification time: {}", e))?
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Failed to convert time: {}", e))?
        .as_secs();
    
    Ok(timestamp)
}

#[cfg(not(target_os = "windows"))]
pub fn get_claude_metadata(relative_path: &str) -> Result<u64, String> {
    let claude_dir = get_claude_dir().map_err(|e| e.to_string())?;
    let path = claude_dir.join(relative_path);
    
    let metadata = fs::metadata(&path)
        .map_err(|e| format!("Failed to read metadata: {}", e))?;
    
    let created_at = metadata
        .created()
        .or_else(|_| metadata.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH)
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    Ok(created_at)
}

/// Create a directory in the Claude directory
#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub fn create_claude_directory(relative_path: &str) -> Result<(), String> {
    let dir_path = get_claude_unc_path(relative_path)?;
    
    std::fs::create_dir_all(&dir_path)
        .map_err(|e| format!("Failed to create directory {}: {}", relative_path, e))
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
pub fn create_claude_directory(relative_path: &str) -> Result<(), String> {
    let claude_dir = get_claude_dir().map_err(|e| e.to_string())?;
    let dir_path = claude_dir.join(relative_path);
    
    fs::create_dir_all(&dir_path)
        .map_err(|e| format!("Failed to create directory {}: {}", relative_path, e))
}

/// Delete a file from the Claude directory
#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub fn delete_claude_file(relative_path: &str) -> Result<(), String> {
    let file_path = get_claude_unc_path(relative_path)?;
    
    if file_path.exists() {
        std::fs::remove_file(&file_path)
            .map_err(|e| format!("Failed to delete file {}: {}", relative_path, e))?;
    }
    
    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
pub fn delete_claude_file(relative_path: &str) -> Result<(), String> {
    let claude_dir = get_claude_dir().map_err(|e| e.to_string())?;
    let file_path = claude_dir.join(relative_path);
    
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| format!("Failed to delete file {}: {}", relative_path, e))?;
    }
    
    Ok(())
}

/// Check if a path is a directory in the Claude directory
#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub fn claude_is_directory(relative_path: &str) -> bool {
    get_claude_unc_path(relative_path)
        .ok()
        .map(|p| p.is_dir())
        .unwrap_or(false)
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
pub fn claude_is_directory(relative_path: &str) -> bool {
    if let Ok(claude_dir) = get_claude_dir() {
        claude_dir.join(relative_path).is_dir()
    } else {
        false
    }
}

/// Find all files matching a pattern in the Claude directory (optimized for large directories)
#[cfg(target_os = "windows")]
pub fn find_claude_files(relative_path: &str, pattern: &str) -> Result<Vec<String>, String> {
    use walkdir::WalkDir;
    use glob::Pattern;
    
    let search_path = get_claude_unc_path(relative_path)?;
    
    if !search_path.exists() {
        return Ok(Vec::new());
    }
    
    let base_path = CLAUDE_UNC_PATH.as_ref().map_err(|e| e.clone())?;
    let glob_pattern = Pattern::new(pattern)
        .map_err(|e| format!("Invalid pattern {}: {}", pattern, e))?;
    
    let mut results = Vec::new();
    
    for entry in WalkDir::new(&search_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Some(file_name) = entry.file_name().to_str() {
                if glob_pattern.matches(file_name) {
                    // Convert absolute path to relative path from ~/.claude
                    if let Ok(relative) = entry.path().strip_prefix(base_path) {
                        if let Some(relative_str) = relative.to_str() {
                            // Convert Windows path separators to forward slashes
                            results.push(relative_str.replace('\\', "/"));
                        }
                    }
                }
            }
        }
    }
    
    Ok(results)
}

#[cfg(not(target_os = "windows"))]
pub fn find_claude_files(relative_path: &str, pattern: &str) -> Result<Vec<String>, String> {
    use std::process::Command;
    
    let claude_dir = get_claude_dir().map_err(|e| e.to_string())?;
    let search_path = claude_dir.join(relative_path);
    
    if !search_path.exists() {
        return Ok(Vec::new());
    }
    
    let search_path_str = search_path.to_str()
        .ok_or_else(|| "Search path contains invalid UTF-8 characters".to_string())?;
    
    let output = Command::new("find")
        .args(&[search_path_str, "-name", pattern, "-type", "f"])
        .output()
        .map_err(|e| format!("Failed to run find command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Find command failed: {}", stderr.trim()));
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let claude_dir_str = claude_dir.to_str()
        .ok_or_else(|| "Claude directory path contains invalid UTF-8 characters".to_string())?;
    
    Ok(output_str
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            // Remove the claude directory prefix to get relative paths
            s.strip_prefix(&format!("{}/", claude_dir_str)).unwrap_or(s).to_string()
        })
        .collect())
}

/// Create the cache directory structure (~/.claudia/cache/)
pub fn ensure_cache_directory() -> Result<(), String> {
    // Create cache directory using idiomatic std::fs approach for all platforms
    #[cfg(target_os = "windows")]
    {
        let userprofile = std::env::var("USERPROFILE")
            .map_err(|_| "USERPROFILE environment variable not found".to_string())?;
        
        let cache_dir = PathBuf::from(userprofile)
            .join(".claudia")
            .join("cache");
        
        std::fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let home = dirs::home_dir().ok_or_else(|| "Failed to get home directory".to_string())?;
        let claudia_dir = home.join(".claudia");
        let cache_dir = claudia_dir.join("cache");
        
        std::fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    }
    
    Ok(())
}

/// Read a cache file from ~/.claudia/cache/
pub fn read_cache_file(cache_name: &str) -> Result<String, String> {
    // Validate cache name to prevent path traversal
    if cache_name.contains("..") || cache_name.contains("/") || cache_name.contains("\\") {
        return Err("Invalid cache file name".to_string());
    }
    #[cfg(target_os = "windows")]
    {
        let userprofile = std::env::var("USERPROFILE")
            .map_err(|_| "USERPROFILE environment variable not found".to_string())?;
        
        let cache_path = format!("{}\\.claudia\\cache\\{}", userprofile, cache_name);
        
        std::fs::read_to_string(&cache_path)
            .map_err(|e| format!("Failed to read cache file {}: {}", cache_name, e))
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let home = dirs::home_dir().ok_or_else(|| "Failed to get home directory".to_string())?;
        let cache_path = home.join(".claudia").join("cache").join(cache_name);
        
        std::fs::read_to_string(&cache_path)
            .map_err(|e| format!("Failed to read cache file {}: {}", cache_name, e))
    }
}

/// Write a cache file to ~/.claudia/cache/
pub fn write_cache_file(cache_name: &str, content: &str) -> Result<(), String> {
    // Validate cache name to prevent path traversal
    if cache_name.contains("..") || cache_name.contains("/") || cache_name.contains("\\") {
        return Err("Invalid cache file name".to_string());
    }
    
    // Ensure cache directory exists first
    ensure_cache_directory()?;
    
    #[cfg(target_os = "windows")]
    {
        let userprofile = std::env::var("USERPROFILE")
            .map_err(|_| "USERPROFILE environment variable not found".to_string())?;
        
        let cache_path = format!("{}\\.claudia\\cache\\{}", userprofile, cache_name);
        
        std::fs::write(&cache_path, content)
            .map_err(|e| format!("Failed to write cache file {}: {}", cache_name, e))
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let home = dirs::home_dir().ok_or_else(|| "Failed to get home directory".to_string())?;
        let cache_path = home.join(".claudia").join("cache").join(cache_name);
        
        std::fs::write(&cache_path, content)
            .map_err(|e| format!("Failed to write cache file {}: {}", cache_name, e))
    }
}

/// Check if a cache file exists and get its modification time
pub fn get_cache_file_metadata(cache_name: &str) -> Result<u64, String> {
    // Validate cache name to prevent path traversal
    if cache_name.contains("..") || cache_name.contains("/") || cache_name.contains("\\") {
        return Err("Invalid cache file name".to_string());
    }
    #[cfg(target_os = "windows")]
    {
        let userprofile = std::env::var("USERPROFILE")
            .map_err(|_| "USERPROFILE environment variable not found".to_string())?;
        
        let cache_path = format!("{}\\.claudia\\cache\\{}", userprofile, cache_name);
        
        let metadata = std::fs::metadata(&cache_path)
            .map_err(|e| format!("Failed to read cache metadata {}: {}", cache_name, e))?;
        
        let timestamp = metadata.modified()
            .map_err(|e| format!("Failed to get modification time: {}", e))?
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Failed to convert time: {}", e))?
            .as_secs();
        
        Ok(timestamp)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let home = dirs::home_dir().ok_or_else(|| "Failed to get home directory".to_string())?;
        let cache_path = home.join(".claudia").join("cache").join(cache_name);
        
        let metadata = std::fs::metadata(&cache_path)
            .map_err(|e| format!("Failed to read cache metadata {}: {}", cache_name, e))?;
        
        let timestamp = metadata.modified()
            .map_err(|e| format!("Failed to get modification time: {}", e))?
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Failed to convert time: {}", e))?
            .as_secs();
        
        Ok(timestamp)
    }
}