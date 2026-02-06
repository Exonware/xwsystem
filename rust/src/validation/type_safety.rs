// #exonware/xwsystem/rust/src/validation/type_safety.rs
//! Generic type safety validation.
//! 
//! This module provides type validation utilities for safe operations
//! with untrusted data in any application.

/// Base exception for generic security-related errors.
#[derive(Debug)]
pub struct GenericSecurityError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for GenericSecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GenericSecurityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl GenericSecurityError {
    pub fn new(message: impl Into<String>) -> Self {
        GenericSecurityError {
            message: message.into(),
            source: None,
        }
    }

}

/// Validates types for safe operations.
pub struct SafeTypeValidator;

impl SafeTypeValidator {
    /// Check if a value is of a safe type.
    pub fn is_safe_type(value: &serde_json::Value) -> bool {
        matches!(
            value,
            serde_json::Value::String(_)
                | serde_json::Value::Number(_)
                | serde_json::Value::Bool(_)
                | serde_json::Value::Array(_)
                | serde_json::Value::Object(_)
                | serde_json::Value::Null
        )
    }

    /// Check if a value is of an immutable type.
    pub fn is_immutable_type(value: &serde_json::Value) -> bool {
        matches!(
            value,
            serde_json::Value::String(_)
                | serde_json::Value::Number(_)
                | serde_json::Value::Bool(_)
                | serde_json::Value::Null
        )
    }

    /// Validate data from untrusted sources.
    pub fn validate_untrusted_data(
        data: &serde_json::Value,
        max_depth: Option<usize>,
    ) -> Result<(), GenericSecurityError> {
        let max = max_depth.unwrap_or(100);
        Self::_check_recursive(data, 0, max)
    }

    fn _check_recursive(
        obj: &serde_json::Value,
        depth: usize,
        max_depth: usize,
    ) -> Result<(), GenericSecurityError> {
        if depth > max_depth {
            return Err(GenericSecurityError::new("Data structure too deep"));
        }

        if !Self::is_safe_type(obj) {
            return Err(GenericSecurityError::new(format!(
                "Unsafe type detected: {:?}",
                obj
            )));
        }

        match obj {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    if !key.is_empty() {
                        // In Rust, JSON object keys are always strings, so this check is simpler
                        Self::_check_recursive(value, depth + 1, max_depth)?;
                    }
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    Self::_check_recursive(item, depth + 1, max_depth)?;
                }
            }
            _ => {
                // Primitive types are safe
            }
        }

        Ok(())
    }

    /// Sanitize data for safe caching by ensuring immutable types.
    pub fn sanitize_for_caching(data: &serde_json::Value) -> serde_json::Value {
        if Self::is_immutable_type(data) {
            data.clone()
        } else if let serde_json::Value::Array(arr) = data {
            let sanitized: Vec<serde_json::Value> = arr
                .iter()
                .map(|item| Self::sanitize_for_caching(item))
                .collect();
            serde_json::Value::Array(sanitized)
        } else if let serde_json::Value::Object(map) = data {
            let mut sorted_pairs: Vec<(&String, serde_json::Value)> = map
                .iter()
                .map(|(k, v)| (k, Self::sanitize_for_caching(v)))
                .collect();
            sorted_pairs.sort_by(|a, b| a.0.cmp(b.0));
            
            let mut result = serde_json::Map::new();
            for (key, value) in sorted_pairs {
                result.insert(key.clone(), value);
            }
            serde_json::Value::Object(result)
        } else {
            // For non-safe types, return a safe representation
            serde_json::Value::String(format!("{:?}", data))
        }
    }
}

/// Convenience function for validating untrusted data.
pub fn validate_untrusted_data(
    data: &serde_json::Value,
    max_depth: Option<usize>,
) -> Result<(), GenericSecurityError> {
    SafeTypeValidator::validate_untrusted_data(data, max_depth)
}

/// Convenience function for checking safe types.
pub fn is_safe_type(value: &serde_json::Value) -> bool {
    SafeTypeValidator::is_safe_type(value)
}

/// Convenience function for checking immutable types.
pub fn is_immutable_type(value: &serde_json::Value) -> bool {
    SafeTypeValidator::is_immutable_type(value)
}
