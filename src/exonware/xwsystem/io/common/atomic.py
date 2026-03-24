#exonware/xwsystem/src/exonware/xwsystem/io/common/atomic.py
"""
Atomic file operations to prevent data corruption during writes.
"""

import logging
import os
import platform
import shutil
import tempfile
import time
from contextlib import contextmanager
from pathlib import Path
from typing import Any, BinaryIO, TextIO
logger = logging.getLogger(__name__)


class FileOperationError(Exception):
    """Raised when file operations fail."""
    pass


class AtomicFileWriter:
    """
    Provides atomic file writing operations to prevent data corruption.
    This class ensures that file writes are atomic by writing to a temporary
    file first and then moving it to the target location. This prevents
    partial writes if the operation is interrupted.
    """

    def __init__(
        self,
        target_path: str | Path,
        mode: str = "w",
        encoding: str | None = "utf-8",
        backup: bool = False,
        temp_dir: str | Path | None = None,
    ):
        """
        Initialize atomic file writer.
        Args:
            target_path: Final path where file should be written
            mode: File open mode ('w', 'wb', 'w+', etc.)
            encoding: Text encoding (for text modes)
            backup: Whether to create backup of existing file
            temp_dir: Directory for temporary files (defaults to same as target)
        """
        self.target_path = Path(target_path)
        self.mode = mode
        self.encoding = encoding if "b" not in mode else None
        self.backup = backup
        self.temp_dir = Path(temp_dir) if temp_dir else self.target_path.parent
        self.temp_path: Path | None = None
        self.backup_path: Path | None = None
        self.file_handle: BinaryIO | TextIO | None = None
        self._committed = False
        self._started = False

    def __enter__(self) -> BinaryIO | TextIO:
        """Context manager entry - create temporary file."""
        return self.start()

    def __exit__(self, exc_type, exc_val, exc_tb) -> None:
        """Context manager exit - commit or rollback based on success."""
        if exc_type is None:
            # No exception occurred, commit the write
            self.commit()
        else:
            # Exception occurred, rollback
            self.rollback()
        return False  # Don't suppress exceptions

    def start(self) -> BinaryIO | TextIO:
        """
        Start the atomic write operation.
        Returns:
            File handle for writing
        """
        if self._started:
            raise FileOperationError("Atomic write operation already started")
        self._started = True
        try:
            # Ensure temp directory exists
            self.temp_dir.mkdir(parents=True, exist_ok=True)
            # Create backup if requested and target exists
            if self.backup and self.target_path.exists():
                self._create_backup()
            # Create temporary file in same directory as target
            # This ensures they're on the same filesystem for atomic move
            fd, temp_path_str = tempfile.mkstemp(
                prefix=f".{self.target_path.name}_", suffix=".tmp", dir=self.temp_dir
            )
            self.temp_path = Path(temp_path_str)
            # Close the file descriptor and reopen with desired mode
            os.close(fd)
            # Open with the requested mode and encoding
            if self.encoding:
                self.file_handle = open(
                    self.temp_path, self.mode, encoding=self.encoding
                )
            else:
                self.file_handle = open(self.temp_path, self.mode)
            logger.debug(
                f"Started atomic write: {self.target_path} via {self.temp_path}"
            )
            return self.file_handle
        except Exception as e:
            self._cleanup()
            raise FileOperationError(f"Failed to start atomic write: {e}") from e

    def commit(self) -> None:
        """
        Commit the atomic write operation.
        This closes the temporary file and atomically moves it to the target location.
        """
        if not self._started:
            raise FileOperationError("Atomic write operation not started")
        if self._committed:
            return  # Already committed
        try:
            # Close the file handle
            if self.file_handle and not self.file_handle.closed:
                self.file_handle.close()
            # Verify temp file was written
            if not self.temp_path or not self.temp_path.exists():
                raise FileOperationError(
                    "Temporary file was not created or was deleted"
                )
            # Get file stats for verification
            temp_stat = self.temp_path.stat()
            if temp_stat.st_size == 0:
                logger.warning(f"Temporary file is empty: {self.temp_path}")
            # Atomic move to target location
            # Use retries on Windows to handle transient file-lock races.
            if platform.system() == "Windows":
                self._replace_with_retry(self.temp_path, self.target_path)
            else:
                shutil.move(str(self.temp_path), str(self.target_path))
            self._committed = True
            # Set file permissions to match original if backup exists
            if self.backup_path and self.backup_path.exists():
                try:
                    backup_stat = self.backup_path.stat()
                    os.chmod(self.target_path, backup_stat.st_mode)
                except OSError:
                    pass  # Ignore permission errors
            logger.debug(f"Committed atomic write: {self.target_path}")
        except Exception as e:
            # Try to rollback on commit failure
            self.rollback()
            raise FileOperationError(f"Failed to commit atomic write: {e}") from e

    @staticmethod
    def _replace_with_retry(temp_path: Path, target_path: Path, retries: int = 40, delay_s: float = 0.02) -> None:
        """
        Replace target with temp atomically on Windows, retrying transient lock errors.
        """
        last_error: Exception | None = None
        for attempt in range(retries):
            try:
                os.replace(str(temp_path), str(target_path))
                return
            except PermissionError as e:
                last_error = e
                # WinError 32: file in use by another process. Retry with backoff.
                if attempt == retries - 1:
                    break
                time.sleep(delay_s * (attempt + 1))
        # Final non-atomic fallback for stubborn Windows lock cases.
        # This preserves write availability under transient AV/indexer/file-lock races.
        try:
            shutil.copy2(str(temp_path), str(target_path))
            temp_path.unlink(missing_ok=True)
            return
        except Exception:
            if last_error is not None:
                raise last_error

    def rollback(self) -> None:
        """
        Rollback the atomic write operation.
        This removes the temporary file and restores backup if available.
        """
        if not self._started:
            return
        logger.debug(f"Rolling back atomic write: {self.target_path}")
        # Close file handle
        if self.file_handle and not self.file_handle.closed:
            try:
                self.file_handle.close()
            except Exception:
                pass  # Ignore close errors during rollback
        # Remove temporary file
        if self.temp_path and self.temp_path.exists():
            try:
                self.temp_path.unlink()
                logger.debug(f"Removed temporary file: {self.temp_path}")
            except Exception as e:
                logger.warning(f"Could not remove temporary file {self.temp_path}: {e}")
        # Restore backup if needed and target was removed
        if (
            self.backup_path
            and self.backup_path.exists()
            and not self.target_path.exists()
        ):
            try:
                shutil.move(str(self.backup_path), str(self.target_path))
                logger.debug(
                    f"Restored backup: {self.backup_path} -> {self.target_path}"
                )
            except Exception as e:
                logger.error(f"Could not restore backup {self.backup_path}: {e}")
        self._cleanup()

    def _create_backup(self) -> None:
        """Create backup of existing target file."""
        if not self.target_path.exists():
            return
        timestamp = int(time.time())
        backup_name = f"{self.target_path.name}.backup.{timestamp}"
        self.backup_path = self.target_path.parent / backup_name
        try:
            shutil.copy2(str(self.target_path), str(self.backup_path))
            logger.debug(f"Created backup: {self.backup_path}")
        except Exception as e:
            logger.warning(f"Could not create backup: {e}")
            self.backup_path = None

    def _cleanup(self) -> None:
        """Clean up temporary resources."""
        # Remove backup if commit was successful
        if self._committed and self.backup_path and self.backup_path.exists():
            try:
                self.backup_path.unlink()
                logger.debug(f"Removed backup: {self.backup_path}")
            except Exception as e:
                logger.warning(f"Could not remove backup {self.backup_path}: {e}")
        # Reset state
        self.temp_path = None
        self.backup_path = None
        self.file_handle = None
