// #exonware/xwsystem/rust/src/io/defs.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! IO module definitions - ALL enums and types in ONE place.
//! 
//! Consolidated from all submodules for maintainability.

// From common

// From common

// From common

// From common

// From folder

// From stream

// From stream

// From filesystem

// From archive

// From archive

// From archive

// From manager

/// File operation modes.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileMode {
    #[serde(rename = "r")]
    Read,
    #[serde(rename = "w")]
    Write,
    #[serde(rename = "a")]
    Append,
    #[serde(rename = "r+")]
    ReadWrite,
    #[serde(rename = "w+")]
    WriteRead,
    #[serde(rename = "a+")]
    AppendRead,
    #[serde(rename = "rb")]
    BinaryRead,
    #[serde(rename = "wb")]
    BinaryWrite,
    #[serde(rename = "ab")]
    BinaryAppend,
    #[serde(rename = "rb+")]
    BinaryReadWrite,
    #[serde(rename = "wb+")]
    BinaryWriteRead,
    #[serde(rename = "ab+")]
    BinaryAppendRead,
}

/// File types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    Text,
    Binary,
    Json,
    Yaml,
    Xml,
    Csv,
    Config,
    Log,
    Temp,
    Backup,
}

/// Path types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathType {
    File,
    Directory,
    Link,
    Unknown,
}

/// Operation result status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationResult {
    Success,
    Failure,
    Partial,
    Skipped,
}

/// Lock types for file operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockType {
    Shared,
    Exclusive,
    None,
}

/// Atomic operation modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtomicMode {
    Write,
    #[serde(rename = "write_backup")]
    WriteBackup,
    Move,
    Copy,
}

/// File watcher event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WatcherEvent {
    Created,
    Modified,
    Deleted,
    Moved,
}

/// File lock modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockMode {
    Exclusive,
    Shared,
    Blocking,
    #[serde(rename = "non_blocking")]
    NonBlocking,
}

// No validation (unsafe!)
/// Path validation security levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathSecurityLevel {
    Strict,
    Moderate,
    Relaxed,
    Disabled,
}

// Page by record boundaries
// Auto-detect best strategy
/// Paging strategies for file reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PagingMode {
    Byte,
    Line,
    Record,
    Smart,
    Auto,
}

/// Common file encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileEncoding {
    #[serde(rename = "utf-8")]
    Utf8,
    #[serde(rename = "utf-16")]
    Utf16,
    #[serde(rename = "utf-32")]
    Utf32,
    Ascii,
    #[serde(rename = "latin-1")]
    Latin1,
    Cp1252,
}

/// Directory traversal modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraversalMode {
    #[serde(rename = "depth_first")]
    DepthFirst,
    #[serde(rename = "breadth_first")]
    BreadthFirst,
    #[serde(rename = "files_only")]
    FilesOnly,
    #[serde(rename = "dirs_only")]
    DirsOnly,
}

/// Stream operation modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamMode {
    Sync,
    Async,
}

/// Codec I/O modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodecIOMode {
    Full,
    Paged,
    Streaming,
}

/// Filesystem scheme types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FSScheme {
    File,
    S3,
    Ftp,
    Sftp,
    Http,
    Zip,
    Mem,
}

// Future formats (extensible!)
/// Archive format types.
///
/// Current: ZIP, TAR
/// Future: 7Z, RAR, CAB, ISO, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArchiveFormat {
    Zip,
    Tar,
    #[serde(rename = "tar.gz")]
    TarGz,
    #[serde(rename = "tar.bz2")]
    TarBz2,
    #[serde(rename = "tar.xz")]
    TarXz,
    #[serde(rename = "7z")]
    SevenZip,
    Rar,
    Cab,
    Iso,
    Arj,
    Lzh,
    Ace,
}

// Future algorithms (extensible!)
// Zstandard (faster than gzip)
// XZ (high compression)
/// Compression algorithm types.
///
/// Current: gzip, bz2, lzma
/// Future: zstd, brotli, snappy, lz4, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    None,
    Gzip,
    Bz2,
    Lzma,
    Zstd,
    Brotli,
    Snappy,
    Lz4,
    Xz,
}

/// Compression level presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionLevel {
    Fast,
    Balanced,
    Best,
}

/// Manager operation modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManagerMode {
    Strict,
    Relaxed,
}

/// Codec capabilities for introspection.
/// 
/// Used as bitflags to indicate codec capabilities.
/// Can be combined using bitwise OR (|) operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodecCapability(u8);

impl CodecCapability {
    pub const NONE: CodecCapability = CodecCapability(0);
    pub const STREAMING: CodecCapability = CodecCapability(1 << 0);
    pub const BIDIRECTIONAL: CodecCapability = CodecCapability(1 << 1);
    pub const SCHEMA_BASED: CodecCapability = CodecCapability(1 << 2);
    pub const BINARY_OUTPUT: CodecCapability = CodecCapability(1 << 3);
    pub const TEXT_OUTPUT: CodecCapability = CodecCapability(1 << 4);
    pub const COMPRESSION: CodecCapability = CodecCapability(1 << 5);

    /// Check if this capability contains the given flag.
    pub fn contains(self, other: CodecCapability) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Combine two capabilities.
    pub fn union(self, other: CodecCapability) -> CodecCapability {
        CodecCapability(self.0 | other.0)
    }
}

impl std::ops::BitOr for CodecCapability {
    type Output = CodecCapability;

    fn bitor(self, rhs: CodecCapability) -> Self::Output {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for CodecCapability {
    fn bitor_assign(&mut self, rhs: CodecCapability) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for CodecCapability {
    type Output = CodecCapability;

    fn bitand(self, rhs: CodecCapability) -> Self::Output {
        CodecCapability(self.0 & rhs.0)
    }
}

// json, yaml, xml, toml, pickle
// zip, 7z, tar, rar, zst
// gzip, bz2, lzma (raw compression)
/// Codec categories for format conversion compatibility.
///
/// Formats can only convert within the same category:
/// - ARCHIVE: zip ↔ 7z ✓ (both archives)
/// - SERIALIZATION: json ↔ yaml ✓ (both serialization)
/// - ARCHIVE ↔ SERIALIZATION: ✗ (incompatible)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodecCategory {
    Serialization,
    Archive,
    Formatter,
    Encryption,
    Encoding,
    Compression,
}
