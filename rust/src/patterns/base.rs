// #exonware/xwsystem/rust/src/patterns/base.rs
//exonware/xwsystem/patterns/base.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Abstract base classes for XSystem patterns.


use crate::contracts::{IHandler, IPattern};

/// Abstract base class for handlers that work with GenericHandlerFactory.
///
/// This provides a standard interface that handlers should implement.
pub trait AHandler {
    /// Handle the given data.
    /// Args:
    /// data: Data to handle
    /// Returns:
    /// Handled data
    fn handle(&self, data: serde_json::Value) -> serde_json::Value;

    /// Check if this handler can process the given data.
    /// Args:
    /// data: Data to check
    /// Returns:
    /// True if handler can process the data
    fn can_handle(&self, data: serde_json::Value) -> bool;

    /// Get the priority of this handler (higher = more important).
    /// Returns:
    /// Priority value
    fn get_priority(&self) -> i64;

    /// Process the data (convenience method that calls handle).
    fn process(&self, data: serde_json::Value) -> serde_json::Value {
        self.handle(data)
    }

    /// Validate input data before processing.
    fn validate_input(&self, _data: &serde_json::Value) -> bool {
        true
    }

    /// Get list of formats this handler supports.
    fn get_supported_formats(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Base pattern implementation.
pub struct BasePattern {
    _name: String,
    _enabled: bool,
}

impl IPattern for BasePattern {
    // Trait methods would be implemented here if IPattern has any
}

impl BasePattern {
    /// Initialize the pattern.
    pub fn new(name: Option<String>) -> Self {
        Self {
            _name: name.unwrap_or_else(|| "base_pattern".to_string()),
            _enabled: true,
        }
    }

    /// Get pattern name.
    pub fn name(&self) -> &str {
        &self._name
    }

    /// Check if pattern is enabled.
    pub fn enabled(&self) -> bool {
        self._enabled
    }

    /// Enable the pattern.
    pub fn enable(&mut self) {
        self._enabled = true;
    }

    /// Disable the pattern.
    pub fn disable(&mut self) {
        self._enabled = false;
    }

    /// Apply the pattern to data.
    pub fn apply(&self, data: serde_json::Value) -> serde_json::Value {
        if !self._enabled {
            return data;
        }
        self._process(data)
    }

    /// Process data with the pattern.
    pub fn _process(&self, data: serde_json::Value) -> serde_json::Value {
        // Base implementation - can be overridden
        data
    }

    /// Validate data for pattern application.
    pub fn validate(&self, _data: &serde_json::Value) -> bool {
        true
    }
}
