// #exonware/xwsystem/rust/src/caching/errors.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Caching module errors - exception classes for caching functionality.


/// Base exception for caching errors.
#[derive(Debug)]
pub struct CacheError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache key is invalid or not found.
#[derive(Debug)]
pub struct CacheKeyError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheKeyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheKeyError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheKeyError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheKeyError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache size limit is exceeded.
#[derive(Debug)]
pub struct CacheSizeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheSizeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheSizeError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheSizeError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheSizeError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache TTL (Time To Live) is invalid.
#[derive(Debug)]
pub struct CacheTTLError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheTTLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheTTLError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheTTLError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheTTLError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheTTLError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache serialization fails.
#[derive(Debug)]
pub struct CacheSerializationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheSerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheSerializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheSerializationError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheSerializationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheSerializationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache deserialization fails.
#[derive(Debug)]
pub struct CacheDeserializationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheDeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheDeserializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheDeserializationError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheDeserializationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheDeserializationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache connection fails.
#[derive(Debug)]
pub struct CacheConnectionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheConnectionError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheConnectionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheConnectionError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache operation times out.
#[derive(Debug)]
pub struct CacheTimeoutError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheTimeoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheTimeoutError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheTimeoutError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheTimeoutError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache permission is denied.
#[derive(Debug)]
pub struct CachePermissionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CachePermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CachePermissionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CachePermissionError {
    pub fn new(message: impl Into<String>) -> Self {
        CachePermissionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CachePermissionError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache configuration is invalid.
#[derive(Debug)]
pub struct CacheConfigurationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheConfigurationError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheConfigurationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheConfigurationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when distributed cache operation fails.
#[derive(Debug)]
pub struct DistributedCacheError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DistributedCacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DistributedCacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl DistributedCacheError {
    pub fn new(message: impl Into<String>) -> Self {
        DistributedCacheError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        DistributedCacheError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache lock operation fails.
#[derive(Debug)]
pub struct CacheLockError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheLockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheLockError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheLockError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheLockError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache eviction fails.
#[derive(Debug)]
pub struct CacheEvictionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheEvictionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheEvictionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheEvictionError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheEvictionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheEvictionError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache validation fails (security check).
#[derive(Debug)]
pub struct CacheValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheValidationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheValidationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache integrity check fails.
#[derive(Debug)]
pub struct CacheIntegrityError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheIntegrityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheIntegrityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheIntegrityError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheIntegrityError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheIntegrityError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache rate limit is exceeded.
#[derive(Debug)]
pub struct CacheRateLimitError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheRateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheRateLimitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheRateLimitError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheRateLimitError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheRateLimitError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache value exceeds maximum allowed size.
#[derive(Debug)]
pub struct CacheValueSizeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheValueSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheValueSizeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheValueSizeError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheValueSizeError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheValueSizeError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when cache key exceeds maximum allowed size.
#[derive(Debug)]
pub struct CacheKeySizeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CacheKeySizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CacheKeySizeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CacheKeySizeError {
    pub fn new(message: impl Into<String>) -> Self {
        CacheKeySizeError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CacheKeySizeError {
            message: message.into(),
            source: Some(source),
        }
    }
}
