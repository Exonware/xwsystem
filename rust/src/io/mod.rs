// #exonware/xwsystem/rust/src/io/mod.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! I/O utilities for safe file operations, path management, and codec integration.
//! 
//! FINAL CLEAN STRUCTURE:
//! ├── contracts.py     - ALL interfaces in ONE file
//! ├── defs.py          - ALL enums in ONE file
//! ├── errors.py        - ALL exceptions in ONE file
//! ├── base.py          - Root base classes
//! ├── codec/           - Codec abstractions
//! ├── common/          - Shared utilities (atomic, path, watcher, lock)
//! ├── file/            - File-specific implementations + modular paging
//! ├── folder/          - Folder-specific implementations
//! ├── stream/          - Stream operations + codec integration
//! ├── filesystem/      - Virtual filesystem abstractions
//! ├── archive/         - Archive + compression (registry-based)
//! └── manager/         - High-level managers
//! 
//! Priority 1 (Security): Safe operations, validation, atomic writes
//! Priority 2 (Usability): Clean organization, easy imports
//! Priority 3 (Maintainability): No duplication, single source of truth
//! Priority 4 (Performance): Efficient imports, registry patterns
//! Priority 5 (Extensibility): Modular design, easy to extend

// ═══════════════════════════════════════════════════════════════════════
// ROOT LEVEL - Core Definitions (1 defs, 1 errors, 1 contracts)
// ═══════════════════════════════════════════════════════════════════════

pub mod archive;
pub mod base;
pub mod codec;
pub mod common;
pub mod contracts;
pub mod defs;
pub mod errors;
pub mod facade;
pub mod file;
pub mod filesystem;
pub mod folder;
pub mod serialization;
pub mod stream;

pub use archive::{Archive, ArchiveFormatRegistry, Compression, TarArchiver, TarFile, ZipArchiver, ZipFile, get_archiver_for_file, get_global_archive_registry, register_archive_format};

pub use base::{AAsyncIO, AAtomicOperations, ABackupOperations, AFile, AFileManager, AFolder, APath, AStream, ATemporaryOperations, AUnifiedIO};

pub use codec::base::{ACodec};

pub use codec::contracts::{ICodec, ICodecMetadata};

pub use codec::registry::{UniversalCodecRegistry, get_registry};

pub use common::{AtomicFileWriter, FileLock, FileOperationError, FileWatcher, PathManager, safe_read_bytes, safe_read_text, safe_read_with_fallback, safe_write_bytes, safe_write_text};

pub use contracts::{DecodeOptions, EncodeOptions, Formatter, IArchiveFile, IArchiver, IAsyncIO, IAtomicOperations, IBackupOperations, ICodecIO, ICompression, IDataSource, IFile, IFileLock, IFileManager, IFileSystem, IFileWatcher, IFolder, IPagedCodecIO, IPagedDataSource, IPath, IStream, ITemporaryOperations, IUnifiedIO, Serializer};

pub use defs::{ArchiveFormat, AtomicMode, CodecIOMode, CompressionAlgorithm, CompressionLevel, FSScheme, FileEncoding, FileMode, FileType, LockMode, LockType, ManagerMode, OperationResult, PagingMode, PathSecurityLevel, PathType, StreamMode, TraversalMode, WatcherEvent};

pub use errors::{FileLockError, FileReadError, FileWriteError, XWFileNotFoundError, XWPermissionError};

pub use facade::{XWIO};

pub use file::{BytePagingStrategy, FileDataSource, LinePagingStrategy, PagedFileSource, PagingStrategyRegistry, RecordPagingStrategy, XWFile, auto_detect_paging_strategy, get_global_paging_registry, get_paging_strategy, register_paging_strategy};

pub use filesystem::{LocalFileSystem};

pub use folder::{XWFolder};

pub use serialization::{ASerialization, ISerialization, SerializationRegistry, get_serialization_registry};

pub use stream::{AsyncAtomicFileWriter, CodecIO, PagedCodecIO};
