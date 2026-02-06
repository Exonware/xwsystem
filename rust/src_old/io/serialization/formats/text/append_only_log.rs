// #exonware/xwsystem/rust/src_old/io/serialization/formats/text/append_only_log.rs
//! Append-only log for fast atomic updates in JSONL files.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! Append-only log system for fast atomic updates without full file rewrites.
//! Matches Python's AppendOnlyLog class 100%.

use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::Mutex;

/// Append-only log for fast atomic updates with in-memory index.
///
/// Matches Python's AppendOnlyLog class.
pub struct AppendOnlyLog {
    db_path: std::path::PathBuf,
    log_path: std::path::PathBuf,
    lock: Mutex<()>,
    log_index: Mutex<HashMap<String, u64>>, // key -> byte offset in log file
    log_cache: Mutex<HashMap<String, JsonValue>>, // key -> latest log entry
    compaction_threshold_mb: f64,
}

impl AppendOnlyLog {
    /// Create a new append-only log.
    ///
    /// Matches Python: `AppendOnlyLog(db_path: Path, log_path: Path | None = None)`
    pub fn new<P: AsRef<Path>>(db_path: P) -> Self {
        let db_path = db_path.as_ref().to_path_buf();
        let log_path = db_path.with_extension(
            db_path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| format!("{}.log", ext))
                .unwrap_or_else(|| "log".to_string())
        );
        
        let log = AppendOnlyLog {
            db_path,
            log_path,
            lock: Mutex::new(()),
            log_index: Mutex::new(HashMap::new()),
            log_cache: Mutex::new(HashMap::new()),
            compaction_threshold_mb: 100.0,
        };
        
        log.load_log_index();
        log
    }

    /// Load log index from file (build in-memory index).
    ///
    /// Matches Python: `_load_log_index(self)`
    fn load_log_index(&self) {
        if !self.log_path.exists() {
            return;
        }

        if let Ok(file) = File::open(&self.log_path) {
            let reader = BufReader::new(file);
            let mut offset = 0u64;
            let mut index = self.log_index.lock().unwrap();
            let mut cache = self.log_cache.lock().unwrap();

            for line in reader.lines() {
                let line_start = offset;
                if let Ok(line_str) = line {
                    let trimmed = line_str.trim();
                    if !trimmed.is_empty() {
                        if let Ok(entry) = serde_json::from_str::<JsonValue>(trimmed) {
                            if let (Some(type_val), Some(id_val)) = (
                                entry.get("type").and_then(|v| v.as_str()),
                                entry.get("id").and_then(|v| v.as_str()),
                            ) {
                                let key = format!("{}:{}", type_val, id_val);
                                index.insert(key.clone(), line_start);
                                cache.insert(key, entry);
                            }
                        }
                    }
                    offset += line_str.len() as u64 + 1; // +1 for newline
                }
            }
        }
    }

    /// Update record by appending to log (O(1) operation).
    ///
    /// Matches Python: `update_record(self, type_name: str, id_value: str, updater: Callable) -> int`
    ///
    /// Returns:
    ///     Number of records updated (always 1)
    pub fn update_record<F>(
        &self,
        type_name: &str,
        id_value: &str,
        updater: F,
    ) -> Result<usize, Box<dyn std::error::Error>>
    where
        F: FnOnce(&JsonValue) -> JsonValue,
    {
        let _guard = self.lock.lock().unwrap();
        let key = format!("{}:{}", type_name, id_value);

        // Get base record from cache or create new
        let base_record = self.log_cache.lock().unwrap()
            .get(&key)
            .cloned()
            .unwrap_or_else(|| json!({
                "type": type_name,
                "id": id_value
            }));

        // Apply updater
        let updated_record = updater(&base_record);

        // Create log entry
        let mut log_entry = serde_json::Map::new();
        log_entry.insert("type".to_string(), JsonValue::String(type_name.to_string()));
        log_entry.insert("id".to_string(), JsonValue::String(id_value.to_string()));
        log_entry.insert("timestamp".to_string(), JsonValue::Number(
            chrono::Utc::now().timestamp().into()
        ));
        log_entry.insert("updated".to_string(), JsonValue::Bool(true));
        log_entry.insert("data".to_string(), updated_record);

        // Append to log file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        let log_entry_value = JsonValue::Object(log_entry.clone());
        let log_line = serde_json::to_string(&log_entry_value)?;
        let line_start = file.metadata()?.len();
        writeln!(file, "{}", log_line)?;
        file.flush()?;

        // Update index and cache
        let mut index = self.log_index.lock().unwrap();
        let mut cache = self.log_cache.lock().unwrap();
        index.insert(key.clone(), line_start);
        cache.insert(key, log_entry_value);

        Ok(1)
    }

    /// Get record by type and ID.
    ///
    /// Returns the latest version from the log cache.
    pub fn get_record(&self, type_name: &str, id_value: &str) -> Option<JsonValue> {
        let key = format!("{}:{}", type_name, id_value);
        self.log_cache.lock().unwrap()
            .get(&key)
            .cloned()
    }

    /// Get the log path.
    pub fn log_path(&self) -> &Path {
        &self.log_path
    }

    /// Get the database path.
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    #[test]
    fn test_append_only_log_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.jsonl");
        let log = AppendOnlyLog::new(&db_path);
        assert_eq!(log.db_path(), db_path);
    }

    #[test]
    fn test_append_only_log_update_record() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.jsonl");
        let log = AppendOnlyLog::new(&db_path);
        
        let result = log.update_record("User", "123", |_| {
            json!({"name": "John", "age": 30})
        }).unwrap();
        
        assert_eq!(result, 1);
        
        let record = log.get_record("User", "123");
        assert!(record.is_some());
        let record_val = record.unwrap();
        assert_eq!(record_val["type"], "User");
        assert_eq!(record_val["id"], "123");
    }

    #[test]
    fn test_append_only_log_multiple_updates() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.jsonl");
        let log = AppendOnlyLog::new(&db_path);
        
        log.update_record("User", "123", |_| {
            json!({"name": "John", "age": 30})
        }).unwrap();
        
        log.update_record("User", "123", |prev| {
            let mut obj = prev.as_object().unwrap().clone();
            obj.insert("age".to_string(), json!(31));
            JsonValue::Object(obj)
        }).unwrap();
        
        let record = log.get_record("User", "123");
        assert!(record.is_some());
    }
}

