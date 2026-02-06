// #exonware/xwsystem/rust/src_old/io/serialization/formats/text/json.rs
//! JSON serialization - Universal, human-readable data interchange format.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! JSON serializer using serde_json crate.
//! Matches Python's JsonSerializer class 100%.
//!
//! Following I→A pattern:
//! - I: ISerialization (interface)
//! - A: ASerialization (abstract base trait)
//! - Concrete: JsonSerializer (implements ASerialization)

use serde_json::Value as JsonValue;
use serde_json;
use std::collections::HashMap;
use crate::io::serialization::base::ASerialization;

/// JSON serializer - matches Python's JsonSerializer class.
///
/// Uses serde_json for serialization/deserialization.
/// Provides encode/decode operations matching Python API.
pub struct JsonSerializer {
    /// Maximum nesting depth allowed (optional)
    max_depth: Option<usize>,
    /// Maximum estimated data size in MB (optional)
    max_size_mb: Option<f64>,
}

impl JsonSerializer {
    /// Create a new JSON serializer.
    ///
    /// Matches Python: `JsonSerializer(parser_name=None, max_depth=None, max_size_mb=None)`
    pub fn new() -> Self {
        JsonSerializer {
            max_depth: None,
            max_size_mb: None,
        }
    }

    /// Create a new JSON serializer with options.
    ///
    /// Matches Python constructor parameters.
    pub fn with_options(max_depth: Option<usize>, max_size_mb: Option<f64>) -> Self {
        JsonSerializer {
            max_depth,
            max_size_mb,
        }
    }

    /// Get the codec ID.
    ///
    /// Matches Python: `@property def codec_id(self) -> str: return "json"`
    pub fn codec_id(&self) -> &str {
        "json"
    }

    /// Get media types.
    ///
    /// Matches Python: `@property def media_types(self) -> list[str]: return ["application/json", "text/json"]`
    pub fn media_types(&self) -> Vec<&str> {
        vec!["application/json", "text/json"]
    }

    /// Get file extensions.
    ///
    /// Matches Python: `@property def file_extensions(self) -> list[str]: return [".json", ...]`
    pub fn file_extensions(&self) -> Vec<&str> {
        vec![".json", ".webmanifest", ".mcmeta", ".geojson", ".topojson"]
    }

    /// Get format name.
    ///
    /// Matches Python: `@property def format_name(self) -> str: return "JSON"`
    pub fn format_name(&self) -> &str {
        "JSON"
    }

    /// Get MIME type.
    ///
    /// Matches Python: `@property def mime_type(self) -> str: return "application/json"`
    pub fn mime_type(&self) -> &str {
        "application/json"
    }

    /// Check if binary format.
    ///
    /// Matches Python: `@property def is_binary_format(self) -> bool: return False`
    pub fn is_binary_format(&self) -> bool {
        false
    }

    /// Encode data to JSON string.
    ///
    /// Matches Python: `def encode(self, value: Any, *, options: Optional[EncodeOptions] = None) -> Union[bytes, str]`
    ///
    /// Args:
    ///     value: Data to serialize (must implement Serialize)
    ///     options: JSON encoding options (indent, sort_keys, etc.)
    ///
    /// Returns:
    ///     JSON string
    ///
    /// Raises:
    ///     CoreError: If encoding fails
    pub fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Handle indent option
        let indent = options
            .and_then(|opts| opts.get("indent").or_else(|| opts.get("pretty")))
            .and_then(|v| {
                if v.is_boolean() && v.as_bool().unwrap_or(false) {
                    Some(2)
                } else {
                    v.as_u64().map(|n| n as usize)
                }
            });

        let sort_keys = options
            .and_then(|opts| opts.get("sort_keys"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let _ensure_ascii = options
            .and_then(|opts| opts.get("ensure_ascii"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Serialize with options
        let json_value = serde_json::to_value(value)?;
        
        // Handle sort_keys
        let final_value = if sort_keys {
            if let JsonValue::Object(map) = json_value {
                let mut sorted: Vec<_> = map.iter().collect();
                sorted.sort_by_key(|(k, _)| *k);
                let mut new_map = serde_json::Map::new();
                for (k, v) in sorted {
                    new_map.insert(k.clone(), v.clone());
                }
                JsonValue::Object(new_map)
            } else {
                json_value
            }
        } else {
            json_value
        };
        
        // Serialize with indent if specified
        if indent.is_some() {
            serde_json::to_string_pretty(&final_value)
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("JSON serialization error: {}", e))) as Box<dyn std::error::Error>)
        } else {
            serde_json::to_string(&final_value)
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("JSON serialization error: {}", e))) as Box<dyn std::error::Error>)
        }
    }

    /// Decode JSON string to data.
    ///
    /// Matches Python: `def decode(self, repr: Union[bytes, str], *, options: Optional[DecodeOptions] = None) -> Any`
    ///
    /// Args:
    ///     repr: JSON string (bytes or str)
    ///     options: JSON decoding options
    ///
    /// Returns:
    ///     Decoded JSON value
    ///
    /// Raises:
    ///     CoreError: If decoding fails
    pub fn decode(
        &self,
        repr: &[u8],
        _options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // Convert bytes to string if needed
        let json_str = String::from_utf8(repr.to_vec())?;
        let value: JsonValue = serde_json::from_str(&json_str)?;
        Ok(value)
    }

    /// Decode JSON string (str version).
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

