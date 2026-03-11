// #exonware/xwsystem/rust/src/validation/fluent_validator.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 26, 2025
//! 
//! Fluent validator with chainable API for data validation.


use std::collections::HashMap;

use crate::config::logging_setup::get_logger;
use crate::validation::declarative::ValidationError;

// Convenience functions

// Common validation rules

// Create temporary validator for field
// Collect errors with field prefix
/// Fluent validator with chainable API for data validation.
///
/// Features:
/// - Chainable validation methods
/// - Custom validator support
/// - Error collection (all errors before raising)
/// - Type checking and range validation
/// - Required field validation
/// - Custom error messages
pub struct FluentValidator {
    pub data: serde_json::Value,
    errors: Vec<String>,
    rules: Vec<HashMap<String, serde_json::Value>>,
}

impl FluentValidator {
    /// Initialize fluent validator.
    ///
    /// Args:
    /// data: Data to validate (optional, can be set later)
    pub fn new(data: Option<serde_json::Value>) -> Self {
        Self {
            data: data.unwrap_or(serde_json::Value::Null),
            errors: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Require a field to be present and not None.
    ///
    /// Args:
    /// field_name: Name of field to check
    /// message: Custom error message
    ///
    /// Returns:
    /// Self for method chaining
    pub fn require(&mut self, field_name: String, message: Option<String>) -> &mut Self {
        if let Some(obj) = self.data.as_object() {
            if !obj.contains_key(&field_name) || obj.get(&field_name).map(|v| v.is_null()).unwrap_or(true) {
                let error_msg = message.unwrap_or_else(|| format!("Field '{}' is required", field_name));
                self.errors.push(error_msg);
            }
        } else {
            let error_msg = message.unwrap_or_else(|| format!("Data must be a dictionary to check field '{}'", field_name));
            self.errors.push(error_msg);
        }
        self
    }

    /// Check if data is of expected type.
    ///
    /// Args:
    /// expected_type: Expected type name (e.g., "string", "number", "object", "array")
    /// message: Custom error message
    ///
    /// Returns:
    /// Self for method chaining
    pub fn type_check(&mut self, expected_type: &str, message: Option<String>) -> &mut Self {
        let actual_type = match self.data {
            serde_json::Value::Null => "null",
            serde_json::Value::Bool(_) => "boolean",
            serde_json::Value::Number(_) => "number",
            serde_json::Value::String(_) => "string",
            serde_json::Value::Array(_) => "array",
            serde_json::Value::Object(_) => "object",
        };
        
        if actual_type != expected_type {
            let error_msg = message.unwrap_or_else(|| format!("Expected {}, got {}", expected_type, actual_type));
            self.errors.push(error_msg);
        }
        self
    }

    /// Check if numeric data is within range.
    ///
    /// Args:
    /// min_value: Minimum allowed value
    /// max_value: Maximum allowed value
    /// message: Custom error message
    ///
    /// Returns:
    /// Self for method chaining
    pub fn range_check(&mut self, min_value: Option<i64>, max_value: Option<i64>, message: Option<String>) -> &mut Self {
        if let Some(num) = self.data.as_i64() {
            if let Some(min) = min_value {
                if num < min {
                    let error_msg = message.unwrap_or_else(|| format!("Value {} is below minimum {}", num, min));
                    self.errors.push(error_msg);
                }
            }
            if let Some(max) = max_value {
                if num > max {
                    let error_msg = message.unwrap_or_else(|| format!("Value {} is above maximum {}", num, max));
                    self.errors.push(error_msg);
                }
            }
        } else if let Some(num) = self.data.as_f64() {
            if let Some(min) = min_value {
                if num < min as f64 {
                    let error_msg = message.unwrap_or_else(|| format!("Value {} is below minimum {}", num, min));
                    self.errors.push(error_msg);
                }
            }
            if let Some(max) = max_value {
                if num > max as f64 {
                    let error_msg = message.unwrap_or_else(|| format!("Value {} is above maximum {}", num, max));
                    self.errors.push(error_msg);
                }
            }
        } else {
            let error_msg = message.unwrap_or_else(|| "Range check requires numeric data".to_string());
            self.errors.push(error_msg);
        }
        self
    }

    /// Check if data length is within range.
    ///
    /// Args:
    /// min_length: Minimum allowed length
    /// max_length: Maximum allowed length
    /// message: Custom error message
    ///
    /// Returns:
    /// Self for method chaining
    pub fn length_check(&mut self, min_length: Option<i64>, max_length: Option<i64>, message: Option<String>) -> &mut Self {
        let length = match &self.data {
            serde_json::Value::String(s) => s.len() as i64,
            serde_json::Value::Array(a) => a.len() as i64,
            serde_json::Value::Object(o) => o.len() as i64,
            _ => {
                let error_msg = message.unwrap_or_else(|| "Length check requires data with length".to_string());
                self.errors.push(error_msg);
                return self;
            }
        };
        
        if let Some(min) = min_length {
            if length < min {
                let error_msg = message.unwrap_or_else(|| format!("Length {} is below minimum {}", length, min));
                self.errors.push(error_msg);
            }
        }
        if let Some(max) = max_length {
            if length > max {
                let error_msg = message.unwrap_or_else(|| format!("Length {} is above maximum {}", length, max));
                self.errors.push(error_msg);
            }
        }
        self
    }

    /// Check if string data matches pattern (regex).
    ///
    /// Args:
    /// pattern: Regular expression pattern
    /// message: Custom error message
    ///
    /// Returns:
    /// Self for method chaining
    pub fn pattern_check(&mut self, pattern: &str, message: Option<String>) -> &mut Self {
        if let Some(s) = self.data.as_str() {
            // Use regex crate for pattern matching
            if let Ok(re) = regex::Regex::new(pattern) {
                if !re.is_match(s) {
                    let error_msg = message.unwrap_or_else(|| format!("String '{}' does not match pattern '{}'", s, pattern));
                    self.errors.push(error_msg);
                }
            } else {
                let error_msg = message.unwrap_or_else(|| format!("Invalid regex pattern: {}", pattern));
                self.errors.push(error_msg);
            }
        } else {
            let error_msg = message.unwrap_or_else(|| "Pattern check requires string data".to_string());
            self.errors.push(error_msg);
        }
        self
    }

    /// Add custom validation rule.
    ///
    /// Args:
    /// validator_func: Function that takes data and returns true if valid
    /// message: Custom error message
    ///
    /// Returns:
    /// Self for method chaining
    pub fn add_rule<F>(&mut self, validator_func: F, message: Option<String>) -> &mut Self
    where
        F: Fn(&serde_json::Value) -> bool,
    {
        if !validator_func(&self.data) {
            let error_msg = message.unwrap_or_else(|| "Custom validation rule failed".to_string());
            self.errors.push(error_msg);
        }
        self
    }

    /// Check if data is valid (no errors).
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Validate data and raise ValidationError if invalid.
    ///
    /// Returns:
    /// Self if valid
    ///
    /// Raises:
    /// ValidationError: If validation fails
    pub fn validate(&self) -> Result<&Self, ValidationError> {
        if !self.is_valid() {
            let error_msg = format!("Validation failed: {}", self.errors.join("; "));
            return Err(ValidationError::new(error_msg));
        }
        Ok(self)
    }

    /// Get list of validation errors.
    pub fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    /// Clear all validation errors.
    pub fn clear_errors(&mut self) -> &mut Self {
        self.errors.clear();
        self
    }

    /// Set data to validate.
    pub fn set_data(&mut self, data: serde_json::Value) -> &mut Self {
        self.data = data;
        self
    }

    /// Validate a specific field with rules.
    ///
    /// Args:
    /// field_name: Name of field being validated
    /// field_data: Data for the field
    /// rules: List of validation rule functions to apply
    ///
    /// Returns:
    /// Self for method chaining
    pub fn validate_field<F>(&mut self, field_name: &str, field_data: serde_json::Value, rules: Vec<F>) -> &mut Self
    where
        F: Fn(&mut FluentValidator),
    {
        // Create temporary validator for field
        let mut field_validator = FluentValidator::new(Some(field_data));
        
        // Apply rules
        for rule in rules {
            rule(&mut field_validator);
        }
        
        // Collect errors with field prefix
        for error in field_validator.get_errors() {
            self.errors.push(format!("Field '{}': {}", field_name, error));
        }
        
        self
    }

    /// Validate multiple fields in a dictionary.
    ///
    /// Args:
    /// field_rules: Dictionary mapping field names to validation rules
    ///
    /// Returns:
    /// Self for method chaining
    pub fn validate_dict_fields<F>(&mut self, field_rules: HashMap<String, Vec<F>>) -> &mut Self
    where
        F: Fn(&mut FluentValidator),
    {
        if let Some(obj) = self.data.as_object() {
            for (field_name, rules) in field_rules {
                let field_data = obj.get(&field_name).cloned().unwrap_or(serde_json::Value::Null);
                self.validate_field(&field_name, field_data, rules);
            }
        } else {
            self.errors.push("Data must be a dictionary for field validation".to_string());
        }
        self
    }

}

/// Create a fluent validator for data.
///
/// Args:
/// data: Data to validate
///
/// Returns:
/// FluentValidator instance
pub fn validate_data(data: serde_json::Value) -> FluentValidator {
    FluentValidator::new(Some(data))
}

/// Create a fluent validator for dictionary data.
///
/// Args:
/// data: Dictionary to validate
///
/// Returns:
/// FluentValidator instance
pub fn validate_dict(data: HashMap<String, serde_json::Value>) -> FluentValidator {
    FluentValidator::new(Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Object(serde_json::Map::new()))))
}

