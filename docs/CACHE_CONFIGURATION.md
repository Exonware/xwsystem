# Cache Configuration Guide

**Company:** eXonware.com  
**Author:** Eng. Muhammad AlShehri  
**Email:** connect@exonware.com  
**Version:** 0.1.0  
**Date:** January 2025

## Overview

The eXonware caching system now supports **configurable cache engines** that can be selected via settings or environment variables. This allows you to choose the best cache type for your specific use case while maintaining optimal performance.

**Default:** FunctoolsLRUCache (fastest Python cache - 3,333 ops/sec in MIXED operations, 33,335 ops/sec in GET operations)

## Quick Start

### Using Default (FunctoolsLRU - Fastest Python Cache)

```python
from exonware.xwsystem.caching import create_cache

# Uses FunctoolsLRUCache by default (fastest Python cache)
cache = create_cache(capacity=1000)
```

### Override via Environment Variable

```bash
# Set globally
export XWSYSTEM_CACHE_TYPE=lru

# Set for specific namespace
export XWSYSTEM_CACHE_TYPE_XWSTORAGE=ttl
```

### Override Programmatically

```python
from exonware.xwsystem.caching import create_cache

# Use LRU instead
cache = create_cache(capacity=1000, cache_type='lru')

# Use TTL with expiration
cache = create_cache(capacity=1000, cache_type='ttl', ttl=3600)
```

### Namespace-Specific Configuration

```python
from exonware.xwsystem.caching import CacheFactory

# Set namespace-specific cache type
CacheFactory.set_setting('cache.type.xwstorage', 'lru')

# Now all xwstorage caches will use LRU
cache = create_cache(capacity=1000, namespace='xwstorage')
```

## Configuration Methods

### 1. Environment Variables

**Global Configuration:**
```bash
export XWSYSTEM_CACHE_TYPE=functools_lru  # Options: functools_lru, lru, lfu, ttl, optimized_lfu, etc.
```

**Namespace-Specific Configuration:**
```bash
export XWSYSTEM_CACHE_TYPE_XWSTORAGE=lru
export XWSYSTEM_CACHE_TYPE_XWDATA=ttl
export XWSYSTEM_CACHE_TYPE_XWJSON=optimized_lfu
```

### 2. Programmatic Settings

```python
from exonware.xwsystem.caching import CacheFactory

# Global setting
CacheFactory.set_setting('cache.type', 'lru')

# Namespace-specific setting
CacheFactory.set_setting('cache.type.xwstorage', 'ttl')
CacheFactory.set_setting('cache.type.xwdata', 'lru')

# TTL setting (for TTL caches)
CacheFactory.set_setting('cache.ttl', 3600)  # 1 hour

# Memory budget (for memory-bounded caches)
CacheFactory.set_setting('cache.memory_budget_mb', 200.0)
```

### 3. Per-Instance Override

```python
from exonware.xwsystem.caching import create_cache

# Override for this specific cache instance
cache = create_cache(
    capacity=1000,
    cache_type='lru',  # Override any global/namespace settings
    namespace='xwstorage',
    name='MyCustomCache'
)
```

## Available Cache Types

| Cache Type | Identifier | Description | Best For |
|------------|-----------|-------------|----------|
| **FunctoolsLRU** (Default) | `functools_lru` | Standard library LRU - **Fastest Python cache** (3,333 ops/sec MIXED, 33,335 ops/sec GET) | General purpose, maximum performance |
| LFU | `lfu` | Least Frequently Used | High-frequency access patterns |
| LRU | `lru` | Least Recently Used | Temporal locality patterns |
| TTL | `ttl` | Time-To-Live expiration | Time-sensitive data |
| Optimized LFU | `optimized_lfu` | O(1) LFU eviction | Large caches with frequent eviction |
| Memory Bounded LRU | `memory_bounded_lru` | LRU with memory limits | Variable-size objects |
| Memory Bounded LFU | `memory_bounded_lfu` | LFU with memory limits | Variable-size objects, frequency-based |
| Secure LRU | `secure_lru` | LRU with rate limiting | Security-sensitive data |
| Secure LFU | `secure_lfu` | LFU with rate limiting | Security-sensitive, high-frequency |
| Secure TTL | `secure_ttl` | TTL with rate limiting | Security-sensitive, time-based |

## Configuration Priority

The cache factory uses the following priority order:

