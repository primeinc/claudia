use crate::claude_paths::{find_claude_files, read_claude_file};
use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use rayon::prelude::*;
use tauri::command;

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
    project_name: String,  // The actual directory name, not a path
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
    project_name: String,
    display_name: String,
    total_cost: f64,
    total_tokens: u64,
    session_count: u64,
    last_used: String,
}

// Pricing constants
const OPUS_4_INPUT_PRICE: f64 = 15.0;
const OPUS_4_OUTPUT_PRICE: f64 = 75.0;
const OPUS_4_CACHE_WRITE_PRICE: f64 = 18.75;
const OPUS_4_CACHE_READ_PRICE: f64 = 1.50;

const SONNET_4_INPUT_PRICE: f64 = 3.0;
const SONNET_4_OUTPUT_PRICE: f64 = 15.0;
const SONNET_4_CACHE_WRITE_PRICE: f64 = 3.75;
const SONNET_4_CACHE_READ_PRICE: f64 = 0.30;

#[command]
pub fn get_usage_stats(days: Option<u32>) -> Result<UsageStats, String> {
    log::info!("Getting usage stats for days: {:?}", days);
    
    // Calculate date range
    let (start_date, end_date) = if let Some(days) = days {
        let end = Local::now().naive_local().date();
        let start = end - chrono::Duration::days(days as i64);
        (Some(start), Some(end))
    } else {
        (None, None)
    };
    
    // Get all usage entries
    let entries = collect_usage_entries(start_date, end_date)?;
    
    if entries.is_empty() {
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
    calculate_usage_stats(entries)
}

fn collect_usage_entries(
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<Vec<UsageEntry>, String> {
    // Find all JSONL files in projects directory
    let jsonl_files = find_claude_files("projects", "*.jsonl")
        .map_err(|e| format!("Failed to find JSONL files: {}", e))?;
    
    log::info!("Found {} JSONL files", jsonl_files.len());
    
    // Process files in parallel
    let entries: Vec<UsageEntry> = jsonl_files
        .par_iter()
        .flat_map(|file_path| {
            process_jsonl_file(file_path, start_date, end_date)
        })
        .collect();
    
    log::info!("Collected {} usage entries", entries.len());
    Ok(entries)
}

fn process_jsonl_file(
    relative_path: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Vec<UsageEntry> {
    // Extract project name and session ID from path
    // Path format: projects/PROJECT_NAME/SESSION_ID.jsonl
    let path_parts: Vec<&str> = relative_path.split('/').collect();
    
    if path_parts.len() < 3 {
        log::warn!("Invalid path format: {}", relative_path);
        return vec![];
    }
    
    let project_name = path_parts[1].to_string();
    let session_id = path_parts[2]
        .strip_suffix(".jsonl")
        .unwrap_or(path_parts[2])
        .to_string();
    
    // Read and parse the file
    let content = match read_claude_file(relative_path) {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to read {}: {}", relative_path, e);
            return vec![];
        }
    };
    
    let mut entries = Vec::new();
    let mut seen_messages = HashSet::new();
    
    for (line_num, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        
        match serde_json::from_str::<Value>(line) {
            Ok(json) => {
                if let Some(entry) = extract_usage_entry(
                    &json,
                    &project_name,
                    &session_id,
                    &mut seen_messages,
                    start_date,
                    end_date,
                ) {
                    entries.push(entry);
                }
            }
            Err(e) => {
                if line_num < 5 {
                    log::warn!("Failed to parse JSON in {} line {}: {}", relative_path, line_num + 1, e);
                }
            }
        }
    }
    
    entries
}

fn extract_usage_entry(
    json: &Value,
    project_name: &str,
    session_id: &str,
    seen_messages: &mut HashSet<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Option<UsageEntry> {
    // Check if this is a message with usage data
    let message = json.get("message")?;
    let usage = message.get("usage")?;
    
    // Extract timestamp and check date range
    let timestamp = json.get("timestamp")?.as_str()?;
    
    if let Some(start) = start_date {
        if let Ok(dt) = DateTime::parse_from_rfc3339(timestamp) {
            let date = dt.naive_local().date();
            if date < start {
                return None;
            }
        }
    }
    
    if let Some(end) = end_date {
        if let Ok(dt) = DateTime::parse_from_rfc3339(timestamp) {
            let date = dt.naive_local().date();
            if date > end {
                return None;
            }
        }
    }
    
    // Deduplicate by message ID + request ID
    if let (Some(msg_id), Some(req_id)) = (
        message.get("id").and_then(|v| v.as_str()),
        json.get("requestId").and_then(|v| v.as_str()),
    ) {
        let unique_id = format!("{}:{}", msg_id, req_id);
        if !seen_messages.insert(unique_id) {
            return None; // Already seen
        }
    }
    
    // Extract token counts
    let input_tokens = usage.get("input_tokens")?.as_u64()?;
    let output_tokens = usage.get("output_tokens")?.as_u64()?;
    let cache_creation_tokens = usage.get("cache_creation_input_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let cache_read_tokens = usage.get("cache_read_input_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    
    // Skip entries with no meaningful usage
    if input_tokens == 0 && output_tokens == 0 && cache_creation_tokens == 0 && cache_read_tokens == 0 {
        return None;
    }
    
    // Extract model and calculate cost
    let model = message.get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    
    let cost = json.get("costUSD")
        .and_then(|v| v.as_f64())
        .unwrap_or_else(|| calculate_cost(&model, input_tokens, output_tokens, cache_creation_tokens, cache_read_tokens));
    
    Some(UsageEntry {
        timestamp: timestamp.to_string(),
        model,
        input_tokens,
        output_tokens,
        cache_creation_tokens,
        cache_read_tokens,
        cost,
        session_id: session_id.to_string(),
        project_name: project_name.to_string(),
    })
}

fn calculate_cost(
    model: &str,
    input_tokens: u64,
    output_tokens: u64,
    cache_creation_tokens: u64,
    cache_read_tokens: u64,
) -> f64 {
    let (input_price, output_price, cache_write_price, cache_read_price) =
        if model.contains("opus-4") || model.contains("claude-opus-4") {
            (OPUS_4_INPUT_PRICE, OPUS_4_OUTPUT_PRICE, OPUS_4_CACHE_WRITE_PRICE, OPUS_4_CACHE_READ_PRICE)
        } else if model.contains("sonnet-4") || model.contains("claude-sonnet-4") {
            (SONNET_4_INPUT_PRICE, SONNET_4_OUTPUT_PRICE, SONNET_4_CACHE_WRITE_PRICE, SONNET_4_CACHE_READ_PRICE)
        } else {
            (0.0, 0.0, 0.0, 0.0) // Unknown model
        };
    
    // Prices are per million tokens
    (input_tokens as f64 * input_price / 1_000_000.0)
        + (output_tokens as f64 * output_price / 1_000_000.0)
        + (cache_creation_tokens as f64 * cache_write_price / 1_000_000.0)
        + (cache_read_tokens as f64 * cache_read_price / 1_000_000.0)
}

fn calculate_usage_stats(entries: Vec<UsageEntry>) -> Result<UsageStats, String> {
    let mut total_cost = 0.0;
    let mut total_input_tokens = 0u64;
    let mut total_output_tokens = 0u64;
    let mut total_cache_creation_tokens = 0u64;
    let mut total_cache_read_tokens = 0u64;
    let mut sessions = HashSet::new();
    
    let mut by_model: HashMap<String, ModelUsage> = HashMap::new();
    let mut by_date: HashMap<String, DailyUsage> = HashMap::new();
    let mut by_project: HashMap<String, ProjectUsage> = HashMap::new();
    
    for entry in &entries {
        // Update totals
        total_cost += entry.cost;
        total_input_tokens += entry.input_tokens;
        total_output_tokens += entry.output_tokens;
        total_cache_creation_tokens += entry.cache_creation_tokens;
        total_cache_read_tokens += entry.cache_read_tokens;
        sessions.insert(&entry.session_id);
        
        // Update by model
        let model_usage = by_model.entry(entry.model.clone()).or_insert(ModelUsage {
            model: entry.model.clone(),
            total_cost: 0.0,
            total_tokens: 0,
            input_tokens: 0,
            output_tokens: 0,
            cache_creation_tokens: 0,
            cache_read_tokens: 0,
            session_count: 0,
        });
        
        model_usage.total_cost += entry.cost;
        model_usage.input_tokens += entry.input_tokens;
        model_usage.output_tokens += entry.output_tokens;
        model_usage.cache_creation_tokens += entry.cache_creation_tokens;
        model_usage.cache_read_tokens += entry.cache_read_tokens;
        model_usage.total_tokens = model_usage.input_tokens + model_usage.output_tokens 
            + model_usage.cache_creation_tokens + model_usage.cache_read_tokens;
        
        // Update by date
        if let Ok(dt) = DateTime::parse_from_rfc3339(&entry.timestamp) {
            let date_str = dt.format("%Y-%m-%d").to_string();
            let daily = by_date.entry(date_str.clone()).or_insert(DailyUsage {
                date: date_str,
                total_cost: 0.0,
                total_tokens: 0,
                models_used: vec![],
            });
            
            daily.total_cost += entry.cost;
            daily.total_tokens += entry.input_tokens + entry.output_tokens 
                + entry.cache_creation_tokens + entry.cache_read_tokens;
            
            if !daily.models_used.contains(&entry.model) {
                daily.models_used.push(entry.model.clone());
            }
        }
        
        // Update by project
        let project = by_project.entry(entry.project_name.clone()).or_insert(ProjectUsage {
            project_name: entry.project_name.clone(),
            display_name: decode_project_name(&entry.project_name),
            total_cost: 0.0,
            total_tokens: 0,
            session_count: 0,
            last_used: entry.timestamp.clone(),
        });
        
        project.total_cost += entry.cost;
        project.total_tokens += entry.input_tokens + entry.output_tokens 
            + entry.cache_creation_tokens + entry.cache_read_tokens;
        
        if entry.timestamp > project.last_used {
            project.last_used = entry.timestamp.clone();
        }
    }
    
    // Count sessions per model and project
    for entry in &entries {
        if let Some(model_usage) = by_model.get_mut(&entry.model) {
            model_usage.session_count = sessions.len() as u64;
        }
        
        if let Some(project) = by_project.get_mut(&entry.project_name) {
            project.session_count += 1;
        }
    }
    
    // Convert to sorted vectors
    let mut by_model: Vec<ModelUsage> = by_model.into_values().collect();
    by_model.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap());
    
    let mut by_date: Vec<DailyUsage> = by_date.into_values().collect();
    by_date.sort_by(|a, b| a.date.cmp(&b.date));
    
    let mut by_project: Vec<ProjectUsage> = by_project.into_values().collect();
    by_project.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap());
    
    Ok(UsageStats {
        total_cost,
        total_tokens: total_input_tokens + total_output_tokens 
            + total_cache_creation_tokens + total_cache_read_tokens,
        total_input_tokens,
        total_output_tokens,
        total_cache_creation_tokens,
        total_cache_read_tokens,
        total_sessions: sessions.len() as u64,
        by_model,
        by_date,
        by_project,
    })
}

/// Get the actual project directory path from JSONL entries
fn get_actual_project_directory(project_name: &str) -> Option<String> {
    use crate::claude_paths::{list_claude_directory, read_claude_file};
    use serde_json::Value;
    
    // List files in the project directory
    let project_dir = format!("projects/{}", project_name);
    let files = match list_claude_directory(&project_dir) {
        Ok(files) => files,
        Err(e) => {
            log::debug!("Failed to list directory {}: {}", project_dir, e);
            return None;
        }
    };
    
    // Find a JSONL file
    let jsonl_file = files.iter()
        .find(|f| f.ends_with(".jsonl"))?;
    
    let file_path = format!("{}/{}", project_dir, jsonl_file);
    
    // Read the file and look for cwd
    let content = match read_claude_file(&file_path) {
        Ok(content) => content,
        Err(e) => {
            log::debug!("Failed to read {}: {}", file_path, e);
            return None;
        }
    };
    
    // Check first few lines for cwd
    for (i, line) in content.lines().enumerate() {
        if i >= 10 { break; } // Only check first 10 lines
        
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            if let Some(cwd) = json.get("cwd").and_then(|v| v.as_str()) {
                return Some(cwd.to_string());
            }
        }
    }
    
    None
}

fn decode_project_name(encoded: &str) -> String {
    // Try to get the actual project directory from JSONL files
    if let Some(actual_dir) = get_actual_project_directory(encoded) {
        return actual_dir;
    }
    
    // Fallback: Convert encoded directory name to readable format
    // Example: -home-will-local-dev-portfolio -> /home/will/local-dev/portfolio
    // 
    // IMPORTANT: Claude encodes BOTH / and _ as - so we can't perfectly reverse it
    // We just convert all - to / for display purposes
    if encoded.starts_with('-') {
        encoded.replacen('-', "/", 1).replace('-', "/")
    } else {
        encoded.to_string()
    }
}