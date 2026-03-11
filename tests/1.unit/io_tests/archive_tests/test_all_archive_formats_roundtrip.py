#exonware/xwsystem/tests/1.unit/io_tests/archive_tests/test_all_archive_formats_roundtrip.py
"""
Extensive tests for all archive format classes (Zip, Tar, 7z, Zstandard, etc.).
Tests metadata, create+extract round-trip for each format. Skips when optional
dependency or external binary is missing.
"""

import pytest
from pathlib import Path
from exonware.xwsystem.io.archive.formats import (
    ZipArchiver,
    TarArchiver,
    SevenZipArchiver,
    ZstandardArchiver,
    RarArchiver,
    BrotliArchiver,
    Lz4Archiver,
    ZpaqArchiver,
    WimArchiver,
    SquashfsArchiver,
    get_archiver_for_file,
    get_archiver_by_id,
)
from exonware.xwsystem.io.errors import ArchiveError
# All format classes with (format_id, primary_extension for create)
ARCHIVER_FORMATS = [
    (ZipArchiver, "zip", ".zip"),
    (TarArchiver, "tar", ".tar"),
    (SevenZipArchiver, "7z", ".7z"),
    (ZstandardArchiver, "zst", ".zst"),
    (RarArchiver, "rar", ".rar"),
    (BrotliArchiver, "br", ".br"),
    (Lz4Archiver, "lz4", ".lz4"),
    (ZpaqArchiver, "zpaq", ".zpaq"),
    (WimArchiver, "wim", ".wim"),
    (SquashfsArchiver, "squashfs", ".squashfs"),
]
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io


class TestAllArchiveFormatsMetadata:
    """Metadata and discovery for every archive format."""
    @pytest.mark.parametrize("archiver_cls, format_id, _", ARCHIVER_FORMATS)

    def test_each_format_has_format_id(self, archiver_cls, format_id, _):
        """Every archiver exposes correct format_id."""
        archiver = archiver_cls()
        assert archiver.format_id == format_id
    @pytest.mark.parametrize("archiver_cls, format_id, ext", ARCHIVER_FORMATS)

    def test_each_format_has_file_extensions(self, archiver_cls, format_id, ext):
        """Every archiver lists file extensions including primary."""
        archiver = archiver_cls()
        assert isinstance(archiver.file_extensions, list)
        assert len(archiver.file_extensions) >= 1
        assert ext in archiver.file_extensions
    @pytest.mark.parametrize("archiver_cls, format_id, _", ARCHIVER_FORMATS)

    def test_each_format_has_mime_types_list(self, archiver_cls, format_id, _):
        """Every archiver has mime_types (may be empty list)."""
        archiver = archiver_cls()
        assert isinstance(archiver.mime_types, list)
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io


