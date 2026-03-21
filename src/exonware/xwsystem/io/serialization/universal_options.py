#exonware/xwsystem/src/exonware/xwsystem/io/serialization/universal_options.py
"""
Universal Serialization Options
This module provides universal option mapping across different serialization formats.
Options like pretty, compact, sorted, canonical can be mapped to format-specific options.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.17
"""

from typing import Any
from enum import Enum


class UniversalOption(Enum):
    """Universal serialization options."""
    PRETTY = "pretty"
    COMPACT = "compact"
    SORTED = "sorted"
    CANONICAL = "canonical"
    INDENT = "indent"
    ENSURE_ASCII = "ensure_ascii"
    ALLOW_NAN = "allow_nan"
    STRIP_WHITESPACE = "strip_whitespace"
    PRESERVE_QUOTES = "preserve_quotes"
    LINE_SEPARATOR = "line_separator"
    ITEM_SEPARATOR = "item_separator"
# Format-specific option mappings
_FORMAT_OPTION_MAPS: dict[str, dict[str, str]] = {
    "json": {
        "pretty": "indent",
        "compact": "separators",
        "sorted": "sort_keys",
        "canonical": "sort_keys",
        "indent": "indent",
        "ensure_ascii": "ensure_ascii",
        "allow_nan": "allow_nan",
    },
    "yaml": {
        "pretty": "default_flow_style",
        "compact": "default_flow_style",
        "sorted": "sort_keys",
        "canonical": "canonical",
        "indent": "indent",
        "strip_whitespace": "strip_whitespace",
    },
    "xml": {
        "pretty": "pretty_print",
        "compact": "pretty_print",
        "sorted": "sort_attributes",
        "canonical": "canonical",
        "indent": "indent",
        "preserve_quotes": "preserve_quotes",
        "declaration": "full_document",  # xmltodict uses full_document to control XML declaration
        "encoding": "encoding",
    },
    "toml": {
        "pretty": "pretty",
        "compact": "indent",
        "sorted": "sort_keys",
        "indent": "indent",
    },
    "csv": {
        "pretty": "lineterminator",
        "compact": "lineterminator",
        "sorted": "sort_columns",
        "line_separator": "lineterminator",
        "item_separator": "delimiter",
    },
}
# Supported options per format
_FORMAT_SUPPORTED_OPTIONS: dict[str, set[str]] = {
    "json": {
        "pretty", "compact", "sorted", "canonical", "indent",
        "ensure_ascii", "allow_nan"
    },
    "yaml": {
        "pretty", "compact", "sorted", "canonical", "indent",
        "strip_whitespace"
    },
    "xml": {
        "pretty", "compact", "sorted", "canonical", "indent",
        "preserve_quotes", "declaration", "encoding"
    },
    "toml": {
        "pretty", "compact", "sorted", "indent"
    },
    "csv": {
        "pretty", "compact", "sorted", "line_separator", "item_separator"
    },
    "msgpack": {
        "compact"  # Always compact
    },
    "bson": {
        "compact"  # Always compact
    },
    "cbor": {
        "compact"  # Always compact
    },
}


def map_universal_options(
    format_name: str,
    universal_options: dict[str, Any] | None = None,
    **kwargs: Any
) -> dict[str, Any]:
    """
    Map universal options to format-specific options.
    Args:
        format_name: Serialization format name (json, yaml, xml, etc.)
        universal_options: Optional dictionary of universal options
        **kwargs: Universal options as keyword arguments (pretty, compact, sorted, etc.)
    Returns:
        Dictionary of format-specific options
    Example:
        >>> # Using dict
        >>> options = map_universal_options("json", {"pretty": True, "sorted": True})
        >>> # Returns: {"indent": 2, "sort_keys": True}
        >>> 
        >>> # Using keyword arguments
        >>> options = map_universal_options("json", pretty=True, sorted=True)
        >>> # Returns: {"indent": 2, "sort_keys": True}
    """
    # Merge dict and kwargs
    if universal_options is None:
        universal_options = {}
    # Merge kwargs into universal_options (kwargs override dict values)
    merged_options = {**universal_options, **kwargs}
    format_lower = format_name.lower()
    option_map = _FORMAT_OPTION_MAPS.get(format_lower, {})
    format_options = {}
    # Handle canonical first (it implies sorted)
    if merged_options.get("canonical"):
        merged_options["sorted"] = True
    # Handle compact overriding pretty
    if merged_options.get("compact") and merged_options.get("pretty"):
        # Compact wins
        merged_options.pop("pretty", None)
    for universal_key, universal_value in merged_options.items():
        if universal_key in option_map:
            format_key = option_map[universal_key]
            # Handle special mappings
            if universal_key == "pretty" and format_lower == "json":
                format_options[format_key] = 2 if universal_value else None
                format_options["use_orjson"] = False
            elif universal_key == "compact" and format_lower == "json":
                format_options["indent"] = None
                format_options["separators"] = (",", ":") if universal_value else None
                format_options["use_orjson"] = True
            elif universal_key == "pretty" and format_lower == "yaml":
                format_options[format_key] = False if universal_value else True
                format_options["indent"] = 2 if universal_value else None
            elif universal_key == "compact" and format_lower == "yaml":
                format_options[format_key] = True if universal_value else False
            elif universal_key == "pretty" and format_lower == "xml":
                format_options[format_key] = universal_value
            elif universal_key == "compact" and format_lower == "xml":
                format_options[format_key] = False if universal_value else True
            elif universal_key == "canonical" and format_lower == "json":
                format_options["sort_keys"] = True
                format_options["ensure_ascii"] = True
                format_options["indent"] = None
                format_options["canonical"] = True
            else:
                format_options[format_key] = universal_value
    return format_options