1. **Per-instance override** (`cache_type` parameter)
2. **Namespace-specific setting** (`cache.type.{namespace}`)
3. **Namespace-specific environment variable** (`XWSYSTEM_CACHE_TYPE_{NAMESPACE}`)
4. **Global setting** (`cache.type` or `xwsystem.cache.type`)
5. **Global environment variable** (`XWSYSTEM_CACHE_TYPE`)
6. **Default** (`functools_lru` - fastest Python cache)

## Examples

### Example 1: Global LRU Configuration

```python
import os
os.environ['XWSYSTEM_CACHE_TYPE'] = 'lru'

from exonware.xwsystem.caching import create_cache

# All caches will use LRU
cache1 = create_cache(capacity=1000)
cache2 = create_cache(capacity=500)
```

### Example 2: Package-Specific Configuration

```python
from exonware.xwsystem.caching import CacheFactory

# xwstorage uses TTL
CacheFactory.set_setting('cache.type.xwstorage', 'ttl')
CacheFactory.set_setting('cache.ttl', 1800)  # 30 minutes

# xwdata uses LRU
CacheFactory.set_setting('cache.type.xwdata', 'lru')

# Now create caches
from exonware.xwsystem.caching import create_cache

storage_cache = create_cache(capacity=1000, namespace='xwstorage')  # Uses TTL
data_cache = create_cache(capacity=5000, namespace='xwdata')  # Uses LRU
```

### Example 3: Custom Cache Registration

```python
from exonware.xwsystem.caching import CacheFactory, ACache

class MyCustomCache(ACache):
    # Your custom implementation
    pass

# Register custom cache type
CacheFactory.register_cache_type('custom', MyCustomCache)

# Use it
cache = create_cache(capacity=1000, cache_type='custom')
```

## Performance Recommendations

Based on benchmark results:

- **General Purpose:** Use `functools_lru` (default) - Fastest Python cache (3,333 ops/sec MIXED, 33,335 ops/sec GET)
- **Temporal Patterns:** Use `lru` - Better for recently accessed items
- **Time-Sensitive:** Use `ttl` - Automatic expiration
- **Large Caches:** Use `optimized_lfu` - O(1) eviction for better scalability
- **Memory Constraints:** Use `memory_bounded_lfu` - Memory-aware eviction
- **Security:** Use `secure_lfu` - Rate limiting and security features

## Migration Guide

### Before (Hardcoded)

```python
from exonware.xwsystem.caching import LFUCache

cache = LFUCache(capacity=1000)
```

### After (Configurable)

```python
from exonware.xwsystem.caching import create_cache

# Default (FunctoolsLRU - fastest Python cache)
cache = create_cache(capacity=1000)

# Or with namespace
cache = create_cache(capacity=1000, namespace='xwstorage')
```

## API Reference

### `create_cache()`

```python
def create_cache(
    capacity: int = 128,
    cache_type: Optional[str] = None,
    namespace: Optional[str] = None,
    name: Optional[str] = None,
    **kwargs
) -> ACache
```

**Parameters:**
- `capacity`: Cache capacity (default: 128)
- `cache_type`: Override cache type (optional)
- `namespace`: Namespace for namespace-specific configuration (optional)
- `name`: Cache name for debugging (optional)
- `**kwargs`: Additional cache-specific parameters (e.g., `ttl`, `memory_budget_mb`)

**Returns:** Cache instance

### `CacheFactory`

```python
class CacheFactory:
    @classmethod
    def set_setting(cls, key: str, value: Any) -> None
    
    @classmethod
    def get_setting(cls, key: str, default: Any = None) -> Any
    
    @classmethod
    def get_cache_type(cls, namespace: Optional[str] = None) -> str
    
    @classmethod
    def create(cls, capacity: int = 128, cache_type: Optional[str] = None, ...) -> ACache
    
    @classmethod
    def register_cache_type(cls, cache_type: str, cache_class: Type[ACache]) -> None
    
    @classmethod
    def get_available_types(cls) -> list[str]
```

## Troubleshooting

### Check Current Configuration

```python
from exonware.xwsystem.caching import CacheFactory

# Get current cache type for namespace
cache_type = CacheFactory.get_cache_type('xwstorage')
print(f"xwstorage uses: {cache_type}")

# List all available types
types = CacheFactory.get_available_types()
print(f"Available types: {types}")
```

### Debug Cache Creation

```python
from exonware.xwsystem.caching import create_cache

# Enable debug logging to see which cache type is created
cache = create_cache(
    capacity=1000,
    namespace='xwstorage',
    name='DebugCache'  # Will appear in logs
)
```

---

*For more information, see the [RUST_vs_PY_JOURNEY.md](../benchmarks/caching/RUST_vs_PY_JOURNEY.md) for performance benchmarks.*

