// #exonware/xwsystem/rust/src/validation/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Validation protocol interfaces for XWSystem.


use std::collections::HashMap;

use crate::defs::{ValidationLevel, ValidationResult, ValidationType};

// ============================================================================

// VALIDATION MANAGER INTERFACES

// ============================================================================

// ============================================================================

// SCHEMA VALIDATION INTERFACES

// ============================================================================

// ============================================================================

// VALIDATION PROTOCOLS

// ============================================================================

/// Interface for objects that can be validated.
///
/// Enforces consistent validation behavior across XWSystem.
pub trait IValidatable {
    /// Validate this object.
    /// Returns:
    /// True if valid
    fn validate(&self) -> bool;

    /// Check if object is valid.
    /// Returns:
    /// True if valid
    fn is_valid(&self) -> bool;

    /// Get validation errors.
    /// Returns:
    /// List of error messages
    fn get_errors(&self) -> Vec<String>;

    /// Clear validation errors.
    fn clear_errors(&self) -> ();

    /// Check if object has validation errors.
    /// Returns:
    /// True if has errors
    fn has_errors(&self) -> bool;

    /// Add validation error.
    /// Args:
    /// error: Error message
    fn add_error(&self, error: String) -> ();

}

/// Interface for validation management.
///
/// Enforces consistent validation management across XWSystem.
pub trait IValidationManager {
    /// Add validator function.
    /// Args:
    /// name: Validator name
    /// validator: Validator function
    fn add_validator(&self, name: String, validator: fn()) -> ();

    /// Remove validator.
    /// Args:
    /// name: Validator name
    /// Returns:
    /// True if removed
    fn remove_validator(&self, name: String) -> bool;

    /// Validate object with specified validators.
    /// Args:
    /// obj: Object to validate
    /// validators: List of validator names
    /// Returns:
    /// Tuple of (is_valid, error_messages)
    fn validate_object(&self, obj: serde_json::Value, validators: Vec<String>) -> (bool, Vec<String>);

    /// Get list of available validators.
    /// Returns:
    /// List of validator names
    fn get_validators(&self) -> Vec<String>;

}

/// Interface for schema validation.
///
/// Enforces consistent schema validation across XWSystem.
pub trait ISchemaValidator {
    /// Validate data against schema.
    /// Args:
    /// data: Data to validate
    /// schema: Schema definition
    /// Returns:
    /// Tuple of (is_valid, error_messages)
    fn validate_schema(&self, data: serde_json::Value, schema: HashMap<String, serde_json::Value>) -> (bool, Vec<String>);

    /// Create schema from data.
    /// Args:
    /// data: Data to create schema from
    /// Returns:
    /// Schema definition
    fn create_schema(&self, data: serde_json::Value) -> HashMap<String, serde_json::Value>;

    /// Validate data type.
    /// Args:
    /// data: Data to validate
    /// expected_type: Expected type name
    /// Returns:
    /// True if type matches
    fn validate_type(&self, data: serde_json::Value, expected_type: String) -> bool;

    /// Validate data range.
    /// Args:
    /// data: Data to validate
    /// min_value: Minimum value
    /// max_value: Maximum value
    /// Returns:
    /// True if in range
    fn validate_range(&self, data: serde_json::Value, min_value: serde_json::Value, max_value: serde_json::Value) -> bool;

    /// Validate string pattern.
    /// Args:
    /// data: String to validate
    /// pattern: Regex pattern
    /// Returns:
    /// True if pattern matches
    fn validate_pattern(&self, data: String, pattern: String) -> bool;

}

/// Protocol for objects that support data validation (simpler interface than IValidatable).
pub trait IValidatableSimple {
    /// Validate data against rules.
    fn validate(&self, data: serde_json::Value) -> bool;

    /// Get validation errors.
    fn get_errors(&self) -> Vec<HashMap<String, serde_json::Value>>;

}
