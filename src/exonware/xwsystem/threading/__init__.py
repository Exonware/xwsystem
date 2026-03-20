#exonware/xwsystem/src/exonware/xwsystem/threading/__init__.py
"""
Threading utilities for safe concurrent operations.
Performance (from benchmarks): For hot-path locking without timeout or stats,
use fast_lock() or EnhancedRLock(track_stats=False) (~3.4M ops/s vs ~1.5M with
track_stats=True). threading.RLock remains ~7.1M ops/s when you need no extras.
"""

from .locks import EnhancedRLock
from .safe_factory import MethodGenerator, ThreadSafeFactory
from .async_primitives import AsyncRWLock
# Unified Facade
from .facade import XWConcurrency


def fast_lock(name: str | None = None) -> EnhancedRLock:
    """
    Create an EnhancedRLock with track_stats=False for maximum throughput.
    Use when you need reentrant locking but not timeout or get_stats().
    Benchmarks: ~3.4M ops/s vs ~1.5M with track_stats=True (see REF_54_BENCH).
    """
    return EnhancedRLock(track_stats=False, name=name or None)


def create_thread_safe_cache(max_size: int = 128):
    """
    Create a thread-safe cache with LRU eviction policy.
    Uses flexible create_cache() to allow configuration via environment/settings.
    Defaults to PylruCache when pylru is installed (highest throughput in benchmarks),
    else FunctoolsLRUCache.
    Args:
        max_size: Maximum number of items in cache
    Returns:
        Cache instance (defaults to PylruCache when installed)
    """
    from ..caching import create_cache
    return create_cache(capacity=max_size, namespace='xwsystem.threading')
__all__ = [
    # Unified Facade
    "XWConcurrency",
    # Core Classes
    "ThreadSafeFactory",
    "MethodGenerator",
    "EnhancedRLock",
    "AsyncRWLock",
    "fast_lock",
    "create_thread_safe_cache",
]
