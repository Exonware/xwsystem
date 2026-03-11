// #exonware/xwsystem/rust/src/io/contracts.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! IO module contracts - interfaces and enums for input/output operations.


use std::collections::HashMap;
use std::fs;

use crate::defs::{CodecCapability, FileMode, FileType, LockType, OperationResult, PathType};
use crate::typing_extensions::{TypeAlias};
use std::option::{Protocol, runtime_checkable};
use std::path::{Path, PathBuf};

// Type aliases for Python compatibility
pub type TextIO = fs::File;
pub type BinaryIO = fs::File;

// ============================================================================

// FOLDER INTERFACES

// ============================================================================

// ============================================================================

// PATH INTERFACES

// ============================================================================

// ============================================================================

// STREAM INTERFACES

// ============================================================================

// ============================================================================

// ASYNC I/O INTERFACES

// ============================================================================

// ============================================================================

// ATOMIC OPERATIONS INTERFACES

// ============================================================================

// ============================================================================

// BACKUP OPERATIONS INTERFACES

// ============================================================================

// ============================================================================

// TEMPORARY OPERATIONS INTERFACES

// ============================================================================

// ============================================================================

// UNIFIED I/O INTERFACE

// ============================================================================

// ============================================================================

// FILE MANAGER INTERFACE

// ============================================================================

// ============================================================================

// DATA SOURCE INTERFACES (Used by file/, stream/)

// ============================================================================

// ============================================================================

// CODEC-INTEGRATED IO INTERFACES (Used by stream/)

// ============================================================================

// ============================================================================

// FILE SYSTEM INTERFACES (Used by common/, filesystem/)

// ============================================================================

// ============================================================================

// ARCHIVE INTERFACES - MOVED TO AFTER ICodec DEFINITION

// (See line ~1400 for IArchiver, IArchiveFile, ICompression)

// ============================================================================

// ============================================================================

// SUBFOLDER CONTRACTS (Consolidated)

// ============================================================================

// From archive/

// From codec/

// From common/

// ============================================================================

// ARCHIVE INTERFACES (Dual Architecture: Codec + File)

// ============================================================================

// From filesystem/

// From folder/

// From manager/

// From stream/

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
// STATIC UTILITY METHODS (File Manager Features)
// ============================================================================
/// Interface for file operations with both static and instance methods.
///
/// Provides comprehensive file operations including:
/// - File I/O operations (read, write, save, load)
/// - File metadata operations (size, permissions, timestamps)
/// - File validation and safety checks
/// - Static utility methods for file operations
pub trait IFile {
    /// Open file with specified mode.
    fn open(&self, mode: FileMode) -> ();

    /// Read from file.
    fn read(&self, size: Option<i64>) -> String;

    /// Write to file.
    fn write(&self, data: String) -> i64;

    /// Close file.
    fn close(&self) -> ();

    /// Save data to file.
    fn save(&self, data: serde_json::Value) -> bool;

    /// Load data from file.
    fn load(&self) -> serde_json::Value;

    /// Save data to specific path.
    fn save_as(&self, path: String, data: serde_json::Value) -> bool;

    /// Write current object to file.
    fn to_file(&self, path: String) -> bool;

    /// Load object from file.
    fn from_file(&self, path: String) -> String;

    /// Check if file exists.
    // Python decorators: @staticmethod
    fn exists(path: String) -> bool;

    /// Get file size.
    // Python decorators: @staticmethod
    fn size(path: String) -> i64;

    /// Delete file.
    // Python decorators: @staticmethod
    fn delete(path: String) -> bool;

    /// Copy file.
    // Python decorators: @staticmethod
    fn copy(source: String, destination: String) -> bool;

    /// Move file.
    // Python decorators: @staticmethod
    fn move(source: String, destination: String) -> bool;

    /// Rename file.
    // Python decorators: @staticmethod
    fn rename(old_path: String, new_path: String) -> bool;

    /// Get file modification time.
    // Python decorators: @staticmethod
    fn get_modified_time(path: String) -> f64;

    /// Get file creation time.
    // Python decorators: @staticmethod
    fn get_created_time(path: String) -> f64;

    /// Get file permissions.
    // Python decorators: @staticmethod
    fn get_permissions(path: String) -> i64;

    /// Check if file is readable.
    // Python decorators: @staticmethod
    fn is_readable(path: String) -> bool;

    /// Check if file is writable.
    // Python decorators: @staticmethod
    fn is_writable(path: String) -> bool;

