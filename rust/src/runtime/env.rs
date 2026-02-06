// #exonware/xwsystem/rust/src/runtime/env.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Environment management utilities for runtime configuration and detection.


use std::collections::HashMap;

use crate::config::logging_setup::{get_logger};
use std::path::{Path};

// Global instance for convenience

// Global instance for convenience

/// Comprehensive environment management for runtime configuration,
/// platform detection, and environment variable handling.
pub struct EnvironmentManager {
    _cache: HashMap<String, serde_json::Value>,
    _env_vars: HashMap<String, String>,
}

impl EnvironmentManager {
    /// Initialize environment manager.
    pub fn new() -> Self {
        let mut env_vars = HashMap::new();
        for (key, value) in std::env::vars() {
            env_vars.insert(key, value);
        }
        Self {
            _cache: HashMap::new(),
            _env_vars: env_vars,
        }
    }

    /// Get comprehensive platform information.
    pub fn platform_info(&mut self) -> HashMap<String, String> {
        if !self._cache.contains_key("platform_info") {
            let mut info = HashMap::new();
            info.insert("system".to_string(), std::env::consts::OS.to_string());
            info.insert("arch".to_string(), std::env::consts::ARCH.to_string());
            info.insert("family".to_string(), std::env::consts::FAMILY.to_string());
            self._cache.insert("platform_info".to_string(), serde_json::to_value(&info).unwrap());
        }
        serde_json::from_value(self._cache["platform_info"].clone()).unwrap_or_default()
    }

    /// Check if running on Windows.
    pub fn is_windows(&self) -> bool {
        std::env::consts::OS == "windows"
    }

    /// Check if running on Linux.
    pub fn is_linux(&self) -> bool {
        std::env::consts::OS == "linux"
    }

    /// Check if running on macOS.
    pub fn is_macos(&self) -> bool {
        std::env::consts::OS == "macos"
    }

    /// Check if running on 64-bit architecture.
    pub fn is_64bit(&self) -> bool {
        std::env::consts::ARCH.contains("64")
    }

    /// Get Python runtime information.
    pub fn python_info(&self) -> HashMap<String, serde_json::Value> {
        // In Rust, we don't have Python runtime info
        // This would be populated by a Python bridge if needed
        HashMap::new()
    }

    /// Get environment variable with optional default.
    pub fn get_env(&self, key: &str, default: Option<&str>) -> Option<String> {
        self._env_vars.get(key).cloned().or_else(|| default.map(|s| s.to_string()))
    }

    /// Get environment variable as boolean.
    pub fn get_env_bool(&self, key: &str, default: Option<bool>) -> bool {
        let value = self.get_env(key, None);
        if let Some(v) = value {
            let lower = v.to_lowercase();
            matches!(lower.as_str(), "true" | "1" | "yes" | "on" | "enabled")
        } else {
            default.unwrap_or(false)
        }
    }

    /// Get environment variable as integer.
    pub fn get_env_int(&self, key: &str, default: Option<i64>) -> i64 {
        let value = self.get_env(key, None);
        if let Some(v) = value {
            v.parse().unwrap_or_else(|_| {
                default.unwrap_or(0)
            })
        } else {
            default.unwrap_or(0)
        }
    }

    /// Get environment variable as float.
    pub fn get_env_float(&self, key: &str, default: Option<f64>) -> f64 {
        let value = self.get_env(key, None);
        if let Some(v) = value {
            v.parse().unwrap_or_else(|_| {
                default.unwrap_or(0.0)
            })
        } else {
            default.unwrap_or(0.0)
        }
    }

    /// Get environment variable as list.
    pub fn get_env_list(&self, key: &str, separator: Option<&str>, default: Option<Vec<String>>) -> Vec<String> {
        let value = self.get_env(key, None);
        if let Some(v) = value {
            let sep = separator.unwrap_or(",");
            v.split(sep)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            default.unwrap_or_default()
        }
    }

    /// Set environment variable.
    pub fn set_env(&mut self, key: String, value: String) {
        std::env::set_var(&key, &value);
        self._env_vars.insert(key, value);
    }

