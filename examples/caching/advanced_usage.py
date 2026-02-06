#!/usr/bin/env python3
#exonware/xwsystem/examples/caching/advanced_usage.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1.388
Generation Date: 01-Nov-2025

Advanced caching examples for xwsystem.
"""

import sys
import time
import asyncio
from pathlib import Path

# Add src to path
src_path = Path(__file__).parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from exonware.xwsystem.caching.decorators import xwcached, xw_async_cached
from exonware.xwsystem.caching.lru_cache import AsyncLRUCache
from exonware.xwsystem.caching.secure_cache import SecureLRUCache
from exonware.xwsystem.caching.memory_bounded import MemoryBoundedLRUCache
from exonware.xwsystem.caching.read_through import ReadThroughCache, WriteThroughCache
from exonware.xwsystem.caching.tagging import TaggedCache


def example_decorator_basic():
    """Example 1: Basic decorator usage."""
    print("="*80)
    print("Example 1: Function Caching with @xwcached")
    print("="*80)
    
    call_count = [0]
    
    @xwcached(ttl=300)
    def expensive_computation(n):
        """Simulate expensive computation."""
        call_count[0] += 1
        result = sum(i * i for i in range(n))
        return result
    
    # First call - executes function
    print("First call...")
    result1 = expensive_computation(1000)
    print(f"Result: {result1}, Function called: {call_count[0]} time(s)")
    
    # Second call - uses cache
    print("Second call with same argument...")
    result2 = expensive_computation(1000)
    print(f"Result: {result2}, Function called: {call_count[0]} time(s)")
    
    # Different argument - executes function
    print("Third call with different argument...")
    result3 = expensive_computation(2000)
    print(f"Result: {result3}, Function called: {call_count[0]} time(s)")
    print()


def example_decorator_advanced():
    """Example 2: Advanced decorator with hooks."""
    print("="*80)
    print("Example 2: Advanced Decorator with Hooks")
    print("="*80)
    
    hits = []
    misses = []
    
    @xwcached(
        namespace="user_api",
        condition=lambda args, kwargs: args[0] > 0,  # Only cache positive IDs
        on_hit=lambda k, v: hits.append(k),
        on_miss=lambda k, v: misses.append(k),
        key_builder=lambda f, a, kw: f"user:{a[0]}"
    )
    def get_user(user_id):
        """Simulated user lookup."""
        return {'id': user_id, 'name': f'User_{user_id}'}
    
    # Positive ID - will cache
    user1 = get_user(123)
    user2 = get_user(123)  # Cache hit
    print(f"Hits: {len(hits)}, Misses: {len(misses)}")
    
    # Negative ID - won't cache
    user3 = get_user(-1)
    user4 = get_user(-1)  # Not cached, executes again
    print(f"Negative IDs don't cache")
    print()


async def example_async_caching():
    """Example 3: Async caching."""
    print("="*80)
    print("Example 3: Async Caching")
    print("="*80)
    
    cache = AsyncLRUCache(capacity=100)
    
    # Async operations
    await cache.put('key1', 'value1')
    value = await cache.get('key1')
    print(f"Async get: {value}")
    
    # Async context manager
    async with AsyncLRUCache(50) as temp_cache:
        await temp_cache.put('temp', 'data')
        temp_value = await temp_cache.get('temp')
        print(f"Temp cache value: {temp_value}")
    
    # Async decorator
    call_count = [0]
    
    @xw_async_cached(ttl=60)
    async def async_computation(x):
        call_count[0] += 1
        await asyncio.sleep(0.01)  # Simulate async work
        return x * 2
    
    result1 = await async_computation(5)
    result2 = await async_computation(5)  # Cached
    print(f"Async decorator: result={result2}, calls={call_count[0]}")
    print()


def example_secure_cache():
    """Example 4: Secure caching with validation."""
    print("="*80)
    print("Example 4: Secure Caching (Production)")
    print("="*80)
    
    cache = SecureLRUCache(
        capacity=1000,
        enable_integrity=True,
        enable_rate_limit=True,
        max_ops_per_second=10000,
        max_key_size=1024,
        max_value_size_mb=10.0
    )
    
    # Normal usage
    cache.put('api_response', {'data': [1, 2, 3, 4, 5]})
    response = cache.get('api_response')
    print(f"Cached API response: {response}")
    
    # Security stats
    sec_stats = cache.get_security_stats()
    print(f"Integrity enabled: {sec_stats['enable_integrity']}")
    print(f"Rate limiting enabled: {sec_stats['enable_rate_limit']}")
    print()


def example_memory_bounded():
    """Example 5: Memory-bounded caching."""
    print("="*80)
    print("Example 5: Memory-Bounded Caching")
    print("="*80)
    
    # Cache limited by memory, not entry count
    cache = MemoryBoundedLRUCache(
        capacity=10000,           # Fallback limit
        memory_budget_mb=100.0    # Primary: 100MB limit
    )
    
    # Store variable-size objects
    large_data = {'data': 'x' * 10000}  # ~10KB
    cache.put('large_obj', large_data)
    
    stats = cache.get_stats()
    print(f"Memory used: {stats.get('memory_used_mb', 0):.2f}MB")
    print(f"Memory budget: {cache.memory_budget_mb}MB")
    print()


def example_read_through():
    """Example 6: Read-through caching."""
    print("="*80)
    print("Example 6: Read-Through Caching")
    print("="*80)
    
    # Simulated database
    fake_db = {
        'user:1': {'name': 'Alice', 'email': 'alice@example.com'},
        'user:2': {'name': 'Bob', 'email': 'bob@example.com'},
    }
    
    def load_from_db(key):
        """Loader function."""
        print(f"  Loading {key} from database...")
        return fake_db.get(key)
    
    cache = ReadThroughCache(
        capacity=100,
        loader=load_from_db
    )
    
    # First access - loads from DB
    print("First access to user:1...")
    user1 = cache.get('user:1')
    print(f"Got: {user1}")
    
    # Second access - from cache
    print("Second access to user:1...")
    user1_cached = cache.get('user:1')
    print(f"Got: {user1_cached} (from cache)")
    
    stats = cache.get_stats()
    print(f"Loader calls: {stats['loader_calls']}")
    print()


def example_tagged_cache():
    """Example 7: Tag-based invalidation."""
    print("="*80)
    print("Example 7: Tag-Based Cache Invalidation")
    print("="*80)
    
    cache = TaggedCache(capacity=1000)
    
    # Tag entries by type
    cache.put('user:1', {'name': 'Alice'}, tags=['user', 'active'])
    cache.put('user:2', {'name': 'Bob'}, tags=['user', 'inactive'])
    cache.put('product:1', {'name': 'Widget'}, tags=['product'])
    cache.put('product:2', {'name': 'Gadget'}, tags=['product', 'featured'])
    
    print(f"Total entries: {cache.size()}")
    print(f"Tags in cache: {cache.get_all_tags()}")
    
    # Get keys by tag
    user_keys = cache.get_keys_by_tag('user')
    print(f"User keys: {user_keys}")
    
    # Invalidate all users
    invalidated = cache.invalidate_by_tag('user')
    print(f"Invalidated {invalidated} user entries")
    print(f"Remaining entries: {cache.size()}")
    
    # Product entries still cached
    product = cache.get('product:1')
    print(f"Product still cached: {product}")
    print()


def main():
    """Run all examples."""
    example_1_basic_lru()
    example_2_lfu_optimized()
    example_3_ttl_cache()
    example_4_context_manager()
    example_5_statistics()
    example_decorator_basic()
    example_decorator_advanced()
    
    # Async example
    asyncio.run(example_async_caching())
    
    example_secure_cache()
    example_memory_bounded()
    example_read_through()
    example_tagged_cache()
    
    print("="*80)
    print("✅ All caching examples completed successfully!")
    print("="*80)


# Placeholder functions for completeness
def example_4_context_manager():
    """Basic context manager example."""
    from exonware.xwsystem.caching.lru_cache import LRUCache
    
    print("="*80)
    print("Example 4: Context Manager")
    print("="*80)
    with LRUCache(capacity=10) as cache:
        cache.put('key', 'value')
        print(f"Value: {cache.get('key')}")
    print()

def example_5_statistics():
    """Basic statistics example."""
    from exonware.xwsystem.caching.lru_cache import LRUCache
    
    print("="*80)
    print("Example 5: Statistics")
    print("="*80)
    cache = LRUCache(capacity=10)
    cache.put('key1', 'value1')
    cache.get('key1')
    stats = cache.get_stats()
    print(f"Stats: {stats}")
    print()


if __name__ == "__main__":
    main()
