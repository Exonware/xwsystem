#exonware/xwsystem/src/exonware/xwsystem/io/indexing/facade.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.32
Generation Date: January 2026
XWIndex - Unified Indexing Facade
Simplified API for indexing line-oriented files (JSONL/NDJSON):
- Build indexes with line offsets
- ID-based indexing
- Paging support
- Streaming operations
"""

from pathlib import Path
from typing import Any

from collections.abc import Callable, Iterator
from ..data_operations import (
    ADataOperations,
    JsonIndex,
    JsonIndexMeta,
    JsonMatchFn,
    JsonUpdateFn,
)
from ..serialization.parsers.registry import get_best_available_parser
from ...config.logging_setup import get_logger
logger = get_logger(__name__)


class JsonlDataOperations(ADataOperations):
    """Concrete implementation for JSONL/NDJSON files."""

    def stream_read(
        self,
        file_path: str | Path,
        match: JsonMatchFn,
        path: list[object] | None = None,
        encoding: str = "utf-8",
    ) -> Any:
        """Return the first record that matches the predicate."""
        parser = get_best_available_parser()
        with open(file_path, encoding=encoding) as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    obj = parser.loads(line)
                    if match(obj):
                        return obj
                except Exception:
                    continue
        return None

    def stream_update(
        self,
        file_path: str | Path,
        match: JsonMatchFn,
        updater: JsonUpdateFn,
        *,
        encoding: str = "utf-8",
        newline: str = "\n",
        atomic: bool = True,
    ) -> int:
        """Stream-copy the backing store, applying updater to matching records."""
        import tempfile
        import shutil
        parser = get_best_available_parser()
        updated_count = 0
        file_path = Path(file_path)
        temp_path = file_path.with_suffix(file_path.suffix + ".tmp")
        try:
            with open(file_path, encoding=encoding) as infile, \
                 open(temp_path, "w", encoding=encoding) as outfile:
                for line in infile:
                    original_line = line
                    line = line.strip()
                    if not line:
                        outfile.write(original_line)
                        continue
                    try:
                        obj = parser.loads(line)
                        if match(obj):
                            updated_obj = updater(obj)
                            updated_line = parser.dumps(updated_obj) + newline
                            outfile.write(updated_line)
                            updated_count += 1
                        else:
                            outfile.write(original_line)
                    except Exception:
                        outfile.write(original_line)
            if atomic:
                shutil.move(str(temp_path), str(file_path))
            else:
                shutil.copy2(str(temp_path), str(file_path))
                temp_path.unlink()
        except Exception as e:
            if temp_path.exists():
                temp_path.unlink()
            raise
        return updated_count

    def build_index(
        self,
        file_path: str | Path,
        *,
        encoding: str = "utf-8",
        id_field: str | None = None,
        max_id_index: int | None = None,
    ) -> JsonIndex:
        """Build an index structure suitable for random access and paging."""
        import os
        import time
        parser = get_best_available_parser()
        file_path = Path(file_path)
        line_offsets: list[int] = []
        id_index: dict[str, int] | None = {} if id_field else None
        stat = file_path.stat()
        size = stat.st_size
        mtime = stat.st_mtime
        with open(file_path, "rb") as f:
            current_offset = 0
            while True:
                line_start = current_offset
                line = f.readline()
                if not line:
                    break
                current_offset = f.tell()
                # Skip empty lines
                if not line.strip():
                    continue
                line_offsets.append(line_start)
                line_idx = len(line_offsets) - 1
                # Build ID index if requested
                if id_field and id_index is not None:
                    if max_id_index is None or len(id_index) < max_id_index:
                        try:
                            obj = parser.loads(line)
                            if isinstance(obj, dict) and id_field in obj:
                                id_val = str(obj[id_field])
                                id_index[id_val] = line_idx
                        except Exception:
                            pass  # skip invalid lines
        meta = JsonIndexMeta(
            path=str(file_path),
            size=size,
            mtime=mtime,
            version=1
        )
        return JsonIndex(
            meta=meta,
            line_offsets=line_offsets,
            id_index=id_index if id_index else None
        )

    def indexed_get_by_line(
        self,
        file_path: str | Path,
        line_number: int,
        *,
        encoding: str = "utf-8",
        index: JsonIndex | None = None,
    ) -> Any:
        """Random-access a specific record by line number."""
        parser = get_best_available_parser()
        if index is None:
            index = self.build_index(file_path, encoding=encoding)
        if line_number < 0 or line_number >= len(index.line_offsets):
            return None
        offset = index.line_offsets[line_number]
        with open(file_path, "rb") as f:
            f.seek(offset)
            line = f.readline().decode(encoding).strip()
            if not line:
                return None
            try:
                return parser.loads(line)
            except Exception:
                return None

    def indexed_get_by_id(
        self,
        file_path: str | Path,
        id_value: Any,
        *,
        encoding: str = "utf-8",
        id_field: str = "id",
        index: JsonIndex | None = None,
    ) -> Any:
        """Random-access a record by logical ID."""
        if index is None:
            index = self.build_index(file_path, encoding=encoding, id_field=id_field)
        if index.id_index is None:
            # Fallback to linear scan
            return self.stream_read(
                file_path,
                match=lambda obj: isinstance(obj, dict) and obj.get(id_field) == id_value,
                encoding=encoding
            )
        id_str = str(id_value)
        if id_str not in index.id_index:
            return None
        line_number = index.id_index[id_str]
        return self.indexed_get_by_line(file_path, line_number, encoding=encoding, index=index)

    def get_page(
        self,
        file_path: str | Path,
        page_number: int,
        page_size: int,
        *,
        encoding: str = "utf-8",
        index: JsonIndex | None = None,
    ) -> list[Any]:
        """Return a page of records using index."""
        if index is None:
            index = self.build_index(file_path, encoding=encoding)
        start_idx = page_number * page_size
        end_idx = start_idx + page_size
        results = []
        for line_number in range(start_idx, min(end_idx, len(index.line_offsets))):
            record = self.indexed_get_by_line(file_path, line_number, encoding=encoding, index=index)
            if record is not None:
                results.append(record)
        return results


class XWIndex:
    """
    Unified indexing facade - simple API for indexing line-oriented files.
    Examples:
        >>> # Simple indexing
        >>> index = XWIndex("data.jsonl")  # Auto-detects JSONL
        >>> index = XWIndex("data.ndjson", format="jsonl")
        >>> # Build index
        >>> index.build()  # Builds line offsets + optional ID index
        >>> # Query
        >>> record = index.get_by_line(42)
        >>> record = index.get_by_id("user_123")
        >>> # Paging
        >>> page = index.get_page(page=1, size=10)
        >>> # Streaming
        >>> for record in index.stream(match=lambda r: r["active"]):
        ...     print(record)
    """

    def __init__(
        self,
        file_path: str | Path,
        *,
        format: str | None = None,
        encoding: str = "utf-8",
        id_field: str | None = None,
    ):
        """
        Initialize unified index.
        Args:
            file_path: Path to line-oriented file (JSONL/NDJSON)
            format: File format ("jsonl", "ndjson") - auto-detected if None
            encoding: File encoding (default: "utf-8")
            id_field: Optional field name for ID-based indexing
        """
        self.file_path = Path(file_path)
        self.format = format or self._detect_format()
        self.encoding = encoding
        self.id_field = id_field
        self._operations = JsonlDataOperations()
        self._index: JsonIndex | None = None

    def _detect_format(self) -> str:
        """Auto-detect format from file extension."""
        ext = self.file_path.suffix.lower()
        if ext in (".jsonl", ".ndjson", ".jsonlines"):
            return "jsonl"
        return "jsonl"  # Default to JSONL

    def build(
        self,
        *,
        id_field: str | None = None,
        max_id_index: int | None = None,
        force: bool = False,
    ) -> JsonIndex:
        """
        Build index structure.
        Args:
            id_field: Field name for ID-based indexing (uses instance id_field if None)
            max_id_index: Maximum number of IDs to index
            force: Force rebuild even if index exists
        Returns:
            JsonIndex object
        """
        if self._index is None or force:
            self._index = self._operations.build_index(
                self.file_path,
                encoding=self.encoding,
                id_field=id_field or self.id_field,
                max_id_index=max_id_index,
            )
        return self._index

    def get_by_line(self, line_number: int) -> Any:
        """Get record by line number (0-based)."""
        if self._index is None:
            self.build()
        return self._operations.indexed_get_by_line(
            self.file_path,
            line_number,
            encoding=self.encoding,
            index=self._index,
        )

    def get_by_id(self, id_value: Any, id_field: str | None = None) -> Any:
        """Get record by ID."""
        if self._index is None:
            self.build(id_field=id_field or self.id_field)
        return self._operations.indexed_get_by_id(
            self.file_path,
            id_value,
            encoding=self.encoding,
            id_field=id_field or self.id_field or "id",
            index=self._index,
        )

    def get_page(self, page: int = 0, size: int = 10) -> list[Any]:
        """Get a page of records."""
        if self._index is None:
            self.build()
        return self._operations.get_page(
            self.file_path,
            page,
            size,
            encoding=self.encoding,
            index=self._index,
        )

    def stream(self, match: Callable[[Any], bool] | None = None) -> Iterator[Any]:
        """
        Stream records matching predicate.
        Args:
            match: Optional predicate function (returns all records if None)
        """
        parser = get_best_available_parser()
        with open(self.file_path, encoding=self.encoding) as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    obj = parser.loads(line)
                    if match is None or match(obj):
                        yield obj
                except Exception:
                    continue

    def update(
        self,
        match: Callable[[Any], bool],
        updater: Callable[[Any], Any],
        *,
        atomic: bool = True,
    ) -> int:
        """
        Update records matching predicate.
        Args:
            match: Predicate function to match records
            updater: Function to update matched records
            atomic: Use atomic file replacement
        Returns:
            Number of updated records
        """
        return self._operations.stream_update(
            self.file_path,
            match,
            updater,
            encoding=self.encoding,
            atomic=atomic,
        )
    @property

    def index(self) -> JsonIndex | None:
        """Get current index (builds if needed)."""
        if self._index is None:
            self.build()
        return self._index
