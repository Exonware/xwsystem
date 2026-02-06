// #exonware/xwsystem/rust/src_old/io/serialization/formats/text/toml.rs
//! TOML serialization - Configuration file format.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! TOML serializer using toml crate.
//! Matches Python's TomlSerializer class 100%.
//!
//! Following I→A pattern:
//! - I: ISerialization (interface)
//! - A: ASerialization (abstract base trait)
//! - Concrete: TomlSerializer (implements ASerialization)

use serde_json::Value as JsonValue;
use toml;
use std::collections::HashMap;
use crate::io::serialization::base::ASerialization;
use crate::defs::CodecCapability;

/// TOML serializer - matches Python's TomlSerializer class.
///
/// Uses toml crate for serialization/deserialization.
pub struct TomlSerializer;

impl TomlSerializer {
    /// Create a new TOML serializer.
    ///
    /// Matches Python: `TomlSerializer()`
    pub fn new() -> Self {
        TomlSerializer
    }

    /// Get the codec ID.
    ///
    /// Matches Python: `@property def codec_id(self) -> str: return "toml"`
    pub fn codec_id(&self) -> &str {
        "toml"
    }

    /// Get media types.
    ///
    /// Matches Python: `@property def media_types(self) -> list[str]: return ["application/toml", "text/toml"]`
    pub fn media_types(&self) -> Vec<&str> {
        vec!["application/toml", "text/toml"]
    }

    /// Get file extensions.
    ///
    /// Matches Python: `@property def file_extensions(self) -> list[str]: return [".toml", ".tml"]`
    pub fn file_extensions(&self) -> Vec<&str> {
        vec![".toml", ".tml"]
    }

    /// Get format name.
    ///
    /// Matches Python: `@property def format_name(self) -> str: return "TOML"`
    pub fn format_name(&self) -> &str {
        "TOML"
    }

    /// Get MIME type.
    ///
    /// Matches Python: `@property def mime_type(self) -> str: return "application/toml"`
    pub fn mime_type(&self) -> &str {
        "application/toml"
    }

    /// Check if binary format.
    ///
    /// Matches Python: `@property def is_binary_format(self) -> bool: return False`
    pub fn is_binary_format(&self) -> bool {
        false
    }

    /// Get aliases.
    ///
    /// Matches Python: `@property def aliases(self) -> list[str]: return ["toml", "TOML"]`
    pub fn aliases(&self) -> Vec<&str> {
        vec!["toml", "TOML"]
    }

    /// Check if streaming is supported.
    ///
    /// Matches Python: `@property def supports_streaming(self) -> bool: return False`
    pub fn supports_streaming(&self) -> bool {
        false  // TOML doesn't support streaming
    }

    /// Get codec capabilities.
    ///
    /// Matches Python: `@property def capabilities(self) -> CodecCapability: return CodecCapability.BIDIRECTIONAL`
    pub fn capabilities(&self) -> CodecCapability {
        CodecCapability::BIDIRECTIONAL
    }

    /// Get codec types.
    ///
    /// Matches Python: `@property def codec_types(self) -> list[str]: return ["config", "serialization"]`
    pub fn codec_types(&self) -> Vec<&str> {
        vec!["config", "serialization"]
    }

    /// Recursively remove None/null values from data structure.
    ///
    /// Matches Python: `def _remove_none_values(self, data: Any) -> Any`
    /// Root cause fixed: TOML doesn't support None/null values natively.
    /// Solution: Recursively remove None values before encoding (omit them).
    fn remove_none_values(&self, data: &JsonValue) -> JsonValue {
        match data {
            JsonValue::Object(map) => {
                let mut result = serde_json::Map::new();
                for (key, value) in map {
                    if !value.is_null() {
                        let cleaned_value = self.remove_none_values(value);
                        if !cleaned_value.is_null() {
                            result.insert(key.clone(), cleaned_value);
                        }
                    }
                }
                JsonValue::Object(result)
            }
            JsonValue::Array(arr) => {
                let result: Vec<JsonValue> = arr.iter()
                    .filter_map(|item| {
                        let cleaned = self.remove_none_values(item);
                        if !cleaned.is_null() {
                            Some(cleaned)
                        } else {
                            None
                        }
                    })
                    .collect();
                JsonValue::Array(result)
            }
            _ => data.clone(),
        }
    }

