#exonware/xwsystem/src/exonware/xwsystem/io/serialization/services/archive.py
"""
Archive (compression) service for serialization pipeline: bytes -> compressed bytes.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Uses stdlib gzip and optional zstd/lz4 for stream compression.
Enables save_file(..., archive="zst") or archive="gzip".
"""

from __future__ import annotations
import gzip
from typing import Any, Optional
# Magic bytes for auto-detect
GZIP_MAGIC = b"\x1f\x8b"
ZST_MAGIC = b"\x28\xb5\x2f\xfd"
LZ4_MAGIC_PREFIX = b"\x04\x22\x4d\x18"  # LZ4 frame


class ArchiveService:
    """
    Bytes-in, bytes-out compression for the serialization pipeline.
    Supports gzip (stdlib), zst (zstandard), lz4 when available.
    """

    def compress(self, data: bytes, *, format: str = "gzip", **kwargs: Any) -> bytes:
        """
        Compress data. format: 'gzip', 'zst', 'lz4'.
        """
        fmt = format.lower()
        if fmt in ("gzip", "gz"):
            return gzip.compress(data, **kwargs)
        if fmt in ("zst", "zstd"):
            return self._zstd_compress(data, **kwargs)
        if fmt == "lz4":
            return self._lz4_compress(data, **kwargs)
        raise ValueError(f"Unsupported archive format: {format}")

    def decompress(self, data: bytes, *, format: Optional[str] = None, **kwargs: Any) -> bytes:
        """
        Decompress data. If format is None, auto-detect from magic bytes.
        """
        if format is not None:
            fmt = format.lower()
            if fmt in ("gzip", "gz"):
                return gzip.decompress(data, **kwargs)
            if fmt in ("zst", "zstd"):
                return self._zstd_decompress(data, **kwargs)
            if fmt == "lz4":
                return self._lz4_decompress(data, **kwargs)
            raise ValueError(f"Unsupported archive format: {format}")
        # Auto-detect
        if data[:2] == GZIP_MAGIC:
            return gzip.decompress(data, **kwargs)
        if data[:4] == ZST_MAGIC:
            return self._zstd_decompress(data, **kwargs)
        if data[:4] == LZ4_MAGIC_PREFIX:
            return self._lz4_decompress(data, **kwargs)
        raise ValueError("Could not detect compression format from magic bytes")

    def _zstd_compress(self, data: bytes, **kwargs: Any) -> bytes:
        try:
            import zstandard as zstd
            cctx = zstd.ZstdCompressor(**kwargs)
            return cctx.compress(data)
        except ImportError:
            raise ImportError("zstandard package required for zst compression. pip install zstandard") from None

    def _zstd_decompress(self, data: bytes, **kwargs: Any) -> bytes:
        try:
            import zstandard as zstd
            dctx = zstd.ZstdDecompressor()
            return dctx.decompress(data)
        except ImportError:
            raise ImportError("zstandard package required for zst decompression. pip install zstandard") from None

    def _lz4_compress(self, data: bytes, **kwargs: Any) -> bytes:
        try:
            import lz4.frame
            return lz4.frame.compress(data, **kwargs)
        except ImportError:
            raise ImportError("lz4 package required for lz4 compression. pip install lz4") from None

    def _lz4_decompress(self, data: bytes, **kwargs: Any) -> bytes:
        try:
            import lz4.frame
            return lz4.frame.decompress(data, **kwargs)
        except ImportError:
            raise ImportError("lz4 package required for lz4 decompression. pip install lz4") from None
