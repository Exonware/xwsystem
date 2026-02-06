// #exonware/xwsystem/rust/src/io/serialization/errors.rs
//exonware/xwsystem/serialization/errors.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Error classes for XWSystem serialization module.


use std::option::{Option};

// ============================================================================

// SCHEMA REGISTRY ERRORS (Moved from enterprise)

// ============================================================================

/// Base exception for serialization errors.
#[derive(Debug)]
pub struct SerializationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SerializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl SerializationError {
    pub fn new(message: impl Into<String>) -> Self {
        SerializationError {
            message: message.into(),
            source: None,
        }
    }

}

/// Error when format detection fails.
#[derive(Debug)]
pub struct FormatDetectionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FormatDetectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FormatDetectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FormatDetectionError {
    pub fn new(message: impl Into<String>) -> Self {
        FormatDetectionError {
            message: message.into(),
            source: None,
        }
    }
}

/// Error when validation fails.
#[derive(Debug)]
pub struct ValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        ValidationError {
            message: message.into(),
            source: None,
        }
    }
}

/// CSV serialization error.
#[derive(Debug)]
pub struct CsvError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CsvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CsvError {
    pub fn new(message: impl Into<String>) -> Self {
        CsvError {
            message: message.into(),
            source: None,
        }
    }

}

/// CBOR serialization error.
#[derive(Debug)]
pub struct CborError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CborError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CborError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CborError {
    pub fn new(message: impl Into<String>) -> Self {
        CborError {
            message: message.into(),
            source: None,
        }
    }

}

