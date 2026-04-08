#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/lz4_format.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.35
Generation Date: November 1, 2025
LZ4 compression format - RANK #7 FASTEST COMPRESSION.
**Very fast, lower ratio, real-time archiving**
Priority 1 (Security): Safe compression
Priority 2 (Usability): Extremely fast
Priority 3 (Maintainability): Clean lz4 handling
Priority 4 (Performance): Best speed (slower ratio)
Priority 5 (Extensibility): Optional lz4 wheel (see pyproject [dev]/[full])

Install: ``pip install lz4`` or ``pip install exonware-xwsystem[dev]`` / ``[full]``.
"""

import io
import tarfile
from pathlib import Path

import lz4.frame as lz4_frame

from ...contracts import IArchiveFormat
from ...errors import ArchiveError


class Lz4Archiver(IArchiveFormat):
    """
    LZ4 archive format handler - RANK #7.
    FormatAction naming: Lz4Archiver
    Ultra-fast compression with:
    - Extremely fast compression/decompression
    - Lower compression ratio (trade-off for speed)
    - Ideal for real-time archiving
    - Log rotation, streaming backups
    Examples:
        >>> archiver = Lz4Archiver()
        >>> archiver.create([Path("logs.txt")], Path("logs.tar.lz4"))
        >>>
        >>> # High compression
        >>> archiver.create(files, output, compression_level=12)
        >>>
        >>> # Ultra fast (default)
        >>> archiver.create(files, output, compression_level=0)
    """
    @property

    def format_id(self) -> str:
        """Format identifier."""
        return "lz4"
    @property

    def file_extensions(self) -> list[str]:
        """Supported extensions."""
        return [".lz4", ".tar.lz4", ".tlz4"]
    @property

    def mime_types(self) -> list[str]:
        """MIME types."""
        return ["application/x-lz4"]

    def create(self, files: list[Path], output: Path, **opts) -> None:
        """
        Create LZ4-compressed tar archive.
        Options:
            compression_level: 0 (fastest) to 12 (better ratio)
            block_size: Block size for compression
        """
        output.parent.mkdir(parents=True, exist_ok=True)
        compression_level = opts.get('compression_level', 0)  # Default: ultra fast
        try:
            tar_buffer = io.BytesIO()
            with tarfile.open(fileobj=tar_buffer, mode='w') as tar:
                for file_path in files:
                    if file_path.exists():
                        tar.add(file_path, arcname=file_path.name)
            tar_data = tar_buffer.getvalue()
            compressed = lz4_frame.compress(tar_data, compression_level=compression_level)
            output.write_bytes(compressed)
        except Exception as e:
            raise ArchiveError(f"Failed to create lz4 archive: {e}")

    def extract(self, archive: Path, output_dir: Path, members: list[str] | None = None, **opts) -> list[Path]:
        """Extract LZ4 archive."""
        output_dir.mkdir(parents=True, exist_ok=True)
        try:
            compressed = archive.read_bytes()
            decompressed = lz4_frame.decompress(compressed)
            tar_buffer = io.BytesIO(decompressed)
            extracted = []
            with tarfile.open(fileobj=tar_buffer, mode='r') as tar:
                extract_kwargs = {}
                extract_kwargs['filter'] = 'data'
                if members:
                    for member in members:
                        tar.extract(member, output_dir, **extract_kwargs)
                        extracted.append(output_dir / member)
                else:
                    tar.extractall(output_dir, **extract_kwargs)
                    extracted = [output_dir / m.name for m in tar.getmembers()]
            return extracted
        except Exception as e:
            raise ArchiveError(f"Failed to extract lz4 archive: {e}")

    def list_contents(self, archive: Path) -> list[str]:
        """List LZ4 archive contents."""
        try:
            compressed = archive.read_bytes()
            decompressed = lz4_frame.decompress(compressed)
            tar_buffer = io.BytesIO(decompressed)
            with tarfile.open(fileobj=tar_buffer, mode='r') as tar:
                return [m.name for m in tar.getmembers()]
        except Exception as e:
            raise ArchiveError(f"Failed to list lz4 contents: {e}")

    def add_file(self, archive: Path, file: Path, arcname: str | None = None) -> None:
        """Not supported - recreate archive instead."""
        raise ArchiveError("LZ4 doesn't support append mode. Recreate the archive.")
