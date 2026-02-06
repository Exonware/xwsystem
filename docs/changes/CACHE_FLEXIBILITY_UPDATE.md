# Cache Flexibility Update - All Modules

**Date:** January 2025  
**Change:** Updated all production code to use flexible `create_cache()` instead of direct cache instantiation

## Summary

All production code across eXonware codebases has been updated to use the flexible `create_cache()` function instead of directly instantiating cache classes. This ensures all caching can be configured via environment variables or settings, and will automatically use the fastest default cache (FunctoolsLRUCache).

## Files Updated

### 1. xwnode/src/exonware/xwnode/common/caching/adapters.py
**Changes:**
- Updated `LRUCacheAdapter` to use `create_cache()` instead of direct `LRUCache()` instantiation
- Updated `LFUCacheAdapter` to use `create_cache()` instead of direct `LFUCache()` instantiation
- Updated `TTLCacheAdapter` to use `create_cache(cache_type='ttl', ...)` for TTL functionality

**Impact:** All xwnode cache adapters now use the flexible factory pattern and respect configuration.

### 2. xwsystem/src/exonware/xwsystem/io/serialization/base.py
**Changes:**
- Updated `_file_serializer_cache` to use `create_cache()` instead of direct `LRUCache()` instantiation
- Added namespace `'xwsystem.serialization'` for better organization

**Impact:** File serializer cache now uses the fastest default cache and can be configured.

### 3. xwsystem/src/exonware/xwsystem/security/path_validator.py
**Changes:**
- Updated `PathValidator._cache` to use `create_cache()` instead of direct `LRUCache()` instantiation
- Added namespace `'xwsystem.security'` for better organization

**Impact:** Path validator cache now uses the fastest default cache and can be configured.

### 4. xwsystem/src/exonware/xwsystem/caching/two_tier_cache.py
**Changes:**
- Updated `TwoTierCache.memory_cache` to use `create_cache()` instead of direct `LRUCache()` instantiation
- Added namespace support for better organization

**Impact:** Two-tier cache memory tier now uses the fastest default cache and can be configured.

### 5. xwsystem/src/exonware/xwsystem/caching/decorators.py
**Changes:**
- Updated `xwcached()` decorator default cache to use `create_cache()` instead of direct `LRUCache()` instantiation
- Added namespace `'xwsystem.decorators'` for better organization

**Impact:** All decorated functions now use the fastest default cache and can be configured.

### 6. xwdata/src/exonware/xwdata/data/engine.py
**Changes:**
- Updated comment to reflect new default (FunctoolsLRU instead of LFU)
- Already using `create_cache()` correctly

**Impact:** Documentation updated to reflect current default.

## Codebases Already Using Flexible Caching

These codebases were already using `create_cache()` correctly:
- ✅ **xwdata** - Uses `create_cache()` with namespace
- ✅ **xwstorage** - All cache instances use `create_cache()` with namespaces
- ✅ **xwjson** - Uses `create_cache()` with namespace
- ✅ **xwaction** - No caching usage found
- ✅ **xwai** - No caching usage found
- ✅ **xwapi** - No caching usage found
- ✅ **xwauth** - No caching usage found
- ✅ **xwbase** - No caching usage found
- ✅ **xwbots** - No caching usage found
- ✅ **xwentity** - No caching usage found
- ✅ **xwformats** - No caching usage found
- ✅ **xwquery** - No caching usage found
- ✅ **xwschema** - No caching usage found
- ✅ **xwsyntax** - No caching usage found

## Special Cases

### xwlazy
**Status:** Uses its own caching strategies (not xwsystem)
- xwlazy has its own `LFUCache`, `LRUCache`, and `TTLCache` implementations
- These are separate from xwsystem's caching system
- No changes needed - this is by design

### Example Files
**Status:** Example files may use direct instantiation for demonstration purposes
- Example files in `examples/` directories are for demonstration
- They may show direct instantiation for clarity
- Production code uses flexible `create_cache()`

## Benefits

1. **Configuration Flexibility:** All caches can now be configured via:
   - Environment variables: `XWSYSTEM_CACHE_TYPE=functools_lru`
   - Settings: `CacheFactory.set_setting('cache.type', 'functools_lru')`
   - Per-instance: `create_cache(cache_type='lfu')`

2. **Automatic Performance:** All caches automatically use the fastest default (FunctoolsLRUCache) unless overridden

3. **Namespace Support:** All caches can be configured per namespace for fine-grained control

4. **Consistency:** All production code follows the same pattern for cache creation

## Verification

To verify all caches are using the flexible approach:

```python
# Check that create_cache() is used
grep -r "create_cache" src/

# Check for direct instantiation (should only find in tests/examples)
grep -r "LRUCache(" src/ | grep -v test | grep -v example
grep -r "LFUCache(" src/ | grep -v test | grep -v example
grep -r "TTLCache(" src/ | grep -v test | grep -v example
```

## Testing

All changes maintain backward compatibility:
- Existing code continues to work
- Default behavior is improved (faster cache)
- Configuration is optional (works with defaults)

## References

- **Cache Configuration Guide:** `xwsystem/docs/CACHE_CONFIGURATION.md`
- **Default Update:** `xwsystem/CACHE_DEFAULT_UPDATE.md`
- **Factory Implementation:** `xwsystem/src/exonware/xwsystem/caching/factory.py`

