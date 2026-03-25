#exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/text/json.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.22
Generation Date: November 2, 2025
JSON serialization - Universal, human-readable data interchange format.
Following I→A pattern:
- I: ISerialization (interface)
- A: ASerialization (abstract base)
- Concrete: JsonSerializer
"""

import json
from typing import Any
from pathlib import Path
from ...base import ASerialization
from ...parsers.registry import get_parser
from ...parsers.base import AJsonParser
from ....contracts import EncodeOptions, DecodeOptions
from ....defs import CodecCapability
from ....errors import SerializationError


class JsonSerializer(ASerialization):
    """
    JSON serializer - follows the I→A pattern.
    I: ISerialization (interface)
    A: ASerialization (abstract base)
    Concrete: JsonSerializer
    Uses pluggable JSON parser (auto-detects best available: hyperjson > hybrid > stdlib).
    When hyperjson or hyperlight-hyperjson is installed, it is used for encode/decode;
    atomic operations (atomic_update_path, atomic_read_path, load_file, save_file) use
    the same parser. Falls back to hybrid (msgspec+orjson) or stdlib if unavailable.
    Examples:
        >>> serializer = JsonSerializer()
        >>> 
        >>> # Encode data
        >>> json_str = serializer.encode({"key": "value"})
        >>> # b'{"key": "value"}'
        >>> 
        >>> # Decode data
        >>> data = serializer.decode(b'{"key": "value"}')
        >>> # {'key': 'value'}
        >>> 
        >>> # Save to file
        >>> serializer.save_file({"name": "John"}, "user.json")
        >>> 
        >>> # Load from file
        >>> user = serializer.load_file("user.json")
    """

    def __init__(
        self,
        parser_name: str | None = None,
        max_depth: int | None = None,
        max_size_mb: float | None = None,
    ):
        """
        Initialize JSON serializer with optional parser selection.
        Args:
            parser_name: Parser name ("standard", "orjson", or None for auto-detect)
            max_depth: Maximum nesting depth allowed (passed to ASerialization / ACodec)
            max_size_mb: Maximum estimated data size in MB (passed to ASerialization / ACodec)
        """
        super().__init__(max_depth=max_depth, max_size_mb=max_size_mb)
        self._parser: AJsonParser = get_parser(parser_name)
    # ========================================================================
    # CODEC METADATA
    # ========================================================================
    @property

    def codec_id(self) -> str:
        return "json"
    @property

    def media_types(self) -> list[str]:
        return ["application/json", "text/json"]
    @property

    def file_extensions(self) -> list[str]:
        return [".json", ".webmanifest", ".mcmeta", ".geojson", ".topojson"]
    @property

    def format_name(self) -> str:
        return "JSON"
    @property

    def mime_type(self) -> str:
        return "application/json"
    @property

    def is_binary_format(self) -> bool:
        return False  # JSON is text-based
    @property

    def supports_streaming(self) -> bool:
        return False  # Standard JSON doesn't support streaming
    @property

    def capabilities(self) -> CodecCapability:
        return CodecCapability.BIDIRECTIONAL
    @property

    def aliases(self) -> list[str]:
        return ["json", "JSON"]
    # ========================================================================
    # ADVANCED FEATURE CAPABILITIES
    # ========================================================================
    @property

    def supports_path_based_updates(self) -> bool:
        """JSON supports path-based updates via JSONPointer."""
        return True
    @property

    def supports_atomic_path_write(self) -> bool:
        """JSON supports atomic path writes using JSONPointer."""
        return True
    @property

    def supports_query(self) -> bool:
        """JSON supports queries via JSONPath."""
        return True
    @property

    def supports_lazy_loading(self) -> bool:
        """
        JSON supports lazy loading for large files.
        For large files (10GB+), use atomic path operations (atomic_read_path, atomic_update_path)
        which skip full file validation and only load what's needed.
        """
        return True
    # ========================================================================
    # CORE ENCODE/DECODE (Using official json library)
    # ========================================================================

    def encode(self, value: Any, *, options: EncodeOptions | None = None) -> bytes | str:
        """
        Encode data to JSON string.
        Uses pluggable JSON parser (orjson if available, else stdlib).
        Args:
            value: Data to serialize
            options: JSON options (indent, sort_keys, ensure_ascii, etc.)
        Returns:
            JSON string (as text, not bytes for compatibility)
        Raises:
            SerializationError: If encoding fails
        """
        try:
            opts = options or {}
            # Common JSON options
            indent = opts.get('indent', opts.get('pretty', None))
            if indent is True:
                indent = 2
            sort_keys = opts.get('sort_keys', False)
            ensure_ascii = opts.get('ensure_ascii', False)
            # Use pluggable parser
            result = self._parser.dumps(
                value,
                indent=indent,
                sort_keys=sort_keys,
                ensure_ascii=ensure_ascii,
                default=opts.get('default', None),
                cls=opts.get('cls', None)
            )
            # Convert bytes to str if needed (for compatibility)
            if isinstance(result, bytes):
                # For orjson, decode to string for compatibility
                return result.decode("utf-8")
            return result
        except (TypeError, ValueError, OverflowError) as e:
            raise SerializationError(
                f"Failed to encode JSON: {e}",
                format_name=self.format_name,
                original_error=e
            )

    def decode(self, repr: bytes | str, *, options: DecodeOptions | None = None) -> Any:
        """
        Decode JSON string to data.
        Uses pluggable JSON parser (orjson if available, else stdlib).
        Args:
            repr: JSON string (bytes or str)
            options: JSON options (object_hook, parse_float, etc.)
        Returns:
            Decoded Python object
        Raises:
            SerializationError: If decoding fails
        """
        try:
            opts = options or {}
            # Use pluggable parser (handles bytes/str conversion internally)
            data = self._parser.loads(repr)
            # Note: Advanced options (object_hook, parse_float, etc.) are not
            # supported by orjson. If these are needed, fall back to standard parser.
            # For now, we prioritize performance over feature completeness.
            if opts.get('object_hook') or opts.get('parse_float') or opts.get('parse_int'):
                # Fallback to stdlib for advanced options
                if isinstance(repr, bytes):
                    repr = repr.decode('utf-8')
                return json.loads(
                    repr,
                    object_hook=opts.get('object_hook', None),
                    parse_float=opts.get('parse_float', None),
                    parse_int=opts.get('parse_int', None),
                    parse_constant=opts.get('parse_constant', None),
                    cls=opts.get('cls', None)
                )
            return data
        except (json.JSONDecodeError, ValueError, UnicodeDecodeError) as e:
            raise SerializationError(
                f"Failed to decode JSON: {e}",
                format_name=self.format_name,
                original_error=e
            )
    # ========================================================================
    # ADVANCED FEATURES (Path-based operations)
    # ========================================================================

    @staticmethod
    def _parse_json_pointer(path: str) -> list[str]:
        """Parse RFC6901 JSON Pointer into unescaped tokens."""
        if path == "":
            return []
        if not path.startswith("/"):
            raise ValueError(f"Invalid JSON Pointer path: {path}")
        tokens = path.split("/")[1:]
        return [token.replace("~1", "/").replace("~0", "~") for token in tokens]

    def _resolve_pointer_fallback(self, data: Any, path: str) -> Any:
        """Resolve JSON Pointer without external jsonpointer dependency."""
        current = data
        for token in self._parse_json_pointer(path):
            if isinstance(current, list):
                try:
                    index = int(token)
                except ValueError as e:
                    raise KeyError(f"Invalid list index in path: {token}") from e
                if index < 0 or index >= len(current):
                    raise KeyError(f"List index out of range in path: {token}")
                current = current[index]
            elif isinstance(current, dict):
                if token not in current:
                    raise KeyError(f"Path segment not found: {token}")
                current = current[token]
            else:
                raise KeyError(f"Cannot traverse path segment '{token}' on non-container")
        return current

    def _set_pointer_fallback(self, data: Any, path: str, value: Any) -> None:
        """Set JSON Pointer value without external jsonpointer dependency."""
        tokens = self._parse_json_pointer(path)
        if not tokens:
            raise ValueError("Cannot replace document root via atomic_update_path")
        parent = self._resolve_pointer_fallback(data, "/" + "/".join(
            token.replace("~", "~0").replace("/", "~1") for token in tokens[:-1]
        )) if len(tokens) > 1 else data
        key = tokens[-1]
        if isinstance(parent, list):
            try:
                index = int(key)
            except ValueError as e:
                raise KeyError(f"Invalid list index in path: {key}") from e
            if index < 0 or index >= len(parent):
                raise KeyError(f"List index out of range in path: {key}")
            parent[index] = value
            return
        if isinstance(parent, dict):
            if key not in parent:
                raise KeyError(f"Path segment not found: {key}")
            parent[key] = value
            return
        raise KeyError(f"Cannot set path segment '{key}' on non-container")

    def atomic_update_path(
        self, 
        file_path: str | Path, 
        path: str, 
        value: Any, 
        **options
    ) -> None:
        """
        Atomically update a single path in a JSON file using JSONPointer.
        Uses jsonpointer library for efficient path operations. For large files,
        this still loads the entire file but provides atomic write guarantees.
        Future optimizations could use streaming JSON parsers for true partial updates.
        Args:
            file_path: Path to the JSON file
            path: JSONPointer path (e.g., "/users/0/name")
            value: Value to set at the specified path
            **options: Options (backup=True, etc.)
        Raises:
            SerializationError: If update fails
            ValueError: If path is invalid
            KeyError: If path doesn't exist
        Example:
            >>> serializer = JsonSerializer()
            >>> serializer.atomic_update_path("config.json", "/database/host", "localhost")
        """
        jsonpointer = None
        try:
            # Optional dependency; fallback logic is used when unavailable.
            import jsonpointer as _jsonpointer
            jsonpointer = _jsonpointer
        except ImportError:
            pass
        try:
            path_obj = Path(file_path)
            if not path_obj.exists():
                raise FileNotFoundError(f"File not found: {path_obj}")
            # Validate path security
            from ...utils.path_ops import validate_path_security
            validate_path_security(path)
            # Load entire file
            # For large files (10GB+), skip size validation to allow atomic operations
            # Root cause: Large files use atomic path operations without full validation
            # Solution: Skip size check for atomic operations (depth check still performed)
            large_file_options = {**options, 'skip_size_check': True}
            data = self.load_file(file_path, **large_file_options)
            if jsonpointer is not None:
                jsonpointer.set_pointer(data, path, value)
            else:
                self._set_pointer_fallback(data, path, value)
            # Save atomically using AtomicFileWriter
            from ....common.atomic import AtomicFileWriter
            repr_data = self.encode(data, options=options or None)
            path_obj.parent.mkdir(parents=True, exist_ok=True)
            backup = options.get('backup', True)
            encoding = options.get('encoding', 'utf-8')
            # JSON encode returns str, not bytes - write as text
            with AtomicFileWriter(path_obj, mode='w', encoding=encoding, backup=backup) as writer:
                if isinstance(repr_data, bytes):
                    # If somehow we got bytes, decode first
                    writer.write(repr_data.decode(encoding))
                else:
                    writer.write(repr_data)
        except (FileNotFoundError, ValueError, KeyError):
            raise
        except Exception as e:
            if jsonpointer is not None and isinstance(e, jsonpointer.JsonPointerException):
                raise KeyError(f"Path not found: {path}") from e
            raise SerializationError(
                f"Failed to atomically update path '{path}' in JSON file: {e}",
                format_name=self.format_name,
                original_error=e
            ) from e

    def atomic_read_path(
        self, 
        file_path: str | Path, 
        path: str, 
        **options
    ) -> Any:
        """
        Read a single path from a JSON file using JSONPointer.
        Uses jsonpointer library for efficient path operations. For large files,
        this still loads the entire file. Future optimizations could use streaming
        JSON parsers for true partial reads.
        Args:
            file_path: Path to the JSON file
            path: JSONPointer path (e.g., "/users/0/name")
            **options: Options
        Returns:
            Value at the specified path
        Raises:
            SerializationError: If read fails
            KeyError: If path doesn't exist
        Example:
            >>> serializer = JsonSerializer()
            >>> host = serializer.atomic_read_path("config.json", "/database/host")
        """
        jsonpointer = None
        try:
            # Optional dependency; fallback logic is used when unavailable.
            import jsonpointer as _jsonpointer
            jsonpointer = _jsonpointer
        except ImportError:
            pass
        try:
            path_obj = Path(file_path)
            if not path_obj.exists():
                raise FileNotFoundError(f"File not found: {path_obj}")
            # Validate path security
            from ...utils.path_ops import validate_path_security
            validate_path_security(path)
            # Load entire file
            # For large files (10GB+), skip size validation to allow atomic operations
            # Root cause: Large files use atomic path operations without full validation
            # Solution: Skip size check for atomic operations (depth check still performed)
            large_file_options = {**options, 'skip_size_check': True}
            data = self.load_file(file_path, **large_file_options)
            if jsonpointer is not None:
                return jsonpointer.resolve_pointer(data, path)
            return self._resolve_pointer_fallback(data, path)
        except (FileNotFoundError, KeyError, ValueError):
            raise
        except Exception as e:
            if jsonpointer is not None and isinstance(e, jsonpointer.JsonPointerException):
                raise KeyError(f"Path not found: {path}") from e
            raise SerializationError(
                f"Failed to read path '{path}' from JSON file: {e}",
                format_name=self.format_name,
                original_error=e
            ) from e

    def query(
        self, 
        file_path: str | Path, 
        query_expr: str, 
        **options
    ) -> Any:
        """
        Query JSON file using JSONPath expression.
        Uses jsonpath-ng library for JSONPath queries.
        Args:
            file_path: Path to the JSON file
            query_expr: JSONPath expression (e.g., "$.users[*].name")
            **options: Query options
        Returns:
            Query results (list of matching values)
        Raises:
            SerializationError: If query fails
            ValueError: If query expression is invalid
        Example:
            >>> serializer = JsonSerializer()
            >>> names = serializer.query("users.json", "$.users[*].name")
        """
        # Import jsonpath_ng (lazy loaded via lazy_package system)
        from jsonpath_ng import parse as parse_jsonpath
        try:
            path_obj = Path(file_path)
            if not path_obj.exists():
                raise FileNotFoundError(f"File not found: {path_obj}")
            # Load file
            data = self.load_file(file_path, **options)
            # Parse JSONPath expression
            jsonpath_expr = parse_jsonpath(query_expr)
            # Execute query
            matches = [match.value for match in jsonpath_expr.find(data)]
            return matches
        except (FileNotFoundError, ValueError) as e:
            raise
        except Exception as e:
            raise SerializationError(
                f"Failed to query JSON file with expression '{query_expr}': {e}",
                format_name=self.format_name,
                original_error=e
            ) from e
