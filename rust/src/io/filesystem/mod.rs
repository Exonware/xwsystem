// #exonware/xwsystem/rust/src/io/filesystem/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/filesystem/__init__.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Virtual filesystem abstractions.
//! 
//! Following codec/ pattern.
//! 
//! Priority 1 (Security): Safe filesystem operations
//! Priority 2 (Usability): Uniform API across backends
//! Priority 3 (Maintainability): Clean filesystem abstraction
//! Priority 4 (Performance): Efficient operations
//! Priority 5 (Extensibility): Ready for S3, ZIP, FTP filesystems

pub mod base;
pub mod local;

pub use base::{AFileSystem};

pub use local::{LocalFileSystem};