    /// Check if file is executable.
    // Python decorators: @staticmethod
    fn is_executable(path: String) -> bool;

    /// Read file as text.
    // Python decorators: @staticmethod
    fn read_text(path: String, encoding: String) -> String;

    /// Read file as bytes.
    // Python decorators: @staticmethod
    fn read_bytes(path: String) -> Vec<u8>;

    /// Write text to file.
    // Python decorators: @staticmethod
    fn write_text(path: String, content: String, encoding: String) -> bool;

    /// Write bytes to file.
    // Python decorators: @staticmethod
    fn write_bytes(path: String, content: Vec<u8>) -> bool;

    /// Safely read text file, returning None on error.
    // Python decorators: @staticmethod
    fn safe_read_text(path: String, encoding: String) -> Option<String>;

    /// Safely read binary file, returning None on error.
    // Python decorators: @staticmethod
    fn safe_read_bytes(path: String) -> Option<Vec<u8>>;

    /// Safely write text to file.
    // Python decorators: @staticmethod
    fn safe_write_text(path: String, content: String, encoding: String) -> bool;

    /// Safely write bytes to file.
    // Python decorators: @staticmethod
    fn safe_write_bytes(path: String, content: Vec<u8>) -> bool;

    /// Atomically write data to file (static version).
    // Python decorators: @staticmethod
    fn atomic_write(file_path: String, data: String, backup: bool) -> OperationResult;

    /// Atomically copy file (static version).
    // Python decorators: @staticmethod
    fn atomic_copy(source: String, destination: String) -> OperationResult;

    /// Atomically move file (static version).
    // Python decorators: @staticmethod
    fn atomic_move(source: String, destination: String) -> OperationResult;

    /// Atomically delete file (static version).
    // Python decorators: @staticmethod
    fn atomic_delete(file_path: String, backup: bool) -> OperationResult;

    /// Create backup of file (static version).
    // Python decorators: @staticmethod
    fn create_backup(source: String, backup_dir: String) -> Option<PathBuf>;

    /// Restore from backup (static version).
    // Python decorators: @staticmethod
    fn restore_backup(backup_path: String, target: String) -> OperationResult;

    /// Create temporary file (static version).
    // Python decorators: @staticmethod
    fn create_temp_file(suffix: Option<String>, prefix: Option<String>) -> PathBuf;

    /// Create temporary directory (static version).
    // Python decorators: @staticmethod
    fn create_temp_directory(suffix: Option<String>, prefix: Option<String>) -> PathBuf;

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Interface for folder/directory operations with both static and instance methods.
///
/// Provides comprehensive directory operations including:
/// - Directory I/O operations (create, delete, list, walk)
/// - Directory metadata operations (size, permissions, contents)
/// - Directory validation and safety checks
/// - Static utility methods for directory operations
pub trait IFolder {
    /// Create directory.
    fn create(&self, parents: bool, exist_ok: bool) -> bool;

    /// Delete directory.
    fn delete(&self, recursive: bool) -> bool;

    /// List files in directory.
    fn list_files(&self, pattern: Option<String>, recursive: bool) -> Vec<PathBuf>;

    /// List subdirectories.
    fn list_directories(&self, recursive: bool) -> Vec<PathBuf>;

    /// Walk directory tree.
    fn walk(&self) -> Vec<(PathBuf, Vec<String>, Vec<String>)>;

    /// Get directory size.
    fn get_size(&self) -> i64;

    /// Check if directory is empty.
    fn is_empty(&self) -> bool;

    /// Copy directory to destination.
    fn copy_to(&self, destination: String) -> bool;

    /// Move directory to destination.
    fn move_to(&self, destination: String) -> bool;

    /// Check if directory exists.
    // Python decorators: @staticmethod
    fn exists(path: String) -> bool;

    /// Create directory.
    // Python decorators: @staticmethod
    fn create_dir(path: String, parents: bool, exist_ok: bool) -> bool;

    /// Delete directory.
    // Python decorators: @staticmethod
    fn delete_dir(path: String, recursive: bool) -> bool;

    /// List files in directory.
    // Python decorators: @staticmethod
    fn list_files_static(path: String, pattern: Option<String>, recursive: bool) -> Vec<PathBuf>;

    /// List subdirectories.
    // Python decorators: @staticmethod
    fn list_directories_static(path: String, recursive: bool) -> Vec<PathBuf>;

