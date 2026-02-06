// #exonware/xwsystem/rust/src/validation/declarative.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Pydantic-style declarative validation with type hints and automatic coercion.


use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

use crate::config::logging_setup::get_logger;

/// Raised when validation fails.
#[derive(Debug)]
pub struct ValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    errors: Vec<FieldError>,
}

#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub value: Option<serde_json::Value>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.errors.is_empty() {
            let error_msgs: Vec<String> = self.errors.iter()
                .map(|e| format!("{}: {}", e.field, e.message))
                .collect();
            write!(f, "Validation failed: {}", error_msgs.join("; "))
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        ValidationError {
            message: message.into(),
            source: None,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, field: String, message: String, value: Option<serde_json::Value>) {
        self.errors.push(FieldError { field, message, value });
    }
}

/// Field configuration for validation and metadata.
#[derive(Debug, Clone)]
pub struct Field {
    pub default: Option<serde_json::Value>,
    pub alias: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    
    // Validation constraints
    pub gt: Option<f64>,  // Greater than
    pub ge: Option<f64>,  // Greater than or equal
    pub lt: Option<f64>,  // Less than
    pub le: Option<f64>,  // Less than or equal
    pub min_length: Option<i64>,
    pub max_length: Option<i64>,
    pub pattern: Option<String>,
    pub enum_values: Option<Vec<serde_json::Value>>,
    
    // String validation
    pub strip_whitespace: bool,
    pub to_lower: bool,
    pub to_upper: bool,
    
    // Advanced constraints
    pub multiple_of: Option<f64>,
    pub allow_inf_nan: bool,
    
    // Metadata
    pub examples: Option<Vec<serde_json::Value>>,
    pub deprecated: bool,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            default: None,
            alias: None,
            title: None,
            description: None,
            gt: None,
            ge: None,
            lt: None,
            le: None,
            min_length: None,
            max_length: None,
            pattern: None,
            enum_values: None,
            strip_whitespace: true,
            to_lower: false,
            to_upper: false,
            multiple_of: None,
            allow_inf_nan: true,
            examples: None,
            deprecated: false,
        }
    }
}

impl Field {
    pub fn new() -> Self {
        Self::default()
    }
}

// Note: Rust doesn't have metaclasses like Python.
// Field processing is handled at compile time or through macros.
// For now, we'll use a simpler approach with explicit field registration.

// Get all field information
// Check if field is optional
// Validate and coerce value
// Check for extra fields
// Get the actual type (handle Optional, Union, etc.)
// Apply field constraints
// Filter out NoneType for Optional
// Multiple non-None types, return the first one for now
// Try to parse as JSON array
// Split by comma as fallback
// Try common datetime formats
// Default: try direct conversion
// Check for NaN or infinity
// Collection constraints
// Convert complex types to serializable forms
// Determine if field is required
// Generate property schema
// Handle Optional types
// Default to string for unknown types
/// Base class for declarative validation models (Pydantic-style).
///
/// Features:
/// - Type hint-based validation
/// - Automatic type coercion
/// - Field constraints and validation
/// - JSON schema generation
/// - Serialization/deserialization
/// - IDE support and type checking
pub struct XModel {
    _fields: HashMap<String, Field>,
    _field_types: HashMap<String, String>,
    _data: HashMap<String, serde_json::Value>,
}

impl XModel {
    /// Initialize model with data validation and coercion.
    pub fn new() -> Self {
        Self {
            _fields: HashMap::new(),
            _field_types: HashMap::new(),
            _data: HashMap::new(),
        }
    }

    /// Initialize model with field definitions.
    pub fn with_fields(fields: HashMap<String, Field>, field_types: HashMap<String, String>) -> Self {
        Self {
            _fields: fields,
            _field_types: field_types,
            _data: HashMap::new(),
        }
    }

