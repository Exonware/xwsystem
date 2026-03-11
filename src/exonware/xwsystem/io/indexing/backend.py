#exonware/xwsystem/src/exonware/xwsystem/io/indexing/backend.py
"""
Pluggable index backends for general-purpose physical indexes.
Provides IIndexBackend protocol and implementations (in-memory, file-backed)
for B-tree–like key access. Used by file-based stores (e.g. XWJSON) alongside
the line-oriented XWIndex facade. Enables "indexing → 5" for the stack.
File-backed backends use xwsystem JsonSerializer (flyweight) for persistence
to reuse the same serialization stack and parser (e.g. orjson) for performance.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
"""

from __future__ import annotations
import re
from pathlib import Path
from typing import Any, Iterator, Optional, Protocol, runtime_checkable
from exonware.xwsystem import get_logger, get_serializer, JsonSerializer
logger = get_logger(__name__)
# Reuse xwsystem JsonSerializer (flyweight) for file persistence and performance
_json = get_serializer(JsonSerializer)
@runtime_checkable


class IIndexBackend(Protocol):
    """Pluggable index backend for key-based get/put/delete and range scan."""

    def get(self, key: str) -> Optional[Any]:
        """Return value for key or None."""
        ...

    def put(self, key: str, value: Any) -> None:
        """Store key -> value."""
        ...

    def delete(self, key: str) -> bool:
        """Remove key; return True if existed."""
        ...

    def list_keys(self, prefix: str = "") -> Iterator[str]:
        """Yield keys optionally filtered by prefix."""
        ...

    def contains(self, key: str) -> bool:
        """Return True if key exists."""
        ...


class InMemoryIndexBackend:
    """
    In-memory general-purpose index backend (dict-based).
    Suitable for small indexes and tests; can be backed by file persist in wrapper.
    """

    def __init__(self) -> None:
        self._store: dict[str, Any] = {}

    def get(self, key: str) -> Optional[Any]:
        return self._store.get(key)

    def put(self, key: str, value: Any) -> None:
        self._store[key] = value

    def delete(self, key: str) -> bool:
        if key in self._store:
            del self._store[key]
            return True
        return False

    def list_keys(self, prefix: str = "") -> Iterator[str]:
        for k in self._store:
            if k.startswith(prefix):
                yield k

    def contains(self, key: str) -> bool:
        return key in self._store


class FileBackedIndexBackend:
    """
    File-backed index backend (JSON). General-purpose physical index for
    file-based stores; persists key/value to a single JSON file.
    """

    def __init__(self, path: str | Path) -> None:
        self._path = Path(path)
        self._store: dict[str, Any] = {}
        self._load()

    def _load(self) -> None:
        if self._path.exists():
            try:
                self._store = _json.load_file(self._path)
                if not isinstance(self._store, dict):
                    self._store = {}
            except Exception as e:
                logger.warning("Index backend load failed %s: %s", self._path, e)
                self._store = {}

    def _save(self) -> None:
        self._path.parent.mkdir(parents=True, exist_ok=True)
        _json.save_file(self._store, self._path)

    def get(self, key: str) -> Optional[Any]:
        return self._store.get(key)

    def put(self, key: str, value: Any) -> None:
        self._store[key] = value
        self._save()

    def delete(self, key: str) -> bool:
        if key in self._store:
            del self._store[key]
            self._save()
            return True
        return False

    def list_keys(self, prefix: str = "") -> Iterator[str]:
        for k in self._store:
            if k.startswith(prefix):
                yield k

    def contains(self, key: str) -> bool:
        return key in self._store


class BTreeIndexBackend:
    """
    In-memory ordered index backend (B-tree–like): keys are stored in sort order
    so list_keys yields in order and range_scan(start, end) supports range queries.
    Used when ordered iteration or range scans are required (e.g. for scoring
    indexing capability vs PostgreSQL B-tree/GIN). Implements IIndexBackend.
    """

    def __init__(self) -> None:
        self._store: dict[str, Any] = {}
        self._sorted_keys: list[str] = []

    def get(self, key: str) -> Optional[Any]:
        return self._store.get(key)

    def put(self, key: str, value: Any) -> None:
        self._store[key] = value
        if key not in self._sorted_keys:
            self._sorted_keys.append(key)
            self._sorted_keys.sort()

    def delete(self, key: str) -> bool:
        if key in self._store:
            del self._store[key]
            self._sorted_keys = [k for k in self._sorted_keys if k != key]
            return True
        return False

    def list_keys(self, prefix: str = "") -> Iterator[str]:
        for k in self._sorted_keys:
            if k.startswith(prefix):
                yield k

    def contains(self, key: str) -> bool:
        return key in self._store

    def range_scan(
        self, start_key: str, end_key: str
    ) -> Iterator[tuple[str, Any]]:
        """Yield (key, value) for keys in [start_key, end_key] in order."""
        for k in self._sorted_keys:
            if start_key <= k <= end_key:
                v = self._store.get(k)
                if v is not None:
                    yield (k, v)
            elif k > end_key:
                break


