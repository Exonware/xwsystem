// #exonware/xwsystem/rust/src/operations/patch.rs
//! #exonware/xwsystem/src/exonware/xwsystem/operations/patch.py
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 27, 2025
//! 
//! Patch operations implementation (RFC 6902 JSON Patch).

use std::collections::HashMap;
use std::sync::Mutex;

use super::base::PatchError;
use super::contracts::IPatchOperation;
use super::defs::PatchResult;

/// Thread-safe patch operation implementation.
pub struct PatchOperationImpl {
    _lock: Mutex<()>,
}

impl PatchOperationImpl {
    /// Constructor
    pub fn new() -> Self {
        Self {
            _lock: Mutex::new(()),
        }
    }

    /// Execute patch operation.
    pub fn execute(&self, args: &[serde_json::Value], _kwargs: &HashMap<String, serde_json::Value>) -> Result<serde_json::Value, PatchError> {
        if args.len() < 2 {
            return Err(PatchError::new("Patch requires data and operations"));
        }

        let data = &args[0];
        let operations = &args[1];
        
        let operations_vec: Vec<HashMap<String, serde_json::Value>> = serde_json::from_value(operations.clone())
            .map_err(|e| PatchError::new(format!("Invalid operations format: {}", e)))?;

        let result = self.apply_patch(data.clone(), operations_vec)?;
        Ok(result.result)
    }

    /// Apply patch operations to data (RFC 6902 JSON Patch).
    pub fn apply_patch(&self, data: serde_json::Value, operations: Vec<HashMap<String, serde_json::Value>>) -> Result<PatchResult, PatchError> {
        let _guard = self._lock.lock().map_err(|e| PatchError::new(format!("Lock error: {}", e)))?;
        drop(_guard);

        let mut result = data;
        let mut operations_applied = 0;
        let mut errors = Vec::new();

        for (i, operation) in operations.iter().enumerate() {
            match self._apply_single_operation(&mut result, operation) {
                Ok(_) => {
                    operations_applied += 1;
                }
                Err(e) => {
                    errors.push(format!("Operation {}: {}", i, e));
                }
            }
        }

        Ok(PatchResult::new(
            errors.is_empty(),
            operations_applied,
            errors,
            result,
        ))
    }

