// #exonware/xwsystem/rust/src/io/serialization/utils/path_ops.rs
//exonware/xwsystem/src/exonware/xwsystem/io/serialization/utils/path_ops.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 9, 2025
//! 
//! Path operations utilities for serialization formats.
//! 
//! Provides JSONPointer path parsing and manipulation utilities that serializers
//! can use for path-based operations. Includes path validation for security.


use crate::io::serialization::errors::SerializationError;

/// Raised when path operations fail.
#[derive(Debug)]
pub struct PathOperationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PathOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PathOperationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PathOperationError {
    pub fn new(message: impl Into<String>) -> Self {
        PathOperationError {
            message: message.into(),
            source: None,
        }
    }
}

impl From<PathOperationError> for SerializationError {
    fn from(err: PathOperationError) -> Self {
        SerializationError::new(err.message)
    }
}

/// Path component can be either a string key or an integer index
#[derive(Debug, Clone, PartialEq)]
pub enum PathComponent {
    Key(String),
    Index(usize),
}

// JSONPointer must start with / for non-empty paths
// Validate path segments (basic validation)
// Empty segments not allowed except for root
/// Validate a JSONPointer path expression.
///
/// JSONPointer syntax: /path/to/key or /path/to/0 for arrays
///
///
/// Args:
/// path: JSONPointer path to validate
///
///
/// Returns:
/// True if path is valid
///
///
/// Raises:
/// ValueError: If path is invalid
///
/// Example:
/// >>> validate_json_pointer("/users/0/name")  # Valid
/// True
/// >>> validate_json_pointer("invalid")  # Invalid - must start with /
/// ValueError
pub fn validate_json_pointer(path: &str) -> Result<bool, PathOperationError> {
    if path.is_empty() {
        return Err(PathOperationError::new("Path cannot be empty"));
    }
    
    // JSONPointer must start with / for non-empty paths
    if path != "/" && !path.starts_with('/') {
        return Err(PathOperationError::new(format!(
            "JSONPointer path must start with '/', got: {}", path
        )));
    }
    
    // Validate path segments (basic validation)
    if path != "/" {
        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        for segment in segments {
            // Empty segments not allowed except for root
            if segment.is_empty() && segment != "0" {
                return Err(PathOperationError::new(format!(
                    "Invalid path segment in JSONPointer: {}", path
                )));
            }
        }
    }
    
    Ok(true)
}

// Handle escaped characters in JSONPointer
// Try to convert to int if it looks like a number (for array indices)
/// Parse a JSONPointer path into a list of keys/indices.
///
///
/// Args:
/// path: JSONPointer path (e.g., "/users/0/name")
///
///
/// Returns:
/// List of path components (strings for keys, ints for array indices)
///
///
/// Raises:
/// ValueError: If path is invalid
///
/// Example:
/// >>> parse_json_pointer("/users/0/name")
/// ['users', 0, 'name']
/// >>> parse_json_pointer("/")
/// []
pub fn parse_json_pointer(path: &str) -> Result<Vec<PathComponent>, PathOperationError> {
    validate_json_pointer(path)?;
    
    if path == "/" {
        return Ok(Vec::new());
    }
    
    let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    let mut result = Vec::new();
    
    for segment in segments {
        // Handle escaped characters in JSONPointer
        let segment = segment.replace("~1", "/").replace("~0", "~");
        
        // Try to convert to int if it looks like a number (for array indices)
        if let Ok(index) = segment.parse::<usize>() {
            result.push(PathComponent::Index(index));
        } else {
            result.push(PathComponent::Key(segment));
        }
    }
    
    Ok(result)
}

/// Get value from data structure using JSONPointer path.
///
///
/// Args:
/// data: Data structure (dict, list, etc.)
/// path: JSONPointer path
///
///
/// Returns:
/// Value at the specified path
///
///
/// Raises:
/// KeyError: If path doesn't exist
/// ValueError: If path is invalid or cannot navigate
///
/// Example:
/// >>> data = {"users": [{"name": "John"}]}
/// >>> get_value_by_path(data, "/users/0/name")
/// 'John'
pub fn get_value_by_path(data: &serde_json::Value, path: &str) -> Result<serde_json::Value, PathOperationError> {
    let path_parts = parse_json_pointer(path)?;
    let mut current = data;
    
    for part in path_parts {
        match part {
            PathComponent::Key(key) => {
                if let serde_json::Value::Object(map) = current {
                    if let Some(value) = map.get(&key) {
                        current = value;
                    } else {
                        return Err(PathOperationError::new(format!(
                            "Path not found: {} (missing key: {})", path, key
                        )));
                    }
                } else {
                    return Err(PathOperationError::new(format!(
                        "Cannot navigate path {}: expected object, got {}", 
                        path, 
                        match current {
                            serde_json::Value::Array(_) => "array",
                            serde_json::Value::String(_) => "string",
                            serde_json::Value::Number(_) => "number",
                            serde_json::Value::Bool(_) => "bool",
                            serde_json::Value::Null => "null",
                            _ => "unknown",
                        }
                    )));
                }
            }
            PathComponent::Index(index) => {
                if let serde_json::Value::Array(arr) = current {
                    if index >= arr.len() {
                        return Err(PathOperationError::new(format!(
                            "Index {} out of range for list at path: {}", index, path
                        )));
                    }
                    current = &arr[index];
                } else {
                    return Err(PathOperationError::new(format!(
                        "Cannot use integer index '{}' on non-list at path: {}", index, path
                    )));
                }
            }
        }
    }
    
    // Clone the final value since we can't return a reference
    Ok(current.clone())
}

