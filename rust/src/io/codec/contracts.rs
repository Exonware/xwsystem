// #exonware/xwsystem/rust/src/io/codec/contracts.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Codec module contracts - interfaces for codec operations.


use std::collections::HashMap;

use crate::contracts::{CodecCapability, DecodeOptions, EncodeOptions};
use std::path::{Path};

/// Universal codec interface for bidirectional transformation.
///
/// A codec transforms between a model (T) and its representation (R).
/// This is the minimal contract that all codecs must implement.
///
/// Type Parameters:
/// T: Model type (e.g., dict, AST, dataclass)
/// R: Representation type (bytes or str)
///
/// Examples:
/// JSON serializer:  ICodec[dict, bytes]
/// SQL formatter:    ICodec[QueryAST, str]
/// Pickle:           ICodec[Any, bytes]
/// Python unparser:  ICodec[ast.AST, str]
///
/// Design Principles:
/// - Bidirectional by default (encode/decode)
/// - Options-based configuration (not constructor pollution)
/// - Representation-type specific (bytes OR str, not both)
/// - Composable via adapters
pub trait ICodec {
    /// Encode a model to its representation.
    /// Args:
    /// value: Model instance to encode
    /// options: Format-specific encoding options (e.g., {'pretty': True})
    /// Returns:
    /// Representation (bytes or str depending on codec type)
    /// Raises:
    /// EncodeError: If encoding fails
    /// Examples:
    /// >>> codec = JsonSerializer()
    /// >>> codec.encode({"key": "value"})
    /// b'{"key":"value"}'
    /// >>> formatter = SqlFormatter()
    /// >>> formatter.encode(select_ast, options={"pretty": True})
    /// 'SELECT *\nFROM users\nWHERE id = 1'
    fn encode(&self, value: T) -> R;

    /// Decode a representation to a model.
    /// Args:
    /// repr: Representation to decode (bytes or str)
    /// options: Format-specific decoding options (e.g., {'strict': False})
    /// Returns:
    /// Model instance
    /// Raises:
    /// DecodeError: If decoding fails
    /// Examples:
    /// >>> codec = JsonSerializer()
    /// >>> codec.decode(b'{"key":"value"}')
    /// {'key': 'value'}
    /// >>> formatter = SqlFormatter()
    /// >>> formatter.decode('SELECT * FROM users')
    /// QueryAST(...)
    fn decode(&self, repr: R) -> T;

}

/// Metadata protocol for codec discovery and registration.
///
/// Codecs that implement this protocol can self-register and be
/// discovered by the registry system with no hardcoding.
///
/// Example:
/// >>> class JsonCodec:
/// ...     codec_id = "json"
/// ...     media_types = ["application/json", "text/json"]
/// ...     file_extensions = [".json", ".jsonl"]
/// ...     aliases = ["JSON"]
/// ...
/// ...     def capabilities(self):
/// ...         return CodecCapability.BIDIRECTIONAL | CodecCapability.TEXT
pub trait ICodecMetadata {
    /// Unique codec identifier.
    /// Should be lowercase, alphanumeric + dash/underscore.
    /// Examples:
    /// - "json"
    /// - "sql"
    /// - "protobuf"
    /// - "python-ast"
    // Python decorators: @property
    fn codec_id(&self) -> &str;

    /// Supported media types / content types (RFC 2046).
    /// Used for content negotiation and HTTP Content-Type headers.
    /// Examples:
    /// - JSON: ["application/json", "text/json"]
    /// - SQL: ["application/sql", "text/x-sql"]
    /// - Protobuf: ["application/protobuf", "application/x-protobuf"]
    // Python decorators: @property
    fn media_types(&self) -> Vec<String>;

    /// Supported file extensions (with leading dot).
    /// Used for auto-detection from file paths.
    /// Examples:
    /// - JSON: [".json", ".jsonl"]
    /// - SQL: [".sql", ".ddl", ".dml"]
    /// - Python: [".py", ".pyi"]
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

    /// Alternative names for this codec.
    /// Used for flexible lookup (case-insensitive matching).
    /// Examples:
    /// - JSON: ["json", "JSON"]
    /// - SQL: ["sql", "SQL", "structured-query"]
    // Python decorators: @property
    fn aliases(&self) -> Vec<String>;

    /// Codec categories for classification and filtering (can be multiple).
    /// Used to group codecs by their purposes. A codec can belong to multiple categories.
    /// Standard types:
    /// - "serialization": Data serialization formats (JSON, YAML, XML, etc.)
    /// - "archive": Archive/compression formats (ZIP, TAR, GZ, etc.)
    /// - "compression": Pure compression formats (GZIP, BZIP2, etc.)
    /// - "query": Query language parsers (SQL, GraphQL, Cypher, etc.)
    /// - "syntax": Programming language syntax (Python, JavaScript, etc.)
    /// - "binary": Binary formats (Protobuf, MessagePack, etc.)
    /// - "markup": Markup languages (HTML, Markdown, XML, etc.)
    /// - "schema": Schema definition languages (JSON Schema, XSD, etc.)
    /// - "config": Configuration formats (INI, ENV, TOML, etc.)
    /// - "data": Data exchange formats (CSV, Excel, etc.)
    /// Examples:
    /// - JSON codec: ["serialization"]
    /// - XML codec: ["serialization", "markup"]
    /// - TOML codec: ["config", "serialization"]
    /// - ZIP codec: ["archive"]
    /// - SQL codec: ["query"]
    /// - Python codec: ["syntax"]
    // Python decorators: @property
    fn codec_types(&self) -> Vec<String>;

    /// Get capabilities supported by this codec.
    /// Returns:
    /// Flag combination of supported features
    /// Example:
    /// >>> codec.capabilities()
    /// CodecCapability.BIDIRECTIONAL | CodecCapability.TEXT | CodecCapability.SCHEMA
    fn capabilities(&self) -> CodecCapability;

}

/// Interface for format configuration.
///
/// This interface represents configuration for codec/format selection
/// and is used across xwsystem for format-aware operations. It provides
/// a standardized way to specify and validate codec/format configurations
/// with integration to the codec registry.
///
/// Originally defined in xwstorage, moved to xwsystem.io.codec as it
/// is a general-purpose codec configuration abstraction.
pub trait IFormatConfig {
    /// Get the format type identifier (codec_id).
    /// Returns:
    /// Format type identifier (e.g., "json", "yaml", "xml")
    // Python decorators: @property
    fn format_type(&self) -> &str;

    /// Get the codec identifier from xwsystem codec registry.
    /// This should match a codec_id in the UniversalCodecRegistry.
    /// Returns:
    /// Codec identifier (typically same as format_type)
    // Python decorators: @property
    fn codec_id(&self) -> &str;

    /// Convert configuration to dictionary.
    /// Returns:
    /// Dictionary representation of the format configuration
    fn to_dict(&self) -> HashMap<String, serde_json::Value>;

    /// Validate format configuration.
    /// Checks if the codec_id is available in the xwsystem codec registry.
    /// Returns:
    /// True if format is valid and available in codec registry
    fn validate(&self) -> bool;

}
