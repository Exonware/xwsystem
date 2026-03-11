// #exonware/xwsystem/rust/src/config.rs
//exonware/xwsystem/src/exonware/xwsystem/config.py
//! Root-level configuration classes for XWSystem.
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Jan-2025
//! 
//! This module provides root-level configuration classes for XWSystem.


use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::defs::{CoreMode, DEFAULT_CACHE_DIR, DEFAULT_CACHE_SIZE, DEFAULT_CACHE_TTL, DEFAULT_CONFIG_DIR, DEFAULT_CONFIG_FILE, DEFAULT_HTTP_RETRIES, DEFAULT_HTTP_TIMEOUT, DEFAULT_LOG_DIR, DEFAULT_LOG_LEVEL, DEFAULT_MAX_WORKERS, DEFAULT_PLUGIN_TIMEOUT, DEFAULT_RETRY_COUNT, DEFAULT_RETRY_DELAY, DEFAULT_THREAD_POOL_SIZE, DEFAULT_TIMEOUT, LogLevel, PerformanceMode, ValidationLevel};

// ============================================================================

// CONFIGURATION FACTORY

// ============================================================================

// ============================================================================

// ============================================================================

/// Root configuration class for XWSystem.
///
/// Provides centralized configuration for all XWSystem components.
#[derive(Debug, Clone)]
pub struct XWSystemConfig {
    // System configuration
    pub mode: CoreMode,
    pub version: String,
    pub debug: bool,
    pub verbose: bool,
    
    // Paths
    pub config_dir: PathBuf,
    pub log_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub plugin_dir: Option<PathBuf>,
    
    // Performance configuration
    pub performance_mode: PerformanceMode,
    pub max_workers: i64,
    pub thread_pool_size: i64,
    pub timeout: f64,
    
    // Retry configuration
    pub retry_count: i64,
    pub retry_delay: f64,
    
    // Logging configuration
    pub log_level: LogLevel,
    pub log_file: Option<PathBuf>,
    pub log_format: String,
    pub log_date_format: String,
    
    // Validation configuration
    pub validation_level: ValidationLevel,
    pub validate_input: bool,
    pub validate_output: bool,
    
    // Security configuration
    pub security_enabled: bool,
    pub path_validation_enabled: bool,
    pub input_sanitization_enabled: bool,
    
    // Caching configuration
    pub cache_enabled: bool,
    pub cache_size: i64,
    pub cache_ttl: f64,
    
    // HTTP client configuration
    pub http_timeout: f64,
    pub http_retries: i64,
    pub http_verify_ssl: bool,
    
    // Plugin configuration
    pub plugins_enabled: bool,
    pub plugin_timeout: f64,
    pub plugin_sandbox_enabled: bool,
    
    // Monitoring configuration
    pub monitoring_enabled: bool,
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
    
    // IPC configuration
    pub ipc_enabled: bool,
    pub ipc_timeout: f64,
    
    // Custom configuration
    pub custom: HashMap<String, serde_json::Value>,
}

