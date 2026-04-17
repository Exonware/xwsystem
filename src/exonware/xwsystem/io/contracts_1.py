#exonware/xwsystem/src/exonware/xwsystem/io/contracts_1.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.40
Generation Date: September 04, 2025
IO module contracts - interfaces and enums for input/output operations.
"""

from __future__ import annotations
from typing import Protocol, runtime_checkable, Any, BinaryIO, TextIO

from collections.abc import Callable, Iterator, AsyncGenerator
from typing import TypeAlias
from pathlib import Path
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
# BASE CODEC INTERFACES (Moved up for TypeAlias resolution)
# ============================================================================
EncodeOptions: TypeAlias = dict[str, Any]
DecodeOptions: TypeAlias = dict[str, Any]
@runtime_checkable

class ICodec[T, R](Protocol):
    """
    Universal codec interface for bidirectional transformation.
    Type Parameters:
        T: Model type (e.g., dict, AST, dataclass)
        R: Representation type (bytes or str)
    """

    def encode(self, value: T, *, options: EncodeOptions | None = None) -> R:
        """Encode a model to its representation."""
        ...

    def decode(self, repr: R, *, options: DecodeOptions | None = None) -> T:
        """Decode a representation to a model."""
        ...
@runtime_checkable

class ICodecMetadata(Protocol):
    """Metadata protocol for codec discovery and registration."""
    @property

    def codec_id(self) -> str:
        """Unique codec identifier (e.g., 'json', 'sql')."""
        ...
    @property

    def media_types(self) -> list[str]:
        """Supported media types / content types (RFC 2046)."""
        ...
    @property

    def file_extensions(self) -> list[str]:
        """Supported file extensions (e.g., ['.json'])."""
        ...
    @property

    def aliases(self) -> list[str]:
        """Alternative names for this codec."""
        ...

    def capabilities(self) -> CodecCapability:
        """Get capabilities supported by this codec."""
        ...
# ============================================================================
# TYPE ALIASES
# ============================================================================
Serializer: TypeAlias = ICodec[Any, bytes]
Formatter: TypeAlias = ICodec[Any, str]
# ============================================================================
# PATH INTERFACES
# ============================================================================
@runtime_checkable

class IPath(Protocol):
    """
    Interface for path operations with both static and instance methods.
    """
    @staticmethod

    def normalize(path: PathType) -> Path:
        """Normalize path."""
        ...
    @staticmethod

    def resolve(path: PathType) -> Path:
        """Resolve path."""
        ...
    @staticmethod

    def absolute(path: PathType) -> Path:
        """Get absolute path."""
        ...
    @staticmethod

    def relative(path: PathType, start: PathType | None = None) -> Path:
        """Get relative path."""
        ...
    @staticmethod

    def join(*paths: PathType) -> Path:
        """Join paths."""
        ...
    @staticmethod

    def split(path: PathType) -> tuple[Path, str]:
        """Split path into directory and filename."""
        ...
    @staticmethod

    def get_extension(path: PathType) -> str:
        """Get file extension."""
        ...
    @staticmethod

    def get_stem(path: PathType) -> str:
        """Get file stem (name without extension)."""
        ...
    @staticmethod

    def get_name(path: PathType) -> str:
        """Get file/directory name."""
        ...
    @staticmethod

    def get_parent(path: PathType) -> Path:
        """Get parent directory."""
        ...
    @staticmethod

    def is_absolute(path: PathType) -> bool:
        """Check if path is absolute."""
        ...
    @staticmethod

    def is_relative(path: PathType) -> bool:
        """Check if path is relative."""
        ...
    @staticmethod

    def get_parts(path: PathType) -> tuple:
        """Get path parts."""
        ...
    @staticmethod

    def match(path: PathType, pattern: str) -> bool:
        """Check if path matches pattern."""
        ...
    @staticmethod

    def with_suffix(path: PathType, suffix: str) -> Path:
        """Get path with new suffix."""
        ...
    @staticmethod

    def with_name(path: PathType, name: str) -> Path:
        """Get path with new name."""
        ...
@runtime_checkable

class IPathValidator(Protocol):
    """Interface for path validation and security checks."""

    def validate_path(self, path: PathType) -> bool:
        """Validate path safety."""
        ...

    def is_safe_path(self, path: PathType) -> bool:
        """Check if path is safe to use."""
        ...

    def normalize_path(self, path: PathType) -> Path:
        """Normalize and resolve path."""
        ...
# ============================================================================
# STREAM & ASYNC INTERFACES
# ============================================================================
@runtime_checkable

class IStream(Protocol):
    """Interface for stream operations."""
    # Instance Methods

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
    # Static Methods
    @staticmethod

    def open_file(path: PathType, mode: str = 'r', encoding: str | None = None) -> TextIO | BinaryIO:
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
@runtime_checkable

class IAsyncIO(Protocol):
    """Interface for async I/O operations."""
    # Instance Methods

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
    # Static Methods
    @staticmethod

    async def aopen_file(path: PathType, mode: str = 'r', encoding: str | None = None) -> Any:
        """Async open file."""
        ...
    @staticmethod

    async def aread_text(path: PathType, encoding: str = 'utf-8') -> str:
        """Async read text file."""
        ...
    @staticmethod

    async def aread_bytes(path: PathType) -> bytes:
        """Async read binary file."""
        ...
    @staticmethod

    async def awrite_text(path: PathType, content: str, encoding: str = 'utf-8') -> bool:
        """Async write text to file."""
        ...
    @staticmethod

    async def awrite_bytes(path: PathType, content: bytes) -> bool:
        """Async write bytes to file."""
        ...
# ============================================================================
# FILE SYSTEM INTERFACES
# ============================================================================
@runtime_checkable

class IFile(Protocol):
    """
    Interface for file operations with both static and instance methods.
    """
    # Instance Methods

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

    def save_as(self, path: PathType, data: Any, **kwargs) -> bool:
        """Save data to specific path."""
        ...

    def to_file(self, path: PathType, **kwargs) -> bool:
        """Write current object to file."""
        ...

    def from_file(self, path: PathType, **kwargs) -> IFile:
        """Load object from file."""
        ...
    # Static Methods
    @staticmethod

    def exists(path: PathType) -> bool:
        """Check if file exists."""
        ...
    @staticmethod

    def size(path: PathType) -> int:
        """Get file size."""
        ...
    @staticmethod

    def delete(path: PathType) -> bool:
        """Delete file."""
        ...
    @staticmethod

    def copy(source: PathType, destination: PathType) -> bool:
        """Copy file."""
        ...
    @staticmethod

    def move(source: PathType, destination: PathType) -> bool:
        """Move file."""
        ...
    @staticmethod

    def rename(old_path: PathType, new_path: PathType) -> bool:
        """Rename file."""
        ...
    @staticmethod

    def get_modified_time(path: PathType) -> float:
        """Get file modification time."""
        ...
    @staticmethod

    def get_created_time(path: PathType) -> float:
        """Get file creation time."""
        ...
    @staticmethod

    def get_permissions(path: PathType) -> int:
        """Get file permissions."""
        ...
    @staticmethod

    def is_readable(path: PathType) -> bool:
        """Check if file is readable."""
        ...
    @staticmethod

    def is_writable(path: PathType) -> bool:
        """Check if file is writable."""
        ...
    @staticmethod

    def is_executable(path: PathType) -> bool:
        """Check if file is executable."""
        ...
    @staticmethod

    def read_text(path: PathType, encoding: str = 'utf-8') -> str:
        """Read file as text."""
        ...
    @staticmethod

    def read_bytes(path: PathType) -> bytes:
        """Read file as bytes."""
        ...
    @staticmethod

    def write_text(path: PathType, content: str, encoding: str = 'utf-8') -> bool:
        """Write text to file."""
        ...
    @staticmethod

    def write_bytes(path: PathType, content: bytes) -> bool:
        """Write bytes to file."""
        ...
    @staticmethod

    def safe_read_text(path: PathType, encoding: str = 'utf-8') -> str | None:
        """Safely read text file, returning None on error."""
        ...
    @staticmethod

    def safe_read_bytes(path: PathType) -> bytes | None:
        """Safely read binary file, returning None on error."""
        ...
    @staticmethod

    def safe_write_text(path: PathType, content: str, encoding: str = 'utf-8') -> bool:
        """Safely write text to file."""
        ...
    @staticmethod

    def safe_write_bytes(path: PathType, content: bytes) -> bool:
        """Safely write bytes to file."""
        ...
    # Static Utility Methods (File Manager Features)
    @staticmethod

    def atomic_write(file_path: PathType, data: str | bytes, 
                    backup: bool = True) -> OperationResult:
        """Atomically write data to file (static version)."""
        ...
    @staticmethod

    def atomic_copy(source: PathType, destination: PathType) -> OperationResult:
        """Atomically copy file (static version)."""
        ...
    @staticmethod

    def atomic_move(source: PathType, destination: PathType) -> OperationResult:
        """Atomically move file (static version)."""
        ...
    @staticmethod

    def atomic_delete(file_path: PathType, backup: bool = True) -> OperationResult:
        """Atomically delete file (static version)."""
        ...
    @staticmethod

    def create_backup(source: PathType, backup_dir: PathType) -> Path | None:
        """Create backup of file (static version)."""
        ...
    @staticmethod

    def restore_backup(backup_path: PathType, target: PathType) -> OperationResult:
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
@runtime_checkable

class IFolder(Protocol):
    """
    Interface for folder/directory operations.
    """
    # Instance Methods

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

    def copy_to(self, destination: PathType) -> bool:
        """Copy directory to destination."""
        ...

    def move_to(self, destination: PathType) -> bool:
        """Move directory to destination."""
        ...
    # Static Methods
    @staticmethod

    def exists(path: PathType) -> bool:
        """Check if directory exists."""
        ...
    @staticmethod

    def create_dir(path: PathType, parents: bool = True, exist_ok: bool = True) -> bool:
        """Create directory."""
        ...
    @staticmethod

    def delete_dir(path: PathType, recursive: bool = False) -> bool:
        """Delete directory."""
        ...
    @staticmethod

    def list_files_static(path: PathType, pattern: str | None = None, recursive: bool = False) -> list[Path]:
        """List files in directory."""
        ...
    @staticmethod

    def list_directories_static(path: PathType, recursive: bool = False) -> list[Path]:
        """List subdirectories."""
        ...
    @staticmethod

    def walk_static(path: PathType) -> list[tuple[Path, list[str], list[str]]]:
        """Walk directory tree."""
        ...
    @staticmethod

    def get_size_static(path: PathType) -> int:
        """Get directory size."""
        ...
    @staticmethod

    def is_empty_static(path: PathType) -> bool:
        """Check if directory is empty."""
        ...
    @staticmethod

    def copy_dir(source: PathType, destination: PathType) -> bool:
        """Copy directory."""
        ...
    @staticmethod

    def move_dir(source: PathType, destination: PathType) -> bool:
        """Move directory."""
        ...
    @staticmethod

    def get_permissions(path: PathType) -> int:
        """Get directory permissions."""
        ...
    @staticmethod

    def is_readable(path: PathType) -> bool:
        """Check if directory is readable."""
        ...
    @staticmethod

    def is_writable(path: PathType) -> bool:
        """Check if directory is writable."""
        ...
    @staticmethod

    def is_executable(path: PathType) -> bool:
        """Check if directory is executable."""
        ...
# ============================================================================
# SYSTEM UTILITY INTERFACES (Watcher, Lock, VirtualFS)
# ============================================================================
@runtime_checkable

class IFileWatcher(Protocol):
    """Interface for watching file system changes."""

    def watch(self, path: PathType) -> None:
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

    def read(self, path: str) -> bytes:
        """Read file contents."""
        ...

    def write(self, path: str, data: bytes) -> None:
        """Write file contents."""
        ...
# ============================================================================
# ADVANCED OPERATION INTERFACES (Atomic, Backup, Temp)
# ============================================================================
@runtime_checkable

class IAtomicOperations(Protocol):
    """Interface for atomic operations."""
    # Instance Methods

    def atomic_write(self, file_path: PathType, data: str | bytes, 
                    backup: bool = True) -> OperationResult:
        """Atomically write data to file."""
        ...

    def atomic_copy(self, source: PathType, destination: PathType) -> OperationResult:
        """Atomically copy file."""
        ...

    def atomic_move(self, source: PathType, destination: PathType) -> OperationResult:
        """Atomically move file."""
        ...

    def atomic_delete(self, file_path: PathType, backup: bool = True) -> OperationResult:
        """Atomically delete file."""
        ...

    def atomic_rename(self, old_path: PathType, new_path: PathType) -> OperationResult:
        """Atomically rename file."""
        ...
    # Static Methods
    @staticmethod

    def atomic_write_static(file_path: PathType, data: str | bytes, 
                           backup: bool = True) -> OperationResult:
        """Atomically write data to file."""
        ...
    @staticmethod

    def atomic_copy_static(source: PathType, destination: PathType) -> OperationResult:
        """Atomically copy file."""
        ...
    @staticmethod

    def atomic_move_static(source: PathType, destination: PathType) -> OperationResult:
        """Atomically move file."""
        ...
    @staticmethod

    def atomic_delete_static(file_path: PathType, backup: bool = True) -> OperationResult:
        """Atomically delete file."""
        ...
    @staticmethod

    def atomic_rename_static(old_path: PathType, new_path: PathType) -> OperationResult:
        """Atomically rename file."""
        ...
@runtime_checkable

class IAtomicWriter(Protocol):
    """Interface for atomic file write context manager."""

    def write(self, data: bytes) -> int:
        """Write data atomically."""
        ...

    def __enter__(self) -> IAtomicWriter:
        """Enter context manager."""
        ...

    def __exit__(self, exc_type, exc_val, exc_tb) -> None:
        """Exit context manager."""
        ...
@runtime_checkable

class IBackupOperations(Protocol):
    """Interface for backup operations."""
    # Instance Methods

    def create_backup(self, source: PathType, backup_dir: PathType) -> Path | None:
        """Create backup of file or directory."""
        ...

    def restore_backup(self, backup_path: PathType, target: PathType) -> OperationResult:
        """Restore from backup."""
        ...

    def list_backups(self, backup_dir: PathType) -> list[Path]:
        """List available backups."""
        ...

    def cleanup_backups(self, backup_dir: PathType, max_age_days: int = 30) -> int:
        """Cleanup old backups."""
        ...

    def verify_backup(self, backup_path: PathType) -> bool:
        """Verify backup integrity."""
        ...
    # Static Methods
    @staticmethod

    def create_backup_static(source: PathType, backup_dir: PathType) -> Path | None:
        """Create backup of file or directory."""
        ...
    @staticmethod

    def restore_backup_static(backup_path: PathType, target: PathType) -> OperationResult:
        """Restore from backup."""
        ...
    @staticmethod

    def list_backups_static(backup_dir: PathType) -> list[Path]:
        """List available backups."""
        ...
    @staticmethod

    def cleanup_backups_static(backup_dir: PathType, max_age_days: int = 30) -> int:
        """Cleanup old backups."""
        ...
    @staticmethod

    def verify_backup_static(backup_path: PathType) -> bool:
        """Verify backup integrity."""
        ...
@runtime_checkable

class ITemporaryOperations(Protocol):
    """Interface for temporary operations."""
    # Instance Methods

    def create_temp_file(self, suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary file."""
        ...

    def create_temp_directory(self, suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary directory."""
        ...

    def cleanup_temp(self, path: PathType) -> bool:
        """Cleanup temporary file or directory."""
        ...

    def cleanup_all_temp(self) -> int:
        """Cleanup all temporary files and directories."""
        ...
    # Static Methods
    @staticmethod

    def create_temp_file_static(suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary file."""
        ...
    @staticmethod

    def create_temp_directory_static(suffix: str | None = None, prefix: str | None = None) -> Path:
        """Create temporary directory."""
        ...
    @staticmethod

    def cleanup_temp_static(path: PathType) -> bool:
        """Cleanup temporary file or directory."""
        ...
    @staticmethod

    def get_temp_base_dir() -> Path:
        """Get temporary base directory."""
        ...
    @staticmethod

    def is_temp(path: PathType) -> bool:
        """Check if path is temporary."""
        ...
# ============================================================================
# ARCHIVE & COMPRESSION INTERFACES
# ============================================================================
@runtime_checkable

class ICompression(Protocol):
    """
    Interface for raw compression operations (gzip, bz2, lzma, etc.).
    This operates on bytes, not archive file formats.
    """
    @property

    def algorithm_id(self) -> str:
        """Unique algorithm identifier."""
        ...
    @property

    def file_extensions(self) -> list[str]:
        """Supported file extensions."""
        ...

    def compress(self, data: bytes, **options) -> bytes:
        """Compress raw bytes."""
        ...

    def decompress(self, data: bytes, **options) -> bytes:
        """Decompress raw bytes."""
        ...

    def can_handle(self, data: bytes) -> bool:
        """Check if this compressor can handle the data."""
        ...
@runtime_checkable

class IArchiveFormat(Protocol):
    """
    Interface for archive format handlers (ZIP, TAR, 7Z, RAR).
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

class IArchiveMetadata(Protocol):
    """Metadata protocol for self-describing archivers."""
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


class IArchiver[T](ICodec[T, bytes]):
    """
    Archive codec interface - operates in MEMORY on ANY data.
    Delegates to encode/decode but provides specific archive terminology.
    """

    def compress(self, data: T, **options) -> bytes:
        """Compress data to archive bytes (delegates to encode)."""
        ...

    def extract(self, archive_bytes: bytes, **options) -> T:
        """Extract archive bytes to data (delegates to decode)."""
        ...


class IArchiveFile(IFile):
    """
    Archive FILE interface - operates on DISK.
    Uses IArchiver internally for composition.
    """

    def add_files(self, files: list[Path], **options) -> None:
        """Add files to archive."""
        ...

    def extract_to(self, dest: Path, **options) -> list[Path]:
        """Extract archive to destination."""
        ...

    def list_contents(self) -> list[str]:
        """List files in archive."""
        ...

    def get_archiver(self) -> IArchiver:
        """Get the underlying archiver codec."""
        ...
# ============================================================================
# DATA SOURCE INTERFACES
# ============================================================================
@runtime_checkable

class IDataSource[T](Protocol):
    """Universal data source interface."""

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
    """Strategy interface for paging through file data."""
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
        """Read specific page using this strategy."""
        ...

    def iter_pages(
        self,
        file_path: Path,
        page_size: int,
        mode: str = 'rb',
        encoding: str | None = None,
        **options
    ) -> Iterator[bytes | str]:
        """Iterate over pages using this strategy."""
        ...
@runtime_checkable

class IFolderSource(Protocol):
    """Interface for folder source operations."""

    def create(self, parents: bool = True, exist_ok: bool = True) -> bool:
        """Create directory."""
        ...

    def delete(self, recursive: bool = False) -> bool:
        """Delete directory."""
        ...

    def list_files(self, pattern: str | None = None, recursive: bool = False) -> list[Path]:
        """List files in directory."""
        ...
# ============================================================================
# CODEC-INTEGRATED IO INTERFACES
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

class IPagedCodecIO[T, R](ICodecIO[T, R]):
    """
    Interface for paged codec I/O.
    Inherits read_as/write_as from ICodecIO.
    """

    def read_page_as(self, page_number: int, codec: str):
        """Read and decode a page using specified codec."""
        ...

    def iter_items(self, page_size: int, **opts):
        """Iterate over decoded items."""
        ...

    def load_page(self, page: int, page_size: int, **opts) -> list[T]:
        """Load specific page."""
        ...
# ============================================================================
# HIGH-LEVEL COMPOSITE INTERFACES
# ============================================================================


class IUnifiedIO(IFile, IFolder, IPath, IStream, IAsyncIO, IAtomicOperations, IBackupOperations, ITemporaryOperations):
    """
    Unified I/O interface combining all existing I/O capabilities.
    Provides complete I/O functionality for any data source, consolidating:
    - File/Folder/Path operations
    - Stream/Async operations
    - Atomic/Backup/Temporary operations
    """
    pass


class IFileManager(IFile, IFolder, IPath, IAtomicOperations, IBackupOperations, ITemporaryOperations):
    """
    File Manager interface for comprehensive file operations.
    Specifically designed for managing files on disk with support for:
    - Universal file type support
    - Intelligent format detection
    - Atomic/Backup/Temporary safety features
    """
    pass
@runtime_checkable

class IIOManager(Protocol):
    """Generic interface for I/O managers."""

    def open(self, **opts):
        """Open resource."""
        ...

    def close(self) -> None:
        """Close resource."""
        ...
