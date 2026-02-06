// #exonware/xwsystem/rust/src/validation/data_validator.rs
//! Data Validation Utilities for XSystem
//! 
//! These utilities provide data structure validation, path validation, and memory estimation
//! capabilities. They were previously embedded in xData and have been extracted for
//! framework-wide reusability.


use crate::config::{DEFAULT_MAX_DICT_DEPTH, DEFAULT_MAX_PATH_DEPTH, DEFAULT_MAX_PATH_LENGTH, DEFAULT_MAX_RESOLUTION_DEPTH};

use super::errors::{DepthValidationError, MemoryValidationError, PathValidationError, ValidationError};

// ======================

// Path Validation

// ======================

/// Comprehensive data validator with configurable limits.
///
/// This class provides a unified interface for all data validation
/// operations with customizable limits and consistent error handling.
pub struct DataValidator {
    pub max_dict_depth: Option<i64>,
    pub max_path_length: Option<i64>,
    pub max_path_depth: Option<i64>,
    pub max_resolution_depth: Option<i64>,
}

impl DataValidator {
    /// Initialize validator with custom limits.
    ///
    ///
    /// Args:
    /// max_dict_depth: Maximum data structure nesting depth
    /// max_path_length: Maximum path string length
    /// max_path_depth: Maximum path segment count
    /// max_resolution_depth: Maximum reference resolution depth
    pub fn new(
        max_dict_depth: Option<i64>,
        max_path_length: Option<i64>,
        max_path_depth: Option<i64>,
        max_resolution_depth: Option<i64>
    ) -> Self {
        Self {
            max_dict_depth,
            max_path_length,
            max_path_depth,
            max_resolution_depth,
        }
    }

    /// Validate data structure depth and complexity.
    pub fn validate_data(
        &self,
        data: &serde_json::Value,
        operation_name: Option<&str>,
    ) -> Result<(), ValidationError> {
        self.validate_data_structure(data, operation_name)
    }

    /// Validate data structure depth and complexity.
    pub fn validate_data_structure(
        &self,
        data: &serde_json::Value,
        operation_name: Option<&str>,
    ) -> Result<(), ValidationError> {
        let max_depth = self.max_dict_depth.unwrap_or(DEFAULT_MAX_DICT_DEPTH) as usize;
        check_data_depth(data, 0, Some(max_depth))
            .map_err(|e| ValidationError::new(format!("{}: {}", operation_name.unwrap_or("data_validation"), e)))
    }

    /// Validate path string and depth.
    pub fn validate_path(
        &self,
        path: &str,
        operation_name: Option<&str>,
    ) -> Result<(), PathValidationError> {
        let max_length = self.max_path_length.unwrap_or(DEFAULT_MAX_PATH_LENGTH) as usize;
        let max_depth = self.max_path_depth.unwrap_or(DEFAULT_MAX_PATH_DEPTH) as usize;
        
        validate_path_input(path, max_length, operation_name)?;
        
        // Also validate path depth if it's a delimited path
        if path.contains('.') {
            let path_parts: Vec<&str> = path.split('.').collect();
            validate_path_depth(&path_parts, max_depth, operation_name)?;
        }
        
        Ok(())
    }

    /// Validate resolution depth.
    pub fn validate_resolution(
        &self,
        depth: usize,
        operation_name: Option<&str>,
    ) -> Result<(), DepthValidationError> {
        let max_depth = self.max_resolution_depth.unwrap_or(DEFAULT_MAX_RESOLUTION_DEPTH) as usize;
        validate_resolution_depth(depth, max_depth, operation_name)
    }

    /// Estimate memory usage of data structure.
    pub fn estimate_memory(&self, data: &serde_json::Value) -> f64 {
        estimate_memory_usage(data)
    }

    /// Validate that data structure doesn't exceed memory limits.
    pub fn validate_memory_usage(
        &self,
        data: &serde_json::Value,
        max_memory_mb: f64,
        operation_name: Option<&str>,
    ) -> Result<(), MemoryValidationError> {
        let estimated = estimate_memory_usage(data);
        if estimated > max_memory_mb {
            return Err(MemoryValidationError::new(format!(
                "{}: Estimated memory usage ({:.2} MB) exceeds maximum allowed ({} MB)",
                operation_name.unwrap_or("memory_validation"),
                estimated,
                max_memory_mb
            )));
        }
        Ok(())
    }
}

