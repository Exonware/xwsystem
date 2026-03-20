#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/io/archive/archivers.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.16
Generation Date: 30-Oct-2025
Archive codecs - In-memory archive processors.
Following FormatAction naming: ZipArchiver, TarArchiver, 7zArchiver
IArchiver extends ICodec:
- Works on ANY data in RAM (not just files!)
- compress() / extract() delegates to encode() / decode()
Priority 1 (Security): Safe archive operations with validation
Priority 2 (Usability): Simple compress/extract API
Priority 3 (Maintainability): Clean codec pattern
Priority 4 (Performance): Efficient in-memory operations
Priority 5 (Extensibility): Easy to add new formats (7z, RAR, etc.)
"""

import sys
import zipfile
import tarfile
import io
from pathlib import Path
from typing import Any
from ..archive.base import AArchiver
from ..contracts import IArchiver, EncodeOptions, DecodeOptions
from ..defs import ArchiveFormat, CodecCapability, CodecCategory
from ..errors import ArchiveError, EncodeError, DecodeError


class ZipArchiver(AArchiver):
    """
    Zip archive codec - operates in MEMORY.
    Follows I→A→XW pattern:
    - I: IArchiver (interface)
    - A: AArchiver (abstract base)
    - XW: XWZipArchiver (concrete implementation)
    Can compress:
    - bytes (raw data)
    - str (text)
    - dict/list (structured data)
    - Any Python objects
    NOT limited to files - works on data in RAM!
    Examples:
        >>> archiver = ZipArchiver()
        >>> 
        >>> # Compress dict in RAM
        >>> data = {"file1.txt": b"content1", "file2.txt": b"content2"}
        >>> zip_bytes = archiver.compress(data)
        >>> 
        >>> # Extract from RAM
        >>> extracted = archiver.extract(zip_bytes)
        >>> 
        >>> # Or use codec methods
        >>> zip_bytes = archiver.encode(data)
        >>> data = archiver.decode(zip_bytes)
    """
    # Codec metadata
    @property

    def codec_id(self) -> str:
        return "zip"
    @property

    def media_types(self) -> list[str]:
        return ["application/zip", "application/x-zip-compressed"]
    @property

    def file_extensions(self) -> list[str]:
        return [".zip"]
    @property

    def capabilities(self) -> CodecCapability:
        return CodecCapability.BIDIRECTIONAL
    @property

    def category(self) -> CodecCategory:
        """Codec category: ARCHIVE."""
        return CodecCategory.ARCHIVE
    @property

    def aliases(self) -> list[str]:
        """Codec aliases."""
        return ["zip", "ZIP"]

    def supports_capability(self, capability: CodecCapability) -> bool:
        """Check capability support."""
        return capability in (CodecCapability.BIDIRECTIONAL, CodecCapability.COMPRESSION)
    # ----------------------------------------------------------------------
    # File-based convenience API (used by archive facade + tests)
    # ----------------------------------------------------------------------
    @property

    def format_id(self) -> str:
        """Format identifier (same as codec_id)."""
        return self.codec_id

    def create(self, files: list[Path], archive_path: Path, compression: int = zipfile.ZIP_DEFLATED) -> Path:
        """
        Create a ZIP archive on disk from a list of files.
        """
        try:
            archive_path.parent.mkdir(parents=True, exist_ok=True)
            with zipfile.ZipFile(archive_path, "w", compression=compression) as zf:
                for file_path in files:
                    zf.write(file_path, arcname=file_path.name)
            return archive_path
        except Exception as e:
            raise ArchiveError(f"Failed to create zip archive '{archive_path}': {e}")

    def list_contents(self, archive_path: Path) -> list[str]:
        """List contents of a ZIP archive on disk."""
        try:
            with zipfile.ZipFile(archive_path, "r") as zf:
                return zf.namelist()
        except Exception as e:
            raise ArchiveError(f"Failed to list zip contents for '{archive_path}': {e}")

    def add_file(self, archive_path: Path, file_path: Path, compression: int = zipfile.ZIP_DEFLATED) -> None:
        """Add a file to an existing ZIP archive."""
        try:
            with zipfile.ZipFile(archive_path, "a", compression=compression) as zf:
                zf.write(file_path, arcname=file_path.name)
        except Exception as e:
            raise ArchiveError(f"Failed to add file to zip '{archive_path}': {e}")

    def encode(self, value: Any, *, options: EncodeOptions | None = None) -> bytes:
        """
        Encode data to zip bytes (in RAM).
        Args:
            value: Data to archive (dict, bytes, list, etc.)
            options: Compression options
        Returns:
            Zip archive bytes
        """
        options = options or {}
        compression = options.get('compression', zipfile.ZIP_DEFLATED)
        try:
            # Create zip in memory
            zip_buffer = io.BytesIO()
            with zipfile.ZipFile(zip_buffer, 'w', compression=compression) as zf:
                if isinstance(value, dict):
                    # dict: keys are filenames, values are contents
                    for filename, content in value.items():
                        if isinstance(content, str):
                            zf.writestr(filename, content)
                        else:
                            zf.writestr(filename, content)
                elif isinstance(value, bytes):
                    # Raw bytes: create single file
                    zf.writestr('data', value)
                elif isinstance(value, str):
                    # String: create single text file
                    zf.writestr('data.txt', value)
                else:
                    # Other: serialize as string
                    zf.writestr('data', str(value))
            return zip_buffer.getvalue()
        except Exception as e:
            raise EncodeError(f"Failed to create zip archive: {e}")

    def decode(self, repr: bytes, *, options: DecodeOptions | None = None) -> Any:
        """
        Decode zip bytes to data (in RAM).
        Args:
            repr: Zip archive bytes
            options: Extraction options
        Returns:
            Extracted data (dict of filename → content)
        """
        try:
            # Extract zip from memory
            zip_buffer = io.BytesIO(repr)
            result = {}
            with zipfile.ZipFile(zip_buffer, 'r') as zf:
                for name in zf.namelist():
                    result[name] = zf.read(name)
            return result
        except Exception as e:
            raise DecodeError(f"Failed to extract zip archive: {e}")

    def compress(self, data: Any, **options) -> bytes:
        """
        User-friendly: Compress data to zip bytes.
        Delegates to encode().
        """
        return self.encode(data, options=options)

    def extract(self, source: bytes | Path, extract_dir: Path | None = None, **options) -> Any:
        """
        Extract ZIP from either in-memory bytes or a file path.
        - If source is bytes: returns dict[str, bytes] via decode()
        - If source is Path: extracts to extract_dir and returns list[Path]
        """
        if isinstance(source, (bytes, bytearray)):
            return self.decode(bytes(source), options=options)
        if extract_dir is None:
            raise ArchiveError("extract_dir is required when extracting from a file path")
        try:
            extract_dir.mkdir(parents=True, exist_ok=True)
            with zipfile.ZipFile(source, "r") as zf:
                zf.extractall(extract_dir)
                return [extract_dir / name for name in zf.namelist()]
        except Exception as e:
            raise ArchiveError(f"Failed to extract zip archive '{source}': {e}")


class TarArchiver(AArchiver):
    """
    Tar archive codec - operates in MEMORY.
    Follows I→A→XW pattern:
    - I: IArchiver (interface)
    - A: AArchiver (abstract base)
    - XW: XWTarArchiver (concrete implementation)
    Similar to XWZipArchiver but uses tar format.
    Supports compression: gzip, bz2, xz
    """
    # Codec metadata
    @property

    def codec_id(self) -> str:
        return "tar"
    @property

    def media_types(self) -> list[str]:
        return ["application/x-tar", "application/x-gtar"]
    @property

    def file_extensions(self) -> list[str]:
        return [".tar", ".tar.gz", ".tgz", ".tar.bz2", ".tar.xz"]
    @property

    def capabilities(self) -> CodecCapability:
        return CodecCapability.BIDIRECTIONAL
    @property

    def category(self) -> CodecCategory:
        """Codec category: ARCHIVE."""
        return CodecCategory.ARCHIVE
    @property

    def aliases(self) -> list[str]:
        """Codec aliases."""
        return ["tar", "TAR"]

    def supports_capability(self, capability: CodecCapability) -> bool:
        """Check capability support."""
        return capability in (CodecCapability.BIDIRECTIONAL, CodecCapability.COMPRESSION)
    # ----------------------------------------------------------------------
    # File-based convenience API (used by archive facade + tests)
    # ----------------------------------------------------------------------
    @property

    def format_id(self) -> str:
        """Format identifier (same as codec_id)."""
        return self.codec_id

    def create(self, files: list[Path], archive_path: Path, mode: str = "w") -> Path:
        """Create a TAR archive on disk from a list of files."""
        try:
            archive_path.parent.mkdir(parents=True, exist_ok=True)
            with tarfile.open(archive_path, mode) as tf:
                for file_path in files:
                    tf.add(file_path, arcname=file_path.name)
            return archive_path
        except Exception as e:
            raise ArchiveError(f"Failed to create tar archive '{archive_path}': {e}")

    def list_contents(self, archive_path: Path) -> list[str]:
        """List contents of a TAR archive on disk."""
        try:
            with tarfile.open(archive_path, "r:*") as tf:
                return [m.name for m in tf.getmembers() if m.name]
        except Exception as e:
            raise ArchiveError(f"Failed to list tar contents for '{archive_path}': {e}")

    def add_file(self, archive_path: Path, file_path: Path) -> None:
        """Add a file to an existing TAR archive (append mode)."""
        try:
            with tarfile.open(archive_path, "a") as tf:
                tf.add(file_path, arcname=file_path.name)
        except Exception as e:
            raise ArchiveError(f"Failed to add file to tar '{archive_path}': {e}")

    def encode(self, value: Any, *, options: EncodeOptions | None = None) -> bytes:
        """Encode data to tar bytes (in RAM)."""
        options = options or {}
        compression = options.get('compression', '')  # '', 'gz', 'bz2', 'xz'
        mode = f'w:{compression}' if compression else 'w'
        try:
            tar_buffer = io.BytesIO()
            with tarfile.open(fileobj=tar_buffer, mode=mode) as tf:
                if isinstance(value, dict):
                    for filename, content in value.items():
                        info = tarfile.TarInfo(name=filename)
                        if isinstance(content, str):
                            content = content.encode('utf-8')
                        info.size = len(content)
                        tf.addfile(info, io.BytesIO(content))
                elif isinstance(value, bytes):
                    info = tarfile.TarInfo(name='data')
                    info.size = len(value)
                    tf.addfile(info, io.BytesIO(value))
                else:
                    content = str(value).encode('utf-8')
                    info = tarfile.TarInfo(name='data')
                    info.size = len(content)
                    tf.addfile(info, io.BytesIO(content))
            return tar_buffer.getvalue()
        except Exception as e:
            raise EncodeError(f"Failed to create tar archive: {e}")

    def decode(self, repr: bytes, *, options: DecodeOptions | None = None) -> Any:
        """Decode tar bytes to data (in RAM)."""
        try:
            tar_buffer = io.BytesIO(repr)
            result = {}
            with tarfile.open(fileobj=tar_buffer, mode='r:*') as tf:
                for member in tf.getmembers():
                    if member.isfile():
                        f = tf.extractfile(member)
                        if f:
                            result[member.name] = f.read()
            return result
        except Exception as e:
            raise DecodeError(f"Failed to extract tar archive: {e}")

    def compress(self, data: Any, **options) -> bytes:
        """User-friendly: Compress data to tar bytes."""
        return self.encode(data, options=options)

    def extract(self, source: bytes | Path, extract_dir: Path | None = None, **options) -> Any:
        """
        Extract TAR from either in-memory bytes or a file path.
        - If source is bytes: returns dict[str, bytes] via decode()
        - If source is Path: extracts to extract_dir and returns list[Path]
        """
        if isinstance(source, (bytes, bytearray)):
            return self.decode(bytes(source), options=options)
        if extract_dir is None:
            raise ArchiveError("extract_dir is required when extracting from a file path")
        try:
            extract_dir.mkdir(parents=True, exist_ok=True)
            with tarfile.open(source, "r:*") as tf:
                # Use data filter for Python 3.12+ compatibility (prevents deprecation warning)
                # data_filter allows all files but validates paths for security
                # For older Python versions, filter parameter is not available
                extract_kwargs = {}
                extract_kwargs['filter'] = 'data'
                tf.extractall(extract_dir, **extract_kwargs)
                return [extract_dir / m.name for m in tf.getmembers() if m.name]
        except Exception as e:
            raise ArchiveError(f"Failed to extract tar archive '{source}': {e}")
