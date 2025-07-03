#[cfg(not(target_os = "windows"))]
use std::fs;
use std::path::PathBuf;
#[cfg(not(target_os = "windows"))]
use std::time::SystemTime;

#[cfg(target_os = "windows")]
use once_cell::sync::Lazy;

#[cfg(target_os = "windows")]
static WSL_HOME_DIR: Lazy<Result<String, String>> = Lazy::new(|| {
    let output = std::process::Command::new("wsl")
        .args(&["--", "echo", "$HOME"])
        .output()
        .map_err(|e| format!("Failed to get WSL home directory: {}", e))?;
    
    if !output.status.success() {
        return Err("Failed to get WSL home directory".to_string());
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
});

/// Get the WSL home directory path (cached)
#[cfg(target_os = "windows")]
fn get_wsl_home_dir() -> Result<String, String> {
    match WSL_HOME_DIR.as_ref() {
        Ok(dir) => Ok(dir.clone()),
        Err(e) => Err(e.clone()),
    }
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
    let home_dir = get_wsl_home_dir()?;
    
    let output = std::process::Command::new("wsl")
        .args(&["--", "cat", &format!("{}/.claude/{}", home_dir, relative_path)])
        .output()
        .map_err(|e| format!("Failed to run WSL command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("File not found: {} ({})", relative_path, stderr.trim()));
    }
    
    String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to decode file: {}", e))
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
    let home_dir = get_wsl_home_dir()?;
    
    let mut cmd = std::process::Command::new("wsl");
    cmd.args(&["--", "sh", "-c", &format!("cat > {}/.claude/{}", home_dir, relative_path)]);
    cmd.stdin(std::process::Stdio::piped());
    
    let mut child = cmd.spawn()
        .map_err(|e| format!("Failed to run WSL command: {}", e))?;
    
    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write to WSL stdin: {}", e))?;
    }
    
    let output = child.wait_with_output()
        .map_err(|e| format!("Failed to wait for WSL command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to write file: {} ({})", relative_path, stderr.trim()));
    }
    
    Ok(())
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
    let home_dir = get_wsl_home_dir()?;
    
    let output = std::process::Command::new("wsl")
        .args(&["--", "ls", "-1a", &format!("{}/.claude/{}", home_dir, relative_path)])
        .output()
        .map_err(|e| format!("Failed to run WSL command: {}", e))?;
    
    if !output.status.success() {
        return Ok(Vec::new()); // Directory might not exist
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty() && s != "." && s != "..")
        .collect())
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
    match get_wsl_home_dir() {
        Ok(home_dir) => {
            match std::process::Command::new("wsl")
                .args(&["--", "test", "-f", &format!("{}/.claude/{}", home_dir, relative_path)])
                .status() {
                Ok(status) => status.success(),
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
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
    let home_dir = get_wsl_home_dir()?;
    
    // On Windows/WSL, we'll use the modification time since creation time isn't reliable
    let output = std::process::Command::new("wsl")
        .args(&["--", "stat", "-c", "%Y", &format!("{}/.claude/{}", home_dir, relative_path)])
        .output()
        .map_err(|e| format!("Failed to run WSL stat command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get file metadata: {}", stderr.trim()));
    }
    
    let timestamp_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    timestamp_str.parse::<u64>()
        .map_err(|e| format!("Failed to parse timestamp: {}", e))
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
    let home_dir = get_wsl_home_dir()?;
    
    let output = std::process::Command::new("wsl")
        .args(&["--", "mkdir", "-p", &format!("{}/.claude/{}", home_dir, relative_path)])
        .output()
        .map_err(|e| format!("Failed to run WSL command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to create directory: {} ({})", relative_path, stderr.trim()));
    }
    
    Ok(())
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
    let home_dir = get_wsl_home_dir()?;
    
    let output = std::process::Command::new("wsl")
        .args(&["--", "rm", "-f", &format!("{}/.claude/{}", home_dir, relative_path)])
        .output()
        .map_err(|e| format!("Failed to run WSL command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to delete file: {} ({})", relative_path, stderr.trim()));
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
    match get_wsl_home_dir() {
        Ok(home_dir) => {
            match std::process::Command::new("wsl")
                .args(&["--", "test", "-d", &format!("{}/.claude/{}", home_dir, relative_path)])
                .status() {
                Ok(status) => status.success(),
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
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
    let home_dir = get_wsl_home_dir()?;
    
    let output = std::process::Command::new("wsl")
        .args(&["--", "find", &format!("{}/.claude/{}", home_dir, relative_path), 
                "-name", pattern, "-type", "f"])
        .output()
        .map_err(|e| format!("Failed to run WSL find command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // If directory doesn't exist, return empty list rather than error
        if stderr.contains("No such file or directory") {
            return Ok(Vec::new());
        }
        return Err(format!("Find command failed: {}", stderr.trim()));
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let base_path = format!("{}/.claude/", home_dir);
    
    Ok(output_str
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            // Remove the base path to get relative paths
            s.strip_prefix(&base_path).unwrap_or(s).to_string()
        })
        .collect())
}

#[cfg(not(target_os = "windows"))]
pub fn find_claude_files(relative_path: &str, pattern: &str) -> Result<Vec<String>, String> {
    use std::process::Command;
    
    let claude_dir = get_claude_dir().map_err(|e| e.to_string())?;
    let search_path = claude_dir.join(relative_path);
    
    if !search_path.exists() {
        return Ok(Vec::new());
    }
    
    let output = Command::new("find")
        .args(&[search_path.to_str().unwrap(), "-name", pattern, "-type", "f"])
        .output()
        .map_err(|e| format!("Failed to run find command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Find command failed: {}", stderr.trim()));
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let claude_dir_str = claude_dir.to_str().unwrap();
    
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
    // First ensure ~/.claudia exists (for cross-platform compatibility)
    #[cfg(target_os = "windows")]
    {
        let userprofile = std::env::var("USERPROFILE")
            .map_err(|_| "USERPROFILE environment variable not found".to_string())?;
        
        let output = std::process::Command::new("cmd")
            .args(&["/C", "if", "not", "exist", &format!("{}/.claudia", userprofile), "mkdir", &format!("{}/.claudia", userprofile)])
            .output()
            .map_err(|e| format!("Failed to create .claudia directory: {}", e))?;
            
        if !output.status.success() {
            return Err("Failed to create .claudia directory".to_string());
        }
        
        // Create cache subdirectory
        let output = std::process::Command::new("cmd")
            .args(&["/C", "if", "not", "exist", &format!("{}/.claudia/cache", userprofile), "mkdir", &format!("{}/.claudia/cache", userprofile)])
            .output()
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;
            
        if !output.status.success() {
            return Err("Failed to create cache directory".to_string());
        }
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
    #[cfg(target_os = "windows")]
    {
        let userprofile = std::env::var("USERPROFILE")
            .map_err(|_| "USERPROFILE environment variable not found".to_string())?;
        
        let cache_path = format!("{}/.claudia/cache/{}", userprofile, cache_name);
        
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
    // Ensure cache directory exists first
    ensure_cache_directory()?;
    
    #[cfg(target_os = "windows")]
    {
        let userprofile = std::env::var("USERPROFILE")
            .map_err(|_| "USERPROFILE environment variable not found".to_string())?;
        
        let cache_path = format!("{}/.claudia/cache/{}", userprofile, cache_name);
        
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
    #[cfg(target_os = "windows")]
    {
        let userprofile = std::env::var("USERPROFILE")
            .map_err(|_| "USERPROFILE environment variable not found".to_string())?;
        
        let cache_path = format!("{}/.claudia/cache/{}", userprofile, cache_name);
        
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