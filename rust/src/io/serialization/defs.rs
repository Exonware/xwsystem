// #exonware/xwsystem/rust/src/io/serialization/defs.rs
//exonware/xwsystem/serialization/types.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Serialization types and enums for XWSystem.

/// Supported serialization formats.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerializationFormat {
    Json,
    Yaml,
    Xml,
    Toml,
    Pickle,
    Msgpack,
    Cbor,
    Bson,
    Protobuf,
    Avro,
    Native,
}

/// Serialization modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerializationMode {
    Compact,
    Pretty,
    Binary,
    Text,
}

/// Serialization types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerializationType {
    Object,
    Array,
    Primitive,
    Custom,
}

// Advanced features (added in ADR-001)
// Supports JSONPointer/XPath/YAML path updates
// Can atomically update paths without loading full file
// Supports schema validation
// True incremental streaming (not chunked full-file)
// Supports multiple documents in one file
// Supports query/filter operations (JSONPath, XPath)
// Supports merge/update operations
// Supports lazy loading for large files
/// Serialization capabilities for introspection.
pub struct SerializationCapability {
    pub STREAMING: String,
    pub PARTIAL_ACCESS: String,
    pub TYPED_DECODE: String,
    pub ZERO_COPY: String,
    pub CANONICAL: String,
    pub RANDOM_ACCESS: String,
    pub PATH_BASED_UPDATES: String,
    pub ATOMIC_PATH_WRITE: String,
    pub SCHEMA_VALIDATION: String,
    pub INCREMENTAL_STREAMING: String,
    pub MULTI_DOCUMENT: String,
    pub QUERY_SUPPORT: String,
    pub MERGE_OPERATIONS: String,
    pub LAZY_LOADING: String,
}

impl Flag for SerializationCapability {
    // TODO: Implement trait methods
}

impl SerializationCapability {
}

/// Schema compatibility levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "BACKWARD")]
    Backward,
    #[serde(rename = "BACKWARD_TRANSITIVE")]
    BackwardTransitive,
    #[serde(rename = "FORWARD")]
    Forward,
    #[serde(rename = "FORWARD_TRANSITIVE")]
    ForwardTransitive,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "FULL_TRANSITIVE")]
    FullTransitive,
}
