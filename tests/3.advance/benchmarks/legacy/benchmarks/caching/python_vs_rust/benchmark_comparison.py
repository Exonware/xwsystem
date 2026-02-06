#!/usr/bin/env python3
#exonware/xwsystem/tests/3.advance/benchmarks/legacy/benchmarks/caching/python_vs_rust/benchmark_comparison.py
"""
Python vs Rust Caching Benchmark Comparison

This script benchmarks Python and Rust implementations of caching modules
and generates a comprehensive comparison report.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
"""

import time
import statistics
import json
from pathlib import Path
from typing import Dict, List, Any
import sys

# Import Python implementations
try:
    from exonware.xwsystem.caching import (
        LRUCache as PythonLRUCache,
        LFUCache as PythonLFUCache,
        TTLCache as PythonTTLCache,
    )
except ImportError:
    print("Warning: Could not import Python caching modules")
    PythonLRUCache = None
    PythonLFUCache = None
    PythonTTLCache = None

# Import Rust implementations
try:
    from exonware.rust.xwsystem.caching import (
        LRUCache as RustLRUCache,
        LFUCache as RustLFUCache,
        TTLCache as RustTTLCache,
    )
    RUST_AVAILABLE = True
except ImportError:
    print("Warning: Could not import Rust caching modules. Install with: pip install -e xwsystem/rust --features python")
    RUST_AVAILABLE = False
    RustLRUCache = None
    RustLFUCache = None
    RustTTLCache = None


