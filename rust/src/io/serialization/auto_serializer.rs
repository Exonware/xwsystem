// #exonware/xwsystem/rust/src/io/serialization/auto_serializer.rs
//exonware\xwsystem\serialization\auto_serializer.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Automatic serializer that detects format and delegates to appropriate serializer.


use std::collections::HashMap;

use std::collections::HashMap;
use std::path::Path;
use std::fs;

use crate::config::logging_setup::get_logger;
use crate::io::serialization::contracts::ISerialization;
use crate::io::serialization::format_detector::{FormatDetector, detect_format};
use crate::io::serialization::formats_text::{
    JsonSerializer, YamlSerializer, TomlSerializer, XmlSerializer, 
    CsvSerializer, ConfigParserSerializer, FormDataSerializer, MultipartSerializer,
    JsonLinesSerializer, Json5Serializer
};
use crate::io::serialization::formats_binary::{
    BsonSerializer, CborSerializer, MsgPackSerializer, PickleSerializer,
    MarshalSerializer, PlistSerializer
};
use crate::io::serialization::formats_database::{
    Sqlite3Serializer, DbmSerializer, ShelveSerializer
};
use crate::io::serialization::errors::SerializationError;

// Global auto-serializer instance

// Global auto-serializer instance

// First, try UniversalCodecRegistry for auto-registered formats
// All auto-registering formats (xwjson, xwformats, xwsyntax, etc.) register themselves
// when their modules are imported, so we just check the registry - no special cases needed
// Try format name as-is, then lowercase, then uppercase, then aliases
// Check if codec implements ISerialization (has dumps/loads)
// Codec adapters (ICodec only) have encode/decode but NOT dumps/loads
// This is a full ISerialization implementation
// This is only ICodec (codec adapter), skip it
// Registry not available or format not found, continue to hardcoded map
// Fallback to hardcoded module_map for xwsystem formats
// Dynamic import to avoid circular dependencies.
// Root cause fix: import concrete serializers from the canonical
// xwsystem.io.serialization.formats packages instead of a parallel
// exonware.xwsystem.serialization namespace (which should not exist).
// Binary / database formats
// Schema-based / advanced formats (placeholders for future modules)
// These entries intentionally point to non-existent modules today.
// The lazy installation system will handle installing/adding them
// when the corresponding format implementations are introduced.
// Import from canonical xwsystem.io serialization path
// Lazy installation system will handle missing dependencies
// First try to get instance from registry (for auto-registered formats)
// Try format name as-is, then lowercase, then uppercase, then aliases
// Check if codec has ISerialization interface (dumps/loads methods)
// Codec adapters (ICodec only) have encode/decode but NOT dumps/loads
// We need ISerialization (which extends ICodec) with dumps/loads
// This is a full ISerialization implementation
// This is only ICodec (codec adapter), not ISerialization
// Skip it - we need a serializer with dumps/loads for detect_and_serialize
// Registry not available or codec not compatible, create from class
// Fallback: create instance from class
// Use format hint if provided
// Try to detect from file extension
// Use format hint if provided
// Try multiple detection methods
// Use format hint if provided
// Use format hint if provided
// Try to read a small sample for content detection
// Fallback to extension-based detection
// Use format hint if provided
// Use format hint if provided
// For async, we'll primarily rely on extension detection
// to avoid blocking I/O for content sampling
/// Intelligent auto-serializer that automatically detects format and
/// delegates to the appropriate specialized serializer.
pub struct AutoSerializer {
    pub detector: FormatDetector,
    pub serializer_cache: HashMap<String, Box<dyn ISerialization>>,
    pub default_format: String,
    pub confidence_threshold: f64,
}

impl AutoSerializer {
    /// Initialize auto-serializer.
    ///
    ///
    /// Args:
    /// default_format: Default format when detection fails
    /// confidence_threshold: Minimum confidence for format detection
    pub fn new(
        default_format: Option<String>,
        confidence_threshold: Option<f64>
    ) -> Self {
        Self {
            detector: FormatDetector::new(confidence_threshold),
            serializer_cache: HashMap::new(),
            default_format: default_format.unwrap_or_else(|| "JSON".to_string()),
            confidence_threshold: confidence_threshold.unwrap_or(0.7),
        }
    }

