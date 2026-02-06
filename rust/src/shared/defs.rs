// #exonware/xwsystem/rust/src/shared/defs.rs
//exonware/xwsystem/shared/defs.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 10-Sep-2025
//! 
//! Shared types and enums for XWSystem modules.

// ============================================================================

// CORE ENUMS (merged from former xwsystem.core)

// ============================================================================

// For structures module
/// Standard validation levels used across XWSystem modules.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationLevel {
    None,
    Basic,
    Strict,
    Paranoid,
    Comprehensive,
}

// Performance module levels
// Monitoring module levels
/// Standard performance levels used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceLevel {
    Low,
    Medium,
    High,
    Critical,
    Excellent,
    Good,
    Average,
    Poor,
}

/// Standard authentication types used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthType {
    None,
    Basic,
    Bearer,
    #[serde(rename = "api_key")]
    ApiKey,
    OAuth2,
    Jwt,
    Saml,
}

// Threading module lock types
/// Standard lock types used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockType {
    Shared,
    Exclusive,
    #[serde(rename = "non_blocking")]
    NonBlocking,
    Mutex,
    Rwlock,
    Semaphore,
    Condition,
    Event,
    Barrier,
}

// Utils module path types
/// Standard path types used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathType {
    File,
    Directory,
    Symlink,
    Mount,
    Socket,
    Pipe,
    Device,
    Absolute,
    Relative,
    Resolved,
    Normalized,
}

/// Standard logging levels used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "DEBUG")]
    Debug,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "CRITICAL")]
    Critical,
}

/// Standard operation result status used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationResult {
    Success,
    Failed,
    Partial,
    Skipped,
    Timeout,
    Error,
    Warning,
}

/// Core data types supported by XWSystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    List,
    Dict,
    Bytes,
    None,
    Custom,
}

/// Cloning modes for object duplication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloneMode {
    Shallow,
    Deep,
    Reference,
}

/// Core system states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoreState {
    Uninitialized,
    Initializing,
    Initialized,
    Starting,
    Running,
    Stopping,
    Stopped,
    #[serde(rename = "shutting_down")]
    ShuttingDown,
    Shutdown,
    Error,
}

/// Core system modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoreMode {
    Development,
    Testing,
    Staging,
    Production,
    Debug,
    Performance,
}

/// Core system priorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorePriority {
    Low,
    Normal,
    High,
    Critical,
}