// Navigate to parent of target
// Create intermediate dict
// Extend list if needed
// Convert to dict if possible
// Extend list if needed
/// Set value in data structure using JSONPointer path.
///
///
/// Args:
/// data: Data structure (dict, list, etc.)
/// path: JSONPointer path
/// value: Value to set
/// create: If True, create missing intermediate paths
///
///
/// Raises:
/// KeyError: If path doesn't exist and create=False
/// ValueError: If path is invalid or cannot navigate
///
/// Example:
/// >>> data = {"users": [{"name": "John"}]}
/// >>> set_value_by_path(data, "/users/0/name", "Jane")
/// >>> data["users"][0]["name"]
/// 'Jane'
pub fn set_value_by_path(data: &mut serde_json::Value, path: &str, value: serde_json::Value, create: Option<bool>) -> Result<(), PathOperationError> {
    let path_parts = parse_json_pointer(path)?;
    let create = create.unwrap_or(false);
    
    if path_parts.is_empty() {
        return Err(PathOperationError::new("Cannot set value at root path '/'"));
    }
    
    let mut current = data;
    
    // Navigate to parent of target
    for i in 0..path_parts.len() - 1 {
        let part = &path_parts[i];
        match part {
            PathComponent::Key(key) => {
                if let serde_json::Value::Object(map) = current {
                    if !map.contains_key(key) {
                        if create {
                            // Create intermediate dict
                            map.insert(key.clone(), serde_json::Value::Object(serde_json::Map::new()));
                        } else {
                            return Err(PathOperationError::new(format!(
                                "Path not found: {} (missing key: {})", path, key
                            )));
                        }
                    }
                    current = map.get_mut(key).unwrap();
                } else {
                    if create {
                        return Err(PathOperationError::new(format!(
                            "Cannot create path {}: intermediate value is not an object", path
                        )));
                    }
                    return Err(PathOperationError::new(format!(
                        "Cannot navigate path {}: reached non-container type", path
                    )));
                }
            }
            PathComponent::Index(index) => {
                if let serde_json::Value::Array(arr) = current {
                    if *index >= arr.len() {
                        if create {
                            // Extend list if needed
                            while arr.len() <= *index {
                                arr.push(serde_json::Value::Object(serde_json::Map::new()));
                            }
                        } else {
                            return Err(PathOperationError::new(format!(
                                "Index {} out of range", index
                            )));
                        }
                    }
                    current = &mut arr[*index];
                } else {
                    return Err(PathOperationError::new(format!(
                        "Cannot use non-integer index '{}' on non-list", index
                    )));
                }
            }
        }
    }
    
    // Set the value
    let final_key = &path_parts[path_parts.len() - 1];
    match final_key {
        PathComponent::Key(key) => {
            if let serde_json::Value::Object(map) = current {
                map.insert(key.clone(), value);
            } else {
                return Err(PathOperationError::new(format!(
                    "Cannot set value at {}: parent is not an object", path
                )));
            }
        }
        PathComponent::Index(index) => {
            if let serde_json::Value::Array(arr) = current {
                if *index >= arr.len() {
                    if create {
                        // Extend list if needed
                        while arr.len() <= *index {
                            arr.push(serde_json::Value::Null);
                        }
                    } else {
                        return Err(PathOperationError::new(format!(
                            "Index {} out of range", index
                        )));
                    }
                }
                arr[*index] = value;
            } else {
                return Err(PathOperationError::new(format!(
                    "Cannot set value at {}: parent is not a list", path
                )));
            }
        }
    }
    
    Ok(())
}

    // Check for path traversal
    /// Validate path for security concerns (injection prevention).
    ///
    /// Checks for:
    /// - Path traversal attempts (../, ..\)
    /// - Excessive depth
    /// - Suspicious patterns
    ///
    ///
    /// Args:
    /// path: Path to validate
    /// max_depth: Maximum allowed path depth
    ///
    ///
    /// Returns:
    /// True if path is safe
    ///
    ///
    /// Raises:
    /// ValueError: If path contains security issues
    ///
    ///
    /// Note:
    /// This is a basic security check. Format-specific serializers should
    /// implement additional validation as needed.
pub fn validate_path_security(path: &str, max_depth: Option<i64>) -> Result<bool, PathOperationError> {
    let max_depth = max_depth.unwrap_or(100);
    
    // Check for path traversal
    if path.contains("..") || path.contains("..\\") || path.contains("../") {
        return Err(PathOperationError::new(format!(
            "Path traversal detected in path: {}", path
        )));
    }
    
    // Check depth
    let depth = path.matches('/').count();
    if depth > max_depth as usize {
        return Err(PathOperationError::new(format!(
            "Path depth {} exceeds maximum {}: {}", depth, max_depth, path
        )));
    }
    
    // Check for null bytes
    if path.contains('\x00') {
        return Err(PathOperationError::new(format!(
            "Null byte detected in path: {}", path
        )));
    }
    
    Ok(true)
}

/// Normalize a JSONPointer path.
///
/// Ensures path follows JSONPointer standard format.
///
///
/// Args:
/// path: Path to normalize
///
///
/// Returns:
/// Normalized path
///
/// Example:
/// >>> normalize_path("users/0/name")
/// '/users/0/name'
/// >>> normalize_path("/users/0/name/")
/// '/users/0/name'
pub fn normalize_path(path: &str) -> String {
    if path.is_empty() {
        return "/".to_string();
    }
    
    let mut normalized = path.to_string();
    
    // Ensure starts with /
    if !normalized.starts_with('/') {
        normalized = format!("/{}", normalized);
    }
    
    // Remove trailing /
    normalized = normalized.trim_end_matches('/').to_string();
    
    // Handle root case
    if normalized.is_empty() {
        return "/".to_string();
    }
    
    normalized
}