    /// Encode data to TOML string.
    ///
    /// Matches Python: `def encode(self, value: Any, *, options: Optional[EncodeOptions] = None) -> Union[bytes, str]`
    ///
    /// Root cause fixed: TOML doesn't support None/null values.
    /// Solution: Remove None values before encoding using remove_none_values().
    ///
    /// Args:
    ///     value: Data to serialize (must be compatible with TOML)
    ///     options: TOML encoding options
    ///
    /// Returns:
    ///     TOML string
    pub fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Convert to JSON value first for None handling
        let json_value = serde_json::to_value(value)?;
        
        // TOML requires a table (dict) at the top level
        let toml_value = match json_value {
            JsonValue::Object(_) => json_value,
            JsonValue::Array(arr) => {
                // Auto-wrap top-level list into "items" table (matching Python behavior)
                let mut map = serde_json::Map::new();
                map.insert("items".to_string(), JsonValue::Array(arr));
                JsonValue::Object(map)
            }
            other => {
                // Fallback: wrap primitive/other types into a single "value" key
                let mut map = serde_json::Map::new();
                map.insert("value".to_string(), other);
                JsonValue::Object(map)
            }
        };
        
        // Root cause fixed: Remove None values before encoding (TOML doesn't support None)
        let cleaned_value = self.remove_none_values(&toml_value);
        
        // Convert to TOML string
        // Note: We need to convert JsonValue to toml::Value
        let toml_value: toml::Value = toml::from_str(&serde_json::to_string(&cleaned_value)?)?;
        
        // Apply multiline_strings option if provided
        let multiline_strings = options
            .and_then(|opts| opts.get("multiline_strings"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        // Serialize to TOML string
        // Note: toml crate doesn't support multiline_strings option directly,
        // but we can use to_string_pretty for better formatting
        if multiline_strings {
            toml::to_string_pretty(&toml_value)
        } else {
            toml::to_string(&toml_value)
        }
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("TOML serialization error: {}", e))) as Box<dyn std::error::Error>)
    }

    /// Decode TOML string to data.
    ///
    /// Matches Python: `def decode(self, repr: Union[bytes, str], *, options: Optional[DecodeOptions] = None) -> Any`
    ///
    /// Args:
    ///     repr: TOML string (bytes or str)
    ///     options: TOML decoding options
    ///
    /// Returns:
    ///     Decoded JSON value (TOML is converted to JSON-compatible structure)
    ///     If this looks like an auto-wrapped list payload (see encode), unwraps it
    pub fn decode(
        &self,
        repr: &[u8],
        _options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        let toml_str = String::from_utf8(repr.to_vec())?;
        let value: toml::Value = toml::from_str(&toml_str)?;
        
        // Convert TOML Value to JSON Value
        let json_value = match value {
            toml::Value::String(s) => JsonValue::String(s),
            toml::Value::Integer(i) => JsonValue::Number(i.into()),
            toml::Value::Float(f) => {
                JsonValue::Number(serde_json::Number::from_f64(f).unwrap_or(serde_json::Number::from(0)))
            }
            toml::Value::Boolean(b) => JsonValue::Bool(b),
            toml::Value::Datetime(dt) => JsonValue::String(dt.to_string()),
            toml::Value::Array(arr) => {
                let json_arr: Vec<JsonValue> = arr.iter()
                    .map(|v| self.toml_value_to_json(v))
                    .collect();
                JsonValue::Array(json_arr)
            }
            toml::Value::Table(table) => {
                let mut json_obj = serde_json::Map::new();
                for (k, v) in table {
                    json_obj.insert(k, self.toml_value_to_json(&v));
                }
                JsonValue::Object(json_obj)
            }
        };
        
        // If this looks like an auto-wrapped list payload (see encode), unwrap it
        // Matching Python behavior: if dict has only "items" key with a list value, return the list
        if let JsonValue::Object(map) = &json_value {
            if map.len() == 1 {
                if let Some(items) = map.get("items") {
                    if items.is_array() {
                        return Ok(items.clone());
                    }
                }
            }
        }
        
        Ok(json_value)
    }

    /// Helper to convert TOML Value to JSON Value.
    fn toml_value_to_json(&self, value: &toml::Value) -> JsonValue {
        match value {
            toml::Value::String(s) => JsonValue::String(s.clone()),
            toml::Value::Integer(i) => JsonValue::Number((*i).into()),
            toml::Value::Float(f) => {
                JsonValue::Number(serde_json::Number::from_f64(*f).unwrap_or(serde_json::Number::from(0)))
            }
            toml::Value::Boolean(b) => JsonValue::Bool(*b),
            toml::Value::Datetime(dt) => JsonValue::String(dt.to_string()),
            toml::Value::Array(arr) => {
                let json_arr: Vec<JsonValue> = arr.iter()
                    .map(|v| self.toml_value_to_json(v))
                    .collect();
                JsonValue::Array(json_arr)
            }
            toml::Value::Table(table) => {
                let mut json_obj = serde_json::Map::new();
                for (k, v) in table {
                    json_obj.insert(k.clone(), self.toml_value_to_json(v));
                }
                JsonValue::Object(json_obj)
            }
        }
    }

    /// Decode TOML string (str version).
    ///
    /// Convenience method for string input.
    pub fn decode_str(
        &self,
        repr: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        self.decode(repr.as_bytes(), options)
    }
}

