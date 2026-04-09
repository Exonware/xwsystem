#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/security/file_security.py
Secure File Operations for xwsystem.
Provides generic file security operations that can be used by any library:
- Path validation (REUSES xwsystem.security.PathValidator)
- Resource limits (REUSES xwsystem.security.resource_limits)
- File permission checks
- Secure read/write operations
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.36
Generation Date: 26-Jan-2025
"""

import os
import stat
from pathlib import Path
from .path_validator import PathValidator, PathSecurityError
from .resource_limits import get_resource_limits, GenericLimitError


class FileSecurityError(Exception):
    """Base exception for file security errors."""

    def __init__(self, message: str, path: str | None = None, context: dict | None = None):
        super().__init__(message)
        self.path = path
        self.context = context or {}


class FileSizeLimitError(FileSecurityError):
    """Exception raised when file size exceeds limit."""

    def __init__(self, message: str, size: int, limit: int, path: str | None = None):
        super().__init__(message, path=path, context={'size': size, 'limit': limit})
        self.size = size
        self.limit = limit


class FileIOError(FileSecurityError):
    """Exception raised when file I/O operations fail."""
    pass


class FileSecurity:
    """
    Secure file operations with validation and limits.
    Generic file security class that can be used by any library.
    REUSES xwsystem security features for consistency.
    """

    def __init__(
        self,
        max_file_size: int = 100 * 1024 * 1024,  # 100MB default
        allowed_directories: list[str] | None = None,
        allow_absolute_paths: bool = False
    ):
        """
        Initialize file security.
        Args:
            max_file_size: Maximum file size in bytes
            allowed_directories: List of allowed base directories
            allow_absolute_paths: If True, allow absolute paths
        """
        self._max_file_size = max_file_size
        self._allowed_directories = allowed_directories
        self._allow_absolute_paths = allow_absolute_paths
        self._path_validator = PathValidator(
            base_path=Path(allowed_directories[0]).resolve() if allowed_directories else None,
            allow_absolute=allow_absolute_paths,
            enable_cache=True
        )

    def validate_file_path(
        self,
        path: str | Path,
        operation: str = "access",
        check_exists: bool = False
    ) -> Path:
        """
        Validate file path for security.
        REUSES xwsystem PathValidator for consistency.
        Args:
            path: File path to validate
            operation: Operation being performed
            check_exists: If True, check if file exists
        Returns:
            Validated Path object
        Raises:
            PathSecurityError: If path is invalid
            FileIOError: If file doesn't exist (when check_exists=True)
        """
        path_obj = Path(path)
        # REUSE xwsystem PathValidator
        try:
            for_writing = (operation == "write") or (operation == "convert_file" and not check_exists)
            validated_path = self._path_validator.validate_path(str(path), for_writing=for_writing)
        except PathSecurityError as e:
            raise PathSecurityError(
                f"Path validation failed: {e}",
                path=str(path),
                context={'operation': operation}
            )
        validated_path = Path(validated_path)
        # Check if file exists (if required)
        if check_exists and not validated_path.exists():
            raise FileIOError(
                f"File does not exist: {validated_path}",
                path=str(validated_path)
            )
        return validated_path

    def check_file_size(self, path: str | Path) -> int:
        """
        Check file size and validate against limits.
        REUSES xwsystem resource limits for consistency.
        Args:
            path: File path
        Returns:
            File size in bytes
        Raises:
            FileSizeLimitError: If file exceeds size limit
            FileIOError: If file cannot be accessed
        """
        path_obj = self.validate_file_path(path, operation="size_check", check_exists=True)
        try:
            size = path_obj.stat().st_size
        except OSError as e:
            raise FileIOError(
                f"Cannot access file: {e}",
                path=str(path_obj)
            )
        # REUSE xwsystem resource limits for consistency
        resource_limits = get_resource_limits()
        limits_file_size = getattr(resource_limits, 'max_file_size', self._max_file_size)
        max_file_size = min(self._max_file_size, limits_file_size)
        if size > max_file_size:
            raise FileSizeLimitError(
                f"File size {size} exceeds limit {max_file_size}",
                size=size,
                limit=max_file_size,
                path=str(path_obj)
            )
        return size

    def validate_file_permissions(
        self,
        path: str | Path,
        required_permission: str = "read"
    ) -> None:
        """
        Validate file permissions.
        Args:
            path: File path
            required_permission: Required permission ('read', 'write', 'execute')
        Raises:
            FileIOError: If file doesn't have required permissions
        """
        path_obj = self.validate_file_path(path, operation="permission_check", check_exists=True)
        try:
            file_stat = path_obj.stat()
            file_mode = file_stat.st_mode
        except OSError as e:
            raise FileIOError(
                f"Cannot access file permissions: {e}",
                path=str(path_obj)
            )
        # Check permissions
        if required_permission == "read":
            if not os.access(path_obj, os.R_OK):
                raise FileIOError(
                    "File does not have read permission",
                    path=str(path_obj)
                )
        elif required_permission == "write":
            if not os.access(path_obj, os.W_OK):
                raise FileIOError(
                    "File does not have write permission",
                    path=str(path_obj)
                )
        elif required_permission == "execute":
            if not os.access(path_obj, os.X_OK):
                raise FileIOError(
                    "File does not have execute permission",
                    path=str(path_obj)
                )

    def secure_read_file(self, path: str | Path) -> bytes:
        """
        Securely read a file with validation.
        Args:
            path: File path
        Returns:
            File contents as bytes
        Raises:
            PathSecurityError: If path is invalid
            FileSizeLimitError: If file exceeds size limit
            FileIOError: If file cannot be read
        """
        # Validate path
        validated_path = self.validate_file_path(path, operation="read", check_exists=True)
        # Check file size
        self.check_file_size(validated_path)
        # Check permissions
        self.validate_file_permissions(validated_path, required_permission="read")
        # Read file
        try:
            with open(validated_path, 'rb') as f:
                return f.read()
        except OSError as e:
            raise FileIOError(
                f"Cannot read file: {e}",
                path=str(validated_path)
            )

    def secure_write_file(
        self,
        path: str | Path,
        data: bytes,
        check_size: bool = True
    ) -> None:
        """
        Securely write a file with validation.
        Args:
            path: File path
            data: Data to write
            check_size: If True, check data size against limit
        Raises:
            PathSecurityError: If path is invalid
            FileSizeLimitError: If data exceeds size limit
            FileIOError: If file cannot be written
        """
        # Validate path
        validated_path = self.validate_file_path(path, operation="write")
        # Check data size
        if check_size and len(data) > self._max_file_size:
            raise FileSizeLimitError(
                f"Data size {len(data)} exceeds limit {self._max_file_size}",
                size=len(data),
                limit=self._max_file_size,
                path=str(validated_path)
            )
        # Create parent directory if needed
        validated_path.parent.mkdir(parents=True, exist_ok=True)
        # Write file
        try:
            with open(validated_path, 'wb') as f:
                f.write(data)
        except OSError as e:
            raise FileIOError(
                f"Cannot write file: {e}",
                path=str(validated_path)
            )

    def is_safe_path(self, path: str | Path) -> bool:
        """
        Check if path is safe (doesn't raise exception).
        Args:
            path: Path to check
        Returns:
            True if path is safe
        """
        try:
            self.validate_file_path(path, operation="check")
            return True
        except (PathSecurityError, FileIOError):
            return False
# Global file security instance
_file_security: FileSecurity | None = None


def get_file_security() -> FileSecurity:
    """Get global file security instance."""
    global _file_security
    if _file_security is None:
        _file_security = FileSecurity()
    return _file_security


def set_file_security(security: FileSecurity) -> None:
    """Set global file security instance."""
    global _file_security
    _file_security = security