def get_supported_universal_options(format_name: str | None = None) -> set[str] | dict[str, Any]:
    """
    Get list of supported universal options for a format, or all universal options info.
    Args:
        format_name: Optional serialization format name. If None, returns all universal options info.
    Returns:
        If format_name provided: Set of supported universal option names for that format
        If format_name is None: Dictionary with all universal options metadata
    Example:
        >>> # Get options for a specific format
        >>> options = get_supported_universal_options("json")
        >>> # Returns: {"pretty", "compact", "sorted", "canonical", ...}
        >>> 
        >>> # Get all universal options info
        >>> all_options = get_supported_universal_options()
        >>> # Returns: {"pretty": {"type": bool, "default": False, "formats": [...]}, ...}
    """
    if format_name is None:
        # Return all universal options with metadata
        option_info = {}
        type_map = {
            "pretty": bool,
            "compact": bool,
            "sorted": bool,
            "canonical": bool,
            "indent": int,
            "ensure_ascii": bool,
            "allow_nan": bool,
            "preserve_quotes": bool,
            "strip_whitespace": bool,
            "encoding": str,
            "declaration": bool,
            "line_separator": str,
            "item_separator": str,
        }
        default_map = {
            "pretty": False,
            "compact": False,
            "sorted": False,
            "canonical": False,
            "indent": None,
            "ensure_ascii": True,
            "allow_nan": True,
            "preserve_quotes": False,
            "strip_whitespace": False,
            "encoding": "utf-8",
            "declaration": True,
            "line_separator": "\n",
            "item_separator": None,
        }
        # Collect all options across all formats
        all_option_names = set()
        for format_opts in _FORMAT_SUPPORTED_OPTIONS.values():
            all_option_names.update(format_opts)
        # Build option info
        for option_name in all_option_names:
            # Find formats that support this option
            supporting_formats = [
                fmt.upper() for fmt, opts in _FORMAT_SUPPORTED_OPTIONS.items()
                if option_name in opts
            ]
            option_info[option_name] = {
                "type": type_map.get(option_name, Any),
                "default": default_map.get(option_name, None),
                "formats": supporting_formats,
            }
        return option_info
    format_lower = format_name.lower()
    return _FORMAT_SUPPORTED_OPTIONS.get(format_lower, set())


def validate_universal_options(
    format_name: str,
    universal_options: dict[str, Any] | None = None,
    **kwargs: Any
) -> tuple[bool, str | None]:
    """
    Validate universal options for a format.
    Args:
        format_name: Serialization format name
        universal_options: Optional dictionary of universal options to validate
        **kwargs: Universal options as keyword arguments
    Returns:
        Tuple of (is_valid, error_message)
    Example:
        >>> # Using dict
        >>> is_valid, error = validate_universal_options(
        ...     "json", {"pretty": True, "invalid_option": True}
        ... )
        >>> # Returns: (False, "Unsupported option: invalid_option")
        >>> 
        >>> # Using keyword arguments
        >>> is_valid, error = validate_universal_options("json", pretty=True)
        >>> # Returns: (True, None)
    """
    # Merge dict and kwargs
    if universal_options is None:
        universal_options = {}
    # Merge kwargs into universal_options (kwargs override dict values)
    merged_options = {**universal_options, **kwargs}
    format_lower = format_name.lower()
    supported = get_supported_universal_options(format_lower)
    # Check for unsupported options
    unsupported = set(merged_options.keys()) - supported
    if unsupported:
        return False, f"Unsupported options for {format_name}: {', '.join(unsupported)}"
    # Validate option types
    type_map = {
        "pretty": bool,
        "compact": bool,
        "sorted": bool,
        "canonical": bool,
        "ensure_ascii": bool,
        "allow_nan": bool,
        "preserve_quotes": bool,
        "strip_whitespace": bool,
        "indent": int,
        "encoding": str,
        "declaration": bool,
        "line_separator": str,
        "item_separator": str,
    }
    for key, value in merged_options.items():
        expected_type = type_map.get(key)
        if expected_type and not isinstance(value, expected_type):
            return False, f"Option '{key}' expects type {expected_type.__name__}, got {type(value).__name__}"
    # Check for conflicting options
    if merged_options.get("pretty") and merged_options.get("compact"):
        if merged_options["pretty"] and merged_options["compact"]:
            return False, "Cannot use both 'pretty' and 'compact' options together"
    return True, None


def get_all_supported_formats() -> list[str]:
    """
    Get list of all formats that support universal options.
    Returns:
        List of format names
    """
    return list(_FORMAT_SUPPORTED_OPTIONS.keys())


def get_format_option_info(format_name: str) -> dict[str, Any]:
    """
    Get detailed information about format option support.
    Args:
        format_name: Serialization format name
    Returns:
        Dictionary with option information
    """
    format_lower = format_name.lower()
    supported = get_supported_universal_options(format_lower)
    option_map = _FORMAT_OPTION_MAPS.get(format_lower, {})
    return {
        "format": format_name,
        "supported_options": list(supported),
        "option_mappings": option_map,
        "total_options": len(supported),
    }