def _tokenize(text: str) -> list[str]:
    """Split text into lowercase alphanumeric terms; skip empty."""
    return [t for t in re.split(r"[^a-zA-Z0-9]+", text.lower()) if t]


class FullTextIndexBackend:
    """
    In-memory full-text term index: index documents by text, search by query terms.
    Uses term -> set of doc_ids; search returns doc_ids containing all query terms (AND).
    Implements no ranking; for production ranking use a dedicated search engine.
    """

    def __init__(self) -> None:
        self._term_to_docs: dict[str, set[str]] = {}
        self._doc_to_terms: dict[str, set[str]] = {}

    def index(self, doc_id: str, text: str) -> None:
        """Index a document: tokenize text and add doc_id to each term's set."""
        terms = set(_tokenize(text))
        self.remove_document(doc_id)
        for term in terms:
            self._term_to_docs.setdefault(term, set()).add(doc_id)
        self._doc_to_terms[doc_id] = terms

    def search(self, query: str) -> list[str]:
        """Return doc_ids that contain all query terms (AND). Empty query returns no docs."""
        terms = _tokenize(query)
        if not terms:
            return []
        sets = [self._term_to_docs.get(t, set()) for t in terms]
        if not sets:
            return []
        result = sets[0].copy()
        for s in sets[1:]:
            result &= s
        return sorted(result)

    def search_any(self, query: str) -> list[str]:
        """Return doc_ids that contain any query term (OR)."""
        terms = _tokenize(query)
        if not terms:
            return []
        result: set[str] = set()
        for t in terms:
            result |= self._term_to_docs.get(t, set())
        return sorted(result)

    def remove_document(self, doc_id: str) -> None:
        """Remove document from the index."""
        if doc_id not in self._doc_to_terms:
            return
        for term in self._doc_to_terms[doc_id]:
            self._term_to_docs[term].discard(doc_id)
            if not self._term_to_docs[term]:
                del self._term_to_docs[term]
        del self._doc_to_terms[doc_id]

    def contains_document(self, doc_id: str) -> bool:
        """Return True if doc_id is in the index."""
        return doc_id in self._doc_to_terms

    def document_count(self) -> int:
        """Return number of indexed documents."""
        return len(self._doc_to_terms)


class FileBackedFullTextIndexBackend(FullTextIndexBackend):
    """
    Full-text term index persisted to a JSON file. Same API as FullTextIndexBackend;
    load on init, save on index() and remove_document(). Production use when
    the index must survive process restarts.
    """

    def __init__(self, path: str | Path) -> None:
        super().__init__()
        self._path = Path(path)
        self._load()

    def _load(self) -> None:
        if self._path.exists():
            try:
                data = _json.load_file(self._path)
                if not isinstance(data, dict):
                    data = {}
                self._term_to_docs = {
                    k: set(v) for k, v in data.get("term_to_docs", {}).items()
                }
                self._doc_to_terms = {
                    k: set(v) for k, v in data.get("doc_to_terms", {}).items()
                }
            except Exception as e:
                logger.warning("FullText index load failed %s: %s", self._path, e)
                self._term_to_docs = {}
                self._doc_to_terms = {}

    def _save(self) -> None:
        self._path.parent.mkdir(parents=True, exist_ok=True)
        data = {
            "term_to_docs": {k: list(v) for k, v in self._term_to_docs.items()},
            "doc_to_terms": {k: list(v) for k, v in self._doc_to_terms.items()},
        }
        _json.save_file(data, self._path)

    def index(self, doc_id: str, text: str) -> None:
        super().index(doc_id, text)
        self._save()

    def remove_document(self, doc_id: str) -> None:
        super().remove_document(doc_id)
        self._save()
