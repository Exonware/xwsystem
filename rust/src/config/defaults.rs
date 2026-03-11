// #exonware/xwsystem/rust/src/config/defaults.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Default configuration constants for XSystem framework.
//! 
//! These constants provide default values and limits for system operations.
//! All modules should import from this central location to ensure consistency.


use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::config::base::AConfigBase;
use crate::config::defs::ConfigType;

// Core Configuration
pub const DEFAULT_ENCODING: &str = "utf-8";
pub const DEFAULT_PATH_DELIMITER: &str = ".";
pub const DEFAULT_LOCK_TIMEOUT: f64 = 10.0;

// Memory Safety Limits
pub const DEFAULT_MAX_FILE_SIZE_MB: i64 = 100;
pub const DEFAULT_MAX_MEMORY_USAGE_MB: i64 = 500;
pub const DEFAULT_MAX_DICT_DEPTH: i64 = 50;

// Path and Resolution Limits
pub const DEFAULT_MAX_PATH_DEPTH: i64 = 20;
pub const DEFAULT_MAX_PATH_LENGTH: i64 = 500;
pub const DEFAULT_MAX_RESOLUTION_DEPTH: i64 = 10;
pub const DEFAULT_MAX_TO_DICT_SIZE_MB: i64 = 50;

// Data Structure Limits
pub const DEFAULT_MAX_CIRCULAR_DEPTH: i64 = 100;
pub const DEFAULT_MAX_EXTENSION_LENGTH: i64 = 5;
pub const DEFAULT_CONTENT_SNIPPET_LENGTH: i64 = 200;
pub const DEFAULT_MAX_TRAVERSAL_DEPTH: i64 = 100;

// Protocol and Format Identifiers
pub const URI_SCHEME_SEPARATOR: &str = "://";
pub const JSON_POINTER_PREFIX: &str = "#";
pub const PATH_SEPARATOR_FORWARD: &str = "/";
pub const PATH_SEPARATOR_BACKWARD: &str = "\\";

// Placeholder Messages
pub const CIRCULAR_REFERENCE_PLACEHOLDER: &str = "[Circular Reference]";
pub const MAX_DEPTH_EXCEEDED_PLACEHOLDER: &str = "[Max Depth Exceeded]";

// Logging Configuration
pub const LOGGING_ENABLED: bool = true;
pub const LOGGING_LEVEL: &str = "INFO";

/// Default configuration manager for XSystem framework.
///
/// Provides default configuration values and management functionality.
pub struct DefaultConfig {
    config_type: ConfigType,
    config: Arc<RwLock<HashMap<String, serde_json::Value>>>,
    defaults: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl AConfigBase for DefaultConfig {
    fn load(&mut self, source: String) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&source) {
            if let Some(obj) = value.as_object() {
                let mut config = self.config.write().unwrap();
                for (k, v) in obj {
                    config.insert(k.clone(), v.clone());
                }
            }
        }
    }

    fn save(&self, destination: String) {
        let config = self.config.read().unwrap();
        if let Ok(json) = serde_json::to_string_pretty(&*config) {
            let _ = std::fs::write(&destination, json);
        }
    }

    fn get(&self, key: &str, default: Option<serde_json::Value>) -> serde_json::Value {
        let config = self.config.read().unwrap();
        config.get(key).cloned().or(default).unwrap_or_else(|| {
            let defaults = self.defaults.read().unwrap();
            defaults.get(key).cloned().unwrap_or(serde_json::Value::Null)
        })
    }

    fn set(&mut self, key: String, value: serde_json::Value) {
        let mut config = self.config.write().unwrap();
        config.insert(key, value);
    }

    fn delete(&mut self, key: &str) -> bool {
        let mut config = self.config.write().unwrap();
        config.remove(key).is_some()
    }

    fn has(&self, key: &str) -> bool {
        let config = self.config.read().unwrap();
        config.contains_key(key)
    }

    fn keys(&self) -> Vec<String> {
        let config = self.config.read().unwrap();
        config.keys().cloned().collect()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        let config = self.config.read().unwrap();
        config.values().cloned().collect()
    }

    fn items(&self) -> Vec<(String, serde_json::Value)> {
        let config = self.config.read().unwrap();
        config.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    fn clear(&mut self) {
        let mut config = self.config.write().unwrap();
        config.clear();
    }

    fn validate(&self) -> bool {
        let config = self.config.read().unwrap();
        config.values().all(|v| !v.is_null())
    }
}

impl DefaultConfig {
    /// Initialize default configuration.
    pub fn new(config_type: Option<ConfigType>) -> Self {
        let defaults = {
            let mut map = HashMap::new();
            map.insert("encoding".to_string(), serde_json::Value::String(DEFAULT_ENCODING.to_string()));
            map.insert("path_delimiter".to_string(), serde_json::Value::String(DEFAULT_PATH_DELIMITER.to_string()));
            map.insert("lock_timeout".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(DEFAULT_LOCK_TIMEOUT).unwrap()));
            map.insert("max_file_size_mb".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_FILE_SIZE_MB)));
            map.insert("max_memory_usage_mb".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_MEMORY_USAGE_MB)));
            map.insert("max_dict_depth".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_DICT_DEPTH)));
            map.insert("max_path_depth".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_PATH_DEPTH)));
            map.insert("max_path_length".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_PATH_LENGTH)));
            map.insert("max_resolution_depth".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_RESOLUTION_DEPTH)));
            map.insert("max_to_dict_size_mb".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_TO_DICT_SIZE_MB)));
            map.insert("max_circular_depth".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_CIRCULAR_DEPTH)));
            map.insert("max_extension_length".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_EXTENSION_LENGTH)));
            map.insert("content_snippet_length".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_CONTENT_SNIPPET_LENGTH)));
            map.insert("max_traversal_depth".to_string(), serde_json::Value::Number(serde_json::Number::from(DEFAULT_MAX_TRAVERSAL_DEPTH)));
            map.insert("logging_enabled".to_string(), serde_json::Value::Bool(LOGGING_ENABLED));
            map.insert("logging_level".to_string(), serde_json::Value::String(LOGGING_LEVEL.to_string()));
            map
        };
        let config = defaults.clone();
        
        Self {
            config_type: config_type.unwrap_or(ConfigType::Dict),
            config: Arc::new(RwLock::new(config)),
            defaults: Arc::new(RwLock::new(defaults)),
        }
    }

    /// Get default value for key.
    pub fn get_default(&self, key: &str) -> serde_json::Value {
        let defaults = self.defaults.read().unwrap();
        defaults.get(key).cloned().unwrap_or(serde_json::Value::Null)
    }

    /// Set default value for key.
    pub fn set_default(&mut self, key: String, value: serde_json::Value) {
        let mut defaults = self.defaults.write().unwrap();
        defaults.insert(key, value);
    }

    /// Load default configuration values.
    pub fn load_defaults(&mut self) {
        let defaults = self.defaults.read().unwrap();
        let mut config = self.config.write().unwrap();
        *config = defaults.clone();
    }
}