    /// Walk directory tree.
    // Python decorators: @staticmethod
    fn walk_static(path: String) -> Vec<(PathBuf, Vec<String>, Vec<String>)>;

    /// Get directory size.
    // Python decorators: @staticmethod
    fn get_size_static(path: String) -> i64;

    /// Check if directory is empty.
    // Python decorators: @staticmethod
    fn is_empty_static(path: String) -> bool;

    /// Copy directory.
    // Python decorators: @staticmethod
    fn copy_dir(source: String, destination: String) -> bool;

    /// Move directory.
    // Python decorators: @staticmethod
    fn move_dir(source: String, destination: String) -> bool;

    /// Get directory permissions.
    // Python decorators: @staticmethod
    fn get_permissions(path: String) -> i64;

    /// Check if directory is readable.
    // Python decorators: @staticmethod
    fn is_readable(path: String) -> bool;

    /// Check if directory is writable.
    // Python decorators: @staticmethod
    fn is_writable(path: String) -> bool;

    /// Check if directory is executable.
    // Python decorators: @staticmethod
    fn is_executable(path: String) -> bool;

}

// ============================================================================
// ============================================================================
/// Interface for path operations with both static and instance methods.
///
/// Provides comprehensive path operations including:
/// - Path manipulation (resolve, normalize, join, split)
/// - Path validation and safety checks
/// - Static utility methods for path operations
pub trait IPath {
    /// Normalize path.
    // Python decorators: @staticmethod
    fn normalize(path: String) -> PathBuf;

    /// Resolve path.
    // Python decorators: @staticmethod
    fn resolve(path: String) -> PathBuf;

    /// Get absolute path.
    // Python decorators: @staticmethod
    fn absolute(path: String) -> PathBuf;

    /// Get relative path.
    // Python decorators: @staticmethod
    fn relative(path: String, start: Option<String>) -> PathBuf;

    /// Join paths.
    // Python decorators: @staticmethod
    fn join() -> PathBuf;

    /// Split path into directory and filename.
    // Python decorators: @staticmethod
    fn split(path: String) -> (PathBuf, String);

    /// Get file extension.
    // Python decorators: @staticmethod
    fn get_extension(path: String) -> String;

    /// Get file stem (name without extension).
    // Python decorators: @staticmethod
    fn get_stem(path: String) -> String;

    /// Get file/directory name.
    // Python decorators: @staticmethod
    fn get_name(path: String) -> String;

    /// Get parent directory.
    // Python decorators: @staticmethod
    fn get_parent(path: String) -> PathBuf;

    /// Check if path is absolute.
    // Python decorators: @staticmethod
    fn is_absolute(path: String) -> bool;

    /// Check if path is relative.
    // Python decorators: @staticmethod
    fn is_relative(path: String) -> bool;

    /// Get path parts.
    // Python decorators: @staticmethod
    fn get_parts(path: String) -> ();

    /// Check if path matches pattern.
    // Python decorators: @staticmethod
    fn match(path: String, pattern: String) -> bool;

    /// Get path with new suffix.
    // Python decorators: @staticmethod
    fn with_suffix(path: String, suffix: String) -> PathBuf;

    /// Get path with new name.
    // Python decorators: @staticmethod
    fn with_name(path: String, name: String) -> PathBuf;

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Interface for stream operations with both static and instance methods.
///
/// Provides comprehensive stream operations including:
/// - Stream I/O operations (read, write, seek, tell)
/// - Stream validation and safety checks
/// - Static utility methods for stream operations
pub trait IStream {
    /// Read from stream.
    fn read(&self, size: Option<i64>) -> String;

    /// Write to stream.
    fn write(&self, data: String) -> i64;

    /// Seek stream position.
    fn seek(&self, position: i64, whence: i64) -> i64;

    /// Get current stream position.
    fn tell(&self) -> i64;

    /// Flush stream buffer.
    fn flush(&self) -> ();

    /// Close stream.
    fn close(&self) -> ();

    /// Open file as stream.
    // Python decorators: @staticmethod
    fn open_file(path: String, mode: String, encoding: Option<String>) -> TextIO;

    /// Check if stream is closed.
    // Python decorators: @staticmethod
    fn is_closed(stream: TextIO) -> bool;

    /// Check if stream is readable.
    // Python decorators: @staticmethod
    fn readable(stream: TextIO) -> bool;

    /// Check if stream is writable.
    // Python decorators: @staticmethod
    fn writable(stream: TextIO) -> bool;

