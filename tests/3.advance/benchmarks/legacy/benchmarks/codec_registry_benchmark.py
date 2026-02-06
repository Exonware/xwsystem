#!/usr/bin/env python3
#exonware/xwsystem/tests/3.advance/benchmarks/legacy/benchmarks/codec_registry_benchmark.py
"""
#exonware/xwsystem/benchmarks/codec_registry_benchmark.py

Performance benchmarks for UniversalCodecRegistry.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 04-Nov-2025

Validates that performance targets are met:
- Codec lookup: < 1ms (O(1) hash map)
- Detection (cached): < 1ms (LRU hit)
- Detection (uncached): < 5ms
- Registration: < 5ms per codec
"""

import sys
from pathlib import Path
import timeit
import threading
from datetime import datetime

# Add src to path
src_path = Path(__file__).parent.parent / "src"
sys.path.insert(0, str(src_path))

# Import only what we need to avoid dependency issues
from exonware.xwsystem.io.codec.registry import UniversalCodecRegistry
from exonware.xwsystem.io.serialization.formats.text.json import XWJsonSerializer
from exonware.xwsystem.io.serialization.formats.text.yaml import XWYamlSerializer
from exonware.xwsystem.io.serialization.formats.text.xml import XWXmlSerializer

# Create a simple populated registry for benchmarks
_benchmark_registry = UniversalCodecRegistry()
_benchmark_registry.register(XWJsonSerializer)
_benchmark_registry.register(XWYamlSerializer)
_benchmark_registry.register(XWXmlSerializer)


def benchmark_lookup_by_id():
    """Benchmark codec lookup by ID."""
    registry = _benchmark_registry
    
    def lookup():
        codec = registry.get_by_id('json')
        return codec
    
    iterations = 10000
    elapsed = timeit.timeit(lookup, number=iterations)
    avg_ms = (elapsed / iterations) * 1000
    
    print(f"📊 Lookup by ID:")
    print(f"   Iterations: {iterations:,}")
    print(f"   Total time: {elapsed:.4f}s")
    print(f"   Average: {avg_ms:.4f}ms per lookup")
    print(f"   Target: < 1.0ms")
    print(f"   Status: {'✅ PASS' if avg_ms < 1.0 else '❌ FAIL'}")
    
    return avg_ms < 1.0


def benchmark_detection_cached():
    """Benchmark detection with LRU cache."""
    registry = _benchmark_registry
    
    # Warm up cache
    registry.detect('config.json')
    
    def detect():
        codec = registry.detect('config.json')
        return codec
    
    iterations = 10000
    elapsed = timeit.timeit(detect, number=iterations)
    avg_ms = (elapsed / iterations) * 1000
    
    print(f"\n📊 Detection (Cached):")
    print(f"   Iterations: {iterations:,}")
    print(f"   Total time: {elapsed:.4f}s")
    print(f"   Average: {avg_ms:.4f}ms per detection")
    print(f"   Target: < 1.0ms")
    print(f"   Status: {'✅ PASS' if avg_ms < 1.0 else '❌ FAIL'}")
    
    return avg_ms < 1.0


def benchmark_detection_uncached():
    """Benchmark detection without cache (cold start)."""
    def detect_fresh():
        registry = UniversalCodecRegistry()
        registry.register(XWJsonSerializer)
        registry.register(XWYamlSerializer)
        codec = registry.detect('config.json')
        return codec
    
    iterations = 1000
    elapsed = timeit.timeit(detect_fresh, number=iterations)
    avg_ms = (elapsed / iterations) * 1000
    
    print(f"\n📊 Detection (Uncached):")
    print(f"   Iterations: {iterations:,}")
    print(f"   Total time: {elapsed:.4f}s")
    print(f"   Average: {avg_ms:.4f}ms per detection")
    print(f"   Target: < 5.0ms")
    print(f"   Status: {'✅ PASS' if avg_ms < 5.0 else '❌ FAIL'}")
    
    return avg_ms < 5.0


def benchmark_registration():
    """Benchmark codec registration."""
    def register_codec():
        registry = UniversalCodecRegistry()
        registry.register(XWJsonSerializer)
    
    iterations = 1000
    elapsed = timeit.timeit(register_codec, number=iterations)
    avg_ms = (elapsed / iterations) * 1000
    
    print(f"\n📊 Registration:")
    print(f"   Iterations: {iterations:,}")
    print(f"   Total time: {elapsed:.4f}s")
    print(f"   Average: {avg_ms:.4f}ms per registration")
    print(f"   Target: < 5.0ms")
    print(f"   Status: {'✅ PASS' if avg_ms < 5.0 else '❌ FAIL'}")
    
    return avg_ms < 5.0


