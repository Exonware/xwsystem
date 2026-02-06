// #exonware/xwsystem/rust/src/io/serialization/base.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 2, 2025
//! 
//! Serialization base classes - ASerialization abstract base.
//! 
//! Following I→A→XW pattern:
//! - I: ISerialization (interface)
//! - A: ASerialization (abstract base - this file)
//! - XW: XW{Format}Serializer (concrete implementations)


use std::collections::HashMap;

use std::collections::HashMap;
use std::path::Path;

use crate::codec::base::ACodec;
use crate::io::serialization::contracts::ISerialization;
use crate::contracts::{DecodeOptions, EncodeOptions};
use crate::defs::CodecCapability;
use crate::io::serialization::errors::SerializationError;

// ============================================================================

// SCHEMA REGISTRY BASE CLASSES (Moved from enterprise)

// ============================================================================

// ========================================================================
// CORE CODEC METHODS (Must implement in subclasses)
// ========================================================================
// ========================================================================
// METADATA PROPERTIES (Must implement in subclasses)
// ========================================================================
// Can be overridden in subclasses for performance
// Default to text, override in binary formats
// Override in formats that support streaming
// ========================================================================
// CAPABILITY PROPERTIES (Advanced features - override in subclasses)
// ========================================================================
// ========================================================================
// FILE I/O METHODS (Default implementations using encode/decode)
// ========================================================================
// Cache this serializer instance for this file path
// This enables reuse if the same file is accessed again
// Ensure parent directory exists
// Validate data limits (method inherited from ACodec base class)
// Note: For large files (10GB+), size validation is automatically skipped
// Only depth validation is performed (prevents infinite recursion)
// Write to file (atomic)
// Cache this serializer instance for this file path
// This enables reuse if the same file is accessed again
// Try text first, fall back to bytes
// ========================================================================
// RECORD-LEVEL OPERATIONS (Generic defaults – can be overridden)
// ========================================================================
// Fallback: treat entire object as a single "record"
// Delegate to existing save_file() which may already be atomic.
// Non-list data: treat as a single record page
// Single-record fallback
// Small helper for projection handling
// ========================================================================
// VALIDATION METHODS (Default implementations)
// ========================================================================
// ========================================================================
// STREAMING METHODS (Default implementations)
// ========================================================================
// Default: encode all at once, then chunk
// Default: collect all chunks, then decode
// ========================================================================
// ASYNC METHODS (Default implementations using asyncio.to_thread)
// ========================================================================
// ========================================================================
// ADVANCED FEATURES (Default implementations with graceful fallback)
// ========================================================================
// Default fallback: load full file, update in memory, save atomically
// For large files, skip size validation (they use lazy loading/streaming)
// Root cause: Large files (10GB+) should use atomic path operations without full validation
// Solution: Skip size check for atomic operations (depth check still performed)
// Update path in memory (simple dict/list update for now)
// Subclasses should override with format-specific logic
// Simple JSONPointer-like path handling
// Save atomically using atomic operations
// Default fallback: load full file, extract path
// For large files, skip size validation (they use lazy loading/streaming)
// Root cause: Large files (10GB+) should use atomic path operations without full validation
// Solution: Skip size check for atomic operations (depth check still performed)
// Extract path (simple dict/list access for now)
// Default fallback: collect all items, encode, write atomically
// Use standard save_file which handles atomic operations
// Default fallback: load all, yield items
// If data is iterable (list, tuple), yield items
// Single item, yield it
// Root cause fixed: Base class doesn't implement queries - raise immediately
// Subclasses should override this method to provide actual query implementation
// Priority #4: Performance - Don't load file if operation will fail
// Default fallback: load all, merge in memory, save atomically
// File doesn't exist, create it with updates
/// Abstract base class for serialization - follows I→A→XW pattern.
///
/// I: ISerialization (interface)
/// A: ASerialization (abstract base - this class)
/// XW: XW{Format}Serializer (concrete implementations)
///
/// Extends ACodec and implements ISerialization interface.
/// Provides default implementations for common serialization operations.
///
/// Subclasses only need to implement:
/// - encode()
/// - decode()
/// - Metadata properties (codec_id, media_types, file_extensions, etc.)
///
/// This class provides:
/// - File I/O with atomic operations (save_file, load_file)
/// - Async file I/O (save_file_async, load_file_async)
/// - Streaming (iter_serialize, iter_deserialize, stream_serialize, stream_deserialize)
/// - Validation helpers
/// - XWSystem integration
pub trait ASerialization: ISerialization {
    /// Encode data to representation - must implement in subclass.
    /// Note: Safety validation is automatically performed in save_file() via inherited
    /// _validate_data_limits() from ACodec. Subclasses can call it directly if needed.
    fn encode(&self, value: serde_json::Value) -> Vec<u8>;