    /// Check if stream is seekable.
    // Python decorators: @staticmethod
    fn seekable(stream: TextIO) -> bool;

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Interface for async I/O operations with both static and instance methods.
///
/// Provides comprehensive async I/O operations including:
/// - Async file operations (aread, awrite, aseek, atell)
/// - Async stream operations
/// - Static utility methods for async operations
pub trait IAsyncIO {
    /// Async read operation.
    async fn aread(&self, size: Option<i64>) -> String;

    /// Async write operation.
    async fn awrite(&self, data: String) -> i64;

    /// Async seek operation.
    async fn aseek(&self, position: i64, whence: i64) -> i64;

    /// Async tell operation.
    async fn atell(&self) -> i64;

    /// Async flush operation.
    async fn aflush(&self) -> ();

    /// Async close operation.
    async fn aclose(&self) -> ();

    /// Async open file.
    // Python decorators: @staticmethod
    async fn aopen_file(path: String, mode: String, encoding: Option<String>) -> serde_json::Value;

    /// Async read text file.
    // Python decorators: @staticmethod
    async fn aread_text(path: String, encoding: String) -> String;

    /// Async read binary file.
    // Python decorators: @staticmethod
    async fn aread_bytes(path: String) -> Vec<u8>;

    /// Async write text to file.
    // Python decorators: @staticmethod
    async fn awrite_text(path: String, content: String, encoding: String) -> bool;

    /// Async write bytes to file.
    // Python decorators: @staticmethod
    async fn awrite_bytes(path: String, content: Vec<u8>) -> bool;

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Interface for atomic operations with both static and instance methods.
///
/// Provides comprehensive atomic operations including:
/// - Atomic file operations (atomic write, copy, move, delete)
/// - Backup and restore operations
/// - Static utility methods for atomic operations
pub trait IAtomicOperations {
    /// Atomically write data to file.
    fn atomic_write(&self, file_path: String, data: String, backup: bool) -> OperationResult;

    /// Atomically copy file.
    fn atomic_copy(&self, source: String, destination: String) -> OperationResult;

    /// Atomically move file.
    fn atomic_move(&self, source: String, destination: String) -> OperationResult;

    /// Atomically delete file.
    fn atomic_delete(&self, file_path: String, backup: bool) -> OperationResult;

    /// Atomically rename file.
    fn atomic_rename(&self, old_path: String, new_path: String) -> OperationResult;

    /// Atomically write data to file.
    // Python decorators: @staticmethod
    fn atomic_write_static(file_path: String, data: String, backup: bool) -> OperationResult;

    /// Atomically copy file.
    // Python decorators: @staticmethod
    fn atomic_copy_static(source: String, destination: String) -> OperationResult;

    /// Atomically move file.
    // Python decorators: @staticmethod
    fn atomic_move_static(source: String, destination: String) -> OperationResult;

    /// Atomically delete file.
    // Python decorators: @staticmethod
    fn atomic_delete_static(file_path: String, backup: bool) -> OperationResult;

    /// Atomically rename file.
    // Python decorators: @staticmethod
    fn atomic_rename_static(old_path: String, new_path: String) -> OperationResult;

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Interface for backup operations with both static and instance methods.
///
/// Provides comprehensive backup operations including:
/// - Backup creation and restoration
/// - Backup management and cleanup
/// - Static utility methods for backup operations
pub trait IBackupOperations {
    /// Create backup of file or directory.
    fn create_backup(&self, source: String, backup_dir: String) -> Option<PathBuf>;

    /// Restore from backup.
    fn restore_backup(&self, backup_path: String, target: String) -> OperationResult;

    /// List available backups.
    fn list_backups(&self, backup_dir: String) -> Vec<PathBuf>;

    /// Cleanup old backups.
    fn cleanup_backups(&self, backup_dir: String, max_age_days: i64) -> i64;

    /// Verify backup integrity.
    fn verify_backup(&self, backup_path: String) -> bool;

    /// Create backup of file or directory.
    // Python decorators: @staticmethod
    fn create_backup_static(source: String, backup_dir: String) -> Option<PathBuf>;

    /// Restore from backup.
    // Python decorators: @staticmethod
    fn restore_backup_static(backup_path: String, target: String) -> OperationResult;

    /// List available backups.
    // Python decorators: @staticmethod
    fn list_backups_static(backup_dir: String) -> Vec<PathBuf>;

