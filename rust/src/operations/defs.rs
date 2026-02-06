// #exonware/xwsystem/rust/src/operations/defs.rs
//! #exonware/xwsystem/src/exonware/xwsystem/operations/defs.py
//! 
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 27, 2025
//! 
//! Operations definitions and data structures.

// Recursive merge (default)
// Append lists instead of replacing
// Merge lists with uniqueness
/// Merge strategies for data operations.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeStrategy {
    Deep,
    Shallow,
    Overwrite,
    Append,
    Unique,
}

// Compare structure only
// Compare both structure and content
/// Diff operation modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffMode {
    Structural,
    Content,
    Full,
}

/// JSON Patch operation types (RFC 6902).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchOperation {
    Add,
    Remove,
    Replace,
    Move,
    Copy,
    Test,
}

/// Result of a diff operation.
#[derive(Debug, Clone)]
pub struct DiffResult {
    pub operations: Vec<serde_json::Value>,
    pub mode: DiffMode,
    pub paths_changed: Vec<String>,
    pub total_changes: usize,
}

impl DiffResult {
    pub fn new(operations: Vec<serde_json::Value>, mode: DiffMode, paths_changed: Vec<String>) -> Self {
        DiffResult {
            total_changes: operations.len(),
            operations,
            mode,
            paths_changed,
        }
    }
}

/// Result of a patch operation.
#[derive(Debug, Clone)]
pub struct PatchResult {
    pub success: bool,
    pub operations_applied: usize,
    pub errors: Vec<String>,
    pub result: serde_json::Value,
}

impl PatchResult {
    pub fn new(success: bool, operations_applied: usize, errors: Vec<String>, result: serde_json::Value) -> Self {
        PatchResult {
            success,
            operations_applied,
            errors,
            result,
        }
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use MergeStrategy;
pub use DiffMode;
pub use PatchOperation;
pub use DiffResult;
pub use PatchResult;