/// JSON-specific serialization error.
#[derive(Debug)]
pub struct JsonError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl JsonError {
    /// Constructor
    pub fn new(message: impl Into<String>) -> Self {
        JsonError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_error(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        JsonError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// YAML serialization error.
#[derive(Debug)]
pub struct YamlError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for YamlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl YamlError {
    pub fn new(message: impl Into<String>) -> Self {
        YamlError {
            message: message.into(),
            source: None,
        }
    }
}

/// Form data serialization error.
#[derive(Debug)]
pub struct FormDataError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FormDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FormDataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FormDataError {
    pub fn new(message: impl Into<String>) -> Self {
        FormDataError {
            message: message.into(),
            source: None,
        }
    }

}

/// Multipart serialization error.
#[derive(Debug)]
pub struct MultipartError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for MultipartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MultipartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl MultipartError {
    pub fn new(message: impl Into<String>) -> Self {
        MultipartError {
            message: message.into(),
            source: None,
        }
    }

}

/// Marshal serialization error.
#[derive(Debug)]
pub struct MarshalError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for MarshalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MarshalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl MarshalError {
    pub fn new(message: impl Into<String>) -> Self {
        MarshalError {
            message: message.into(),
            source: None,
        }
    }

}

/// XML serialization error.
#[derive(Debug)]
pub struct XmlError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XmlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XmlError {
    pub fn new(message: impl Into<String>) -> Self {
        XmlError {
            message: message.into(),
            source: None,
        }
    }

}

/// Pickle serialization error.
#[derive(Debug)]
pub struct PickleError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PickleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PickleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PickleError {
    pub fn new(message: impl Into<String>) -> Self {
        PickleError {
            message: message.into(),
            source: None,
        }
    }

}

/// FlatBuffers serialization error.
#[derive(Debug)]
pub struct FlatBuffersError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FlatBuffersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FlatBuffersError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FlatBuffersError {
    pub fn new(message: impl Into<String>) -> Self {
        FlatBuffersError {
            message: message.into(),
            source: None,
        }
    }
}

/// FlatBuffers serialization error.
#[derive(Debug)]
pub struct FlatBuffersError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FlatBuffersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FlatBuffersError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FlatBuffersError {
    pub fn new(message: impl Into<String>) -> Self {
        FlatBuffersError {
            message: message.into(),
            source: None,
        }
    }
}

/// Cap'n Proto serialization error.
#[derive(Debug)]
pub struct CapnProtoError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CapnProtoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CapnProtoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CapnProtoError {
    pub fn new(message: impl Into<String>) -> Self {
        CapnProtoError {
            message: message.into(),
            source: None,
        }
    }
}

/// Apache ORC serialization error.
#[derive(Debug)]
pub struct OrcError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for OrcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OrcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl OrcError {
    pub fn new(message: impl Into<String>) -> Self {
        OrcError {
            message: message.into(),
            source: None,
        }
    }
}

/// Apache Parquet serialization error.
#[derive(Debug)]
pub struct ParquetError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ParquetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParquetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ParquetError {
    pub fn new(message: impl Into<String>) -> Self {
        ParquetError {
            message: message.into(),
            source: None,
        }
    }
}

/// BSON serialization error.
#[derive(Debug)]
pub struct BsonError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for BsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl BsonError {
    pub fn new(message: impl Into<String>) -> Self {
        BsonError {
            message: message.into(),
            source: None,
        }
    }
}

/// Apache Thrift serialization error.
#[derive(Debug)]
pub struct ThriftError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ThriftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThriftError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ThriftError {
    pub fn new(message: impl Into<String>) -> Self {
        ThriftError {
            message: message.into(),
            source: None,
        }
    }
}

/// Protocol Buffers serialization error.
#[derive(Debug)]
pub struct ProtobufError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ProtobufError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ProtobufError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ProtobufError {
    pub fn new(message: impl Into<String>) -> Self {
        ProtobufError {
            message: message.into(),
            source: None,
        }
    }
}

/// Apache Avro serialization error.
#[derive(Debug)]
pub struct AvroError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for AvroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AvroError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl AvroError {
    pub fn new(message: impl Into<String>) -> Self {
        AvroError {
            message: message.into(),
            source: None,
        }
    }
}

/// SQLite3 serialization error.
#[derive(Debug)]
pub struct Sqlite3Error {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for Sqlite3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Sqlite3Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl Sqlite3Error {
    pub fn new(message: impl Into<String>) -> Self {
        Sqlite3Error {
            message: message.into(),
            source: None,
        }
    }
}

/// Shelve serialization error.
#[derive(Debug)]
pub struct ShelveError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ShelveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ShelveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ShelveError {
    pub fn new(message: impl Into<String>) -> Self {
        ShelveError {
            message: message.into(),
            source: None,
        }
    }
}

/// DBM serialization error.
#[derive(Debug)]
pub struct DbmError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DbmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DbmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl DbmError {
    pub fn new(message: impl Into<String>) -> Self {
        DbmError {
            message: message.into(),
            source: None,
        }
    }
}

/// ConfigParser serialization error.
#[derive(Debug)]
pub struct ConfigParserError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConfigParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConfigParserError {
    pub fn new(message: impl Into<String>) -> Self {
        ConfigParserError {
            message: message.into(),
            source: None,
        }
    }
}

/// Plistlib serialization error.
#[derive(Debug)]
pub struct PlistlibError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PlistlibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PlistlibError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PlistlibError {
    pub fn new(message: impl Into<String>) -> Self {
        PlistlibError {
            message: message.into(),
            source: None,
        }
    }
}

/// TOML serialization error.
#[derive(Debug)]
pub struct TomlError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for TomlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TomlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl TomlError {
    pub fn new(message: impl Into<String>) -> Self {
        TomlError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when schema registry operation fails.
#[derive(Debug)]
pub struct SchemaRegistryError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for SchemaRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SchemaRegistryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl SchemaRegistryError {
    pub fn new(message: impl Into<String>) -> Self {
        SchemaRegistryError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when schema is not found.
#[derive(Debug)]
pub struct SchemaNotFoundError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for SchemaNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SchemaNotFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl SchemaNotFoundError {
    pub fn new(message: impl Into<String>) -> Self {
        SchemaNotFoundError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when schema validation fails.
#[derive(Debug)]
pub struct SchemaValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for SchemaValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SchemaValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl SchemaValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        SchemaValidationError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when schema version is invalid.
#[derive(Debug)]
pub struct SchemaVersionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for SchemaVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SchemaVersionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl SchemaVersionError {
    pub fn new(message: impl Into<String>) -> Self {
        SchemaVersionError {
            message: message.into(),
            source: None,
        }
    }
}