    /// Cleanup old backups.
    // Python decorators: @staticmethod
    fn cleanup_backups_static(backup_dir: String, max_age_days: i64) -> i64;

    /// Verify backup integrity.
    // Python decorators: @staticmethod
    fn verify_backup_static(backup_path: String) -> bool;

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Interface for temporary operations with both static and instance methods.
///
/// Provides comprehensive temporary operations including:
/// - Temporary file and directory creation
/// - Temporary resource cleanup
/// - Static utility methods for temporary operations
pub trait ITemporaryOperations {
    /// Create temporary file.
    fn create_temp_file(&self, suffix: Option<String>, prefix: Option<String>) -> PathBuf;

    /// Create temporary directory.
    fn create_temp_directory(&self, suffix: Option<String>, prefix: Option<String>) -> PathBuf;

    /// Cleanup temporary file or directory.
    fn cleanup_temp(&self, path: String) -> bool;

    /// Cleanup all temporary files and directories.
    fn cleanup_all_temp(&self) -> i64;

    /// Create temporary file.
    // Python decorators: @staticmethod
    fn create_temp_file_static(suffix: Option<String>, prefix: Option<String>) -> PathBuf;

    /// Create temporary directory.
    // Python decorators: @staticmethod
    fn create_temp_directory_static(suffix: Option<String>, prefix: Option<String>) -> PathBuf;

    /// Cleanup temporary file or directory.
    // Python decorators: @staticmethod
    fn cleanup_temp_static(path: String) -> bool;

    /// Get temporary base directory.
    // Python decorators: @staticmethod
    fn get_temp_base_dir() -> PathBuf;

    /// Check if path is temporary.
    // Python decorators: @staticmethod
    fn is_temp(path: String) -> bool;

}

/// Unified I/O interface combining all existing I/O capabilities.
///
/// This is the unified interface for all input/output operations across XWSystem.
/// It combines all existing I/O interfaces into a single, comprehensive interface
/// that provides complete I/O functionality for any data source.
///
/// Features:
/// - File operations (read, write, save, load)
/// - Directory operations (create, delete, list, walk)
/// - Path operations (resolve, normalize, join, split)
/// - Stream operations (open, read, write, seek)
/// - Async operations (async read/write, async streams)
/// - Atomic operations (atomic write, copy, move, delete)
/// - Backup operations (create, restore, list, cleanup)
/// - Temporary operations (create temp files/dirs, cleanup)
///
/// This interface follows the xwsystem pattern of combining existing interfaces
/// rather than creating new abstractions, maximizing code reuse and maintaining
/// backward compatibility.
pub trait IUnifiedIO: IFile + IFolder + IPath + IStream + IAsyncIO + IAtomicOperations + IBackupOperations + ITemporaryOperations {
}

/// File Manager interface for comprehensive file operations.
///
/// This interface combines file, directory, path, atomic, backup, and temporary
/// operations to provide a complete file management solution. It's designed
/// to handle any file type (docx, json, photo, movie, etc.) with intelligent
/// format detection and appropriate handling.
///
/// Features:
/// - Universal file type support (any format)
/// - Intelligent format detection
/// - Atomic file operations
/// - Backup and restore capabilities
/// - Temporary file management
/// - Path validation and normalization
/// - Directory operations
/// - File metadata and permissions
///
/// This interface is specifically designed for file management tasks where
/// you need to handle various file types without knowing the specific format
/// in advance.
pub trait IFileManager: IFile + IFolder + IPath + IAtomicOperations + IBackupOperations + ITemporaryOperations {
}

/// Universal data source interface for various data sources.
pub trait IDataSource {
    /// Read data from source.
    fn read(&self) -> T;

    /// Write data to source.
    fn write(&self, data: T) -> ();

}

/// Paged data source interface for large data sets.
pub trait IPagedDataSource {
    /// Read a specific page of data.
    fn read_page(&self, page_number: i64) -> Vec<T>;

    /// Get total number of pages.
    fn get_page_count(&self) -> i64;

}

/// Codec-integrated IO interface with source type T and result type R.
pub trait ICodecIO {
    /// Read and decode data using specified codec.
    fn read_as(&self, codec: String) -> ();

    /// Encode and write data using specified codec.
    fn write_as(&self, data: String, codec: String) -> ();

}

/// Paged codec-integrated IO interface with source type T and result type R.
pub trait IPagedCodecIO {
    /// Read and decode a page using specified codec.
    fn read_page_as(&self, page_number: i64, codec: String) -> ();

}

/// Interface for watching file system changes.
pub trait IFileWatcher {
    /// Start watching a path.
    fn watch(&self, path: String) -> ();

