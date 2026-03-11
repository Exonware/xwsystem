// #exonware/xwsystem/rust/src/http_client/defs.rs
//exonware/xwsystem/http_client/defs.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! HTTP types and enums for XWSystem.


use serde::{Serialize, Deserialize};

use crate::shared::defs::{AuthType};

/// HTTP methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "OPTIONS")]
    Options,
}

/// HTTP status codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpStatus {
    Ok,
    Created,
    Accepted,
    NoContent,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    Conflict,
    InternalServerError,
    BadGateway,
    ServiceUnavailable,
}

/// Content types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "application/json")]
    Json,
    #[serde(rename = "application/xml")]
    Xml,
    #[serde(rename = "application/x-www-form-urlencoded")]
    Form,
    #[serde(rename = "multipart/form-data")]
    Multipart,
    #[serde(rename = "text/plain")]
    Text,
    #[serde(rename = "text/html")]
    Html,
    #[serde(rename = "application/octet-stream")]
    Binary,
}

/// Retry strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetryStrategy {
    None,
    Linear,
    Exponential,
    Fixed,
}
