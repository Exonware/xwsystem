// #exonware/xwsystem/rust/src/io/serialization/formats/database/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/database/__init__.py
//! Database-based serialization formats (core lightweight formats only).

// Core lightweight database formats (built-in, ~0 KB)
pub mod dbm;
pub mod shelve;
pub mod sqlite3;

pub use dbm::{DbmSerializer};

pub use shelve::{ShelveSerializer};

pub use sqlite3::{Sqlite3Serializer};