    /// Validate and coerce input data.
    pub fn _validate_and_coerce(&self, data: HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>, ValidationError> {
        let mut validated = HashMap::new();
        let mut errors = ValidationError::new("Validation failed");
        
        // Process each field
        for (field_name, field_type) in &self._field_types {
            let field_config = self._fields.get(field_name).cloned().unwrap_or_else(Field::new);
            let alias = field_config.alias.as_ref().unwrap_or(field_name);
            
            // Get value from data
            let value = if let Some(v) = data.get(alias) {
                v.clone()
            } else if let Some(v) = data.get(field_name) {
                v.clone()
            } else if let Some(default) = &field_config.default {
                default.clone()
            } else {
                // Check if field is optional
                if Self::_is_optional(field_type) {
                    serde_json::Value::Null
                } else {
                    errors.add_error(field_name.clone(), "Field is required".to_string(), None);
                    continue;
                }
            };
            
            // Validate and coerce value
            match self._validate_field(field_name, value, field_type, &field_config) {
                Ok(validated_value) => {
                    validated.insert(field_name.clone(), validated_value);
                }
                Err(e) => {
                    errors.add_error(field_name.clone(), e.to_string(), None);
                }
            }
        }
        
        // Check for extra fields
        let all_field_names: HashSet<String> = self._field_types.keys().cloned().collect();
        let all_aliases: HashSet<String> = self._fields.values()
            .filter_map(|f| f.alias.clone())
            .collect();
        let provided_fields: HashSet<String> = data.keys().cloned().collect();
        let extra_fields: Vec<String> = provided_fields.iter()
            .filter(|k| !all_field_names.contains(*k) && !all_aliases.contains(*k))
            .cloned()
            .collect();
        
        for extra_field in extra_fields {
            errors.add_error(extra_field.clone(), "Extra field not allowed".to_string(), data.get(&extra_field).cloned());
        }
        
        if !errors.errors.is_empty() {
            return Err(errors);
        }
        
        Ok(validated)
    }

    /// Validate and coerce a single field.
    pub fn _validate_field(&self, field_name: &str, value: serde_json::Value, field_type: &str, field_config: &Field) -> Result<serde_json::Value, ValidationError> {
        if value.is_null() {
            if Self::_is_optional(field_type) {
                return Ok(serde_json::Value::Null);
            } else {
                return Err(ValidationError::new("None is not allowed for non-optional field"));
            }
        }
        
        // Get the actual type (handle Optional, Union, etc.)
        let actual_type = Self::_get_actual_type(field_type);
        
        // Type coercion
        let coerced_value = self._coerce_type(value, &actual_type, field_name)?;
        
        // Apply field constraints
        self._apply_constraints(&coerced_value, field_config, field_name)?;
        
        Ok(coerced_value)
    }

    /// Check if field type is Optional.
    pub fn _is_optional(field_type: &str) -> bool {
        field_type.starts_with("Option<") || field_type.contains("Optional")
    }

    /// Get actual type from Optional/Union types.
    pub fn _get_actual_type(field_type: &str) -> String {
        // Handle Option<T> -> T
        if field_type.starts_with("Option<") && field_type.ends_with('>') {
            return field_type[7..field_type.len()-1].to_string();
        }
        
        // Handle Optional[T] -> T
        if let Some(start) = field_type.find("Optional[") {
            if let Some(end) = field_type.rfind(']') {
                return field_type[start+9..end].to_string();
            }
        }
        
        // Handle Union types - return first non-None type
        if let Some(start) = field_type.find("Union[") {
            if let Some(end) = field_type.rfind(']') {
                let types_str = &field_type[start+6..end];
                let types: Vec<&str> = types_str.split(',').map(|s| s.trim()).collect();
                // Filter out NoneType for Optional
                for t in types {
                    if t != "None" && !t.contains("None") {
                        return t.to_string();
                    }
                }
            }
        }
        
        field_type.to_string()
    }

    /// Coerce value to target type.
    pub fn _coerce_type(&self, value: serde_json::Value, target_type: &str, field_name: &str) -> Result<serde_json::Value, ValidationError> {
        match target_type {
            "String" | "str" => {
                match value {
                    serde_json::Value::String(s) => Ok(serde_json::Value::String(s)),
                    _ => Ok(serde_json::Value::String(value.to_string())),
                }
            }
            "i64" | "int" | "i32" => {
                match value {
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            Ok(serde_json::Value::Number(i.into()))
                        } else {
                            Err(ValidationError::new(format!("Cannot convert to integer: {}", n)))
                        }
                    }
                    serde_json::Value::String(s) => {
                        s.parse::<i64>()
                            .map(|i| serde_json::Value::Number(i.into()))
                            .map_err(|_| ValidationError::new(format!("Cannot convert '{}' to int", s)))
                    }
                    _ => Err(ValidationError::new(format!("Cannot convert {} to int", value))),
                }
            }
            "f64" | "float" => {
                match value {
                    serde_json::Value::Number(n) => {
                        if let Some(f) = n.as_f64() {
                            Ok(serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0))))
                        } else {
                            Err(ValidationError::new(format!("Cannot convert to float: {}", n)))
                        }
                    }
                    serde_json::Value::String(s) => {
                        match s.parse::<f64>() {
                            Ok(f) => {
                                serde_json::Number::from_f64(f)
                                    .map(|n| serde_json::Value::Number(n))
                                    .ok_or_else(|| ValidationError::new(format!("Cannot convert '{}' to float", s)))
                            }
                            Err(_) => Err(ValidationError::new(format!("Cannot convert '{}' to float", s))),
                        }
                    }
                    _ => Err(ValidationError::new(format!("Cannot convert {} to float", value))),
                }
            }
            "bool" | "boolean" => {
                match value {
                    serde_json::Value::Bool(b) => Ok(serde_json::Value::Bool(b)),
                    serde_json::Value::String(s) => {
                        let lower = s.to_lowercase();
                        Ok(serde_json::Value::Bool(matches!(lower.as_str(), "true" | "1" | "yes" | "on" | "y")))
                    }
                    serde_json::Value::Number(n) => {
                        Ok(serde_json::Value::Bool(n.as_i64().map(|i| i != 0).unwrap_or(false)))
                    }
                    _ => Err(ValidationError::new(format!("Cannot convert {} to bool", value))),
                }
            }
            "Vec" | "Array" | "list" => {
                match value {
                    serde_json::Value::Array(a) => Ok(serde_json::Value::Array(a)),
                    serde_json::Value::String(s) => {
                        // Try to parse as JSON array
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&s) {
                            if parsed.is_array() {
                                return Ok(parsed);
                            }
                        }
                        // Split by comma as fallback
                        let items: Vec<serde_json::Value> = s.split(',')
                            .map(|item| serde_json::Value::String(item.trim().to_string()))
                            .collect();
                        Ok(serde_json::Value::Array(items))
                    }
                    _ => Err(ValidationError::new(format!("Cannot convert {} to array", value))),
                }
            }
            "HashMap" | "dict" | "object" => {
                match value {
                    serde_json::Value::Object(o) => Ok(serde_json::Value::Object(o)),
                    serde_json::Value::String(s) => {
                        serde_json::from_str::<serde_json::Value>(&s)
                            .map_err(|_| ValidationError::new(format!("Cannot convert string '{}' to dict", s)))
                    }
                    _ => Err(ValidationError::new(format!("Cannot convert {} to dict", value))),
                }
            }
            _ => {
                // Default: try direct conversion or return as-is
                Ok(value)
            }
        }
    }

    /// Apply field constraints validation.
    pub fn _apply_constraints(&self, value: &serde_json::Value, field_config: &Field, field_name: &str) -> Result<(), ValidationError> {
        // Numeric constraints
        if let Some(num) = value.as_f64() {
            if let Some(gt) = field_config.gt {
                if num <= gt {
                    return Err(ValidationError::new(format!("Value must be greater than {}", gt)));
                }
            }
            if let Some(ge) = field_config.ge {
                if num < ge {
                    return Err(ValidationError::new(format!("Value must be greater than or equal to {}", ge)));
                }
            }
            if let Some(lt) = field_config.lt {
                if num >= lt {
                    return Err(ValidationError::new(format!("Value must be less than {}", lt)));
                }
            }
            if let Some(le) = field_config.le {
                if num > le {
                    return Err(ValidationError::new(format!("Value must be less than or equal to {}", le)));
                }
            }
            if let Some(multiple_of) = field_config.multiple_of {
                if (num % multiple_of).abs() > f64::EPSILON {
                    return Err(ValidationError::new(format!("Value must be a multiple of {}", multiple_of)));
                }
            }
            // Check for NaN or infinity
            if !field_config.allow_inf_nan && (num.is_nan() || num.is_infinite()) {
                return Err(ValidationError::new("Infinite and NaN values are not allowed"));
            }
        }
        
        // String constraints
        if let Some(s) = value.as_str() {
            let len = s.len() as i64;
            if let Some(min) = field_config.min_length {
                if len < min {
                    return Err(ValidationError::new(format!("String length must be at least {}", min)));
                }
            }
            if let Some(max) = field_config.max_length {
                if len > max {
                    return Err(ValidationError::new(format!("String length must not exceed {}", max)));
                }
            }
            if let Some(pattern) = &field_config.pattern {
                if let Ok(re) = regex::Regex::new(pattern) {
                    if !re.is_match(s) {
                        return Err(ValidationError::new(format!("String does not match pattern: {}", pattern)));
                    }
                }
            }
        }
        
        // Collection constraints
        let collection_len = match value {
            serde_json::Value::Array(a) => Some(a.len() as i64),
            serde_json::Value::Object(o) => Some(o.len() as i64),
            _ => None,
        };
        
        if let Some(len) = collection_len {
            if let Some(min) = field_config.min_length {
                if len < min {
                    return Err(ValidationError::new(format!("Collection length must be at least {}", min)));
                }
            }
            if let Some(max) = field_config.max_length {
                if len > max {
                    return Err(ValidationError::new(format!("Collection length must not exceed {}", max)));
                }
            }
        }
        
        // Enum constraint
        if let Some(enum_vals) = &field_config.enum_values {
            if !enum_vals.contains(value) {
                return Err(ValidationError::new(format!("Value must be one of: {:?}", enum_vals)));
            }
        }
        
        Ok(())
    }

    /// Create and validate model from dictionary data.
    pub fn model_validate(&self, data: HashMap<String, serde_json::Value>) -> Result<Self, ValidationError> {
        let validated = self._validate_and_coerce(data)?;
        Ok(Self {
            _fields: self._fields.clone(),
            _field_types: self._field_types.clone(),
            _data: validated,
        })
    }

    /// Create and validate model from JSON string.
    pub fn model_validate_json(&self, json_data: &str) -> Result<Self, ValidationError> {
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(json_data)
            .map_err(|e| ValidationError::new(format!("Invalid JSON: {}", e)))?;
        self.model_validate(data)
    }

    /// Export model to dictionary.
    pub fn model_dump(&self, include: Option<&HashSet<String>>, exclude: Option<&HashSet<String>>, by_alias: Option<bool>) -> HashMap<String, serde_json::Value> {
        let mut data = HashMap::new();
        let use_alias = by_alias.unwrap_or(false);
        
        for (field_name, value) in &self._data {
            // Check include/exclude
            if let Some(inc) = include {
                if !inc.contains(field_name) {
                    continue;
                }
            }
            if let Some(exc) = exclude {
                if exc.contains(field_name) {
                    continue;
                }
            }
            
            let key = if use_alias {
                self._fields.get(field_name)
                    .and_then(|f| f.alias.as_ref())
                    .unwrap_or(field_name)
            } else {
                field_name
            };
            
            // Convert complex types to serializable forms
            data.insert(key.to_string(), value.clone());
        }
        
        data
    }

    /// Export model to JSON string.
    pub fn model_dump_json(&self, include: Option<&HashSet<String>>, exclude: Option<&HashSet<String>>, by_alias: Option<bool>) -> Result<String, serde_json::Error> {
        let data = self.model_dump(include, exclude, by_alias);
        serde_json::to_string(&data)
    }

    /// Generate JSON Schema for the model.
    pub fn model_json_schema(&self) -> HashMap<String, serde_json::Value> {
        let mut schema = HashMap::new();
        schema.insert("type".to_string(), serde_json::Value::String("object".to_string()));
        schema.insert("title".to_string(), serde_json::Value::String("XModel".to_string()));
        
        let mut properties = HashMap::new();
        let mut required = Vec::new();
        
        for (field_name, field_type) in &self._field_types {
            let field_config = self._fields.get(field_name).cloned().unwrap_or_else(Field::new);
            
            // Determine if field is required
            if !Self::_is_optional(field_type) && field_config.default.is_none() {
                required.push(serde_json::Value::String(field_name.clone()));
            }
            
            // Generate property schema
            let prop_schema = Self::_type_to_json_schema(field_type, &field_config);
            
            // Add field metadata
            let mut final_schema = prop_schema;
            if let Some(title) = &field_config.title {
                final_schema.insert("title".to_string(), serde_json::Value::String(title.clone()));
            }
            if let Some(desc) = &field_config.description {
                final_schema.insert("description".to_string(), serde_json::Value::String(desc.clone()));
            }
            if let Some(examples) = &field_config.examples {
                final_schema.insert("examples".to_string(), serde_json::to_value(examples).unwrap_or(serde_json::Value::Array(vec![])));
            }
            if field_config.deprecated {
                final_schema.insert("deprecated".to_string(), serde_json::Value::Bool(true));
            }
            
            properties.insert(field_name.clone(), serde_json::Value::Object({
                let mut map = serde_json::Map::new();
                for (k, v) in final_schema {
                    map.insert(k, v);
                }
                map
            }));
        }
        
        schema.insert("properties".to_string(), serde_json::Value::Object({
            let mut map = serde_json::Map::new();
            for (k, v) in properties {
                map.insert(k, v);
            }
            map
        }));
        schema.insert("required".to_string(), serde_json::Value::Array(required));
        
        schema
    }

    /// Convert type to JSON Schema property.
    pub fn _type_to_json_schema(field_type: &str, field_config: &Field) -> HashMap<String, serde_json::Value> {
        // Handle Optional types
        let actual_type = if Self::_is_optional(field_type) {
            Self::_get_actual_type(field_type)
        } else {
            field_type.to_string()
        };
        
        let mut schema = match actual_type.as_str() {
            "String" | "str" => {
                let mut s = HashMap::new();
                s.insert("type".to_string(), serde_json::Value::String("string".to_string()));
                if let Some(min) = field_config.min_length {
                    s.insert("minLength".to_string(), serde_json::Value::Number(min.into()));
                }
                if let Some(max) = field_config.max_length {
                    s.insert("maxLength".to_string(), serde_json::Value::Number(max.into()));
                }
                if let Some(pattern) = &field_config.pattern {
                    s.insert("pattern".to_string(), serde_json::Value::String(pattern.clone()));
                }
                if let Some(enum_vals) = &field_config.enum_values {
                    s.insert("enum".to_string(), serde_json::to_value(enum_vals).unwrap_or(serde_json::Value::Array(vec![])));
                }
                s
            }
            "i64" | "int" | "i32" => {
                let mut s = HashMap::new();
                s.insert("type".to_string(), serde_json::Value::String("integer".to_string()));
                if let Some(gt) = field_config.gt {
                    if let Some(n) = serde_json::Number::from_f64(gt) {
                        s.insert("exclusiveMinimum".to_string(), serde_json::Value::Number(n));
                    }
                }
                if let Some(ge) = field_config.ge {
                    if let Some(n) = serde_json::Number::from_f64(ge) {
                        s.insert("minimum".to_string(), serde_json::Value::Number(n));
                    }
                }
                if let Some(lt) = field_config.lt {
                    if let Some(n) = serde_json::Number::from_f64(lt) {
                        s.insert("exclusiveMaximum".to_string(), serde_json::Value::Number(n));
                    }
                }
                if let Some(le) = field_config.le {
                    if let Some(n) = serde_json::Number::from_f64(le) {
                        s.insert("maximum".to_string(), serde_json::Value::Number(n));
                    }
                }
                if let Some(multiple_of) = field_config.multiple_of {
                    if let Some(n) = serde_json::Number::from_f64(multiple_of) {
                        s.insert("multipleOf".to_string(), serde_json::Value::Number(n));
                    }
                }
                s
            }
            "f64" | "float" => {
                let mut s = HashMap::new();
                s.insert("type".to_string(), serde_json::Value::String("number".to_string()));
                if let Some(gt) = field_config.gt {
                    if let Some(n) = serde_json::Number::from_f64(gt) {
                        s.insert("exclusiveMinimum".to_string(), serde_json::Value::Number(n));
                    }
                }
                if let Some(ge) = field_config.ge {
                    if let Some(n) = serde_json::Number::from_f64(ge) {
                        s.insert("minimum".to_string(), serde_json::Value::Number(n));
                    }
                }
                if let Some(lt) = field_config.lt {
                    if let Some(n) = serde_json::Number::from_f64(lt) {
                        s.insert("exclusiveMaximum".to_string(), serde_json::Value::Number(n));
                    }
                }
                if let Some(le) = field_config.le {
                    if let Some(n) = serde_json::Number::from_f64(le) {
                        s.insert("maximum".to_string(), serde_json::Value::Number(n));
                    }
                }
                if let Some(multiple_of) = field_config.multiple_of {
                    if let Some(n) = serde_json::Number::from_f64(multiple_of) {
                        s.insert("multipleOf".to_string(), serde_json::Value::Number(n));
                    }
                }
                s
            }
            "bool" | "boolean" => {
                let mut s = HashMap::new();
                s.insert("type".to_string(), serde_json::Value::String("boolean".to_string()));
                s
            }
            "Vec" | "Array" | "list" => {
                let mut s = HashMap::new();
                s.insert("type".to_string(), serde_json::Value::String("array".to_string()));
                if let Some(min) = field_config.min_length {
                    s.insert("minItems".to_string(), serde_json::Value::Number(min.into()));
                }
                if let Some(max) = field_config.max_length {
                    s.insert("maxItems".to_string(), serde_json::Value::Number(max.into()));
                }
                s
            }
            "HashMap" | "dict" | "object" => {
                let mut s = HashMap::new();
                s.insert("type".to_string(), serde_json::Value::String("object".to_string()));
                s
            }
            _ => {
                // Default to string for unknown types
                let mut s = HashMap::new();
                s.insert("type".to_string(), serde_json::Value::String("string".to_string()));
                s
            }
        };
        
        // Add nullable for Optional types
        if Self::_is_optional(field_type) {
            schema.insert("nullable".to_string(), serde_json::Value::Bool(true));
        }
        
        schema
    }
}