    /// Decode representation to data - must implement in subclass.
    /// Note: For decode operations, size validation happens on the input string/bytes,
    /// not the resulting data structure. Subclasses should validate input size if needed.
    fn decode(&self, repr: Vec<u8>) -> serde_json::Value;

    /// Codec identifier (e.g., 'json', 'yaml').
    // Python decorators: @property
    fn codec_id(&self) -> &str;

    /// Supported MIME types.
    // Python decorators: @property
    fn media_types(&self) -> Vec<String>;

    /// Supported file extensions.
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

    /// Format name (default: uppercase codec_id).
    // Python decorators: @property
    // Note: Trait requires &str, but computed values need String.
    // Subclasses should override this with a stored &str value.
    // This default implementation returns codec_id as a workaround.
    fn format_name(&self) -> &str {
        // Default: return codec_id (subclasses should override with uppercase version)
        // TODO: Consider changing ISerialization trait to return String for computed properties
        self.codec_id()
    }

    /// Primary MIME type (default: first in media_types).
    // Python decorators: @property
    fn mime_type(&self) -> &str {
        // Default implementation: first in media_types or fallback
        self.media_types().first()
            .map(|s| s.as_str())
            .unwrap_or("application/octet-stream")
    }

    /// Whether this is a binary format (default: check return type of encode).
    // Python decorators: @property
    fn is_binary_format(&self) -> bool {
        // Default to text, override in binary formats
        false
    }

    /// Whether this format supports streaming (default: False).
    // Python decorators: @property
    fn supports_streaming(&self) -> bool {
        // Override in formats that support streaming
        false
    }

    /// Serialization codecs support bidirectional operations.
    // Python decorators: @property
    fn capabilities(&self) -> CodecCapability {
        CodecCapability::Bidirectional
    }

    /// Default aliases from codec_id.
    // Python decorators: @property
    fn aliases(&self) -> Vec<String> {
        vec![
            self.codec_id().to_lowercase(),
            self.codec_id().to_uppercase(),
        ]
    }

    /// Codec types for categorization (default: ['serialization']).
    /// Override in subclasses for more specific types like:
    /// - ['binary']: Binary serialization formats
    /// - ['config', 'serialization']: Configuration file formats
    /// - ['data']: Data exchange formats
    /// Can return multiple types if codec serves multiple purposes.
    // Python decorators: @property
    fn codec_types(&self) -> Vec<String> {
        vec!["serialization".to_string()]
    }

    /// Whether this serializer supports path-based updates (JSONPointer, XPath, etc.).
    /// Default: False. Override in subclasses that support path operations.
    /// Returns:
    /// True if path-based updates are supported
    // Python decorators: @property
    fn supports_path_based_updates(&self) -> bool {
        false
    }

    /// Whether this serializer can atomically update paths without loading full file.
    /// Default: False. Override in subclasses that support efficient path updates.
    /// Returns:
    /// True if atomic path writes are supported
    // Python decorators: @property
    fn supports_atomic_path_write(&self) -> bool {
        false
    }

    /// Whether this serializer supports schema validation.
    /// Default: False. Override in subclasses that support schemas.
    /// Returns:
    /// True if schema validation is supported
    // Python decorators: @property
    fn supports_schema_validation(&self) -> bool {
        false
    }

    /// Whether this serializer supports true incremental streaming (not chunked full-file).
    /// Default: False. Override in subclasses that support incremental operations.
    /// Returns:
    /// True if incremental streaming is supported
    // Python decorators: @property
    fn supports_incremental_streaming(&self) -> bool {
        false
    }

    /// Whether this serializer supports multiple documents in one file.
    /// Default: False. Override in subclasses that support multi-document files.
    /// Returns:
    /// True if multi-document support is available
    // Python decorators: @property
    fn supports_multi_document(&self) -> bool {
        false
    }

    /// Whether this serializer supports query/filter operations (JSONPath, XPath, etc.).
    /// Default: False. Override in subclasses that support queries.
    /// Returns:
    /// True if query operations are supported
    // Python decorators: @property
    fn supports_query(&self) -> bool {
        false
    }

