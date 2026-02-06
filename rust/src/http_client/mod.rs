// #exonware/xwsystem/rust/src/http_client/mod.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! XSystem HTTP Package
//! 
//! Provides high-performance HTTP client with retry mechanisms,
//! connection pooling, and comprehensive error handling.

pub mod advanced_client;
pub mod base;
pub mod client;
pub mod contracts;
pub mod defs;
pub mod errors;

pub use advanced_client::{AdvancedHttpClient, AdvancedHttpConfig, Http2Config, MockResponse, MockTransport, StreamingConfig};
pub use base::{AAsyncHttpClientBase, AHttpClientBase, AHttpResponseBase, AHttpRetryBase, AHttpSessionBase, AHttpStreamBase};
pub use client::{AsyncHttpClient, HttpClient, RetryConfig};
pub use contracts::{IHttpClient, IHttpResponse, IHttpRetryConfig, IHttpSession, ITransport};
pub use defs::{ContentType, HttpMethod, HttpStatus, RetryStrategy};
pub use errors::{HttpAuthenticationError, HttpAuthorizationError, HttpConnectionError, HttpContentError, HttpError, HttpProxyError, HttpRedirectError, HttpRequestError, HttpRetryError, HttpResponseError, HttpSSLError, HttpSessionError, HttpStatusError, HttpStreamError, HttpTimeoutError};
