// #exonware/xwsystem/rust/src/io/serialization/registry.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 2, 2025
//! 
//! Serialization Registry - Delegates to UniversalCodecRegistry.
//! 
//! Provides serialization-specific convenience methods for format discovery.


use std::collections::HashMap;
use std::path::Path;

use crate::io::serialization::contracts::ISerialization;
use crate::io::serialization::format_detector::FormatDetector;
use crate::io::serialization::auto_serializer::AutoSerializer;

// Global serialization registry instance

// Global serialization registry instance

/// Serialization registry - delegates to UniversalCodecRegistry.
///
/// Provides serialization-focused API on top of the universal codec registry.
///
/// Features:
/// - Format detection from file paths
/// - List all serialization formats
/// - Get serializer by format ID
/// - Get serializer by file extension
///
/// Examples:
/// >>> registry = SerializationRegistry()
/// >>>
/// >>> # Get serializer by ID
/// >>> json_ser = registry.get_by_format("json")
/// >>>
/// >>> # Auto-detect from file path
/// >>> ser = registry.detect_from_file("data.yaml")
/// >>>
/// >>> # List all formats
/// >>> formats = registry.list_formats()
pub struct SerializationRegistry {
    pub codec_registry: Option<String>, // Placeholder for UniversalCodecRegistry
    pub format_detector: FormatDetector,
    pub serializer_cache: HashMap<String, String>, // Placeholder for serializer instances
}

impl SerializationRegistry {
    /// Initialize serialization registry.
    ///
    ///
    /// Args:
    /// codec_registry: Optional UniversalCodecRegistry instance
    /// (defaults to global registry)
    pub fn new(codec_registry: Option<String>) -> Self {
        Self {
            codec_registry,
            format_detector: FormatDetector::new(None),
            serializer_cache: HashMap::new(),
        }
    }

    /// Get serializer by format ID.
    ///
    ///
    /// Args:
    /// format_id: Format identifier (e.g., 'json', 'yaml')
    ///
    ///
    /// Returns:
    /// Serializer instance or None
    ///
    /// Examples:
    /// >>> json_ser = registry.get_by_format("json")
    /// >>> yaml_ser = registry.get_by_format("yaml")
    pub fn get_by_format(&self, format_id: String) -> Option<Box<dyn ISerialization>> {
        // TODO: Delegate to UniversalCodecRegistry when available
        // For now, use AutoSerializer's _get_serializer method
        let mut auto_serializer = AutoSerializer::new(None, None);
        auto_serializer._get_serializer(&format_id).ok()
    }

    /// Auto-detect serializer from file path.
    ///
    /// Uses file extension to determine the appropriate serializer.
    ///
    ///
    /// Args:
    /// file_path: File path to detect from
    ///
    ///
    /// Returns:
    /// Serializer instance or None
    ///
    /// Examples:
    /// >>> ser = registry.detect_from_file("config.yaml")
    /// >>> data = ser.load_file("config.yaml")
    pub fn detect_from_file(&self, file_path: String) -> Option<Box<dyn ISerialization>> {
        // Use format detector to detect format from file path
        if let Some(format_name) = self.format_detector.get_best_format(Some(file_path), None, None) {
            let mut auto_serializer = AutoSerializer::new(None, None);
            auto_serializer._get_serializer(&format_name).ok()
        } else {
            None
        }
    }

    /// Get serializer by file extension.
    ///
    ///
    /// Args:
    /// extension: File extension (with or without dot)
    ///
    ///
    /// Returns:
    /// Serializer instance or None
    ///
    /// Examples:
    /// >>> json_ser = registry.get_by_extension(".json")
    /// >>> yaml_ser = registry.get_by_extension("yaml")
    pub fn get_by_extension(&self, extension: String) -> Option<Box<dyn ISerialization>> {
        // Normalize extension (add dot if missing)
        let ext = if extension.starts_with('.') {
            extension
        } else {
            format!(".{}", extension)
        };
        
        // Use format detector to get format from extension
        let formats = self.format_detector.detect_from_extension(format!("dummy{}", ext));
        if let Some(format_name) = formats.first() {
            let mut auto_serializer = AutoSerializer::new(None, None);
            auto_serializer._get_serializer(format_name).ok()
        } else {
            None
        }
    }

