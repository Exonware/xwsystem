// #exonware/xwsystem/rust/src/io/stream/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/stream/__init__.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Stream operations and codec-integrated I/O.
//! 
//! Following codec/ pattern + THE KILLER FEATURE (CodecIO)!
//! 
//! Priority 1 (Security): Safe streaming with validation
//! Priority 2 (Usability): Seamless codec integration - auto-detect format!
//! Priority 3 (Maintainability): Clean codec + I/O separation
//! Priority 4 (Performance): Memory-efficient streaming
//! Priority 5 (Extensibility): Works with any codec + any source

pub mod async_operations;
pub mod base;
pub mod codec_io;

pub use async_operations::{AsyncAtomicFileWriter};

pub use base::{ACodecIO, APagedCodecIO};

pub use codec_io::{CodecIO, PagedCodecIO};
