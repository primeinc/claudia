use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// JSONL Schema Versions based on comprehensive analysis
#[derive(Debug, Clone, PartialEq)]
pub enum JsonlSchemaVersion {
    /// Conversation entries with message, sessionId, timestamp
    ConversationV1,
    /// Summary entries with leafUuid, summary, type
    SummaryV1,
    /// Unknown schema type
    Unknown(String),
}

/// Result of parsing a JSONL line
#[derive(Debug)]
pub enum JsonlEntry {
    Conversation(ConversationEntry),
    Summary(SummaryEntry),
    Unknown(Value),
    Invalid(String, String), // (error, raw_line)
}

/// Conversation entry schema
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConversationEntry {
    pub cwd: Option<String>,
    pub version: String,
    pub is_sidechain: bool,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub parent_uuid: Option<String>,
    pub session_id: String,
    pub uuid: String,
    pub timestamp: String,
    pub user_type: String,
    pub message: MessageData,
    
    // Optional fields
    pub request_id: Option<String>,
    pub is_compact_summary: Option<bool>,
    pub is_api_error_message: Option<bool>,
    pub is_meta: Option<bool>,
    pub tool_use_result: Option<Value>,
    pub leaf_uuid: Option<String>,
    
    // Capture any unknown fields
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Message data within conversation entries
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageData {
    pub role: String,
    #[serde(deserialize_with = "deserialize_content")]
    pub content: MessageContent,
    
    // Optional fields from assistant messages
    pub id: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "type")]
    pub message_type: Option<String>,
    pub stop_reason: Option<Value>,
    pub stop_sequence: Option<Value>,
    pub usage: Option<UsageData>,
}

/// Message content can be string or array of content blocks
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

/// Content block for structured messages
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub tool_use_id: Option<String>,
    pub content: Option<String>,
    pub is_error: Option<bool>,
    pub thinking: Option<String>,
    pub signature: Option<String>,
    pub input: Option<Value>,
}

/// Usage data for token tracking
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UsageData {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
    pub service_tier: Option<String>,
    pub server_tool_use: Option<Value>,
}

/// Summary entry schema
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SummaryEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub summary: String,
    pub leaf_uuid: String,
    
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Defensive JSONL loader
pub struct JsonlLoader {
    /// Track schema versions seen
    schema_stats: HashMap<String, usize>,
    /// Track parsing errors
    error_count: usize,
    /// Strict mode - fail on unknown schemas
    strict: bool,
}

impl JsonlLoader {
    pub fn new(strict: bool) -> Self {
        Self {
            schema_stats: HashMap::new(),
            error_count: 0,
            strict,
        }
    }
    
    /// Parse a single JSONL line
    pub fn parse_line(&mut self, line: &str) -> JsonlEntry {
        // Skip empty lines
        if line.trim().is_empty() {
            return JsonlEntry::Invalid("Empty line".to_string(), line.to_string());
        }
        
        // Try to parse as JSON
        let value: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                self.error_count += 1;
                return JsonlEntry::Invalid(format!("JSON parse error: {}", e), line.to_string());
            }
        };
        
        // Determine schema type
        let schema_type = self.detect_schema(&value);
        
        // Track schema usage
        *self.schema_stats.entry(schema_type.clone()).or_insert(0) += 1;
        
        // Parse based on detected schema
        match schema_type.as_str() {
            "conversation" => {
                match serde_json::from_value::<ConversationEntry>(value.clone()) {
                    Ok(entry) => JsonlEntry::Conversation(entry),
                    Err(e) => {
                        self.error_count += 1;
                        JsonlEntry::Invalid(
                            format!("Failed to parse conversation entry: {}", e),
                            line.to_string()
                        )
                    }
                }
            },
            "summary" => {
                match serde_json::from_value::<SummaryEntry>(value.clone()) {
                    Ok(entry) => JsonlEntry::Summary(entry),
                    Err(e) => {
                        self.error_count += 1;
                        JsonlEntry::Invalid(
                            format!("Failed to parse summary entry: {}", e),
                            line.to_string()
                        )
                    }
                }
            },
            _ => {
                if self.strict {
                    self.error_count += 1;
                    JsonlEntry::Invalid(
                        format!("Unknown schema type: {}", schema_type),
                        line.to_string()
                    )
                } else {
                    JsonlEntry::Unknown(value)
                }
            }
        }
    }
    
    /// Detect schema type from JSON value
    fn detect_schema(&self, value: &Value) -> String {
        if let Some(obj) = value.as_object() {
            // Check for summary entry signature
            if obj.contains_key("leafUuid") && obj.contains_key("summary") && obj.contains_key("type") {
                return "summary".to_string();
            }
            
            // Check for conversation entry signature
            if obj.contains_key("message") && obj.contains_key("sessionId") && obj.contains_key("timestamp") {
                return "conversation".to_string();
            }
            
            // Unknown type - create signature from keys
            let mut keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
            keys.sort();
            format!("unknown_{}", keys.join("_"))
        } else {
            "invalid_not_object".to_string()
        }
    }
    
    /// Get statistics about parsed schemas
    pub fn get_stats(&self) -> &HashMap<String, usize> {
        &self.schema_stats
    }
    
    /// Get error count
    pub fn get_error_count(&self) -> usize {
        self.error_count
    }
}

/// Helper function to deserialize content field
fn deserialize_content<'de, D>(deserializer: D) -> Result<MessageContent, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    
    match value {
        Value::String(s) => Ok(MessageContent::Text(s)),
        Value::Array(arr) => {
            let blocks: Result<Vec<ContentBlock>, _> = arr.into_iter()
                .map(|v| serde_json::from_value(v))
                .collect();
            
            match blocks {
                Ok(b) => Ok(MessageContent::Blocks(b)),
                Err(e) => Err(serde::de::Error::custom(format!("Failed to parse content blocks: {}", e)))
            }
        },
        _ => Err(serde::de::Error::custom("Content must be string or array"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_conversation_entry() {
        let mut loader = JsonlLoader::new(false);
        
        let json = r#"{"cwd":"/home/user","version":"1.0.0","isSidechain":false,"type":"message","parentUuid":null,"sessionId":"abc-123","uuid":"def-456","timestamp":"2024-01-01T00:00:00Z","userType":"human","message":{"role":"user","content":"Hello"}}"#;
        
        match loader.parse_line(json) {
            JsonlEntry::Conversation(entry) => {
                assert_eq!(entry.cwd, Some("/home/user".to_string()));
                assert_eq!(entry.session_id, "abc-123");
                assert!(!entry.is_sidechain);
            },
            _ => panic!("Expected conversation entry")
        }
    }
    
    #[test]
    fn test_parse_summary_entry() {
        let mut loader = JsonlLoader::new(false);
        
        let json = r#"{"type":"summary","summary":"Test summary","leafUuid":"abc-123"}"#;
        
        match loader.parse_line(json) {
            JsonlEntry::Summary(entry) => {
                assert_eq!(entry.summary, "Test summary");
                assert_eq!(entry.leaf_uuid, "abc-123");
            },
            _ => panic!("Expected summary entry")
        }
    }
    
    #[test]
    fn test_parse_invalid_json() {
        let mut loader = JsonlLoader::new(false);
        
        match loader.parse_line("not valid json") {
            JsonlEntry::Invalid(err, _) => {
                assert!(err.contains("JSON parse error"));
            },
            _ => panic!("Expected invalid entry")
        }
    }
}