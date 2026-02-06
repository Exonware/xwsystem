// #exonware/xwsystem/rust/src/http_client/base.rs
//exonware/xwsystem/http/base.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! HTTP module base classes - abstract classes for HTTP client functionality.


use std::collections::HashMap;

use crate::contracts::{AuthType, ContentType, HttpMethod, HttpStatus};

/// Abstract base class for HTTP client operations.
pub trait AHttpClientBase {
    /// Make GET request.
    fn get(&self, url: String) -> serde_json::Value;

    /// Make POST request.
    fn post(&self, url: String, data: Option<serde_json::Value>) -> serde_json::Value;

    /// Make PUT request.
    fn put(&self, url: String, data: Option<serde_json::Value>) -> serde_json::Value;

    /// Make DELETE request.
    fn delete(&self, url: String) -> serde_json::Value;

    /// Make PATCH request.
    fn patch(&self, url: String, data: Option<serde_json::Value>) -> serde_json::Value;

    /// Make HEAD request.
    fn head(&self, url: String) -> serde_json::Value;

    /// Make OPTIONS request.
    fn options(&self, url: String) -> serde_json::Value;

    /// Make HTTP request.
    fn request(&self, method: HttpMethod, url: String) -> serde_json::Value;

    /// Close HTTP client session.
    fn close(&self) -> ();

}

/// Abstract base class for async HTTP client operations.
pub trait AAsyncHttpClientBase {
    /// Make async GET request.
    async fn aget(&self, url: String) -> serde_json::Value;

    /// Make async POST request.
    async fn apost(&self, url: String, data: Option<serde_json::Value>) -> serde_json::Value;

    /// Make async PUT request.
    async fn aput(&self, url: String, data: Option<serde_json::Value>) -> serde_json::Value;

    /// Make async DELETE request.
    async fn adelete(&self, url: String) -> serde_json::Value;

    /// Make async PATCH request.
    async fn apatch(&self, url: String, data: Option<serde_json::Value>) -> serde_json::Value;

    /// Make async HEAD request.
    async fn ahead(&self, url: String) -> serde_json::Value;

    /// Make async OPTIONS request.
    async fn aoptions(&self, url: String) -> serde_json::Value;

    /// Make async HTTP request.
    async fn arequest(&self, method: HttpMethod, url: String) -> serde_json::Value;

    /// Close async HTTP client session.
    async fn aclose(&self) -> ();

}

/// Abstract base class for HTTP response.
pub trait AHttpResponseBase {
    /// Parse response as JSON.
    fn json(&self) -> HashMap<String, serde_json::Value>;

    /// Get response as text.
    fn text(&self) -> String;

    /// Get response as bytes.
    fn bytes(&self) -> Vec<u8>;

    /// Check if response is successful.
    fn is_success(&self) -> bool;

    /// Check if response is error.
    fn is_error(&self) -> bool;

    /// Get response header.
    fn get_header(&self, name: String) -> Option<String>;

    /// Get response cookies.
    fn get_cookies(&self) -> HashMap<String, String>;

}

/// Abstract base class for HTTP session management.
pub trait AHttpSessionBase {
    /// Set authentication.
    fn set_auth(&self, auth_type: AuthType) -> ();

    /// Set default headers.
    fn set_headers(&self, headers: HashMap<String, String>) -> ();

    /// Set cookies.
    fn set_cookies(&self, cookies: HashMap<String, String>) -> ();

    /// Set proxy.
    fn set_proxy(&self, proxy_url: String) -> ();

    /// Set SSL verification.
    fn set_verify_ssl(&self, verify: bool) -> ();

    /// Set request timeout.
    fn set_timeout(&self, timeout: i64) -> ();

    /// Get session information.
    fn get_session_info(&self) -> HashMap<String, serde_json::Value>;

}

/// Abstract base class for HTTP retry logic.
pub trait AHttpRetryBase {
    /// Check if request should be retried.
    fn should_retry(&self, response: AHttpResponseBase, attempt: i64) -> bool;

    /// Get retry delay for attempt.
    fn get_retry_delay(&self, attempt: i64) -> f64;

    /// Get status codes that should trigger retry.
    fn get_retry_status_codes(&self) -> Vec<i64>;

    /// Set status codes that should trigger retry.
    fn set_retry_status_codes(&self, status_codes: Vec<i64>) -> ();

}

/// Abstract base class for HTTP streaming.
pub trait AHttpStreamBase {
    /// Stream HTTP request.
    fn stream_request(&self, method: HttpMethod, url: String) -> String;

    /// Stream HTTP response.
    fn stream_response(&self, response: AHttpResponseBase) -> String;

    /// Upload file via HTTP.
    fn upload_file(&self, url: String, file_path: String) -> serde_json::Value;

    /// Download file via HTTP.
    fn download_file(&self, url: String, file_path: String) -> ();

}