    /// Stop watching.
    fn stop(&self) -> ();

}

/// Interface for file locking.
pub trait IFileLock {
    /// Acquire the lock.
    fn acquire(&self) -> bool;

    /// Release the lock.
    fn release(&self) -> ();

}

/// Virtual file system interface.
pub trait IFileSystem {
    /// Read file contents.
    fn read(&self, path: String) -> Vec<u8>;

    /// Write file contents.
    fn write(&self, path: String, data: Vec<u8>) -> ();

    /// Check if path exists.
    fn exists(&self, path: String) -> bool;

}

/// Interface for archive format handlers.
///
/// Each format (ZIP, TAR, 7Z, RAR) implements this.
pub trait IArchiveFormat {
    /// Unique format identifier.
    // Python decorators: @property
    fn format_id(&self) -> &str;

    /// Supported file extensions.
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

    /// Supported MIME types.
    // Python decorators: @property
    fn mime_types(&self) -> Vec<String>;

    /// Create archive from files.
    fn create(&self, files: Vec<PathBuf>, output: PathBuf) -> ();

    /// Extract archive.
    fn extract(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>) -> Vec<PathBuf>;

    /// List archive contents.
    fn list_contents(&self, archive: PathBuf) -> Vec<String>;

    /// Add file to existing archive.
    fn add_file(&self, archive: PathBuf, file: PathBuf, arcname: Option<String>) -> ();

}

/// Interface for compression algorithms.
///
/// Each algorithm (gzip, bz2, lzma, zstd) implements this.
pub trait ICompressor {
    /// Unique algorithm identifier.
    // Python decorators: @property
    fn algorithm_id(&self) -> &str;

    /// Supported file extensions.
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

    /// Compress bytes.
    fn compress(&self, data: Vec<u8>, level: i64) -> Vec<u8>;

    /// Decompress bytes.
    fn decompress(&self, data: Vec<u8>) -> Vec<u8>;

    /// Check if this compressor can handle the data.
    fn can_handle(&self, data: Vec<u8>) -> bool;

}

/// Metadata protocol for self-describing archivers.
///
/// Like ICodecMetadata for codecs!
pub trait IArchiveMetadata {
    /// Format identifier.
    // Python decorators: @property
    fn format_id(&self) -> &str;

    /// Supported extensions.
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

    /// MIME types.
    // Python decorators: @property
    fn mime_types(&self) -> Vec<String>;

    /// Human-readable description.
    // Python decorators: @property
    fn description(&self) -> &str;

}

/// Universal codec interface for bidirectional transformation.
///
/// A codec transforms between a model (T) and its representation (R).
/// This is the minimal contract that all codecs must implement.
///
/// Type Parameters:
/// T: Model type (e.g., dict, AST, dataclass)
/// R: Representation type (bytes or str)
///
/// Examples:
/// JSON serializer:  ICodec[dict, bytes]
/// SQL formatter:    ICodec[QueryAST, str]
/// Pickle:           ICodec[Any, bytes]
/// Python unparser:  ICodec[ast.AST, str]
///
/// Design Principles:
/// - Bidirectional by default (encode/decode)
/// - Options-based configuration (not constructor pollution)
/// - Representation-type specific (bytes OR str, not both)
/// - Composable via adapters
pub trait ICodec {
    /// Encode a model to its representation.
    /// Args:
    /// value: Model instance to encode
    /// options: Format-specific encoding options (e.g., {'pretty': True})
    /// Returns:
    /// Representation (bytes or str depending on codec type)
    /// Raises:
    /// EncodeError: If encoding fails
    /// Examples:
    /// >>> codec = JsonSerializer()
    /// >>> codec.encode({"key": "value"})
    /// b'{"key":"value"}'
    /// >>> formatter = SqlFormatter()
    /// >>> formatter.encode(select_ast, options={"pretty": True})
    /// 'SELECT *\nFROM users\nWHERE id = 1'
    fn encode(&self, value: serde_json::Value) -> Vec<u8>;