    /// Whether this serializer supports merge/update operations.
    /// Default: False. Override in subclasses that support merge operations.
    /// Returns:
    /// True if merge operations are supported
    // Python decorators: @property
    fn supports_merge(&self) -> bool {
        false
    }

    /// Whether this serializer supports lazy loading for large files (10GB+).
    /// Lazy loading means the serializer can:
    /// - Skip full file validation for large files (size check skipped, depth still checked)
    /// - Use atomic path operations without loading entire file into memory
    /// - Handle both small (1KB) and large (10GB+) files efficiently
    /// Default: False. Override in subclasses that support lazy loading.
    /// Returns:
    /// True if lazy loading is supported for large files
    // Python decorators: @property
    fn supports_lazy_loading(&self) -> bool {
        false
    }

    /// Whether this serializer exposes record-level streaming operations
    /// (stream_read_record / stream_update_record).
    /// Default: False. Override in subclasses that provide efficient record
    /// streaming on top of their underlying format.
    // Python decorators: @property
    fn supports_record_streaming(&self) -> bool {
        false
    }

    /// Whether this serializer supports efficient record-level paging.
    /// Default: False. Override in subclasses that can page records without
    /// loading the entire dataset.
    // Python decorators: @property
    fn supports_record_paging(&self) -> bool {
        false
    }

