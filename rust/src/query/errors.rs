// #exonware/xwsystem/rust/src/query/errors.rs
//! #exonware/xwsystem/src/exonware/xwsystem/query/errors.py
//! 
//! Query provider errors (foundation layer).
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 28-Dec-2025


/// Base error for query provider integration.
#[derive(Debug)]
pub struct QueryProviderError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for QueryProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for QueryProviderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl QueryProviderError {
    pub fn new(message: impl Into<String>) -> Self {
        QueryProviderError {
            message: message.into(),
            source: None,
        }
    }

}

/// Raised when a consumer requests a query provider but none is registered.
#[derive(Debug)]
pub struct QueryProviderNotRegisteredError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for QueryProviderNotRegisteredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for QueryProviderNotRegisteredError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl QueryProviderNotRegisteredError {
    pub fn new(message: impl Into<String>) -> Self {
        QueryProviderNotRegisteredError {
            message: message.into(),
            source: None,
        }
    }
}
