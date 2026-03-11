#!/usr/bin/env python3
"""
Unit tests for xwsystem io/indexing pluggable backends.
Tests IIndexBackend protocol, InMemoryIndexBackend, and FileBackedIndexBackend:
get, put, delete, list_keys, contains, and persistence.
Company: eXonware.com
"""

import pytest
from pathlib import Path
from exonware.xwsystem import get_serializer, JsonSerializer
from exonware.xwsystem.io.indexing.backend import (
    IIndexBackend,
    InMemoryIndexBackend,
    FileBackedIndexBackend,
    BTreeIndexBackend,
    FullTextIndexBackend,
    FileBackedFullTextIndexBackend,
)
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io


class TestInMemoryIndexBackend:
    """Test InMemoryIndexBackend."""
    @pytest.fixture

    def backend(self):
        return InMemoryIndexBackend()

    def test_put_get(self, backend):
        """put then get returns value."""
        backend.put("k1", {"id": 1})
        assert backend.get("k1") == {"id": 1}

    def test_get_missing_returns_none(self, backend):
        """get for missing key returns None."""
        assert backend.get("missing") is None

    def test_delete_existing(self, backend):
        """delete existing key returns True and removes."""
        backend.put("k1", "v1")
        assert backend.delete("k1") is True
        assert backend.get("k1") is None

    def test_delete_missing_returns_false(self, backend):
        """delete missing key returns False."""
        assert backend.delete("missing") is False

    def test_contains(self, backend):
        """contains returns True/False correctly."""
        backend.put("a", 1)
        assert backend.contains("a") is True
        assert backend.contains("b") is False

    def test_list_keys_empty_prefix(self, backend):
        """list_keys with empty prefix yields all keys."""
        backend.put("a", 1)
        backend.put("b", 2)
        assert set(backend.list_keys("")) == {"a", "b"}

    def test_list_keys_with_prefix(self, backend):
        """list_keys with prefix filters."""
        backend.put("user:1", 1)
        backend.put("user:2", 2)
        backend.put("order:1", 3)
        assert set(backend.list_keys("user:")) == {"user:1", "user:2"}
        assert list(backend.list_keys("order:")) == ["order:1"]

    def test_conforms_to_protocol(self, backend):
        """InMemoryIndexBackend conforms to IIndexBackend."""
        assert isinstance(backend, IIndexBackend)
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io


class TestFileBackedIndexBackend:
    """Test FileBackedIndexBackend and persistence."""
    @pytest.fixture

    def index_path(self, tmp_path):
        return tmp_path / "index.json"
    @pytest.fixture

    def backend(self, index_path):
        return FileBackedIndexBackend(index_path)

    def test_put_get_persists(self, backend, index_path):
        """put persists; get after new instance loads value."""
        backend.put("k1", "v1")
        back2 = FileBackedIndexBackend(index_path)
        assert back2.get("k1") == "v1"

    def test_delete_persists(self, backend, index_path):
        """delete persists across instances."""
        backend.put("k1", "v1")
        backend.delete("k1")
        back2 = FileBackedIndexBackend(index_path)
        assert back2.get("k1") is None
        assert back2.contains("k1") is False

    def test_list_keys_prefix(self, backend):
        """list_keys with prefix works on file backend."""
        backend.put("p:a", 1)
        backend.put("p:b", 2)
        backend.put("q:a", 3)
        assert set(backend.list_keys("p:")) == {"p:a", "p:b"}

    def test_conforms_to_protocol(self, backend):
        """FileBackedIndexBackend conforms to IIndexBackend."""
        assert isinstance(backend, IIndexBackend)

    def test_persisted_file_readable_by_xwsystem_json_serializer(self, backend, index_path):
        """Reuse: file written by backend is readable by xwsystem JsonSerializer (same stack)."""
        backend.put("k1", {"a": 1})
        backend.put("k2", [1, 2, 3])
        serializer = get_serializer(JsonSerializer)
        data = serializer.load_file(index_path)
        assert data == {"k1": {"a": 1}, "k2": [1, 2, 3]}
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io


class TestBTreeIndexBackend:
    """Test BTreeIndexBackend (ordered keys, range_scan)."""
    @pytest.fixture

    def backend(self):
        return BTreeIndexBackend()

    def test_put_get_delete(self, backend):
        """put, get, delete behave as IIndexBackend."""
        backend.put("k1", "v1")
        assert backend.get("k1") == "v1"
        assert backend.delete("k1") is True
        assert backend.get("k1") is None

    def test_list_keys_ordered(self, backend):
        """list_keys yields keys in sort order."""
        backend.put("z", 1)
        backend.put("a", 2)
        backend.put("m", 3)
        assert list(backend.list_keys("")) == ["a", "m", "z"]

    def test_range_scan(self, backend):
        """range_scan yields (key, value) in [start, end] in order."""
        backend.put("a", 1)
        backend.put("b", 2)
        backend.put("c", 3)
        backend.put("d", 4)
        backend.put("e", 5)
        assert list(backend.range_scan("b", "d")) == [("b", 2), ("c", 3), ("d", 4)]
        assert list(backend.range_scan("b", "b")) == [("b", 2)]
        assert list(backend.range_scan("x", "z")) == []

    def test_conforms_to_protocol(self, backend):
        """BTreeIndexBackend conforms to IIndexBackend."""
        assert isinstance(backend, IIndexBackend)
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io


