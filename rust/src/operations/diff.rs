// #exonware/xwsystem/rust/src/operations/diff.rs
//! #exonware/xwsystem/src/exonware/xwsystem/operations/diff.py
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 27, 2025
//! 
//! Diff operations implementation.

use std::collections::HashMap;
use std::sync::Mutex;

use super::base::DiffError;
use super::contracts::IDiffOperation;
use super::defs::{DiffMode, DiffResult};

/// Thread-safe diff operation implementation.
pub struct DiffOperation {
    _lock: Mutex<()>,
}

impl DiffOperation {
    /// Constructor
    pub fn new() -> Self {
        Self {
            _lock: Mutex::new(()),
        }
    }

    /// Execute diff operation.
    pub fn execute(&self, args: &[serde_json::Value], kwargs: &HashMap<String, serde_json::Value>) -> Result<serde_json::Value, DiffError> {
        if args.len() < 2 {
            return Err(DiffError::new("Diff requires original and modified data"));
        }

        let original = &args[0];
        let modified = &args[1];
        let mode = kwargs.get("mode")
            .and_then(|v| serde_json::from_value::<DiffMode>(v.clone()).ok())
            .unwrap_or(DiffMode::Full);

        let result = self.diff(original.clone(), modified.clone(), mode)?;
        Ok(serde_json::to_value(&result).map_err(|e| DiffError::new(format!("Serialization error: {}", e)))?)
    }

    /// Generate diff between original and modified data.
    pub fn diff(&self, original: serde_json::Value, modified: serde_json::Value, mode: DiffMode) -> Result<DiffResult, DiffError> {
        let _guard = self._lock.lock().map_err(|e| DiffError::new(format!("Lock error: {}", e)))?;
        drop(_guard); // Release immediately since we're not actually using it for recursion

        let mut operations = Vec::new();
        let mut paths_changed = Vec::new();

        match mode {
            DiffMode::Structural => {
                self._structural_diff(&original, &modified, String::new(), &mut operations, &mut paths_changed);
            }
            DiffMode::Content => {
                self._content_diff(&original, &modified, String::new(), &mut operations, &mut paths_changed);
            }
            DiffMode::Full => {
                self._full_diff(&original, &modified, String::new(), &mut operations, &mut paths_changed);
            }
        }

        Ok(DiffResult::new(operations, mode, paths_changed))
    }

    /// Compare structure only (keys, types).
    fn _structural_diff(&self, original: &serde_json::Value, modified: &serde_json::Value, path: String, operations: &mut Vec<serde_json::Value>, paths_changed: &mut Vec<String>) {
        // Compare types
        if std::mem::discriminant(original) != std::mem::discriminant(modified) {
            let op = serde_json::json!({
                "op": "replace",
                "path": if path.is_empty() { "/" } else { &path },
                "value": modified,
                "old_value": original,
                "reason": "type_changed"
            });
            operations.push(op);
            paths_changed.push(if path.is_empty() { "/".to_string() } else { path.clone() });
            return;
        }

        // Compare dict keys
        if let (Some(orig_map), Some(mod_map)) = (original.as_object(), modified.as_object()) {
            let all_keys: std::collections::HashSet<_> = orig_map.keys().chain(mod_map.keys()).collect();

            for key in all_keys {
                let key_path = if path.is_empty() {
                    format!("/{}", key)
                } else {
                    format!("{}/{}", path, key)
                };

                if !orig_map.contains_key(key) {
                    let op = serde_json::json!({
                        "op": "add",
                        "path": key_path,
                        "value": mod_map[key],
                        "reason": "key_added"
                    });
                    operations.push(op);
                    paths_changed.push(key_path.clone());
                } else if !mod_map.contains_key(key) {
                    let op = serde_json::json!({
                        "op": "remove",
                        "path": key_path,
                        "old_value": orig_map[key],
                        "reason": "key_removed"
                    });
                    operations.push(op);
                    paths_changed.push(key_path.clone());
                } else {
                    // Recurse for nested structures
                    self._structural_diff(&orig_map[key], &mod_map[key], key_path, operations, paths_changed);
                }
            }
        }
        // Compare list lengths
        else if let (Some(orig_list), Some(mod_list)) = (original.as_array(), modified.as_array()) {
            if orig_list.len() != mod_list.len() {
                let op = serde_json::json!({
                    "op": "replace",
                    "path": if path.is_empty() { "/" } else { &path },
                    "value": modified,
                    "old_value": original,
                    "reason": "list_length_changed"
                });
                operations.push(op);
                paths_changed.push(if path.is_empty() { "/".to_string() } else { path });
            }
        }
    }