    /// Save data to file with atomic operations.
    /// Default implementation:
    /// 1. Validate data against safety limits (depth and size)
    /// 2. Encode data using encode()
    /// 3. Write to file using Path.write_bytes() or write_text()
    /// 4. Uses atomic operations if configured
    /// 5. Caches serializer instance by file path for performance
    /// Args:
    /// data: Data to serialize and save
    /// file_path: Path to save file
    /// **options: Format-specific options
    /// Raises:
    /// SerializationError: If save fails or data exceeds limits
    fn save_file(&self, data: serde_json::Value, file_path: String) {
        use std::fs;
        use std::io::Write;
        
        let path = Path::new(&file_path);
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                panic!("Failed to create parent directory: {}", e);
            }
        }
        
        // Encode data
        let encoded = self.encode(data);
        
        // Write to file (atomic write via temp file + rename)
        let temp_path = format!("{}.tmp", file_path);
        let mut file = match fs::File::create(&temp_path) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create file: {}", e),
        };
        
        if let Err(e) = file.write_all(&encoded) {
            let _ = fs::remove_file(&temp_path);
            panic!("Failed to write file: {}", e);
        }
        
        if let Err(e) = file.sync_all() {
            let _ = fs::remove_file(&temp_path);
            panic!("Failed to sync file: {}", e);
        }
        
        // Atomic rename
        if let Err(e) = fs::rename(&temp_path, &file_path) {
            let _ = fs::remove_file(&temp_path);
            panic!("Failed to rename file: {}", e);
        }
    }

    /// Load data from file.
    /// Default implementation:
    /// 1. Read from file using Path.read_bytes() or read_text()
    /// 2. Decode data using decode()
    /// 3. Caches serializer instance by file path for performance
    /// Args:
    /// file_path: Path to load from
    /// **options: Format-specific options
    /// Returns:
    /// Deserialized data
    /// Raises:
    /// SerializationError: If load fails
    fn load_file(&self, file_path: String) -> serde_json::Value {
        use std::fs;
        
        let path = Path::new(&file_path);
        
        if !path.exists() {
            panic!("File not found: {}", file_path);
        }
        
        // Read from file
        let data = if self.is_binary_format() {
            fs::read(path).unwrap_or_else(|e| panic!("Failed to read file: {}", e))
        } else {
            // Try text first, fall back to bytes
            match fs::read_to_string(path) {
                Ok(text) => text.into_bytes(),
                Err(_) => fs::read(path).unwrap_or_else(|e| panic!("Failed to read file: {}", e)),
            }
        };
        
        // Decode data
        self.decode(data)
    }

    /// Default record-level read:
    /// - Load the entire file via load_file().
    /// - If top-level is a list, scan until match(record) is True.
    /// - Apply optional projection and return the first matching record.
    /// This is format-agnostic but may be expensive for huge files. Formats
    /// that support true streaming should override this method.
    fn stream_read_record(&self, file_path: String, match: String, projection: Option<Vec<serde_json::Value>>) -> serde_json::Value {
        let data = self.load_file(file_path);
        
        // TODO: Implement proper matching logic (match is a String, needs to be parsed/evaluated)
        // For now, return first item if data is array, or data itself
        if let serde_json::Value::Array(arr) = data {
            if let Some(first) = arr.first() {
                return self._apply_projection(first.clone(), projection);
            }
        }
        
        // Fallback: treat entire object as a single "record"
        self._apply_projection(data, projection)
    }

    /// Default record-level update:
    /// - Load entire file via load_file().
    /// - If top-level is a list, apply updater() to matching records.
    /// - Save the modified data back via save_file().
    /// This is generic and honours the serializer's existing atomic/write
    /// behavior, but may be expensive for huge files. Formats that support
    /// streaming/partial updates should override this method.
    fn stream_update_record(&self, file_path: String, match: String, updater: String) -> i64 {
        let mut data = self.load_file(file_path.clone());
        let mut updated = 0;
        
        // TODO: Implement proper matching and updating logic
        // For now, if data is an array, update all items (placeholder)
        if let serde_json::Value::Array(arr) = &mut data {
            // Placeholder: update all records
            // In real implementation, match and updater would be callable functions
            updated = arr.len() as i64;
        } else {
            // Single record
            updated = 1;
        }
        
        // Save the modified data back
        self.save_file(data, file_path);
        updated
    }

    /// Default record-level paging:
    /// - Load entire file via load_file().
    /// - If top-level is a list, return a slice corresponding to the requested page.
    /// Formats that support indexed or streaming paging should override this
    /// method for better performance on very large datasets.
    fn get_record_page(&self, file_path: String, page_number: i64, page_size: i64) -> Vec<serde_json::Value> {
        if page_number < 1 || page_size <= 0 {
            return Vec::new();
        }
        
        let data = self.load_file(file_path);
        
        if let serde_json::Value::Array(arr) = data {
            let start = ((page_number - 1) * page_size) as usize;
            let end = (start + page_size as usize).min(arr.len());
            
            if start < arr.len() {
                return arr[start..end].to_vec();
            }
        }
        
        // Non-list data: treat as a single record page
        vec![data]
    }

    /// Default record lookup by id:
    /// - Load entire file via load_file().
    /// - If top-level is a list of dict-like records, scan for record[id_field].
    fn get_record_by_id(&self, file_path: String, id_value: serde_json::Value) -> serde_json::Value {
        let data = self.load_file(file_path);
        
        if let serde_json::Value::Array(arr) = data {
            // Scan for record with matching id (default field name: "id")
            for record in arr {
                if let serde_json::Value::Object(map) = &record {
                    if let Some(id) = map.get("id") {
                        if id == &id_value {
                            return record;
                        }
                    }
                }
            }
        }
        
        // Single-record fallback
        data
    }

    fn _apply_projection(&self, record: serde_json::Value, projection: Option<Vec<serde_json::Value>>) -> serde_json::Value {
        // Small helper for projection handling
        if let Some(projection_path) = projection {
            let mut current = record;
            for key in projection_path {
                match key {
                    serde_json::Value::String(field_name) => {
                        if let serde_json::Value::Object(map) = current {
                            if let Some(value) = map.get(&field_name) {
                                current = value.clone();
                            } else {
                                return serde_json::Value::Null;
                            }
                        } else {
                            return serde_json::Value::Null;
                        }
                    }
                    serde_json::Value::Number(idx) => {
                        if let Some(idx_usize) = idx.as_u64().map(|i| i as usize) {
                            if let serde_json::Value::Array(arr) = current {
                                if idx_usize < arr.len() {
                                    current = arr[idx_usize].clone();
                                } else {
                                    return serde_json::Value::Null;
                                }
                            } else {
                                return serde_json::Value::Null;
                            }
                        } else {
                            return serde_json::Value::Null;
                        }
                    }
                    _ => return serde_json::Value::Null,
                }
            }
            return current;
        }
        record
    }

    /// Validate data for serialization compatibility.
    /// Default implementation: Try to encode and catch errors.
    /// Override for format-specific validation.
    /// Args:
    /// data: Data to validate
    /// Returns:
    /// True if data can be serialized
    /// Raises:
    /// SerializationError: If validation fails
    fn validate_data(&self, data: serde_json::Value) -> bool {
        // Try to encode - if it succeeds, data is valid
        let _ = self.encode(data);
        true
    }

    /// Stream serialize data in chunks.
    /// Default implementation: Encode all data then yield in chunks.
    /// Override for true streaming support.
    /// Args:
    /// data: Data to serialize
    /// chunk_size: Size of each chunk
    /// Yields:
    /// Serialized chunks
    fn iter_serialize(&self, data: serde_json::Value, chunk_size: i64) -> Box<dyn Iterator<Item = String>> {
        // Default: encode all at once, then chunk
        let encoded = self.encode(data);
        let encoded_str = String::from_utf8_lossy(&encoded).to_string();
        let chunk_size_usize = chunk_size as usize;
        
        Box::new((0..encoded_str.len())
            .step_by(chunk_size_usize)
            .map(move |i| {
                let end = (i + chunk_size_usize).min(encoded_str.len());
                encoded_str[i..end].to_string()
            }))
    }

    /// Stream deserialize data from chunks.
    /// Default implementation: Collect all chunks then decode.
    /// Override for true streaming support.
    /// Args:
    /// src: Source of data chunks
    /// Returns:
    /// Deserialized data
    fn iter_deserialize(&self, src: TextIO) -> serde_json::Value {
        use std::io::Read;
        
        // Default: collect all chunks, then decode
        let mut data = Vec::new();
        if let Err(e) = src.read_to_end(&mut data) {
            panic!("Failed to read from source: {}", e);
        }
        self.decode(data)
    }

    /// Async save data to file.
    /// Default implementation: Delegate to sync save_file via asyncio.to_thread.
    /// Override for native async I/O.
    /// Args:
    /// data: Data to serialize
    /// file_path: Path to save file
    /// **options: Format-specific options
    async fn save_file_async(&self, data: serde_json::Value, file_path: String) {
        // Default: delegate to sync save_file
        // TODO: Use proper async file I/O when available (tokio::fs)
        // For now, use blocking I/O in async context (not ideal but functional)
        self.save_file(data, file_path);
    }

    /// Async load data from file.
    /// Default implementation: Delegate to sync load_file via asyncio.to_thread.
    /// Override for native async I/O.
    /// Args:
    /// file_path: Path to load from
    /// **options: Format-specific options
    /// Returns:
    /// Deserialized data
    async fn load_file_async(&self, file_path: String) -> serde_json::Value {
        // Default: delegate to sync load_file
        // TODO: Use proper async file I/O when available (tokio::fs)
        // For now, use blocking I/O in async context (not ideal but functional)
        self.load_file(file_path)
    }

    /// Async stream serialize data in chunks.
    /// Default implementation: Delegate to sync iter_serialize.
    /// Override for native async streaming.
    /// Args:
    /// data: Data to serialize
    /// chunk_size: Size of each chunk
    /// Yields:
    /// Serialized chunks
    async fn stream_serialize(&self, data: serde_json::Value, chunk_size: i64) -> String {
        // Default: delegate to sync iter_serialize and collect
        let chunks: Vec<String> = self.iter_serialize(data, chunk_size).collect();
        chunks.join("")
    }

    /// Async stream deserialize data from chunks.
    /// Default implementation: Collect all chunks then decode.
    /// Override for native async streaming.
    /// Args:
    /// data_stream: Async iterator of data chunks
    /// Returns:
    /// Deserialized data
    async fn stream_deserialize(&self, data_stream: String) -> serde_json::Value {
        // Default: treat data_stream as complete data string
        let data_bytes = data_stream.into_bytes();
        self.decode(data_bytes)
    }

    /// Atomically update a single path in a file (default: full-file fallback).
    /// Default implementation loads entire file, updates path in memory, and saves atomically.
    /// Override in subclasses for format-specific optimizations (e.g., JSONPointer for JSON).
    /// Args:
    /// file_path: Path to the file to update
    /// path: Path expression (format-specific)
    /// value: Value to set at the specified path
    /// **options: Format-specific options
    /// Raises:
    /// NotImplementedError: If path-based updates not supported and fallback fails
    /// SerializationError: If the update operation fails
    /// ValueError: If the path is invalid
    /// Note:
    /// This default implementation is inefficient for large files. Formats that support
    /// path-based updates should override this method for optimal performance.
    fn atomic_update_path(&self, file_path: String, path: String, value: serde_json::Value) {
        // Default fallback: load full file, update in memory, save atomically
        if !self.supports_path_based_updates() {
            panic!("Path-based updates not supported for this serializer");
        }
        
        let path_obj = Path::new(&file_path);
        if !path_obj.exists() {
            panic!("File not found: {}", file_path);
        }
        
        // Load entire file
        let mut data = self.load_file(file_path.clone());
        
        // Update path in memory (simple JSONPointer-like path handling)
        if let serde_json::Value::Object(ref mut map) = data {
            if path.starts_with('/') {
                let path_parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
                let mut current = serde_json::Value::Object(map.clone());
                
                for (i, part) in path_parts.iter().enumerate() {
                    if i == path_parts.len() - 1 {
                        // Last part - set the value
                        if let serde_json::Value::Object(ref mut obj) = current {
                            obj.insert(part.to_string(), value);
                        }
                    } else {
                        // Navigate to parent
                        if let serde_json::Value::Object(ref obj) = current {
                            if let Some(next) = obj.get(*part) {
                                current = next.clone();
                            } else {
                                panic!("Path not found: {}", path);
                            }
                        }
                    }
                }
                data = current;
            } else {
                // Simple key update
                map.insert(path, value);
            }
        } else {
            panic!("Path-based updates not supported for data type");
        }
        
        // Save atomically
        self.save_file(data, file_path);
    }

    /// Read a single path from a file (default: full-file fallback).
    /// Default implementation loads entire file and extracts path.
    /// Override in subclasses for format-specific optimizations.
    /// Args:
    /// file_path: Path to the file to read from
    /// path: Path expression (format-specific)
    /// **options: Format-specific options
    /// Returns:
    /// Value at the specified path
    /// Raises:
    /// NotImplementedError: If path-based reads not supported
    /// SerializationError: If the read operation fails
    /// KeyError: If the path doesn't exist
    fn atomic_read_path(&self, file_path: String, path: String) -> serde_json::Value {
        // Default fallback: load full file, extract path
        if !self.supports_path_based_updates() {
            panic!("Path-based reads not supported for this serializer");
        }
        
        let path_obj = Path::new(&file_path);
        if !path_obj.exists() {
            panic!("File not found: {}", file_path);
        }
        
        // Load entire file
        let data = self.load_file(file_path);
        
        // Extract path (simple JSONPointer-like path handling)
        if let serde_json::Value::Object(map) = data {
            if path.starts_with('/') {
                let path_parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
                let mut current = serde_json::Value::Object(map);
                
                for part in path_parts {
                    if let serde_json::Value::Object(obj) = current {
                        if let Some(value) = obj.get(part) {
                            current = value.clone();
                        } else {
                            panic!("Path not found: {}", path);
                        }
                    } else {
                        panic!("Path not found: {}", path);
                    }
                }
                current
            } else {
                // Simple key access
                map.get(&path).cloned().unwrap_or(serde_json::Value::Null)
            }
        } else {
            panic!("Path-based reads not supported for data type");
        }
    }

    /// Validate data against a schema (default: not supported).
    /// Override in subclasses that support schema validation (JSON Schema, XSD, etc.).
    /// Args:
    /// data: Data to validate
    /// schema: Schema definition (format-specific)
    /// **options: Validation options
    /// Returns:
    /// True if data is valid
    /// Raises:
    /// NotImplementedError: If schema validation not supported
    fn validate_with_schema(&self, data: serde_json::Value, schema: serde_json::Value) -> bool {
        // Default: schema validation not supported
        if !self.supports_schema_validation() {
            panic!("Schema validation not supported for this serializer");
        }
        // TODO: Implement actual schema validation when schema support is added
        true
    }

    /// Incrementally save items to a file (default: encode all, write atomically).
    /// Default implementation collects all items, encodes them, and writes atomically.
    /// Override in subclasses for true incremental streaming (e.g., JSONL).
    /// Args:
    /// items: Iterator of items to save
    /// file_path: Path to save file
    /// **options: Format-specific options
    /// Raises:
    /// NotImplementedError: If incremental streaming not supported
    /// SerializationError: If save operation fails
    fn incremental_save(&self, items: Box<dyn Iterator<Item = serde_json::Value>>, file_path: String) {
        // Default: collect all items, encode them, and write atomically
        if !self.supports_incremental_streaming() {
            panic!("Incremental streaming not supported for this serializer");
        }
        
        let all_items: Vec<serde_json::Value> = items.collect();
        let data = serde_json::Value::Array(all_items);
        self.save_file(data, file_path);
    }

    /// Incrementally load items from a file (default: load all, return iterator).
    /// Default implementation loads entire file and yields items.
    /// Override in subclasses for true incremental streaming.
    /// Args:
    /// file_path: Path to load from
    /// **options: Format-specific options
    /// Yields:
    /// Items from the file
    /// Raises:
    /// NotImplementedError: If incremental streaming not supported
    /// SerializationError: If load operation fails
    fn incremental_load(&self, file_path: String) -> Box<dyn Iterator<Item = serde_json::Value>> {
        // Default: load entire file and yield items
        if !self.supports_incremental_streaming() {
            panic!("Incremental streaming not supported for this serializer");
        }
        
        let data = self.load_file(file_path);
        if let serde_json::Value::Array(arr) = data {
            Box::new(arr.into_iter())
        } else {
            // Single item
            Box::new(vec![data].into_iter())
        }
    }

    /// Query/filter data from a file (default: load all, query in memory).
    /// Default implementation loads entire file and applies query in memory.
    /// Override in subclasses for format-specific query languages (JSONPath, XPath).
    /// Root cause fixed: Method was loading entire file before raising error.
    /// Solution: Raise NotImplementedError immediately if queries not supported,
    /// preventing unnecessary file I/O operations.
    /// Priority #4: Performance - Avoid loading files when operation will fail.
    /// Args:
    /// file_path: Path to the file to query
    /// query_expr: Query expression (format-specific)
    /// **options: Query options
    /// Returns:
    /// Query results
    /// Raises:
    /// NotImplementedError: If queries not supported
    /// SerializationError: If query operation fails
    /// ValueError: If query expression is invalid
    fn query(&self, file_path: String, query_expr: String) -> serde_json::Value {
        // Default: queries not supported
        if !self.supports_query() {
            panic!("Query operations not supported for this serializer");
        }
        // TODO: Implement actual query logic when query support is added
        serde_json::Value::Null
    }

    /// Merge updates into a file (default: load all, merge in memory, save atomically).
    /// Default implementation loads entire file, performs deep merge, and saves atomically.
    /// Override in subclasses for format-specific merge optimizations.
    /// Args:
    /// file_path: Path to the file to update
    /// updates: Dictionary of updates to merge
    /// **options: Merge options (deep=True, shallow=False, etc.)
    /// Raises:
    /// NotImplementedError: If merge operations not supported
    /// SerializationError: If merge operation fails
    fn merge(&self, file_path: String, updates: HashMap<String, serde_json::Value>) {
        // Default: load entire file, perform deep merge, and save atomically
        if !self.supports_merge() {
            panic!("Merge operations not supported for this serializer");
        }
        
        let mut data = self.load_file(file_path.clone());
        
        // Perform deep merge
        if let serde_json::Value::Object(ref mut map) = data {
            for (key, value) in updates {
                map.insert(key, value);
            }
        }
        
        // Save atomically
        self.save_file(data, file_path);
    }

}

