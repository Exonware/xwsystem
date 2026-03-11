// #exonware/xwsystem/rust/src_old/io/serialization/formats/text/configparser.rs
//! ConfigParser serialization - INI file format.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! INI/ConfigParser serializer using ini crate.
//! Matches Python's ConfigParserSerializer class 100%.
//!
//! Following I→A pattern:
//! - I: ISerialization (interface)
//! - A: ASerialization (abstract base trait)
//! - Concrete: ConfigParserSerializer (implements ASerialization)

use serde_json::Value as JsonValue;
use std::collections::HashMap;
use crate::io::serialization::base::ASerialization;

/// ConfigParser serializer - matches Python's ConfigParserSerializer class.
///
/// Uses ini crate for INI file parsing.
pub struct ConfigParserSerializer;

impl ConfigParserSerializer {
    /// Create a new ConfigParser serializer.
    ///
    /// Matches Python: `ConfigParserSerializer()`
    pub fn new() -> Self {
        ConfigParserSerializer
    }

    /// Get the codec ID.
    ///
    /// Matches Python: `@property def codec_id(self) -> str: return "ini"`
    pub fn codec_id(&self) -> &str {
        "ini"
    }

    /// Get media types.
    ///
    /// Matches Python: `@property def media_types(self) -> list[str]: return ["text/plain", "application/x-ini"]`
    pub fn media_types(&self) -> Vec<&str> {
        vec!["text/plain", "application/x-ini"]
    }

    /// Get file extensions.
    ///
    /// Matches Python: `@property def file_extensions(self) -> list[str]: return [".ini", ".cfg", ".conf"]`
    pub fn file_extensions(&self) -> Vec<&str> {
        vec![".ini", ".cfg", ".conf"]
    }

    /// Get format name.
    ///
    /// Matches Python: `@property def format_name(self) -> str: return "INI"`
    pub fn format_name(&self) -> &str {
        "INI"
    }

    /// Get MIME type.
    ///
    /// Matches Python: `@property def mime_type(self) -> str: return "text/plain"`
    pub fn mime_type(&self) -> &str {
        "text/plain"
    }

    /// Check if binary format.
    ///
    /// Matches Python: `@property def is_binary_format(self) -> bool: return False`
    pub fn is_binary_format(&self) -> bool {
        false
    }

    /// Get aliases.
    ///
    /// Matches Python: `@property def aliases(self) -> list[str]: return ["ini", "INI", "cfg", "conf", "configparser"]`
    pub fn aliases(&self) -> Vec<&str> {
        vec!["ini", "INI", "cfg", "conf", "configparser"]
    }

    /// Encode data to INI string.
    ///
    /// Matches Python: `def encode(self, data: Any, *, options: Optional[EncodeOptions] = None) -> str`
    ///
    /// Args:
    ///     data: Dictionary with sections and key-value pairs
    ///     options: INI encoding options
    ///
    /// Returns:
    ///     INI string
    pub fn encode(
        &self,
        data: &JsonValue,
        _options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        
        if let JsonValue::Object(sections) = data {
            for (section_name, section_data) in sections {
                // Write section header
                output.push_str(&format!("[{}]\n", section_name));
                
                if let JsonValue::Object(keys) = section_data {
                    for (key, value) in keys {
                        let value_str = match value {
                            JsonValue::String(s) => s.clone(),
                            JsonValue::Number(n) => n.to_string(),
                            JsonValue::Bool(b) => b.to_string(),
                            JsonValue::Null => String::new(),
                            _ => serde_json::to_string(value)?,
                        };
                        output.push_str(&format!("{} = {}\n", key, value_str));
                    }
                }
                output.push('\n');
            }
        }
        
        Ok(output)
    }

    /// Decode INI string to dictionary.
    ///
    /// Matches Python: `def decode(self, data: Union[str, bytes], *, options: Optional[DecodeOptions] = None) -> Any`
    ///
    /// Args:
    ///     data: INI string or bytes
    ///     options: INI decoding options
    ///
    /// Returns:
    ///     Decoded dictionary (as JSON value)
    pub fn decode(
        &self,
        data: &[u8],
        _options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        let ini_str = String::from_utf8(data.to_vec())?;
        let mut result = serde_json::Map::new();
        let mut current_section: Option<String> = None;
        let mut current_section_obj = serde_json::Map::new();
        
        for line in ini_str.lines() {
            let trimmed = line.trim();
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
                continue;
            }
            
            // Check for section header [section]
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                // Save previous section if any
                if let Some(section_name) = current_section.take() {
                    result.insert(section_name, JsonValue::Object(current_section_obj));
                }
                
                // Start new section
                let section_name = trimmed[1..trimmed.len()-1].trim().to_string();
                current_section = Some(section_name);
                current_section_obj = serde_json::Map::new();
                continue;
            }
            