    // First, try UniversalCodecRegistry for auto-registered formats
    // All auto-registering formats (xwjson, xwformats, xwsyntax, etc.) register themselves
    // when their modules are imported, so we just check the registry - no special cases needed
    // Try format name as-is, then lowercase, then uppercase, then aliases
    // Check if codec implements ISerialization (has dumps/loads)
    // Codec adapters (ICodec only) have encode/decode but NOT dumps/loads
    // This is a full ISerialization implementation
    // This is only ICodec (codec adapter), skip it
    // Registry not available or format not found, continue to hardcoded map
    // Fallback to hardcoded module_map for xwsystem formats
    // Dynamic import to avoid circular dependencies.
    // Root cause fix: import concrete serializers from the canonical
    // xwsystem.io.serialization.formats packages instead of a parallel
    // exonware.xwsystem.serialization namespace (which should not exist).
    // Binary / database formats
    // Schema-based / advanced formats (placeholders for future modules)
    // These entries intentionally point to non-existent modules today.
    // The lazy installation system will handle installing/adding them
    // when the corresponding format implementations are introduced.
    // Import from canonical xwsystem.io serialization path
    // Lazy installation system will handle missing dependencies
    /// Get serializer class for format name.
    ///
    /// First checks UniversalCodecRegistry for auto-registered formats (like XWJSON),
    /// then falls back to hardcoded module_map for xwsystem formats.
    ///
    ///
    /// Args:
    /// format_name: Format name (e.g., 'JSON', 'YAML', 'XWJSON')
    ///
    ///
    /// Returns:
    /// Format name (for factory pattern)
    ///
    ///
    /// Raises:
    /// SerializationError: If format not available
    fn _get_serializer_class(&self, format_name: &str) -> Result<String, SerializationError> {
        // TODO: Check UniversalCodecRegistry first (when available)
        // For now, use hardcoded format mapping
        
        let format_upper = format_name.to_uppercase();
        
        // Validate format is supported
        match format_upper.as_str() {
            "JSON" | "JSONL" | "NDJSON" | "YAML" | "TOML" | "XML" | "CSV" |
            "CONFIGPARSER" | "FORMDATA" | "MULTIPART" | "JSON5" |
            "BSON" | "MESSAGEPACK" | "CBOR" | "PICKLE" | "MARSHAL" | "PLISTLIB" |
            "SQLITE3" | "DBM" | "SHELVE" => Ok(format_upper),
            _ => Err(SerializationError::new(format!("Unknown format: {}", format_name)))
        }
    }

    // First try to get instance from registry (for auto-registered formats)
    // Try format name as-is, then lowercase, then uppercase, then aliases
    // Check if codec has ISerialization interface (dumps/loads methods)
    // Codec adapters (ICodec only) have encode/decode but NOT dumps/loads
    // We need ISerialization (which extends ICodec) with dumps/loads
    // This is a full ISerialization implementation
    // This is only ICodec (codec adapter), not ISerialization
    // Skip it - we need a serializer with dumps/loads for detect_and_serialize
    // Registry not available or codec not compatible, create from class
    // Fallback: create instance from class
    /// Get cached serializer instance for format.
    ///
    /// First checks UniversalCodecRegistry for instances, then creates from class.
    ///
    ///
    /// Args:
    /// format_name: Format name
    ///
    ///
    /// Returns:
    /// Serializer instance
    pub fn _get_serializer(&mut self, format_name: &str) -> Result<Box<dyn ISerialization>, SerializationError> {
        let format_upper = format_name.to_uppercase();
        
        // TODO: Check UniversalCodecRegistry first (when available)
        
        // Create serializer based on format name
        // Note: For now we create new instances each time since Box<dyn Trait> can't be cloned
        // TODO: Use Arc<dyn ISerialization> or factory pattern for proper caching
        match format_upper.as_str() {
            "JSON" => Ok(Box::new(JsonSerializer::new(None))),
            "YAML" => Ok(Box::new(YamlSerializer::new())),
            "TOML" => Ok(Box::new(TomlSerializer::new())),
            "XML" => Ok(Box::new(XmlSerializer::new())),
            "CSV" => Ok(Box::new(CsvSerializer::new())),
            "CONFIGPARSER" => Ok(Box::new(ConfigParserSerializer::new())),
            "FORMDATA" => Ok(Box::new(FormDataSerializer::new())),
            "MULTIPART" => Ok(Box::new(MultipartSerializer::new())),
            "JSONL" | "NDJSON" => Ok(Box::new(JsonLinesSerializer::new())),
            "JSON5" => Ok(Box::new(Json5Serializer::new())),
            "BSON" => Ok(Box::new(BsonSerializer::new())),
            "MESSAGEPACK" => Ok(Box::new(MsgPackSerializer::new())),
            "CBOR" => Ok(Box::new(CborSerializer::new())),
            "PICKLE" => Ok(Box::new(PickleSerializer::new())),
            "MARSHAL" => Ok(Box::new(MarshalSerializer::new())),
            "PLISTLIB" => Ok(Box::new(PlistSerializer::new())),
            "SQLITE3" => Ok(Box::new(Sqlite3Serializer::new())),
            "DBM" => Ok(Box::new(DbmSerializer::new())),
            "SHELVE" => Ok(Box::new(ShelveSerializer::new())),
            _ => Err(SerializationError::new(format!("Unknown format: {}", format_name)))
        }
    }