    /// Unset environment variable.
    pub fn unset_env(&mut self, key: &str) -> bool {
        if self._env_vars.contains_key(key) {
            std::env::remove_var(key);
            self._env_vars.remove(key);
            true
        } else {
            false
        }
    }

    /// Get user home directory.
    pub fn get_user_home(&self) -> Result<std::path::PathBuf, std::io::Error> {
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map(|s| std::path::PathBuf::from(s))
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Cannot determine user home directory"
            ))
    }

    /// Get user configuration directory.
    pub fn get_user_config_dir(&self, app_name: Option<&str>) -> Result<std::path::PathBuf, std::io::Error> {
        let home = self.get_user_home()?;
        let config_dir = if self.is_windows() {
            home.join("AppData").join("Roaming")
        } else if self.is_macos() {
            home.join("Library").join("Application Support")
        } else {
            home.join(".config")
        };
        
        if let Some(app) = app_name {
            Ok(config_dir.join(app))
        } else {
            Ok(config_dir)
        }
    }

    /// Get user data directory.
    pub fn get_user_data_dir(&self, app_name: Option<&str>) -> Result<std::path::PathBuf, std::io::Error> {
        self.get_user_config_dir(app_name)
    }

    /// Get system temporary directory.
    pub fn get_temp_dir(&self) -> std::path::PathBuf {
        std::env::temp_dir()
    }

    /// Get current working directory.
    pub fn get_current_working_dir(&self) -> Result<std::path::PathBuf, std::io::Error> {
        std::env::current_dir()
    }

    /// Check if running in development mode.
    pub fn is_development_mode(&self) -> bool {
        self.get_env("ENVIRONMENT", Some("development")) == Some("development".to_string()) ||
        self.get_env("ENV", Some("dev")) == Some("dev".to_string()) ||
        self.get_env_bool("DEBUG", Some(true))
    }

    /// Check if running in production mode.
    pub fn is_production_mode(&self) -> bool {
        self.get_env("ENVIRONMENT", None) == Some("production".to_string()) ||
        self.get_env("ENV", None) == Some("prod".to_string())
    }

    /// Check if running in testing mode.
    pub fn is_testing_mode(&self) -> bool {
        self.get_env("ENVIRONMENT", None) == Some("testing".to_string()) ||
        self.get_env("ENV", None) == Some("test".to_string()) ||
        self.get_env_bool("TESTING", Some(false))
    }

    /// Get current environment type.
    pub fn get_environment_type(&self) -> String {
        if self.is_production_mode() {
            "production".to_string()
        } else if self.is_testing_mode() {
            "testing".to_string()
        } else if self.is_development_mode() {
            "development".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Get available system memory in MB.
    pub fn get_available_memory_mb(&self) -> Option<f64> {
        // Would need sysinfo crate for detailed memory info
        None
    }

    /// Get number of CPU cores.
    pub fn get_cpu_count(&self) -> i64 {
        std::thread::available_parallelism()
            .map(|n| n.get() as i64)
            .unwrap_or(1)
    }

    /// Get comprehensive environment summary.
    pub fn get_environment_summary(&mut self) -> HashMap<String, serde_json::Value> {
        let mut summary = HashMap::new();
        summary.insert("platform".to_string(), serde_json::to_value(self.platform_info()).unwrap());
        summary.insert("environment_type".to_string(), serde_json::Value::String(self.get_environment_type()));
        summary.insert("is_windows".to_string(), serde_json::Value::Bool(self.is_windows()));
        summary.insert("is_linux".to_string(), serde_json::Value::Bool(self.is_linux()));
        summary.insert("is_macos".to_string(), serde_json::Value::Bool(self.is_macos()));
        summary.insert("is_64bit".to_string(), serde_json::Value::Bool(self.is_64bit()));
        summary.insert("cpu_count".to_string(), serde_json::Value::Number(self.get_cpu_count().into()));
        summary
    }
}

/// Get global environment manager instance.
pub fn get_environment_manager() -> EnvironmentManager {
    EnvironmentManager::new()
}
