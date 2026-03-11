#!/usr/bin/env python3
"""
Unit tests for SerializableCache save_to_file / load_from_file (pickle and JSON).
JSON format uses xwsystem get_serializer(JsonSerializer); tests verify roundtrip
and that JSON files are readable by the same serializer (reuse).
"""

from __future__ import annotations
import pytest
from pathlib import Path
from exonware.xwsystem import get_serializer, JsonSerializer
from exonware.xwsystem.caching.serializable import SerializableCache
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching


class TestSerializableCachePickle:
    """Tests for SerializableCache with format='pickle'."""

    def test_save_and_load_pickle_roundtrip(self, tmp_path: Path) -> None:
        cache = SerializableCache(capacity=10, name="test")
        cache.put("a", 1)
        cache.put("b", "two")
        path = tmp_path / "cache.pkl"
        assert cache.save_to_file(path, format="pickle") is True
        loaded = SerializableCache.load_from_file(path, format="pickle")
        assert loaded.get("a") == 1
        assert loaded.get("b") == "two"
        assert loaded.capacity == cache.capacity

    def test_load_pickle_invalid_path_raises(self) -> None:
        with pytest.raises(Exception):
            SerializableCache.load_from_file(Path("/nonexistent/cache.pkl"), format="pickle")
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching


class TestSerializableCacheJson:
    """Tests for SerializableCache with format='json' (xwsystem JsonSerializer)."""

    def test_save_and_load_json_roundtrip(self, tmp_path: Path) -> None:
        cache = SerializableCache(capacity=10, name="json_test")
        cache.put("k1", "v1")
        cache.put("k2", 42)
        path = tmp_path / "cache.json"
        assert cache.save_to_file(path, format="json") is True
        loaded = SerializableCache.load_from_file(path, format="json")
        assert loaded.get("k1") == "v1"
        assert loaded.get("k2") == 42

    def test_json_file_readable_by_xwsystem_serializer(self, tmp_path: Path) -> None:
        """JSON files written by SerializableCache must be readable via get_serializer(JsonSerializer).load_file."""
        cache = SerializableCache(capacity=5, name="reuse_test")
        cache.put("x", "y")
        path = tmp_path / "cache.json"
        assert cache.save_to_file(path, format="json") is True
        reader = get_serializer(JsonSerializer)
        data = reader.load_file(path)
        assert isinstance(data, dict)
        assert data.get("items", {}).get("x") == "y"
        assert data.get("capacity") == 5
        assert data.get("name") == "reuse_test"

    def test_save_unsupported_format_returns_false(self, tmp_path: Path) -> None:
        cache = SerializableCache(capacity=5)
        path = tmp_path / "out.xyz"
        assert cache.save_to_file(path, format="xml") is False
        assert not path.exists()
