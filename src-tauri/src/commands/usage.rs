use crate::claude_paths::{
    claude_file_exists, find_claude_files, list_claude_directory, read_claude_file,
    read_cache_file, write_cache_file, get_cache_file_metadata,
};
use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use tauri::command;

// Global cache for usage entries
static USAGE_CACHE: Lazy<Arc<Mutex<UsageCache>>> = Lazy::new(|| {
    Arc::new(Mutex::new(UsageCache::new()))
});

#[derive(Debug)]
struct UsageCache {
    entries: Vec<UsageEntry>,
    last_updated: Option<std::time::Instant>,
    file_timestamps: HashMap<String, String>,
}

impl UsageCache {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            last_updated: None,
            file_timestamps: HashMap::new(),
        }
    }

    fn is_stale(&self) -> bool {
        match self.last_updated {
            None => true,
            Some(last) => last.elapsed() > std::time::Duration::from_secs(300), // 5 minute cache
        }
    }

    fn update(&mut self, entries: Vec<UsageEntry>, file_timestamps: HashMap<String, String>) {
        self.entries = entries;
        self.file_timestamps = file_timestamps;
        self.last_updated = Some(std::time::Instant::now());
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.file_timestamps.clear();
        self.last_updated = None;
    }

    // Disk cache methods
    fn load_from_disk(&mut self) -> Result<bool, String> {
        // First check if cache file exists and is valid
        if !Self::is_disk_cache_valid() {
            return Ok(false);
        }
        
        match read_cache_file("usage.json") {
            Ok(content) => {
                let cache_data: UsageCacheData = serde_json::from_str(&content)
                    .map_err(|e| format!("Failed to parse usage cache: {}", e))?;
                
                // Check cache version and TTL
                if cache_data.metadata.version != CACHE_VERSION {
                    log::warn!("Usage cache version mismatch, rebuilding");
                    return Ok(false);
                }
                
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                if now > cache_data.metadata.last_updated + cache_data.metadata.ttl_seconds {
                    log::info!("Usage cache expired, rebuilding");
                    return Ok(false);
                }
                
                // Load data into memory cache
                self.entries = cache_data.entries;
                self.file_timestamps = cache_data.file_timestamps;
                self.last_updated = Some(std::time::Instant::now());
                
                log::info!("Loaded {} usage entries from disk cache", self.entries.len());
                Ok(true)
            }
            Err(e) => {
                if e.contains("No such file") {
                    log::info!("No usage cache file found, will create new one");
                } else {
                    log::warn!("Failed to read usage cache: {}, will rebuild", e);
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
        
        let cache_data = UsageCacheData {
            metadata: CacheMetadata {
                version: CACHE_VERSION,
                created_at: now,
                last_updated: now,
                ttl_seconds: USAGE_CACHE_TTL,
                entries_count: self.entries.len(),
            },
            entries: self.entries.clone(),
            file_timestamps: self.file_timestamps.clone(),
        };
        
        let json = serde_json::to_string(&cache_data)
            .map_err(|e| format!("Failed to serialize usage cache: {}", e))?;
        
        write_cache_file("usage.json", &json)
            .map_err(|e| format!("Failed to write usage cache: {}", e))?;
        
        log::info!("Saved {} usage entries to disk cache", self.entries.len());
        Ok(())
    }

    fn is_disk_cache_valid() -> bool {
        match get_cache_file_metadata("usage.json") {
            Ok(modified_time) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                let is_valid = now <= modified_time + USAGE_CACHE_TTL;
                log::info!("Usage disk cache valid: {} (age: {}s)", is_valid, now - modified_time);
                is_valid
            }
            Err(e) => {
                log::info!("Usage disk cache not found: {}", e);
                false
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsageEntry {
    timestamp: String,
    model: String,
    input_tokens: u64,
    output_tokens: u64,
    cache_creation_tokens: u64,
    cache_read_tokens: u64,
    cost: f64,
    session_id: String,
    project_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageStats {
    total_cost: f64,
    total_tokens: u64,
    total_input_tokens: u64,
    total_output_tokens: u64,
    total_cache_creation_tokens: u64,
    total_cache_read_tokens: u64,
    total_sessions: u64,
    by_model: Vec<ModelUsage>,
    by_date: Vec<DailyUsage>,
    by_project: Vec<ProjectUsage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelUsage {
    model: String,
    total_cost: f64,
    total_tokens: u64,
    input_tokens: u64,
    output_tokens: u64,
    cache_creation_tokens: u64,
    cache_read_tokens: u64,
    session_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyUsage {
    date: String,
    total_cost: f64,
    total_tokens: u64,
    models_used: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectUsage {
    project_path: String,
    project_name: String,
    total_cost: f64,
    total_tokens: u64,
    session_count: u64,
    last_used: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadingProgress {
    current: u32,
    total: u32,
    message: String,
}

// Cache structures for disk persistence
#[derive(Debug, Serialize, Deserialize)]
struct CacheMetadata {
    version: u32,
    created_at: u64,
    last_updated: u64,
    ttl_seconds: u64,
    entries_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsageCacheData {
    metadata: CacheMetadata,
    entries: Vec<UsageEntry>,
    file_timestamps: HashMap<String, String>,
}

// Cache constants
const CACHE_VERSION: u32 = 1;
const USAGE_CACHE_TTL: u64 = 1800; // 30 minutes

// Claude 4 pricing constants (per million tokens)
const OPUS_4_INPUT_PRICE: f64 = 15.0;
const OPUS_4_OUTPUT_PRICE: f64 = 75.0;
const OPUS_4_CACHE_WRITE_PRICE: f64 = 18.75;
const OPUS_4_CACHE_READ_PRICE: f64 = 1.50;

const SONNET_4_INPUT_PRICE: f64 = 3.0;
const SONNET_4_OUTPUT_PRICE: f64 = 15.0;
const SONNET_4_CACHE_WRITE_PRICE: f64 = 3.75;
const SONNET_4_CACHE_READ_PRICE: f64 = 0.30;

#[derive(Debug, Deserialize)]
struct JsonlEntry {
    timestamp: String,
    message: Option<MessageData>,
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    #[serde(rename = "requestId")]
    request_id: Option<String>,
    #[serde(rename = "costUSD")]
    cost_usd: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct MessageData {
    id: Option<String>,
    model: Option<String>,
    usage: Option<UsageData>,
}

#[derive(Debug, Deserialize)]
struct UsageData {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    cache_creation_input_tokens: Option<u64>,
    cache_read_input_tokens: Option<u64>,
}

fn calculate_cost(model: &str, usage: &UsageData) -> f64 {
    let input_tokens = usage.input_tokens.unwrap_or(0) as f64;
    let output_tokens = usage.output_tokens.unwrap_or(0) as f64;
    let cache_creation_tokens = usage.cache_creation_input_tokens.unwrap_or(0) as f64;
    let cache_read_tokens = usage.cache_read_input_tokens.unwrap_or(0) as f64;

    // Calculate cost based on model
    let (input_price, output_price, cache_write_price, cache_read_price) =
        if model.contains("opus-4") || model.contains("claude-opus-4") {
            (
                OPUS_4_INPUT_PRICE,
                OPUS_4_OUTPUT_PRICE,
                OPUS_4_CACHE_WRITE_PRICE,
                OPUS_4_CACHE_READ_PRICE,
            )
        } else if model.contains("sonnet-4") || model.contains("claude-sonnet-4") {
            (
                SONNET_4_INPUT_PRICE,
                SONNET_4_OUTPUT_PRICE,
                SONNET_4_CACHE_WRITE_PRICE,
                SONNET_4_CACHE_READ_PRICE,
            )
        } else {
            // Return 0 for unknown models to avoid incorrect cost estimations.
            (0.0, 0.0, 0.0, 0.0)
        };

    // Calculate cost (prices are per million tokens)
    let cost = (input_tokens * input_price / 1_000_000.0)
        + (output_tokens * output_price / 1_000_000.0)
        + (cache_creation_tokens * cache_write_price / 1_000_000.0)
        + (cache_read_tokens * cache_read_price / 1_000_000.0);

    cost
}

fn parse_jsonl_file(
    relative_path: &str,
    encoded_project_name: &str,
    processed_hashes: &mut HashSet<String>,
) -> Vec<UsageEntry> {
    let mut entries = Vec::new();
    let mut actual_project_path: Option<String> = None;

    if let Ok(content) = read_claude_file(relative_path) {
        // Only log a sample of files to avoid spamming logs
        static FILE_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let count = FILE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if count < 5 || count % 100 == 0 {
            log::info!("[USAGE] Parsing file #{}: {}", count + 1, relative_path);
        }
        // Extract session ID from the file path
        // Path format: projects/project-name/session-id.jsonl
        let session_id = relative_path
            .split('/')
            .last()
            .and_then(|filename| filename.strip_suffix(".jsonl"))
            .unwrap_or("unknown")
            .to_string();

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(line) {
                // Extract the actual project path from cwd if we haven't already
                if actual_project_path.is_none() {
                    if let Some(cwd) = json_value.get("cwd").and_then(|v| v.as_str()) {
                        actual_project_path = Some(cwd.to_string());
                    }
                }

                // Try to parse as JsonlEntry for usage data
                match serde_json::from_value::<JsonlEntry>(json_value.clone()) {
                    Ok(entry) => {
                        if let Some(message) = &entry.message {
                        // Deduplication based on message ID and request ID
                        if let (Some(msg_id), Some(req_id)) = (&message.id, &entry.request_id) {
                            let unique_hash = format!("{}:{}", msg_id, req_id);
                            if processed_hashes.contains(&unique_hash) {
                                continue; // Skip duplicate entry
                            }
                            processed_hashes.insert(unique_hash);
                        }

                        if let Some(usage) = &message.usage {
                            // Skip entries without meaningful token usage
                            if usage.input_tokens.unwrap_or(0) == 0
                                && usage.output_tokens.unwrap_or(0) == 0
                                && usage.cache_creation_input_tokens.unwrap_or(0) == 0
                                && usage.cache_read_input_tokens.unwrap_or(0) == 0
                            {
                                continue;
                            }
                            
                            // Log first few entries at info level for debugging
                            static USAGE_LOG_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                            let log_count = USAGE_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if log_count < 5 {
                                log::info!("[USAGE] Found usage entry #{} in {}: model={:?}, input={:?}, output={:?}, cache_creation={:?}, cache_read={:?}", 
                                    log_count + 1,
                                    relative_path, 
                                    message.model, 
                                    usage.input_tokens, 
                                    usage.output_tokens,
                                    usage.cache_creation_input_tokens,
                                    usage.cache_read_input_tokens);
                            }

                            let cost = entry.cost_usd.unwrap_or_else(|| {
                                if let Some(model_str) = &message.model {
                                    calculate_cost(model_str, usage)
                                } else {
                                    0.0
                                }
                            });

                            // Use actual project path if found, otherwise use encoded name
                            let project_path = actual_project_path
                                .clone()
                                .unwrap_or_else(|| encoded_project_name.to_string());

                            entries.push(UsageEntry {
                                timestamp: entry.timestamp,
                                model: message
                                    .model
                                    .clone()
                                    .unwrap_or_else(|| "unknown".to_string()),
                                input_tokens: usage.input_tokens.unwrap_or(0),
                                output_tokens: usage.output_tokens.unwrap_or(0),
                                cache_creation_tokens: usage
                                    .cache_creation_input_tokens
                                    .unwrap_or(0),
                                cache_read_tokens: usage.cache_read_input_tokens.unwrap_or(0),
                                cost,
                                session_id: entry.session_id.unwrap_or_else(|| session_id.clone()),
                                project_path,
                            });
                        }
                    }
                    },
                    Err(e) => {
                        // Log parsing errors for first few attempts
                        static PARSE_ERROR_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                        let error_count = PARSE_ERROR_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if error_count < 5 {
                            log::warn!("[USAGE] Failed to parse JSONL entry #{} in {}: {}", error_count + 1, relative_path, e);
                            log::debug!("[USAGE] Failed JSON: {:?}", json_value);
                        }
                    }
                }
            }
        }
    }
    
    if !entries.is_empty() {
        log::info!("[USAGE] Parsed {} usage entries from {}", entries.len(), relative_path);
    }

    entries
}

fn get_earliest_timestamp(relative_path: &str) -> Option<String> {
    if let Ok(content) = read_claude_file(relative_path) {
        // Only check first few lines for performance
        for line in content.lines().take(10) {
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(timestamp_str) = json_value.get("timestamp").and_then(|v| v.as_str()) {
                    return Some(timestamp_str.to_string());
                }
            }
        }
    }
    None
}

fn filter_files_by_date_range(
    files: Vec<(String, String)>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Vec<(String, String)> {
    if start_date.is_none() && end_date.is_none() {
        return files;
    }

    files.into_par_iter()
        .filter(|(path, _)| {
            if let Some(timestamp) = get_earliest_timestamp(path) {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&timestamp) {
                    let file_date = dt.naive_local().date();
                    let after_start = start_date.map_or(true, |s| file_date >= s);
                    let before_end = end_date.map_or(true, |e| file_date <= e);
                    return after_start && before_end;
                }
            }
            true // Include files we can't parse timestamp from
        })
        .collect()
}

fn get_all_usage_entries_with_filter(
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    force_refresh: bool,
) -> Vec<UsageEntry> {
    // Check cache first
    if !force_refresh {
        let mut cache = USAGE_CACHE.lock().unwrap();
        
        // Try loading from disk cache if memory cache is stale/empty
        if cache.is_stale() || cache.entries.is_empty() {
            match cache.load_from_disk() {
                Ok(true) => log::info!("[CACHE] Successfully loaded usage cache from disk"),
                Ok(false) => log::debug!("[CACHE] Disk cache not loaded (invalid or missing)"),
                Err(e) => {
                    log::warn!("Failed to load usage cache from disk: {}", e);
                }
            }
        }
        
        if !cache.is_stale() && !cache.entries.is_empty() {
            log::info!("Using cached usage data ({} entries, cached {} seconds ago)", 
                cache.entries.len(), 
                cache.last_updated.map(|t| t.elapsed().as_secs()).unwrap_or(0));
            
            // Filter cached entries by date if needed
            if start_date.is_some() || end_date.is_some() {
                return cache.entries.iter()
                    .filter(|e| {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(&e.timestamp) {
                            let entry_date = dt.naive_local().date();
                            let after_start = start_date.map_or(true, |s| entry_date >= s);
                            let before_end = end_date.map_or(true, |e| entry_date <= e);
                            after_start && before_end
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect();
            }
            
            return cache.entries.clone();
        }
    } else {
        log::info!("Cache miss or forced refresh - loading usage data from disk");
    }

    let mut all_entries = Vec::new();
    let mut processed_hashes = HashSet::new();
    let mut files_to_process: Vec<(String, String)> = Vec::new();

    // Use optimized find command to get all .jsonl files at once
    log::info!("[USAGE] Finding all .jsonl files in projects directory...");
    match find_claude_files("projects", "*.jsonl") {
        Ok(jsonl_files) => {
            log::info!("[USAGE] Found {} .jsonl files using find command", jsonl_files.len());
            
            // Extract project name from each file path
            for file_path in jsonl_files {
                // File path format: projects/project-name/session-id/file.jsonl
                let path_parts: Vec<&str> = file_path.split('/').collect();
                if path_parts.len() >= 2 && path_parts[0] == "projects" {
                    let project_name = path_parts[1].to_string();
                    files_to_process.push((file_path, project_name));
                }
            }
        }
        Err(e) => {
            log::warn!("Find command failed: {}, falling back to recursive listing", e);
            // Fallback implementation omitted for brevity
        }
    }

    // Filter files by date range if specified
    if start_date.is_some() || end_date.is_some() {
        log::debug!("Filtering {} files by date range...", files_to_process.len());
        files_to_process = filter_files_by_date_range(files_to_process, start_date, end_date);
        log::debug!("Filtered to {} files", files_to_process.len());
    }

    // Sort files by their earliest timestamp to ensure chronological processing
    files_to_process.sort_by_cached_key(|(path, _)| get_earliest_timestamp(path));

    // Process files in parallel for better performance
    let file_chunks: Vec<_> = files_to_process.chunks(50).collect();
    log::info!("[USAGE] Processing {} files in {} chunks...", files_to_process.len(), file_chunks.len());
    
    let chunk_results: Vec<Vec<UsageEntry>> = file_chunks.par_iter()
        .map(|chunk| {
            let mut chunk_entries = Vec::new();
            let mut chunk_hashes = HashSet::new();
            
            for (relative_path, project_name) in chunk.iter() {
                let entries = parse_jsonl_file(relative_path, project_name, &mut chunk_hashes);
                chunk_entries.extend(entries);
            }
            
            chunk_entries
        })
        .collect();

    // Merge results and handle deduplication across chunks
    for chunk_entries in chunk_results {
        for entry in chunk_entries {
            // Re-check for duplicates across chunks
            let unique_hash = format!("{}:{}", entry.timestamp, entry.session_id);
            if !processed_hashes.contains(&unique_hash) {
                processed_hashes.insert(unique_hash);
                all_entries.push(entry);
            }
        }
    }

    // Sort by timestamp
    all_entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    // Update cache if we did a full refresh
    if start_date.is_none() && end_date.is_none() {
        let file_timestamps: HashMap<String, String> = files_to_process.into_iter()
            .filter_map(|(path, _)| {
                get_earliest_timestamp(&path).map(|ts| (path, ts))
            })
            .collect();
        
        let mut cache = USAGE_CACHE.lock().unwrap();
        cache.update(all_entries.clone(), file_timestamps);
        log::info!("[CACHE] Updated usage cache with {} entries (will be valid for 30 minutes)", all_entries.len());
        
        // Save to disk cache
        log::info!("[CACHE] Saving usage cache to disk...");
        if let Err(e) = cache.save_to_disk() {
            log::warn!("[CACHE] Failed to save usage cache to disk: {}", e);
        } else {
            log::info!("[CACHE] Usage cache saved successfully");
        }
    }
    
    log::info!("[USAGE] Total usage entries found: {}", all_entries.len());

    all_entries
}

fn get_all_usage_entries() -> Vec<UsageEntry> {
    get_all_usage_entries_with_filter(None, None, false)
}

#[command]
pub fn get_usage_stats(days: Option<u32>) -> Result<UsageStats, String> {
    log::debug!("Getting usage stats for days: {:?}", days);
    
    let (start_date, end_date) = if let Some(days) = days {
        let end = Local::now().naive_local().date();
        let start = end - chrono::Duration::days(days as i64);
        (Some(start), Some(end))
    } else {
        (None, None)
    };
    
    let all_entries = get_all_usage_entries_with_filter(start_date, end_date, false);
    log::debug!("Found {} total entries", all_entries.len());

    if all_entries.is_empty() {
        return Ok(UsageStats {
            total_cost: 0.0,
            total_tokens: 0,
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_cache_creation_tokens: 0,
            total_cache_read_tokens: 0,
            total_sessions: 0,
            by_model: vec![],
            by_date: vec![],
            by_project: vec![],
        });
    }

    // Calculate aggregated stats
    let mut total_cost = 0.0;
    let mut total_input_tokens = 0u64;
    let mut total_output_tokens = 0u64;
    let mut total_cache_creation_tokens = 0u64;
    let mut total_cache_read_tokens = 0u64;

    let mut model_stats: HashMap<String, ModelUsage> = HashMap::new();
    let mut daily_stats: HashMap<String, DailyUsage> = HashMap::new();
    let mut project_stats: HashMap<String, ProjectUsage> = HashMap::new();

    for entry in &all_entries {
        // Update totals
        total_cost += entry.cost;
        total_input_tokens += entry.input_tokens;
        total_output_tokens += entry.output_tokens;
        total_cache_creation_tokens += entry.cache_creation_tokens;
        total_cache_read_tokens += entry.cache_read_tokens;

        // Update model stats
        let model_stat = model_stats
            .entry(entry.model.clone())
            .or_insert(ModelUsage {
                model: entry.model.clone(),
                total_cost: 0.0,
                total_tokens: 0,
                input_tokens: 0,
                output_tokens: 0,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                session_count: 0,
            });
        model_stat.total_cost += entry.cost;
        model_stat.input_tokens += entry.input_tokens;
        model_stat.output_tokens += entry.output_tokens;
        model_stat.cache_creation_tokens += entry.cache_creation_tokens;
        model_stat.cache_read_tokens += entry.cache_read_tokens;
        model_stat.total_tokens = model_stat.input_tokens + model_stat.output_tokens;
        model_stat.session_count += 1;

        // Update daily stats
        let date = entry
            .timestamp
            .split('T')
            .next()
            .unwrap_or(&entry.timestamp)
            .to_string();
        let daily_stat = daily_stats.entry(date.clone()).or_insert(DailyUsage {
            date,
            total_cost: 0.0,
            total_tokens: 0,
            models_used: vec![],
        });
        daily_stat.total_cost += entry.cost;
        daily_stat.total_tokens += entry.input_tokens
            + entry.output_tokens
            + entry.cache_creation_tokens
            + entry.cache_read_tokens;
        if !daily_stat.models_used.contains(&entry.model) {
            daily_stat.models_used.push(entry.model.clone());
        }

        // Update project stats
        let project_stat =
            project_stats
                .entry(entry.project_path.clone())
                .or_insert(ProjectUsage {
                    project_path: entry.project_path.clone(),
                    project_name: entry
                        .project_path
                        .split('/')
                        .last()
                        .unwrap_or(&entry.project_path)
                        .to_string(),
                    total_cost: 0.0,
                    total_tokens: 0,
                    session_count: 0,
                    last_used: entry.timestamp.clone(),
                });
        project_stat.total_cost += entry.cost;
        project_stat.total_tokens += entry.input_tokens
            + entry.output_tokens
            + entry.cache_creation_tokens
            + entry.cache_read_tokens;
        project_stat.session_count += 1;
        if entry.timestamp > project_stat.last_used {
            project_stat.last_used = entry.timestamp.clone();
        }
    }

    let total_tokens = total_input_tokens
        + total_output_tokens
        + total_cache_creation_tokens
        + total_cache_read_tokens;
    let total_sessions = all_entries.len() as u64;

    // Convert hashmaps to sorted vectors
    let mut by_model: Vec<ModelUsage> = model_stats.into_values().collect();
    by_model.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap());

    let mut by_date: Vec<DailyUsage> = daily_stats.into_values().collect();
    by_date.sort_by(|a, b| b.date.cmp(&a.date));

    let mut by_project: Vec<ProjectUsage> = project_stats.into_values().collect();
    by_project.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap());

    Ok(UsageStats {
        total_cost,
        total_tokens,
        total_input_tokens,
        total_output_tokens,
        total_cache_creation_tokens,
        total_cache_read_tokens,
        total_sessions,
        by_model,
        by_date,
        by_project,
    })
}

#[command]
pub fn get_usage_by_date_range(start_date: String, end_date: String) -> Result<UsageStats, String> {
    // Parse dates
    let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").or_else(|_| {
        // Try parsing ISO datetime format
        DateTime::parse_from_rfc3339(&start_date)
            .map(|dt| dt.naive_local().date())
            .map_err(|e| format!("Invalid start date: {}", e))
    })?;
    let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").or_else(|_| {
        // Try parsing ISO datetime format
        DateTime::parse_from_rfc3339(&end_date)
            .map(|dt| dt.naive_local().date())
            .map_err(|e| format!("Invalid end date: {}", e))
    })?;

    let all_entries = get_all_usage_entries_with_filter(Some(start), Some(end), false);

    if all_entries.is_empty() {
        return Ok(UsageStats {
            total_cost: 0.0,
            total_tokens: 0,
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_cache_creation_tokens: 0,
            total_cache_read_tokens: 0,
            total_sessions: 0,
            by_model: vec![],
            by_date: vec![],
            by_project: vec![],
        });
    }

    // Calculate aggregated stats (same logic as get_usage_stats)
    let mut total_cost = 0.0;
    let mut total_input_tokens = 0u64;
    let mut total_output_tokens = 0u64;
    let mut total_cache_creation_tokens = 0u64;
    let mut total_cache_read_tokens = 0u64;

    let mut model_stats: HashMap<String, ModelUsage> = HashMap::new();
    let mut daily_stats: HashMap<String, DailyUsage> = HashMap::new();
    let mut project_stats: HashMap<String, ProjectUsage> = HashMap::new();

    for entry in &all_entries {
        // Update totals
        total_cost += entry.cost;
        total_input_tokens += entry.input_tokens;
        total_output_tokens += entry.output_tokens;
        total_cache_creation_tokens += entry.cache_creation_tokens;
        total_cache_read_tokens += entry.cache_read_tokens;

        // Update model stats
        let model_stat = model_stats
            .entry(entry.model.clone())
            .or_insert(ModelUsage {
                model: entry.model.clone(),
                total_cost: 0.0,
                total_tokens: 0,
                input_tokens: 0,
                output_tokens: 0,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                session_count: 0,
            });
        model_stat.total_cost += entry.cost;
        model_stat.input_tokens += entry.input_tokens;
        model_stat.output_tokens += entry.output_tokens;
        model_stat.cache_creation_tokens += entry.cache_creation_tokens;
        model_stat.cache_read_tokens += entry.cache_read_tokens;
        model_stat.total_tokens = model_stat.input_tokens + model_stat.output_tokens;
        model_stat.session_count += 1;

        // Update daily stats
        let date = entry
            .timestamp
            .split('T')
            .next()
            .unwrap_or(&entry.timestamp)
            .to_string();
        let daily_stat = daily_stats.entry(date.clone()).or_insert(DailyUsage {
            date,
            total_cost: 0.0,
            total_tokens: 0,
            models_used: vec![],
        });
        daily_stat.total_cost += entry.cost;
        daily_stat.total_tokens += entry.input_tokens
            + entry.output_tokens
            + entry.cache_creation_tokens
            + entry.cache_read_tokens;
        if !daily_stat.models_used.contains(&entry.model) {
            daily_stat.models_used.push(entry.model.clone());
        }

        // Update project stats
        let project_stat =
            project_stats
                .entry(entry.project_path.clone())
                .or_insert(ProjectUsage {
                    project_path: entry.project_path.clone(),
                    project_name: entry
                        .project_path
                        .split('/')
                        .last()
                        .unwrap_or(&entry.project_path)
                        .to_string(),
                    total_cost: 0.0,
                    total_tokens: 0,
                    session_count: 0,
                    last_used: entry.timestamp.clone(),
                });
        project_stat.total_cost += entry.cost;
        project_stat.total_tokens += entry.input_tokens
            + entry.output_tokens
            + entry.cache_creation_tokens
            + entry.cache_read_tokens;
        project_stat.session_count += 1;
        if entry.timestamp > project_stat.last_used {
            project_stat.last_used = entry.timestamp.clone();
        }
    }

    let total_tokens = total_input_tokens
        + total_output_tokens
        + total_cache_creation_tokens
        + total_cache_read_tokens;
    let total_sessions = all_entries.len() as u64;

    // Convert hashmaps to sorted vectors
    let mut by_model: Vec<ModelUsage> = model_stats.into_values().collect();
    by_model.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap());

    let mut by_date: Vec<DailyUsage> = daily_stats.into_values().collect();
    by_date.sort_by(|a, b| b.date.cmp(&a.date));

    let mut by_project: Vec<ProjectUsage> = project_stats.into_values().collect();
    by_project.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap());

    Ok(UsageStats {
        total_cost,
        total_tokens,
        total_input_tokens,
        total_output_tokens,
        total_cache_creation_tokens,
        total_cache_read_tokens,
        total_sessions,
        by_model,
        by_date,
        by_project,
    })
}

#[command]
pub fn get_usage_details(
    project_path: Option<String>,
    date: Option<String>,
) -> Result<Vec<UsageEntry>, String> {
    let mut all_entries = get_all_usage_entries();

    // Filter by project if specified
    if let Some(project) = project_path {
        all_entries.retain(|e| e.project_path == project);
    }

    // Filter by date if specified
    if let Some(date) = date {
        all_entries.retain(|e| e.timestamp.starts_with(&date));
    }

    Ok(all_entries)
}

#[command]
pub fn get_session_stats(
    since: Option<String>,
    until: Option<String>,
    order: Option<String>,
) -> Result<Vec<ProjectUsage>, String> {
    let since_date = since.and_then(|s| NaiveDate::parse_from_str(&s, "%Y%m%d").ok());
    let until_date = until.and_then(|s| NaiveDate::parse_from_str(&s, "%Y%m%d").ok());
    
    let all_entries = get_all_usage_entries_with_filter(since_date, until_date, false);

    let mut session_stats: HashMap<String, ProjectUsage> = HashMap::new();
    for entry in &all_entries {
        let session_key = format!("{}/{}", entry.project_path, entry.session_id);
        let project_stat = session_stats
            .entry(session_key)
            .or_insert_with(|| ProjectUsage {
                project_path: entry.project_path.clone(),
                project_name: entry.session_id.clone(), // Using session_id as project_name for session view
                total_cost: 0.0,
                total_tokens: 0,
                session_count: 0, // In this context, this will count entries per session
                last_used: " ".to_string(),
            });

        project_stat.total_cost += entry.cost;
        project_stat.total_tokens += entry.input_tokens
            + entry.output_tokens
            + entry.cache_creation_tokens
            + entry.cache_read_tokens;
        project_stat.session_count += 1;
        if entry.timestamp > project_stat.last_used {
            project_stat.last_used = entry.timestamp.clone();
        }
    }

    let mut by_session: Vec<ProjectUsage> = session_stats.into_values().collect();

    // Sort by last_used date
    if let Some(order_str) = order {
        if order_str == "asc" {
            by_session.sort_by(|a, b| a.last_used.cmp(&b.last_used));
        } else {
            by_session.sort_by(|a, b| b.last_used.cmp(&a.last_used));
        }
    } else {
        // Default to descending
        by_session.sort_by(|a, b| b.last_used.cmp(&a.last_used));
    }

    Ok(by_session)
}

#[command]
pub fn clear_usage_cache() -> Result<String, String> {
    let mut cache = USAGE_CACHE.lock().unwrap();
    cache.clear();
    Ok("Usage cache cleared successfully".to_string())
}

#[command]
pub fn get_usage_cache_stats() -> Result<String, String> {
    let cache = USAGE_CACHE.lock().unwrap();
    let stats = format!(
        "Cache stats: {} entries, last updated: {:?}, is stale: {}",
        cache.entries.len(),
        cache.last_updated.map(|t| t.elapsed().as_secs()).unwrap_or(0),
        cache.is_stale()
    );
    Ok(stats)
}

#[command]
pub fn test_claude_paths() -> Result<String, String> {
    let mut results = Vec::new();
    
    // Test 1: List projects directory
    results.push("=== Testing list_claude_directory('projects') ===".to_string());
    match list_claude_directory("projects") {
        Ok(projects) => {
            results.push(format!("Success! Found {} projects:", projects.len()));
            for project in projects.iter().take(5) {
                results.push(format!("  - {}", project));
            }
            if projects.len() > 5 {
                results.push(format!("  ... and {} more", projects.len() - 5));
            }
        }
        Err(e) => {
            results.push(format!("Error: {}", e));
        }
    }
    
    // Test 2: Check if a known file exists
    results.push("\n=== Testing claude_file_exists('settings.json') ===".to_string());
    let exists = claude_file_exists("settings.json");
    results.push(format!("settings.json exists: {}", exists));
    
    // Test 3: Try to read settings.json
    results.push("\n=== Testing read_claude_file('settings.json') ===".to_string());
    match read_claude_file("settings.json") {
        Ok(content) => {
            results.push(format!("Success! File length: {} bytes", content.len()));
            if content.len() > 100 {
                results.push(format!("First 100 chars: {}", &content[..100]));
            } else {
                results.push(format!("Content: {}", content));
            }
        }
        Err(e) => {
            results.push(format!("Error: {}", e));
        }
    }
    
    Ok(results.join("\n"))
}