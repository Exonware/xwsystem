// #exonware/xwsystem/rust/src/io/serialization/formats/binary/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/binary/__init__.py
//! Binary serialization formats (core lightweight formats).

// Core binary formats
pub mod bson;
pub mod cbor;
pub mod marshal;
pub mod msgpack;
pub mod pickle;
pub mod plistlib;

pub use bson::{BsonSerializer};

pub use cbor::{CborSerializer};

pub use marshal::{MarshalSerializer};

pub use msgpack::{MsgPackSerializer};

pub use pickle::{PickleSerializer};

pub use plistlib::{PlistSerializer};
