// #exonware/xwsystem/rust/src/io/serialization/format_detector.rs
//exonware\xwsystem\serialization\format_detector.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Intelligent format detection for automatic serialization format selection.


use std::collections::HashMap;

use crate::config::logging_setup::{get_logger};
use std::path::{Path};

// Global format detector instance

// Global format detector instance

// Binary formats with clear magic bytes
// Some MessagePack implementations
// Avro object container
// Binary content, check magic bytes
// Limit content size for performance
// Normalize confidence based on number of patterns
// Extension-based detection (high confidence)
// Magic bytes detection (highest confidence)
// Content-based detection
// Sort by confidence descending
/// Intelligent format detector that can identify serialization formats
/// from file extensions, content analysis, and magic bytes.
pub struct FormatDetector {
    pub extension_map: HashMap<String, Vec<String>>,
    pub magic_bytes: HashMap<Vec<u8>, Vec<String>>,
    pub content_patterns: HashMap<String, Vec<(String, f64)>>,
    pub confidence_threshold: f64,
}

impl FormatDetector {
    /// Initialize format detector.
    ///
    ///
    /// Args:
    /// confidence_threshold: Minimum confidence level for format detection.
    /// Default is 0.3 so that simple, high-signal heuristics like
    /// extension-only detection (e.g. ``test.json``) are accepted
    /// without requiring magic-bytes or content analysis.
    pub fn new(confidence_threshold: Option<f64>) -> Self {
        let threshold = confidence_threshold.unwrap_or(0.3);
        Self {
            extension_map: Self::_build_extension_map(),
            magic_bytes: Self::_build_magic_bytes_map(),
            content_patterns: Self::_build_content_patterns(),
            confidence_threshold: threshold,
        }
    }

    /// Build mapping from file extensions to format names.
    fn _build_extension_map() -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        // Text formats
        map.insert(".json".to_string(), vec!["JSON".to_string()]);
        map.insert(".yaml".to_string(), vec!["YAML".to_string()]);
        map.insert(".yml".to_string(), vec!["YAML".to_string()]);
        map.insert(".toml".to_string(), vec!["TOML".to_string()]);
        map.insert(".xml".to_string(), vec!["XML".to_string()]);
        map.insert(".csv".to_string(), vec!["CSV".to_string()]);
        map.insert(".ini".to_string(), vec!["ConfigParser".to_string()]);
        map.insert(".cfg".to_string(), vec!["ConfigParser".to_string()]);
        
        // Binary formats
        map.insert(".bson".to_string(), vec!["BSON".to_string()]);
        map.insert(".msgpack".to_string(), vec!["MessagePack".to_string()]);
        map.insert(".mp".to_string(), vec!["MessagePack".to_string()]);
        map.insert(".cbor".to_string(), vec!["CBOR".to_string()]);
        map.insert(".pkl".to_string(), vec!["Pickle".to_string()]);
        map.insert(".pickle".to_string(), vec!["Pickle".to_string()]);
        map.insert(".p".to_string(), vec!["Pickle".to_string()]);
        map.insert(".marshal".to_string(), vec!["Marshal".to_string()]);
        map.insert(".db".to_string(), vec!["SQLite3".to_string()]);
        map.insert(".sqlite".to_string(), vec!["SQLite3".to_string()]);
        map.insert(".sqlite3".to_string(), vec!["SQLite3".to_string()]);
        map.insert(".dbm".to_string(), vec!["DBM".to_string()]);
        map.insert(".shelf".to_string(), vec!["Shelve".to_string()]);
        map.insert(".plist".to_string(), vec!["Plistlib".to_string()]);
        
