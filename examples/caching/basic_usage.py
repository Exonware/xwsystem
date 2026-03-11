#!/usr/bin/env python3
#exonware/xwsystem/examples/caching/basic_usage.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.388
Generation Date: 01-Nov-2025
Basic caching examples for xwsystem.
"""

import sys
from pathlib import Path
# Add src to path
src_path = Path(__file__).parent.parent.parent / "src"
sys.path.insert(0, str(src_path))
from exonware.xwsystem.caching.lru_cache import LRUCache
from exonware.xwsystem.caching.lfu_optimized import OptimizedLFUCache
from exonware.xwsystem.caching.ttl_cache import TTLCache


def example_1_basic_lru():
    """Example 1: Basic LRU caching."""
    print("="*80)
    print("Example 1: Basic LRU Caching")
    print("="*80)
    # Create cache
    cache = LRUCache(capacity=100)
    # Store data
    cache.put('user:123', {'name': 'Alice', 'email': 'alice@example.com'})
    cache.put('user:456', {'name': 'Bob', 'email': 'bob@example.com'})
    # Retrieve data
    user = cache.get('user:123')
    print(f"Retrieved user: {user}")
    # Dictionary-like interface
    cache['product:789'] = {'name': 'Widget', 'price': 29.99}
    product = cache['product:789']
    print(f"Retrieved product: {product}")
    # Check existence
    if 'user:123' in cache:
        print("User 123 is cached")
    # Get statistics
    stats = cache.get_stats()
    print(f"Cache stats: {stats}")
    print()


def example_2_lfu_optimized():
    """Example 2: High-performance LFU caching."""
    print("="*80)
    print("Example 2: Optimized LFU Caching (O(1) eviction)")
    print("="*80)
    # For frequently accessed data
    cache = OptimizedLFUCache(capacity=1000)
    # Simulate access patterns
    for i in range(2000):
        cache.put(f'item_{i}', f'data_{i}')
        # Hot items accessed more frequently
        if i % 10 == 0:
            cache.get(f'item_{i}')
            cache.get(f'item_{i}')
            cache.get(f'item_{i}')
    stats = cache.get_stats()
    print(f"Cache size: {stats['size']}")
    print(f"Hit rate: {stats['hit_rate']:.2%}")
    print(f"Evictions: {stats['evictions']}")
    print(f"Min frequency: {stats['min_freq']}")
    print()


def example_3_ttl_cache():
    """Example 3: TTL caching with expiration."""
    print("="*80)
    print("Example 3: TTL Caching (Time-based expiration)")
    print("="*80)
    import time
    # Create TTL cache
    cache = TTLCache(capacity=100, ttl=2.0)  # 2 second TTL
    # Store data
    cache.put('session:abc', {'user_id': 123, 'token': 'xyz'})
    # Available immediately
    session = cache.get('session:abc')
    print(f"Session (fresh): {session}")
    # Wait for expiration
    print("Waiting 2.5 seconds for TTL expiration...")
    time.sleep(2.5)
    # Should be expired
    expired_session = cache.get('session:abc')
    print(f"Session (after TTL): {expired_session}")
    print()


def example_4_context_manager():
    """Example 4: Using cache as context manager."""
    print("="*80)
    print("Example 4: Context Manager Pattern")
    print("="*80)
    # Automatic resource management
    with LRUCache(capacity=50) as cache:
        cache.put('temp_key', 'temp_value')
        print(f"Inside context: {cache.get('temp_key')}")
    print("Context exited successfully")
    print()


def example_5_statistics():
    """Example 5: Cache statistics and monitoring."""
    print("="*80)
    print("Example 5: Statistics and Monitoring")
    print("="*80)
    cache = LRUCache(capacity=10)
    # Perform operations
    cache.put('key1', 'value1')
    cache.put('key2', 'value2')
    cache.get('key1')  # Hit
    cache.get('key3')  # Miss
    cache.get('key1')  # Hit
    # Get detailed statistics
    stats = cache.get_stats()
    print(f"Cache Name: {stats['name']}")
    print(f"Type: {stats['type']}")
    print(f"Capacity: {stats['capacity']}")
    print(f"Size: {stats['size']}")
    print(f"Hits: {stats['hits']}")
    print(f"Misses: {stats['misses']}")
    print(f"Hit Rate: {stats['hit_rate']:.2%}")
    print()
if __name__ == "__main__":
    example_1_basic_lru()
    example_2_lfu_optimized()
    example_3_ttl_cache()
    example_4_context_manager()
    example_5_statistics()
    print("="*80)
    print("✅ All examples completed successfully!")
    print("="*80)
