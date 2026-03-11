// #exonware/xwsystem/rust/src/io/serialization/contracts.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 2, 2025
//! 
//! Serialization contracts - ISerialization interface extending ICodec.
//! 
//! Following I→A→XW pattern:
//! - I: ISerialization (interface - this file)
//! - A: ASerialization (abstract base)
//! - XW: XW{Format}Serializer (concrete implementations)


use std::collections::HashMap;

use std::collections::HashMap;
use std::path::Path;

use crate::codec::contracts::{ICodec, ICodecMetadata};
use crate::contracts::{DecodeOptions, EncodeOptions, TextIO};
use crate::defs::CodecCapability;

// ========================================================================
// METADATA PROPERTIES (from ICodecMetadata)
// ========================================================================
// ========================================================================
// CORE CODEC METHODS (from ICodec)
// ========================================================================
// ========================================================================
// FILE I/O METHODS (Serialization-specific)
// ========================================================================
// ========================================================================
// ========================================================================
// ========================================================================
// ========================================================================
// ========================================================================
// ========================================================================
// ========================================================================
// ADVANCED FEATURES (Optional - format-specific implementations)
// ========================================================================
// ========================================================================
// RECORD-LEVEL OPERATIONS (Optional, generic defaults in ASerialization)
// ========================================================================
/// Serialization interface extending ICodec.
///
/// Provides serialization-specific functionality on top of the universal codec interface.
///
/// Type: ICodec[Any, Union[bytes, str]]
/// - T (model type): Any (supports any Python object)
/// - R (representation): Union[bytes, str] (can be text or binary)
///
/// This interface extends ICodec with:
/// - File I/O methods (save_file, load_file)
/// - Format detection
/// - Validation
/// - Streaming support
/// - Async operations
///
/// All serializers in xwsystem implement this interface.
pub trait ISerialization {
    /// Get the serialization format name (e.g., 'JSON', 'YAML').
    // Python decorators: @property
    fn format_name(&self) -> &str;

    /// Get supported file extensions for this format.
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

    /// Get the MIME type for this serialization format.
    // Python decorators: @property
    fn mime_type(&self) -> &str;

    /// Whether this is a binary or text-based format.
    // Python decorators: @property
    fn is_binary_format(&self) -> bool;

    /// Whether this format supports streaming serialization.
    // Python decorators: @property
    fn supports_streaming(&self) -> bool;

    /// Encode data to representation (bytes or str).
    /// Core codec method - all serializers must implement.
    fn encode(&self, value: serde_json::Value) -> Vec<u8>;

    /// Decode representation (bytes or str) to data.
    /// Core codec method - all serializers must implement.
    fn decode(&self, repr: Vec<u8>) -> serde_json::Value;

    /// Save data to file with atomic operations.
    /// Args:
    /// data: Data to serialize and save
    /// file_path: Path to save file
    /// **options: Format-specific options
    /// Raises:
    /// SerializationError: If save fails
    fn save_file(&self, data: serde_json::Value, file_path: String) -> ();

    /// Load data from file.
    /// Args:
    /// file_path: Path to load from
    /// **options: Format-specific options
    /// Returns:
    /// Deserialized data
    /// Raises:
    /// SerializationError: If load fails
    fn load_file(&self, file_path: String) -> serde_json::Value;

    /// Validate data for serialization compatibility.
    /// Args:
    /// data: Data to validate
    /// Returns:
    /// True if data can be serialized
    /// Raises:
    /// SerializationError: If validation fails
    fn validate_data(&self, data: serde_json::Value) -> bool;

    /// Stream serialize data in chunks.
    /// Args:
    /// data: Data to serialize
    /// chunk_size: Size of each chunk
    /// Yields:
    /// Serialized chunks
    fn iter_serialize(&self, data: serde_json::Value, chunk_size: i64) -> Box<dyn Iterator<Item = String>>;

    /// Stream deserialize data from chunks.
    /// Args:
    /// src: Source of data chunks
    /// Returns:
    /// Deserialized data
    fn iter_deserialize(&self, src: TextIO) -> serde_json::Value;

    /// Async save data to file.
    /// Args:
    /// data: Data to serialize
    /// file_path: Path to save file
    /// **options: Format-specific options
    async fn save_file_async(&self, data: serde_json::Value, file_path: String) -> ();

    /// Async load data from file.
    /// Args:
    /// file_path: Path to load from
    /// **options: Format-specific options
    /// Returns:
    /// Deserialized data
    async fn load_file_async(&self, file_path: String) -> serde_json::Value;

    /// Async stream serialize data in chunks.
    /// Args:
    /// data: Data to serialize
    /// chunk_size: Size of each chunk
    /// Yields:
    /// Serialized chunks
    async fn stream_serialize(&self, data: serde_json::Value, chunk_size: i64) -> String;

    /// Async stream deserialize data from chunks.
    /// Args:
    /// data_stream: Async iterator of data chunks
    /// Returns:
    /// Deserialized data
    async fn stream_deserialize(&self, data_stream: String) -> serde_json::Value;

    /// Atomically update a single path in a file without loading the entire file.
    /// This method allows efficient updates to large files by only modifying
    /// the specific path (e.g., JSONPointer "/users/0/name") without loading
    /// the entire file into memory.
    /// Args:
    /// file_path: Path to the file to update
    /// path: Path expression (format-specific: JSONPointer, XPath, YAML path, etc.)
    /// value: Value to set at the specified path
    /// **options: Format-specific options (backup, atomic, etc.)
    /// Raises:
    /// NotImplementedError: If this format doesn't support path-based updates
    /// SerializationError: If the update operation fails
    /// ValueError: If the path is invalid or doesn't exist
    /// Example:
    /// >>> serializer.atomic_update_path("config.json", "/database/host", "localhost")
    fn atomic_update_path(&self, file_path: String, path: String, value: serde_json::Value) -> ();

