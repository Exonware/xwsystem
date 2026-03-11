# XWSystem Caching Examples

Comprehensive examples for the xwsystem caching module.

## Files

### `basic_usage.py`
Basic caching patterns and common use cases:
- Example 1: Basic LRU caching
- Example 2: Optimized LFU caching (O(1) performance)
- Example 3: TTL caching with expiration
- Example 4: Context manager usage
- Example 5: Statistics and monitoring

### `advanced_usage.py`
Advanced features and patterns:
- Decorator usage (@xwcached, @xw_async_cached)
- Advanced decorator with hooks
- Async caching
- Secure caching
- Memory-bounded caching
- Read-through caching
- Tag-based invalidation

## Running Examples

```bash
cd examples/caching

# Run basic examples
python basic_usage.py

# Run advanced examples
python advanced_usage.py
```

## Quick Examples

### Basic LRU Caching

```python
from exonware.xwsystem.caching import LRUCache

cache = LRUCache(capacity=100)
cache.put('key', 'value')
value = cache.get('key')
```

### Function Caching

```python
from exonware.xwsystem.caching import xwcached

@xwcached(ttl=300)
def expensive_function(x):
    return x * 2

result = expensive_function(5)  # Cached for 5 minutes
```

### Secure Caching

```python
from exonware.xwsystem.caching import SecureLRUCache

cache = SecureLRUCache(
    capacity=1000,
    enable_integrity=True,
    enable_rate_limit=True
)
```

### Tag-Based Invalidation

```python
from exonware.xwsystem.caching import TaggedCache

cache = TaggedCache(capacity=1000)
cache.put('user:1', data, tags=['user', 'active'])
cache.put('user:2', data, tags=['user', 'inactive'])

# Invalidate all user entries
cache.invalidate_by_tag('user')
```

## Performance Tips

1. **Use OptimizedLFUCache** for better performance (O(1) eviction)
2. **Use MemoryBoundedCache** for large variable-size objects
3. **Use TTLCache** for time-sensitive data
4. **Use SecureCache** in production for security
5. **Use TaggedCache** for bulk invalidation patterns
6. **Use WriteBehindCache** for high write throughput
7. **Use BloomFilterCache** for high-miss-rate scenarios

## Integration Examples

### With Web Frameworks
```python
# Flask/FastAPI
@app.get("/users/{user_id}")
@xwcached(ttl=300)
def get_user(user_id):
    return db.query_user(user_id)
```

### With Async Code
```python
from exonware.xwsystem.caching import AsyncLRUCache, xw_async_cached

@xw_async_cached()
async def fetch_api_data(endpoint):
    async with httpx.AsyncClient() as client:
        return await client.get(endpoint)
```

### With Database
```python
cache = ReadThroughCache(
    capacity=1000,
    loader=lambda key: db.get(key)
)

# Auto-loads from DB on miss
user = cache.get('user:123')
```

## See Also

- [Caching Documentation](../../docs/caching.md)
- [Performance Guide](../../docs/performance.md)
- [Security Best Practices](../../docs/security.md)

---

**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Email:** connect@exonware.com  
**Version:** 0.0.1.388