    // Use format hint if provided
    // Try to detect from file extension
    /// Auto-detect format and serialize data.
    ///
    ///
    /// Args:
    /// data: Data to serialize
    /// file_path: Optional file path for format detection
    /// format_hint: Optional format hint to use
    /// **opts: Additional serializer-specific options (pretty, indent, etc.)
    ///
    ///
    /// Returns:
    /// Serialized data
    pub fn detect_and_serialize(&mut self, data: serde_json::Value, file_path: Option<String>, format_hint: Option<String>, opts: HashMap<String, String>) -> Result<String, SerializationError> {
        // Use format hint if provided
        let format_name = if let Some(hint) = format_hint {
            hint.to_uppercase()
        } else {
            // Try to detect from file extension
            if let Some(path) = &file_path {
                self.detector.get_best_format(Some(path.clone()), None, None)
                    .unwrap_or_else(|| self.default_format.clone())
            } else {
                self.default_format.clone()
            }
        };
        
        let serializer = self._get_serializer(&format_name)?;
        let encoded = serializer.encode(data);
        String::from_utf8(encoded).map_err(|e| SerializationError::new(format!("Failed to convert to string: {}", e)))
    }

    // Use format hint if provided
    // Try multiple detection methods
    /// Auto-detect format and deserialize data.
    ///
    ///
    /// Args:
    /// data: Data to deserialize
    /// file_path: Optional file path for format detection
    /// format_hint: Optional format hint to use
    ///
    ///
    /// Returns:
    /// Deserialized object
    pub fn detect_and_deserialize(&mut self, data: String, file_path: Option<String>, format_hint: Option<String>) -> Result<serde_json::Value, SerializationError> {
        // Use format hint if provided
        let format_name = if let Some(hint) = format_hint {
            hint.to_uppercase()
        } else {
            // Try multiple detection methods
            self.detector.get_best_format(
                file_path.clone(),
                Some(data.clone()),
                None
            ).unwrap_or_else(|| self.default_format.clone())
        };
        
        let serializer = self._get_serializer(&format_name)?;
        let data_bytes = data.into_bytes();
        Ok(serializer.decode(data_bytes))
    }

    // Use format hint if provided
    /// Auto-detect format and save to file.
    ///
    ///
    /// Args:
    /// data: Data to save
    /// file_path: File path (format detected from extension)
    /// format_hint: Optional format hint to override detection
    pub fn auto_save_file(&mut self, data: serde_json::Value, file_path: String, format_hint: Option<String>) -> Result<(), SerializationError> {
        // Use format hint if provided
        let format_name = if let Some(hint) = format_hint {
            hint.to_uppercase()
        } else {
            self.detector.get_best_format(Some(file_path.clone()), None, None)
                .unwrap_or_else(|| {
                    // Warning logged in Python, we'll use default
                    self.default_format.clone()
                })
        };
        
        let serializer = self._get_serializer(&format_name)?;
        serializer.save_file(data, file_path);
        Ok(())
    }

    // Use format hint if provided
    // Try to read a small sample for content detection
    // Fallback to extension-based detection
    /// Auto-detect format and load from file.
    ///
    ///
    /// Args:
    /// file_path: File path to load
    /// format_hint: Optional format hint to override detection
    ///
    ///
    /// Returns:
    /// Loaded data
    pub fn auto_load_file(&mut self, file_path: String, format_hint: Option<String>) -> Result<serde_json::Value, SerializationError> {
        let path = Path::new(&file_path);
        
        if !path.exists() {
            return Err(SerializationError::new(format!("File not found: {}", file_path)));
        }
        
        // Use format hint if provided
        let format_name = if let Some(hint) = format_hint {
            hint.to_uppercase()
        } else {
            // Try to read a small sample for content detection
            let format_from_ext = self.detector.get_best_format(Some(file_path.clone()), None, None);
            
            if let Some(ref detected_format) = format_from_ext {
                // Check if binary format
                if self.detector.is_binary_format(detected_format.clone()) {
                    // Read binary sample
                    if let Ok(sample) = fs::read(path).map(|bytes| bytes.into_iter().take(1024).collect::<Vec<u8>>()) {
                        self.detector.get_best_format(Some(file_path.clone()), None, Some(sample))
                            .unwrap_or_else(|| self.default_format.clone())
                    } else {
                        format_from_ext.unwrap_or_else(|| self.default_format.clone())
                    }
                } else {
                    // Read text sample
                    if let Ok(sample) = fs::read_to_string(path).map(|s| s.chars().take(1024).collect::<String>()) {
                        self.detector.get_best_format(Some(file_path.clone()), Some(sample), None)
                            .unwrap_or_else(|| self.default_format.clone())
                    } else {
                        format_from_ext.unwrap_or_else(|| self.default_format.clone())
                    }
                }
            } else {
                self.default_format.clone()
            }
        };
        
        let serializer = self._get_serializer(&format_name)?;
        Ok(serializer.load_file(file_path))
    }