    /// Decode a representation to a model.
    /// Args:
    /// repr: Representation to decode (bytes or str)
    /// options: Format-specific decoding options (e.g., {'strict': False})
    /// Returns:
    /// Model instance
    /// Raises:
    /// DecodeError: If decoding fails
    /// Examples:
    /// >>> codec = JsonSerializer()
    /// >>> codec.decode(b'{"key":"value"}')
    /// {'key': 'value'}
    /// >>> formatter = SqlFormatter()
    /// >>> formatter.decode('SELECT * FROM users')
    /// QueryAST(...)
    fn decode(&self, repr: Vec<u8>) -> serde_json::Value;

}

/// Metadata protocol for codec discovery and registration.
///
/// Codecs that implement this protocol can self-register and be
/// discovered by the registry system with no hardcoding.
///
/// Example:
/// >>> class JsonCodec:
/// ...     codec_id = "json"
/// ...     media_types = ["application/json", "text/json"]
/// ...     file_extensions = [".json", ".jsonl"]
/// ...     aliases = ["JSON"]
/// ...
/// ...     def capabilities(self):
/// ...         return CodecCapability.BIDIRECTIONAL | CodecCapability.TEXT
pub trait ICodecMetadata {
    /// Unique codec identifier.
    /// Should be lowercase, alphanumeric + dash/underscore.
    /// Examples:
    /// - "json"
    /// - "sql"
    /// - "protobuf"
    /// - "python-ast"
    // Python decorators: @property
    fn codec_id(&self) -> &str;

    /// Supported media types / content types (RFC 2046).
    /// Used for content negotiation and HTTP Content-Type headers.
    /// Examples:
    /// - JSON: ["application/json", "text/json"]
    /// - SQL: ["application/sql", "text/x-sql"]
    /// - Protobuf: ["application/protobuf", "application/x-protobuf"]
    // Python decorators: @property
    fn media_types(&self) -> Vec<String>;

    /// Supported file extensions (with leading dot).
    /// Used for auto-detection from file paths.
    /// Examples:
    /// - JSON: [".json", ".jsonl"]
    /// - SQL: [".sql", ".ddl", ".dml"]
    /// - Python: [".py", ".pyi"]
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

    /// Alternative names for this codec.
    /// Used for flexible lookup (case-insensitive matching).
    /// Examples:
    /// - JSON: ["json", "JSON"]
    /// - SQL: ["sql", "SQL", "structured-query"]
    // Python decorators: @property
    fn aliases(&self) -> Vec<String>;

    /// Get capabilities supported by this codec.
    /// Returns:
    /// Flag combination of supported features
    /// Example:
    /// >>> codec.capabilities()
    /// CodecCapability.BIDIRECTIONAL | CodecCapability.TEXT | CodecCapability.SCHEMA
    fn capabilities(&self) -> CodecCapability;

}

/// Interface for atomic file write operations.
pub trait IAtomicWriter {
    /// Write data atomically.
    fn write(&self, data: Vec<u8>) -> i64;

}

/// Archive codec interface - operates in MEMORY on ANY data.
///
/// Extends ICodec to provide dual API:
/// - encode()/decode() - Low-level codec operations
/// - compress()/extract() - User-friendly archive operations
///
/// Type: ICodec[T, bytes] where T can be:
/// - bytes (raw data)
/// - str (text data)
/// - dict/list (structured data)
/// - Any (objects)
///
/// NOT limited to file paths - works on data in RAM!
pub trait IArchiver: ICodec {
    /// Compress data to archive bytes (in RAM).
    ///
    /// Delegates to encode() internally.
    fn compress(&self, data: serde_json::Value, options: HashMap<String, String>) -> Vec<u8> {
        // Default implementation delegates to encode
        self.encode(data)
    }

    /// Extract archive bytes to data (in RAM).
    ///
    /// Delegates to decode() internally.
    fn extract(&self, archive_bytes: Vec<u8>, options: HashMap<String, String>) -> serde_json::Value {
        // Default implementation delegates to decode
        self.decode(archive_bytes)
    }
}

/// Archive FILE interface - operates on DISK.
///
/// Extends IFile for file operations.
/// USES IArchiver internally for compression (composition).
///
/// This handles:
/// - File I/O with archive files on disk
/// - Adding/extracting files to/from archives
/// - Archive file management
pub trait IArchiveFile: IFile {
    /// Add files to archive (uses archiver.compress internally).
    fn add_files(&self, files: Vec<PathBuf>, options: HashMap<String, String>);

    /// Extract archive to destination (uses archiver.extract internally).
    fn extract_to(&self, dest: PathBuf, options: HashMap<String, String>) -> Vec<PathBuf>;

