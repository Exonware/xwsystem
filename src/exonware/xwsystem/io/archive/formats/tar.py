#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/tar.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.7
Generation Date: 30-Oct-2025
TAR archive format implementation.
Priority 1 (Security): Safe TAR operations
Priority 2 (Usability): Simple TAR API
Priority 3 (Maintainability): Clean TAR handling
Priority 4 (Performance): Efficient TAR operations
Priority 5 (Extensibility): Registered via registry
"""

import sys
import tarfile
from pathlib import Path
from ...contracts import IArchiveFormat


class TarArchiver(IArchiveFormat):
    """TAR archive format handler (supports tar, tar.gz, tar.bz2, tar.xz)."""
    @property

    def format_id(self) -> str:
        """Format identifier."""
        return "tar"
    @property

    def file_extensions(self) -> list[str]:
        """Supported extensions."""
        return [".tar", ".tar.gz", ".tgz", ".tar.bz2", ".tbz2", ".tar.xz", ".txz"]
    @property

    def mime_types(self) -> list[str]:
        """MIME types."""
        return [
            "application/x-tar",
            "application/x-gtar",
            "application/x-compressed-tar",
        ]

    def _determine_mode(self, path: Path, write: bool = True) -> str:
        """Determine TAR mode from file extension."""
        suffix = "".join(path.suffixes).lower()
        if write:
            if '.gz' in suffix or '.tgz' in suffix:
                return 'w:gz'
            elif '.bz2' in suffix or '.tbz' in suffix:
                return 'w:bz2'
            elif '.xz' in suffix or '.txz' in suffix:
                return 'w:xz'
            else:
                return 'w'
        else:
            return 'r:*'  # Auto-detect compression on read

    def create(self, files: list[Path], output: Path, **opts) -> None:
        """Create TAR archive."""
        output.parent.mkdir(parents=True, exist_ok=True)
        mode = self._determine_mode(output, write=True)
        with tarfile.open(output, mode) as tf:
            for file_path in files:
                arcname = opts.get('arcname', file_path.name)
                tf.add(file_path, arcname=arcname)

    def extract(self, archive: Path, output_dir: Path, members: list[str] | None = None, **opts) -> list[Path]:
        """Extract TAR archive."""
        output_dir.mkdir(parents=True, exist_ok=True)
        extracted: list[Path] = []
        with tarfile.open(archive, 'r:*') as tf:
            if members:
                for member in members:
                    tf.extract(member, output_dir, filter='data')
                    extracted.append(output_dir / member)
            else:
                # Use data filter for Python 3.14+ compatibility (prevents deprecation warning)
                # data_filter allows all files but validates paths for security
                tf.extractall(output_dir, filter='data')
                extracted = [output_dir / member.name for member in tf.getmembers()]
        return extracted

    def list_contents(self, archive: Path) -> list[str]:
        """List TAR contents."""
        with tarfile.open(archive, 'r:*') as tf:
            return [member.name for member in tf.getmembers()]

    def add_file(self, archive: Path, file: Path, arcname: str | None = None) -> None:
        """Add file to TAR archive (supports compressed archives)."""
        arcname = arcname or file.name
        # Uncompressed TAR - direct append
        if archive.suffix == '.tar':
            with tarfile.open(archive, 'a') as tf:
                tf.add(file, arcname=arcname)
        else:
            # Compressed TAR - decompress, append, recompress
            import tempfile
            import shutil
            # Determine compression type
            suffix = "".join(archive.suffixes).lower()
            compression = None
            if '.gz' in suffix or '.tgz' in suffix:
                compression = 'gz'
            elif '.bz2' in suffix or '.tbz' in suffix:
                compression = 'bz2'
            elif '.xz' in suffix or '.txz' in suffix:
                compression = 'xz'
            if compression is None:
                raise ValueError(f"Unsupported compression format: {suffix}")
            # Create temporary uncompressed TAR
            with tempfile.NamedTemporaryFile(suffix='.tar', delete=False) as tmp_tar:
                tmp_tar_path = Path(tmp_tar.name)
            try:
                # Extract existing archive to temporary TAR
                if archive.exists():
                    with tarfile.open(archive, 'r:*') as source_tf:
                        with tarfile.open(tmp_tar_path, 'w') as tmp_tf:
                            for member in source_tf.getmembers():
                                tmp_tf.addfile(member, source_tf.extractfile(member))
                # Add new file to temporary TAR
                with tarfile.open(tmp_tar_path, 'a') as tmp_tf:
                    tmp_tf.add(file, arcname=arcname)
                # Recompress to original archive
                mode_map = {'gz': 'w:gz', 'bz2': 'w:bz2', 'xz': 'w:xz'}
                with tarfile.open(tmp_tar_path, 'r') as tmp_tf:
                    with tarfile.open(archive, mode_map[compression]) as compressed_tf:
                        for member in tmp_tf.getmembers():
                            compressed_tf.addfile(member, tmp_tf.extractfile(member))
            finally:
                # Clean up temporary file
                if tmp_tar_path.exists():
                    tmp_tar_path.unlink()
