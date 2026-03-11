// #exonware/xwsystem/rust/src_old/io/serialization/formats/text/csv.rs
//! CSV serialization - Comma-separated values format.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! CSV serializer using csv crate.
//! Matches Python's CsvSerializer class 100%.
//!
//! Following I→A pattern:
//! - I: ISerialization (interface)
//! - A: ASerialization (abstract base trait)
//! - Concrete: CsvSerializer (implements ASerialization)

use csv::{ReaderBuilder, WriterBuilder};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use crate::io::serialization::base::ASerialization;

/// CSV serializer - matches Python's CsvSerializer class.
///
/// Uses csv crate for reading/writing CSV data.
pub struct CsvSerializer;

impl CsvSerializer {
    /// Create a new CSV serializer.
    ///
    /// Matches Python: `CsvSerializer()`
    pub fn new() -> Self {
        CsvSerializer
    }

    /// Get the codec ID.
    ///
    /// Matches Python: `@property def codec_id(self) -> str: return "csv"`
    pub fn codec_id(&self) -> &str {
        "csv"
    }

    /// Get media types.
    ///
    /// Matches Python: `@property def media_types(self) -> list[str]: return ["text/csv", "application/csv"]`
    pub fn media_types(&self) -> Vec<&str> {
        vec!["text/csv", "application/csv"]
    }

    /// Get file extensions.
    ///
    /// Matches Python: `@property def file_extensions(self) -> list[str]: return [".csv", ".tsv", ".psv"]`
    pub fn file_extensions(&self) -> Vec<&str> {
        vec![".csv", ".tsv", ".psv"]
    }

    /// Get format name.
    ///
    /// Matches Python: `@property def format_name(self) -> str: return "CSV"`
    pub fn format_name(&self) -> &str {
        "CSV"
    }

    /// Get MIME type.
    ///
    /// Matches Python: `@property def mime_type(self) -> str: return "text/csv"`
    pub fn mime_type(&self) -> &str {
        "text/csv"
    }

    /// Check if binary format.
    ///
    /// Matches Python: `@property def is_binary_format(self) -> bool: return False`
    pub fn is_binary_format(&self) -> bool {
        false
    }

    /// Get aliases.
    ///
    /// Matches Python: `@property def aliases(self) -> list[str]: return ["csv", "CSV", "tsv", "TSV"]`
    pub fn aliases(&self) -> Vec<&str> {
        vec!["csv", "CSV", "tsv", "TSV"]
    }

    /// Encode data to CSV string.
    ///
    /// Matches Python: `def encode(self, data: Any, *, options: Optional[EncodeOptions] = None) -> str`
    ///
    /// Args:
    ///     data: List of objects (each becomes a row) or single object
    ///     options: CSV encoding options (delimiter, headers, etc.)
    ///
    /// Returns:
    ///     CSV string
    pub fn encode(
        &self,
        data: &JsonValue,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Get delimiter from options (default: comma)
        let delimiter = options
            .and_then(|opts| opts.get("delimiter"))
            .and_then(|v| v.as_str())
            .and_then(|s| s.as_bytes().first().copied())
            .unwrap_or(b',');

        // Handle list of objects or single object
        let rows = if let JsonValue::Array(arr) = data {
            arr.clone()
        } else {
            vec![data.clone()]
        };

        let mut writer = WriterBuilder::new()
            .delimiter(delimiter)
            .has_headers(true)
            .from_writer(Vec::new());

        // Extract headers from first row if it's an object
        if let Some(JsonValue::Object(first_row)) = rows.first() {
            let headers: Vec<String> = first_row.keys().cloned().collect();
            writer.write_record(&headers)?;
        }

        // Write rows
        for row in rows {
            if let JsonValue::Object(obj) = row {
                let values: Vec<String> = obj.values()
                    .map(|v| match v {
                        JsonValue::String(s) => s.clone(),
                        JsonValue::Number(n) => n.to_string(),
                        JsonValue::Bool(b) => b.to_string(),
                        JsonValue::Null => String::new(),
                        _ => serde_json::to_string(v).unwrap_or_default(),
                    })
                    .collect();
                writer.write_record(&values)?;
            }
        }

        writer.flush()?;
        let bytes = writer.into_inner().unwrap();
        Ok(String::from_utf8(bytes)?)
    }

