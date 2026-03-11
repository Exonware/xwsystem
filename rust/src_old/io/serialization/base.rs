// #exonware/xwsystem/rust/src_old/io/serialization/base.rs
//! Serialization base trait - ASerialization.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This module provides the ASerialization trait matching Python's ASerialization abstract base class.
//!
//! Following I→A pattern:
//! - I: ISerialization (interface - to be implemented)
//! - A: ASerialization (abstract base trait - this file)
//! - Concrete: JsonSerializer, YamlSerializer, etc.

use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::Path;

/// Abstract base trait for serialization.
///
/// Matches Python's ASerialization abstract base class.
/// Provides the core interface that all serializers must implement.
///
/// Subclasses must implement:
/// - encode()
/// - decode()
/// - Metadata properties (codec_id, media_types, file_extensions, etc.)
pub trait ASerialization {
    /// Get the codec identifier (e.g., "json", "yaml").
    fn codec_id(&self) -> &str;

    /// Get supported MIME types.
    fn media_types(&self) -> Vec<&str>;

    /// Get supported file extensions.
    fn file_extensions(&self) -> Vec<&str>;

    /// Get format name (default: uppercase codec_id).
    fn format_name(&self) -> &str;

    /// Get primary MIME type (default: first in media_types).
    fn mime_type(&self) -> &str {
        self.media_types().first().unwrap_or(&"application/octet-stream")
    }

    /// Check if binary format (default: false).
    fn is_binary_format(&self) -> bool {
        false
    }

    /// Check if streaming is supported (default: false).
    fn supports_streaming(&self) -> bool {
        false
    }

    /// Get codec capabilities (default: BIDIRECTIONAL).
    ///
    /// Matches Python: `@property def capabilities(self) -> CodecCapability: return CodecCapability.BIDIRECTIONAL`
    fn capabilities(&self) -> crate::defs::CodecCapability {
        crate::defs::CodecCapability::BIDIRECTIONAL
    }

    /// Get codec types (default: ["serialization"]).
    ///
    /// Matches Python: `@property def codec_types(self) -> list[str]: return ["serialization"]`
    fn codec_types(&self) -> Vec<&str> {
        vec!["serialization"]
    }

    /// Encode data to representation.
    ///
    /// Args:
    ///     value: Data to serialize (must implement Serialize)
    ///     options: Encoding options
    ///
    /// Returns:
    ///     Serialized representation (String for text formats)
    fn encode<T: serde::Serialize>(
        &self,
        value: &T,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<String, Box<dyn std::error::Error>>;

    /// Decode representation to data.
    ///
    /// Args:
    ///     repr: Serialized representation (bytes)
    ///     options: Decoding options
    ///
    /// Returns:
    ///     Decoded JSON value
    fn decode(
        &self,
        repr: &[u8],
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>>;

    /// Decode from string (convenience method).
    fn decode_str(
        &self,
        repr: &str,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        self.decode(repr.as_bytes(), options)
    }

    /// Save data to file (default implementation).
    ///
    /// Default implementation encodes data and writes to file.
    /// Can be overridden for format-specific optimizations.
    fn save_file<T: serde::Serialize>(
        &self,
        data: &T,
        file_path: &Path,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;
        
        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Encode data
        let repr = self.encode(data, options)?;
        
        // Write to file
        fs::write(file_path, repr)?;
        
        Ok(())
    }

    /// Load data from file (default implementation).
    ///
    /// Default implementation reads file and decodes data.
    /// Can be overridden for format-specific optimizations.
    fn load_file(
        &self,
        file_path: &Path,
        options: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        use std::fs;
        
        // Read from file
        let repr = fs::read(file_path)?;
        
        // Decode data
        self.decode(&repr, options)
    }

    /// Validate data for serialization compatibility (default implementation).
    ///
    /// Default implementation tries to encode and catches errors.
    /// Can be overridden for format-specific validation.
    fn validate_data<T: serde::Serialize>(
        &self,
        data: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Try to encode - if it succeeds, data is valid
        self.encode(data, None)?;
        Ok(())
    }
}

