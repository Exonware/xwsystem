#exonware/xwsystem/src/exonware/xwsystem/io/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.37
Generation Date: September 04, 2025
IO module contracts - interfaces and enums for input/output operations.
"""

from __future__ import annotations
from typing import Protocol, runtime_checkable
from typing import Any, BinaryIO, TextIO, Protocol, runtime_checkable

from collections.abc import AsyncGenerator, Callable, Iterator
from typing import TypeAlias
from pathlib import Path
# Type aliases for codec options
# Root cause: Migrating to Python 3.12 built-in generic syntax for consistency
# Priority #3: Maintainability - Modern type annotations improve code clarity
EncodeOptions: TypeAlias = dict[str, Any]
DecodeOptions: TypeAlias = dict[str, Any]
Serializer: TypeAlias = 'ICodec[Any, bytes]'
Formatter: TypeAlias = 'ICodec[Any, str]'
# Import enums from types module
from .defs import (
    FileMode,
    FileType,
    PathType,
    OperationResult,
    LockType,
    CodecCapability,
)
# ============================================================================
# FILE INTERFACES
# ============================================================================
@runtime_checkable

class IFile(Protocol):
    """
    Interface for file operations with both static and instance methods.
    Provides comprehensive file operations including:
    - File I/O operations (read, write, save, load)
    - File metadata operations (size, permissions, timestamps)
    - File validation and safety checks
    - Static utility methods for file operations
    """
    # ============================================================================
    # INSTANCE METHODS
    # ============================================================================

    def open(self, mode: FileMode = FileMode.READ) -> None:
        """Open file with specified mode."""
        ...

    def read(self, size: int | None = None) -> str | bytes:
        """Read from file."""
        ...

    def write(self, data: str | bytes) -> int:
        """Write to file."""
        ...

    def close(self) -> None:
        """Close file."""
        ...

    def save(self, data: Any, **kwargs) -> bool:
        """Save data to file."""
        ...

    def load(self, **kwargs) -> Any:
        """Load data from file."""
        ...

    def save_as(self, path: str | Path, data: Any, **kwargs) -> bool:
        """Save data to specific path."""
        ...

    def to_file(self, path: str | Path, **kwargs) -> bool:
        """Write current object to file."""
        ...

    def from_file(self, path: str | Path, **kwargs) -> IFile:
        """Load object from file."""
        ...
    # ============================================================================
    # STATIC METHODS
    # ============================================================================
    @staticmethod

    def exists(path: str | Path) -> bool:
        """Check if file exists."""
        ...
    @staticmethod

    def size(path: str | Path) -> int:
        """Get file size."""
        ...
    @staticmethod

    def delete(path: str | Path) -> bool:
        """Delete file."""
        ...
    @staticmethod

    def copy(source: str | Path, destination: str | Path) -> bool:
        """Copy file."""
        ...
    @staticmethod

    def move(source: str | Path, destination: str | Path) -> bool:
        """Move file."""
        ...
    @staticmethod

    def rename(old_path: str | Path, new_path: str | Path) -> bool:
        """Rename file."""
        ...
    @staticmethod

    def get_modified_time(path: str | Path) -> float:
        """Get file modification time."""
        ...
    @staticmethod

    def get_created_time(path: str | Path) -> float:
        """Get file creation time."""
        ...
    @staticmethod

    def get_permissions(path: str | Path) -> int:
        """Get file permissions."""
        ...
    @staticmethod

    def is_readable(path: str | Path) -> bool:
        """Check if file is readable."""
        ...
    @staticmethod

    def is_writable(path: str | Path) -> bool:
        """Check if file is writable."""
        ...
    @staticmethod

    def is_executable(path: str | Path) -> bool:
        """Check if file is executable."""
        ...
    @staticmethod

    def read_text(path: str | Path, encoding: str = 'utf-8') -> str:
        """Read file as text."""
        ...
    @staticmethod

    def read_bytes(path: str | Path) -> bytes:
        """Read file as bytes."""
        ...
    @staticmethod

    def write_text(path: str | Path, content: str, encoding: str = 'utf-8') -> bool:
        """Write text to file."""
        ...
    @staticmethod

    def write_bytes(path: str | Path, content: bytes) -> bool:
        """Write bytes to file."""
        ...
    @staticmethod

    def safe_read_text(path: str | Path, encoding: str = 'utf-8') -> str | None:
        """Safely read text file, returning None on error."""
        ...
    @staticmethod

    def safe_read_bytes(path: str | Path) -> bytes | None:
        """Safely read binary file, returning None on error."""
        ...
    @staticmethod

    def safe_write_text(path: str | Path, content: str, encoding: str = 'utf-8') -> bool:
        """Safely write text to file."""
        ...
    @staticmethod

    def safe_write_bytes(path: str | Path, content: bytes) -> bool:
        """Safely write bytes to file."""
        ...
    # ============================================================================
    # STATIC UTILITY METHODS (File Manager Features)
    # ============================================================================
    @staticmethod

    def atomic_write(file_path: str | Path, data: str | bytes, 
                    backup: bool = True) -> OperationResult:
        """Atomically write data to file (static version)."""
        ...
    @staticmethod

    def atomic_copy(source: str | Path, destination: str | Path) -> OperationResult:
        """Atomically copy file (static version)."""
        ...
    @staticmethod

    def atomic_move(source: str | Path, destination: str | Path) -> OperationResult:
        """Atomically move file (static version)."""
        ...
    @staticmethod

    def atomic_delete(file_path: str | Path, backup: bool = True) -> OperationResult:
        """Atomically delete file (static version)."""
        ...
    @staticmethod

    def create_backup(source: str | Path, backup_dir: str | Path) -> Path | None:
        """Create backup of file (static version)."""
        ...
    @staticmethod

    def restore_backup(backup_path: str | Path, target: str | Path) -> OperationResult:
        """Restore from backup (static version)."""
        ...
    @staticmethod

    def create_temp_file(suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary file (static version)."""
        ...
    @staticmethod

    def create_temp_directory(suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary directory (static version)."""
        ...
# ============================================================================
# FOLDER INTERFACES
# ============================================================================
@runtime_checkable

class IFolder(Protocol):
    """
    Interface for folder/directory operations with both static and instance methods.
    Provides comprehensive directory operations including:
    - Directory I/O operations (create, delete, list, walk)
    - Directory metadata operations (size, permissions, contents)
    - Directory validation and safety checks
    - Static utility methods for directory operations
    """
    # ============================================================================
    # INSTANCE METHODS
    # ============================================================================

    def create(self, parents: bool = True, exist_ok: bool = True) -> bool:
        """Create directory."""
        ...

    def delete(self, recursive: bool = False) -> bool:
        """Delete directory."""
        ...

    def list_files(self, pattern: str | None = None, recursive: bool = False) -> list[Path]:
        """List files in directory."""
        ...

    def list_directories(self, recursive: bool = False) -> list[Path]:
        """List subdirectories."""
        ...

    def walk(self) -> list[tuple[Path, list[str], list[str]]]:
        """Walk directory tree."""
        ...

    def get_size(self) -> int:
        """Get directory size."""
        ...

    def is_empty(self) -> bool:
        """Check if directory is empty."""
        ...

    def copy_to(self, destination: str | Path) -> bool:
        """Copy directory to destination."""
        ...

    def move_to(self, destination: str | Path) -> bool:
        """Move directory to destination."""
        ...
    # ============================================================================
    # STATIC METHODS
    # ============================================================================
    @staticmethod

    def exists(path: str | Path) -> bool:
        """Check if directory exists."""
        ...
    @staticmethod

    def create_dir(path: str | Path, parents: bool = True, exist_ok: bool = True) -> bool:
        """Create directory."""
        ...
    @staticmethod

    def delete_dir(path: str | Path, recursive: bool = False) -> bool:
        """Delete directory."""
        ...
    @staticmethod

    def list_files_static(path: str | Path, pattern: str | None = None, recursive: bool = False) -> list[Path]:
        """List files in directory."""
        ...
    @staticmethod

    def list_directories_static(path: str | Path, recursive: bool = False) -> list[Path]:
        """List subdirectories."""
        ...
    @staticmethod

    def walk_static(path: str | Path) -> list[tuple[Path, list[str], list[str]]]:
        """Walk directory tree."""
        ...
    @staticmethod

    def get_size_static(path: str | Path) -> int:
        """Get directory size."""
        ...
    @staticmethod

    def is_empty_static(path: str | Path) -> bool:
        """Check if directory is empty."""
        ...
    @staticmethod

    def copy_dir(source: str | Path, destination: str | Path) -> bool:
        """Copy directory."""
        ...
    @staticmethod

    def move_dir(source: str | Path, destination: str | Path) -> bool:
        """Move directory."""
        ...
    @staticmethod

    def get_permissions(path: str | Path) -> int:
        """Get directory permissions."""
        ...
    @staticmethod

    def is_readable(path: str | Path) -> bool:
        """Check if directory is readable."""
        ...
    @staticmethod

    def is_writable(path: str | Path) -> bool:
        """Check if directory is writable."""
        ...
    @staticmethod

    def is_executable(path: str | Path) -> bool:
        """Check if directory is executable."""
        ...
# ============================================================================
# PATH INTERFACES
# ============================================================================
@runtime_checkable

class IPath(Protocol):
    """
    Interface for path operations with both static and instance methods.
    Provides comprehensive path operations including:
    - Path manipulation (resolve, normalize, join, split)
    - Path validation and safety checks
    - Static utility methods for path operations
    """
    # ============================================================================
    # STATIC METHODS
    # ============================================================================
    @staticmethod

    def normalize(path: str | Path) -> Path:
        """Normalize path."""
        ...
    @staticmethod

    def resolve(path: str | Path) -> Path:
        """Resolve path."""
        ...
    @staticmethod

    def absolute(path: str | Path) -> Path:
        """Get absolute path."""
        ...
    @staticmethod

    def relative(path: str | Path, start: str | Path | None = None) -> Path:
        """Get relative path."""
        ...
    @staticmethod

    def join(*paths: str | Path) -> Path:
        """Join paths."""
        ...
    @staticmethod

    def split(path: str | Path) -> tuple[Path, str]:
        """Split path into directory and filename."""
        ...
    @staticmethod

    def get_extension(path: str | Path) -> str:
        """Get file extension."""
        ...
    @staticmethod

    def get_stem(path: str | Path) -> str:
        """Get file stem (name without extension)."""
        ...
    @staticmethod

    def get_name(path: str | Path) -> str:
        """Get file/directory name."""
        ...
    @staticmethod

    def get_parent(path: str | Path) -> Path:
        """Get parent directory."""
        ...
    @staticmethod

    def is_absolute(path: str | Path) -> bool:
        """Check if path is absolute."""
        ...
    @staticmethod

    def is_relative(path: str | Path) -> bool:
        """Check if path is relative."""
        ...
    @staticmethod

    def get_parts(path: str | Path) -> tuple:
        """Get path parts."""
        ...
    @staticmethod

    def match(path: str | Path, pattern: str) -> bool:
        """Check if path matches pattern."""
        ...
    @staticmethod

    def with_suffix(path: str | Path, suffix: str) -> Path:
        """Get path with new suffix."""
        ...
    @staticmethod

    def with_name(path: str | Path, name: str) -> Path:
        """Get path with new name."""
        ...
# ============================================================================
# STREAM INTERFACES
# ============================================================================
@runtime_checkable

class IStream(Protocol):
    """
    Interface for stream operations with both static and instance methods.
    Provides comprehensive stream operations including:
    - Stream I/O operations (read, write, seek, tell)
    - Stream validation and safety checks
    - Static utility methods for stream operations
    """
    # ============================================================================
    # INSTANCE METHODS
    # ============================================================================

    def read(self, size: int | None = None) -> str | bytes:
        """Read from stream."""
        ...

    def write(self, data: str | bytes) -> int:
        """Write to stream."""
        ...

    def seek(self, position: int, whence: int = 0) -> int:
        """Seek stream position."""
        ...

    def tell(self) -> int:
        """Get current stream position."""
        ...

    def flush(self) -> None:
        """Flush stream buffer."""
        ...

    def close(self) -> None:
        """Close stream."""
        ...
    # ============================================================================
    # STATIC METHODS
    # ============================================================================
    @staticmethod

    def open_file(path: str | Path, mode: str = 'r', encoding: str | None = None) -> TextIO | BinaryIO:
        """Open file as stream."""
        ...
    @staticmethod

    def is_closed(stream: TextIO | BinaryIO) -> bool:
        """Check if stream is closed."""
        ...
    @staticmethod

    def readable(stream: TextIO | BinaryIO) -> bool:
        """Check if stream is readable."""
        ...
    @staticmethod

    def writable(stream: TextIO | BinaryIO) -> bool:
        """Check if stream is writable."""
        ...
    @staticmethod

    def seekable(stream: TextIO | BinaryIO) -> bool:
        """Check if stream is seekable."""
        ...
# ============================================================================
# ASYNC I/O INTERFACES
# ============================================================================
@runtime_checkable

class IAsyncIO(Protocol):
    """
    Interface for async I/O operations with both static and instance methods.
    Provides comprehensive async I/O operations including:
    - Async file operations (aread, awrite, aseek, atell)
    - Async stream operations
    - Static utility methods for async operations
    """
    # ============================================================================
    # INSTANCE METHODS
    # ============================================================================

    async def aread(self, size: int | None = None) -> str | bytes:
        """Async read operation."""
        ...

    async def awrite(self, data: str | bytes) -> int:
        """Async write operation."""
        ...

    async def aseek(self, position: int, whence: int = 0) -> int:
        """Async seek operation."""
        ...

    async def atell(self) -> int:
        """Async tell operation."""
        ...

    async def aflush(self) -> None:
        """Async flush operation."""
        ...

    async def aclose(self) -> None:
        """Async close operation."""
        ...
    # ============================================================================
    # STATIC METHODS
    # ============================================================================
    @staticmethod

    async def aopen_file(path: str | Path, mode: str = 'r', encoding: str | None = None) -> Any:
        """Async open file."""
        ...
    @staticmethod

    async def aread_text(path: str | Path, encoding: str = 'utf-8') -> str:
        """Async read text file."""
        ...
    @staticmethod

    async def aread_bytes(path: str | Path) -> bytes:
        """Async read binary file."""
        ...
    @staticmethod

    async def awrite_text(path: str | Path, content: str, encoding: str = 'utf-8') -> bool:
        """Async write text to file."""
        ...
    @staticmethod

    async def awrite_bytes(path: str | Path, content: bytes) -> bool:
        """Async write bytes to file."""
        ...
# ============================================================================
# ATOMIC OPERATIONS INTERFACES
# ============================================================================
@runtime_checkable

class IAtomicOperations(Protocol):
    """
    Interface for atomic operations with both static and instance methods.
    Provides comprehensive atomic operations including:
    - Atomic file operations (atomic write, copy, move, delete)
    - Backup and restore operations
    - Static utility methods for atomic operations
    """
    # ============================================================================
    # INSTANCE METHODS
    # ============================================================================

    def atomic_write(self, file_path: str | Path, data: str | bytes, 
                    backup: bool = True) -> OperationResult:
        """Atomically write data to file."""
        ...

    def atomic_copy(self, source: str | Path, destination: str | Path) -> OperationResult:
        """Atomically copy file."""
        ...

    def atomic_move(self, source: str | Path, destination: str | Path) -> OperationResult:
        """Atomically move file."""
        ...

    def atomic_delete(self, file_path: str | Path, backup: bool = True) -> OperationResult:
        """Atomically delete file."""
        ...

    def atomic_rename(self, old_path: str | Path, new_path: str | Path) -> OperationResult:
        """Atomically rename file."""
        ...
    # ============================================================================
    # STATIC METHODS
    # ============================================================================
    @staticmethod

    def atomic_write_static(file_path: str | Path, data: str | bytes, 
                           backup: bool = True) -> OperationResult:
        """Atomically write data to file."""
        ...
    @staticmethod

    def atomic_copy_static(source: str | Path, destination: str | Path) -> OperationResult:
        """Atomically copy file."""
        ...
    @staticmethod

    def atomic_move_static(source: str | Path, destination: str | Path) -> OperationResult:
        """Atomically move file."""
        ...
    @staticmethod

    def atomic_delete_static(file_path: str | Path, backup: bool = True) -> OperationResult:
        """Atomically delete file."""
        ...
    @staticmethod

    def atomic_rename_static(old_path: str | Path, new_path: str | Path) -> OperationResult:
        """Atomically rename file."""
        ...
# ============================================================================
# BACKUP OPERATIONS INTERFACES
# ============================================================================
@runtime_checkable

class IBackupOperations(Protocol):
    """
    Interface for backup operations with both static and instance methods.
    Provides comprehensive backup operations including:
    - Backup creation and restoration
    - Backup management and cleanup
    - Static utility methods for backup operations
    """
    # ============================================================================
    # INSTANCE METHODS
    # ============================================================================

    def create_backup(self, source: str | Path, backup_dir: str | Path) -> Path | None:
        """Create backup of file or directory."""
        ...

    def restore_backup(self, backup_path: str | Path, target: str | Path) -> OperationResult:
        """Restore from backup."""
        ...

    def list_backups(self, backup_dir: str | Path) -> list[Path]:
        """List available backups."""
        ...

    def cleanup_backups(self, backup_dir: str | Path, max_age_days: int = 30) -> int:
        """Cleanup old backups."""
        ...

    def verify_backup(self, backup_path: str | Path) -> bool:
        """Verify backup integrity."""
        ...
    # ============================================================================
    # STATIC METHODS
    # ============================================================================
    @staticmethod

    def create_backup_static(source: str | Path, backup_dir: str | Path) -> Path | None:
        """Create backup of file or directory."""
        ...
    @staticmethod

    def restore_backup_static(backup_path: str | Path, target: str | Path) -> OperationResult:
        """Restore from backup."""
        ...
    @staticmethod

    def list_backups_static(backup_dir: str | Path) -> list[Path]:
        """List available backups."""
        ...
    @staticmethod

    def cleanup_backups_static(backup_dir: str | Path, max_age_days: int = 30) -> int:
        """Cleanup old backups."""
        ...
    @staticmethod

    def verify_backup_static(backup_path: str | Path) -> bool:
        """Verify backup integrity."""
        ...
# ============================================================================
# TEMPORARY OPERATIONS INTERFACES
# ============================================================================
@runtime_checkable

class ITemporaryOperations(Protocol):
    """
    Interface for temporary operations with both static and instance methods.
    Provides comprehensive temporary operations including:
    - Temporary file and directory creation
    - Temporary resource cleanup
    - Static utility methods for temporary operations
    """
    # ============================================================================
    # INSTANCE METHODS
    # ============================================================================

    def create_temp_file(self, suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary file."""
        ...

    def create_temp_directory(self, suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary directory."""
        ...

    def cleanup_temp(self, path: str | Path) -> bool:
        """Cleanup temporary file or directory."""
        ...

    def cleanup_all_temp(self) -> int:
        """Cleanup all temporary files and directories."""
        ...
    # ============================================================================
    # STATIC METHODS
    # ============================================================================
    @staticmethod

    def create_temp_file_static(suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary file."""
        ...
    @staticmethod

    def create_temp_directory_static(suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary directory."""
        ...
    @staticmethod

    def cleanup_temp_static(path: str | Path) -> bool:
        """Cleanup temporary file or directory."""
        ...
    @staticmethod

    def get_temp_base_dir() -> Path:
        """Get temporary base directory."""
        ...
    @staticmethod

    def is_temp(path: str | Path) -> bool:
        """Check if path is temporary."""
        ...
# ============================================================================
# UNIFIED I/O INTERFACE
# ============================================================================


class IUnifiedIO(IFile, IFolder, IPath, IStream, IAsyncIO, IAtomicOperations, IBackupOperations, ITemporaryOperations):
    """
    Unified I/O interface combining all existing I/O capabilities.
    This is the unified interface for all input/output operations across XWSystem.
    It combines all existing I/O interfaces into a single, comprehensive interface
    that provides complete I/O functionality for any data source.
    Features:
    - File operations (read, write, save, load)
    - Directory operations (create, delete, list, walk)
    - Path operations (resolve, normalize, join, split)
    - Stream operations (open, read, write, seek)
    - Async operations (async read/write, async streams)
    - Atomic operations (atomic write, copy, move, delete)
    - Backup operations (create, restore, list, cleanup)
    - Temporary operations (create temp files/dirs, cleanup)
    This interface follows the xwsystem pattern of combining existing interfaces
    rather than creating new abstractions, maximizing code reuse and maintaining
    code reuse.
    """
    pass
# ============================================================================
# FILE MANAGER INTERFACE
# ============================================================================


class IFileManager(IFile, IFolder, IPath, IAtomicOperations, IBackupOperations, ITemporaryOperations):
    """
    File Manager interface for comprehensive file operations.
    This interface combines file, directory, path, atomic, backup, and temporary
    operations to provide a complete file management solution. It's designed
    to handle any file type (docx, json, photo, movie, etc.) with intelligent
    format detection and appropriate handling.
    Features:
    - Universal file type support (any format)
    - Intelligent format detection
    - Atomic file operations
    - Backup and restore capabilities
    - Temporary file management
    - Path validation and normalization
    - Directory operations
    - File metadata and permissions
    This interface is specifically designed for file management tasks where
    you need to handle various file types without knowing the specific format
    in advance.
    """
    pass
# ============================================================================
# DATA SOURCE INTERFACES (Used by file/, stream/)
# ============================================================================
@runtime_checkable

class IDataSource[T](Protocol):
    """Universal data source interface for various data sources."""

    def read(self) -> T:
        """Read data from source."""
        ...

    def write(self, data: T) -> None:
        """Write data to source."""
        ...
@runtime_checkable

class IPagedDataSource[T](Protocol):
    """Paged data source interface for large data sets."""

    def read_page(self, page_number: int) -> list[T]:
        """Read a specific page of data."""
        ...

    def get_page_count(self) -> int:
        """Get total number of pages."""
        ...
# ============================================================================
# CODEC-INTEGRATED IO INTERFACES (Used by stream/)
# ============================================================================
@runtime_checkable

class ICodecIO[T, R](Protocol):
    """Codec-integrated IO interface with source type T and result type R."""

    def read_as(self, codec: str):
        """Read and decode data using specified codec."""
        ...

    def write_as(self, data, codec: str) -> None:
        """Encode and write data using specified codec."""
        ...
@runtime_checkable

class IPagedCodecIO[T, R](Protocol):
    """Paged codec-integrated IO interface with source type T and result type R."""

    def read_page_as(self, page_number: int, codec: str):
        """Read and decode a page using specified codec."""
        ...
# ============================================================================
# FILE SYSTEM INTERFACES (Used by common/, filesystem/)
# ============================================================================
@runtime_checkable

class IFileWatcher(Protocol):
    """Interface for watching file system changes."""

    def watch(self, path: str | Path) -> None:
        """Start watching a path."""
        ...

    def stop(self) -> None:
        """Stop watching."""
        ...
@runtime_checkable

class IFileLock(Protocol):
    """Interface for file locking."""

    def acquire(self) -> bool:
        """Acquire the lock."""
        ...

    def release(self) -> None:
        """Release the lock."""
        ...
@runtime_checkable

class IFileSystem(Protocol):
    """Virtual file system interface."""

    def read(self, path: str) -> bytes:
        """Read file contents."""
        ...

    def write(self, path: str, data: bytes) -> None:
        """Write file contents."""
        ...

    def exists(self, path: str) -> bool:
        """Check if path exists."""
        ...
# ============================================================================
# ARCHIVE INTERFACES - MOVED TO AFTER ICodec DEFINITION
# (See line ~1400 for IArchiver, IArchiveFile, ICompression)
# ============================================================================
# ============================================================================
# SUBFOLDER CONTRACTS (Consolidated)
# ============================================================================
# From archive/
@runtime_checkable

class IArchiveFormat(Protocol):
    """
    Interface for archive format handlers.
    Each format (ZIP, TAR, 7Z, RAR) implements this.
    """
    @property

    def format_id(self) -> str:
        """Unique format identifier."""
        ...
    @property

    def file_extensions(self) -> list[str]:
        """Supported file extensions."""
        ...
    @property

    def mime_types(self) -> list[str]:
        """Supported MIME types."""
        ...

    def create(self, files: list[Path], output: Path, **opts) -> None:
        """Create archive from files."""
        ...

    def extract(self, archive: Path, output_dir: Path, members: list[str] | None = None, **opts) -> list[Path]:
        """Extract archive."""
        ...

    def list_contents(self, archive: Path) -> list[str]:
        """List archive contents."""
        ...

    def add_file(self, archive: Path, file: Path, arcname: str | None = None) -> None:
        """Add file to existing archive."""
        ...
@runtime_checkable

class ICompressor(Protocol):
    """
    Interface for compression algorithms.
    Each algorithm (gzip, bz2, lzma, zstd) implements this.
    """
    @property

    def algorithm_id(self) -> str:
        """Unique algorithm identifier."""
        ...
    @property

    def file_extensions(self) -> list[str]:
        """Supported file extensions."""
        ...

    def compress(self, data: bytes, level: int = 6) -> bytes:
        """Compress bytes."""
        ...

    def decompress(self, data: bytes) -> bytes:
        """Decompress bytes."""
        ...

    def can_handle(self, data: bytes) -> bool:
        """Check if this compressor can handle the data."""
        ...
@runtime_checkable

class IArchiveMetadata(Protocol):
    """
    Metadata protocol for self-describing archivers.
    Like ICodecMetadata for codecs!
    """
    @property

    def format_id(self) -> str:
        """Format identifier."""
        ...
    @property

    def file_extensions(self) -> list[str]:
        """Supported extensions."""
        ...
    @property

    def mime_types(self) -> list[str]:
        """MIME types."""
        ...
    @property

    def description(self) -> str:
        """Human-readable description."""
        ...
# From codec/
@runtime_checkable

class ICodec[T, R](Protocol):
    """
    Universal codec interface for bidirectional transformation.
    A codec transforms between a model (T) and its representation (R).
    This is the minimal contract that all codecs must implement.
    Type Parameters:
        T: Model type (e.g., dict, AST, dataclass)
        R: Representation type (bytes or str)
    Examples:
        JSON serializer:  ICodec[dict, bytes]
        SQL formatter:    ICodec[QueryAST, str]
        Pickle:           ICodec[Any, bytes]
        Python unparser:  ICodec[ast.AST, str]
    Design Principles:
        - Bidirectional by default (encode/decode)
        - Options-based configuration (not constructor pollution)
        - Representation-type specific (bytes OR str, not both)
        - Composable via adapters
    """

    def encode(self, value: T, *, options: EncodeOptions | None = None) -> R:
        """
        Encode a model to its representation.
        Args:
            value: Model instance to encode
            options: Format-specific encoding options (e.g., {'pretty': True})
        Returns:
            Representation (bytes or str depending on codec type)
        Raises:
            EncodeError: If encoding fails
        Examples:
            >>> codec = JsonSerializer()
            >>> codec.encode({"key": "value"})
            b'{"key":"value"}'
            >>> formatter = SqlFormatter()
            >>> formatter.encode(select_ast, options={"pretty": True})
            'SELECT *\\nFROM users\\nWHERE id = 1'
        """
        ...

    def decode(self, repr: R, *, options: DecodeOptions | None = None) -> T:
        """
        Decode a representation to a model.
        Args:
            repr: Representation to decode (bytes or str)
            options: Format-specific decoding options (e.g., {'strict': False})
        Returns:
            Model instance
        Raises:
            DecodeError: If decoding fails
        Examples:
            >>> codec = JsonSerializer()
            >>> codec.decode(b'{"key":"value"}')
            {'key': 'value'}
            >>> formatter = SqlFormatter()
            >>> formatter.decode('SELECT * FROM users')
            QueryAST(...)
        """
        ...
@runtime_checkable

class ICodecMetadata(Protocol):
    """
    Metadata protocol for codec discovery and registration.
    Codecs that implement this protocol can self-register and be
    discovered by the registry system with no hardcoding.
    Example:
        >>> class JsonCodec:
        ...     codec_id = "json"
        ...     media_types = ["application/json", "text/json"]
        ...     file_extensions = [".json", ".jsonl"]
        ...     aliases = ["JSON"]
        ...     
        ...     def capabilities(self):
        ...         return CodecCapability.BIDIRECTIONAL | CodecCapability.TEXT
    """
    @property

    def codec_id(self) -> str:
        """
        Unique codec identifier.
        Should be lowercase, alphanumeric + dash/underscore.
        Examples:
            - "json"
            - "sql"
            - "protobuf"
            - "python-ast"
        """
        ...
    @property

    def media_types(self) -> list[str]:
        """
        Supported media types / content types (RFC 2046).
        Used for content negotiation and HTTP Content-Type headers.
        Examples:
            - JSON: ["application/json", "text/json"]
            - SQL: ["application/sql", "text/x-sql"]
            - Protobuf: ["application/protobuf", "application/x-protobuf"]
        """
        ...
    @property

    def file_extensions(self) -> list[str]:
        """
        Supported file extensions (with leading dot).
        Used for auto-detection from file paths.
        Examples:
            - JSON: [".json", ".jsonl"]
            - SQL: [".sql", ".ddl", ".dml"]
            - Python: [".py", ".pyi"]
        """
        ...
    @property

    def aliases(self) -> list[str]:
        """
        Alternative names for this codec.
        Used for flexible lookup (case-insensitive matching).
        Examples:
            - JSON: ["json", "JSON"]
            - SQL: ["sql", "SQL", "structured-query"]
        """
        ...

    def capabilities(self) -> CodecCapability:
        """
        Get capabilities supported by this codec.
        Returns:
            Flag combination of supported features
        Example:
            >>> codec.capabilities()
            CodecCapability.BIDIRECTIONAL | CodecCapability.TEXT | CodecCapability.SCHEMA
        """
        ...
# From common/
@runtime_checkable

class IAtomicWriter(Protocol):
    """Interface for atomic file write operations."""

    def write(self, data: bytes) -> int:
        """Write data atomically."""
        ...

    def __enter__(self) -> IAtomicWriter:
        """Enter context manager."""
        ...

    def __exit__(self, exc_type, exc_val, exc_tb) -> None:
        """Exit context manager."""
        ...
# ============================================================================
# ARCHIVE INTERFACES (Dual Architecture: Codec + File)
# ============================================================================


class IArchiver[T](ICodec[T, bytes]):
    """
    Archive codec interface - operates in MEMORY on ANY data.
    Extends ICodec to provide dual API:
    - encode()/decode() - Low-level codec operations
    - compress()/extract() - User-friendly archive operations
    Type: ICodec[T, bytes] where T can be:
    - bytes (raw data)
    - str (text data)
    - dict/list (structured data)
    - Any (objects)
    NOT limited to file paths - works on data in RAM!
    """

    def compress(self, data: T, **options) -> bytes:
        """
        Compress data to archive bytes (in RAM).
        Delegates to encode() internally.
        """
        ...

    def extract(self, archive_bytes: bytes, **options) -> T:
        """
        Extract archive bytes to data (in RAM).
        Delegates to decode() internally.
        """
        ...


class IArchiveFile(IFile):
    """
    Archive FILE interface - operates on DISK.
    Extends IFile for file operations.
    USES IArchiver internally for compression (composition).
    This handles:
    - File I/O with archive files on disk
    - Adding/extracting files to/from archives
    - Archive file management
    """

    def add_files(self, files: list[Path], **options) -> None:
        """Add files to archive (uses archiver.compress internally)."""
        ...

    def extract_to(self, dest: Path, **options) -> list[Path]:
        """Extract archive to destination (uses archiver.extract internally)."""
        ...

    def list_contents(self) -> list[str]:
        """List files in archive."""
        ...

    def get_archiver(self) -> IArchiver:
        """Get the underlying archiver codec."""
        ...
@runtime_checkable

class ICompression(Protocol):
    """
    Interface for raw compression operations (gzip, bz2, lzma, etc.).
    This is for compressing RAW BYTES (not archives).
    Separate from IArchiver which handles archive formats.
    """

    def compress(self, data: bytes, **options) -> bytes:
        """Compress raw bytes."""
        ...

    def decompress(self, data: bytes, **options) -> bytes:
        """Decompress raw bytes."""
        ...
@runtime_checkable

class IPathValidator(Protocol):
    """Interface for path validation and security checks."""

    def validate_path(self, path: str | Path) -> bool:
        """Validate path safety."""
        ...

    def is_safe_path(self, path: str | Path) -> bool:
        """Check if path is safe to use."""
        ...

    def normalize_path(self, path: str | Path) -> Path:
        """Normalize and resolve path."""
        ...
# From file/
@runtime_checkable

class IFileSource(Protocol):
    """Interface for file data sources."""
    @property

    def uri(self) -> str:
        """Source URI."""
        ...
    @property

    def scheme(self) -> str:
        """URI scheme."""
        ...

    def read(self, **options) -> bytes | str:
        """Read entire file content."""
        ...

    def write(self, data: bytes | str, **options) -> None:
        """Write entire content to file."""
        ...

    def exists(self) -> bool:
        """Check if file exists."""
        ...

    def delete(self) -> None:
        """Delete file."""
        ...
@runtime_checkable

class IPagedSource(Protocol):
    """Interface for paged file sources."""
    @property

    def total_size(self) -> int:
        """Total file size in bytes."""
        ...

    def read_page(self, page: int, page_size: int, **options) -> bytes | str:
        """Read specific page."""
        ...

    def read_chunk(self, offset: int, size: int, **options) -> bytes | str:
        """Read chunk by byte offset."""
        ...

    def iter_pages(self, page_size: int, **options) -> Iterator[bytes | str]:
        """Iterate over pages."""
        ...

    def iter_chunks(self, chunk_size: int, **options) -> Iterator[bytes | str]:
        """Iterate over chunks."""
        ...
@runtime_checkable

class IPagingStrategy(Protocol):
    """
    Strategy interface for paging through file data.
    Enables pluggable paging algorithms:
    - BytePagingStrategy: Page by byte offsets
    - LinePagingStrategy: Page by line counts
    - RecordPagingStrategy: Page by record boundaries (CSV, JSONL)
    - SmartPagingStrategy: Adaptive paging based on content
    """
    @property

    def strategy_id(self) -> str:
        """Unique strategy identifier."""
        ...

    def read_page(
        self,
        file_path: Path,
        page: int,
        page_size: int,
        mode: str = 'rb',
        encoding: str | None = None,
        **options
    ) -> bytes | str:
        """
        Read specific page using this strategy.
        Args:
            file_path: Path to file
            page: Page number (0-based)
            page_size: Items per page (interpretation depends on strategy)
            mode: File mode
            encoding: Text encoding (for text mode)
            **options: Strategy-specific options
        Returns:
            Page content
        """
        ...

    def iter_pages(
        self,
        file_path: Path,
        page_size: int,
        mode: str = 'rb',
        encoding: str | None = None,
        **options
    ) -> Iterator[bytes | str]:
        """
        Iterate over pages using this strategy.
        Args:
            file_path: Path to file
            page_size: Items per page
            mode: File mode
            encoding: Text encoding
            **options: Strategy-specific options
        Yields:
            Page content
        """
        ...
# From filesystem/
@runtime_checkable

class IVirtualFS(Protocol):
    """Interface for virtual filesystem operations."""
    @property

    def scheme(self) -> str:
        """URI scheme."""
        ...

    def exists(self, path: str) -> bool:
        """Check if path exists."""
        ...

    def is_file(self, path: str) -> bool:
        """Check if path is file."""
        ...
# From folder/
@runtime_checkable

class IFolderSource(Protocol):
    """Interface for folder operations."""

    def create(self, parents: bool = True, exist_ok: bool = True) -> bool:
        """Create directory."""
        ...

    def delete(self, recursive: bool = False) -> bool:
        """Delete directory."""
        ...

    def list_files(self, pattern: str | None = None, recursive: bool = False) -> list[Path]:
        """List files in directory."""
        ...
# From manager/
@runtime_checkable

class IIOManager(Protocol):
    """Interface for I/O managers."""

    def open(self, **opts):
        """Open resource."""
        ...

    def close(self) -> None:
        """Close resource."""
        ...
# From stream/


class IPagedCodecIO[T, R](ICodecIO[T, R]):
    """
    Interface for paged codec I/O.
    """

    def iter_items(self, page_size: int, **opts):
        """Iterate over decoded items."""
        ...

    def load_page(self, page: int, page_size: int, **opts) -> list[T]:
        """Load specific page."""
        ...