    /// Get serializer by MIME type.
    ///
    ///
    /// Args:
    /// mime_type: MIME type string
    ///
    ///
    /// Returns:
    /// Serializer instance or None
    ///
    /// Examples:
    /// >>> json_ser = registry.get_by_mime_type("application/json")
    pub fn get_by_mime_type(&self, mime_type: String) -> Option<Box<dyn ISerialization>> {
        // TODO: Implement MIME type to format mapping when available
        // For now, try common mappings
        let format_name = match mime_type.as_str() {
            "application/json" | "text/json" => "JSON",
            "application/x-yaml" | "text/yaml" => "YAML",
            "application/xml" | "text/xml" => "XML",
            "text/toml" => "TOML",
            "text/csv" => "CSV",
            _ => return None,
        };
        
        let mut auto_serializer = AutoSerializer::new(None, None);
        auto_serializer._get_serializer(format_name).ok()
    }

    /// List all registered format IDs.
    ///
    ///
    /// Returns:
    /// List of format identifiers
    ///
    /// Examples:
    /// >>> formats = registry.list_formats()
    /// >>> # ['json', 'yaml', 'toml', 'xml', ...]
    pub fn list_formats(&self) -> Vec<String> {
        // Return list of known formats
        vec![
            "JSON".to_string(), "YAML".to_string(), "TOML".to_string(),
            "XML".to_string(), "CSV".to_string(), "ConfigParser".to_string(),
            "BSON".to_string(), "MessagePack".to_string(), "CBOR".to_string(),
            "Pickle".to_string(), "Marshal".to_string(), "Plistlib".to_string(),
            "SQLite3".to_string(), "DBM".to_string(), "Shelve".to_string(),
        ]
    }

    /// List all registered file extensions.
    ///
    ///
    /// Returns:
    /// List of file extensions
    ///
    /// Examples:
    /// >>> extensions = registry.list_extensions()
    /// >>> # ['.json', '.yaml', '.yml', '.toml', ...]
    pub fn list_extensions(&self) -> Vec<String> {
        // Return list of known extensions
        vec![
            ".json".to_string(), ".yaml".to_string(), ".yml".to_string(),
            ".toml".to_string(), ".xml".to_string(), ".csv".to_string(),
            ".ini".to_string(), ".cfg".to_string(), ".bson".to_string(),
            ".msgpack".to_string(), ".mp".to_string(), ".cbor".to_string(),
            ".pkl".to_string(), ".pickle".to_string(), ".p".to_string(),
            ".marshal".to_string(), ".db".to_string(), ".sqlite".to_string(),
            ".sqlite3".to_string(), ".dbm".to_string(), ".shelf".to_string(),
            ".plist".to_string(),
        ]
    }

    /// List all registered MIME types.
    ///
    ///
    /// Returns:
    /// List of MIME types
    ///
    /// Examples:
    /// >>> mime_types = registry.list_mime_types()
    /// >>> # ['application/json', 'application/x-yaml', ...]
    pub fn list_mime_types(&self) -> Vec<String> {
        // Return list of known MIME types
        vec![
            "application/json".to_string(), "text/json".to_string(),
            "application/x-yaml".to_string(), "text/yaml".to_string(),
            "application/xml".to_string(), "text/xml".to_string(),
            "text/toml".to_string(), "text/csv".to_string(),
            "application/bson".to_string(), "application/x-msgpack".to_string(),
            "application/cbor".to_string(),
        ]
    }

    /// Register a serializer class.
    ///
    ///
    /// Args:
    /// serializer_class: Serializer class to register
    ///
    /// Examples:
    /// >>> registry.register(JsonSerializer)
    pub fn register(&self, serializer_class: String) {
        // TODO: Implement registration when UniversalCodecRegistry is available
        // For now, this is a placeholder
    }

}

    /// Get the global serialization registry.
    ///
    ///
    /// Returns:
    /// Global SerializationRegistry instance
    pub fn get_serialization_registry() -> SerializationRegistry {
        // Return global registry instance
        SerializationRegistry::new(None)
    }