    fn _apply_single_operation(&self, data: &mut serde_json::Value, operation: &HashMap<String, serde_json::Value>) -> Result<(), PatchError> {
        let op_type = operation.get("op")
            .and_then(|v| v.as_str())
            .ok_or_else(|| PatchError::new("Missing 'op' field in operation"))?;
        
        let path = operation.get("path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "/".to_string());

        match op_type {
            "add" => {
                let value = operation.get("value")
                    .ok_or_else(|| PatchError::new("Missing 'value' field for add operation"))?;
                *data = self._apply_add(data.clone(), path, value.clone())?;
            }
            "remove" => {
                *data = self._apply_remove(data.clone(), path)?;
            }
            "replace" => {
                let value = operation.get("value")
                    .ok_or_else(|| PatchError::new("Missing 'value' field for replace operation"))?;
                *data = self._apply_replace(data.clone(), path, value.clone())?;
            }
            "move" => {
                let from_path = operation.get("from")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'from' field for move operation"))?;
                *data = self._apply_move(data.clone(), from_path.to_string(), path)?;
            }
            "copy" => {
                let from_path = operation.get("from")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'from' field for copy operation"))?;
                *data = self._apply_copy(data.clone(), from_path.to_string(), path)?;
            }
            "test" => {
                let value = operation.get("value")
                    .ok_or_else(|| PatchError::new("Missing 'value' field for test operation"))?;
                self._apply_test(data.clone(), path, value.clone())?;
            }
            _ => {
                return Err(PatchError::new(format!("Unknown operation type: {}", op_type)));
            }
        }

        Ok(())
    }

    /// Parse JSON Pointer path.
    fn _parse_path(&self, path: &str) -> Vec<String> {
        if path == "/" {
            return Vec::new();
        }

        // Remove leading slash and split
        let parts: Vec<String> = path.trim_start_matches('/').split('/')
            .map(|part| {
                // Unescape special characters
                part.replace("~1", "/").replace("~0", "~")
            })
            .collect();

        parts
    }

    /// Get value at path.
    fn _get_value(&self, data: &serde_json::Value, path: &str) -> Result<serde_json::Value, PatchError> {
        let parts = self._parse_path(path);
        
        let mut current = data;
        for part in &parts {
            match current {
                serde_json::Value::Object(map) => {
                    current = map.get(part)
                        .ok_or_else(|| PatchError::new(format!("Path not found: {}", path)))?;
                }
                serde_json::Value::Array(arr) => {
                    let index: usize = part.parse()
                        .map_err(|_| PatchError::new(format!("Invalid array index: {}", part)))?;
                    current = arr.get(index)
                        .ok_or_else(|| PatchError::new(format!("Array index out of bounds: {}", index)))?;
                }
                _ => {
                    return Err(PatchError::new(format!("Cannot navigate path: {}", path)));
                }
            }
        }

        Ok(current.clone())
    }

    /// Set value at path.
    fn _set_value(&self, mut data: serde_json::Value, path: &str, value: serde_json::Value) -> Result<serde_json::Value, PatchError> {
        let parts = self._parse_path(path);
        
        if parts.is_empty() {
            return Ok(value);
        }

        let mut current = &mut data;
        for i in 0..parts.len() - 1 {
            let part = &parts[i];
            match current {
                serde_json::Value::Object(map) => {
                    if !map.contains_key(part) {
                        // Create intermediate structure
                        let next_part = &parts[i + 1];
                        let intermediate = if next_part.parse::<usize>().is_ok() {
                            serde_json::Value::Array(Vec::new())
                        } else {
                            serde_json::Value::Object(serde_json::Map::new())
                        };
                        map.insert(part.clone(), intermediate);
                    }
                    current = map.get_mut(part)
                        .ok_or_else(|| PatchError::new(format!("Path navigation failed: {}", path)))?;
                }
                serde_json::Value::Array(arr) => {
                    let index: usize = part.parse()
                        .map_err(|_| PatchError::new(format!("Invalid array index: {}", part)))?;
                    current = arr.get_mut(index)
                        .ok_or_else(|| PatchError::new(format!("Array index out of bounds: {}", index)))?;
                }
                _ => {
                    return Err(PatchError::new(format!("Cannot navigate path: {}", path)));
                }
            }
        }

        // Set the final value
        let last_part = &parts[parts.len() - 1];
        match current {
            serde_json::Value::Object(map) => {
                map.insert(last_part.clone(), value);
            }
            serde_json::Value::Array(arr) => {
                let index: usize = last_part.parse()
                    .map_err(|_| PatchError::new(format!("Invalid array index: {}", last_part)))?;
                if index == arr.len() {
                    arr.push(value);
                } else if index < arr.len() {
                    arr[index] = value;
                } else {
                    return Err(PatchError::new(format!("Array index out of bounds: {}", index)));
                }
            }
            _ => {
                return Err(PatchError::new(format!("Cannot set value at path: {}", path)));
            }
        }

        Ok(data)
    }

    /// Apply add operation.
    fn _apply_add(&self, data: serde_json::Value, path: String, value: serde_json::Value) -> Result<serde_json::Value, PatchError> {
        self._set_value(data, &path, value)
    }

    /// Apply remove operation.
    fn _apply_remove(&self, mut data: serde_json::Value, path: String) -> Result<serde_json::Value, PatchError> {
        let parts = self._parse_path(&path);
        
        if parts.is_empty() {
            return Err(PatchError::new("Cannot remove root"));
        }

        let mut current = &mut data;
        for i in 0..parts.len() - 1 {
            let part = &parts[i];
            match current {
                serde_json::Value::Object(map) => {
                    current = map.get_mut(part)
                        .ok_or_else(|| PatchError::new(format!("Path not found: {}", path)))?;
                }
                serde_json::Value::Array(arr) => {
                    let index: usize = part.parse()
                        .map_err(|_| PatchError::new(format!("Invalid array index: {}", part)))?;
                    current = arr.get_mut(index)
                        .ok_or_else(|| PatchError::new(format!("Array index out of bounds: {}", index)))?;
                }
                _ => {
                    return Err(PatchError::new(format!("Cannot navigate path: {}", path)));
                }
            }
        }

        let last_part = &parts[parts.len() - 1];
        match current {
            serde_json::Value::Object(map) => {
                map.remove(last_part)
                    .ok_or_else(|| PatchError::new(format!("Key not found: {}", last_part)))?;
            }
            serde_json::Value::Array(arr) => {
                let index: usize = last_part.parse()
                    .map_err(|_| PatchError::new(format!("Invalid array index: {}", last_part)))?;
                if index < arr.len() {
                    arr.remove(index);
                } else {
                    return Err(PatchError::new(format!("Array index out of bounds: {}", index)));
                }
            }
            _ => {
                return Err(PatchError::new(format!("Cannot remove at path: {}", path)));
            }
        }

        Ok(data)
    }

    /// Apply replace operation.
    fn _apply_replace(&self, data: serde_json::Value, path: String, value: serde_json::Value) -> Result<serde_json::Value, PatchError> {
        // First remove, then add
        let data = self._apply_remove(data, path.clone())?;
        self._apply_add(data, path, value)
    }

    /// Apply move operation.
    fn _apply_move(&self, data: serde_json::Value, from_path: String, to_path: String) -> Result<serde_json::Value, PatchError> {
        let value = self._get_value(&data, &from_path)?;
        let data = self._apply_remove(data, from_path)?;
        self._apply_add(data, to_path, value)
    }

    /// Apply copy operation.
    fn _apply_copy(&self, data: serde_json::Value, from_path: String, to_path: String) -> Result<serde_json::Value, PatchError> {
        let value = self._get_value(&data, &from_path)?;
        // Deep clone the value
        let cloned_value = serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
        self._apply_add(data, to_path, cloned_value)
    }

    /// Apply test operation.
    fn _apply_test(&self, data: serde_json::Value, path: String, value: serde_json::Value) -> Result<(), PatchError> {
        let current_value = self._get_value(&data, &path)?;
        if current_value != value {
            return Err(PatchError::new(format!(
                "Test failed at {}: expected {:?}, got {:?}",
                path, value, current_value
            )));
        }
        Ok(())
    }
}

impl IPatchOperation for PatchOperationImpl {
    fn apply_patch(&self, data: serde_json::Value, operations: Vec<HashMap<String, serde_json::Value>>) -> PatchResult {
        self.apply_patch(data, operations).unwrap_or_else(|e| {
            PatchResult::new(false, 0, vec![e.to_string()], serde_json::Value::Null)
        })
    }
}

/// Convenience function for patch operations.
pub fn apply_patch(data: serde_json::Value, operations: Vec<HashMap<String, serde_json::Value>>) -> PatchResult {
    let patcher = PatchOperationImpl::new();
    patcher.apply_patch(data, operations).unwrap_or_else(|e| {
        PatchResult::new(false, 0, vec![e.to_string()], serde_json::Value::Null)
    })
}

// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use PatchOperationImpl;
pub use apply_patch;
