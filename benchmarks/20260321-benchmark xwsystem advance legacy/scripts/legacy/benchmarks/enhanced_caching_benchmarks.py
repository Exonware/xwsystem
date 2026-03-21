#!/usr/bin/env python3
#exonware/xwsystem/tests/3.advance/benchmarks/legacy/benchmarks/enhanced_caching_benchmarks.py
"""
#exonware/xwsystem/benchmarks/enhanced_caching_benchmarks.py
Enhanced benchmarking suite for improved caching module.
Tests new features and compares to baseline.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.388
Generation Date: 01-Nov-2025
"""

import time
import sys
from pathlib import Path
# Add src to path
src_path = Path(__file__).parent.parent / "src"
sys.path.insert(0, str(src_path))
from caching_benchmarks import CacheBenchmark, BenchmarkResult
from exonware.xwsystem.caching import (
    LRUCache,
    LFUCache,
    OptimizedLFUCache,
    SecureLRUCache,
    MemoryBoundedLRUCache,
)


class EnhancedCacheBenchmark(CacheBenchmark):
    """Enhanced benchmarks for v0.0.1.388 features."""

    def benchmark_optimized_lfu(self):
        """Benchmark optimized O(1) LFU cache."""
        print("\n=== Benchmarking Optimized LFU Cache ===")
        cache = OptimizedLFUCache(capacity=1000)
        # PUT operations
        counter = [0]
        def put_op():
            key = f"key_{counter[0] % 1000}"
            cache.put(key, f"value_{counter[0]}")
            counter[0] += 1
        result = self.benchmark_operation("OPTIMIZED_LFU_PUT", put_op)
        print(f"PUT: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms")
        # GET operations
        counter = [0]
        def get_op():
            key = f"key_{counter[0] % 1000}"
            cache.get(key)
            counter[0] += 1
        result = self.benchmark_operation("OPTIMIZED_LFU_GET", get_op)
        print(f"GET: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms")
        # EVICTION performance (CRITICAL - should be 100x faster)
        cache = OptimizedLFUCache(capacity=100)
        for i in range(100):
            cache.put(f"key_{i}", f"value_{i}")
        counter = [100]
        def eviction_op():
            cache.put(f"key_{counter[0]}", f"value_{counter[0]}")
            counter[0] += 1
        result = self.benchmark_operation("OPTIMIZED_LFU_EVICTION", eviction_op, iterations=1000)
        print(f"EVICTION: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms")
        print(f"  IMPROVEMENT: Should be ~100x faster than naive LFU")

    def benchmark_batch_operations(self):
        """Benchmark batch operations."""
        print("\n=== Benchmarking Batch Operations ===")
        cache = LRUCache(capacity=1000)
        # Individual puts
        start = time.perf_counter()
        for i in range(1000):
            cache.put(f"key_{i}", f"value_{i}")
        individual_time = time.perf_counter() - start
        individual_ops_per_sec = 1000 / individual_time
        print(f"INDIVIDUAL PUT: {individual_ops_per_sec:.2f} ops/sec")
        # Clear cache
        cache.clear()
        # Batch put
        items = {f"key_{i}": f"value_{i}" for i in range(1000)}
        start = time.perf_counter()
        cache.put_many(items)
        batch_time = time.perf_counter() - start
        batch_ops_per_sec = 1000 / batch_time
        speedup = batch_ops_per_sec / individual_ops_per_sec
        print(f"BATCH PUT: {batch_ops_per_sec:.2f} ops/sec")
        print(f"SPEEDUP: {speedup:.2f}x faster")
        result = BenchmarkResult(
            name="BATCH_PUT_SPEEDUP",
            operations_per_second=batch_ops_per_sec,
            latency_p50_ms=0,
            latency_p95_ms=0,
            latency_p99_ms=0,
            memory_bytes=0,
            duration_seconds=batch_time,
            success=True
        )
        self.results.append(result)

    def benchmark_secure_cache_overhead(self):
        """Benchmark security feature overhead."""
        print("\n=== Benchmarking Security Overhead ===")
        # Standard cache (no security)
        standard_cache = LRUCache(capacity=1000)
        counter = [0]
        def standard_op():
            cache = standard_cache
            cache.put(f"key_{counter[0] % 1000}", f"value_{counter[0]}")
            cache.get(f"key_{counter[0] % 1000}")
            counter[0] += 1
        result = self.benchmark_operation("STANDARD_CACHE", standard_op)
        standard_ops_per_sec = result.operations_per_second
        # Secure cache (with validation + rate limiting)
        secure_cache = SecureLRUCache(
            capacity=1000,
            enable_integrity=False,  # Disable for fair comparison
            enable_rate_limit=True,
            max_ops_per_second=50000
        )
        counter = [0]
        def secure_op():
            cache = secure_cache
            cache.put(f"key_{counter[0] % 1000}", f"value_{counter[0]}")
            cache.get(f"key_{counter[0] % 1000}")
            counter[0] += 1
        result = self.benchmark_operation("SECURE_CACHE", secure_op)
        secure_ops_per_sec = result.operations_per_second
        overhead_pct = ((standard_ops_per_sec - secure_ops_per_sec) / 
                       standard_ops_per_sec * 100)
        print(f"STANDARD: {standard_ops_per_sec:.2f} ops/sec")
        print(f"SECURE:   {secure_ops_per_sec:.2f} ops/sec")
        print(f"OVERHEAD: {overhead_pct:.1f}%")
        print(f"  TARGET: < 10% overhead")

    def benchmark_memory_bounded(self):
        """Benchmark memory-bounded cache."""
        print("\n=== Benchmarking Memory-Bounded Cache ===")
        cache = MemoryBoundedLRUCache(
            capacity=10000,
            memory_budget_mb=10.0
        )
        # Fill with 1KB values
        counter = [0]
        def put_op():
            cache.put(f"key_{counter[0] % 1000}", "x" * 1024)
            counter[0] += 1
        result = self.benchmark_operation("MEMORY_BOUNDED_PUT", put_op)
        print(f"PUT: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms")
        # Check memory stats
        stats = cache.get_stats()
        print(f"MEMORY USED: {stats['memory_used_pct']:.1f}%")

    def run_all_enhanced_benchmarks(self):
        """Run all enhanced benchmarks."""
        print("=" * 80)
        print("ENHANCED CACHING MODULE BENCHMARKS (v0.0.1.388)")
        print("=" * 80)
        print(f"Operations per test: {self.num_operations:,}")
        print("=" * 80)
        # Original benchmarks
        self.benchmark_lru_cache()
        self.benchmark_lfu_cache()
        self.benchmark_ttl_cache()
        # NEW: Optimized LFU (key improvement)
        self.benchmark_optimized_lfu()
        # NEW: Batch operations
        self.benchmark_batch_operations()
        # NEW: Security overhead
        self.benchmark_secure_cache_overhead()
        # NEW: Memory bounded
        self.benchmark_memory_bounded()
        # Concurrent and async
        self.benchmark_concurrent_access()
        self.benchmark_async_cache()
        print("\n" + "=" * 80)
        print("ENHANCED BENCHMARK SUMMARY")
        print("=" * 80)
        for result in self.results:
            if result.success:
                print(f"{result.name:35s}: {result.operations_per_second:15,.2f} ops/sec")
            else:
                print(f"{result.name:35s}: FAILED - {result.error}")
        print("=" * 80)


def main():
    """Run enhanced benchmarks."""
    benchmark = EnhancedCacheBenchmark(num_operations=10000)
    benchmark.run_all_enhanced_benchmarks()
    return benchmark
if __name__ == "__main__":
    main()
