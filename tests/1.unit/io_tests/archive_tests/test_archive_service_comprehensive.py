#exonware/xwsystem/tests/1.unit/io_tests/archive_tests/test_archive_service_comprehensive.py
"""
Comprehensive tests for ArchiveService (serialization pipeline compression).
Covers gzip, zstd, lz4 round-trip, auto-detect, error handling, and edge cases.
"""

import pytest
from exonware.xwsystem.io.serialization.services.archive import (
    ArchiveService,
    GZIP_MAGIC,
    ZST_MAGIC,
    LZ4_MAGIC_PREFIX,
)
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io


class TestArchiveServiceComprehensive:
    """Extensive tests for ArchiveService used by the serialization pipeline."""
    @pytest.fixture

    def service(self):
        return ArchiveService()
    @pytest.fixture

    def sample_data(self):
        return b"Hello, World! " * 100
    # -------------------------------------------------------------------------
    # GZIP (stdlib - always available)
    # -------------------------------------------------------------------------

    def test_gzip_roundtrip(self, service, sample_data):
        """Gzip compress and decompress preserves data."""
        compressed = service.compress(sample_data, format="gzip")
        assert isinstance(compressed, bytes)
        assert compressed[:2] == GZIP_MAGIC
        decompressed = service.decompress(compressed, format="gzip")
        assert decompressed == sample_data

    def test_gzip_alias_gz(self, service, sample_data):
        """Format 'gz' works like 'gzip'."""
        compressed = service.compress(sample_data, format="gz")
        assert compressed[:2] == GZIP_MAGIC
        assert service.decompress(compressed, format="gz") == sample_data

    def test_gzip_auto_detect(self, service, sample_data):
        """Decompress with format=None auto-detects gzip from magic."""
        compressed = service.compress(sample_data, format="gzip")
        decompressed = service.decompress(compressed, format=None)
        assert decompressed == sample_data

    def test_gzip_empty_data(self, service):
        """Gzip handles empty bytes."""
        compressed = service.compress(b"", format="gzip")
        assert len(compressed) > 0
        assert service.decompress(compressed, format="gzip") == b""

    def test_gzip_large_data(self, service):
        """Gzip handles larger payload."""
        data = b"x" * (1024 * 1024)  # 1 MiB
        compressed = service.compress(data, format="gzip")
        assert len(compressed) < len(data)
        assert service.decompress(compressed, format="gzip") == data
    # -------------------------------------------------------------------------
    # ZSTD (optional)
    # -------------------------------------------------------------------------

    def test_zstd_roundtrip(self, service, sample_data):
        """Zstd compress and decompress when zstandard is installed."""
        pytest.importorskip("zstandard")
        compressed = service.compress(sample_data, format="zst")
        assert isinstance(compressed, bytes)
        assert compressed[:4] == ZST_MAGIC
        decompressed = service.decompress(compressed, format="zst")
        assert decompressed == sample_data

    def test_zstd_alias_zstd(self, service, sample_data):
        """Format 'zstd' works like 'zst'."""
        pytest.importorskip("zstandard")
        compressed = service.compress(sample_data, format="zstd")
        assert service.decompress(compressed, format="zstd") == sample_data

    def test_zstd_auto_detect(self, service, sample_data):
        """Decompress with format=None auto-detects zstd from magic."""
        pytest.importorskip("zstandard")
        compressed = service.compress(sample_data, format="zst")
        decompressed = service.decompress(compressed, format=None)
        assert decompressed == sample_data

    def test_zstd_empty_data(self, service):
        """Zstd handles empty bytes."""
        pytest.importorskip("zstandard")
        compressed = service.compress(b"", format="zst")
        assert service.decompress(compressed, format="zst") == b""
    # -------------------------------------------------------------------------
    # LZ4 (optional)
    # -------------------------------------------------------------------------

    def test_lz4_roundtrip(self, service, sample_data):
        """LZ4 compress and decompress when lz4 is installed."""
        pytest.importorskip("lz4")
        compressed = service.compress(sample_data, format="lz4")
        assert isinstance(compressed, bytes)
        assert compressed[:4] == LZ4_MAGIC_PREFIX
        decompressed = service.decompress(compressed, format="lz4")
        assert decompressed == sample_data

    def test_lz4_auto_detect(self, service, sample_data):
        """Decompress with format=None auto-detects lz4 from magic."""
        pytest.importorskip("lz4")
        compressed = service.compress(sample_data, format="lz4")
        decompressed = service.decompress(compressed, format=None)
        assert decompressed == sample_data

    def test_lz4_empty_data(self, service):
        """LZ4 handles empty bytes."""
        pytest.importorskip("lz4")
        compressed = service.compress(b"", format="lz4")
        assert service.decompress(compressed, format="lz4") == b""
    # -------------------------------------------------------------------------
    # Error handling
    # -------------------------------------------------------------------------

    def test_unsupported_format_compress_raises(self, service, sample_data):
        """Compress with unsupported format raises ValueError."""
        with pytest.raises(ValueError, match="Unsupported archive format"):
            service.compress(sample_data, format="unknown")

    def test_unsupported_format_decompress_raises(self, service):
        """Decompress with unsupported format raises ValueError."""
        with pytest.raises(ValueError, match="Unsupported archive format"):
            service.decompress(b"fake", format="unknown")

    def test_auto_detect_unknown_magic_raises(self, service):
        """Decompress with format=None and unknown magic raises."""
        with pytest.raises(ValueError, match="Could not detect"):
            service.decompress(b"\x00\x01\x02\x03\x04", format=None)

    def test_auto_detect_empty_raises(self, service):
        """Decompress empty bytes with format=None raises (no magic)."""
        with pytest.raises(ValueError, match="Could not detect"):
            service.decompress(b"", format=None)
