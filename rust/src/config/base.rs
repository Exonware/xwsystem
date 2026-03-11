// #exonware/xwsystem/rust/src/config/base.rs
//exonware/xwsystem/config/base.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Config module base classes - abstract classes for configuration functionality.


use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::config::defs::{ConfigType, PerformanceMode};
use crate::shared::defs::LogLevel;

// Concrete implementation for backward compatibility

/// Abstract base class for configuration management.
pub trait AConfigBase {
    /// Load configuration from source.
    fn load(&mut self, source: String);

    /// Save configuration to destination.
    fn save(&self, destination: String);

    /// Get configuration value.
    fn get(&self, key: &str, default: Option<serde_json::Value>) -> serde_json::Value;

    /// Set configuration value.
    fn set(&mut self, key: String, value: serde_json::Value);

    /// Delete configuration key.
    fn delete(&mut self, key: &str) -> bool;

    /// Check if configuration key exists.
    fn has(&self, key: &str) -> bool;

    /// Get all configuration keys.
    fn keys(&self) -> Vec<String>;

    /// Get all configuration values.
    fn values(&self) -> Vec<serde_json::Value>;

    /// Get all configuration items.
    fn items(&self) -> Vec<(String, serde_json::Value)>;

    /// Clear all configuration.
    fn clear(&mut self);

    /// Validate configuration.
    fn validate(&self) -> bool;
}

/// Abstract base class for logging configuration.
pub trait ALoggingConfigBase {
    /// Setup logging configuration.
    fn setup_logging(&mut self, level: LogLevel);

    /// Get logger instance.
    fn get_logger(&self, name: &str) -> serde_json::Value;

    /// Set logging level.
    fn set_level(&mut self, level: LogLevel);

    /// Add logging handler.
    fn add_handler(&mut self, handler: serde_json::Value);

    /// Remove logging handler.
    fn remove_handler(&mut self, handler: serde_json::Value);

    /// Configure log formatter.
    fn configure_formatter(&mut self, format_string: String);
}

/// Abstract base class for performance configuration.
pub trait APerformanceConfigBase {
    /// Set performance mode.
    fn set_mode(&mut self, mode: PerformanceMode);

    /// Get performance setting.
    fn get_setting(&self, key: &str, default: Option<serde_json::Value>) -> serde_json::Value;

    /// Set performance setting.
    fn set_setting(&mut self, key: String, value: serde_json::Value);

    /// Optimize settings for current mode.
    fn optimize_for_mode(&mut self);

    /// Get benchmark configuration.
    fn get_benchmark_config(&self) -> HashMap<String, serde_json::Value>;

    /// Validate performance settings.
    fn validate_settings(&self) -> bool;
}

/// Abstract base class for configuration validation.
pub trait AConfigValidatorBase {
    /// Validate configuration dictionary.
    fn validate_config(&self, config: &HashMap<String, serde_json::Value>) -> bool;

    /// Validate configuration key-value pair.
    fn validate_key(&self, key: &str, value: &serde_json::Value) -> bool;

    /// Get validation errors.
    fn get_validation_errors(&self) -> Vec<String>;

    /// Clear validation errors.
    fn clear_errors(&mut self);

    /// Add validation rule for key.
    fn add_validation_rule(&mut self, key: String, rule: String);

    /// Remove validation rule for key.
    fn remove_validation_rule(&mut self, key: &str);
}

/// Abstract base class for configuration management.
pub trait AConfigManagerBase {
    /// Create new configuration instance.
    fn create_config(&mut self, name: String, config_type: ConfigType) -> Box<dyn AConfigBase>;

    /// Get configuration instance by name.
    fn get_config(&self, name: &str) -> Option<Box<dyn AConfigBase>>;

    /// Remove configuration instance.
    fn remove_config(&mut self, name: &str) -> bool;

    /// List all configuration names.
    fn list_configs(&self) -> Vec<String>;

    /// Backup configuration.
    fn backup_config(&self, name: &str, backup_path: &str);

    /// Restore configuration from backup.
    fn restore_config(&mut self, name: &str, backup_path: &str);
}

/// Concrete implementation of AConfigBase for backward compatibility.
pub struct BaseConfig {
    config_type: ConfigType,
    config: Arc<RwLock<HashMap<String, serde_json::Value>>>,
    defaults: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl AConfigBase for BaseConfig {
    fn load(&mut self, source: String) {
        // Try to parse as JSON first
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&source) {
            if let Some(obj) = value.as_object() {
                let mut config = self.config.write().unwrap();
                for (k, v) in obj {
                    config.insert(k.clone(), v.clone());
                }
            }
        }
        // Could implement file loading here if source is a file path
    }

    fn save(&self, destination: String) {
        // Could implement file saving here
        let config = self.config.read().unwrap();
        if let Ok(json) = serde_json::to_string_pretty(&*config) {
            if let Err(e) = std::fs::write(&destination, json) {
                eprintln!("Failed to save config to {}: {}", destination, e);
            }
        }
    }

    fn get(&self, key: &str, default: Option<serde_json::Value>) -> serde_json::Value {
        let config = self.config.read().unwrap();
        config.get(key).cloned().or(default).unwrap_or(serde_json::Value::Null)
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

impl BaseConfig {
    /// Create a new BaseConfig instance.
    pub fn new(config_type: Option<ConfigType>) -> Self {
        Self {
            config_type: config_type.unwrap_or(ConfigType::Dict),
            config: Arc::new(RwLock::new(HashMap::new())),
            defaults: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get the configuration type.
    pub fn config_type(&self) -> ConfigType {
        self.config_type
    }
}