impl XWSystemConfig {
    /// Create a new XWSystemConfig with default values.
    pub fn new() -> Self {
        let config_dir = PathBuf::from(DEFAULT_CONFIG_DIR);
        let log_dir = PathBuf::from(DEFAULT_LOG_DIR);
        let cache_dir = PathBuf::from(DEFAULT_CACHE_DIR);
        let plugin_dir = Some(config_dir.join("plugins"));
        let log_file = Some(log_dir.join("xwsystem.log"));
        
        Self {
            mode: CoreMode::Development,
            version: "0.1.0.1".to_string(),
            debug: false,
            verbose: false,
            config_dir,
            log_dir,
            cache_dir,
            plugin_dir,
            performance_mode: PerformanceMode::Balanced,
            max_workers: DEFAULT_MAX_WORKERS,
            thread_pool_size: DEFAULT_THREAD_POOL_SIZE,
            timeout: DEFAULT_TIMEOUT as f64,
            retry_count: DEFAULT_RETRY_COUNT,
            retry_delay: DEFAULT_RETRY_DELAY,
            log_level: DEFAULT_LOG_LEVEL,
            log_file,
            log_format: "%(asctime)s - %(name)s - %(levelname)s - %(message)s".to_string(),
            log_date_format: "%Y-%m-%d %H:%M:%S".to_string(),
            validation_level: ValidationLevel::Strict,
            validate_input: true,
            validate_output: true,
            security_enabled: true,
            path_validation_enabled: true,
            input_sanitization_enabled: true,
            cache_enabled: true,
            cache_size: DEFAULT_CACHE_SIZE,
            cache_ttl: DEFAULT_CACHE_TTL as f64,
            http_timeout: DEFAULT_HTTP_TIMEOUT as f64,
            http_retries: DEFAULT_HTTP_RETRIES,
            http_verify_ssl: true,
            plugins_enabled: true,
            plugin_timeout: DEFAULT_PLUGIN_TIMEOUT as f64,
            plugin_sandbox_enabled: true,
            monitoring_enabled: true,
            metrics_enabled: true,
            tracing_enabled: false,
            ipc_enabled: true,
            ipc_timeout: DEFAULT_TIMEOUT as f64,
            custom: HashMap::new(),
        }
    }

    /// Convert configuration to dictionary.
    pub fn to_dict(&self) -> HashMap<String, serde_json::Value> {
        use serde_json::json;
        let mut dict = HashMap::new();
        
        dict.insert("mode".to_string(), json!(format!("{:?}", self.mode)));
        dict.insert("version".to_string(), json!(self.version.clone()));
        dict.insert("debug".to_string(), json!(self.debug));
        dict.insert("verbose".to_string(), json!(self.verbose));
        dict.insert("config_dir".to_string(), json!(self.config_dir.to_string_lossy().to_string()));
        dict.insert("log_dir".to_string(), json!(self.log_dir.to_string_lossy().to_string()));
        dict.insert("cache_dir".to_string(), json!(self.cache_dir.to_string_lossy().to_string()));
        dict.insert("plugin_dir".to_string(), json!(self.plugin_dir.as_ref().map(|p| p.to_string_lossy().to_string())));
        dict.insert("performance_mode".to_string(), json!(format!("{:?}", self.performance_mode)));
        dict.insert("max_workers".to_string(), json!(self.max_workers));
        dict.insert("thread_pool_size".to_string(), json!(self.thread_pool_size));
        dict.insert("timeout".to_string(), json!(self.timeout));
        dict.insert("retry_count".to_string(), json!(self.retry_count));
        dict.insert("retry_delay".to_string(), json!(self.retry_delay));
        dict.insert("log_level".to_string(), json!(format!("{:?}", self.log_level)));
        dict.insert("log_file".to_string(), json!(self.log_file.as_ref().map(|p| p.to_string_lossy().to_string())));
        dict.insert("log_format".to_string(), json!(self.log_format.clone()));
        dict.insert("log_date_format".to_string(), json!(self.log_date_format.clone()));
        dict.insert("validation_level".to_string(), json!(format!("{:?}", self.validation_level)));
        dict.insert("validate_input".to_string(), json!(self.validate_input));
        dict.insert("validate_output".to_string(), json!(self.validate_output));
        dict.insert("security_enabled".to_string(), json!(self.security_enabled));
        dict.insert("path_validation_enabled".to_string(), json!(self.path_validation_enabled));
        dict.insert("input_sanitization_enabled".to_string(), json!(self.input_sanitization_enabled));
        dict.insert("cache_enabled".to_string(), json!(self.cache_enabled));
        dict.insert("cache_size".to_string(), json!(self.cache_size));
        dict.insert("cache_ttl".to_string(), json!(self.cache_ttl));
        dict.insert("http_timeout".to_string(), json!(self.http_timeout));
        dict.insert("http_retries".to_string(), json!(self.http_retries));
        dict.insert("http_verify_ssl".to_string(), json!(self.http_verify_ssl));
        dict.insert("plugins_enabled".to_string(), json!(self.plugins_enabled));
        dict.insert("plugin_timeout".to_string(), json!(self.plugin_timeout));
        dict.insert("plugin_sandbox_enabled".to_string(), json!(self.plugin_sandbox_enabled));
        dict.insert("monitoring_enabled".to_string(), json!(self.monitoring_enabled));
        dict.insert("metrics_enabled".to_string(), json!(self.metrics_enabled));
        dict.insert("tracing_enabled".to_string(), json!(self.tracing_enabled));
        dict.insert("ipc_enabled".to_string(), json!(self.ipc_enabled));
        dict.insert("ipc_timeout".to_string(), json!(self.ipc_timeout));
        dict.insert("custom".to_string(), json!(self.custom));
        
        dict
    }

