// #exonware/xwsystem/rust/src/config/logging.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Logging configuration for XSystem.
//! 
//! Provides simple logging control functions and configuration management.


use std::sync::{Arc, RwLock};
use crate::config::defaults::{LOGGING_ENABLED, LOGGING_LEVEL};

// Global logging config instance

// Global logging config instance

// Convenience functions

// Disable logging BEFORE any other imports
// Handle cases where logging level comparison might fail
// Check if handler is a MagicMock to avoid comparison issues
/// Simple logging configuration control.
pub struct LoggingConfig {
    enabled: Arc<RwLock<bool>>,
    level: Arc<RwLock<String>>,
}

impl LoggingConfig {
    /// Constructor
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(RwLock::new(LOGGING_ENABLED)),
            level: Arc::new(RwLock::new(LOGGING_LEVEL.to_string())),
        }
    }

    /// Disable all logging.
    pub fn disable(&mut self) {
        std::env::set_var("XSYSTEM_LOGGING_DISABLE", "true");
        let mut enabled = self.enabled.write().unwrap();
        *enabled = false;
    }

    /// Enable logging.
    pub fn enable(&mut self) {
        std::env::set_var("XSYSTEM_LOGGING_DISABLE", "false");
        let mut enabled = self.enabled.write().unwrap();
        *enabled = true;
    }

    /// Set logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL).
    pub fn set_level(&mut self, level: String) {
        let mut level_guard = self.level.write().unwrap();
        *level_guard = level.to_uppercase();
    }

    /// Check if logging is enabled.
    pub fn enabled(&self) -> bool {
        *self.enabled.read().unwrap()
    }

    /// Get current logging level.
    pub fn level(&self) -> String {
        self.level.read().unwrap().clone()
    }

    /// Get current logging level.
    pub fn get_level(&self) -> String {
        self.level()
    }

    /// Add a logging handler.
    pub fn add_handler(&self, _handler: String) {
        // Handler implementation would go here with a proper logging crate
    }
}

// Global logging config instance
use std::sync::OnceLock;

static LOGGING_CONFIG: OnceLock<Arc<RwLock<LoggingConfig>>> = OnceLock::new();

fn get_logging_config() -> Arc<RwLock<LoggingConfig>> {
    LOGGING_CONFIG.get_or_init(|| Arc::new(RwLock::new(LoggingConfig::new()))).clone()
}

/// Disable all logging.
pub fn logging_disable() {
    std::env::set_var("XSYSTEM_LOGGING_DISABLE", "true");
    let mut config = get_logging_config().write().unwrap();
    config.disable();
}

/// Enable logging.
pub fn logging_enable() {
    let mut config = get_logging_config().write().unwrap();
    config.enable();
}

/// Set logging level.
pub fn logging_set_level(level: &str) {
    let mut config = get_logging_config().write().unwrap();
    config.set_level(level.to_string());
}
