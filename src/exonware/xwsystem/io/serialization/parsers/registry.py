#exonware/xwsystem/src/exonware/xwsystem/io/serialization/parsers/registry.py
"""Parser registry for JSON parser selection and auto-detection."""

from typing import Optional, Type
from .base import AJsonParser
from .standard import StandardJsonParser
from .hybrid_parser import HybridParser
from .hyperjson_parser import HyperjsonParser
# Registry of available parsers (order of preference when auto-detecting)
_PARSERS: dict[str, Type[AJsonParser]] = {
    "hyperjson": HyperjsonParser,  # Preferred when installed (fastest in benchmarks)
    "hybrid": HybridParser,  # msgspec read + orjson write
    "standard": StandardJsonParser,  # Fallback only
}
# Cache of parser instances
_PARSER_INSTANCES: dict[str, AJsonParser] = {}


def get_best_available_parser() -> AJsonParser:
    """
    Auto-detect and return the best available parser.
    Preference: hyperjson (when installed) > hybrid (msgspec + orjson) > standard (stdlib).
    hyperjson is fastest in benchmarks; supports atomic operations via JsonSerializer
    (atomic_update_path, atomic_read_path, load_file, save_file) since they all use
    the same parser for encode/decode.
    Returns:
        Best available parser instance
    """
    # Try hyperjson first (fastest when installed)
    parser_class = _PARSERS.get("hyperjson")
    if parser_class:
        parser = parser_class()
        if parser.is_available:
            return parser
    # Then hybrid (msgspec read + orjson write)
    parser_class = _PARSERS.get("hybrid")
    if parser_class:
        parser = parser_class()
        if parser.is_available:
            return parser
    # Fallback to standard (stdlib)
    return StandardJsonParser()


def get_parser(name: Optional[str] = None) -> AJsonParser:
    """
    Get parser by name or auto-detect best available.
    Args:
        name: Parser name ("standard", "orjson", or None for auto-detect)
    Returns:
        Parser instance (falls back to best available if requested parser unavailable)
    """
    if name is None:
        return get_best_available_parser()
    # Check cache first
    if name in _PARSER_INSTANCES:
        parser = _PARSER_INSTANCES[name]
        if parser.is_available:
            return parser
    # Create new instance
    parser_class = _PARSERS.get(name, StandardJsonParser)
    parser = parser_class()
    # Cache if available
    if parser.is_available:
        _PARSER_INSTANCES[name] = parser
    else:
        # Fallback to available parser if requested parser unavailable
        if name != "standard":
            return get_best_available_parser()
    return parser


def register_parser(name: str, parser_class: Type[AJsonParser]):
    """
    Register a new parser implementation.
    Args:
        name: Parser identifier
        parser_class: Parser class implementing AJsonParser
    """
    _PARSERS[name] = parser_class
    # Clear cache to allow new parser to be used
    if name in _PARSER_INSTANCES:
        del _PARSER_INSTANCES[name]