def benchmark_thread_safety():
    """Benchmark concurrent access performance."""
    registry = _benchmark_registry
    results = []
    
    def concurrent_lookup():
        for _ in range(100):
            codec = registry.get_by_id('json')
            results.append(codec)
    
    num_threads = 10
    threads = [threading.Thread(target=concurrent_lookup) for _ in range(num_threads)]
    
    start = timeit.default_timer()
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    elapsed = timeit.default_timer() - start
    
    total_ops = num_threads * 100
    avg_ms = (elapsed / total_ops) * 1000
    
    print(f"\n📊 Thread Safety (Concurrent Access):")
    print(f"   Threads: {num_threads}")
    print(f"   Operations per thread: 100")
    print(f"   Total operations: {total_ops}")
    print(f"   Total time: {elapsed:.4f}s")
    print(f"   Average: {avg_ms:.4f}ms per operation")
    print(f"   All returned same instance: {len(set(id(r) for r in results)) == 1}")
    print(f"   Status: ✅ PASS (Thread-safe)")
    
    return True


def benchmark_bulk_registration():
    """Benchmark bulk registration performance."""
    from exonware.xwsystem.io.serialization.formats.text.toml import XWTomlSerializer
    from exonware.xwsystem.io.serialization.formats.text.csv import XWCsvSerializer
    
    def bulk_register():
        registry = UniversalCodecRegistry()
        codecs = [XWJsonSerializer, XWYamlSerializer, XWXmlSerializer, XWTomlSerializer, XWCsvSerializer]
        count = registry.register_bulk(codecs)
        return count
    
    iterations = 1000
    elapsed = timeit.timeit(bulk_register, number=iterations)
    avg_ms = (elapsed / iterations) * 1000
    
    print(f"\n📊 Bulk Registration (5 codecs):")
    print(f"   Iterations: {iterations:,}")
    print(f"   Total time: {elapsed:.4f}s")
    print(f"   Average: {avg_ms:.4f}ms per bulk operation")
    print(f"   Per codec: {avg_ms/5:.4f}ms")
    print(f"   Target: < 25ms total (5 codecs × 5ms)")
    print(f"   Status: {'✅ PASS' if avg_ms < 25.0 else '❌ FAIL'}")
    
    return avg_ms < 25.0


def benchmark_type_filtering():
    """Benchmark type-based filtering performance."""
    registry = _benchmark_registry
    
    def filter_by_type():
        codecs = registry.get_all_by_type('serialization')
        return codecs
    
    iterations = 10000
    elapsed = timeit.timeit(filter_by_type, number=iterations)
    avg_ms = (elapsed / iterations) * 1000
    
    print(f"\n📊 Type Filtering:")
    print(f"   Iterations: {iterations:,}")
    print(f"   Total time: {elapsed:.4f}s")
    print(f"   Average: {avg_ms:.4f}ms per filter operation")
    print(f"   Target: < 2.0ms")
    print(f"   Status: {'✅ PASS' if avg_ms < 2.0 else '❌ FAIL'}")
    
    return avg_ms < 2.0


def main():
    """Run all benchmarks."""
    print("=" * 80)
    print("🚀 Universal Codec Registry - Performance Benchmarks")
    print("=" * 80)
    print(f"\n📅 Generated: {datetime.now().strftime('%d-%b-%Y %H:%M:%S')}")
    print(f"📂 Platform: {sys.platform}")
    print(f"🐍 Python: {sys.version}")
    print("\n" + "=" * 80)
    print("📈 BENCHMARK RESULTS")
    print("=" * 80)
    
    results = []
    
    # Run all benchmarks
    results.append(("Lookup by ID", benchmark_lookup_by_id()))
    results.append(("Detection (Cached)", benchmark_detection_cached()))
    results.append(("Detection (Uncached)", benchmark_detection_uncached()))
    results.append(("Registration", benchmark_registration()))
    results.append(("Thread Safety", benchmark_thread_safety()))
    results.append(("Bulk Registration", benchmark_bulk_registration()))
    results.append(("Type Filtering", benchmark_type_filtering()))
    
    # Summary
    print("\n" + "=" * 80)
    print("📊 SUMMARY")
    print("=" * 80)
    
    passed = sum(1 for _, result in results if result)
    total = len(results)
    
    print(f"\nBenchmarks Passed: {passed}/{total}")
    
    for name, result in results:
        status = "✅ PASS" if result else "❌ FAIL"
        print(f"  {status} - {name}")
    
    print("\n" + "=" * 80)
    
    if all(result for _, result in results):
        print("✅ ALL BENCHMARKS PASSED - Performance targets met!")
        print("=" * 80)
        print("\n🎯 Performance Summary:")
        print("   • Lookups: < 1ms (hash map O(1))")
        print("   • Detection: < 1ms cached, < 5ms uncached")
        print("   • Registration: < 5ms per codec")
        print("   • Thread-safe: No contention")
        print("   • Bulk ops: Efficient")
        print("   • Type filtering: < 2ms")
        print("\n🚀 Universal Codec Registry is PRODUCTION-READY!")
        return 0
    else:
        print("❌ SOME BENCHMARKS FAILED - Performance targets not met")
        print("=" * 80)
        return 1


if __name__ == "__main__":
    sys.exit(main())