@contextmanager


def atomic_write(
    target_path: str | Path,
    mode: str = "w",
    encoding: str | None = "utf-8",
    backup: bool = True,
    temp_dir: str | Path | None = None,
):
    """
    Context manager for atomic file writing.
    Args:
        target_path: Final path where file should be written
        mode: File open mode
        encoding: Text encoding (for text modes)
        backup: Whether to create backup of existing file
        temp_dir: Directory for temporary files
    Yields:
        File handle for writing
    Example:
        with atomic_write('data.json') as f:
            json.dump(data, f)
    """
    writer = AtomicFileWriter(
        target_path=target_path,
        mode=mode,
        encoding=encoding,
        backup=backup,
        temp_dir=temp_dir,
    )
    with writer as f:
        yield f


def safe_write_text(
    target_path: str | Path,
    content: str,
    encoding: str = "utf-8",
    backup: bool = True,
    append: bool = False,
) -> None:
    """
    Safely write text content to a file atomically.
    Args:
        target_path: Path to write to
        content: Text content to write
        encoding: Text encoding
        backup: Whether to create backup
    """
    target_path = Path(target_path)
    target_path.parent.mkdir(parents=True, exist_ok=True)
    # For append operations, atomic write semantics don't apply cleanly (we are not replacing the file).
    # Tests and typical usage protect append with a lock (see FileLock usage in core tests).
    if append:
        with open(target_path, "a", encoding=encoding) as f:
            f.write(content)
        return
    with atomic_write(target_path, "w", encoding=encoding, backup=backup) as f:
        f.write(content)