    /// List files in archive.
    fn list_contents(&self) -> Vec<String>;

    /// Get the underlying archiver codec.
    fn get_archiver(&self) -> Box<dyn IArchiver>;
}

/// Interface for raw compression operations (gzip, bz2, lzma, etc.).
///
/// This is for compressing RAW BYTES (not archives).
/// Separate from IArchiver which handles archive formats.
pub trait ICompression {
    /// Compress raw bytes.
    fn compress(&self, data: Vec<u8>) -> Vec<u8>;

    /// Decompress raw bytes.
    fn decompress(&self, data: Vec<u8>) -> Vec<u8>;

}

/// Interface for path validation and security checks.
pub trait IPathValidator {
    /// Validate path safety.
    fn validate_path(&self, path: String) -> bool;

    /// Check if path is safe to use.
    fn is_safe_path(&self, path: String) -> bool;

    /// Normalize and resolve path.
    fn normalize_path(&self, path: String) -> PathBuf;

}

/// Interface for file data sources.
pub trait IFileSource {
    /// Source URI.
    // Python decorators: @property
    fn uri(&self) -> &str;

    /// URI scheme.
    // Python decorators: @property
    fn scheme(&self) -> &str;

    /// Read entire file content.
    fn read(&self) -> Vec<u8>;

    /// Write entire content to file.
    fn write(&self, data: Vec<u8>) -> ();

    /// Check if file exists.
    fn exists(&self) -> bool;

    /// Delete file.
    fn delete(&self) -> ();

}

/// Interface for paged file sources.
pub trait IPagedSource {
    /// Total file size in bytes.
    // Python decorators: @property
    fn total_size(&self) -> i64;

    /// Read specific page.
    fn read_page(&self, page: i64, page_size: i64) -> Vec<u8>;

    /// Read chunk by byte offset.
    fn read_chunk(&self, offset: i64, size: i64) -> Vec<u8>;

    /// Iterate over pages.
    fn iter_pages(&self, page_size: i64) -> Box<dyn Iterator<Item = Vec<u8>>>;

    /// Iterate over chunks.
    fn iter_chunks(&self, chunk_size: i64) -> Box<dyn Iterator<Item = Vec<u8>>>;

}

/// Strategy interface for paging through file data.
///
/// Enables pluggable paging algorithms:
/// - BytePagingStrategy: Page by byte offsets
/// - LinePagingStrategy: Page by line counts
/// - RecordPagingStrategy: Page by record boundaries (CSV, JSONL)
/// - SmartPagingStrategy: Adaptive paging based on content
pub trait IPagingStrategy {
    /// Unique strategy identifier.
    // Python decorators: @property
    fn strategy_id(&self) -> &str;

    /// Read specific page using this strategy.
    /// Args:
    /// file_path: Path to file
    /// page: Page number (0-based)
    /// page_size: Items per page (interpretation depends on strategy)
    /// mode: File mode
    /// encoding: Text encoding (for text mode)
    /// **options: Strategy-specific options
    /// Returns:
    /// Page content
    fn read_page(&self, file_path: Path, page: i64, page_size: i64, mode: String, encoding: Option<String>) -> Vec<u8>;

    /// Iterate over pages using this strategy.
    /// Args:
    /// file_path: Path to file
    /// page_size: Items per page
    /// mode: File mode
    /// encoding: Text encoding
    /// **options: Strategy-specific options
    /// Yields:
    /// Page content
    fn iter_pages(&self, file_path: Path, page_size: i64, mode: String, encoding: Option<String>) -> Box<dyn Iterator<Item = Vec<u8>>>;

}

/// Interface for virtual filesystem operations.
pub trait IVirtualFS {
    /// URI scheme.
    // Python decorators: @property
    fn scheme(&self) -> &str;

    /// Check if path exists.
    fn exists(&self, path: String) -> bool;

    /// Check if path is file.
    fn is_file(&self, path: String) -> bool;

}

/// Interface for folder operations.
pub trait IFolderSource {
    /// Create directory.
    fn create(&self, parents: bool, exist_ok: bool) -> bool;

    /// Delete directory.
    fn delete(&self, recursive: bool) -> bool;

    /// List files in directory.
    fn list_files(&self, pattern: Option<String>, recursive: bool) -> Vec<PathBuf>;

}

/// Interface for I/O managers.
pub trait IIOManager {
    /// Open resource.
    fn open(&self) -> ();

    /// Close resource.
    fn close(&self) -> ();

}