    /// Create configuration from dictionary.
    pub fn from_dict(data: &HashMap<String, serde_json::Value>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::new();
        
        // Helper to get string value
        let get_str = |key: &str| -> Option<String> {
            data.get(key)?.as_str().map(|s| s.to_string())
        };
        
        // Helper to get enum value
        if let Some(mode_str) = get_str("mode") {
            // Try to parse CoreMode from string
            // This is a simplified version - you may need to implement proper parsing
            config.mode = CoreMode::Development; // Default, implement proper parsing
        }
        
        if let Some(version) = get_str("version") {
            config.version = version;
        }
        
        if let Some(debug) = data.get("debug").and_then(|v| v.as_bool()) {
            config.debug = debug;
        }
        
        if let Some(verbose) = data.get("verbose").and_then(|v| v.as_bool()) {
            config.verbose = verbose;
        }
        
        if let Some(dir) = get_str("config_dir") {
            config.config_dir = PathBuf::from(dir);
        }
        
        if let Some(dir) = get_str("log_dir") {
            config.log_dir = PathBuf::from(dir);
        }
        
        if let Some(dir) = get_str("cache_dir") {
            config.cache_dir = PathBuf::from(dir);
        }
        
        if let Some(dir) = get_str("plugin_dir") {
            config.plugin_dir = Some(PathBuf::from(dir));
        }
        
        if let Some(workers) = data.get("max_workers").and_then(|v| v.as_i64()) {
            config.max_workers = workers;
        }
        
        if let Some(size) = data.get("thread_pool_size").and_then(|v| v.as_i64()) {
            config.thread_pool_size = size;
        }
        
        if let Some(timeout) = data.get("timeout").and_then(|v| v.as_f64()) {
            config.timeout = timeout;
        }
        
        if let Some(count) = data.get("retry_count").and_then(|v| v.as_i64()) {
            config.retry_count = count;
        }
        
        if let Some(delay) = data.get("retry_delay").and_then(|v| v.as_f64()) {
            config.retry_delay = delay;
        }
        
        if let Some(format) = get_str("log_format") {
            config.log_format = format;
        }
        
        if let Some(format) = get_str("log_date_format") {
            config.log_date_format = format;
        }
        
        if let Some(file) = get_str("log_file") {
            config.log_file = Some(PathBuf::from(file));
        }
        
        if let Some(enabled) = data.get("validate_input").and_then(|v| v.as_bool()) {
            config.validate_input = enabled;
        }
        
        if let Some(enabled) = data.get("validate_output").and_then(|v| v.as_bool()) {
            config.validate_output = enabled;
        }
        
        if let Some(enabled) = data.get("security_enabled").and_then(|v| v.as_bool()) {
            config.security_enabled = enabled;
        }
        
        if let Some(enabled) = data.get("path_validation_enabled").and_then(|v| v.as_bool()) {
            config.path_validation_enabled = enabled;
        }
        
        if let Some(enabled) = data.get("input_sanitization_enabled").and_then(|v| v.as_bool()) {
            config.input_sanitization_enabled = enabled;
        }
        
        if let Some(enabled) = data.get("cache_enabled").and_then(|v| v.as_bool()) {
            config.cache_enabled = enabled;
        }
        
        if let Some(size) = data.get("cache_size").and_then(|v| v.as_i64()) {
            config.cache_size = size;
        }
        
        if let Some(ttl) = data.get("cache_ttl").and_then(|v| v.as_f64()) {
            config.cache_ttl = ttl;
        }
        
        if let Some(timeout) = data.get("http_timeout").and_then(|v| v.as_f64()) {
            config.http_timeout = timeout;
        }
        
        if let Some(retries) = data.get("http_retries").and_then(|v| v.as_i64()) {
            config.http_retries = retries;
        }
        
        if let Some(verify) = data.get("http_verify_ssl").and_then(|v| v.as_bool()) {
            config.http_verify_ssl = verify;
        }
        
        if let Some(enabled) = data.get("plugins_enabled").and_then(|v| v.as_bool()) {
            config.plugins_enabled = enabled;
        }
        
        if let Some(timeout) = data.get("plugin_timeout").and_then(|v| v.as_f64()) {
            config.plugin_timeout = timeout;
        }
        
        if let Some(enabled) = data.get("plugin_sandbox_enabled").and_then(|v| v.as_bool()) {
            config.plugin_sandbox_enabled = enabled;
        }
        
        if let Some(enabled) = data.get("monitoring_enabled").and_then(|v| v.as_bool()) {
            config.monitoring_enabled = enabled;
        }
        
        if let Some(enabled) = data.get("metrics_enabled").and_then(|v| v.as_bool()) {
            config.metrics_enabled = enabled;
        }
        
        if let Some(enabled) = data.get("tracing_enabled").and_then(|v| v.as_bool()) {
            config.tracing_enabled = enabled;
        }
        
        if let Some(enabled) = data.get("ipc_enabled").and_then(|v| v.as_bool()) {
            config.ipc_enabled = enabled;
        }
        
        if let Some(timeout) = data.get("ipc_timeout").and_then(|v| v.as_f64()) {
            config.ipc_timeout = timeout;
        }
        
        if let Some(custom) = data.get("custom").and_then(|v| v.as_object()) {
            for (key, value) in custom {
                config.custom.insert(key.clone(), value.clone());
            }
        }
        
        Ok(config)
    }

