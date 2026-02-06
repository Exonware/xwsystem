// #exonware/xwsystem/rust/src/shared/errors.rs
//exonware/xwsystem/shared/errors.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Shared exceptions (merged from the former core module).


/// Base exception for core errors.
#[derive(Debug)]
pub struct CoreError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreError {
            message: message.into(),
            source: None,
        }
    }

}

/// Raised when core initialization fails.
#[derive(Debug)]
pub struct CoreInitializationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreInitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreInitializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreInitializationError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreInitializationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CoreInitializationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when core shutdown fails.
#[derive(Debug)]
pub struct CoreShutdownError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreShutdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreShutdownError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreShutdownError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreShutdownError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CoreShutdownError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when core state is invalid.
#[derive(Debug)]
pub struct CoreStateError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreStateError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreStateError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CoreStateError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when core dependency is missing or invalid.
#[derive(Debug)]
pub struct CoreDependencyError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreDependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreDependencyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreDependencyError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreDependencyError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CoreDependencyError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when core configuration is invalid.
#[derive(Debug)]
pub struct CoreConfigurationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreConfigurationError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreConfigurationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CoreConfigurationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when core resource operation fails.
#[derive(Debug)]
pub struct CoreResourceError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreResourceError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreResourceError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CoreResourceError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when core operation times out.
#[derive(Debug)]
pub struct CoreTimeoutError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreTimeoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreTimeoutError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreTimeoutError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CoreTimeoutError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when core permission is denied.
#[derive(Debug)]
pub struct CorePermissionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CorePermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CorePermissionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CorePermissionError {
    pub fn new(message: impl Into<String>) -> Self {
        CorePermissionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CorePermissionError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when core validation fails.
#[derive(Debug)]
pub struct CoreValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreValidationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CoreValidationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when core operation fails.
#[derive(Debug)]
pub struct CoreOperationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CoreOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreOperationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CoreOperationError {
    pub fn new(message: impl Into<String>) -> Self {
        CoreOperationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CoreOperationError {
            message: message.into(),
            source: Some(source),
        }
    }
}
