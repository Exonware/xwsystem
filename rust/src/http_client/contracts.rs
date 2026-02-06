// #exonware/xwsystem/rust/src/http_client/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! HTTP module contracts - interfaces and enums for HTTP client functionality.


use std::collections::HashMap;

use crate::defs::{AuthType, ContentType, HttpMethod, HttpStatus, RetryStrategy};

/// Interface for HTTP client operations.
pub trait IHttpClient {
    /// Make GET request.
    async fn get(&self, url: String) -> String;

    /// Make POST request.
    async fn post(&self, url: String, data: Option<serde_json::Value>) -> String;

    /// Make PUT request.
    async fn put(&self, url: String, data: Option<serde_json::Value>) -> String;

    /// Make DELETE request.
    async fn delete(&self, url: String) -> String;

    /// Make HTTP request.
    async fn request(&self, method: HttpMethod, url: String) -> String;

}

/// Interface for HTTP response.
pub trait IHttpResponse {
    /// Response status code.
    // Python decorators: @property
    fn status_code(&self) -> i64;

    /// Response headers.
    // Python decorators: @property
    fn headers(&self) -> HashMap<String, String>;

    /// Response content as bytes.
    // Python decorators: @property
    fn content(&self) -> Vec<u8>;

    /// Response content as text.
    // Python decorators: @property
    fn text(&self) -> &str;

    /// Response content as JSON.
    fn json(&self) -> serde_json::Value;

    /// Stream response content.
    async fn stream(&self) -> String;

}

/// Interface for HTTP session management.
pub trait IHttpSession {
    /// Set authentication.
    fn set_auth(&self, auth_type: AuthType) -> ();

    /// Set default headers.
    fn set_headers(&self, headers: HashMap<String, String>) -> ();

    /// Set request timeout.
    fn set_timeout(&self, timeout: f64) -> ();

}

/// Interface for retry configuration.
pub trait IRetryConfig {
    /// Maximum number of retries.
    // Python decorators: @property
    fn max_retries(&self) -> i64;

    /// Retry strategy.
    // Python decorators: @property
    fn strategy(&self) -> RetryStrategy;

    /// Backoff factor for retries.
    // Python decorators: @property
    fn backoff_factor(&self) -> f64;

    /// Status codes to retry on.
    // Python decorators: @property
    fn retry_on_status(&self) -> Vec<i64>;

}

/// Abstract base class for HTTP transport implementations.
pub trait ITransport {
    /// Make an HTTP request.
    async fn request(&self, method: String, url: String) -> serde_json::Value;

}