    /// Update configuration with new values.
    pub fn update(&mut self, kwargs: &HashMap<String, serde_json::Value>) {
        for (key, value) in kwargs {
            match key.as_str() {
                "mode" => {
                    // Parse mode if string, otherwise skip
                }
                "version" => {
                    if let Some(v) = value.as_str() {
                        self.version = v.to_string();
                    }
                }
                "debug" => {
                    if let Some(v) = value.as_bool() {
                        self.debug = v;
                    }
                }
                "verbose" => {
                    if let Some(v) = value.as_bool() {
                        self.verbose = v;
                    }
                }
                "config_dir" => {
                    if let Some(v) = value.as_str() {
                        self.config_dir = PathBuf::from(v);
                    }
                }
                "log_dir" => {
                    if let Some(v) = value.as_str() {
                        self.log_dir = PathBuf::from(v);
                    }
                }
                "cache_dir" => {
                    if let Some(v) = value.as_str() {
                        self.cache_dir = PathBuf::from(v);
                    }
                }
                "plugin_dir" => {
                    if let Some(v) = value.as_str() {
                        self.plugin_dir = Some(PathBuf::from(v));
                    }
                }
                "max_workers" => {
                    if let Some(v) = value.as_i64() {
                        self.max_workers = v;
                    }
                }
                "thread_pool_size" => {
                    if let Some(v) = value.as_i64() {
                        self.thread_pool_size = v;
                    }
                }
                "timeout" => {
                    if let Some(v) = value.as_f64() {
                        self.timeout = v;
                    }
                }
                "retry_count" => {
                    if let Some(v) = value.as_i64() {
                        self.retry_count = v;
                    }
                }
                "retry_delay" => {
                    if let Some(v) = value.as_f64() {
                        self.retry_delay = v;
                    }
                }
                "log_level" => {
                    // Parse log level if needed
                }
                "log_file" => {
                    if let Some(v) = value.as_str() {
                        self.log_file = Some(PathBuf::from(v));
                    }
                }
                "log_format" => {
                    if let Some(v) = value.as_str() {
                        self.log_format = v.to_string();
                    }
                }
                "log_date_format" => {
                    if let Some(v) = value.as_str() {
                        self.log_date_format = v.to_string();
                    }
                }
                "validate_input" => {
                    if let Some(v) = value.as_bool() {
                        self.validate_input = v;
                    }
                }
                "validate_output" => {
                    if let Some(v) = value.as_bool() {
                        self.validate_output = v;
                    }
                }
                "security_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.security_enabled = v;
                    }
                }
                "path_validation_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.path_validation_enabled = v;
                    }
                }
                "input_sanitization_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.input_sanitization_enabled = v;
                    }
                }
                "cache_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.cache_enabled = v;
                    }
                }
                "cache_size" => {
                    if let Some(v) = value.as_i64() {
                        self.cache_size = v;
                    }
                }
                "cache_ttl" => {
                    if let Some(v) = value.as_f64() {
                        self.cache_ttl = v;
                    }
                }
                "http_timeout" => {
                    if let Some(v) = value.as_f64() {
                        self.http_timeout = v;
                    }
                }
                "http_retries" => {
                    if let Some(v) = value.as_i64() {
                        self.http_retries = v;
                    }
                }
                "http_verify_ssl" => {
                    if let Some(v) = value.as_bool() {
                        self.http_verify_ssl = v;
                    }
                }
                "plugins_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.plugins_enabled = v;
                    }
                }
                "plugin_timeout" => {
                    if let Some(v) = value.as_f64() {
                        self.plugin_timeout = v;
                    }
                }
                "plugin_sandbox_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.plugin_sandbox_enabled = v;
                    }
                }
                "monitoring_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.monitoring_enabled = v;
                    }
                }
                "metrics_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.metrics_enabled = v;
                    }
                }
                "tracing_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.tracing_enabled = v;
                    }
                }
                "ipc_enabled" => {
                    if let Some(v) = value.as_bool() {
                        self.ipc_enabled = v;
                    }
                }
                "ipc_timeout" => {
                    if let Some(v) = value.as_f64() {
                        self.ipc_timeout = v;
                    }
                }
                _ => {
                    // Store in custom
                    self.custom.insert(key.clone(), value.clone());
                }
            }
        }
    }

    /// Get configuration value.
    pub fn get(&self, key: &str, default: Option<serde_json::Value>) -> serde_json::Value {
        match key {
            "mode" => serde_json::Value::String(format!("{:?}", self.mode)),
            "version" => serde_json::Value::String(self.version.clone()),
            "debug" => serde_json::Value::Bool(self.debug),
            "verbose" => serde_json::Value::Bool(self.verbose),
            "config_dir" => serde_json::Value::String(self.config_dir.to_string_lossy().to_string()),
            "log_dir" => serde_json::Value::String(self.log_dir.to_string_lossy().to_string()),
            "cache_dir" => serde_json::Value::String(self.cache_dir.to_string_lossy().to_string()),
            "plugin_dir" => serde_json::Value::String(self.plugin_dir.as_ref().map(|p| p.to_string_lossy().to_string()).unwrap_or_default()),
            "performance_mode" => serde_json::Value::String(format!("{:?}", self.performance_mode)),
            "max_workers" => serde_json::Value::Number(self.max_workers.into()),
            "thread_pool_size" => serde_json::Value::Number(self.thread_pool_size.into()),
            "timeout" => serde_json::Value::Number(serde_json::Number::from_f64(self.timeout).unwrap()),
            "retry_count" => serde_json::Value::Number(self.retry_count.into()),
            "retry_delay" => serde_json::Value::Number(serde_json::Number::from_f64(self.retry_delay).unwrap()),
            "log_level" => serde_json::Value::String(format!("{:?}", self.log_level)),
            "log_file" => serde_json::Value::String(self.log_file.as_ref().map(|p| p.to_string_lossy().to_string()).unwrap_or_default()),
            "log_format" => serde_json::Value::String(self.log_format.clone()),
            "log_date_format" => serde_json::Value::String(self.log_date_format.clone()),
            "validation_level" => serde_json::Value::String(format!("{:?}", self.validation_level)),
            "validate_input" => serde_json::Value::Bool(self.validate_input),
            "validate_output" => serde_json::Value::Bool(self.validate_output),
            "security_enabled" => serde_json::Value::Bool(self.security_enabled),
            "path_validation_enabled" => serde_json::Value::Bool(self.path_validation_enabled),
            "input_sanitization_enabled" => serde_json::Value::Bool(self.input_sanitization_enabled),
            "cache_enabled" => serde_json::Value::Bool(self.cache_enabled),
            "cache_size" => serde_json::Value::Number(self.cache_size.into()),
            "cache_ttl" => serde_json::Value::Number(serde_json::Number::from_f64(self.cache_ttl).unwrap()),
            "http_timeout" => serde_json::Value::Number(serde_json::Number::from_f64(self.http_timeout).unwrap()),
            "http_retries" => serde_json::Value::Number(self.http_retries.into()),
            "http_verify_ssl" => serde_json::Value::Bool(self.http_verify_ssl),
            "plugins_enabled" => serde_json::Value::Bool(self.plugins_enabled),
            "plugin_timeout" => serde_json::Value::Number(serde_json::Number::from_f64(self.plugin_timeout).unwrap()),
            "plugin_sandbox_enabled" => serde_json::Value::Bool(self.plugin_sandbox_enabled),
            "monitoring_enabled" => serde_json::Value::Bool(self.monitoring_enabled),
            "metrics_enabled" => serde_json::Value::Bool(self.metrics_enabled),
            "tracing_enabled" => serde_json::Value::Bool(self.tracing_enabled),
            "ipc_enabled" => serde_json::Value::Bool(self.ipc_enabled),
            "ipc_timeout" => serde_json::Value::Number(serde_json::Number::from_f64(self.ipc_timeout).unwrap()),
            _ => {
                // Check custom
                self.custom.get(key).cloned().unwrap_or_else(|| {
                    default.unwrap_or(serde_json::Value::Null)
                })
            }
        }
    }

    /// Set configuration value.
    pub fn set(&mut self, key: &str, value: serde_json::Value) {
        match key {
            "version" => {
                if let Some(v) = value.as_str() {
                    self.version = v.to_string();
                }
            }
            "debug" => {
                if let Some(v) = value.as_bool() {
                    self.debug = v;
                }
            }
            "verbose" => {
                if let Some(v) = value.as_bool() {
                    self.verbose = v;
                }
            }
            "config_dir" => {
                if let Some(v) = value.as_str() {
                    self.config_dir = PathBuf::from(v);
                }
            }
            "log_dir" => {
                if let Some(v) = value.as_str() {
                    self.log_dir = PathBuf::from(v);
                }
            }
            "cache_dir" => {
                if let Some(v) = value.as_str() {
                    self.cache_dir = PathBuf::from(v);
                }
            }
            "plugin_dir" => {
                if let Some(v) = value.as_str() {
                    self.plugin_dir = Some(PathBuf::from(v));
                }
            }
            "max_workers" => {
                if let Some(v) = value.as_i64() {
                    self.max_workers = v;
                }
            }
            "thread_pool_size" => {
                if let Some(v) = value.as_i64() {
                    self.thread_pool_size = v;
                }
            }
            "timeout" => {
                if let Some(v) = value.as_f64() {
                    self.timeout = v;
                }
            }
            "retry_count" => {
                if let Some(v) = value.as_i64() {
                    self.retry_count = v;
                }
            }
            "retry_delay" => {
                if let Some(v) = value.as_f64() {
                    self.retry_delay = v;
                }
            }
            "log_file" => {
                if let Some(v) = value.as_str() {
                    self.log_file = Some(PathBuf::from(v));
                }
            }
            "log_format" => {
                if let Some(v) = value.as_str() {
                    self.log_format = v.to_string();
                }
            }
            "log_date_format" => {
                if let Some(v) = value.as_str() {
                    self.log_date_format = v.to_string();
                }
            }
            "validate_input" => {
                if let Some(v) = value.as_bool() {
                    self.validate_input = v;
                }
            }
            "validate_output" => {
                if let Some(v) = value.as_bool() {
                    self.validate_output = v;
                }
            }
            "security_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.security_enabled = v;
                }
            }
            "path_validation_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.path_validation_enabled = v;
                }
            }
            "input_sanitization_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.input_sanitization_enabled = v;
                }
            }
            "cache_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.cache_enabled = v;
                }
            }
            "cache_size" => {
                if let Some(v) = value.as_i64() {
                    self.cache_size = v;
                }
            }
            "cache_ttl" => {
                if let Some(v) = value.as_f64() {
                    self.cache_ttl = v;
                }
            }
            "http_timeout" => {
                if let Some(v) = value.as_f64() {
                    self.http_timeout = v;
                }
            }
            "http_retries" => {
                if let Some(v) = value.as_i64() {
                    self.http_retries = v;
                }
            }
            "http_verify_ssl" => {
                if let Some(v) = value.as_bool() {
                    self.http_verify_ssl = v;
                }
            }
            "plugins_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.plugins_enabled = v;
                }
            }
            "plugin_timeout" => {
                if let Some(v) = value.as_f64() {
                    self.plugin_timeout = v;
                }
            }
            "plugin_sandbox_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.plugin_sandbox_enabled = v;
                }
            }
            "monitoring_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.monitoring_enabled = v;
                }
            }
            "metrics_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.metrics_enabled = v;
                }
            }
            "tracing_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.tracing_enabled = v;
                }
            }
            "ipc_enabled" => {
                if let Some(v) = value.as_bool() {
                    self.ipc_enabled = v;
                }
            }
            "ipc_timeout" => {
                if let Some(v) = value.as_f64() {
                    self.ipc_timeout = v;
                }
            }
            _ => {
                // Store in custom
                self.custom.insert(key.to_string(), value);
            }
        }
    }
}

