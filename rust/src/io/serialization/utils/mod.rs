// #exonware/xwsystem/rust/src/io/serialization/utils/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/serialization/utils/__init__.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 9, 2025
//! 
//! Serialization utilities module.

pub mod path_ops;

pub use path_ops::{
    PathComponent, PathOperationError, get_value_by_path, normalize_path, 
    parse_json_pointer, set_value_by_path, validate_json_pointer, validate_path_security
};
