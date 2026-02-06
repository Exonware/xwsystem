// #exonware/xwsystem/rust/src/io/serialization/mod.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 2, 2025
//! 
//! Serialization module - 29+ serialization formats with I→A→XW pattern.
//! 
//! This module provides comprehensive serialization support following the
//! universal codec architecture.
//! 
//! Following I→A→XW pattern:
//! - I: ISerialization (interface)
//! - A: ASerialization (abstract base)
//! - XW: XW{Format}Serializer (concrete implementations)

// Contracts and base classes
pub mod auto_serializer;
pub mod base;
pub mod contracts;
pub mod defs;
pub mod errors;
pub mod flyweight;
pub mod format_detector;
pub mod formats_binary;
pub mod formats_database;
pub mod formats_text;
pub mod registry;
pub mod serializer;

pub use auto_serializer::{AutoSerializer};

pub use base::{ASchemaRegistry, ASerialization};

pub use contracts::{ISerialization};

pub use defs::{SerializationCapability, SerializationFormat, SerializationMode, SerializationType};

pub use errors::{SerializationError};

pub use flyweight::{SerializerPool, clear_serializer_cache, create_serializer, get_cache_info, get_flyweight_stats, get_serializer};

pub use format_detector::{detect_format};

pub use formats_binary::{BsonSerializer, CborSerializer, MarshalSerializer, MsgPackSerializer, PickleSerializer, PlistSerializer};

pub use formats_database::{DbmSerializer, ShelveSerializer, Sqlite3Serializer};

pub use formats_text::{ConfigParserSerializer, CsvSerializer, FormDataSerializer, Json5Serializer, JsonLinesSerializer, JsonSerializer, MultipartSerializer, TomlSerializer, XmlSerializer, YamlSerializer};

pub use registry::{SerializationRegistry, get_serialization_registry};

pub use serializer::{XWSerializer};
