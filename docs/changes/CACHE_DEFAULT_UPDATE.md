# Cache Default Update: FunctoolsLRUCache

**Date:** January 2025  
**Change:** Updated default cache type from `LFUCache` to `FunctoolsLRUCache`

## Summary

Based on comprehensive benchmarking results documented in `benchmarks/caching/RUST_VS_PY_JOURNEY.md`, `FunctoolsLRUCache` has been set as the default cache type across all eXonware codebases.

**Performance Metrics:**
- **MIXED Operations:** 3,333 ops/sec (fastest Python cache)
- **GET Operations:** 33,335 ops/sec (fastest Python cache)
- **PUT Operations:** 25,009 ops/sec (excellent performance)

## What Changed

### 1. Factory Default (`xwsystem/src/exonware/xwsystem/caching/factory.py`)
- Default cache type changed from `'lfu'` to `'functools_lru'`
- `FunctoolsLRUCache` added to cache type registry
- All fallback defaults updated to use `FunctoolsLRUCache`

### 2. Documentation (`xwsystem/docs/CACHE_CONFIGURATION.md`)
- Updated all references to reflect new default
- Updated performance recommendations
- Updated examples and migration guide

## How to Use

### Using the Default (FunctoolsLRUCache)

```python
from exonware.xwsystem.caching import create_cache

# Automatically uses FunctoolsLRUCache (fastest Python cache)
cache = create_cache(capacity=1000)
```

### Override via Environment Variable

```bash
# Change default globally
export XWSYSTEM_CACHE_TYPE=lfu

# Change default for specific namespace
export XWSYSTEM_CACHE_TYPE_XWSTORAGE=ttl
```

### Override via Parameter

```python
from exonware.xwsystem.caching import create_cache

# Use different cache type for this instance
cache = create_cache(capacity=1000, cache_type='lfu')
cache = create_cache(capacity=1000, cache_type='lru')
cache = create_cache(capacity=1000, cache_type='ttl', ttl=3600)
```

### Override via Settings

```python
from exonware.xwsystem.caching import CacheFactory

# Change default globally
CacheFactory.set_setting('cache.type', 'lfu')

# Change default for specific namespace
CacheFactory.set_setting('cache.type.xwstorage', 'ttl')
```

## Available Cache Types

| Cache Type | Identifier | Performance | Best For |
|------------|-----------|-------------|----------|
| **FunctoolsLRU** (Default) | `functools_lru` | 3,333 ops/sec MIXED, 33,335 ops/sec GET | Maximum performance |
| LFU | `lfu` | 2,219 ops/sec MIXED | High-frequency access |
| LRU | `lru` | 1,661 ops/sec MIXED | Temporal locality |
| TTL | `ttl` | 1,250 ops/sec MIXED | Time-sensitive data |
| Optimized LFU | `optimized_lfu` | 1,537 ops/sec MIXED | Large caches |
| Memory Bounded LRU | `memory_bounded_lru` | Variable | Memory constraints |
| Memory Bounded LFU | `memory_bounded_lfu` | Variable | Memory constraints, frequency-based |
| Secure LRU | `secure_lru` | Lower (security overhead) | Security-sensitive |
| Secure LFU | `secure_lfu` | Lower (security overhead) | Security-sensitive, high-frequency |
| Secure TTL | `secure_ttl` | Lower (security overhead) | Security-sensitive, time-based |

## Impact on Existing Code

**No code changes required!** All existing code using `create_cache()` will automatically use the new default (`FunctoolsLRUCache`). The change is backward compatible.

### Codebases Affected

All codebases that use `create_cache()` from `xwsystem.caching`:
- ✅ xwaction
- ✅ xwai
- ✅ xwapi
- ✅ xwauth
- ✅ xwbase
- ✅ xwbots
- ✅ xwdata
- ✅ xwentity
- ✅ xwformats
- ✅ xwjson
- ✅ xwlazy
- ✅ xwnode
- ✅ xwquery
- ✅ xwschema
- ✅ xwstorage
- ✅ xwsyntax
- ✅ xwsystem

## Configuration Priority

The cache factory uses the following priority order:

1. **Per-instance override** (`cache_type` parameter)
2. **Namespace-specific setting** (`cache.type.{namespace}`)
3. **Namespace-specific environment variable** (`XWSYSTEM_CACHE_TYPE_{NAMESPACE}`)
4. **Global setting** (`cache.type` or `xwsystem.cache.type`)
5. **Global environment variable** (`XWSYSTEM_CACHE_TYPE`)
6. **Default** (`functools_lru` - fastest Python cache)

## Testing

To verify the default is working:

```python
from exonware.xwsystem.caching import create_cache

cache = create_cache(capacity=100)
print(type(cache).__name__)  # Should print: FunctoolsLRUCache
```

## References

- **Benchmark Results:** `xwsystem/benchmarks/caching/RUST_VS_PY_JOURNEY.md`
- **Configuration Guide:** `xwsystem/docs/CACHE_CONFIGURATION.md`
- **Factory Implementation:** `xwsystem/src/exonware/xwsystem/caching/factory.py`

