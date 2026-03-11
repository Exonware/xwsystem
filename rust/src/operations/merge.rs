// #exonware/xwsystem/rust/src/operations/merge.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 26, 2025
//! 
//! Merge operations implementation.

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use super::base::MergeError;
use super::contracts::IMergeOperation;
use super::defs::MergeStrategy;

/// Thread-safe merge operation implementation.
pub struct MergeOperation {
    _lock: Mutex<()>,
}

impl MergeOperation {
    /// Constructor
    pub fn new() -> Self {
        Self {
            _lock: Mutex::new(()),
        }
    }

    /// Execute merge operation.
    pub fn execute(&self, args: &[serde_json::Value], kwargs: &HashMap<String, serde_json::Value>) -> Result<serde_json::Value, MergeError> {
        if args.len() < 2 {
            return Err(MergeError::new("Merge requires at least target and source"));
        }

        let target = &args[0];
        let source = &args[1];
        let strategy = kwargs.get("strategy")
            .and_then(|v| serde_json::from_value::<MergeStrategy>(v.clone()).ok())
            .unwrap_or(MergeStrategy::Deep);

        Ok(self.merge(target.clone(), source.clone(), strategy)?)
    }

    /// Merge source into target using specified strategy.
    pub fn merge(&self, target: serde_json::Value, source: serde_json::Value, strategy: MergeStrategy) -> Result<serde_json::Value, MergeError> {
        let _guard = self._lock.lock().map_err(|e| MergeError::new(format!("Lock error: {}", e)))?;
        drop(_guard);

        match strategy {
            MergeStrategy::Deep => Ok(self._deep_merge(target, source)),
            MergeStrategy::Shallow => Ok(self._shallow_merge(target, source)),
            MergeStrategy::Overwrite => Ok(self._overwrite_merge(target, source)),
            MergeStrategy::Append => Ok(self._append_merge(target, source)),
            MergeStrategy::Unique => Ok(self._unique_merge(target, source)),
        }
    }

    /// Deep recursive merge.
    fn _deep_merge(&self, target: serde_json::Value, source: serde_json::Value) -> serde_json::Value {
        if let (Some(target_map), Some(source_map)) = (target.as_object(), source.as_object()) {
            let mut result = target_map.clone();
            
            for (key, value) in source_map {
                if let Some(existing) = result.get(key) {
                    if existing.is_object() && value.is_object() {
                        result.insert(key.clone(), self._deep_merge(existing.clone(), value.clone()));
                    } else {
                        result.insert(key.clone(), value.clone());
                    }
                } else {
                    result.insert(key.clone(), value.clone());
                }
            }
            
            serde_json::Value::Object(result)
        } else {
            source
        }
    }

    /// Shallow merge (top-level only).
    fn _shallow_merge(&self, target: serde_json::Value, source: serde_json::Value) -> serde_json::Value {
        if let (Some(target_map), Some(source_map)) = (target.as_object(), source.as_object()) {
            let mut result = target_map.clone();
            result.extend(source_map.clone());
            serde_json::Value::Object(result)
        } else {
            source
        }
    }

    /// Overwrite merge (replace entirely).
    fn _overwrite_merge(&self, _target: serde_json::Value, source: serde_json::Value) -> serde_json::Value {
        source
    }

    /// Append merge (append lists instead of replacing).
    fn _append_merge(&self, target: serde_json::Value, source: serde_json::Value) -> serde_json::Value {
        if let (Some(target_list), Some(source_list)) = (target.as_array(), source.as_array()) {
            let mut result = target_list.clone();
            result.extend_from_slice(source_list);
            serde_json::Value::Array(result)
        } else if let (Some(target_map), Some(source_map)) = (target.as_object(), source.as_object()) {
            let mut result = target_map.clone();
            for (key, value) in source_map {
                if let Some(existing) = result.get(key) {
                    if let (Some(existing_list), Some(value_list)) = (existing.as_array(), value.as_array()) {
                        let mut merged = existing_list.clone();
                        merged.extend_from_slice(value_list);
                        result.insert(key.clone(), serde_json::Value::Array(merged));
                    } else {
                        result.insert(key.clone(), value.clone());
                    }
                } else {
                    result.insert(key.clone(), value.clone());
                }
            }
            serde_json::Value::Object(result)
        } else {
            source
        }
    }

    /// Unique merge (merge lists with uniqueness).
    fn _unique_merge(&self, target: serde_json::Value, source: serde_json::Value) -> serde_json::Value {
        if let (Some(target_list), Some(source_list)) = (target.as_array(), source.as_array()) {
            let mut seen = HashSet::new();
            let mut result = Vec::new();
            
            // Add items from target first
            for item in target_list {
                let item_str = serde_json::to_string(&item).unwrap_or_default();
                if !seen.contains(&item_str) {
                    seen.insert(item_str);
                    result.push(item);
                }
            }
            
            // Add unique items from source
            for item in source_list {
                let item_str = serde_json::to_string(&item).unwrap_or_default();
                if !seen.contains(&item_str) {
                    seen.insert(item_str);
                    result.push(item);
                }
            }
            
            serde_json::Value::Array(result)
        } else if let (Some(target_map), Some(source_map)) = (target.as_object(), source.as_object()) {
            let mut result = target_map.clone();
            for (key, value) in source_map {
                if let Some(existing) = result.get(key) {
                    if let (Some(existing_list), Some(value_list)) = (existing.as_array(), value.as_array()) {
                        result.insert(key.clone(), self._unique_merge(existing.clone(), value.clone()));
                    } else {
                        result.insert(key.clone(), value.clone());
                    }
                } else {
                    result.insert(key.clone(), value.clone());
                }
            }
            serde_json::Value::Object(result)
        } else {
            source
        }
    }
}

impl IMergeOperation for MergeOperation {
    fn merge(&self, target: serde_json::Value, source: serde_json::Value, strategy: MergeStrategy) -> serde_json::Value {
        self.merge(target, source, strategy).unwrap_or_else(|_| serde_json::Value::Null)
    }
}

/// Convenience function for deep merge operations.
pub fn deep_merge(target: serde_json::Value, source: serde_json::Value, strategy: Option<MergeStrategy>) -> serde_json::Value {
    let merger = MergeOperation::new();
    merger.merge(target, source, strategy.unwrap_or(MergeStrategy::Deep)).unwrap_or_else(|_| serde_json::Value::Null)
}

// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use MergeOperation;
pub use deep_merge;