    // Use format hint if provided
    /// Auto-detect format and save to file asynchronously.
    ///
    ///
    /// Args:
    /// data: Data to save
    /// file_path: File path (format detected from extension)
    /// format_hint: Optional format hint to override detection
    pub async fn auto_save_file_async(&mut self, data: serde_json::Value, file_path: String, format_hint: Option<String>) -> Result<(), SerializationError> {
        // Use format hint if provided
        let format_name = if let Some(hint) = format_hint {
            hint.to_uppercase()
        } else {
            self.detector.get_best_format(Some(file_path.clone()), None, None)
                .unwrap_or_else(|| self.default_format.clone())
        };
        
        let serializer = self._get_serializer(&format_name)?;
        serializer.save_file_async(data, file_path).await;
        Ok(())
    }

    // Use format hint if provided
    // For async, we'll primarily rely on extension detection
    // to avoid blocking I/O for content sampling
    /// Auto-detect format and load from file asynchronously.
    ///
    ///
    /// Args:
    /// file_path: File path to load
    /// format_hint: Optional format hint to override detection
    ///
    ///
    /// Returns:
    /// Loaded data
    pub async fn auto_load_file_async(&mut self, file_path: String, format_hint: Option<String>) -> Result<serde_json::Value, SerializationError> {
        let path = Path::new(&file_path);
        
        if !path.exists() {
            return Err(SerializationError::new(format!("File not found: {}", file_path)));
        }
        
        // Use format hint if provided
        let format_name = if let Some(hint) = format_hint {
            hint.to_uppercase()
        } else {
            // For async, we'll primarily rely on extension detection
            // to avoid blocking I/O for content sampling
            self.detector.get_best_format(Some(file_path.clone()), None, None)
                .unwrap_or_else(|| self.default_format.clone())
        };
        
        let serializer = self._get_serializer(&format_name)?;
        Ok(serializer.load_file_async(file_path).await)
    }

    /// Get format suggestions for given data.
    ///
    ///
    /// Args:
    /// data: Data to analyze
    /// file_path: Optional file path
    ///
    ///
    /// Returns:
    /// List of (format_name, confidence) tuples
    pub fn get_format_suggestions(&self, data: String, file_path: Option<String>) -> Vec<(String, f64)> {
        self.detector.get_format_suggestions(
            file_path,
            Some(data),
            None,
            None
        )
    }

    /// Clear serializer cache.
    pub fn clear_cache(&mut self)
    {
        self.serializer_cache.clear();
    }

}

    /// Convenience function for auto-serialization.
    ///
    ///
    /// Args:
    /// data: Data to serialize
    /// file_path: Optional file path for format detection
    /// format_hint: Optional format hint
    /// **opts: Additional serializer options
    ///
    ///
    /// Returns:
    /// Serialized data
    pub fn auto_serialize(data: serde_json::Value, file_path: Option<String>, format_hint: Option<String>, opts: HashMap<String, String>) -> Result<String, SerializationError> {
        let mut serializer = AutoSerializer::new(None, None);
        serializer.detect_and_serialize(data, file_path, format_hint, opts)
    }

    /// Convenience function for auto-deserialization.
    ///
    ///
    /// Args:
    /// data: Data to deserialize
    /// file_path: Optional file path for format detection
    /// format_hint: Optional format hint
    ///
    ///
    /// Returns:
    /// Deserialized object
    pub fn auto_deserialize(data: &str, file_path: Option<String>, format_hint: Option<String>) -> Result<serde_json::Value, SerializationError> {
        let mut serializer = AutoSerializer::new(None, None);
        serializer.detect_and_deserialize(data.to_string(), file_path, format_hint)
    }

    /// Convenience function for auto-saving files.
    ///
    ///
    /// Args:
    /// data: Data to save
    /// file_path: File path
    /// format_hint: Optional format hint
    pub fn auto_save_file(data: serde_json::Value, file_path: &str, format_hint: Option<String>) -> Result<(), SerializationError> {
        let mut serializer = AutoSerializer::new(None, None);
        serializer.auto_save_file(data, file_path.to_string(), format_hint)
    }

    /// Convenience function for auto-loading files.
    ///
    ///
    /// Args:
    /// file_path: File path to load
    /// format_hint: Optional format hint
    ///
    ///
    /// Returns:
    /// Loaded data
    pub fn auto_load_file(file_path: &str, format_hint: Option<String>) -> Result<serde_json::Value, SerializationError> {
        let mut serializer = AutoSerializer::new(None, None);
        serializer.auto_load_file(file_path.to_string(), format_hint)
    }