impl Default for XWSystemConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Create XWSystem configuration.
///
/// Args:
/// mode: System mode (optional)
/// kwargs: Additional configuration options
///
/// Returns:
/// XWSystemConfig instance
pub fn create_config(mode: Option<CoreMode>, kwargs: HashMap<String, serde_json::Value>) -> XWSystemConfig {
    let mut config = XWSystemConfig::new();
    
    if let Some(m) = mode {
        config.mode = m;
    }
    
    if !kwargs.is_empty() {
        config.update(&kwargs);
    }
    
    config
}

/// Load configuration from file.
///
/// Args:
/// file_path: Path to configuration file
///
/// Returns:
/// XWSystemConfig instance
pub fn load_config_from_file(file_path: &Path) -> Result<XWSystemConfig, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file_path)?;
    let data: HashMap<String, serde_json::Value> = serde_json::from_str(&contents)?;
    XWSystemConfig::from_dict(&data)
}

/// Save configuration to file.
///
/// Args:
/// config: Configuration to save
/// file_path: Path to save configuration file
pub fn save_config_to_file(config: &XWSystemConfig, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let json = serde_json::to_string_pretty(&config.to_dict())?;
    std::fs::write(file_path, json)?;
    Ok(())
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use XWSystemConfig;
pub use create_config;
pub use load_config_from_file;
pub use save_config_to_file;
