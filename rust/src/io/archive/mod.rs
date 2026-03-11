// #exonware/xwsystem/rust/src/io/archive/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/__init__.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Archive and compression with REGISTRY PATTERN!
//! 
//! Like codec system - auto-detection and extensibility!
//! 
//! Priority 1 (Security): Safe archive operations
//! Priority 2 (Usability): Auto-detect format from extension
//! Priority 3 (Maintainability): Clean registry pattern
//! Priority 4 (Performance): Efficient compression
//! Priority 5 (Extensibility): Easy to add 7z, RAR, zstd, etc.

// Contracts
pub mod archive;
pub mod archive_files;
pub mod archivers;
pub mod base;
pub mod compression;
pub mod formats;

pub use archive::{Archive};

pub use archive_files::{TarFile, ZipFile};

pub use archivers::{TarArchiver, ZipArchiver};

pub use base::{AArchiveFormat, ACompressor, ArchiveFormatRegistry, CompressionRegistry, get_global_archive_registry, get_global_compression_registry};

pub use compression::{Compression};

pub use formats::{get_archiver_by_id, get_archiver_for_file, register_archive_format};
