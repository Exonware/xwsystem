// #exonware/xwsystem/rust/src/config/logging_setup.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 05, 2025
//! 
//! Logging configuration setup for XSystem.
//! 
//! Provides centralized logging setup functions and logger factory.
//! This module handles the implementation details of logging configuration.


use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::config::defaults::{LOGGING_ENABLED, LOGGING_LEVEL};

/// Logging setup manager for XSystem framework.
pub struct LoggingSetup {
    configured: Arc<RwLock<bool>>,
}

impl LoggingSetup {
    /// Initialize logging setup.
    pub fn new() -> Self {
        Self {
            configured: Arc::new(RwLock::new(false)),
        }
    }

    /// Setup logging configuration.
    pub fn setup_logging(&mut self, level: Option<String>, _kwargs: HashMap<String, String>) {
        if let Some(level_str) = level {
            std::env::set_var("XSYSTEM_LOGGING_LEVEL", &level_str);
        }
        let mut configured = self.configured.write().unwrap();
        *configured = true;
    }

    /// Configure a logger with the specified name.
    pub fn configure_logger(&self, name: &str) -> String {
        format!("logger:{}", name)
    }

    /// Check if logging is configured.
    pub fn is_configured(&self) -> bool {
        *self.configured.read().unwrap()
    }
}

/// Centralized logging setup for production use.
/// - Console and rotating file handlers
/// - Structured formatting
/// - Easy integration for all entry points
pub fn setup_logging(
    log_file: Option<&str>,
    level: Option<&str>,
    _max_bytes: Option<&str>,
    _backup_count: Option<&str>,
    _fmt: Option<&str>,
) {
    // Check if logging is disabled via environment variable - do this FIRST
    if std::env::var("XSYSTEM_LOGGING_DISABLE")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase() == "true"
    {
        return;
    }

    if let Some(level_str) = level {
        std::env::set_var("XSYSTEM_LOGGING_LEVEL", level_str);
    }

    if let Some(log_file_path) = log_file {
        if let Some(log_dir) = std::path::Path::new(log_file_path).parent() {
            let _ = std::fs::create_dir_all(log_dir);
        }
    }
}

/// Get a logger instance with the specified name.
///
/// Args:
/// name: Logger name (defaults to "root" if not provided)
///
/// Returns:
/// Logger name string (would return actual logger with proper logging crate)
pub fn get_logger(name: Option<&str>) -> String {
    // Check if logging is disabled
    if std::env::var("XSYSTEM_LOGGING_DISABLE")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase() == "true"
    {
        return "disabled".to_string();
    }

    name.unwrap_or("root").to_string()
}