/// Check data structure depth to prevent excessive nesting.
pub fn check_data_depth(
    data: &serde_json::Value,
    current_depth: usize,
    max_depth: Option<usize>,
) -> Result<(), DepthValidationError> {
    let max = max_depth.unwrap_or(DEFAULT_MAX_DICT_DEPTH as usize);

    if current_depth > max {
        return Err(DepthValidationError::new(format!(
            "Data structure depth ({}) exceeds maximum allowed ({}). This may indicate malformed or malicious data.",
            current_depth, max
        )));
    }

    match data {
        serde_json::Value::Object(map) => {
            for value in map.values() {
                check_data_depth(value, current_depth + 1, Some(max))?;
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                check_data_depth(item, current_depth + 1, Some(max))?;
            }
        }
        _ => {
            // Primitive types don't add depth
        }
    }

    Ok(())
}

/// Validate path input for operations to prevent attacks and errors.
pub fn validate_path_input(
    path: &str,
    max_length: usize,
    operation_name: Option<&str>,
) -> Result<(), PathValidationError> {
    let op_name = operation_name.unwrap_or("path_operation");

    if path.is_empty() {
        return Err(PathValidationError::new(format!("{}: Path cannot be empty", op_name)));
    }

    // Check path length to prevent memory exhaustion
    if path.len() > max_length {
        return Err(PathValidationError::new(format!(
            "{}: Path length ({}) exceeds maximum allowed ({}). This may indicate malicious input.",
            op_name, path.len(), max_length
        )));
    }

    // Check for potentially malicious patterns
    let malicious_patterns = vec![
        "../".repeat(10),  // Directory traversal
        "/".repeat(100),   // Excessive separators
        ".".repeat(100),   // Excessive dots
        "\\".repeat(100),  // Excessive backslashes
    ];

    for pattern in malicious_patterns {
        if path.contains(&pattern) {
            return Err(PathValidationError::new(format!(
                "{}: Path contains potentially malicious pattern: {}...",
                op_name,
                &pattern[..pattern.len().min(20)]
            )));
        }
    }

    Ok(())
}

/// Validate path depth to prevent excessive traversal.
pub fn validate_path_depth(
    path_parts: &[&str],
    max_depth: usize,
    operation_name: Option<&str>,
) -> Result<(), PathValidationError> {
    let op_name = operation_name.unwrap_or("path_operation");

    if path_parts.len() > max_depth {
        return Err(PathValidationError::new(format!(
            "{}: Path depth ({}) exceeds maximum allowed ({}). This may indicate malicious input.",
            op_name, path_parts.len(), max_depth
        )));
    }

    Ok(())
}

/// Validate reference resolution depth to prevent infinite recursion.
pub fn validate_resolution_depth(
    current_depth: usize,
    max_depth: usize,
    operation_name: Option<&str>,
) -> Result<(), DepthValidationError> {
    let op_name = operation_name.unwrap_or("resolution");

    if current_depth > max_depth {
        return Err(DepthValidationError::new(format!(
            "{}: Resolution depth ({}) exceeds maximum allowed ({}). This may indicate circular references or malicious data.",
            op_name, current_depth, max_depth
        )));
    }

    Ok(())
}

/// Estimate memory usage of data structure in MB.
pub fn estimate_memory_usage(data: &serde_json::Value) -> f64 {
    // Rough estimation for JSON values
    let mut size_bytes = 0usize;

    match data {
        serde_json::Value::Object(map) => {
            // Base size for HashMap overhead
            size_bytes += std::mem::size_of::<serde_json::Map<String, serde_json::Value>>();
            for (key, value) in map {
                size_bytes += key.len() + std::mem::size_of::<String>();
                size_bytes += estimate_memory_usage(value) as usize * 1024 * 1024; // Recursive
            }
        }
        serde_json::Value::Array(arr) => {
            // Base size for Vec overhead
            size_bytes += std::mem::size_of::<Vec<serde_json::Value>>();
            for item in arr {
                size_bytes += estimate_memory_usage(item) as usize * 1024 * 1024; // Recursive
            }
        }
        serde_json::Value::String(s) => {
            size_bytes += s.len() + std::mem::size_of::<String>();
        }
        serde_json::Value::Number(_) => {
            size_bytes += std::mem::size_of::<serde_json::Number>();
        }
        serde_json::Value::Bool(_) => {
            size_bytes += std::mem::size_of::<bool>();
        }
        serde_json::Value::Null => {
            size_bytes += 1;
        }
    }

    // Convert to MB
    (size_bytes as f64) / (1024.0 * 1024.0)
}