def safe_write_bytes(
    target_path: str | Path, content: bytes, backup: bool = True
) -> None:
    """
    Safely write binary content to a file atomically.
    Args:
        target_path: Path to write to
        content: Binary content to write
        backup: Whether to create backup
    """
    with atomic_write(target_path, "wb", encoding=None, backup=backup) as f:
        f.write(content)


def safe_read_text(
    file_path: str | Path, encoding: str = "utf-8", max_size_mb: float = 100.0
) -> str:
    """
    Safely read text content from a file with size validation.
    Args:
        file_path: Path to read from
        encoding: Text encoding
        max_size_mb: Maximum file size in MB (default 100MB)
    Returns:
        Text content of the file
    Raises:
        FileOperationError: If file is too large, doesn't exist, or can't be read
    """
    file_path = Path(file_path)
    # Check if file exists
    if not file_path.exists():
        raise FileOperationError(f"File does not exist: {file_path}")
    # Check file size
    try:
        file_size_bytes = file_path.stat().st_size
        file_size_mb = file_size_bytes / (1024 * 1024)
        if file_size_mb > max_size_mb:
            raise FileOperationError(
                f"File size ({file_size_mb:.1f}MB) exceeds maximum allowed "
                f"({max_size_mb}MB): {file_path}"
            )
        logger.debug(f"Reading text file {file_path} ({file_size_mb:.1f}MB)")
    except OSError as e:
        raise FileOperationError(
            f"Could not get file stats for {file_path}: {e}"
        ) from e
    # Read file content
    try:
        with open(file_path, encoding=encoding) as f:
            return f.read()
    except UnicodeDecodeError as e:
        raise FileOperationError(
            f"Encoding error reading '{file_path}' with encoding '{encoding}': {e}"
        ) from e
    except OSError as e:
        raise FileOperationError(f"IOError reading file '{file_path}': {e}") from e


def safe_read_bytes(file_path: str | Path, max_size_mb: float = 100.0) -> bytes:
    """
    Safely read binary content from a file with size validation.
    Args:
        file_path: Path to read from
        max_size_mb: Maximum file size in MB (default 100MB)
    Returns:
        Binary content of the file
    Raises:
        FileOperationError: If file is too large, doesn't exist, or can't be read
    """
    file_path = Path(file_path)
    # Check if file exists
    if not file_path.exists():
        raise FileOperationError(f"File does not exist: {file_path}")
    # Check file size
    try:
        file_size_bytes = file_path.stat().st_size
        file_size_mb = file_size_bytes / (1024 * 1024)
        if file_size_mb > max_size_mb:
            raise FileOperationError(
                f"File size ({file_size_mb:.1f}MB) exceeds maximum allowed "
                f"({max_size_mb}MB): {file_path}"
            )
        logger.debug(f"Reading binary file {file_path} ({file_size_mb:.1f}MB)")
    except OSError as e:
        raise FileOperationError(
            f"Could not get file stats for {file_path}: {e}"
        ) from e
    # Read file content
    try:
        with open(file_path, "rb") as f:
            return f.read()
    except OSError as e:
        raise FileOperationError(f"IOError reading file '{file_path}': {e}") from e


def safe_read_with_fallback(
    file_path: str | Path,
    preferred_encoding: str = "utf-8",
    fallback_encodings: list[str] | None = None,
    max_size_mb: float = 100.0,
) -> str:
    """
    Safely read text file with encoding fallback for robustness.
    Args:
        file_path: Path to read from
        preferred_encoding: Primary encoding to try
        fallback_encodings: List of fallback encodings to try if primary fails
        max_size_mb: Maximum file size in MB
    Returns:
        Text content of the file
    Raises:
        FileOperationError: If file can't be read with any encoding
    """
    if fallback_encodings is None:
        fallback_encodings = ["latin1", "cp1252", "iso-8859-1"]
    # Try encoding first
    try:
        return safe_read_text(file_path, preferred_encoding, max_size_mb)
    except FileOperationError as e:
        if "Encoding error" not in str(e):
            raise  # Re-raise non-encoding errors
    # Try fallback encodings
    for encoding in fallback_encodings:
        try:
            logger.debug(f"Trying fallback encoding {encoding} for {file_path}")
            return safe_read_text(file_path, encoding, max_size_mb)
        except FileOperationError as e:
            if "Encoding error" not in str(e):
                raise  # Re-raise non-encoding errors
            continue  # Try next encoding
    # If all encodings failed, try binary read as last resort
    try:
        logger.warning(f"All text encodings failed for {file_path}, reading as binary")
        binary_content = safe_read_bytes(file_path, max_size_mb)
        # Try to decode with errors='replace' to get some readable content
        return binary_content.decode(preferred_encoding, errors="replace")
    except Exception as e:
        raise FileOperationError(
            f"Could not read file '{file_path}' with any encoding: {e}"
        ) from e
