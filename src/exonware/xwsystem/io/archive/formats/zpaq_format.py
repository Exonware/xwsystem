#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/zpaq_format.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.29
Generation Date: November 1, 2025
ZPAQ journaled compression format - RANK #8 EXTREME COMPRESSION.
**Extreme compression ratio, slow, archival only**
Priority 1 (Security): Journaled integrity
Priority 2 (Usability): Extreme compression
Priority 3 (Maintainability): Clean zpaq handling
Priority 4 (Performance): Slow (archival only)
Priority 5 (Extensibility): Lazy installation of zpaq
"""

from pathlib import Path
import subprocess
import shutil
from ...contracts import IArchiveFormat
from ...errors import ArchiveError
# Optional: use pyzpaq when installed (e.g. via xwlazy[archives])
try:
    import pyzpaq as _pyzpaq
except ImportError:
    _pyzpaq = None  # type: ignore


class ZpaqArchiver(IArchiveFormat):
    """
    ZPAQ archive format handler - RANK #8.
    FormatAction naming: ZpaqArchiver
    Extreme archival compression with:
    - Best compression ratio (PAQ algorithm)
    - Journaled incremental backups
    - Deduplication
    - Very slow (archival only)
    NOTE: Requires zpaq binary installed on system.
    Examples:
        >>> archiver = ZpaqArchiver()
        >>> # Create archive (very slow!)
        >>> archiver.create([Path("data/")], Path("archive.zpaq"))
        >>> 
        >>> # Extract
        >>> archiver.extract(Path("archive.zpaq"), Path("output/"))
    """
    @property

    def format_id(self) -> str:
        """Format identifier."""
        return "zpaq"
    @property

    def file_extensions(self) -> list[str]:
        """Supported extensions."""
        return [".zpaq"]
    @property

    def mime_types(self) -> list[str]:
        """MIME types."""
        return ["application/x-zpaq"]

    def _get_zpaq_backend(self) -> tuple[str, Path | None]:
        """Return ('pyzpaq', None) when pyzpaq is available, else ('binary', Path)."""
        if _pyzpaq is not None:
            return ("pyzpaq", None)
        zpaq_path = shutil.which("zpaq")
        if zpaq_path:
            return ("binary", Path(zpaq_path))
        raise ArchiveError(
            "ZPAQ not available. Install pyzpaq (pip install pyzpaq) or the zpaq binary from: http://mattmahoney.net/dc/zpaq.html"
        )

    def create(self, files: list[Path], output: Path, **opts) -> None:
        """
        Create ZPAQ archive.
        Options:
            method: Compression method (0-5, default: 3)
            incremental: Enable incremental backup
        """
        output.parent.mkdir(parents=True, exist_ok=True)
        backend, zpaq_path = self._get_zpaq_backend()
        if backend == "pyzpaq":
            try:
                # pyzpaq API: add(archive_path, file_paths, method=...)
                file_strs = [str(f) for f in files if f.exists()]
                if hasattr(_pyzpaq, "add"):
                    _pyzpaq.add(str(output), file_strs, method=opts.get("method", 3))
                else:
                    raise ArchiveError("pyzpaq.add not found")
            except Exception as e:
                raise ArchiveError(f"ZPAQ creation failed (pyzpaq): {e}")
            return
        method = opts.get("method", 3)
        try:
            cmd = [str(zpaq_path), "add", str(output), "-method", str(method)]
            for file_path in files:
                if file_path.exists():
                    cmd.append(str(file_path))
            subprocess.run(cmd, capture_output=True, text=True, check=True)
        except subprocess.CalledProcessError as e:
            raise ArchiveError(f"ZPAQ creation failed: {e.stderr}")
        except Exception as e:
            raise ArchiveError(f"Failed to create zpaq archive: {e}")

    def extract(self, archive: Path, output_dir: Path, members: list[str] | None = None, **opts) -> list[Path]:
        """Extract ZPAQ archive."""
        output_dir.mkdir(parents=True, exist_ok=True)
        backend, zpaq_path = self._get_zpaq_backend()
        if backend == "pyzpaq":
            try:
                if hasattr(_pyzpaq, "extract"):
                    _pyzpaq.extract(str(archive), str(output_dir))
                else:
                    raise ArchiveError("pyzpaq.extract not found")
                return list(output_dir.rglob("*"))
            except Exception as e:
                raise ArchiveError(f"ZPAQ extraction failed (pyzpaq): {e}")
        try:
            cmd = [str(zpaq_path), "extract", str(archive), "-to", str(output_dir)]
            if members:
                cmd.extend(members)
            subprocess.run(cmd, capture_output=True, text=True, check=True)
            return list(output_dir.rglob("*"))
        except subprocess.CalledProcessError as e:
            raise ArchiveError(f"ZPAQ extraction failed: {e.stderr}")
        except Exception as e:
            raise ArchiveError(f"Failed to extract zpaq archive: {e}")

    def list_contents(self, archive: Path) -> list[str]:
        """List ZPAQ contents."""
        backend, zpaq_path = self._get_zpaq_backend()
        if backend == "pyzpaq":
            try:
                if hasattr(_pyzpaq, "list"):
                    return _pyzpaq.list(str(archive))
                if hasattr(_pyzpaq, "list_archive"):
                    return _pyzpaq.list_archive(str(archive))
                return []
            except Exception as e:
                raise ArchiveError(f"ZPAQ list failed (pyzpaq): {e}")
        try:
            cmd = [str(zpaq_path), "list", str(archive)]
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            lines = result.stdout.strip().split("\n")
            return [line.split()[-1] for line in lines if line.strip()]
        except subprocess.CalledProcessError as e:
            raise ArchiveError(f"ZPAQ list failed: {e.stderr}")
        except Exception as e:
            raise ArchiveError(f"Failed to list zpaq contents: {e}")

    def add_file(self, archive: Path, file: Path, arcname: str | None = None) -> None:
        """Add file to ZPAQ archive (incremental)."""
        backend, zpaq_path = self._get_zpaq_backend()
        if backend == "pyzpaq":
            try:
                if hasattr(_pyzpaq, "add"):
                    _pyzpaq.add(str(archive), [str(file)])
                else:
                    raise ArchiveError("pyzpaq.add not found")
            except Exception as e:
                raise ArchiveError(f"ZPAQ add failed (pyzpaq): {e}")
            return
        try:
            cmd = [str(zpaq_path), "add", str(archive), str(file)]
            subprocess.run(cmd, capture_output=True, text=True, check=True)
        except subprocess.CalledProcessError as e:
            raise ArchiveError(f"ZPAQ add failed: {e.stderr}")
        except Exception as e:
            raise ArchiveError(f"Failed to add file to zpaq: {e}")
