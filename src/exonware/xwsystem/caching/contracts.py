#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/caching/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.12
Generation Date: September 04, 2025
Caching protocol interfaces for XWSystem.
"""

from typing import Any, Protocol, runtime_checkable

from collections.abc import Iterator, Callable
import time
# Import enums from types module
from .defs import (
    CachePolicy,
    CacheStatus,
    CacheEvent,
    CacheLevel
)
# ============================================================================
# CACHEABLE INTERFACES
# ============================================================================
@runtime_checkable

class ICacheable(Protocol):
    """
    Interface for cacheable objects.
    Enforces consistent caching behavior across XWSystem.
    """

    def cache(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """
        Cache a value with key.
        Args:
            key: Cache key
            value: Value to cache
            ttl: Time to live in seconds
        Returns:
            True if cached successfully
        """
        ...

    def get_cached(self, key: str, default: Any = None) -> Any:
        """
        Get cached value by key.
        Args:
            key: Cache key
            default: Default value if not found
        Returns:
            Cached value or default
        """
        ...

    def clear_cache(self) -> None:
        """
        Clear all cached values.
        """
        ...

    def has_cached(self, key: str) -> bool:
        """
        Check if key is cached.
        Args:
            key: Cache key to check
        Returns:
            True if cached
        """
        ...

    def remove_cached(self, key: str) -> bool:
        """
        Remove cached value by key.
        Args:
            key: Cache key to remove
        Returns:
            True if removed
        """
        ...

    def get_cache_size(self) -> int:
        """
        Get number of cached items.
        Returns:
            Number of cached items
        """
        ...

    def get_cache_info(self) -> dict[str, Any]:
        """
        Get cache information.
        Returns:
            Cache information dictionary
        """
        ...
# ============================================================================
# CACHE MANAGER INTERFACES
# ============================================================================
@runtime_checkable

class ICacheManager(Protocol):
    """
    Interface for cache management.
    Enforces consistent cache management across XWSystem.
    """

    def create_cache(self, name: str, max_size: int = 1000, policy: CachePolicy = CachePolicy.LRU) -> ICacheable:
        """
        Create a new cache.
        Args:
            name: Cache name
            max_size: Maximum cache size
            policy: Eviction policy
        Returns:
            Cache instance
        """
        ...

    def get_cache(self, name: str) -> ICacheable | None:
        """
        Get cache by name.
        Args:
            name: Cache name
        Returns:
            Cache instance or None
        """
        ...

    def remove_cache(self, name: str) -> bool:
        """
        Remove cache by name.
        Args:
            name: Cache name to remove
        Returns:
            True if removed
        """
        ...

    def list_caches(self) -> list[str]:
        """
        List all cache names.
        Returns:
            List of cache names
        """
        ...

    def clear_all_caches(self) -> None:
        """
        Clear all caches.
        """
        ...

    def get_cache_stats(self) -> dict[str, dict[str, Any]]:
        """
        Get statistics for all caches.
        Returns:
            Dictionary of cache statistics
        """
        ...

    def set_global_policy(self, policy: CachePolicy) -> None:
        """
        Set global cache policy.
        Args:
            policy: Global eviction policy
        """
        ...

    def get_global_policy(self) -> CachePolicy:
        """
        Get global cache policy.
        Returns:
            Global eviction policy
        """
        ...
# ============================================================================
# CACHE STORAGE INTERFACES
# ============================================================================
@runtime_checkable

class ICacheStorage(Protocol):
    """
    Interface for cache storage backends.
    Enforces consistent cache storage across XWSystem.
    """

    def store(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """
        Store value in cache.
        Args:
            key: Cache key
            value: Value to store
            ttl: Time to live in seconds
        Returns:
            True if stored successfully
        """
        ...

    def retrieve(self, key: str) -> Any | None:
        """
        Retrieve value from cache.
        Args:
            key: Cache key
        Returns:
            Cached value or None
        """
        ...

    def delete(self, key: str) -> bool:
        """
        Delete value from cache.
        Args:
            key: Cache key to delete
        Returns:
            True if deleted
        """
        ...

    def exists(self, key: str) -> bool:
        """
        Check if key exists in cache.
        Args:
            key: Cache key
        Returns:
            True if exists
        """
        ...

    def clear(self) -> None:
        """
        Clear all cached values.
        """
        ...

    def size(self) -> int:
        """
        Get number of cached items.
        Returns:
            Number of cached items
        """
        ...

    def keys(self) -> Iterator[str]:
        """
        Get iterator over cache keys.
        Yields:
            Cache keys
        """
        ...

    def values(self) -> Iterator[Any]:
        """
        Get iterator over cache values.
        Yields:
            Cache values
        """
        ...

    def items(self) -> Iterator[tuple[str, Any]]:
        """
        Get iterator over cache items.
        Yields:
            Tuples of (key, value)
        """
        ...
# ============================================================================
# CACHE EVICTION INTERFACES
# ============================================================================
@runtime_checkable

class ICacheEviction(Protocol):
    """
    Interface for cache eviction strategies.
    Enforces consistent cache eviction across XWSystem.
    """

    def should_evict(self, cache_size: int, max_size: int) -> bool:
        """
        Check if eviction is needed.
        Args:
            cache_size: Current cache size
            max_size: Maximum cache size
        Returns:
            True if eviction needed
        """
        ...

    def select_eviction_candidate(self, items: list[tuple[str, Any, float]]) -> str:
        """
        Select item to evict.
        Args:
            items: List of (key, value, metadata) tuples
        Returns:
            Key of item to evict
        """
        ...

    def update_access(self, key: str, timestamp: float) -> None:
        """
        Update access information for key.
        Args:
            key: Cache key
            timestamp: Access timestamp
        """
        ...

    def get_eviction_policy(self) -> CachePolicy:
        """
        Get eviction policy.
        Returns:
            Eviction policy
        """
        ...

    def set_eviction_policy(self, policy: CachePolicy) -> None:
        """
        Set eviction policy.
        Args:
            policy: Eviction policy to set
        """
        ...

    def get_eviction_stats(self) -> dict[str, Any]:
        """
        Get eviction statistics.
        Returns:
            Eviction statistics dictionary
        """
        ...
# ============================================================================
# CACHE MONITORING INTERFACES
# ============================================================================
@runtime_checkable

class ICacheMonitor(Protocol):
    """
    Interface for cache monitoring.
    Enforces consistent cache monitoring across XWSystem.
    """

    def record_hit(self, key: str) -> None:
        """
        Record cache hit.
        Args:
            key: Cache key that was hit
        """
        ...

    def record_miss(self, key: str) -> None:
        """
        Record cache miss.
        Args:
            key: Cache key that was missed
        """
        ...

    def record_set(self, key: str, size: int) -> None:
        """
        Record cache set operation.
        Args:
            key: Cache key that was set
            size: Size of cached value
        """
        ...

    def record_delete(self, key: str) -> None:
        """
        Record cache delete operation.
        Args:
            key: Cache key that was deleted
        """
        ...

    def record_eviction(self, key: str, reason: str) -> None:
        """
        Record cache eviction.
        Args:
            key: Cache key that was evicted
            reason: Eviction reason
        """
        ...

    def get_hit_rate(self) -> float:
        """
        Get cache hit rate.
        Returns:
            Hit rate as percentage (0.0 to 1.0)
        """
        ...

    def get_miss_rate(self) -> float:
        """
        Get cache miss rate.
        Returns:
            Miss rate as percentage (0.0 to 1.0)
        """
        ...

    def get_stats(self) -> dict[str, Any]:
        """
        Get cache statistics.
        Returns:
            Statistics dictionary
        """
        ...

    def reset_stats(self) -> None:
        """
        Reset cache statistics.
        """
        ...
# ============================================================================
# DISTRIBUTED CACHE INTERFACES
# ============================================================================
@runtime_checkable

class IDistributedCache(Protocol):
    """
    Interface for distributed cache operations.
    Enforces consistent distributed caching across XWSystem.
    """

    def get_node_id(self) -> str:
        """
        Get current node ID.
        Returns:
            Node identifier
        """
        ...

    def get_cluster_nodes(self) -> list[str]:
        """
        Get list of cluster nodes.
        Returns:
            List of node IDs
        """
        ...

    def replicate(self, key: str, value: Any, nodes: list[str]) -> bool:
        """
        Replicate cache entry to nodes.
        Args:
            key: Cache key
            value: Cache value
            nodes: Target nodes
        Returns:
            True if replicated successfully
        """
        ...

    def invalidate(self, key: str, nodes: list[str]) -> bool:
        """
        Invalidate cache entry on nodes.
        Args:
            key: Cache key to invalidate
            nodes: Target nodes
        Returns:
            True if invalidated successfully
        """
        ...

    def sync_with_node(self, node_id: str) -> bool:
        """
        Sync cache with specific node.
        Args:
            node_id: Node to sync with
        Returns:
            True if synced successfully
        """
        ...

    def get_consistency_level(self) -> str:
        """
        Get cache consistency level.
        Returns:
            Consistency level (e.g., 'strong', 'eventual')
        """
        ...

    def set_consistency_level(self, level: str) -> None:
        """
        Set cache consistency level.
        Args:
            level: Consistency level
        """
        ...
# ============================================================================
# CACHE DECORATOR INTERFACES
# ============================================================================
@runtime_checkable

class ICacheDecorator(Protocol):
    """
    Interface for cache decorators.
    Enforces consistent cache decoration across XWSystem.
    """

    def cache_result(self, func: Callable, ttl: int | None = None, key_func: Callable | None = None) -> Callable:
        """
        Decorate function to cache results.
        Args:
            func: Function to decorate
            ttl: Time to live in seconds
            key_func: Function to generate cache key
        Returns:
            Decorated function
        """
        ...

    def cache_invalidate(self, func: Callable, key_func: Callable | None = None) -> Callable:
        """
        Decorate function to invalidate cache.
        Args:
            func: Function to decorate
            key_func: Function to generate cache key
        Returns:
            Decorated function
        """
        ...

    def cache_clear(self, func: Callable) -> Callable:
        """
        Decorate function to clear cache.
        Args:
            func: Function to decorate
        Returns:
            Decorated function
        """
        ...

    def cache_conditional(self, func: Callable, condition: Callable[[Any], bool]) -> Callable:
        """
        Decorate function with conditional caching.
        Args:
            func: Function to decorate
            condition: Condition function for caching
        Returns:
            Decorated function
        """
        ...
# ============================================================================
# CACHE PERSISTENCE INTERFACES
# ============================================================================
@runtime_checkable

class ICachePersistence(Protocol):
    """
    Interface for cache persistence.
    Enforces consistent cache persistence across XWSystem.
    """

    def save_cache(self, cache_name: str, file_path: str) -> bool:
        """
        Save cache to file.
        Args:
            cache_name: Name of cache to save
            file_path: File path to save to
        Returns:
            True if saved successfully
        """
        ...

    def load_cache(self, cache_name: str, file_path: str) -> bool:
        """
        Load cache from file.
        Args:
            cache_name: Name of cache to load
            file_path: File path to load from
        Returns:
            True if loaded successfully
        """
        ...

    def backup_cache(self, cache_name: str, backup_path: str) -> bool:
        """
        Backup cache to file.
        Args:
            cache_name: Name of cache to backup
            backup_path: Backup file path
        Returns:
            True if backed up successfully
        """
        ...

    def restore_cache(self, cache_name: str, backup_path: str) -> bool:
        """
        Restore cache from backup.
        Args:
            cache_name: Name of cache to restore
            backup_path: Backup file path
        Returns:
            True if restored successfully
        """
        ...

    def get_persistence_format(self) -> str:
        """
        Get persistence format.
        Returns:
            Format name (e.g., 'pickle', 'json')
        """
        ...

    def set_persistence_format(self, format_name: str) -> None:
        """
        Set persistence format.
        Args:
            format_name: Format name
        """
        ...
# ============================================================================
# BASIC CACHE INTERFACE
# ============================================================================
@runtime_checkable

class ICache(Protocol):
    """
    Basic cache interface for disk-based and specialized caches.
    Root cause fixed: Added missing ICache interface that was being used by
    TwoTierCache and DiskCache but didn't exist in contracts.
    This is a simpler interface than ICacheable, designed for caches that
    use string keys (like disk caches and two-tier caches).
    """

    def get(self, key: str) -> Any | None:
        """Get value from cache."""
        ...

    def set(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """Set value in cache."""
        ...

    def delete(self, key: str) -> bool:
        """Delete value from cache."""
        ...

    def clear(self) -> bool:
        """Clear all cached values."""
        ...

    def exists(self, key: str) -> bool:
        """Check if key exists in cache."""
        ...

    def size(self) -> int:
        """Get number of cached items."""
        ...
# ============================================================================
# CACHING PROTOCOLS
# ============================================================================
@runtime_checkable

class ICacheableSimple(Protocol):
    """Protocol for objects that support caching (simpler interface than ICacheable)."""

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        ...

    def set(self, key: str, value: Any, ttl: int | None = None) -> None:
        """Set value in cache."""
        ...

    def delete(self, key: str) -> bool:
        """Delete value from cache."""
        ...

    def clear(self) -> None:
        """Clear all cached values."""
        ...
