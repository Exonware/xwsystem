// #exonware/xwsystem/rust/src_old/errors.rs
//! Error hierarchy for XWSystem.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This module provides the error hierarchy for all XWSystem operations.
//! Uses thiserror for clean error definitions with proper error chaining.

use thiserror::Error;

// ============================================================================
// ROOT ERROR CLASSES (matching Python errors.py)
// ============================================================================

/// Base exception for all XWSystem errors.
///
/// Matches Python's XWSystemError base exception class.
/// All XWSystem-specific exceptions should inherit from this.
#[derive(Debug, Error)]
#[error("XWSystem error: {message}")]
pub struct XWSystemError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemError {
    /// Create a new XWSystemError with a message.
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemError {
            message: message.into(),
            source: None,
        }
    }

    /// Create a new XWSystemError with a message and source error.
    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemError {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when XWSystem initialization fails.
///
/// Matches Python's XWSystemInitializationError.
#[derive(Debug, Error)]
#[error("XWSystem initialization error: {message}")]
pub struct XWSystemInitializationError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemInitializationError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemInitializationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemInitializationError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when XWSystem configuration is invalid.
///
/// Matches Python's XWSystemConfigurationError.
#[derive(Debug, Error)]
#[error("XWSystem configuration error: {message}")]
pub struct XWSystemConfigurationError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemConfigurationError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemConfigurationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemConfigurationError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when XWSystem state is invalid.
///
/// Matches Python's XWSystemStateError.
#[derive(Debug, Error)]
#[error("XWSystem state error: {message}")]
pub struct XWSystemStateError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemStateError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemStateError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemStateError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when XWSystem dependency is missing or invalid.
///
/// Matches Python's XWSystemDependencyError.
#[derive(Debug, Error)]
#[error("XWSystem dependency error: {message}")]
pub struct XWSystemDependencyError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemDependencyError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemDependencyError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemDependencyError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when XWSystem resource operation fails.
///
/// Matches Python's XWSystemResourceError.
#[derive(Debug, Error)]
#[error("XWSystem resource error: {message}")]
pub struct XWSystemResourceError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemResourceError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemResourceError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemResourceError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when XWSystem operation times out.
///
/// Matches Python's XWSystemTimeoutError.
#[derive(Debug, Error)]
#[error("XWSystem timeout error: {message}")]
pub struct XWSystemTimeoutError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemTimeoutError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemTimeoutError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemTimeoutError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when XWSystem permission is denied.
///
/// Matches Python's XWSystemPermissionError.
#[derive(Debug, Error)]
#[error("XWSystem permission error: {message}")]
pub struct XWSystemPermissionError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemPermissionError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemPermissionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemPermissionError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when XWSystem validation fails.
///
/// Matches Python's XWSystemValidationError.
#[derive(Debug, Error)]
#[error("XWSystem validation error: {message}")]
pub struct XWSystemValidationError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemValidationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemValidationError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when XWSystem operation fails.
///
/// Matches Python's XWSystemOperationError.
#[derive(Debug, Error)]
#[error("XWSystem operation error: {message}")]
pub struct XWSystemOperationError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl XWSystemOperationError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemOperationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        XWSystemOperationError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// CORE ERROR CLASSES (matching Python shared.errors and core errors)
// ============================================================================

/// Base error type for all XWSystem errors.
#[derive(Debug, Error)]
#[error("Core error: {message}")]
pub struct CoreError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreError {
    /// Create a new CoreError with a message.
    pub fn new(message: impl Into<String>) -> Self {
        CoreError {
            message: message.into(),
            source: None,
        }
    }

    /// Create a new CoreError with a message and source error.
    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreError {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the source error if any.
    pub fn source_error(&self) -> Option<&(dyn std::error::Error + Send + Sync)> {
        self.source.as_deref()
    }
}

/// Raised when core initialization fails.
#[derive(Debug, Error)]
#[error("Core initialization error: {message}")]
pub struct CoreInitializationError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreInitializationError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreInitializationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreInitializationError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when core shutdown fails.
#[derive(Debug, Error)]
#[error("Core shutdown error: {message}")]
pub struct CoreShutdownError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreShutdownError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreShutdownError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreShutdownError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when core state is invalid.
#[derive(Debug, Error)]
#[error("Core state error: {message}")]
pub struct CoreStateError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreStateError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreStateError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreStateError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when core dependency is missing or invalid.
#[derive(Debug, Error)]
#[error("Core dependency error: {message}")]
pub struct CoreDependencyError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreDependencyError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreDependencyError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreDependencyError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when core configuration is invalid.
#[derive(Debug, Error)]
#[error("Core configuration error: {message}")]
pub struct CoreConfigurationError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreConfigurationError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreConfigurationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreConfigurationError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when core resource operation fails.
#[derive(Debug, Error)]
#[error("Core resource error: {message}")]
pub struct CoreResourceError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreResourceError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreResourceError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreResourceError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when core operation times out.
#[derive(Debug, Error)]
#[error("Core timeout error: {message}")]
pub struct CoreTimeoutError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreTimeoutError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreTimeoutError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreTimeoutError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when core permission is denied.
#[derive(Debug, Error)]
#[error("Core permission error: {message}")]
pub struct CorePermissionError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CorePermissionError {
    pub fn new(message: impl Into<String>) -> Self {
        CorePermissionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CorePermissionError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when core validation fails.
#[derive(Debug, Error)]
#[error("Core validation error: {message}")]
pub struct CoreValidationError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreValidationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreValidationError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Raised when core operation fails.
#[derive(Debug, Error)]
#[error("Core operation error: {message}")]
pub struct CoreOperationError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CoreOperationError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreOperationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        CoreOperationError {
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_xwsystem_error_creation() {
        let error = XWSystemError::new("Test error");
        assert_eq!(error.message(), "Test error");
    }

    #[test]
    fn test_xwsystem_initialization_error() {
        let error = XWSystemInitializationError::new("Failed to initialize");
        assert_eq!(error.message(), "Failed to initialize");
    }

    #[test]
    fn test_xwsystem_configuration_error() {
        let error = XWSystemConfigurationError::new("Invalid config");
        assert_eq!(error.message(), "Invalid config");
    }

    #[test]
    fn test_xwsystem_state_error() {
        let error = XWSystemStateError::new("Invalid state");
        assert_eq!(error.message(), "Invalid state");
    }

    #[test]
    fn test_core_error_creation() {
        let error = CoreError::new("Test error");
        assert_eq!(error.message(), "Test error");
    }

    #[test]
    fn test_core_initialization_error() {
        let error = CoreInitializationError::new("Failed to initialize");
        assert_eq!(error.message(), "Failed to initialize");
    }

    #[test]
    fn test_error_display() {
        let error = XWSystemError::new("Test error");
        let display = format!("{}", error);
        assert!(display.contains("XWSystem error"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_error_with_source() {
        let source = Box::new(std::io::Error::new(std::io::ErrorKind::Other, "IO error"));
        let error = XWSystemError::with_source("Wrapper error", source);
        assert_eq!(error.message(), "Wrapper error");
        assert!(error.source().is_some());
    }
}