    /// Read a single path from a file without loading the entire file.
    /// This method allows efficient reads from large files by only accessing
    /// the specific path (e.g., JSONPointer "/users/0/name") without loading
    /// the entire file into memory.
    /// Args:
    /// file_path: Path to the file to read from
    /// path: Path expression (format-specific: JSONPointer, XPath, YAML path, etc.)
    /// **options: Format-specific options
    /// Returns:
    /// Value at the specified path
    /// Raises:
    /// NotImplementedError: If this format doesn't support path-based reads
    /// SerializationError: If the read operation fails
    /// ValueError: If the path is invalid or doesn't exist
    /// KeyError: If the path doesn't exist in the file
    /// Example:
    /// >>> host = serializer.atomic_read_path("config.json", "/database/host")
    fn atomic_read_path(&self, file_path: String, path: String) -> serde_json::Value;

    /// Validate data against a schema.
    /// Args:
    /// data: Data to validate
    /// schema: Schema definition (format-specific)
    /// **options: Validation options
    /// Returns:
    /// True if data is valid
    /// Raises:
    /// NotImplementedError: If this format doesn't support schema validation
    /// SerializationError: If validation fails
    fn validate_with_schema(&self, data: serde_json::Value, schema: serde_json::Value) -> bool;

    /// Incrementally save items to a file using true streaming (not chunked full-file).
    /// This method writes items one at a time as they're provided, enabling
    /// memory-efficient handling of large datasets.
    /// Args:
    /// items: Iterator of items to save
    /// file_path: Path to save file
    /// **options: Format-specific options
    /// Raises:
    /// NotImplementedError: If this format doesn't support incremental streaming
    /// SerializationError: If save operation fails
    fn incremental_save(&self, items: Box<dyn Iterator<Item = serde_json::Value>>, file_path: String) -> ();

    /// Incrementally load items from a file using true streaming.
    /// This method reads items one at a time as they're needed, enabling
    /// memory-efficient handling of large files.
    /// Args:
    /// file_path: Path to load from
    /// **options: Format-specific options
    /// Yields:
    /// Items from the file one at a time
    /// Raises:
    /// NotImplementedError: If this format doesn't support incremental streaming
    /// SerializationError: If load operation fails
    fn incremental_load(&self, file_path: String) -> Box<dyn Iterator<Item = serde_json::Value>>;

    /// Query/filter data from a file using format-specific query language.
    /// Supports query languages like JSONPath for JSON, XPath for XML, etc.
    /// Args:
    /// file_path: Path to the file to query
    /// query_expr: Query expression (format-specific: JSONPath, XPath, etc.)
    /// **options: Query options
    /// Returns:
    /// Query results (format-specific)
    /// Raises:
    /// NotImplementedError: If this format doesn't support queries
    /// SerializationError: If query operation fails
    /// ValueError: If query expression is invalid
    fn query(&self, file_path: String, query_expr: String) -> serde_json::Value;

    /// Merge updates into a file.
    /// Performs deep or shallow merge depending on format capabilities.
    /// Args:
    /// file_path: Path to the file to update
    /// updates: Dictionary of updates to merge
    /// **options: Merge options (deep, shallow, etc.)
    /// Raises:
    /// NotImplementedError: If this format doesn't support merge operations
    /// SerializationError: If merge operation fails
    fn merge(&self, file_path: String, updates: HashMap<String, serde_json::Value>) -> ();

    /// Stream-style read of a single logical record from a file.
    /// Semantics:
    /// - Treat the underlying representation as a sequence of logical records
    /// (e.g., list elements, table rows, NDJSON records).
    /// - Return the first record that satisfies `match(record)`.
    /// - If `projection` is provided, return only that sub-structure.
    /// Concrete serializers may override this for efficient, true streaming
    /// (e.g., NDJSON line-by-line). The default implementation in ASerialization
    /// is allowed to load the full file and scan in memory.
    fn stream_read_record(&self, file_path: String, match: String, projection: Option<Vec<serde_json::Value>>) -> serde_json::Value;

    /// Stream-style update of logical records in a file.
    /// Semantics:
    /// - Apply `updater(record)` to each record for which `match(record)` is True.
    /// - When `atomic=True`, must preserve atomicity guarantees (temp file +
    /// replace, or equivalent) provided by the underlying serializer/I/O.
    /// - Returns the number of updated records.
    /// Concrete serializers may override this to avoid loading the full file.
    /// The default implementation in ASerialization may be full-load.
    fn stream_update_record(&self, file_path: String, match: String, updater: String) -> i64;

    /// Retrieve a logical page of records from a file.
    /// Semantics:
    /// - page_number is 1-based.
    /// - page_size is the number of records.
    /// - Returns a list of native records.
    /// Concrete serializers may override this to use indexes or streaming.
    /// The default implementation in ASerialization may load the entire file
    /// and slice a top-level list.
    fn get_record_page(&self, file_path: String, page_number: i64, page_size: i64) -> Vec<serde_json::Value>;

    /// Retrieve a logical record by identifier (e.g., record[id_field] == id_value).
    /// Concrete serializers may override this to use an index or format-specific
    /// mechanisms. The default implementation in ASerialization may perform a
    /// linear scan over a top-level list.
    fn get_record_by_id(&self, file_path: String, id_value: serde_json::Value) -> serde_json::Value;

}