/// Create a fluent validator for string data.
///
/// Args:
/// data: String to validate
///
/// Returns:
/// FluentValidator instance
pub fn validate_string(data: &str) -> FluentValidator {
    FluentValidator::new(Some(serde_json::Value::String(data.to_string())))
}

/// Create a fluent validator for numeric data.
///
/// Args:
/// data: Numeric data to validate
///
/// Returns:
/// FluentValidator instance
pub fn validate_numeric(data: i64) -> FluentValidator {
    FluentValidator::new(Some(serde_json::Value::Number(data.into())))
}

/// Create a required field rule.
pub fn is_required(field_name: &str) -> impl Fn(&mut FluentValidator) + '_ {
    move |v: &mut FluentValidator| {
        v.require(field_name.to_string(), None);
    }
}

/// Create a type check rule.
pub fn is_type(expected_type: &str) -> impl Fn(&mut FluentValidator) + '_ {
    move |v: &mut FluentValidator| {
        v.type_check(expected_type, None);
    }
}

/// Create a range check rule.
pub fn is_in_range(min_val: Option<i64>, max_val: Option<i64>) -> impl Fn(&mut FluentValidator) {
    move |v: &mut FluentValidator| {
        v.range_check(min_val, max_val, None);
    }
}

/// Create a length check rule.
pub fn has_length(min_len: Option<i64>, max_len: Option<i64>) -> impl Fn(&mut FluentValidator) {
    move |v: &mut FluentValidator| {
        v.length_check(min_len, max_len, None);
    }
}

/// Create a pattern check rule.
pub fn matches_pattern(pattern: &str) -> impl Fn(&mut FluentValidator) + '_ {
    move |v: &mut FluentValidator| {
        v.pattern_check(pattern, None);
    }
}
// Note: The convenience functions and rule creators are exported at module level
