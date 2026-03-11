// #exonware/xwsystem/rust/src_old/io/serialization/formats/text/yaml.rs
//! YAML serialization - Human-readable data serialization format.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! YAML serializer using serde_yaml crate.
//! Matches Python's YamlSerializer class 100%.
//!
//! Following I→A pattern:
//! - I: ISerialization (interface)
//! - A: ASerialization (abstract base trait)
//! - Concrete: YamlSerializer (implements ASerialization)

use serde_json::Value as JsonValue;
use serde_yaml;
use std::collections::HashMap;
use crate::io::serialization::base::ASerialization;

/// YAML serializer - matches Python's YamlSerializer class.
///
/// Uses serde_yaml for serialization/deserialization.
pub struct YamlSerializer;

impl YamlSerializer {
    /// Create a new YAML serializer.
    ///
    /// Matches Python: `YamlSerializer()`
    pub fn new() -> Self {
        YamlSerializer
    }

    /// Get the codec ID.
    ///
    /// Matches Python: `@property def codec_id(self) -> str: return "yaml"`
    pub fn codec_id(&self) -> &str {
        "yaml"
    }

    /// Get media types.
    ///
    /// Matches Python: `@property def media_types(self) -> list[str]: return ["application/x-yaml", "text/yaml", "text/x-yaml"]`
    pub fn media_types(&self) -> Vec<&str> {
        vec!["application/x-yaml", "text/yaml", "text/x-yaml"]
    }

    /// Get file extensions.
    ///
    /// Matches Python: `@property def file_extensions(self) -> list[str]: return [".yaml", ".yml", ...]`
    pub fn file_extensions(&self) -> Vec<&str> {
        vec![".yaml", ".yml", ".clang-format", ".travis.yml", ".gitlab-ci.yml"]
    }

    /// Get format name.
    ///
    /// Matches Python: `@property def format_name(self) -> str: return "YAML"`
    pub fn format_name(&self) -> &str {
        "YAML"
    }

    /// Get MIME type.
    ///
    /// Matches Python: `@property def mime_type(self) -> str: return "application/x-yaml"`
    pub fn mime_type(&self) -> &str {
        "application/x-yaml"
    }

    /// Check if binary format.
    ///
    /// Matches Python: `@property def is_binary_format(self) -> bool: return False`
    pub fn is_binary_format(&self) -> bool {
        false
    }

    /// Get aliases.
    ///
    /// Matches Python: `@property def aliases(self) -> list[str]: return ["yaml", "YAML", "yml", "YML"]`
    pub fn aliases(&self) -> Vec<&str> {
        vec!["yaml", "YAML", "yml", "YML"]
    }

    /// Encode data to YAML string.
    ///
    /// Matches Python: `def encode(self, value: Any, *, options: Optional[EncodeOptions] = None) -> Union[bytes, str]`
    ///
    /// Args:
    ///     value: Data to serialize (must implement Serialize)
    ///     options: YAML encoding options
    ///
    /// Returns:
    ///     YAML string
    pub fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        _options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        serde_yaml::to_string(value)
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("YAML serialization error: {}", e))) as Box<dyn std::error::Error>)
    }

    /// Decode YAML string to data.
    ///
    /// Matches Python: `def decode(self, repr: Union[bytes, str], *, options: Optional[DecodeOptions] = None) -> Any`
    ///
    /// Args:
    ///     repr: YAML string (bytes or str)
    ///     options: YAML decoding options
    ///
    /// Returns:
    ///     Decoded JSON value
    pub fn decode(
        &self,
        repr: &[u8],
        _options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        let yaml_str = String::from_utf8(repr.to_vec())?;
        let value: JsonValue = serde_yaml::from_str(&yaml_str)?;
        Ok(value)
    }

    /// Decode YAML string (str version).
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

impl Default for YamlSerializer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ASERIALIZATION TRAIT IMPLEMENTATION
// ============================================================================
// This implements the ASerialization trait, matching Python's inheritance:
// Python: class YamlSerializer(ASerialization)
// Rust: impl ASerialization for YamlSerializer
//
// The trait methods delegate to the inherent methods defined above.
// ============================================================================

impl ASerialization for YamlSerializer {
    fn codec_id(&self) -> &str {
        YamlSerializer::codec_id(self)
    }

    fn media_types(&self) -> Vec<&str> {
        YamlSerializer::media_types(self)
    }

    fn file_extensions(&self) -> Vec<&str> {
        YamlSerializer::file_extensions(self)
    }

    fn format_name(&self) -> &str {
        YamlSerializer::format_name(self)
    }

    fn mime_type(&self) -> &str {
        YamlSerializer::mime_type(self)
    }

    fn is_binary_format(&self) -> bool {
        YamlSerializer::is_binary_format(self)
    }

    fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        YamlSerializer::encode(self, value, options)
    }

    fn decode(
        &self,
        repr: &[u8],
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        YamlSerializer::decode(self, repr, options)
    }

    fn decode_str(
        &self,
        repr: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        YamlSerializer::decode_str(self, repr, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_yaml_serializer_creation() {
        let serializer = YamlSerializer::new();
        assert_eq!(serializer.codec_id(), "yaml");
        assert_eq!(serializer.format_name(), "YAML");
        assert_eq!(serializer.mime_type(), "application/x-yaml");
        assert!(!serializer.is_binary_format());
    }

    #[test]
    fn test_yaml_encode_basic() {
        let serializer = YamlSerializer::new();
        let data = json!({"key": "value", "number": 42});
        let result = serializer.encode(&data, None).unwrap();
        assert!(result.contains("key:"));
        assert!(result.contains("value"));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_yaml_decode_basic() {
        let serializer = YamlSerializer::new();
        let yaml_str = "key: value\nnumber: 42";
        let result = serializer.decode_str(yaml_str, None).unwrap();
        assert_eq!(result["key"], "value");
        assert_eq!(result["number"], 42);
    }

    #[test]
    fn test_yaml_encode_decode_roundtrip() {
        let serializer = YamlSerializer::new();
        let original = json!({
            "string": "test",
            "number": 123,
            "boolean": true,
            "array": [1, 2, 3],
            "object": {"nested": "value"}
        });
        
        let encoded = serializer.encode(&original, None).unwrap();
        let decoded = serializer.decode_str(&encoded, None).unwrap();
        assert_eq!(decoded["string"], "test");
        assert_eq!(decoded["number"], 123);
        assert_eq!(decoded["boolean"], true);
    }

    #[test]
    fn test_yaml_media_types() {
        let serializer = YamlSerializer::new();
        let media_types = serializer.media_types();
        assert!(media_types.contains(&"application/x-yaml"));
        assert!(media_types.contains(&"text/yaml"));
    }

    #[test]
    fn test_yaml_file_extensions() {
        let serializer = YamlSerializer::new();
        let extensions = serializer.file_extensions();
        assert!(extensions.contains(&".yaml"));
        assert!(extensions.contains(&".yml"));
    }
}

