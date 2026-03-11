// #exonware/xwsystem/rust/src/config/errors.rs
//exonware/xwsystem/config/errors.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Configuration module errors - exception classes for configuration functionality.


// Aliases for backward compatibility

// Aliases for backward compatibility

/// Base exception for configuration errors.
#[derive(Debug)]
pub struct ConfigError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigError {
            message: message.into(),
            source: None,
        }
    }

}

/// Raised when configuration file is not found.
#[derive(Debug)]
pub struct ConfigNotFoundError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigNotFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigNotFoundError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigNotFoundError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ConfigNotFoundError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when configuration parsing fails.
#[derive(Debug)]
pub struct ConfigParseError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigParseError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigParseError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ConfigParseError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when configuration validation fails.
#[derive(Debug)]
pub struct ConfigValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigValidationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ConfigValidationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when configuration key is invalid or not found.
#[derive(Debug)]
pub struct ConfigKeyError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigKeyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigKeyError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigKeyError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when configuration value is invalid.
#[derive(Debug)]
pub struct ConfigValueError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigValueError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigValueError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigValueError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when configuration type is invalid.
#[derive(Debug)]
pub struct ConfigTypeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigTypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigTypeError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigTypeError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when configuration permission is denied.
#[derive(Debug)]
pub struct ConfigPermissionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigPermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigPermissionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigPermissionError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigPermissionError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when configuration lock operation fails.
#[derive(Debug)]
pub struct ConfigLockError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigLockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigLockError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigLockError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when configuration backup operation fails.
#[derive(Debug)]
pub struct ConfigBackupError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigBackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigBackupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigBackupError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigBackupError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when configuration restore operation fails.
#[derive(Debug)]
pub struct ConfigRestoreError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigRestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigRestoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigRestoreError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigRestoreError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when logging configuration fails.
#[derive(Debug)]
pub struct LoggingConfigError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for LoggingConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LoggingConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl LoggingConfigError {
    pub fn new(message: impl Into<String>) -> Self {
        LoggingConfigError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when performance configuration fails.
#[derive(Debug)]
pub struct PerformanceConfigError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PerformanceConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PerformanceConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PerformanceConfigError {
    pub fn new(message: impl Into<String>) -> Self {
        PerformanceConfigError {
            message: message.into(),
            source: None,
        }
    }
}

// Type aliases for backward compatibility
pub type PerformanceError = PerformanceConfigError;
pub type LoggingError = LoggingConfigError;
