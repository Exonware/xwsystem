// #exonware/xwsystem/rust/src_old/io/serialization/formats/text/jsonlines.rs
//! JSON Lines (JSONL/NDJSON) Serialization - Newline-Delimited JSON
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! JSON Lines format (also called NDJSON - Newline Delimited JSON):
//! - One JSON object per line
//! - Perfect for streaming data
//! - Log file friendly
//! - Easy to append
//!
//! Matches Python's JsonLinesSerializer class 100%.
//!
//! Following I→A pattern:
//! - I: ISerialization (interface)
//! - A: ASerialization (abstract base trait)
//! - Concrete: JsonLinesSerializer (implements ASerialization)

use super::json::JsonSerializer;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use crate::io::serialization::base::ASerialization;

/// JSON Lines (JSONL/NDJSON) serializer for streaming data.
///
/// Matches Python's JsonLinesSerializer class.
/// Extends JsonSerializer with line-delimited JSON support.
pub struct JsonLinesSerializer {
    json_serializer: JsonSerializer,
}

impl JsonLinesSerializer {
    /// Create a new JSON Lines serializer.
    ///
    /// Matches Python: `JsonLinesSerializer(parser_name=None)`
    pub fn new() -> Self {
        JsonLinesSerializer {
            json_serializer: JsonSerializer::new(),
        }
    }

    /// Get the codec ID.
    ///
    /// Matches Python: `@property def codec_id(self) -> str: return "jsonl"`
    pub fn codec_id(&self) -> &str {
        "jsonl"
    }

    /// Get media types.
    ///
    /// Matches Python: `@property def media_types(self) -> list[str]: return ["application/x-ndjson", "application/jsonl"]`
    pub fn media_types(&self) -> Vec<&str> {
        vec!["application/x-ndjson", "application/jsonl"]
    }

    /// Get file extensions.
    ///
    /// Matches Python: `@property def file_extensions(self) -> list[str]: return [".jsonl", ".ndjson", ".jsonlines"]`
    pub fn file_extensions(&self) -> Vec<&str> {
        vec![".jsonl", ".ndjson", ".jsonlines"]
    }

    /// Get format name.
    pub fn format_name(&self) -> &str {
        "JSONL"
    }

    /// Get MIME type.
    pub fn mime_type(&self) -> &str {
        "application/x-ndjson"
    }

    /// Check if binary format.
    pub fn is_binary_format(&self) -> bool {
        false
    }

    /// Get aliases.
    ///
    /// Matches Python: `@property def aliases(self) -> list[str]: return ["jsonl", "JSONL", "ndjson", "NDJSON", "jsonlines"]`
    pub fn aliases(&self) -> Vec<&str> {
        vec!["jsonl", "JSONL", "ndjson", "NDJSON", "jsonlines"]
    }

    /// Encode data to JSON Lines string.
    ///
    /// Matches Python: `def encode(self, data: Any, *, options: Optional[dict[str, Any]] = None) -> str`
    ///
    /// Args:
    ///     data: List of objects to encode (each becomes one line), or single object
    ///     options: Encoding options
    ///
    /// Returns:
    ///     JSON Lines string (one JSON object per line)
    pub fn encode(
        &self,
        data: &JsonValue,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Handle list or single object
        let items = if let JsonValue::Array(arr) = data {
            arr.clone()
        } else {
            vec![data.clone()]
        };

        let mut lines = Vec::new();
        for item in items {
            let line = self.json_serializer.encode(&item, options)?;
            lines.push(line);
        }

        Ok(lines.join("\n"))
    }

    /// Decode JSON Lines string to list of objects.
    ///
    /// Matches Python: `def decode(self, data: Union[str, bytes], *, options: Optional[dict[str, Any]] = None) -> list[Any]`
    ///
    /// Args:
    ///     data: JSON Lines string or bytes
    ///     options: Decoding options
    ///
    /// Returns:
    ///     List of decoded JSON values
    pub fn decode(
        &self,
        data: &[u8],
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<Vec<JsonValue>, Box<dyn std::error::Error>> {
        let json_str = String::from_utf8(data.to_vec())?;
        let lines: Vec<&str> = json_str.lines().collect();

        let mut results = Vec::new();
        for line in lines {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let value = self.json_serializer.decode_str(trimmed, options)?;
                results.push(value);
            }
        }

        Ok(results)
    }

