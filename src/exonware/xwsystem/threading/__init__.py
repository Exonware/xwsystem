#exonware/xwsystem/src/exonware/xwsystem/threading/__init__.py
"""
Threading utilities for safe concurrent operations.
"""

from .locks import EnhancedRLock
from .safe_factory import MethodGenerator, ThreadSafeFactory

# Unified Facade
from .facade import XWConcurrency


def create_thread_safe_cache(max_size=128, maxsize=None):
    """
    Create a thread-safe cache with LRU eviction policy.
    
    Uses flexible create_cache() to allow configuration via environment/settings.
    Defaults to FunctoolsLRUCache (fastest Python cache).
    
    Args:
        max_size: Maximum number of items in cache (primary parameter)
        maxsize: Alternative parameter name for compatibility
        
    Returns:
        Cache instance (defaults to FunctoolsLRUCache)
    """
    from ..caching import create_cache
    # Support both parameter names for compatibility
    size = maxsize if maxsize is not None else max_size
    # Use flexible create_cache() to allow configuration via environment/settings
    # Defaults to FunctoolsLRUCache (fastest Python cache)
    return create_cache(capacity=size, namespace='xwsystem.threading')


__all__ = [
    # Unified Facade
    "XWConcurrency",
    # Core Classes
    "ThreadSafeFactory",
    "MethodGenerator",
    "EnhancedRLock",
    "create_thread_safe_cache",
]
