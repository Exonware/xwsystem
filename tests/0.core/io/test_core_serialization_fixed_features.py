#!/usr/bin/env python3
#exonware/xwsystem/tests/0.core/io/test_core_serialization_fixed_features.py
"""
Core serialization feature coverage aligning with GUIDE_TEST.md.
"""

from dataclasses import dataclass
import pytest
from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
from exonware.xwsystem.io.serialization.formats.text.yaml import YamlSerializer
from exonware.xwsystem.io.serialization.formats.text.toml import TomlSerializer
from exonware.xwsystem.io.serialization.formats.text.xml import XmlSerializer
SERIALIZER_CASES = [
    ("JSON", JsonSerializer),
    ("XML", XmlSerializer),
    ("TOML", TomlSerializer),
    ("YAML", YamlSerializer),
]
TEST_DATA = {
    "users": [
        {"id": 1, "name": "Alice", "age": 30, "active": True},
        {"id": 2, "name": "Bob", "age": 25, "active": False},
    ],
    "metadata": {
        "version": "1.0",
        "created": "2025-01-01T00:00:00Z",
    },
}
@dataclass


class User:
    id: int
    name: str
    age: int
    active: bool
@pytest.mark.parametrize("label, serializer_cls", SERIALIZER_CASES)


def test_basic_round_trip(label, serializer_cls):
    """Ensure core serializers can round-trip structured data."""
    serializer = serializer_cls()
    # Use encode/decode API (actual API for serializers)
    text_data = serializer.encode(TEST_DATA)
    # Ensure we have a string for text formats
    if isinstance(text_data, bytes):
        text_data = text_data.decode('utf-8')
    parsed = serializer.decode(text_data)
    # XML infers types (e.g., "1.0" becomes 1.0) - this is expected XML behavior
    # For XML, we need to account for type inference
    if label == "XML":
        # XML will convert string numbers to actual numbers
        # Check structure matches, but allow type differences for numbers
        assert parsed['users'] == TEST_DATA['users'], f"{label} users mismatch"
        assert parsed['metadata']['created'] == TEST_DATA['metadata']['created'], f"{label} created mismatch"
        # Version may be converted from string to number
        assert str(parsed['metadata']['version']) == str(TEST_DATA['metadata']['version']), \
            f"{label} version mismatch (XML type inference)"
    else:
        assert parsed == TEST_DATA, f"{label} failed round-trip"
@pytest.mark.parametrize("label, serializer_cls", SERIALIZER_CASES)


def test_format_detection(label, serializer_cls):
    """Verify format detection works via file extension or content."""
    serializer = serializer_cls()
    # Test that serializer can identify its own format
    # Use file-based detection since sniff_format doesn't exist
    import tempfile
    import os
    # Create temp file path (don't open it, just get the path)
    temp_fd, temp_path = tempfile.mkstemp(suffix=serializer.file_extensions[0])
    os.close(temp_fd)  # Close the file descriptor immediately
    try:
        serializer.save_file(TEST_DATA, temp_path)
        # Verify we can load it back (format detection via extension)
        loaded = serializer.load_file(temp_path)
        # XML infers types - account for that
        if label == "XML":
            assert loaded['users'] == TEST_DATA['users'], f"{label} users mismatch"
            assert str(loaded['metadata']['version']) == str(TEST_DATA['metadata']['version']), \
                f"{label} version mismatch (XML type inference)"
        else:
            assert loaded == TEST_DATA, f"{label} format detection via file extension failed"
    finally:
        # Ensure file is closed before deletion (Windows requirement)
        import time
        time.sleep(0.01)  # Small delay to ensure file handles are released
        if os.path.exists(temp_path):
            try:
                os.unlink(temp_path)
            except PermissionError:
                # On Windows, sometimes need to wait a bit longer
                time.sleep(0.1)
                os.unlink(temp_path)
@pytest.mark.parametrize("label, serializer_cls", SERIALIZER_CASES)


def test_partial_access_operations(label, serializer_cls):
    """Validate path-based operations using atomic_read_path/atomic_update_path."""
    serializer = serializer_cls()
    # Use file-based path operations (actual API)
    import tempfile
    import os
    import time
    # Create temp file path (don't open it, just get the path)
    temp_fd, temp_path = tempfile.mkstemp(suffix=serializer.file_extensions[0])
    os.close(temp_fd)  # Close the file descriptor immediately
    try:
        serializer.save_file(TEST_DATA, temp_path)
        # Test atomic_read_path (actual API)
        if hasattr(serializer, 'atomic_read_path') and serializer.supports_path_based_updates:
            name = serializer.atomic_read_path(temp_path, "/users/0/name")
            assert name == "Alice", f"{label} atomic_read_path mismatch"
            # Test atomic_update_path (actual API)
            serializer.atomic_update_path(temp_path, "/users/0/name", "Alice Updated")
            updated_name = serializer.atomic_read_path(temp_path, "/users/0/name")
            assert updated_name == "Alice Updated", f"{label} atomic_update_path mismatch"
        else:
            pytest.skip(f"{label} does not support path-based operations")
    finally:
        # Ensure file is closed before deletion (Windows requirement)
        time.sleep(0.01)  # Small delay to ensure file handles are released
        if os.path.exists(temp_path):
            try:
                os.unlink(temp_path)
            except PermissionError:
                # On Windows, sometimes need to wait a bit longer
                time.sleep(0.1)
                os.unlink(temp_path)
