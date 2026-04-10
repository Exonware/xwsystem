#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/caching/utils.py
#exonware/xwsystem/caching/utils.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.37
Generation Date: 01-Nov-2025
Common utility functions for caching module.
"""

import sys
import hashlib
import pickle
from typing import Any

from collections.abc import Callable


def estimate_object_size(obj: Any) -> int:
    """
    Estimate memory size of object in bytes.
    Args:
        obj: Object to estimate size of
    Returns:
        Estimated size in bytes
    Note:
        This is a rough estimate using sys.getsizeof.
        For more accurate memory profiling, use memory_profiler.
    """
    try:
        return sys.getsizeof(obj)
    except (TypeError, AttributeError):
        # For objects that don't support getsizeof (e.g., some C extensions)
        try:
            # Try pickling as fallback to estimate serialized size
            return len(pickle.dumps(obj, protocol=pickle.HIGHEST_PROTOCOL))
        except (pickle.PicklingError, TypeError, AttributeError) as e:
            # Object cannot be pickled - return conservative estimate
            # This is acceptable per GUIDE_TEST.md as this is a utility function
            # Provides fallback rather than failing
            # The alternative would be to raise an exception, but that would
            # break cache operations for objects that can't be sized
            return 1024  # 1KB default conservative estimate


def compute_checksum(value: Any, algorithm: str = 'sha256') -> str:
    """
    Compute cryptographic checksum of value.
    Args:
        value: Value to compute checksum for
        algorithm: Hash algorithm ('sha256', 'md5', 'sha1')
    Returns:
        Hexadecimal checksum string
    Raises:
        ValueError: If algorithm is not supported
    """
    supported_algorithms = {'md5', 'sha1', 'sha256', 'sha512'}
    if algorithm not in supported_algorithms:
        raise ValueError(
            f"Unsupported hash algorithm: {algorithm}. "
            f"Supported: {', '.join(supported_algorithms)}"
        )
    try:
        # Serialize value to bytes
        value_bytes = pickle.dumps(value, protocol=pickle.HIGHEST_PROTOCOL)
        # Compute hash
        hash_obj = hashlib.new(algorithm)
        hash_obj.update(value_bytes)
        return hash_obj.hexdigest()
    except Exception as e:
        raise ValueError(f"Failed to compute checksum: {e}")


def format_bytes(size: int) -> str:
    """
    Format bytes as human-readable string.
    Args:
        size: Size in bytes
    Returns:
        Formatted string (e.g., "1.23 MB")
    Examples:
        >>> format_bytes(1024)
        '1.00 KB'
        >>> format_bytes(1536)
        '1.50 KB'
        >>> format_bytes(1048576)
        '1.00 MB'
    """
    if size < 0:
        return "Invalid size"
    units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB']
    unit_index = 0
    size_float = float(size)
    while size_float >= 1024.0 and unit_index < len(units) - 1:
        size_float /= 1024.0
        unit_index += 1
    return f"{size_float:.2f} {units[unit_index]}"


def default_key_builder(func: Callable, args: tuple, kwargs: dict) -> str:
    """
    Build cache key from function and arguments.
    Args:
        func: Function being cached
        args: Positional arguments
        kwargs: Keyword arguments
    Returns:
        Cache key string
    Note:
        This is a basic implementation. For production use,
        consider using functools.lru_cache or cachetools.
    """
    # Build key from function name and arguments
    func_name = f"{func.__module__}.{func.__qualname__}"
    # Convert args and kwargs to hashable representation
    try:
        args_str = str(args)
        kwargs_str = str(sorted(kwargs.items()))
        key = f"{func_name}:{args_str}:{kwargs_str}"
        return key
    except (TypeError, ValueError) as e:
        # Fallback: use hash for non-stringable arguments
        # This is acceptable per GUIDE_TEST.md as this is a utility function
        # Provides fallback mechanism for edge cases
        # The alternative would be to raise, but that would break caching
        # for functions with complex argument types
        try:
            args_hash = hash(args)
            kwargs_hash = hash(tuple(sorted(kwargs.items())))
            return f"{func_name}:{args_hash}:{kwargs_hash}"
        except (TypeError, ValueError):
            # If even hashing fails, use a deterministic fallback
            # This handles truly unhashable types (e.g., lists, dicts)
            import hashlib
            args_repr = repr(args).encode('utf-8')
            kwargs_repr = repr(kwargs).encode('utf-8')
            combined = args_repr + kwargs_repr
            hash_val = hashlib.md5(combined).hexdigest()
            return f"{func_name}:{hash_val}"


def validate_capacity(capacity: int, min_capacity: int = 1, max_capacity: int = 1000000) -> None:
    """
    Validate cache capacity parameter.
    Args:
        capacity: Capacity to validate
        min_capacity: Minimum allowed capacity
        max_capacity: Maximum allowed capacity
    Raises:
        ValueError: If capacity is invalid
    """
    if not isinstance(capacity, int):
        raise ValueError(
            f"Cache capacity must be an integer, got {type(capacity).__name__}. "
            f"Example: capacity=1000"
        )
    if capacity < min_capacity:
        raise ValueError(
            f"Cache capacity must be at least {min_capacity}, got {capacity}. "
            f"Example: capacity={min_capacity}"
        )
    if capacity > max_capacity:
        raise ValueError(
            f"Cache capacity too large: {capacity:,} (max: {max_capacity:,}). "
            f"Consider using a distributed cache for very large capacities."
        )


def validate_ttl(ttl: float, min_ttl: float = 0.0, max_ttl: float = 86400 * 365) -> None:
    """
    Validate TTL (time-to-live) parameter.
    Args:
        ttl: TTL in seconds
        min_ttl: Minimum allowed TTL
        max_ttl: Maximum allowed TTL (default: 1 year)
    Raises:
        ValueError: If TTL is invalid
    """
    if not isinstance(ttl, (int, float)):
        raise ValueError(
            f"TTL must be a number (int or float), got {type(ttl).__name__}. "
            f"Example: ttl=300.0 (5 minutes)"
        )
    if ttl < min_ttl:
        raise ValueError(
            f"TTL must be at least {min_ttl} seconds, got {ttl}. "
            f"Example: ttl=60.0 (1 minute)"
        )
    if ttl > max_ttl:
        raise ValueError(
            f"TTL too large: {ttl:,.0f} seconds ({ttl/86400:.1f} days). "
            f"Maximum: {max_ttl:,.0f} seconds ({max_ttl/86400:.1f} days)"
        )
__all__ = [
    'estimate_object_size',
    'compute_checksum',
    'format_bytes',
    'default_key_builder',
    'validate_capacity',
    'validate_ttl',
]