class TestFullTextIndexBackend:
    """Test FullTextIndexBackend (term index, search AND/OR)."""
    @pytest.fixture

    def backend(self):
        return FullTextIndexBackend()

    def test_index_and_search_and(self, backend):
        """index then search with AND semantics (all terms)."""
        backend.index("d1", "hello world")
        backend.index("d2", "world peace")
        backend.index("d3", "hello world peace")
        assert backend.search("hello world") == ["d1", "d3"]
        assert backend.search("world peace") == ["d2", "d3"]
        assert backend.search("hello peace") == ["d3"]
        assert backend.search("hello") == ["d1", "d3"]
        assert backend.search("missing") == []

    def test_search_any(self, backend):
        """search_any returns docs containing any term (OR)."""
        backend.index("d1", "apple banana")
        backend.index("d2", "banana cherry")
        backend.index("d3", "cherry date")
        assert set(backend.search_any("apple cherry")) == {"d1", "d2", "d3"}
        assert backend.search_any("date") == ["d3"]

    def test_remove_document(self, backend):
        """remove_document removes doc from index."""
        backend.index("d1", "foo bar")
        backend.index("d2", "bar baz")
        assert backend.search("bar") == ["d1", "d2"]
        backend.remove_document("d1")
        assert backend.search("bar") == ["d2"]
        assert backend.search("foo") == []
        assert backend.contains_document("d1") is False
        assert backend.contains_document("d2") is True

    def test_reindex_overwrites(self, backend):
        """index same doc_id again overwrites (re-index)."""
        backend.index("d1", "old text")
        backend.index("d1", "new text")
        assert backend.search("old") == []
        assert backend.search("new") == ["d1"]

    def test_empty_query_returns_empty(self, backend):
        """search with empty or whitespace-only query returns []."""
        backend.index("d1", "something")
        assert backend.search("") == []
        assert backend.search("   ") == []

    def test_contains_document_and_document_count(self, backend):
        """contains_document and document_count."""
        assert backend.document_count() == 0
        backend.index("d1", "a")
        backend.index("d2", "b")
        assert backend.contains_document("d1") is True
        assert backend.contains_document("d2") is True
        assert backend.contains_document("d3") is False
        assert backend.document_count() == 2
        backend.remove_document("d1")
        assert backend.document_count() == 1

    def test_tokenization_case_and_punctuation(self, backend):
        """Tokenization is lowercase; non-alphanumeric splits terms."""
        backend.index("d1", "Hello, World! How are you?")
        assert backend.search("hello") == ["d1"]
        assert backend.search("world") == ["d1"]
        assert backend.search("hello world") == ["d1"]
        assert backend.search("how are you") == ["d1"]

    def test_tokenization_unicode_and_numbers(self, backend):
        """Terms can contain unicode letters and digits; split on non-alphanumeric."""
        backend.index("d1", "café 123 résumé")
        assert backend.search("café") == ["d1"]
        assert backend.search("123") == ["d1"]
        assert backend.search("résumé") == ["d1"]
        assert backend.search("café 123") == ["d1"]

    def test_numbers_only_terms(self, backend):
        """Numeric-only terms are indexed."""
        backend.index("d1", "version 2.0 release 2024")
        assert backend.search("2") == ["d1"]
        assert backend.search("0") == ["d1"]
        assert backend.search("2024") == ["d1"]
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io


class TestFileBackedFullTextIndexBackend:
    """Test FileBackedFullTextIndexBackend persistence."""
    @pytest.fixture

    def index_path(self, tmp_path):
        return tmp_path / "fulltext_index.json"

    def test_persistence_roundtrip(self, index_path):
        """Index, save; new instance loads and search returns same results."""
        b1 = FileBackedFullTextIndexBackend(index_path)
        b1.index("d1", "hello world")
        b1.index("d2", "world peace")
        assert b1.search("world") == ["d1", "d2"]
        b2 = FileBackedFullTextIndexBackend(index_path)
        assert b2.search("world") == ["d1", "d2"]
        assert b2.search("hello") == ["d1"]
        assert b2.document_count() == 2

    def test_remove_persists(self, index_path):
        """remove_document persists; new instance sees update."""
        b1 = FileBackedFullTextIndexBackend(index_path)
        b1.index("d1", "foo")
        b1.index("d2", "bar")
        b1.remove_document("d1")
        b2 = FileBackedFullTextIndexBackend(index_path)
        assert b2.search("foo") == []
        assert b2.search("bar") == ["d2"]
        assert b2.document_count() == 1

    def test_load_missing_file(self, index_path):
        """Missing path yields empty index (no error)."""
        assert index_path.exists() is False
        b = FileBackedFullTextIndexBackend(index_path)
        assert b.document_count() == 0
        b.index("d1", "test")
        assert index_path.exists() is True
        assert b.search("test") == ["d1"]

    def test_persisted_file_readable_by_xwsystem_json_serializer(self, index_path):
        """Reuse: file written by backend is readable by xwsystem JsonSerializer (same stack)."""
        b = FileBackedFullTextIndexBackend(index_path)
        b.index("d1", "alpha beta")
        b.index("d2", "beta gamma")
        serializer = get_serializer(JsonSerializer)
        data = serializer.load_file(index_path)
        assert "term_to_docs" in data and "doc_to_terms" in data
        assert "alpha" in data["term_to_docs"] and "d1" in data["term_to_docs"]["alpha"]
        assert "beta" in data["term_to_docs"] and set(data["term_to_docs"]["beta"]) == {"d1", "d2"}