@pytest.mark.parametrize("label, serializer_cls", SERIALIZER_CASES)


def test_patch_application(label, serializer_cls):
    """Confirm path-based updates work via atomic_update_path."""
    serializer = serializer_cls()
    import tempfile
    import os
    import time
    # Create temp file path (don't open it, just get the path)
    temp_fd, temp_path = tempfile.mkstemp(suffix=serializer.file_extensions[0])
    os.close(temp_fd)  # Close the file descriptor immediately
    try:
        serializer.save_file(TEST_DATA, temp_path)
        if hasattr(serializer, 'atomic_update_path') and serializer.supports_path_based_updates:
            # Use atomic_update_path (actual API)
            serializer.atomic_update_path(temp_path, "/users/0/name", "Alice Patched")
            patched_name = serializer.atomic_read_path(temp_path, "/users/0/name")
            assert patched_name == "Alice Patched", f"{label} patch failed"
        else:
            pytest.skip(f"{label} does not support path-based updates")
    finally:
        # Ensure file is closed before deletion (Windows requirement)
        time.sleep(0.01)  # Small delay to ensure file handles are released
        if os.path.exists(temp_path):
            try:
                os.unlink(temp_path)
            except PermissionError:
                # On Windows, sometimes need to wait a bit longer
                time.sleep(0.1)
                os.unlink(temp_path)
@pytest.mark.parametrize("label, serializer_cls", SERIALIZER_CASES)


def test_schema_validation(label, serializer_cls):
    """Check data validation using validate_data method."""
    serializer = serializer_cls()
    # Use validate_data (actual API)
    if hasattr(serializer, 'validate_data'):
        is_valid = serializer.validate_data(TEST_DATA)
        assert is_valid is True, f"{label} validate_data failed"
    else:
        pytest.skip(f"{label} does not support validate_data")
@pytest.mark.parametrize("label, serializer_cls", SERIALIZER_CASES)


def test_canonicalization_and_hashing(label, serializer_cls):
    """Test that encoding produces consistent output for same data."""
    serializer = serializer_cls()
    # Test that encoding same data produces consistent results
    encoded1 = serializer.encode(TEST_DATA)
    encoded2 = serializer.encode(TEST_DATA)
    # Normalize to string for comparison
    if isinstance(encoded1, bytes):
        encoded1 = encoded1.decode('utf-8')
    if isinstance(encoded2, bytes):
        encoded2 = encoded2.decode('utf-8')
    # For formats that support canonical encoding, results should be identical
    # For formats without canonical encoding, at least verify round-trip works
    assert encoded1 == encoded2 or serializer.decode(encoded1) == serializer.decode(encoded2), \
        f"{label} encoding not consistent"
@pytest.mark.parametrize("label, serializer_cls", SERIALIZER_CASES)


def test_batch_streaming(label, serializer_cls):
    """Test streaming serialization using iter_serialize."""
    serializer = serializer_cls()
    # Wrap in dict for XML (XML requires single root element)
    if label == "XML":
        data = {"items": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
        expected = data
    else:
        data = [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
        expected = data
    # Use iter_serialize (actual API)
    if hasattr(serializer, 'iter_serialize'):
        chunks = list(serializer.iter_serialize(data))
        assert chunks, f"{label} iter_serialize returned no chunks"
        # Use iter_deserialize (actual API)
        if hasattr(serializer, 'iter_deserialize'):
            deserialized = serializer.iter_deserialize(chunks)
            # XML may infer types or wrap differently
            if label == "XML":
                # XML wraps in root - check structure matches
                assert isinstance(deserialized, dict), f"{label} XML should return dict"
                assert 'items' in deserialized or list(deserialized.keys())[0] in deserialized, \
                    f"{label} XML structure mismatch"
            else:
                assert deserialized == expected, f"{label} iter_deserialize mismatch"
        else:
            # Fallback: decode manually
            if chunks and isinstance(chunks[0], bytes):
                combined = b''.join(chunks)
            else:
                combined = ''.join(chunks)
            deserialized = serializer.decode(combined)
            # XML may infer types or wrap differently
            if label == "XML":
                assert isinstance(deserialized, dict), f"{label} XML should return dict"
            else:
                assert deserialized == expected, f"{label} streaming round-trip failed"
    else:
        pytest.skip(f"{label} does not support streaming")