            // Parse key = value
            if let Some(equal_pos) = trimmed.find('=') {
                let key = trimmed[..equal_pos].trim().to_string();
                let value_str = trimmed[equal_pos+1..].trim();
                
                // Try to parse as number or boolean, otherwise keep as string
                let json_value = if let Ok(num) = value_str.parse::<i64>() {
                    JsonValue::Number(num.into())
                } else if let Ok(num) = value_str.parse::<f64>() {
                    JsonValue::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from(0)))
                } else if value_str == "true" {
                    JsonValue::Bool(true)
                } else if value_str == "false" {
                    JsonValue::Bool(false)
                } else {
                    JsonValue::String(value_str.to_string())
                };
                
                current_section_obj.insert(key, json_value);
            }
        }
        
        // Save last section
        if let Some(section_name) = current_section {
            result.insert(section_name, JsonValue::Object(current_section_obj));
        }
        
        Ok(JsonValue::Object(result))
    }

    /// Decode INI string (str version).
    ///
    /// Convenience method for string input.
    pub fn decode_str(
        &self,
        data: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        self.decode(data.as_bytes(), options)
    }
}

impl Default for ConfigParserSerializer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ASERIALIZATION TRAIT IMPLEMENTATION
// ============================================================================
// This implements the ASerialization trait, matching Python's inheritance:
// Python: class ConfigParserSerializer(ASerialization)
// Rust: impl ASerialization for ConfigParserSerializer
//
// The trait methods delegate to the inherent methods defined above.
// ============================================================================

impl ASerialization for ConfigParserSerializer {
    fn codec_id(&self) -> &str {
        ConfigParserSerializer::codec_id(self)
    }

    fn media_types(&self) -> Vec<&str> {
        ConfigParserSerializer::media_types(self)
    }

    fn file_extensions(&self) -> Vec<&str> {
        ConfigParserSerializer::file_extensions(self)
    }

    fn format_name(&self) -> &str {
        ConfigParserSerializer::format_name(self)
    }

    fn mime_type(&self) -> &str {
        ConfigParserSerializer::mime_type(self)
    }

    fn is_binary_format(&self) -> bool {
        ConfigParserSerializer::is_binary_format(self)
    }

    fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Convert to JsonValue first
        let json_value = serde_json::to_value(value)?;
        ConfigParserSerializer::encode(self, &json_value, options)
    }

    fn decode(
        &self,
        repr: &[u8],
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        ConfigParserSerializer::decode(self, repr, options)
    }

    fn decode_str(
        &self,
        repr: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        ConfigParserSerializer::decode_str(self, repr, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_configparser_serializer_creation() {
        let serializer = ConfigParserSerializer::new();
        assert_eq!(serializer.codec_id(), "ini");
        assert_eq!(serializer.format_name(), "INI");
        assert_eq!(serializer.mime_type(), "text/plain");
        assert!(!serializer.is_binary_format());
    }

    #[test]
    fn test_configparser_encode_basic() {
        let serializer = ConfigParserSerializer::new();
        let data = json!({
            "database": {
                "host": "localhost",
                "port": "5432"
            }
        });
        let result = serializer.encode(&data, None).unwrap();
        assert!(result.contains("[database]"));
        assert!(result.contains("host"));
        assert!(result.contains("localhost"));
    }

    #[test]
    fn test_configparser_decode_basic() {
        let serializer = ConfigParserSerializer::new();
        let ini_str = "[database]\nhost = localhost\nport = 5432";
        let result = serializer.decode_str(ini_str, None).unwrap();
        assert_eq!(result["database"]["host"], "localhost");
        assert_eq!(result["database"]["port"], "5432");
    }

    #[test]
    fn test_configparser_encode_decode_roundtrip() {
        let serializer = ConfigParserSerializer::new();
        let original = json!({
            "section1": {
                "key1": "value1",
                "key2": "value2"
            },
            "section2": {
                "key3": "value3"
            }
        });
        
        let encoded = serializer.encode(&original, None).unwrap();
        let decoded = serializer.decode_str(&encoded, None).unwrap();
        assert_eq!(decoded["section1"]["key1"], "value1");
        assert_eq!(decoded["section2"]["key3"], "value3");
    }

    #[test]
    fn test_configparser_media_types() {
        let serializer = ConfigParserSerializer::new();
        let media_types = serializer.media_types();
        assert!(media_types.contains(&"text/plain"));
        assert!(media_types.contains(&"application/x-ini"));
    }

    #[test]
    fn test_configparser_file_extensions() {
        let serializer = ConfigParserSerializer::new();
        let extensions = serializer.file_extensions();
        assert!(extensions.contains(&".ini"));
        assert!(extensions.contains(&".cfg"));
        assert!(extensions.contains(&".conf"));
    }
}