    /// Decode CSV string to list of objects.
    ///
    /// Matches Python: `def decode(self, data: Union[str, bytes], *, options: Optional[DecodeOptions] = None) -> list[Any]`
    ///
    /// Args:
    ///     data: CSV string or bytes
    ///     options: CSV decoding options (delimiter, headers, etc.)
    ///
    /// Returns:
    ///     List of decoded objects (as JSON values)
    pub fn decode(
        &self,
        data: &[u8],
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<Vec<JsonValue>, Box<dyn std::error::Error>> {
        // Get delimiter from options (default: comma)
        let delimiter = options
            .and_then(|opts| opts.get("delimiter"))
            .and_then(|v| v.as_str())
            .and_then(|s| s.as_bytes().first().copied())
            .unwrap_or(b',');

        let csv_str = String::from_utf8(data.to_vec())?;
        let mut reader = ReaderBuilder::new()
            .delimiter(delimiter)
            .has_headers(true)
            .from_reader(csv_str.as_bytes());

        let headers = reader.headers()?.clone();
        let mut results = Vec::new();

        for result in reader.records() {
            let record = result?;
            let mut row_obj = serde_json::Map::new();
            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    // Try to parse as number or boolean, otherwise keep as string
                    let value = if let Ok(num) = field.parse::<i64>() {
                        JsonValue::Number(num.into())
                    } else if let Ok(num) = field.parse::<f64>() {
                        JsonValue::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from(0)))
                    } else if field == "true" {
                        JsonValue::Bool(true)
                    } else if field == "false" {
                        JsonValue::Bool(false)
                    } else {
                        JsonValue::String(field.to_string())
                    };
                    row_obj.insert(header.to_string(), value);
                }
            }
            results.push(JsonValue::Object(row_obj));
        }

        Ok(results)
    }

    /// Decode CSV string (str version).
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

impl Default for CsvSerializer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ASERIALIZATION TRAIT IMPLEMENTATION
// ============================================================================
// This implements the ASerialization trait, matching Python's inheritance:
// Python: class CsvSerializer(ASerialization)
// Rust: impl ASerialization for CsvSerializer
//
// Note: CSV decode returns Vec<JsonValue> (list of rows), which we wrap
// in JsonValue::Array to match the trait signature.
// ============================================================================

impl ASerialization for CsvSerializer {
    fn codec_id(&self) -> &str {
        CsvSerializer::codec_id(self)
    }

    fn media_types(&self) -> Vec<&str> {
        CsvSerializer::media_types(self)
    }

    fn file_extensions(&self) -> Vec<&str> {
        CsvSerializer::file_extensions(self)
    }

    fn format_name(&self) -> &str {
        CsvSerializer::format_name(self)
    }

    fn mime_type(&self) -> &str {
        CsvSerializer::mime_type(self)
    }

    fn is_binary_format(&self) -> bool {
        CsvSerializer::is_binary_format(self)
    }

    fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Convert to JsonValue first
        let json_value = serde_json::to_value(value)?;
        CsvSerializer::encode(self, &json_value, options)
    }

    fn decode(
        &self,
        repr: &[u8],
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // CSV decode returns Vec<JsonValue>, wrap in Array
        let rows = CsvSerializer::decode(self, repr, options)?;
        Ok(JsonValue::Array(rows))
    }

    fn decode_str(
        &self,
        repr: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // CSV decode returns Vec<JsonValue>, wrap in Array
        let rows = CsvSerializer::decode_str(self, repr, options)?;
        Ok(JsonValue::Array(rows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_csv_serializer_creation() {
        let serializer = CsvSerializer::new();
        assert_eq!(serializer.codec_id(), "csv");
        assert_eq!(serializer.format_name(), "CSV");
        assert_eq!(serializer.mime_type(), "text/csv");
        assert!(!serializer.is_binary_format());
    }

    #[test]
    fn test_csv_encode_list() {
        let serializer = CsvSerializer::new();
        let data = json!([
            {"name": "John", "age": 30},
            {"name": "Jane", "age": 25}
        ]);
        let result = serializer.encode(&data, None).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("John"));
        assert!(result.contains("Jane"));
        assert!(result.contains("30"));
        assert!(result.contains("25"));
    }

    #[test]
    fn test_csv_decode_basic() {
        let serializer = CsvSerializer::new();
        let csv_str = "name,age\nJohn,30\nJane,25";
        let result = serializer.decode_str(csv_str, None).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0]["name"], "John");
        assert_eq!(result[0]["age"], 30);
        assert_eq!(result[1]["name"], "Jane");
        assert_eq!(result[1]["age"], 25);
    }

    #[test]
    fn test_csv_encode_decode_roundtrip() {
        let serializer = CsvSerializer::new();
        let original = json!([
            {"id": 1, "value": "test1"},
            {"id": 2, "value": "test2"}
        ]);
        
        let encoded = serializer.encode(&original, None).unwrap();
        let decoded = serializer.decode_str(&encoded, None).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0]["id"], 1);
        assert_eq!(decoded[0]["value"], "test1");
        assert_eq!(decoded[1]["id"], 2);
        assert_eq!(decoded[1]["value"], "test2");
    }

    #[test]
    fn test_csv_media_types() {
        let serializer = CsvSerializer::new();
        let media_types = serializer.media_types();
        assert!(media_types.contains(&"text/csv"));
        assert!(media_types.contains(&"application/csv"));
    }

    #[test]
    fn test_csv_file_extensions() {
        let serializer = CsvSerializer::new();
        let extensions = serializer.file_extensions();
        assert!(extensions.contains(&".csv"));
        assert!(extensions.contains(&".tsv"));
    }
}