class BenchmarkRunner:
    """Run benchmarks comparing Python and Rust implementations."""
    
    def __init__(self, iterations: int = 1000, warmup: int = 100):
        self.iterations = iterations
        self.warmup = warmup
        self.results: Dict[str, Any] = {}
    
    def generate_test_data(self, count: int) -> List[tuple]:
        """Generate test key-value pairs."""
        return [(f"key_{i}", {"value": i, "data": f"data_{i}" * 10}) for i in range(count)]
    
    def benchmark_operation(self, name: str, cache, operation: callable, *args, **kwargs):
        """Benchmark a single operation."""
        # Warmup
        for _ in range(self.warmup):
            try:
                operation(*args, **kwargs)
            except:
                pass
        
        # Actual benchmark
        times = []
        for _ in range(self.iterations):
            start = time.perf_counter()
            try:
                operation(*args, **kwargs)
            except:
                pass
            end = time.perf_counter()
            times.append((end - start) * 1000)  # Convert to milliseconds
        
        return {
            "mean": statistics.mean(times),
            "median": statistics.median(times),
            "stdev": statistics.stdev(times) if len(times) > 1 else 0,
            "min": min(times),
            "max": max(times),
            "total": sum(times),
        }
    
    def benchmark_get(self, cache, test_data: List[tuple]):
        """Benchmark get operations."""
        # Populate cache
        for key, value in test_data:
            try:
                cache.put(key, value)
            except:
                pass
        
        # Benchmark gets
        results = []
        for key, _ in test_data:
            result = self.benchmark_operation(
                "get",
                cache,
                cache.get,
                key,
                None
            )
            results.append(result["mean"])
        
        return {
            "mean": statistics.mean(results),
            "median": statistics.median(results),
            "stdev": statistics.stdev(results) if len(results) > 1 else 0,
        }
    
    def benchmark_put(self, cache, test_data: List[tuple]):
        """Benchmark put operations."""
        results = []
        for key, value in test_data:
            result = self.benchmark_operation(
                "put",
                cache,
                cache.put,
                key,
                value
            )
            results.append(result["mean"])
        
        return {
            "mean": statistics.mean(results),
            "median": statistics.median(results),
            "stdev": statistics.stdev(results) if len(results) > 1 else 0,
        }
    
    def benchmark_cache(self, cache_class, cache_name: str, capacity: int = 1000):
        """Benchmark a cache implementation."""
        print(f"\nBenchmarking {cache_name}...")
        
        cache = cache_class(capacity=capacity)
        test_data = self.generate_test_data(capacity)
        
        results = {
            "cache_name": cache_name,
            "capacity": capacity,
            "get": self.benchmark_get(cache, test_data),
            "put": self.benchmark_put(cache, test_data),
        }
        
        # Test size operation
        size_result = self.benchmark_operation("size", cache, cache.size)
        results["size"] = size_result
        
        return results
    
    def run_all_benchmarks(self):
        """Run all benchmarks."""
        print("=" * 80)
        print("Python vs Rust Caching Benchmark")
        print("=" * 80)
        
        capacity = 1000
        
        # Python benchmarks
        if PythonLRUCache:
            self.results["python_lru"] = self.benchmark_cache(
                PythonLRUCache, "Python LRUCache", capacity
            )
        
        if PythonLFUCache:
            self.results["python_lfu"] = self.benchmark_cache(
                PythonLFUCache, "Python LFUCache", capacity
            )
        
        if PythonTTLCache:
            self.results["python_ttl"] = self.benchmark_cache(
                PythonTTLCache, "Python TTLCache", capacity
            )
        
        # Rust benchmarks
        if RUST_AVAILABLE:
            if RustLRUCache:
                self.results["rust_lru"] = self.benchmark_cache(
                    RustLRUCache, "Rust LRUCache", capacity
                )
            
            if RustLFUCache:
                self.results["rust_lfu"] = self.benchmark_cache(
                    RustLFUCache, "Rust LFUCache", capacity
                )
            
            if RustTTLCache:
                self.results["rust_ttl"] = self.benchmark_cache(
                    RustTTLCache, "Rust TTLCache", capacity
                )
        else:
            print("\nSkipping Rust benchmarks (Rust module not available)")
    
    def generate_report(self) -> str:
        """Generate markdown report."""
        report = []
        report.append("# Python vs Rust Caching Benchmark Report\n")
        report.append(f"Generated: {time.strftime('%Y-%m-%d %H:%M:%S')}\n")
        report.append(f"Iterations: {self.iterations}\n")
        report.append(f"Warmup: {self.warmup}\n\n")
        
        # Comparison table
        report.append("## Performance Comparison\n\n")
        report.append("| Cache Type | Implementation | Get (ms) | Put (ms) | Size (ms) |\n")
        report.append("|------------|----------------|----------|----------|-----------|\n")
        
        cache_types = ["lru", "lfu", "ttl"]
        for cache_type in cache_types:
            python_key = f"python_{cache_type}"
            rust_key = f"rust_{cache_type}"
            
            if python_key in self.results:
                py_result = self.results[python_key]
                report.append(
                    f"| {cache_type.upper()} | Python | "
                    f"{py_result['get']['mean']:.4f} | "
                    f"{py_result['put']['mean']:.4f} | "
                    f"{py_result['size']['mean']:.4f} |\n"
                )
            
            if rust_key in self.results:
                rust_result = self.results[rust_key]
                report.append(
                    f"| {cache_type.upper()} | Rust | "
                    f"{rust_result['get']['mean']:.4f} | "
                    f"{rust_result['put']['mean']:.4f} | "
                    f"{rust_result['size']['mean']:.4f} |\n"
                )
        
        # Speedup calculation
        report.append("\n## Speedup Analysis\n\n")
        for cache_type in cache_types:
            python_key = f"python_{cache_type}"
            rust_key = f"rust_{cache_type}"
            
            if python_key in self.results and rust_key in self.results:
                py_result = self.results[python_key]
                rust_result = self.results[rust_key]
                
                get_speedup = py_result['get']['mean'] / rust_result['get']['mean']
                put_speedup = py_result['put']['mean'] / rust_result['put']['mean']
                
                report.append(f"### {cache_type.upper()} Cache\n\n")
                report.append(f"- Get speedup: {get_speedup:.2f}x\n")
                report.append(f"- Put speedup: {put_speedup:.2f}x\n\n")
        
        # Detailed results
        report.append("## Detailed Results\n\n")
        report.append("```json\n")
        report.append(json.dumps(self.results, indent=2))
        report.append("\n```\n")
        
        return "".join(report)
    
    def save_results(self, output_dir: Path):
        """Save results to files."""
        output_dir.mkdir(parents=True, exist_ok=True)
        
        # Save JSON
        json_path = output_dir / "benchmark_results.json"
        with open(json_path, 'w') as f:
            json.dump(self.results, f, indent=2)
        
        # Save markdown report
        report_path = output_dir / "benchmark_report.md"
        with open(report_path, 'w') as f:
            f.write(self.generate_report())
        
        print(f"\nResults saved to:")
        print(f"  - {json_path}")
        print(f"  - {report_path}")


def main():
    """Main entry point."""
    runner = BenchmarkRunner(iterations=1000, warmup=100)
    runner.run_all_benchmarks()
    
    # Generate and save report
    output_dir = Path(__file__).parent / "results"
    runner.save_results(output_dir)
    
    # Print summary
    print("\n" + "=" * 80)
    print("Benchmark Summary")
    print("=" * 80)
    print(runner.generate_report())


if __name__ == "__main__":
    main()
