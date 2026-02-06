#exonware/xwsystem/src/exonware/xwsystem/io/archive/facade.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.5
Generation Date: January 2026

XWArchive - Unified Archiving Facade

Simplified API for all archive formats:
- ZIP, TAR, TAR.GZ, TAR.BZ2, TAR.XZ, 7Z, RAR, etc.
- Auto-detect format from file path
- Compression support
"""

from pathlib import Path
from typing import Any, Optional, Union, List

from .archive import Archive
from .archivers import ZipArchiver, TarArchiver
from .formats import get_archiver_for_file, get_archiver_by_id
from .archive_files import ZipFile, TarFile
from ..defs import ArchiveFormat, CompressionAlgorithm, CompressionLevel
from ...config.logging_setup import get_logger

logger = get_logger(__name__)


class XWArchive:
    """
    Unified archiving facade - simple API for all archive formats.
    
    Examples:
        >>> # Format-specific
        >>> archive = XWArchive("zip")  # or format="zip"
        >>> archive = XWArchive("tar.gz")
        >>> archive = XWArchive("7z")
        
        >>> # Auto-detect from path
        >>> archive = XWArchive("backup.zip")  # Detects ZIP
        
        >>> # With compression
        >>> archive = XWArchive("tar.gz", compression="gzip", level="best")
        
        >>> # Quick operations
        >>> XWArchive.create("files.zip", ["file1.txt", "file2.txt"])
        >>> XWArchive.extract("archive.zip", "output_dir")
        
        >>> # File operations
        >>> with XWArchive("backup.zip") as arch:
        ...     arch.add("file.txt")
        ...     arch.extract("file.txt", "output/")
    """
    
    def __init__(
        self,
        format_or_path: Union[str, Path],
        *,
        format: Optional[str] = None,
        compression: Optional[str] = None,
        level: Union[str, int] = "balanced",
        **kwargs
    ):
        """
        Initialize unified archive.
        
        Args:
            format_or_path: Format name ("zip", "tar.gz") or file path ("backup.zip")
            format: Explicit format name (overrides auto-detection)
            compression: Compression algorithm ("gzip", "bzip2", "lzma", "zstd", etc.)
            level: Compression level ("fast", "balanced", "best") or int (0-9)
            **kwargs: Additional archive options
        """
        self.format_or_path = format_or_path
        self.format = format
        self.compression = compression
        self.level = level
        self.kwargs = kwargs
        
        # Auto-detect format from path if it looks like a file path
        if isinstance(format_or_path, (str, Path)):
            path = Path(format_or_path)
            # Check if it's a file path (has extension or path separators)
            is_file_path = path.suffix or "/" in str(format_or_path) or "\\" in str(format_or_path)
            
            if is_file_path and not format:
                # Looks like a file path - try to detect format from extension
                try:
                    detected = get_archiver_for_file(str(path))
                    if detected:
                        self.format = detected.codec_id
                        self.file_path = path
                    else:
                        # Fallback: extract format from extension
                        ext = path.suffix.lstrip('.').lower()
                        if ext in ('zip', 'tar', 'gz', 'bz2', 'xz', '7z', 'rar'):
                            self.format = ext
                            self.file_path = path
                        else:
                            self.format = "zip"  # Default
                            self.file_path = path
                except Exception:
                    # Fallback: extract format from extension
                    ext = path.suffix.lstrip('.').lower()
                    self.format = ext if ext else "zip"
                    self.file_path = path
            else:
                # Use as format name
                self.format = format or str(format_or_path)
        
        # Determine compression level
        if isinstance(level, str):
            level_map = {
                "fast": CompressionLevel.FAST,
                "balanced": CompressionLevel.BALANCED,
                "best": CompressionLevel.BEST,
            }
            self.compression_level = level_map.get(level.lower(), CompressionLevel.BALANCED)
        else:
            self.compression_level = level
        
        # Create archiver instance
        self._archiver = self._create_archiver()
        self._archive_file = None
    
    def _create_archiver(self):
        """Create archiver instance based on format."""
        if self.format:
            try:
                archiver = get_archiver_by_id(self.format.lower())
                if archiver:
                    return archiver()
            except Exception:
                pass
        
        # Fallback to common formats
        format_lower = (self.format or "").lower()
        
        if format_lower in ("zip", ".zip"):
            return ZipArchiver()
        elif format_lower in ("tar", ".tar"):
            return TarArchiver()
        elif format_lower in ("tar.gz", ".tar.gz", "targz"):
            return TarArchiver()  # TarArchiver handles compression
        elif format_lower in ("tar.bz2", ".tar.bz2", "tarbz2"):
            return TarArchiver()
        elif format_lower in ("tar.xz", ".tar.xz", "tarxz"):
            return TarArchiver()
        else:
            # Try to get from registry
            try:
                return get_archiver_by_id(format_lower)()
            except Exception:
                raise ValueError(f"Unknown archive format: {self.format}")
    
    def create(self, archive_path: Union[str, Path], files: List[Union[str, Path]], **options) -> Path:
        """
        Create archive from files.
        
        Args:
            archive_path: Path to create archive at
            files: List of files/directories to archive
            **options: Additional archive options
            
        Returns:
            Path to created archive
        """
        archive_path = Path(archive_path)
        files = [Path(f) for f in files]
        
        # Use Archive facade for file operations
        archive = Archive()
        archive.create(files, archive_path, format=self.format or None, **options)
        
        return archive_path
    
    def extract(self, archive_path: Union[str, Path], output_dir: Union[str, Path], **options) -> Path:
        """
        Extract archive to directory.
        
        Args:
            archive_path: Path to archive
            output_dir: Directory to extract to
            **options: Additional extraction options
            
        Returns:
            Output directory path
        """
        archive_path = Path(archive_path)
        output_dir = Path(output_dir)
        
        # Use Archive facade
        archive = Archive()
        archive.extract(archive_path, output_dir, **options)
        
        return output_dir
    
    def add(self, file_path: Union[str, Path], **options) -> None:
        """Add file to archive (when used as context manager)."""
        if not hasattr(self, 'file_path') or not self.file_path:
            raise ValueError("No archive file path specified. Initialize with file path for context manager usage.")
        
        # Use Archive facade to add file
        archive = Archive()
        archive.add_file(self.file_path, Path(file_path), **options)
    
    def list_contents(self) -> List[str]:
        """List files in archive."""
        if not self._archive_file:
            raise ValueError("Archive not opened")
        return self._archive_file.list_contents()
    
    def __enter__(self):
        """Context manager entry."""
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit."""
        if self._archive_file:
            # Archive files handle their own cleanup
            pass
    
    @classmethod
    def create_archive(
        cls,
        archive_path: Union[str, Path],
        files: List[Union[str, Path]],
        format: Optional[str] = None,
        **options
    ) -> Path:
        """
        Static method to create archive.
        
        Examples:
            >>> XWArchive.create_archive("files.zip", ["file1.txt", "file2.txt"])
        """
        # Auto-detect format from path
        archive_path = Path(archive_path)
        if not format:
            format = archive_path.suffix.lstrip('.') or "zip"
        
        archive = cls(format, **options)
        return archive.create(archive_path, files, **options)
    
    @classmethod
    def extract_archive(
        cls,
        archive_path: Union[str, Path],
        output_dir: Union[str, Path],
        **options
    ) -> Path:
        """
        Static method to extract archive.
        
        Examples:
            >>> XWArchive.extract_archive("archive.zip", "output_dir")
        """
        archive_path = Path(archive_path)
        format = archive_path.suffix.lstrip('.') or "zip"
        
        archive = cls(format, **options)
        return archive.extract(archive_path, output_dir, **options)