    /// Compare content only (values).
    fn _content_diff(&self, original: &serde_json::Value, modified: &serde_json::Value, path: String, operations: &mut Vec<serde_json::Value>, paths_changed: &mut Vec<String>) {
        if original != modified {
            let op = serde_json::json!({
                "op": "replace",
                "path": if path.is_empty() { "/" } else { &path },
                "value": modified,
                "old_value": original,
                "reason": "value_changed"
            });
            operations.push(op);
            paths_changed.push(if path.is_empty() { "/".to_string() } else { path });
        }
    }

    /// Compare both structure and content recursively.
    fn _full_diff(&self, original: &serde_json::Value, modified: &serde_json::Value, path: String, operations: &mut Vec<serde_json::Value>, paths_changed: &mut Vec<String>) {
        // Handle type mismatches
        if std::mem::discriminant(original) != std::mem::discriminant(modified) {
            let op = serde_json::json!({
                "op": "replace",
                "path": if path.is_empty() { "/" } else { &path },
                "value": modified,
                "old_value": original,
                "reason": "type_changed"
            });
            operations.push(op);
            paths_changed.push(if path.is_empty() { "/".to_string() } else { path.clone() });
            return;
        }

        // Compare dictionaries recursively
        if let (Some(orig_map), Some(mod_map)) = (original.as_object(), modified.as_object()) {
            let all_keys: std::collections::HashSet<_> = orig_map.keys().chain(mod_map.keys()).collect();

            for key in all_keys {
                let key_path = if path.is_empty() {
                    format!("/{}", key)
                } else {
                    format!("{}/{}", path, key)
                };

                if !orig_map.contains_key(key) {
                    let op = serde_json::json!({
                        "op": "add",
                        "path": key_path,
                        "value": mod_map[key],
                        "reason": "key_added"
                    });
                    operations.push(op);
                    paths_changed.push(key_path.clone());
                } else if !mod_map.contains_key(key) {
                    let op = serde_json::json!({
                        "op": "remove",
                        "path": key_path,
                        "old_value": orig_map[key],
                        "reason": "key_removed"
                    });
                    operations.push(op);
                    paths_changed.push(key_path.clone());
                } else {
                    // Recurse for nested comparison
                    self._full_diff(&orig_map[key], &mod_map[key], key_path, operations, paths_changed);
                }
            }
        }
        // Compare lists
        else if let (Some(orig_list), Some(mod_list)) = (original.as_array(), modified.as_array()) {
            if orig_list != mod_list {
                let op = serde_json::json!({
                    "op": "replace",
                    "path": if path.is_empty() { "/" } else { &path },
                    "value": modified,
                    "old_value": original,
                    "reason": "list_changed"
                });
                operations.push(op);
                paths_changed.push(if path.is_empty() { "/".to_string() } else { path });
            }
        }
        // Compare scalars
        else if original != modified {
            let op = serde_json::json!({
                "op": "replace",
                "path": if path.is_empty() { "/" } else { &path },
                "value": modified,
                "old_value": original,
                "reason": "value_changed"
            });
            operations.push(op);
            paths_changed.push(if path.is_empty() { "/".to_string() } else { path });
        }
    }
}

impl IDiffOperation for DiffOperation {
    fn diff(&self, original: serde_json::Value, modified: serde_json::Value, mode: DiffMode) -> DiffResult {
        self.diff(original, modified, mode).unwrap_or_else(|e| {
            DiffResult::new(vec![], mode, vec![])
        })
    }
}

/// Convenience function for diff operations.
pub fn generate_diff(original: serde_json::Value, modified: serde_json::Value, mode: Option<DiffMode>) -> DiffResult {
    let differ = DiffOperation::new();
    differ.diff(original, modified, mode.unwrap_or(DiffMode::Full)).unwrap_or_else(|_| {
        DiffResult::new(vec![], mode.unwrap_or(DiffMode::Full), vec![])
    })
}

// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use DiffOperation;
pub use generate_diff;