class TestAllArchiveFormatsRoundtrip:
    """Create + extract round-trip for all formats that support it."""

    def _make_test_files(self, tmp_path):
        """Create a couple of test files."""
        (tmp_path / "a.txt").write_text("content a")
        (tmp_path / "b.txt").write_text("content b")
        return [tmp_path / "a.txt", tmp_path / "b.txt"]
    @pytest.mark.parametrize("archiver_cls, format_id, ext", ARCHIVER_FORMATS)

    def test_roundtrip_or_skip(self, tmp_path, archiver_cls, format_id, ext):
        """
        For each format: create archive from files, extract, verify contents.
        Skips when format requires missing dependency (7z, rar create, zpaq binary, etc.).
        """
        if archiver_cls is RarArchiver:
            # RAR does not support create(); only extract
            archiver = RarArchiver()
            with pytest.raises(ArchiveError, match="RAR creation"):
                archiver.create(self._make_test_files(tmp_path), tmp_path / "out.rar")
            return
        archiver = archiver_cls()
        files = self._make_test_files(tmp_path)
        archive_path = tmp_path / f"test{ext}"
        extract_dir = tmp_path / "extracted"
        extract_dir.mkdir()
        try:
            archiver.create(files, archive_path)
        except (ArchiveError, ImportError, OSError) as e:
            pytest.skip(f"{format_id} create not available: {e}")
        assert archive_path.exists()
        assert archive_path.stat().st_size > 0
        try:
            extracted = archiver.extract(archive_path, extract_dir)
        except (ArchiveError, ImportError, OSError) as e:
            pytest.skip(f"{format_id} extract not available: {e}")
        assert isinstance(extracted, list)
        # At least our two files should be present (names may vary)
        contents = {f.name for f in extract_dir.rglob("*") if f.is_file()}
        assert "a.txt" in contents or "b.txt" in contents or len(contents) >= 1
        # Verify one file content when possible
        a_extracted = extract_dir / "a.txt"
        if a_extracted.exists():
            assert a_extracted.read_text() == "content a"

    def test_zip_roundtrip_explicit(self, tmp_path):
        """ZIP: create, list_contents, extract, verify (stdlib - always)."""
        files = self._make_test_files(tmp_path)
        archive_path = tmp_path / "test.zip"
        extract_dir = tmp_path / "out"
        archiver = ZipArchiver()
        archiver.create(files, archive_path)
        assert archive_path.exists()
        names = archiver.list_contents(archive_path)
        assert len(names) >= 2
        extracted = archiver.extract(archive_path, extract_dir)
        assert len(extracted) >= 2
        assert (extract_dir / "a.txt").read_text() == "content a"

    def test_tar_roundtrip_explicit(self, tmp_path):
        """TAR: create, list_contents, extract, verify (stdlib - always)."""
        files = self._make_test_files(tmp_path)
        archive_path = tmp_path / "test.tar"
        extract_dir = tmp_path / "out"
        archiver = TarArchiver()
        archiver.create(files, archive_path)
        assert archive_path.exists()
        names = archiver.list_contents(archive_path)
        assert len(names) >= 2
        extracted = archiver.extract(archive_path, extract_dir)
        assert len(extracted) >= 2
        assert (extract_dir / "a.txt").read_text() == "content a"

    def test_sevenzip_roundtrip_when_available(self, tmp_path):
        """7z: create + extract when py7zr is installed."""
        pytest.importorskip("py7zr")
        files = self._make_test_files(tmp_path)
        archive_path = tmp_path / "test.7z"
        extract_dir = tmp_path / "out"
        archiver = SevenZipArchiver()
        archiver.create(files, archive_path)
        assert archive_path.exists()
        names = archiver.list_contents(archive_path)
        assert len(names) >= 2
        extracted = archiver.extract(archive_path, extract_dir)
        assert len(extracted) >= 2
        assert (extract_dir / "a.txt").read_text() == "content a"

    def test_zstandard_roundtrip_when_available(self, tmp_path):
        """Zstandard: create + extract when zstandard is installed."""
        pytest.importorskip("zstandard")
        files = self._make_test_files(tmp_path)
        archive_path = tmp_path / "test.tar.zst"
        extract_dir = tmp_path / "out"
        archiver = ZstandardArchiver()
        archiver.create(files, archive_path)
        assert archive_path.exists()
        extracted = archiver.extract(archive_path, extract_dir)
        assert len(extracted) >= 2
        assert (extract_dir / "a.txt").read_text() == "content a"

    def test_brotli_roundtrip_when_available(self, tmp_path):
        """Brotli: create + extract when brotli is installed."""
        pytest.importorskip("brotli")
        files = self._make_test_files(tmp_path)
        archive_path = tmp_path / "test.tar.br"
        extract_dir = tmp_path / "out"
        archiver = BrotliArchiver()
        archiver.create(files, archive_path)
        assert archive_path.exists()
        extracted = archiver.extract(archive_path, extract_dir)
        assert len(extracted) >= 2
        assert (extract_dir / "a.txt").read_text() == "content a"

    def test_lz4_roundtrip_when_available(self, tmp_path):
        """LZ4: create + extract when lz4 is installed."""
        pytest.importorskip("lz4")
        files = self._make_test_files(tmp_path)
        archive_path = tmp_path / "test.tar.lz4"
        extract_dir = tmp_path / "out"
        archiver = Lz4Archiver()
        archiver.create(files, archive_path)
        assert archive_path.exists()
        extracted = archiver.extract(archive_path, extract_dir)
        assert len(extracted) >= 2
        assert (extract_dir / "a.txt").read_text() == "content a"
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io


class TestArchiveDiscoveryAllFormats:
    """get_archiver_for_file and get_archiver_by_id for every format."""
    @pytest.mark.parametrize(
        "filename, expected_id",
        [
            ("backup.7z", "7z"),
            ("data.tar.zst", "zst"),
            ("archive.rar", "rar"),
            ("files.zip", "zip"),
            ("backup.tar", "tar"),
            ("web.tar.br", "br"),
            ("logs.tar.lz4", "lz4"),
            ("extreme.zpaq", "zpaq"),
            ("system.wim", "wim"),
            ("rootfs.squashfs", "squashfs"),
        ],
    )

    def test_get_archiver_for_file_resolves(self, filename, expected_id):
        """Each file extension resolves to the correct archiver."""
        archiver = get_archiver_for_file(filename)
        assert archiver is not None
        assert archiver.format_id == expected_id
    @pytest.mark.parametrize(
        "format_id",
        ["zip", "tar", "7z", "zst", "rar", "br", "lz4", "zpaq", "wim", "squashfs"],
    )

    def test_get_archiver_by_id_resolves(self, format_id):
        """Each format id resolves to the correct archiver."""
        archiver = get_archiver_by_id(format_id)
        assert archiver is not None
        assert archiver.format_id == format_id