    /// Decode JSON Lines string (str version).
    ///
    /// Convenience method for string input.
    pub fn decode_str(
        &self,
        data: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<Vec<JsonValue>, Box<dyn std::error::Error>> {
        self.decode(data.as_bytes(), options)
    }
}

impl Default for JsonLinesSerializer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ASERIALIZATION TRAIT IMPLEMENTATION
// ============================================================================
// This implements the ASerialization trait, matching Python's inheritance:
// Python: class JsonLinesSerializer(ASerialization)
// Rust: impl ASerialization for JsonLinesSerializer
//
// Note: JSONL decode returns Vec<JsonValue> (list of lines), which we wrap
// in JsonValue::Array to match the trait signature.
// ============================================================================

impl ASerialization for JsonLinesSerializer {
    fn codec_id(&self) -> &str {
        JsonLinesSerializer::codec_id(self)
    }

    fn media_types(&self) -> Vec<&str> {
        JsonLinesSerializer::media_types(self)
    }

    fn file_extensions(&self) -> Vec<&str> {
        JsonLinesSerializer::file_extensions(self)
    }

    fn format_name(&self) -> &str {
        JsonLinesSerializer::format_name(self)
    }

    fn mime_type(&self) -> &str {
        JsonLinesSerializer::mime_type(self)
    }

    fn is_binary_format(&self) -> bool {
        JsonLinesSerializer::is_binary_format(self)
    }

    fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Convert to JsonValue first
        let json_value = serde_json::to_value(value)?;
        JsonLinesSerializer::encode(self, &json_value, options)
    }

    fn decode(
        &self,
        repr: &[u8],
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // JSONL decode returns Vec<JsonValue>, wrap in Array
        let lines = JsonLinesSerializer::decode(self, repr, options)?;
        Ok(JsonValue::Array(lines))
    }

    fn decode_str(
        &self,
        repr: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // JSONL decode returns Vec<JsonValue>, wrap in Array
        let lines = JsonLinesSerializer::decode_str(self, repr, options)?;
        Ok(JsonValue::Array(lines))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_jsonlines_serializer_creation() {
        let serializer = JsonLinesSerializer::new();
        assert_eq!(serializer.codec_id(), "jsonl");
        assert_eq!(serializer.format_name(), "JSONL");
        assert_eq!(serializer.mime_type(), "application/x-ndjson");
        assert!(!serializer.is_binary_format());
    }

    #[test]
    fn test_jsonlines_encode_list() {
        let serializer = JsonLinesSerializer::new();
        let data = json!([
            {"name": "John", "age": 30},
            {"name": "Jane", "age": 25}
        ]);
        let result = serializer.encode(&data, None).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("John"));
        assert!(lines[1].contains("Jane"));
    }

    #[test]
    fn test_jsonlines_encode_single_object() {
        let serializer = JsonLinesSerializer::new();
        let data = json!({"name": "John", "age": 30});
        let result = serializer.encode(&data, None).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("John"));
    }

    #[test]
    fn test_jsonlines_decode() {
        let serializer = JsonLinesSerializer::new();
        let jsonl_str = r#"{"name": "John", "age": 30}
{"name": "Jane", "age": 25}"#;
        let result = serializer.decode_str(jsonl_str, None).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0]["name"], "John");
        assert_eq!(result[1]["name"], "Jane");
    }

    #[test]
    fn test_jsonlines_decode_skips_empty_lines() {
        let serializer = JsonLinesSerializer::new();
        let jsonl_str = r#"{"name": "John"}

{"name": "Jane"}"#;
        let result = serializer.decode_str(jsonl_str, None).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_jsonlines_encode_decode_roundtrip() {
        let serializer = JsonLinesSerializer::new();
        let original = json!([
            {"id": 1, "value": "test1"},
            {"id": 2, "value": "test2"}
        ]);
        
        let encoded = serializer.encode(&original, None).unwrap();
        let decoded = serializer.decode_str(&encoded, None).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0]["id"], 1);
        assert_eq!(decoded[1]["id"], 2);
    }

    #[test]
    fn test_jsonlines_media_types() {
        let serializer = JsonLinesSerializer::new();
        let media_types = serializer.media_types();
        assert!(media_types.contains(&"application/x-ndjson"));
        assert!(media_types.contains(&"application/jsonl"));
    }

    #[test]
    fn test_jsonlines_file_extensions() {
        let serializer = JsonLinesSerializer::new();
        let extensions = serializer.file_extensions();
        assert!(extensions.contains(&".jsonl"));
        assert!(extensions.contains(&".ndjson"));
        assert!(extensions.contains(&".jsonlines"));
    }
}

