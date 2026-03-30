#exonware/xwsystem/src/exonware/xwsystem/io/serialization/parsers/hyperjson_parser.py
"""hyperlight-hyperjson parser - Tier 1 (fastest in benchmarks)."""

from typing import Any
from .base import AJsonParser
import hyperjson  # type: ignore[reportMissingImports]


class HyperjsonParser(AJsonParser):
    """
    hyperlight-hyperjson parser.
    Fastest in benchmarks (~2.05M decode/s, ~2.26M encode/s small payload).
    Supports same encode/decode as other parsers; atomic operations in JsonSerializer
    (atomic_update_path, atomic_read_path, load_file, save_file) use this parser
    when selected.
    """
    @property

    def parser_name(self) -> str:
        return "hyperjson"
    @property

    def tier(self) -> int:
        return 1
    @property

    def is_available(self) -> bool:
        return True

    def loads(self, s: str | bytes) -> Any:
        """Parse JSON using hyperlight_hyperjson.loads()."""
        if isinstance(s, str):
            s = s.encode("utf-8")
        return hyperjson.loads(s)

    def dumps(self, obj: Any, **kwargs) -> str | bytes:
        """Serialize JSON using hyperlight_hyperjson.dumps()."""
        # hyperjson/hyperlight-hyperjson are orjson forks; support option= when present
        option = 0
        if getattr(hyperjson, "OPT_INDENT_2", None) is not None and kwargs.get("indent"):
            option |= getattr(hyperjson, "OPT_INDENT_2", 0)
        if getattr(hyperjson, "OPT_SORT_KEYS", None) is not None and kwargs.get("sort_keys"):
            option |= getattr(hyperjson, "OPT_SORT_KEYS", 0)
        try:
            result = hyperjson.dumps(obj, option=option)
        except TypeError:
            result = hyperjson.dumps(obj)
        return result
