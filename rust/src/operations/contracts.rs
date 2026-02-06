// #exonware/xwsystem/rust/src/operations/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! 
//! Operations Contracts
//! 
//! Protocol definitions for data operations.


use std::collections::HashMap;

use crate::defs::{DiffMode, DiffResult, MergeStrategy, PatchResult};

// Legacy Protocol interfaces (kept for backward compatibility)

/// Base protocol for all operations.
pub trait IOperation {
    /// Execute the operation.
    fn execute(&self) -> serde_json::Value;

}

/// Protocol for merge operations.
pub trait IMergeOperation: IOperation {
    /// Merge source into target.
    fn merge(&self, target: serde_json::Value, source: serde_json::Value, strategy: MergeStrategy) -> serde_json::Value;

}

/// Protocol for diff operations.
pub trait IDiffOperation: IOperation {
    /// Generate diff between original and modified.
    fn diff(&self, original: serde_json::Value, modified: serde_json::Value, mode: DiffMode) -> DiffResult;

}

/// Protocol for patch operations.
pub trait IPatchOperation: IOperation {
    /// Apply patch operations to data.
    fn apply_patch(&self, data: serde_json::Value, operations: Vec<HashMap<String, serde_json::Value>>) -> PatchResult;

}

/// Protocol for merge operations (legacy).
pub trait IMerge {
    /// Merge two data structures.
    fn merge(&self, base: serde_json::Value, other: serde_json::Value, strategy: MergeStrategy) -> serde_json::Value;

}

/// Protocol for diff operations (legacy).
pub trait IDiff {
    /// Generate diff between two data structures.
    fn diff(&self, old: serde_json::Value, new: serde_json::Value, mode: DiffMode) -> Vec<HashMap<String, serde_json::Value>>;

}

/// Protocol for patch operations (legacy).
pub trait IPatch {
    /// Apply patch operations to data.
    fn patch(&self, data: serde_json::Value, operations: Vec<HashMap<String, serde_json::Value>>) -> serde_json::Value;

}