impl Default for TomlSerializer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ASERIALIZATION TRAIT IMPLEMENTATION
// ============================================================================
// This implements the ASerialization trait, matching Python's inheritance:
// Python: class TomlSerializer(ASerialization)
// Rust: impl ASerialization for TomlSerializer
//
// The trait methods delegate to the inherent methods defined above.
// ============================================================================

impl ASerialization for TomlSerializer {
    fn codec_id(&self) -> &str {
        TomlSerializer::codec_id(self)
    }

    fn media_types(&self) -> Vec<&str> {
        TomlSerializer::media_types(self)
    }

    fn file_extensions(&self) -> Vec<&str> {
        TomlSerializer::file_extensions(self)
    }

    fn format_name(&self) -> &str {
        TomlSerializer::format_name(self)
    }

    fn mime_type(&self) -> &str {
        TomlSerializer::mime_type(self)
    }

    fn is_binary_format(&self) -> bool {
        TomlSerializer::is_binary_format(self)
    }

    fn supports_streaming(&self) -> bool {
        TomlSerializer::supports_streaming(self)
    }

    fn capabilities(&self) -> crate::defs::CodecCapability {
        TomlSerializer::capabilities(self)
    }

    fn codec_types(&self) -> Vec<&str> {
        TomlSerializer::codec_types(self)
    }

    fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        TomlSerializer::encode(self, value, options)
    }

    fn decode(
        &self,
        repr: &[u8],
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        TomlSerializer::decode(self, repr, options)
    }

    fn decode_str(
        &self,
        repr: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        TomlSerializer::decode_str(self, repr, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_serializer_creation() {
        let serializer = TomlSerializer::new();
        assert_eq!(serializer.codec_id(), "toml");
        assert_eq!(serializer.format_name(), "TOML");
        assert_eq!(serializer.mime_type(), "application/toml");
        assert!(!serializer.is_binary_format());
    }

    #[test]
    fn test_toml_encode_basic() {
        let serializer = TomlSerializer::new();
        // TOML requires a table structure, so we use a map with serde_json::Value
        use std::collections::HashMap;
        use serde_json::Value;
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert("key".to_string(), Value::String("value".to_string()));
        data.insert("number".to_string(), Value::Number(42.into()));
        let result = serializer.encode(&Value::Object(serde_json::Map::from_iter(data)), None).unwrap();
        assert!(result.contains("key =")); 
        assert!(result.contains("value"));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_toml_decode_basic() {
        let serializer = TomlSerializer::new();
        let toml_str = r#"key = "value"
number = 42"#;
        let result = serializer.decode_str(toml_str, None).unwrap();
        assert_eq!(result["key"], "value");
        assert_eq!(result["number"], 42);
    }

    #[test]
    fn test_toml_decode_nested() {
        let serializer = TomlSerializer::new();
        let toml_str = r#"[database]
host = "localhost"
port = 5432"#;
        let result = serializer.decode_str(toml_str, None).unwrap();
        assert_eq!(result["database"]["host"], "localhost");
        assert_eq!(result["database"]["port"], 5432);
    }

    #[test]
    fn test_toml_media_types() {
        let serializer = TomlSerializer::new();
        let media_types = serializer.media_types();
        assert!(media_types.contains(&"application/toml"));
        assert!(media_types.contains(&"text/toml"));
    }

    #[test]
    fn test_toml_file_extensions() {
        let serializer = TomlSerializer::new();
        let extensions = serializer.file_extensions();
        assert!(extensions.contains(&".toml"));
        assert!(extensions.contains(&".tml"));
    }
}