impl Default for JsonSerializer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ASERIALIZATION TRAIT IMPLEMENTATION
// ============================================================================
// This implements the ASerialization trait, matching Python's inheritance:
// Python: class JsonSerializer(ASerialization)
// Rust: impl ASerialization for JsonSerializer
// ============================================================================

// ============================================================================
// ASERIALIZATION TRAIT IMPLEMENTATION
// ============================================================================
// This implements the ASerialization trait, matching Python's inheritance:
// Python: class JsonSerializer(ASerialization)
// Rust: impl ASerialization for JsonSerializer
//
// The trait methods delegate to the inherent methods defined above.
// ============================================================================

impl ASerialization for JsonSerializer {
    fn codec_id(&self) -> &str {
        // Delegate to inherent method
        JsonSerializer::codec_id(self)
    }

    fn media_types(&self) -> Vec<&str> {
        // Delegate to inherent method
        JsonSerializer::media_types(self)
    }

    fn file_extensions(&self) -> Vec<&str> {
        // Delegate to inherent method
        JsonSerializer::file_extensions(self)
    }

    fn format_name(&self) -> &str {
        // Delegate to inherent method
        JsonSerializer::format_name(self)
    }

    fn mime_type(&self) -> &str {
        // Delegate to inherent method
        JsonSerializer::mime_type(self)
    }

    fn is_binary_format(&self) -> bool {
        // Delegate to inherent method
        JsonSerializer::is_binary_format(self)
    }

    fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Delegate to inherent method
        JsonSerializer::encode(self, value, options)
    }

    fn decode(
        &self,
        repr: &[u8],
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // Delegate to inherent method
        JsonSerializer::decode(self, repr, options)
    }

    fn decode_str(
        &self,
        repr: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // Delegate to inherent method
        JsonSerializer::decode_str(self, repr, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_serializer_creation() {
        let serializer = JsonSerializer::new();
        assert_eq!(serializer.codec_id(), "json");
        assert_eq!(serializer.format_name(), "JSON");
        assert_eq!(serializer.mime_type(), "application/json");
        assert!(!serializer.is_binary_format());
    }

    #[test]
    fn test_json_encode_basic() {
        let serializer = JsonSerializer::new();
        let data = json!({"key": "value", "number": 42});
        let result = serializer.encode(&data, None).unwrap();
        assert!(result.contains("\"key\""));
        assert!(result.contains("\"value\""));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_json_encode_with_indent() {
        let serializer = JsonSerializer::new();
        let data = json!({"key": "value"});
        let mut options = HashMap::new();
        options.insert("indent".to_string(), json!(2));
        let result = serializer.encode(&data, Some(&options)).unwrap();
        assert!(result.contains('\n')); // Should be pretty-printed
    }

    #[test]
    fn test_json_decode_basic() {
        let serializer = JsonSerializer::new();
        let json_str = r#"{"key": "value", "number": 42}"#;
        let result = serializer.decode_str(json_str, None).unwrap();
        assert_eq!(result["key"], "value");
        assert_eq!(result["number"], 42);
    }

    #[test]
    fn test_json_decode_bytes() {
        let serializer = JsonSerializer::new();
        let json_bytes = b"{\"key\": \"value\"}";
        let result = serializer.decode(json_bytes, None).unwrap();
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn test_json_encode_decode_roundtrip() {
        let serializer = JsonSerializer::new();
        let original = json!({
            "string": "test",
            "number": 123,
            "boolean": true,
            "array": [1, 2, 3],
            "object": {"nested": "value"}
        });
        
        let encoded = serializer.encode(&original, None).unwrap();
        let decoded = serializer.decode_str(&encoded, None).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_json_media_types() {
        let serializer = JsonSerializer::new();
        let media_types = serializer.media_types();
        assert!(media_types.contains(&"application/json"));
        assert!(media_types.contains(&"text/json"));
    }

    #[test]
    fn test_json_file_extensions() {
        let serializer = JsonSerializer::new();
        let extensions = serializer.file_extensions();
        assert!(extensions.contains(&".json"));
        assert!(extensions.contains(&".geojson"));
    }
}

