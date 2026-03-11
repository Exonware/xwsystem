// #exonware/xwsystem/rust/src/io/codec/mod.rs
// exonware/xwsystem/io/codec/__init__.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 30, 2025
//! 
//! Universal Codec Abstraction for eXonware.
//! 
//! Provides unified interface for all encoding/decoding operations
//! across serialization (bytes) and syntax (strings).
//! 
//! Core Principle:
//! ICodec[T, R] - Universal bidirectional transformation
//! - Serializer = ICodec[T, bytes] for persistence/wire
//! - Formatter = ICodec[T, str] for language/syntax
//! 
//! Public API:
//! Core Abstractions:
//! - ICodec[T, R]          Universal encoder/decoder
//! - Serializer[T]         Type alias for ICodec[T, bytes]
//! - Formatter[T]          Type alias for ICodec[T, str]
//! 
//! Metadata:
//! - ICodecMetadata        Self-describing codec protocol
//! - CodecCapability       Capability flags
//! 
//! Registry:
//! - MediaKey              Media type wrapper (RFC 2046)
//! - CodecRegistry         Codec lookup by media-type/extension/id
//! - get_codec()           Get codec by media type
//! - get_codec_for_file()  Get codec by file extension
//! - get_codec_by_id()     Get codec by ID
//! 
//! Adapters:
//! - FormatterToSerializer str → bytes via UTF-8
//! - SerializerToFormatter bytes → str via UTF-8
//! 
//! Base Classes:
//! - ACodec[T, R]       Base implementation with all convenience methods
//! 
//! Errors:
//! - CodecError            Base codec error
//! - EncodeError           Encoding failure
//! - DecodeError           Decoding failure
//! - CodecNotFoundError    Registry lookup failure
//! 
//! Examples:
//! >>> # Get codec by media type
//! >>> codec = get_codec(MediaKey("application/json"))
//! >>> data = codec.encode({"key": "value"})
//! >>>
//! >>> # Auto-detect from file
//! >>> codec = get_codec_for_file("data.json")
//! >>> result = codec.decode(data)
//! >>>
//! >>> # Use adapters
//! >>> formatter = SqlFormatter()  # Returns str
//! >>> serializer = FormatterToSerializer(formatter)  # Now returns bytes

pub mod base;
pub mod contracts;

pub use base::{ACodec, CodecRegistry, FormatterToSerializer, MediaKey, SerializerToFormatter, get_global_registry};

pub use contracts::{IFormatConfig};
