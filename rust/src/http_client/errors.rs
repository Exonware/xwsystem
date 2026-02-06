// #exonware/xwsystem/rust/src/http_client/errors.rs
//exonware/xwsystem/http/errors.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! HTTP module errors - exception classes for HTTP client functionality.

/// Base exception for HTTP client errors.
#[derive(Debug)]
pub struct HttpError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpError {
    pub fn new(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            source: None,
        }
    }

}

/// Raised when HTTP connection fails.
#[derive(Debug)]
pub struct HttpConnectionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpConnectionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            message: message.into(),
            source: Some(source),
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP request times out.
#[derive(Debug)]
pub struct HttpTimeoutError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpTimeoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpTimeoutError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP authentication fails.
#[derive(Debug)]
pub struct HttpAuthenticationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpAuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpAuthenticationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpAuthenticationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP authorization fails.
#[derive(Debug)]
pub struct HttpAuthorizationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpAuthorizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpAuthorizationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpAuthorizationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP redirect fails.
#[derive(Debug)]
pub struct HttpRedirectError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpRedirectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpRedirectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpRedirectError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP retry operation fails.
#[derive(Debug)]
pub struct HttpRetryError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpRetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpRetryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpRetryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP proxy operation fails.
#[derive(Debug)]
pub struct HttpProxyError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpProxyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpProxyError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP SSL operation fails.
#[derive(Debug)]
pub struct HttpSSLError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpSSLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpSSLError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpSSLError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP request is invalid.
#[derive(Debug)]
pub struct HttpRequestError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpRequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpRequestError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP response is invalid.
#[derive(Debug)]
pub struct HttpResponseError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpResponseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP status code indicates error.
#[derive(Debug)]
pub struct HttpStatusError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpStatusError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpStatusError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP content operation fails.
#[derive(Debug)]
pub struct HttpContentError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpContentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpContentError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP streaming operation fails.
#[derive(Debug)]
pub struct HttpStreamError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpStreamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpStreamError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}

/// Raised when HTTP session operation fails.
#[derive(Debug)]
pub struct HttpSessionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    status_code: Option<i64>,
    response_data: Option<serde_json::Value>,
}

impl std::fmt::Display for HttpSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HttpSessionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl HttpSessionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
            status_code: None,
            response_data: None,
        }
    }
}
