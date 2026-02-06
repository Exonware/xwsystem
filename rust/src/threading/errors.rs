// #exonware/xwsystem/rust/src/threading/errors.rs
//exonware/xwsystem/threading/errors.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Threading module errors - exception classes for threading functionality.

/// Base exception for threading errors.
#[derive(Debug)]
pub struct ThreadingError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ThreadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThreadingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ThreadingError {
    pub fn new(message: impl Into<String>) -> Self {
        ThreadingError {
            message: message.into(),
            source: None,
        }
    }

}

/// Raised when thread operation fails.
#[derive(Debug)]
pub struct ThreadError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ThreadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThreadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ThreadError {
    pub fn new(message: impl Into<String>) -> Self {
        ThreadError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ThreadError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when thread creation fails.
#[derive(Debug)]
pub struct ThreadCreationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ThreadCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThreadCreationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ThreadCreationError {
    pub fn new(message: impl Into<String>) -> Self {
        ThreadCreationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ThreadCreationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when thread start fails.
#[derive(Debug)]
pub struct ThreadStartError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ThreadStartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThreadStartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ThreadStartError {
    pub fn new(message: impl Into<String>) -> Self {
        ThreadStartError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ThreadStartError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when thread stop fails.
#[derive(Debug)]
pub struct ThreadStopError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ThreadStopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThreadStopError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ThreadStopError {
    pub fn new(message: impl Into<String>) -> Self {
        ThreadStopError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ThreadStopError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when thread join fails.
#[derive(Debug)]
pub struct ThreadJoinError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ThreadJoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThreadJoinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ThreadJoinError {
    pub fn new(message: impl Into<String>) -> Self {
        ThreadJoinError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ThreadJoinError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when thread operation times out.
#[derive(Debug)]
pub struct ThreadTimeoutError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ThreadTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThreadTimeoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ThreadTimeoutError {
    pub fn new(message: impl Into<String>) -> Self {
        ThreadTimeoutError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ThreadTimeoutError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when lock operation fails.
#[derive(Debug)]
pub struct LockError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for LockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl LockError {
    pub fn new(message: impl Into<String>) -> Self {
        LockError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        LockError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when lock acquisition fails.
#[derive(Debug)]
pub struct LockAcquisitionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for LockAcquisitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LockAcquisitionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl LockAcquisitionError {
    pub fn new(message: impl Into<String>) -> Self {
        LockAcquisitionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        LockAcquisitionError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when lock release fails.
#[derive(Debug)]
pub struct LockReleaseError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for LockReleaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LockReleaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl LockReleaseError {
    pub fn new(message: impl Into<String>) -> Self {
        LockReleaseError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        LockReleaseError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when lock operation times out.
#[derive(Debug)]
pub struct LockTimeoutError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for LockTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LockTimeoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl LockTimeoutError {
    pub fn new(message: impl Into<String>) -> Self {
        LockTimeoutError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        LockTimeoutError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when deadlock is detected.
#[derive(Debug)]
pub struct DeadlockError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DeadlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DeadlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl DeadlockError {
    pub fn new(message: impl Into<String>) -> Self {
        DeadlockError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        DeadlockError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when race condition is detected.
#[derive(Debug)]
pub struct RaceConditionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for RaceConditionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RaceConditionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl RaceConditionError {
    pub fn new(message: impl Into<String>) -> Self {
        RaceConditionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        RaceConditionError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when async operation fails.
#[derive(Debug)]
pub struct AsyncError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for AsyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AsyncError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl AsyncError {
    pub fn new(message: impl Into<String>) -> Self {
        AsyncError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        AsyncError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when async primitive operation fails.
#[derive(Debug)]
pub struct AsyncPrimitiveError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for AsyncPrimitiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AsyncPrimitiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl AsyncPrimitiveError {
    pub fn new(message: impl Into<String>) -> Self {
        AsyncPrimitiveError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        AsyncPrimitiveError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when async operation times out.
#[derive(Debug)]
pub struct AsyncTimeoutError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for AsyncTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AsyncTimeoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl AsyncTimeoutError {
    pub fn new(message: impl Into<String>) -> Self {
        AsyncTimeoutError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        AsyncTimeoutError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when safe factory operation fails.
#[derive(Debug)]
pub struct SafeFactoryError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for SafeFactoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SafeFactoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl SafeFactoryError {
    pub fn new(message: impl Into<String>) -> Self {
        SafeFactoryError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        SafeFactoryError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when thread safety is violated.
#[derive(Debug)]
pub struct ThreadSafetyError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ThreadSafetyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThreadSafetyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ThreadSafetyError {
    pub fn new(message: impl Into<String>) -> Self {
        ThreadSafetyError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ThreadSafetyError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when concurrency operation fails.
#[derive(Debug)]
pub struct ConcurrencyError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConcurrencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConcurrencyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConcurrencyError {
    pub fn new(message: impl Into<String>) -> Self {
        ConcurrencyError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ConcurrencyError {
            message: message.into(),
            source: Some(source),
        }
    }
}
