#exonware/xwsystem/src/exonware/xwsystem/io/serialization/parsers/hyperjson_parser.py
"""hyperjson / hyperlight-hyperjson parser - Tier 1 (fastest in benchmarks)."""

from typing import Any
import importlib.util
# Prefer hyperjson, fallback to hyperlight_hyperjson (maintained fork)
_hyperjson = None
HYPERJSON_AVAILABLE = False
if importlib.util.find_spec("hyperjson") is not None:
    try:
        import hyperjson
        _hyperjson = hyperjson
        HYPERJSON_AVAILABLE = True
    except Exception:
        pass
if not HYPERJSON_AVAILABLE and importlib.util.find_spec("hyperlight_hyperjson") is not None:
    try:
        import hyperlight_hyperjson as hyperjson
        _hyperjson = hyperjson
        HYPERJSON_AVAILABLE = True
    except Exception:
        pass
from .base import AJsonParser


class HyperjsonParser(AJsonParser):
    """
    hyperjson / hyperlight-hyperjson parser.
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
        return HYPERJSON_AVAILABLE

    def loads(self, s: str | bytes) -> Any:
        """Parse JSON using hyperjson.loads()."""
        if not _hyperjson:
            raise RuntimeError("hyperjson not available")
        if isinstance(s, str):
            s = s.encode("utf-8")
        return _hyperjson.loads(s)

    def dumps(self, obj: Any, **kwargs) -> str | bytes:
        """Serialize JSON using hyperjson.dumps()."""
        if not _hyperjson:
            raise RuntimeError("hyperjson not available")
        # hyperjson/hyperlight-hyperjson are orjson forks; support option= when present
        option = 0
        if getattr(_hyperjson, "OPT_INDENT_2", None) is not None and kwargs.get("indent"):
            option |= getattr(_hyperjson, "OPT_INDENT_2", 0)
        if getattr(_hyperjson, "OPT_SORT_KEYS", None) is not None and kwargs.get("sort_keys"):
            option |= getattr(_hyperjson, "OPT_SORT_KEYS", 0)
        try:
            result = _hyperjson.dumps(obj, option=option)
        except TypeError:
            result = _hyperjson.dumps(obj)
        return result
