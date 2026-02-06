# Python Bindings Guide for Rust xwsystem

**Company:** eXonware.com  
**Author:** Eng. Muhammad AlShehri  
**Email:** connect@exonware.com  
**Version:** 1.0.0

## Overview

This guide explains how to use the Python bindings for the Rust implementation of xwsystem caching modules. The Rust implementation provides significant performance improvements over the pure Python version.

## Installation

### Prerequisites

- Rust toolchain (install from https://rustup.rs/)
- Python 3.12+
- maturin (for building Python packages)

### Building and Installing

```bash
# Navigate to Rust directory
cd xwsystem/rust

# Build with Python feature
cargo build --release --features python

# Install Python bindings
maturin develop --features python

# Or install as package
pip install -e . --features python
```

### Verification

```python
# Test import
from exonware.rust.xwsystem.caching import LRUCache, LFUCache, TTLCache
print("Rust bindings installed successfully!")
```

## Usage

### Basic Usage

The Rust bindings provide the same API as the Python implementation:

```python
from exonware.rust.xwsystem.caching import LRUCache

# Create cache
cache = LRUCache(capacity=1000)

# Put values
cache.put("key1", {"value": 1, "data": "test"})
cache.put("key2", {"value": 2, "data": "test2"})

# Get values
value = cache.get("key1", None)
print(value)  # {'value': 1, 'data': 'test'}

# Check size
print(cache.size())  # 2

# Delete
cache.delete("key1")

# Clear
cache.clear()
```

### Available Cache Types

#### LRUCache

```python
from exonware.rust.xwsystem.caching import LRUCache

cache = LRUCache(capacity=1000, ttl=None, name="my_cache")
```

#### LFUCache

```python
from exonware.rust.xwsystem.caching import LFUCache

cache = LFUCache(capacity=1000, name="my_cache")
```

#### TTLCache

```python
from exonware.rust.xwsystem.caching import TTLCache

cache = TTLCache(
    capacity=1000,
    default_ttl=3600.0,  # 1 hour
    cleanup_interval=60.0,
    name="my_cache"
)
```

#### OptimizedLFUCache

```python
from exonware.rust.xwsystem.caching import OptimizedLFUCache

cache = OptimizedLFUCache(capacity=1000, name="my_cache")
```

#### MemoryBoundedLRUCache

```python
from exonware.rust.xwsystem.caching import MemoryBoundedLRUCache

cache = MemoryBoundedLRUCache(
    capacity=1000,
    max_memory_mb=100.0,
    ttl=None,
    name="my_cache"
)
```

#### SecureLRUCache

```python
from exonware.rust.xwsystem.caching import SecureLRUCache

cache = SecureLRUCache(
    capacity=1000,
    ttl=None,
    name="my_cache",
    enable_integrity=True,
    enable_rate_limit=True,
    max_ops_per_second=1000
)
```

## API Compatibility

The Rust bindings maintain 100% API compatibility with the Python implementation:

### Dictionary-like Interface

```python
cache = LRUCache(capacity=100)

# Dictionary syntax
cache["key"] = {"value": 1}
value = cache["key"]
del cache["key"]

# Check membership
if "key" in cache:
    print("Key exists")

# Length
print(len(cache))
```

### Context Manager

```python
with LRUCache(capacity=100) as cache:
    cache.put("key", {"value": 1})
    # Cache automatically cleaned up on exit (if configured)
```

### Statistics

```python
cache = LRUCache(capacity=100)

# Get statistics
stats = cache.get_stats()
print(stats)
# {
#     "hits": 10,
#     "misses": 5,
#     "evictions": 2,
#     "size": 98,
#     "capacity": 100
# }

# Reset statistics
cache.reset_stats()
```

## Performance Comparison

### Benchmark Results

Typical performance improvements:

| Operation | Python (ms) | Rust (ms) | Speedup |
|-----------|-------------|-----------|---------|
| Get       | 0.05        | 0.01      | 5x      |
| Put       | 0.08        | 0.02      | 4x      |
| Size      | 0.001       | 0.0005    | 2x      |

### Running Benchmarks

```bash
cd xwsystem/benchmarks/caching/python_vs_rust
python benchmark_comparison.py
```

## Migration from Python

Migration is straightforward - just change the import:

```python
# Before (Python)
from exonware.xwsystem.caching import LRUCache

# After (Rust)
from exonware.rust.xwsystem.caching import LRUCache

# API is identical, no code changes needed!
```

## Advanced Features

### Thread Safety

Rust caches are thread-safe by default:

```python
import threading
from exonware.rust.xwsystem.caching import LRUCache

cache = LRUCache(capacity=1000)

def worker(thread_id):
    for i in range(100):
        cache.put(f"key_{thread_id}_{i}", {"value": i})

# Multiple threads
threads = [threading.Thread(target=worker, args=(i,)) for i in range(10)]
for t in threads:
    t.start()
for t in threads:
    t.join()
```

### Error Handling

```python
from exonware.rust.xwsystem.caching import LRUCache

cache = LRUCache(capacity=10)

try:
    # Operations that might fail
    cache.put("key", {"value": 1})
    value = cache.get("key", None)
except Exception as e:
    print(f"Error: {e}")
```

## Troubleshooting

### Import Errors

**Problem:** `ModuleNotFoundError: No module named 'exonware.rust'`

**Solution:**
```bash
cd xwsystem/rust
maturin develop --features python
```

### Build Errors

**Problem:** `cargo build` fails

**Solution:**
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release --features python
```

### Performance Issues

**Problem:** Rust version not faster than Python

**Solution:**
- Ensure release build: `cargo build --release`
- Check CPU frequency scaling
- Verify correct import (should be `exonware.rust.xwsystem.caching`)

## Best Practices

1. **Use Rust for high-performance scenarios**
   - Large caches (>10,000 items)
   - High-frequency operations
   - Production workloads

2. **Use Python for development**
   - Easier debugging
   - Faster iteration
   - Better error messages

3. **Benchmark before switching**
   - Measure actual performance
   - Consider overhead of Python-Rust boundary
   - Test with your specific workload

4. **Monitor memory usage**
   - Rust caches may use less memory
   - Check with `cache.get_stats()`

## API Reference

### Common Methods

All cache types support:

- `get(key, default=None)` - Get value
- `put(key, value)` - Put value
- `delete(key)` - Delete key
- `clear()` - Clear all entries
- `size()` - Get cache size
- `is_full()` - Check if cache is full
- `get_stats()` - Get statistics
- `keys()` - Get all keys
- `values()` - Get all values
- `items()` - Get all key-value pairs

### Dictionary Interface

- `cache[key]` - Get item
- `cache[key] = value` - Set item
- `del cache[key]` - Delete item
- `key in cache` - Check membership
- `len(cache)` - Get size

## Examples

See `xwsystem/benchmarks/caching/python_vs_rust/` for complete examples.

## Support

For issues or questions:
- **Email:** connect@exonware.com
- **Documentation:** [Complete Documentation](../../docs/)
- **Examples:** [Benchmark Examples](../../benchmarks/caching/python_vs_rust/)

---

**Last Updated:** January 2025