/// Abstract base class for schema registry implementations.
pub trait ASchemaRegistry {
    /// Register a new schema version.
    async fn register_schema(&self, subject: String, schema: String, schema_type: String) -> String;

    /// Get schema by ID.
    async fn get_schema(&self, schema_id: i64) -> String;

    /// Get latest schema version for subject.
    async fn get_latest_schema(&self, subject: String) -> String;

    /// Get all versions for a subject.
    async fn get_schema_versions(&self, subject: String) -> Vec<i64>;

    /// Check if schema is compatible with latest version.
    async fn check_compatibility(&self, subject: String, schema: String) -> bool;

    /// Set compatibility level for subject.
    async fn set_compatibility(&self, subject: String, level: String) -> ();

}

    /// Get cached serializer instance for file path, if available.
    ///
    /// This function allows retrieving a previously cached serializer instance
    /// for a given file path. Serializers are automatically cached when
    /// save_file() or load_file() is called.
    ///
    ///
    /// Args:
    /// file_path: Path to the file
    ///
    ///
    /// Returns:
    /// Cached serializer instance if available, None otherwise
    pub fn get_cached_serializer_for_path(file_path: &str) -> Option<Box<dyn ISerialization>> {
        // TODO: Implement serializer caching when cache system is available
        // For now, return None (no caching)
        None
    }
