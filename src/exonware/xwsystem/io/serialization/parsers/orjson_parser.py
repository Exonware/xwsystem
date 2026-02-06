#exonware/xwsystem/src/exonware/xwsystem/io/serialization/parsers/orjson_parser.py
"""orjson parser - Tier 1 (3-4x faster than stdlib)."""

from typing import Any
import importlib.util

_orjson_spec = importlib.util.find_spec('orjson')
if _orjson_spec is not None:
    import orjson
    ORJSON_AVAILABLE = True
else:
    ORJSON_AVAILABLE = False
    orjson = None

from .base import AJsonParser


class OrjsonParser(AJsonParser):
    """orjson parser - Tier 1 (3-4x faster than stdlib)."""
    
    @property
    def parser_name(self) -> str:
        return "orjson"
    
    @property
    def tier(self) -> int:
        return 1
    
    @property
    def is_available(self) -> bool:
        return ORJSON_AVAILABLE
    
    def loads(self, s: str | bytes) -> Any:
        """Parse JSON using orjson.loads()."""
        if isinstance(s, str):
            s = s.encode("utf-8")
        return orjson.loads(s)
    
    def dumps(self, obj: Any, **kwargs) -> str | bytes:
        """Serialize JSON using orjson.dumps()."""
        option = 0
        
        # orjson options
        if not kwargs.get("ensure_ascii", True):
            # orjson outputs UTF-8, so ensure_ascii=False is default
            pass
        
        # Handle indent (orjson doesn't support indent directly)
        indent = kwargs.get("indent", None)
        if indent:
            # For pretty printing, use orjson.OPT_INDENT_2
            option |= orjson.OPT_INDENT_2
        
        # Sort keys
        if kwargs.get("sort_keys", False):
            option |= orjson.OPT_SORT_KEYS
        
        result = orjson.dumps(obj, option=option)
        
        # Return as bytes (orjson returns bytes)
        # Caller can decode if string is needed
        return result
