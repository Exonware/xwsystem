// #exonware/xwsystem/rust/src/validation/base.rs
//exonware/xwsystem/validation/base.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Validation module base classes - abstract classes for validation functionality.


use std::collections::HashMap;

use crate::contracts::{ConstraintType, SchemaType, ValidationLevel, ValidationType};

/// Abstract base class for validation operations.
pub trait AValidatorBase {
    /// Validate data.
    fn validate(&self, data: T) -> bool;

    /// Add validation rule.
    fn add_validation_rule(&self, rule_name: String, rule_func: fn()) -> ();

    /// Remove validation rule.
    fn remove_validation_rule(&self, rule_name: String) -> ();

    /// Get validation errors.
    fn get_validation_errors(&self) -> Vec<String>;

    /// Clear validation errors.
    fn clear_validation_errors(&self) -> ();

    /// Set validation level.
    fn set_validation_level(&self, level: ValidationLevel) -> ();

    /// Get validation level.
    fn get_validation_level(&self) -> ValidationLevel;

    /// Get validation rules.
    fn get_validation_rules(&self) -> Vec<String>;

    /// Check if data is valid.
    fn is_valid(&self, data: T) -> bool;

}

/// Abstract base class for data validation.
pub trait ADataValidatorBase {
    /// Validate data.
    fn validate_data(&self, data: serde_json::Value, data_type: Option<String>) -> bool;

    /// Validate data type.
    fn validate_type(&self, data: serde_json::Value, expected_type: String) -> bool;

    /// Validate data value against constraints.
    fn validate_value(&self, data: serde_json::Value, constraints: HashMap<String, serde_json::Value>) -> bool;

    /// Validate data range.
    fn validate_range(&self, data: i64, min_value: Option<i64>, max_value: Option<i64>) -> bool;

    /// Validate data length.
    fn validate_length(&self, data: String, min_length: Option<i64>, max_length: Option<i64>) -> bool;

    /// Validate data pattern.
    fn validate_pattern(&self, data: String, pattern: String) -> bool;

    /// Validate data against enum values.
    fn validate_enum(&self, data: serde_json::Value, enum_values: Vec<serde_json::Value>) -> bool;

    /// Validate required data.
    fn validate_required(&self, data: serde_json::Value) -> bool;

    /// Validate optional data.
    fn validate_optional(&self, data: serde_json::Value) -> bool;

    /// Register validator for data type.
    fn register_validator(&self, data_type: String, validator: AValidatorBase) -> ();

    /// Unregister validator for data type.
    fn unregister_validator(&self, data_type: String) -> ();

    /// Get validator for data type.
    fn get_validator(&self, data_type: String) -> Option<AValidatorBase>;

}

/// Abstract base class for type safety operations.
pub trait ATypeSafetyBase {
    /// Check data type.
    fn check_type(&self, data: serde_json::Value, expected_type: String) -> bool;

    /// Check multiple data types.
    fn check_types(&self, data: HashMap<String, serde_json::Value>, type_annotations: HashMap<String, String>) -> bool;

    /// Coerce data to target type.
    fn coerce_type(&self, data: serde_json::Value, target_type: String) -> serde_json::Value;

    /// Check if data is type safe.
    fn is_type_safe(&self, data: serde_json::Value, expected_type: String) -> bool;

    /// Get type information.
    fn get_type_info(&self, data: serde_json::Value) -> HashMap<String, serde_json::Value>;

    /// Validate type annotations.
    fn validate_type_annotations(&self, annotations: HashMap<String, String>) -> bool;

    /// Set strict type checking mode.
    fn set_strict_mode(&self, strict: bool) -> ();

    /// Check if strict mode is enabled.
    fn is_strict_mode(&self) -> bool;

    /// Get type errors.
    fn get_type_errors(&self) -> Vec<String>;

    /// Clear type errors.
    fn clear_type_errors(&self) -> ();

}

/// Abstract base class for declarative validation.
pub trait ADeclarativeValidatorBase {
    /// Define validation schema.
    fn define_schema(&self, schema_name: String, schema_definition: HashMap<String, serde_json::Value>) -> ();

    /// Validate data against schema.
    fn validate_against_schema(&self, data: serde_json::Value, schema_name: String) -> bool;

    /// Get schema definition.
    fn get_schema(&self, schema_name: String) -> Option<HashMap<String, serde_json::Value>>;

    /// List all schemas.
    fn list_schemas(&self) -> Vec<String>;

    /// Remove schema.
    fn remove_schema(&self, schema_name: String) -> bool;

    /// Validate schema definition.
    fn validate_schema_definition(&self, schema_definition: HashMap<String, serde_json::Value>) -> bool;

    /// Get validation result.
    fn get_validation_result(&self, schema_name: String) -> Option<HashMap<String, serde_json::Value>>;

    /// Get validation errors for schema.
    fn get_validation_errors(&self, schema_name: String) -> Vec<String>;

    /// Clear validation results.
    fn clear_validation_results(&self) -> ();

    /// Export schema.
    fn export_schema(&self, schema_name: String, format: String) -> String;

    /// Import schema.
    fn import_schema(&self, schema_name: String, schema_data: String, format: String) -> ();

}

/// Abstract base class for constraint validation.
pub trait AConstraintValidatorBase {
    /// Add constraint.
    fn add_constraint(&self, constraint_name: String, constraint_type: ConstraintType, constraint_value: serde_json::Value) -> ();

    /// Remove constraint.
    fn remove_constraint(&self, constraint_name: String) -> ();

    /// Validate data against constraint.
    fn validate_constraint(&self, data: serde_json::Value, constraint_name: String) -> bool;

    /// Validate data against all constraints.
    fn validate_all_constraints(&self, data: serde_json::Value) -> HashMap<String, bool>;

    /// Get constraint definition.
    fn get_constraint(&self, constraint_name: String) -> Option<HashMap<String, serde_json::Value>>;

    /// List all constraints.
    fn list_constraints(&self) -> Vec<String>;

    /// Get constraint type.
    fn get_constraint_type(&self, constraint_name: String) -> Option<ConstraintType>;

    /// Get constraint value.
    fn get_constraint_value(&self, constraint_name: String) -> serde_json::Value;

    /// Clear all constraints.
    fn clear_constraints(&self) -> ();

    /// Get constraint validation results.
    fn get_constraint_results(&self) -> HashMap<String, bool>;

    /// Get failed constraints.
    fn get_failed_constraints(&self) -> Vec<String>;

    /// Get passed constraints.
    fn get_passed_constraints(&self) -> Vec<String>;

}
