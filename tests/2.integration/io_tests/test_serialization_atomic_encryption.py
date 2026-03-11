#exonware/xwsystem/tests/2.integration/io_tests/test_serialization_atomic_encryption.py
"""
Integration tests: atomic access with different serialization setups (plain, encrypted).
Ensures encryption never bypasses atomic write (temp file + rename).
"""

import pytest
from pathlib import Path
from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
from exonware.xwsystem.io.serialization.formats.text.jsonlines import JsonLinesSerializer
# Sample data for round-trip tests
JSONL_DATA = [
    {"id": 1, "name": "Alice", "role": "admin"},
    {"id": 2, "name": "Bob", "role": "user"},
]
JSON_DATA = {"config": {"host": "localhost", "port": 8080}, "users": ["a", "b"]}
@pytest.mark.xwsystem_integration
@pytest.mark.xwsystem_io


class TestSerializationAtomicEncryptionSetups:
    """Run save/load with different setups: atomic, encrypted, format combinations."""

    def test_jsonl_plain_atomic(self, tmp_path):
        """JSONL: plain save/load with atomic=True (default)."""
        path = tmp_path / "data.jsonl"
        ser = JsonLinesSerializer()
        ser.save_file(JSONL_DATA, path, atomic=True, backup=False)
        assert path.exists()
        loaded = ser.load_file(path)
        assert loaded == JSONL_DATA

    def test_jsonl_plain_atomic_explicit_backup(self, tmp_path):
        """JSONL: atomic with backup=True."""
        path = tmp_path / "data.jsonl"
        ser = JsonLinesSerializer()
        ser.save_file(JSONL_DATA, path, atomic=True, backup=True)
        loaded = ser.load_file(path)
        assert loaded == JSONL_DATA

    def test_jsonl_encrypted_atomic(self, tmp_path):
        """JSONL: encrypted save/load with atomic write (encryption must not bypass atomic)."""
        path = tmp_path / "encrypted.jsonl"
        ser = JsonLinesSerializer()
        enc_opts = {
            "password": "test-secret-jsonl",
            "algorithm": "aes256-gcm",
        }
        ser.save_file(JSONL_DATA, path, encryption=enc_opts, atomic=True, backup=False)
        assert path.exists()
        # File should be binary (encrypted), not readable as text
        raw = path.read_bytes()
        assert b"Alice" not in raw and b"Bob" not in raw
        loaded = ser.load_file(path, encryption=enc_opts)
        assert loaded == JSONL_DATA

    def test_jsonl_encrypted_atomic_roundtrip_twice(self, tmp_path):
        """JSONL: write encrypted twice (overwrite) with atomic, then load."""
        path = tmp_path / "data.jsonl"
        ser = JsonLinesSerializer()
        enc = {"password": "pwd2", "algorithm": "aes256-gcm"}
        ser.save_file(JSONL_DATA, path, encryption=enc, atomic=True, backup=False)
        other = [{"x": 1}, {"x": 2}]
        ser.save_file(other, path, encryption=enc, atomic=True, backup=False)
        loaded = ser.load_file(path, encryption=enc)
        assert loaded == other

    def test_json_plain_atomic(self, tmp_path):
        """JSON: plain save/load with atomic=True."""
        path = tmp_path / "config.json"
        ser = JsonSerializer()
        ser.save_file(JSON_DATA, path, atomic=True, backup=False)
        loaded = ser.load_file(path)
        assert loaded == JSON_DATA

    def test_json_encrypted_atomic(self, tmp_path):
        """JSON: encrypted save/load with atomic write."""
        path = tmp_path / "encrypted.json"
        ser = JsonSerializer()
        enc_opts = {"password": "json-secret", "algorithm": "aes256-gcm"}
        ser.save_file(JSON_DATA, path, encryption=enc_opts, atomic=True, backup=False)
        assert path.exists()
        raw = path.read_bytes()
        assert b"localhost" not in raw
        loaded = ser.load_file(path, encryption=enc_opts)
        assert loaded == JSON_DATA

    def test_jsonl_atomic_false_plain(self, tmp_path):
        """JSONL: atomic=False still round-trips (direct write)."""
        path = tmp_path / "direct.jsonl"
        ser = JsonLinesSerializer()
        ser.save_file(JSONL_DATA, path, atomic=False, backup=False)
        loaded = ser.load_file(path)
        assert loaded == JSONL_DATA

    def test_jsonl_archived_gzip_atomic(self, tmp_path):
        """JSONL: save/load with archive=gzip (pipeline compression), atomic write."""
        path = tmp_path / "data.jsonl.gz"
        ser = JsonLinesSerializer()
        ser.save_file(JSONL_DATA, path, archive="gzip", atomic=True, backup=False)
        assert path.exists()
        raw = path.read_bytes()
        assert raw[:2] == b"\x1f\x8b"  # gzip magic
        loaded = ser.load_file(path, archive="gzip")
        assert loaded == JSONL_DATA

    def test_json_archived_gzip_atomic(self, tmp_path):
        """JSON: save/load with archive=gzip, atomic write."""
        path = tmp_path / "config.json.gz"
        ser = JsonSerializer()
        ser.save_file(JSON_DATA, path, archive="gzip", atomic=True, backup=False)
        assert path.exists()
        loaded = ser.load_file(path, archive="gzip")
        assert loaded == JSON_DATA
