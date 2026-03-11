#!/usr/bin/env python3
#exonware/xwsystem/tests/3.advance/benchmarks/legacy/benchmarks/caching_benchmarks.py
"""
#exonware/xwsystem/benchmarks/caching_benchmarks.py
Comprehensive benchmarking suite for caching module.
Captures baseline performance metrics before improvements.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.387
Generation Date: 01-Nov-2025
"""

import time
import sys
import threading
import asyncio
from pathlib import Path
from typing import Any, Callable
from dataclasses import dataclass, asdict
import statistics
# Add src to path
src_path = Path(__file__).parent.parent / "src"
sys.path.insert(0, str(src_path))
from exonware.xwsystem.caching import (
    LRUCache, AsyncLRUCache,
    LFUCache, AsyncLFUCache,
    TTLCache, AsyncTTLCache
)
@dataclass


class BenchmarkResult:
    """Result of a single benchmark."""
    name: str
    operations_per_second: float
    latency_p50_ms: float
    latency_p95_ms: float
    latency_p99_ms: float
    memory_bytes: int
    duration_seconds: float
    success: bool
    error: str = ""


class CacheBenchmark:
    """Benchmark suite for cache implementations."""

    def __init__(self, num_operations: int = 10000):
        self.num_operations = num_operations
        self.results: list[BenchmarkResult] = []

    def benchmark_operation(self, name: str, operation: Callable, iterations: int = None) -> BenchmarkResult:
        """
        Benchmark a single operation.
        Args:
            name: Benchmark name
            operation: Function to benchmark
            iterations: Number of iterations (default: self.num_operations)
        """
        iterations = iterations or self.num_operations
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
            # Memory estimate (simplified)
            memory_bytes = sys.getsizeof(latencies)
            result = BenchmarkResult(
                name=name,
                operations_per_second=ops_per_sec,
                latency_p50_ms=p50,
                latency_p95_ms=p95,
                latency_p99_ms=p99,
                memory_bytes=memory_bytes,
                duration_seconds=duration,
                success=True
            )
        except Exception as e:
            result = BenchmarkResult(
                name=name,
                operations_per_second=0,
                latency_p50_ms=0,
                latency_p95_ms=0,
                latency_p99_ms=0,
                memory_bytes=0,
                duration_seconds=0,
                success=False,
                error=str(e)
            )
        self.results.append(result)
        return result

    def benchmark_lru_cache(self):
        """Benchmark LRU cache operations."""
        print("\n=== Benchmarking LRU Cache ===")
        # Setup
        cache = LRUCache(capacity=1000)
        # Benchmark: PUT operations
        counter = [0]
        def put_op():
            key = f"key_{counter[0] % 1000}"
            cache.put(key, f"value_{counter[0]}")
            counter[0] += 1
        result = self.benchmark_operation("LRU_PUT", put_op)
        print(f"PUT: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms, "
              f"p95: {result.latency_p95_ms:.4f}ms")
        # Benchmark: GET operations (hits)
        counter = [0]
        def get_op():
            key = f"key_{counter[0] % 1000}"
            cache.get(key)
            counter[0] += 1
        result = self.benchmark_operation("LRU_GET", get_op)
        print(f"GET: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms, "
              f"p95: {result.latency_p95_ms:.4f}ms")
        # Benchmark: DELETE operations
        counter = [0]
        def delete_op():
            key = f"key_{counter[0] % 1000}"
            cache.delete(key)
            counter[0] += 1
        result = self.benchmark_operation("LRU_DELETE", delete_op)
        print(f"DELETE: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms, "
              f"p95: {result.latency_p95_ms:.4f}ms")
        # Benchmark: Eviction performance
        cache = LRUCache(capacity=100)
        for i in range(100):
            cache.put(f"key_{i}", f"value_{i}")
        counter = [100]
        def eviction_op():
            cache.put(f"key_{counter[0]}", f"value_{counter[0]}")
            counter[0] += 1
        result = self.benchmark_operation("LRU_EVICTION", eviction_op, iterations=1000)
        print(f"EVICTION: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms, "
              f"p95: {result.latency_p95_ms:.4f}ms")
        # Memory usage
        cache = LRUCache(capacity=10000)
        for i in range(10000):
            cache.put(f"key_{i}", f"value_{i}")
        memory_bytes = sys.getsizeof(cache._cache)
        print(f"MEMORY: {memory_bytes:,} bytes for 10,000 entries "
              f"({memory_bytes / 10000:.2f} bytes/entry)")

    def benchmark_lfu_cache(self):
        """Benchmark LFU cache operations."""
        print("\n=== Benchmarking LFU Cache ===")
        # Setup
        cache = LFUCache(capacity=1000)
        # Benchmark: PUT operations
        counter = [0]
        def put_op():
            key = f"key_{counter[0] % 1000}"
            cache.put(key, f"value_{counter[0]}")
            counter[0] += 1
        result = self.benchmark_operation("LFU_PUT", put_op)
        print(f"PUT: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms, "
              f"p95: {result.latency_p95_ms:.4f}ms")
        # Benchmark: GET operations
        counter = [0]
        def get_op():
            key = f"key_{counter[0] % 1000}"
            cache.get(key)
            counter[0] += 1
        result = self.benchmark_operation("LFU_GET", get_op)
        print(f"GET: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms, "
              f"p95: {result.latency_p95_ms:.4f}ms")
        # Benchmark: Eviction performance (CRITICAL - this is O(n) currently!)
        cache = LFUCache(capacity=100)
        for i in range(100):
            cache.put(f"key_{i}", f"value_{i}")
        counter = [100]
        def eviction_op():
            cache.put(f"key_{counter[0]}", f"value_{counter[0]}")
            counter[0] += 1
        result = self.benchmark_operation("LFU_EVICTION", eviction_op, iterations=1000)
        print(f"EVICTION: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms, "
              f"p95: {result.latency_p95_ms:.4f}ms")
        print(f"  NOTE: LFU eviction is currently O(n) - should improve to O(1)")

    def benchmark_ttl_cache(self):
        """Benchmark TTL cache operations."""
        print("\n=== Benchmarking TTL Cache ===")
        # Setup
        cache = TTLCache(capacity=1000, ttl=300.0, cleanup_interval=0)  # Disable background cleanup
        # Benchmark: PUT operations
        counter = [0]
        def put_op():
            key = f"key_{counter[0] % 1000}"
            cache.put(key, f"value_{counter[0]}")
            counter[0] += 1
        result = self.benchmark_operation("TTL_PUT", put_op)
        print(f"PUT: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms, "
              f"p95: {result.latency_p95_ms:.4f}ms")
        # Benchmark: GET operations
        counter = [0]
        def get_op():
            key = f"key_{counter[0] % 1000}"
            cache.get(key)
            counter[0] += 1
        result = self.benchmark_operation("TTL_GET", get_op)
        print(f"GET: {result.operations_per_second:.2f} ops/sec, "
              f"p50: {result.latency_p50_ms:.4f}ms, "
              f"p95: {result.latency_p95_ms:.4f}ms")
        # Benchmark: Cleanup performance
        cache = TTLCache(capacity=1000, ttl=0.001, cleanup_interval=0)  # Very short TTL
        for i in range(1000):
            cache.put(f"key_{i}", f"value_{i}")
        time.sleep(0.01)  # Wait for TTL to expire
        start = time.perf_counter()
        cache._cleanup_expired()
        end = time.perf_counter()
        cleanup_time_ms = (end - start) * 1000
        print(f"CLEANUP: {cleanup_time_ms:.4f}ms for 1000 expired entries")
        cache.shutdown()

    def benchmark_concurrent_access(self):
        """Benchmark thread safety and concurrent access."""
        print("\n=== Benchmarking Concurrent Access ===")
        cache = LRUCache(capacity=1000)
        num_threads = 10
        operations_per_thread = 1000
        def worker(thread_id: int):
            for i in range(operations_per_thread):
                key = f"key_{thread_id}_{i % 100}"
                cache.put(key, f"value_{thread_id}_{i}")
                cache.get(key)
        start_time = time.time()
        threads = []
        for i in range(num_threads):
            thread = threading.Thread(target=worker, args=(i,))
            threads.append(thread)
            thread.start()
        for thread in threads:
            thread.join()
        end_time = time.time()
        duration = end_time - start_time
        total_ops = num_threads * operations_per_thread * 2  # PUT + GET
        ops_per_sec = total_ops / duration
        print(f"CONCURRENT: {num_threads} threads, "
              f"{ops_per_sec:.2f} ops/sec, "
              f"{duration:.2f}s total")
        result = BenchmarkResult(
            name="CONCURRENT_ACCESS",
            operations_per_second=ops_per_sec,
            latency_p50_ms=0,
            latency_p95_ms=0,
            latency_p99_ms=0,
            memory_bytes=0,
            duration_seconds=duration,
            success=True
        )
        self.results.append(result)

    def benchmark_async_cache(self):
        """Benchmark async cache operations."""
        print("\n=== Benchmarking Async Cache ===")
        async def run_async_benchmark():
            cache = AsyncLRUCache(capacity=1000)
            # PUT operations
            start = time.perf_counter()
            for i in range(1000):
                await cache.put(f"key_{i}", f"value_{i}")
            end = time.perf_counter()
            put_duration = end - start
            put_ops_per_sec = 1000 / put_duration
            # GET operations
            start = time.perf_counter()
            for i in range(1000):
                await cache.get(f"key_{i}")
            end = time.perf_counter()
            get_duration = end - start
            get_ops_per_sec = 1000 / get_duration
            print(f"ASYNC PUT: {put_ops_per_sec:.2f} ops/sec")
            print(f"ASYNC GET: {get_ops_per_sec:.2f} ops/sec")
            result = BenchmarkResult(
                name="ASYNC_LRU",
                operations_per_second=(put_ops_per_sec + get_ops_per_sec) / 2,
                latency_p50_ms=0,
                latency_p95_ms=0,
                latency_p99_ms=0,
                memory_bytes=0,
                duration_seconds=put_duration + get_duration,
                success=True
            )
            self.results.append(result)
        asyncio.run(run_async_benchmark())

    def run_all_benchmarks(self):
        """Run all benchmarks."""
        print("=" * 80)
        print("CACHING MODULE BASELINE BENCHMARKS")
        print("=" * 80)
        print(f"Operations per test: {self.num_operations:,}")
        print(f"Python version: {sys.version}")
        print("=" * 80)
        self.benchmark_lru_cache()
        self.benchmark_lfu_cache()
        self.benchmark_ttl_cache()
        self.benchmark_concurrent_access()
        self.benchmark_async_cache()
        print("\n" + "=" * 80)
        print("BENCHMARK SUMMARY")
        print("=" * 80)
        for result in self.results:
            if result.success:
                print(f"{result.name:30s}: {result.operations_per_second:15,.2f} ops/sec")
            else:
                print(f"{result.name:30s}: FAILED - {result.error}")
        print("=" * 80)

    def get_results_dict(self) -> dict[str, Any]:
        """Get results as dictionary."""
        return {
            'timestamp': time.time(),
            'num_operations': self.num_operations,
            'python_version': sys.version,
            'results': [asdict(r) for r in self.results]
        }


def main():
    """Run benchmarks."""
    benchmark = CacheBenchmark(num_operations=10000)
    benchmark.run_all_benchmarks()
    return benchmark
if __name__ == "__main__":
    main()
