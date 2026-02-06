#!/usr/bin/env python3
#exonware/xwsystem/tests/3.advance/benchmarks/legacy/benchmarks/caching/benchmark_rust.py
"""
Caching benchmark comparing Rust bindings vs Python implementation.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.1
"""

import time
import sys
from pathlib import Path
from typing import Any, Callable
from dataclasses import dataclass
import statistics

from exonware.xwsystem.caching import (
        LRUCache as PythonLRUCache,
        LFUCache as PythonLFUCache,
        TTLCache as PythonTTLCache,
        OptimizedLFUCache as PythonOptimizedLFUCache,
        MemoryBoundedLRUCache as PythonMemoryBoundedLRUCache,
        SecureLRUCache as PythonSecureLRUCache,
)

# Try to import Rust implementations
# Add rust/python to path if needed
import sys
from pathlib import Path
rust_python_path = Path(__file__).parent.parent.parent / "rust" / "python"
if rust_python_path.exists() and str(rust_python_path) not in sys.path:
    sys.path.insert(0, str(rust_python_path))

try:
    from exonware.rust.xwsystem.caching import (
            LRUCache as RUSTLRUCache,
            LFUCache as RUSTLFUCache,
            TTLCache as RUSTTTLCache,
            OptimizedLFUCache as RUSTOptimizedLFUCache,
            MemoryBoundedLRUCache as RUSTMemoryBoundedLRUCache,
            SecureLRUCache as RUSTSecureLRUCache,
    )
    HAS_RUST = True
except ImportError as e:
    HAS_RUST = False
    RUSTLRUCache = None
    RUSTLFUCache = None
    RUSTTTLCache = None
    RUSTOptimizedLFUCache = None
    RUSTMemoryBoundedLRUCache = None
    RUSTSecureLRUCache = None
    print(f"Warning: Rust implementations not available: {e}")

# Try to import external Rust caches
try:
    from exonware.rust.xwsystem.caching import (
            MokaCache as RUSTMokaCache,
            MokaTTLCache as RUSTMokaTTLCache,
            MokaWeightedCache as RUSTMokaWeightedCache,
            QuickCache as RUSTQuickCache,
            QuickCacheTTL as RUSTQuickCacheTTL,
            DashMapCache as RUSTDashMapCache,
            DashMapTTLCache as RUSTDashMapTTLCache,
    )
    HAS_EXTERNAL_RUST = True
except ImportError as e:
    HAS_EXTERNAL_RUST = False
    RUSTMokaCache = None
    RUSTMokaTTLCache = None
    RUSTMokaWeightedCache = None
    RUSTQuickCache = None
    RUSTQuickCacheTTL = None
    RUSTDashMapCache = None
    RUSTDashMapTTLCache = None
    print(f"Warning: External Rust caches not available: {e}")

# Import external Python caches - use new proper implementations first
try:
    from exonware.xwsystem.caching.external_caching_python import (
            CacheboxCache,
            FunctoolsLRUCache,
            CachetoolsLRUCache,
            CachetoolsLFUCache,
            CachetoolsTTLCache,
            CachetoolsRRCache,
            HAS_CACHEBOX,
            HAS_CACHETOOLS,
    )
    # Map to old names for backward compatibility
    CacheboxWrapper = CacheboxCache
    FunctoolsLRUWrapper = FunctoolsLRUCache
    CachetoolsLRUWrapper = CachetoolsLRUCache
    CachetoolsLFUWrapper = CachetoolsLFUCache
    CachetoolsTTLWrapper = CachetoolsTTLCache
    CachetoolsRRWrapper = CachetoolsRRCache
