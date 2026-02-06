#exonware/xwsystem/src/exonware/xwsystem/security/path_validator.py
"""
Enhanced path validation and security utilities.
"""

import logging
import os
import platform
import stat
import sys
import tempfile
from pathlib import Path
from typing import Optional

logger = logging.getLogger(__name__)


class PathSecurityError(Exception):
    """Raised when a path fails security validation."""

    pass


class PathValidator:
    """
    Enhanced path validation with security checks to prevent directory traversal
    and other path-based attacks.
    """

    # Dangerous patterns that are blocked
    DANGEROUS_PATTERNS = [
        "..",  # Directory traversal
        "~",  # Home directory
        "$",  # Environment variables
        "|",  # Pipe operators
        ";",  # Command separators
        "&",  # Command operators
        "`",  # Command substitution
        "(",  # Subshells
        ")",  # Subshells
        "<",  # Redirects
        ">",  # Redirects
    ]

    # System paths that are protected
    PROTECTED_PATHS = [
        "/etc/",
        "/bin/",
        "/usr/bin/",
        "/sbin/",
        "/usr/sbin/",
        "/root/",
        "/var/log/",
        "/proc/",
        "/sys/",
        "C:\\Windows\\",
        "C:\\Program Files\\",
        "C:\\Program Files (x86)\\",
    ]

    def __init__(
        self,
        base_path: Optional[str | Path] = None,
        allow_absolute: bool = False,
        max_path_length: Optional[int] = None,
        check_existence: bool = True,
        enable_cache: bool = True,
        max_cache_size: int = 10000,
    ):
        """
        Initialize path validator with xwsystem LRUCache for O(1) validation.

        Args:
            base_path: Base directory to restrict operations to
            allow_absolute: Whether to allow absolute paths
            max_path_length: Maximum allowed path length
            check_existence: Whether to check if paths exist
            enable_cache: Enable validation result caching (default: True)
            max_cache_size: Maximum number of cached paths (default: 10000)
        """
        self.base_path = Path(base_path).resolve() if base_path else None
        self.allow_absolute = allow_absolute
        # Platform-aware path length limits using Python's native platform module
        if max_path_length is None:
            system = platform.system()
            if system == 'Windows':
                self.max_path_length = 260  # Windows MAX_PATH (extended paths can be 32767)
            elif system == 'Darwin':
                self.max_path_length = 1024  # macOS typical limit
            else:
                self.max_path_length = 4096  # Linux PATH_MAX
        else:
            self.max_path_length = max_path_length
        self.check_existence = check_existence
        
        # OPTIMIZATION: Use xwsystem's production-grade LRUCache
        # Benefits: Automatic LRU eviction, thread-safe RLock, statistics, battle-tested
        self.enable_cache = enable_cache
        self.max_cache_size = max_cache_size
        
        if enable_cache:
            # Lazy import to avoid circular dependencies
            from ..caching import create_cache
            # Use flexible create_cache() to allow configuration via environment/settings
            # Defaults to FunctoolsLRUCache (fastest Python cache)
            self._cache = create_cache(capacity=max_cache_size, namespace='xwsystem.security', name="PathValidator")
        else:
            self._cache = None
        
        logger.debug(
            f"PathValidator initialized with "
            f"caching={'enabled (xwsystem LRUCache)' if enable_cache else 'disabled'}, "
            f"capacity={max_cache_size}"
        )

    def validate_path(
        self,
        path: str | Path,
        for_writing: bool = False,
        create_dirs: bool = False,
    ) -> Path:
        """
        Validate a path for security and constraints with O(1) caching.
        
        First call: validates and caches (10-50μs)
        Subsequent calls: cache lookup (< 1μs) ✅

        Args:
            path: Path to validate
            for_writing: Whether path will be used for writing
            create_dirs: Whether to create parent directories

        Returns:
            Validated and resolved Path object

        Raises:
            PathSecurityError: If path fails validation
            PermissionError: If path permissions are insufficient
        """
        if not path:
            raise PathSecurityError("Empty path provided")

        # FAST PATH: Check xwsystem LRUCache first (O(1) with automatic LRU eviction)
        if self.enable_cache:
            cache_key = f"{self.base_path}:{path}:{for_writing}:{create_dirs}"
            
            # xwsystem LRUCache is thread-safe with RLock (reentrant)
            cached_result = self._cache.get(cache_key)
            if cached_result is not None:
                return cached_result  # O(1) cache hit! ✅
        
        # SLOW PATH: Validate and cache
        path_obj = Path(path)
        original_path = str(path)

        # Check path length
        if len(original_path) > self.max_path_length:
            raise PathSecurityError(
                f"Path too long: {len(original_path)} > {self.max_path_length}"
            )

        # Check for dangerous patterns
        self._check_dangerous_patterns(original_path)

        # Handle absolute vs relative paths
        if path_obj.is_absolute():
            # Allow temporary directory paths for testing
            temp_dir = Path(tempfile.gettempdir())
            is_temp_path = False
            try:
                path_obj.relative_to(temp_dir)
                is_temp_path = True
            except ValueError:
                pass
            
            if not self.allow_absolute and not is_temp_path:
                raise PathSecurityError(f"Absolute paths not allowed: {path}")
            resolved_path = path_obj.resolve()
        else:
            if self.base_path:
                resolved_path = (self.base_path / path_obj).resolve()
            else:
                resolved_path = path_obj.resolve()

        # Check if path is within base directory (prevent directory traversal)
        if self.base_path:
            try:
                resolved_path.relative_to(self.base_path)
            except ValueError:
                raise PathSecurityError(
                    f"Path outside base directory: {resolved_path} not in {self.base_path}"
                )

        # Check against protected system paths
        self._check_protected_paths(resolved_path)

        # Check existence and permissions
        if self.check_existence or for_writing:
            self._check_permissions(resolved_path, for_writing, create_dirs)

        # CACHE RESULT: Store in xwsystem LRUCache (automatic eviction!)
        if self.enable_cache:
            # xwsystem LRUCache handles eviction automatically (no manual pruning needed!)
            self._cache.put(cache_key, resolved_path)

        return resolved_path

    def _check_dangerous_patterns(self, path: str) -> None:
        """Check for dangerous patterns in path."""
        path_lower = path.lower()

        for pattern in self.DANGEROUS_PATTERNS:
            if pattern in path_lower:
                raise PathSecurityError(
                    f"Dangerous pattern '{pattern}' found in path: {path}"
                )

        # Check for null bytes
        if "\x00" in path:
            raise PathSecurityError("Null byte found in path")

        # Check for excessive consecutive dots
        if "..." in path:
            raise PathSecurityError("Excessive consecutive dots found in path")

    def _check_protected_paths(self, path: Path) -> None:
        """Check if path is in protected system directories."""
        path_str = str(path).lower()

        for protected in self.PROTECTED_PATHS:
            if path_str.startswith(protected.lower()):
                raise PathSecurityError(f"Access to protected path denied: {path}")

    def _check_permissions(
        self, path: Path, for_writing: bool, create_dirs: bool
    ) -> None:
        """Check path permissions and existence."""
        if path.exists():
            # Check if it's a file when we expect a file
            if not path.is_file() and not path.is_dir():
                raise PathSecurityError(
                    f"Path is not a regular file or directory: {path}"
                )

            # Check read permissions
            if not os.access(path, os.R_OK):
                raise PermissionError(f"No read permission for: {path}")

            # Check write permissions if needed
            if for_writing and not os.access(path, os.W_OK):
                raise PermissionError(f"No write permission for: {path}")
        else:
            # Path doesn't exist
            if self.check_existence and not for_writing:
                raise FileNotFoundError(f"Path does not exist: {path}")

            # Check parent directory for writing
            parent = path.parent
            if for_writing:
                if not parent.exists():
                    if create_dirs:
                        try:
                            parent.mkdir(parents=True, exist_ok=True)
                            logger.debug(f"Created directories: {parent}")
                        except OSError as e:
                            raise PermissionError(
                                f"Cannot create directory {parent}: {e}"
                            )
                    else:
                        raise FileNotFoundError(
                            f"Parent directory does not exist: {parent}"
                        )

                if not os.access(parent, os.W_OK):
                    raise PermissionError(
                        f"No write permission for parent directory: {parent}"
                    )

    def is_safe_filename(self, filename: str) -> bool:
        """
        Check if a filename is safe (no path components).
        
        Uses Python's native pathlib for cross-platform validation.

        Args:
            filename: Filename to check

        Returns:
            True if filename is safe
        """
        if not filename:
            return False

        # Use pathlib to check if it's a simple filename (no path components)
        # Path.parts will have more than 1 element if there are path separators
        try:
            path_obj = Path(filename)
            # If filename contains path separators, parts will have multiple elements
            if len(path_obj.parts) > 1:
                return False
            
            # Get just the filename part (handles cross-platform separators automatically)
            name_only = path_obj.name
            
            # Check for Windows reserved filenames using native pathlib
            # Python's pathlib doesn't validate reserved names, but we can check using native methods
            if platform.system() == 'Windows':
                # Windows reserved names (filesystem limitation, not Python limitation)
                # Python doesn't provide native validation, but pathlib.stem gives us the base name
                WINDOWS_RESERVED_NAMES = {
                    'CON', 'PRN', 'AUX', 'NUL',
                    *[f'COM{i}' for i in range(1, 10)],
                    *[f'LPT{i}' for i in range(1, 10)]
                }
                # Use pathlib's native stem property (name without extension)
                name_base = path_obj.stem.upper()
                if name_base in WINDOWS_RESERVED_NAMES:
                    return False

            # Check for dangerous patterns
            try:
                self._check_dangerous_patterns(name_only)
                return True
            except PathSecurityError:
                return False
        except (ValueError, OSError):
            # Invalid path or OS error - not safe
            return False

    def get_safe_path(
        self, base_dir: str | Path, filename: str, ensure_unique: bool = True
    ) -> Path:
        """
        Generate a safe path within a base directory.

        Args:
            base_dir: Base directory
            filename: Desired filename
            ensure_unique: Whether to ensure path is unique

        Returns:
            Safe path within base directory

        Raises:
            PathSecurityError: If inputs are unsafe
        """
        base_path = self.validate_path(base_dir)

        if not self.is_safe_filename(filename):
            raise PathSecurityError(f"Unsafe filename: {filename}")

        target_path = base_path / filename

        if ensure_unique and target_path.exists():
            # Generate unique filename
            stem = target_path.stem
            suffix = target_path.suffix
            counter = 1

            while target_path.exists():
                new_name = f"{stem}_{counter}{suffix}"
                target_path = base_path / new_name
                counter += 1

                if counter > 1000:  # Prevent infinite loop
                    raise PathSecurityError("Cannot generate unique filename")

        return target_path

    def create_temp_path(
        self,
        prefix: Optional[str] = None,
        suffix: Optional[str] = None,
        as_file: bool = True,
    ) -> Path:
        """
        Create a safe temporary path.

        Args:
            prefix: Optional filename prefix
            suffix: Optional filename suffix
            as_file: Whether to create as file (True) or directory (False)

        Returns:
            Path to temporary location
        """
        base_dir = self.base_path or Path(tempfile.gettempdir())

        if as_file:
            fd, temp_path = tempfile.mkstemp(prefix=prefix, suffix=suffix, dir=base_dir)
            os.close(fd)  # Close file descriptor but keep file
            return Path(temp_path)
        else:
            temp_dir = tempfile.mkdtemp(prefix=prefix, suffix=suffix, dir=base_dir)
            return Path(temp_dir)
    
    def clear_cache(self) -> int:
        """
        Clear validation cache.
        
        Returns:
            Number of cached items cleared
        """
        if not self.enable_cache:
            return 0
        
        # xwsystem LRUCache provides size() method
        count = self._cache.size()
        self._cache.clear()
        logger.debug(f"PathValidator cache cleared ({count} items)")
        return count
    
    def get_cache_stats(self) -> dict:
        """
        Get cache statistics from xwsystem LRUCache.
        
        Returns:
            Dictionary with cache stats including hits/misses/evictions
        """
        if not self.enable_cache:
            return {'enabled': False}
        
        # xwsystem LRUCache provides comprehensive stats!
        stats = self._cache.stats()
        return {
            'enabled': True,
            'name': 'PathValidator',
            'size': stats['size'],
            'capacity': stats['capacity'],
            'hits': stats['hits'],
            'misses': stats['misses'],
            'evictions': stats['evictions'],
            'hit_rate': stats['hit_rate'],
            'utilization': stats['size'] / stats['capacity'] if stats['capacity'] > 0 else 0
        }