        map
    }

    // Binary formats with clear magic bytes
    // Some MessagePack implementations
    // Avro object container
    /// Build mapping from magic bytes to format names.
    fn _build_magic_bytes_map() -> HashMap<Vec<u8>, Vec<String>> {
        let mut map = HashMap::new();
        map.insert(b"BSON".to_vec(), vec!["BSON".to_string()]);
        map.insert(b"SQLite format 3\x00".to_vec(), vec!["SQLite3".to_string()]);
        map.insert(b"bplist".to_vec(), vec!["Plistlib".to_string()]);
        map.insert(b"Obj\x01".to_vec(), vec!["Avro".to_string()]);
        map.insert(b"PAR1".to_vec(), vec!["Parquet".to_string()]);
        map.insert(b"ORC".to_vec(), vec!["ORC".to_string()]);
        map.insert(b"\xc0\x01\x00\x00".to_vec(), vec!["CapnProto".to_string()]);
        map.insert(b"FLATB".to_vec(), vec!["FlatBuffers".to_string()]);
        map
    }

    /// Build regex patterns for content-based detection with confidence scores.
    fn _build_content_patterns() -> HashMap<String, Vec<(String, f64)>> {
        let mut map = HashMap::new();
        // JSON patterns
        map.insert("JSON".to_string(), vec![
            ("{".to_string(), 0.9),
            ("[".to_string(), 0.9),
            ("\"".to_string(), 0.6),
        ]);
        // YAML patterns
        map.insert("YAML".to_string(), vec![
            (":".to_string(), 0.7),
            ("---".to_string(), 0.8),
        ]);
        // XML patterns
        map.insert("XML".to_string(), vec![
            ("<?xml".to_string(), 0.95),
            ("<".to_string(), 0.7),
        ]);
        // TOML patterns
        map.insert("TOML".to_string(), vec![
            ("=".to_string(), 0.6),
            ("[".to_string(), 0.7),
        ]);
        map
    }

    /// Detect format from file extension.
    ///
    ///
    /// Args:
    /// file_path: Path to analyze
    ///
    ///
    /// Returns:
    /// List of possible format names
    pub fn detect_from_extension(&self, file_path: String) -> Vec<String> {
        if let Some(ext) = Path::new(&file_path).extension() {
            if let Some(ext_str) = ext.to_str() {
                let ext_lower = format!(".{}", ext_str.to_lowercase());
                return self.extension_map.get(&ext_lower)
                    .cloned()
                    .unwrap_or_default();
            }
        }
        Vec::new()
    }

    /// Detect format from magic bytes at the beginning of data.
    ///
    ///
    /// Args:
    /// data: Binary data to analyze
    /// max_bytes: Maximum number of bytes to check
    ///
    ///
    /// Returns:
    /// List of possible format names
    pub fn detect_from_magic_bytes(&self, data: Vec<u8>, max_bytes: Option<i64>) -> Vec<String> {
        if data.is_empty() {
            return Vec::new();
        }
        
        let max = max_bytes.unwrap_or(64) as usize;
        let header = data.into_iter().take(max).collect::<Vec<u8>>();
        
        for (magic, formats) in &self.magic_bytes {
            if header.len() >= magic.len() && header[..magic.len()] == *magic {
                return formats.clone();
            }
        }
        
        Vec::new()
    }

    // Binary content, check magic bytes
    // Limit content size for performance
    // Normalize confidence based on number of patterns
    /// Detect format from content analysis with confidence scores.
    ///
    ///
    /// Args:
    /// content: Content to analyze (string or bytes)
    ///
    ///
    /// Returns:
    /// Dictionary mapping format names to confidence scores
    pub fn detect_from_content(&self, content: String) -> HashMap<String, f64> {
        let mut results = HashMap::new();
        
        // Limit content size for performance
        let text_content = if content.len() > 10000 {
            content.chars().take(10000).collect::<String>()
        } else {
            content
        };
        
        for (format_name, patterns) in &self.content_patterns {
            let mut confidence = 0.0;
            let mut matches = 0;
            
            for (pattern, weight) in patterns {
                if text_content.contains(pattern) {
                    confidence += weight;
                    matches += 1;
                }
            }
            
            // Normalize confidence based on number of patterns
            if matches > 0 {
                confidence = (confidence / patterns.len() as f64).min(1.0);
                results.insert(format_name.clone(), confidence);
            }
        }
        
        results
    }

    // Extension-based detection (high confidence)
    // Magic bytes detection (highest confidence)
    // Content-based detection
    /// Comprehensive format detection using all available methods.
    ///
    ///
    /// Args:
    /// file_path: Optional file path for extension-based detection
    /// content: Optional content for pattern-based detection
    /// data: Optional binary data for magic byte detection
    ///
    ///
    /// Returns:
    /// Dictionary mapping format names to confidence scores
    pub fn detect_format(&self, file_path: Option<String>, content: Option<String>, data: Option<Vec<u8>>) -> HashMap<String, f64> {
        let mut results = HashMap::new();
        
        // Extension-based detection (high confidence: 0.8)
        if let Some(ref path) = file_path {
            let extension_formats = self.detect_from_extension(path.clone());
            for fmt in extension_formats {
                *results.entry(fmt).or_insert(0.0) += 0.8;
            }
        }
        
        // Magic bytes detection (highest confidence: 0.95)
        if let Some(ref bytes) = data {
            let magic_formats = self.detect_from_magic_bytes(bytes.clone(), None);
            for fmt in magic_formats {
                *results.entry(fmt).or_insert(0.0) += 0.95;
            }
        }
        
        // Content-based detection
        if let Some(ref text) = content {
            let content_results = self.detect_from_content(text.clone());
            for (fmt, confidence) in content_results {
                *results.entry(fmt).or_insert(0.0) += confidence * 0.7;
            }
        }
        
        // Normalize scores
        let max_possible = 2.45; // 0.8 + 0.95 + 0.7
        for (_, confidence) in results.iter_mut() {
            *confidence = (*confidence / max_possible).min(1.0);
        }
        
        results
    }

    /// Get the most likely format with highest confidence.
    ///
    ///
    /// Args:
    /// file_path: Optional file path for extension-based detection
    /// content: Optional content for pattern-based detection
    /// data: Optional binary data for magic byte detection
    ///
    ///
    /// Returns:
    /// Format name with highest confidence, or None if below threshold
    pub fn get_best_format(&self, file_path: Option<String>, content: Option<String>, data: Option<Vec<u8>>) -> Option<String> {
        let suggestions = self.get_format_suggestions(file_path, content, data, Some(1));
        if let Some((format, confidence)) = suggestions.first() {
            if *confidence >= self.confidence_threshold {
                return Some(format.clone());
            }
        }
        None
    }

    // Sort by confidence descending
    /// Get ranked format suggestions with confidence scores.
    ///
    ///
    /// Args:
    /// file_path: Optional file path for extension-based detection
    /// content: Optional content for pattern-based detection
    /// data: Optional binary data for magic byte detection
    /// max_suggestions: Maximum number of suggestions to return
    ///
    ///
    /// Returns:
    /// List of (format_name, confidence) tuples sorted by confidence
    pub fn get_format_suggestions(&self, file_path: Option<String>, content: Option<String>, data: Option<Vec<u8>>, max_suggestions: Option<i64>) -> Vec<(String, f64)> {
        let mut results: HashMap<String, f64> = HashMap::new();
        
        // Extension-based detection (high confidence: 0.8)
        if let Some(ref path) = file_path {
            if let Some(ext) = Path::new(path).extension() {
                if let Some(ext_str) = ext.to_str() {
                    let ext_lower = format!(".{}", ext_str.to_lowercase());
                    if let Some(formats) = self.extension_map.get(&ext_lower) {
                        for format in formats {
                            results.insert(format.clone(), 0.8);
                        }
                    }
                }
            }
        }
        
        // Magic bytes detection (highest confidence: 0.95)
        if let Some(ref bytes) = data {
            for (magic, formats) in &self.magic_bytes {
                if bytes.len() >= magic.len() && bytes[..magic.len()] == *magic {
                    for format in formats {
                        results.insert(format.clone(), 0.95);
                    }
                }
            }
        }
        
        // Content-based detection (medium confidence: 0.6-0.9)
        if let Some(ref text) = content {
            for (format, patterns) in &self.content_patterns {
                for (pattern, confidence) in patterns {
                    // Simple pattern matching (TODO: implement proper regex)
                    if text.contains(pattern) {
                        let current = results.get(format).copied().unwrap_or(0.0);
                        results.insert(format.clone(), current.max(*confidence));
                    }
                }
            }
        }
        
        // Sort by confidence descending and limit
        let mut sorted: Vec<(String, f64)> = results.into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        if let Some(max) = max_suggestions {
            sorted.truncate(max as usize);
        }
        
        sorted
    }

    /// Check if a format is binary or text-based.
    ///
    ///
    /// Args:
    /// format_name: Format name to check
    ///
    ///
    /// Returns:
    /// True if binary format, False if text format
    pub fn is_binary_format(&self, format_name: String) -> bool {
        let format_upper = format_name.to_uppercase();
        matches!(
            format_upper.as_str(),
            "BSON" | "MESSAGEPACK" | "CBOR" | "PICKLE" | "MARSHAL" | "PLISTLIB" |
            "SQLITE3" | "DBM" | "SHELVE" | "AVRO" | "PROTOBUF" | "THRIFT" |
            "PARQUET" | "ORC" | "CAPNPROTO" | "FLATBUFFERS"
        )
    }

    /// Get the serializer class name for a detected format.
    ///
    ///
    /// Args:
    /// format_name: Detected format name
    ///
    ///
    /// Returns:
    /// Serializer class name
    pub fn get_serializer_class_name(&self, format_name: String) -> String {
        format!("{}Serializer", format_name)
    }

}

    /// Convenience function for format detection using global detector.
    ///
    ///
    /// Args:
    /// file_path: Optional file path for extension-based detection
    /// content: Optional content for pattern-based detection
    /// data: Optional binary data for magic byte detection
    ///
    ///
    /// Returns:
    /// Most likely format name or None
    pub fn detect_format(file_path: Option<String>, content: Option<String>, data: Option<Vec<u8>>) -> Option<String> {
        let detector = FormatDetector::new(None);
        detector.get_best_format(file_path, content, data)
    }

    /// Convenience function for format suggestions using global detector.
    ///
    ///
    /// Args:
    /// file_path: Optional file path for extension-based detection
    /// content: Optional content for pattern-based detection
    /// data: Optional binary data for magic byte detection
    /// max_suggestions: Maximum number of suggestions
    ///
    ///
    /// Returns:
    /// List of (format_name, confidence) tuples
    pub fn get_format_suggestions(file_path: Option<String>, content: Option<String>, data: Option<Vec<u8>>, max_suggestions: Option<i64>) -> Vec<(String, f64)> {
        let detector = FormatDetector::new(None);
        detector.get_format_suggestions(file_path, content, data, max_suggestions)
    }

    /// Convenience function to check if format is binary.
    ///
    ///
    /// Args:
    /// format_name: Format name to check
    ///
    ///
    /// Returns:
    /// True if binary format
    pub fn is_binary_format(format_name: &str) -> bool {
        let detector = FormatDetector::new(None);
        detector.is_binary_format(format_name.to_string())
    }