except ImportError:
    # Fallback to old wrappers if new implementations not available
    try:
        from .external_python_caches import (
                CacheboxWrapper,
                FunctoolsLRUWrapper,
                CachetoolsLRUWrapper,
                CachetoolsLFUWrapper,
                CachetoolsTTLWrapper,
                CachetoolsRRWrapper,
                HAS_CACHEBOX,
                HAS_CACHETOOLS,
        )
    except ImportError:
        # Fallback if running from different directory
        import sys
        from pathlib import Path
        sys.path.insert(0, str(Path(__file__).parent))
        from external_python_caches import (
                CacheboxWrapper,
                FunctoolsLRUWrapper,
                CachetoolsLRUWrapper,
                CachetoolsLFUWrapper,
                CachetoolsTTLWrapper,
                CachetoolsRRWrapper,
                HAS_CACHEBOX,
                HAS_CACHETOOLS,
        )

# Import other external caches (diskcache, cacheout, pylru) from old module
try:
    from .external_python_caches import (
            DiskcacheWrapper,
            CacheoutLRUWrapper,
            CacheoutLFUWrapper,
            CacheoutTTLWrapper,
            CacheoutFIFOWrapper,
            PylruWrapper,
            HAS_DISKCACHE,
            HAS_CACHEOUT,
            HAS_PYLRU,
    )
except ImportError:
    # Fallback if running from different directory
    try:
        import sys
        from pathlib import Path
        sys.path.insert(0, str(Path(__file__).parent))
        from external_python_caches import (
                DiskcacheWrapper,
                CacheoutLRUWrapper,
                CacheoutLFUWrapper,
                CacheoutTTLWrapper,
                CacheoutFIFOWrapper,
                PylruWrapper,
                HAS_DISKCACHE,
                HAS_CACHEOUT,
                HAS_PYLRU,
        )
    except ImportError:
        # These are optional
        DiskcacheWrapper = None
        CacheoutLRUWrapper = None
        CacheoutLFUWrapper = None
        CacheoutTTLWrapper = None
        CacheoutFIFOWrapper = None
        PylruWrapper = None
        HAS_DISKCACHE = False
        HAS_CACHEOUT = False
        HAS_PYLRU = False


@dataclass
class BenchmarkResult:
    """Result of a single benchmark."""
    name: str
    operations_per_second: float
    latency_p50_ms: float
    latency_p95_ms: float
    latency_p99_ms: float
    duration_seconds: float
    success: bool
    error: str = ""


def benchmark_operation(name: str, operation: Callable, iterations: int = 10000) -> BenchmarkResult:
    """Benchmark a single operation."""
    latencies = []
    
    try:
        start_time = time.time()
        
        for _ in range(iterations):
            op_start = time.perf_counter()
            operation()
            op_end = time.perf_counter()
            latencies.append((op_end - op_start) * 1000)  # Convert to ms
        
        end_time = time.time()
        duration = end_time - start_time
        
        # Calculate statistics
        ops_per_sec = iterations / duration if duration > 0 else 0
        latencies.sort()
        
        p50 = latencies[len(latencies) // 2] if latencies else 0
        p95 = latencies[int(len(latencies) * 0.95)] if latencies else 0
        p99 = latencies[int(len(latencies) * 0.99)] if latencies else 0
        
        return BenchmarkResult(
            name=name,
            operations_per_second=ops_per_sec,
            latency_p50_ms=p50,
            latency_p95_ms=p95,
            latency_p99_ms=p99,
            duration_seconds=duration,
            success=True
        )
    except Exception as e:
        return BenchmarkResult(
            name=name,
            operations_per_second=0.0,
            latency_p50_ms=0.0,
            latency_p95_ms=0.0,
            latency_p99_ms=0.0,
            duration_seconds=0.0,
            success=False,
            error=str(e)
        )


def benchmark_cache_put(cache, num_items: int = 1000):
    """Benchmark cache put operations."""
    # Check if this is DashMapTTLCache which has optional ttl_seconds parameter
    cache_type = type(cache).__name__
    for i in range(num_items):
        if 'DashMapTTL' in cache_type:
            # DashMapTTLCache.put(key, value, ttl_seconds=None)
            cache.put(f"key_{i}", {"value": i, "data": f"test_data_{i}"}, None)
        else:
            cache.put(f"key_{i}", {"value": i, "data": f"test_data_{i}"})


def benchmark_cache_get(cache, num_items: int = 1000):
    """Benchmark cache get operations."""
    results = []
    for i in range(num_items):
        val = cache.get(f"key_{i % num_items}")
        results.append(val)
    return results


def benchmark_cache_mixed(cache, num_operations: int = 10000):
    """Benchmark mixed operations."""
    # Check if this is DashMapTTLCache which has optional ttl_seconds parameter
    cache_type = type(cache).__name__
    for i in range(num_operations):
        if i % 2 == 0:
            if 'DashMapTTL' in cache_type:
                # DashMapTTLCache.put(key, value, ttl_seconds=None)
                cache.put(f"key_{i % 1000}", {"value": i}, None)
            else:
                cache.put(f"key_{i % 1000}", {"value": i})
        else:
            cache.get(f"key_{i % 1000}")


def benchmark_cache_class(cache_class, cache_name, capacity=1000, python_impl=True):
    """Benchmark a specific cache class."""
    results = []
    
    try:
        # Initialize cache with appropriate parameters
        if cache_name in ['TTLCache', 'PythonTTLCache', 'RustTTLCache', 'CachetoolsTTL']:
            if 'Cachetools' in cache_name:
                cache = cache_class(capacity=capacity, ttl=60.0)
            else:
                cache = cache_class(capacity=capacity, default_ttl=60.0)
        elif cache_name in ['MemoryBoundedLRUCache', 'PythonMemoryBoundedLRUCache', 'RustMemoryBoundedLRUCache']:
            cache = cache_class(capacity=capacity, max_memory_mb=100)
        elif cache_name in ['SecureLRUCache', 'PythonSecureLRUCache', 'RustSecureLRUCache']:
            # Disable rate limiting and integrity checks for benchmarking to avoid rate limit errors
            cache = cache_class(
                capacity=capacity, 
                enable_rate_limit=False,
                enable_integrity=False,  # Disable integrity for faster benchmarking
                max_ops_per_second=1000000  # Set very high limit as backup
            )
            # Force disable rate limiting (defensive programming)
            if hasattr(cache, 'enable_rate_limit'):
                cache.enable_rate_limit = False
            if hasattr(cache, 'rate_limiter'):
                delattr(cache, 'rate_limiter')
        elif 'Moka' in cache_name:
            if 'TTL' in cache_name:
                cache = cache_class(capacity, 60.0)  # TTL variant
            elif 'Weighted' in cache_name:
                cache = cache_class(capacity)  # Weighted variant
            else:
                cache = cache_class(capacity)  # Basic variant
        elif 'QuickCache' in cache_name:
            if 'TTL' in cache_name:
                cache = cache_class(capacity, 60.0)  # TTL variant
            else:
                cache = cache_class(capacity)  # Basic variant
        elif 'DashMap' in cache_name:
            if 'TTL' in cache_name:
                cache = cache_class(capacity, 60.0)  # TTL variant
            else:
                cache = cache_class(capacity)  # Basic variant
        elif 'Cachebox' in cache_name or 'Functools' in cache_name or 'Cachetools' in cache_name:
            # External Python caches (new implementations)
            if 'TTL' in cache_name or 'CachetoolsTTL' in cache_name or 'CachetoolsTTLCache' in cache_name:
                cache = cache_class(capacity=capacity, ttl=60.0)
            else:
                cache = cache_class(capacity=capacity)
        elif 'Diskcache' in cache_name:
            cache = cache_class(capacity=capacity)
        elif 'Cacheout' in cache_name:
            if 'TTL' in cache_name:
                cache = cache_class(capacity=capacity, ttl=60.0)
            else:
                cache = cache_class(capacity=capacity)
        elif 'Pylru' in cache_name:
            cache = cache_class(capacity=capacity)
        else:
            cache = cache_class(capacity=capacity)
        
        # Warm up
        try:
            # For SecureLRUCache, ensure rate limiting is truly disabled
            if cache_name in ['SecureLRUCache', 'PythonSecureLRUCache', 'RustSecureLRUCache']:
                print(f"  DEBUG WARMUP: enable_rate_limit={cache.enable_rate_limit}, has_rate_limiter={hasattr(cache, 'rate_limiter')}")
                if hasattr(cache, 'enable_rate_limit'):
                    cache.enable_rate_limit = False
                if hasattr(cache, 'rate_limiter'):
                    delattr(cache, 'rate_limiter')
                print(f"  DEBUG WARMUP AFTER: enable_rate_limit={cache.enable_rate_limit}, has_rate_limiter={hasattr(cache, 'rate_limiter')}")
            benchmark_cache_put(cache, 100)
        except Exception as e:
            print(f"  Warning: Warm-up failed for {cache_name}: {e}")
            import traceback
            traceback.print_exc()
            return results
        
        # Benchmark PUT operations
        try:
            result = benchmark_operation(
                f"{cache_name} PUT",
                lambda: benchmark_cache_put(cache, 100),
                iterations=100
            )
            results.append(result)
            print(f"  {result.name}: {result.operations_per_second:.2f} ops/sec, "
                  f"p50: {result.latency_p50_ms:.3f}ms")
        except Exception as e:
            print(f"  Error benchmarking PUT for {cache_name}: {e}")
        
        # Benchmark GET operations
        try:
            # Verify cache configuration for SecureLRUCache - force disable rate limiting
            if cache_name in ['SecureLRUCache', 'PythonSecureLRUCache', 'RustSecureLRUCache']:
                print(f"  DEBUG: Before GET - enable_rate_limit={cache.enable_rate_limit}, has_rate_limiter={hasattr(cache, 'rate_limiter')}")
                cache.enable_rate_limit = False
                if hasattr(cache, 'rate_limiter'):
                    delattr(cache, 'rate_limiter')
                print(f"  DEBUG: After GET - enable_rate_limit={cache.enable_rate_limit}, has_rate_limiter={hasattr(cache, 'rate_limiter')}")
            result = benchmark_operation(
                f"{cache_name} GET",
                lambda: benchmark_cache_get(cache, 100),
                iterations=100
            )
            results.append(result)
            if not result.success and result.error:
                print(f"  {result.name}: {result.operations_per_second:.2f} ops/sec, "
                      f"p50: {result.latency_p50_ms:.3f}ms (ERROR: {result.error})")
            else:
                print(f"  {result.name}: {result.operations_per_second:.2f} ops/sec, "
                      f"p50: {result.latency_p50_ms:.3f}ms")
        except Exception as e:
            print(f"  Error benchmarking GET for {cache_name}: {e}")
        
        # Benchmark mixed operations
        try:
            cache.clear()
            # Verify cache configuration for SecureLRUCache - force disable rate limiting
            if cache_name in ['SecureLRUCache', 'PythonSecureLRUCache', 'RustSecureLRUCache']:
                cache.enable_rate_limit = False
                if hasattr(cache, 'rate_limiter'):
                    delattr(cache, 'rate_limiter')
            result = benchmark_operation(
                f"{cache_name} MIXED",
                lambda: benchmark_cache_mixed(cache, 1000),
                iterations=10
            )
            results.append(result)
            if not result.success and result.error:
                print(f"  {result.name}: {result.operations_per_second:.2f} ops/sec, "
                      f"p50: {result.latency_p50_ms:.3f}ms (ERROR: {result.error})")
            else:
                print(f"  {result.name}: {result.operations_per_second:.2f} ops/sec, "
                      f"p50: {result.latency_p50_ms:.3f}ms")
        except Exception as e:
            print(f"  Error benchmarking MIXED for {cache_name}: {e}")
            
    except Exception as e:
        print(f"  Error initializing {cache_name}: {e}")
    
    return results


def run_benchmarks():
    """Run benchmarks for both Python and Rust implementations."""
    results = []
    
    capacity = 1000
    
    # Define cache classes to benchmark
    python_caches = [
        (PythonLRUCache, "Python LRUCache"),
        (PythonLFUCache, "Python LFUCache"),
        (PythonTTLCache, "Python TTLCache"),
        (PythonOptimizedLFUCache, "Python OptimizedLFUCache"),
        (PythonMemoryBoundedLRUCache, "Python MemoryBoundedLRUCache"),
        (PythonSecureLRUCache, "Python SecureLRUCache"),
    ]
    
    rust_caches = []
    if HAS_RUST:
        rust_caches.extend([
            (RUSTLRUCache, "Rust LRUCache"),
            (RUSTLFUCache, "Rust LFUCache"),
            (RUSTTTLCache, "Rust TTLCache"),
            (RUSTOptimizedLFUCache, "Rust OptimizedLFUCache"),
            (RUSTMemoryBoundedLRUCache, "Rust MemoryBoundedLRUCache"),
            (RUSTSecureLRUCache, "Rust SecureLRUCache"),
        ])
    
    # External Rust caches
    external_rust_caches = []
    if HAS_EXTERNAL_RUST:
        external_rust_caches.extend([
            (RUSTMokaCache, "Rust MokaCache"),
            (RUSTMokaTTLCache, "Rust MokaTTLCache"),
            (RUSTMokaWeightedCache, "Rust MokaWeightedCache"),
            (RUSTQuickCache, "Rust QuickCache"),
            (RUSTQuickCacheTTL, "Rust QuickCacheTTL"),
            (RUSTDashMapCache, "Rust DashMapCache"),
            (RUSTDashMapTTLCache, "Rust DashMapTTLCache"),
        ])
    
    # External Python caches - use new proper implementations
    external_python_caches = []
    try:
        # Try to use the new implementations directly
        from exonware.xwsystem.caching.external_caching_python import (
            CacheboxCache,
            FunctoolsLRUCache,
            CachetoolsLRUCache,
            CachetoolsLFUCache,
            CachetoolsTTLCache,
            CachetoolsRRCache,
        )
        # Add new implementations with proper names
        if HAS_CACHEBOX:
            external_python_caches.append((CacheboxCache, "CacheboxCache"))
        external_python_caches.append((FunctoolsLRUCache, "FunctoolsLRUCache"))
        if HAS_CACHETOOLS:
            external_python_caches.extend([
                (CachetoolsLRUCache, "CachetoolsLRUCache"),
                (CachetoolsLFUCache, "CachetoolsLFUCache"),
                (CachetoolsTTLCache, "CachetoolsTTLCache"),
                (CachetoolsRRCache, "CachetoolsRRCache"),
            ])
    except ImportError:
        # Fallback to wrapper names if direct import fails
        if HAS_CACHEBOX:
            external_python_caches.append((CacheboxWrapper, "Python Cachebox"))
        external_python_caches.append((FunctoolsLRUWrapper, "Python FunctoolsLRU"))
        if HAS_CACHETOOLS:
            external_python_caches.extend([
                (CachetoolsLRUWrapper, "Python CachetoolsLRU"),
                (CachetoolsLFUWrapper, "Python CachetoolsLFU"),
                (CachetoolsTTLWrapper, "Python CachetoolsTTL"),
                (CachetoolsRRWrapper, "Python CachetoolsRR"),
            ])
    if HAS_DISKCACHE:
        external_python_caches.append((DiskcacheWrapper, "Python Diskcache"))
    if HAS_CACHEOUT:
        external_python_caches.extend([
            (CacheoutLRUWrapper, "Python CacheoutLRU"),
            (CacheoutLFUWrapper, "Python CacheoutLFU"),
            (CacheoutTTLWrapper, "Python CacheoutTTL"),
            (CacheoutFIFOWrapper, "Python CacheoutFIFO"),
        ])
    if HAS_PYLRU:
        external_python_caches.append((PylruWrapper, "Python Pylru"))
    
    # Benchmark Python implementations
    print("\n" + "="*60)
    print("Benchmarking Python Cache Implementations")
    print("="*60)
    
    for cache_class, cache_name in python_caches:
        if cache_class:
            print(f"\n{cache_name}:")
            cache_results = benchmark_cache_class(cache_class, cache_name, capacity, python_impl=True)
            results.extend(cache_results)
        else:
            print(f"\n{cache_name}: Not available")
    
    # Benchmark Rust implementations
    if rust_caches:
        print("\n" + "="*60)
        print("Benchmarking Rust Cache Implementations")
        print("="*60)
        
        for cache_class, cache_name in rust_caches:
            if cache_class:
                print(f"\n{cache_name}:")
                cache_results = benchmark_cache_class(cache_class, cache_name, capacity, python_impl=False)
                results.extend(cache_results)
            else:
                print(f"\n{cache_name}: Not available")
    
    # Benchmark External Rust caches
    if external_rust_caches:
        print("\n" + "="*60)
        print("Benchmarking External Rust Cache Libraries")
        print("="*60)
        
        for cache_class, cache_name in external_rust_caches:
            if cache_class:
                print(f"\n{cache_name}:")
                cache_results = benchmark_cache_class(cache_class, cache_name, capacity, python_impl=False)
                results.extend(cache_results)
            else:
                print(f"\n{cache_name}: Not available")
    
    # Benchmark External Python caches
    if external_python_caches:
        print("\n" + "="*60)
        print("Benchmarking External Python Cache Libraries")
        print("="*60)
        
        for cache_class, cache_name in external_python_caches:
            try:
                print(f"\n{cache_name}:")
                cache_results = benchmark_cache_class(cache_class, cache_name, capacity, python_impl=True)
                results.extend(cache_results)
            except Exception as e:
                print(f"\n{cache_name}: Not available ({e})")
    
    # Print comparison summary
    print("\n" + "="*60)
    print("Comparison Summary")
    print("="*60)
    
    # Group results by cache type and operation
    cache_types = set()
    for result in results:
        # Extract cache type from name (e.g., "Python LRUCache PUT" -> "LRUCache")
        parts = result.name.split()
        if len(parts) >= 2:
            cache_type = parts[1]  # e.g., "LRUCache"
            cache_types.add(cache_type)
    
    # Compare each cache type
    for cache_type in sorted(cache_types):
        python_put = next((r for r in results if f"Python {cache_type} PUT" == r.name), None)
        rust_put = next((r for r in results if f"Rust {cache_type} PUT" == r.name), None)
        python_get = next((r for r in results if f"Python {cache_type} GET" == r.name), None)
        rust_get = next((r for r in results if f"Rust {cache_type} GET" == r.name), None)
        python_mixed = next((r for r in results if f"Python {cache_type} MIXED" == r.name), None)
        rust_mixed = next((r for r in results if f"Rust {cache_type} MIXED" == r.name), None)
        
        if python_put or rust_put or python_get or rust_get or python_mixed or rust_mixed:
            print(f"\n{cache_type}:")
            
            if python_put and rust_put:
                speedup = rust_put.operations_per_second / python_put.operations_per_second if python_put.operations_per_second > 0 else 0
                print(f"  PUT:   Python={python_put.operations_per_second:.2f}, Rust={rust_put.operations_per_second:.2f}, Speedup={speedup:.2f}x")
            elif python_put:
                print(f"  PUT:   Python={python_put.operations_per_second:.2f}, Rust=N/A")
            elif rust_put:
                print(f"  PUT:   Python=N/A, Rust={rust_put.operations_per_second:.2f}")
            
            if python_get and rust_get:
                speedup = rust_get.operations_per_second / python_get.operations_per_second if python_get.operations_per_second > 0 else 0
                print(f"  GET:   Python={python_get.operations_per_second:.2f}, Rust={rust_get.operations_per_second:.2f}, Speedup={speedup:.2f}x")
            elif python_get:
                print(f"  GET:   Python={python_get.operations_per_second:.2f}, Rust=N/A")
            elif rust_get:
                print(f"  GET:   Python=N/A, Rust={rust_get.operations_per_second:.2f}")
            
            if python_mixed and rust_mixed:
                speedup = rust_mixed.operations_per_second / python_mixed.operations_per_second if python_mixed.operations_per_second > 0 else 0
                print(f"  MIXED: Python={python_mixed.operations_per_second:.2f}, Rust={rust_mixed.operations_per_second:.2f}, Speedup={speedup:.2f}x")
            elif python_mixed:
                print(f"  MIXED: Python={python_mixed.operations_per_second:.2f}, Rust=N/A")
            elif rust_mixed:
                print(f"  MIXED: Python=N/A, Rust={rust_mixed.operations_per_second:.2f}")
    
    # Show external Rust cache results separately
    external_rust_results = [r for r in results if "Rust Moka" in r.name or "Rust QuickCache" in r.name or "Rust DashMap" in r.name]
    if external_rust_results:
        print("\n" + "="*60)
        print("External Rust Cache Libraries Summary")
        print("="*60)
        
        # Group by cache name
        external_cache_names = set()
        for result in external_rust_results:
            # Extract cache name (e.g., "Rust MokaCache PUT" -> "MokaCache")
            parts = result.name.split()
            if len(parts) >= 3:
                cache_name = parts[1] + parts[2] if len(parts) > 2 else parts[1]
                external_cache_names.add(cache_name)
        
        for cache_name in sorted(external_cache_names):
            put_result = next((r for r in external_rust_results if f"Rust {cache_name} PUT" == r.name), None)
            get_result = next((r for r in external_rust_results if f"Rust {cache_name} GET" == r.name), None)
            mixed_result = next((r for r in external_rust_results if f"Rust {cache_name} MIXED" == r.name), None)
            
            if put_result or get_result or mixed_result:
                print(f"\n{cache_name}:")
                if put_result:
                    print(f"  PUT:   {put_result.operations_per_second:.2f} ops/sec, p50: {put_result.latency_p50_ms:.3f}ms")
                if get_result:
                    print(f"  GET:   {get_result.operations_per_second:.2f} ops/sec, p50: {get_result.latency_p50_ms:.3f}ms")
                if mixed_result:
                    print(f"  MIXED: {mixed_result.operations_per_second:.2f} ops/sec, p50: {mixed_result.latency_p50_ms:.3f}ms")
    
    # Show message if implementations are missing
    rust_caches_available = HAS_RUST and any(cache_class for cache_class, _ in rust_caches)
    external_rust_available = HAS_EXTERNAL_RUST and any(cache_class for cache_class, _ in external_rust_caches)
    
    if not rust_caches_available:
        print("\nNote: Rust implementations not available. To enable Rust benchmarks:")
        print("  cd xwsystem/rust && maturin develop --release")
    
    if not external_rust_available:
        print("\nNote: External Rust caches not available. To enable:")
        print("  cd xwsystem/rust && maturin develop --release --features external-caches")
    
    if not HAS_CACHEBOX:
        print("\nNote: Cachebox not available. Install with: pip install cachebox")
    
    if not HAS_CACHETOOLS:
        print("\nNote: Cachetools not available. Install with: pip install cachetools")
    
    if not HAS_DISKCACHE:
        print("\nNote: Diskcache not available. Install with: pip install diskcache")
    
    if not HAS_CACHEOUT:
        print("\nNote: Cacheout not available. Install with: pip install cacheout")
    
    if not HAS_PYLRU:
        print("\nNote: Pylru not available. Install with: pip install pylru")
    
    return results


if __name__ == "__main__":
    print("Caching Benchmark: Rust vs Python")
    print("="*60)
    print("Testing all available cache implementations")
    print("="*60)
    
    # Check if at least one implementation is available
    python_available = any([
        PythonLRUCache, PythonLFUCache, PythonTTLCache,
        PythonOptimizedLFUCache, PythonMemoryBoundedLRUCache, PythonSecureLRUCache
    ])
    rust_available = RUSTLRUCache is not None
    
    if not python_available and not rust_available:
        print("Error: Neither Python nor Rust implementations available!")
        sys.exit(1)
    
    if not python_available:
        print("Warning: No Python cache implementations available!")
    
    if not rust_available:
        print("Warning: No Rust cache implementations available!")
        print("  To enable Rust benchmarks: cd xwsystem/rust && maturin develop --release")
    
    run_benchmarks()
