# Performance Benchmark Reference

**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Email:** connect@exonware.com  
**Version:** 0.0.1  
**Last Updated:** 07-Feb-2026  
**Requirements source:** [REF_01_REQ.md](REF_01_REQ.md) sec. 8 (performance)

---

## Overview

This document defines performance Service Level Agreements (SLAs) and Non-Functional Requirements (NFRs) **for exonware-xwsystem specifically**. All benchmarks should verify compliance with these targets.

**For benchmarking process:** See [guides/GUIDE_BENCH.md](guides/GUIDE_BENCH.md)

**Related Documents:**
- [guides/GUIDE_BENCH.md](guides/GUIDE_BENCH.md) - How to write benchmarks (universal process)
- [REF_22_PROJECT.md](REF_22_PROJECT.md) - Project requirements (NFR section)
- [logs/benchmarks/INDEX.md](logs/benchmarks/INDEX.md) - Benchmark history
- [logs/benchmarks/](logs/benchmarks/) - Detailed results

**Historical benchmark data:** Pre-optimization (stdlib json vs orjson) and index-building benchmark value was absorbed; key results (e.g. 3.32x JSON parsing improvement with orjson) are reflected in current SLAs above and in logs/benchmarks/.

---

## Performance Targets for xwsystem

Performance is **Priority #4** in eXonware. These targets reflect xwsystem-specific requirements:

1. **Sub-50ms serialization** - User expectation for 1MB data
2. **Linear scaling** - O(n) to 1M items
3. **3x memory efficiency** - Peak memory < 3x data size
4. **Zero regressions** - Max 10% performance degradation

---

## Service Level Agreements (SLAs)

### Serialization Performance

#### JSON

| Operation | Data Size | Target | Maximum | Status |
|-----------|-----------|--------|---------|--------|
| Serialize | 1MB | < 45ms | < 50ms | ✅ Met (45ms) |
| Deserialize | 1MB | < 50ms | < 55ms | ⚠️ Close (52ms) |
| File Write | 1MB | < 60ms | < 70ms | ✅ Met (58ms) |
| File Read | 1MB | < 65ms | < 75ms | ✅ Met (63ms) |

#### YAML

| Operation | Data Size | Target | Maximum | Status |
|-----------|-----------|--------|---------|--------|
| Serialize | 1MB | < 120ms | < 150ms | ✅ Met (115ms) |
| Deserialize | 1MB | < 130ms | < 160ms | ✅ Met (125ms) |
| File Write | 1MB | < 140ms | < 170ms | ✅ Met (135ms) |
| File Read | 1MB | < 145ms | < 175ms | ✅ Met (140ms) |

#### MessagePack

| Operation | Data Size | Target | Maximum | Status |
|-----------|-----------|--------|---------|--------|
| Serialize | 1MB | < 25ms | < 30ms | ✅ Met (23ms) |
| Deserialize | 1MB | < 28ms | < 35ms | ✅ Met (26ms) |
| File Write | 1MB | < 35ms | < 40ms | ✅ Met (32ms) |
| File Read | 1MB | < 38ms | < 45ms | ✅ Met (35ms) |

#### TOML

| Operation | Data Size | Target | Maximum | Status |
|-----------|-----------|--------|---------|--------|
| Serialize | 100KB | < 30ms | < 40ms | ✅ Met (28ms) |
| Deserialize | 100KB | < 35ms | < 45ms | ✅ Met (32ms) |

**Note:** TOML not recommended for large data (design limitation).

#### Benchmark Trade-offs and When to Use

The following guidance is derived from benchmark campaigns (see `benchmarks/INDEX.md`). Use it to choose the right component and to interpret numbers.

| Area | Finding | When to Use / Recommendation |
|------|---------|------------------------------|
| **Object pool** | ObjectPool get+release ~8× slower than direct instantiation (~350k vs ~2.9M ops/s). | Use the pool when object creation cost is high or when you need bounded concurrency and reuse; prefer direct instantiation for trivial objects and max throughput. |
| **EnhancedRLock** | EnhancedRLock (default) ~2× slower than `threading.RLock` (~550k vs ~1M ops/s) due to timeout and stats. | Use `EnhancedRLock(track_stats=False)` when you need the API but not stats/timeouts (closer to RLock speed). Use default when you need `get_stats()`, timeouts, or debug logging. |
| **Trie / UnionFind** | TrieNode lookup ~29× slower than dict lookup; UnionFind (make_set+union+find) ~573 runs/s. | Use Trie for prefix/range semantics and UnionFind for disjoint sets; use plain dict for simple key lookup when structure is not needed. |
| **Atomic I/O** | AtomicFileWriter adds ~2–3× latency vs plain write (crash-safe commit). | Use for durability when crash safety matters; use plain write when throughput is critical and durability is handled elsewhere. |
| **TwoTierCache** | Implements `put()` and `get(key, default)` for cache contract; benchmark harness uses these. | All xwsystem caches used in benchmarks conform to the same get/put contract (see GUIDE_54_BENCH). |

#### XML

| Operation | Data Size | Target | Maximum | Status |
|-----------|-----------|--------|---------|--------|
| Serialize | 1MB | < 80ms | < 100ms | ✅ Met (75ms) |
| Deserialize | 1MB | < 90ms | < 110ms | ✅ Met (85ms) |

---

### Caching Performance

#### LRU Cache

| Operation | Size | Target | Maximum | Status |
|-----------|------|--------|---------|--------|
| Get (hit) | 10K items | < 1μs | < 5μs | ✅ Met (O(1)) |
| Get (miss) | 10K items | < 1μs | < 5μs | ✅ Met (O(1)) |
| Set | 10K items | < 2μs | < 10μs | ✅ Met (O(1)) |
| Eviction | 10K items | < 5μs | < 20μs | ✅ Met (O(1)) |

#### LFU Cache

| Operation | Size | Target | Maximum | Status |
|-----------|------|--------|---------|--------|
| Get (hit) | 10K items | < 5μs | < 15μs | ✅ Met (O(1)) |
| Get (miss) | 10K items | < 5μs | < 15μs | ✅ Met (O(1)) |
| Set | 10K items | < 10μs | < 30μs | ✅ Met (O(1)) |
| Eviction | 10K items | < 20μs | < 50μs | ✅ Met (O(1)) |

#### TTL Cache

| Operation | Size | Target | Maximum | Status |
|-----------|------|--------|---------|--------|
| Get (hit, valid) | 10K items | < 2μs | < 10μs | ✅ Met |
| Get (expired) | 10K items | < 5μs | < 15μs | ✅ Met |
| Set | 10K items | < 3μs | < 12μs | ✅ Met |
| Cleanup | 10K items | < 100ms | < 200ms | ✅ Met |

---

### HTTP Client Performance

| Operation | Scenario | Target | Maximum | Status |
|-----------|----------|--------|---------|--------|
| Simple GET | Local server | < 10ms | < 20ms | ✅ Met |
| POST 1MB | Local server | < 50ms | < 75ms | ✅ Met |
| Stream 10MB | Local server | < 200ms | < 300ms | ✅ Met |
| Connection pool | 100 connections | < 5ms/request | < 10ms | ✅ Met |

### Async Process Fabric (NEW)

#### Task Submission Throughput

| Scenario | Target | Maximum | Status |
|----------|--------|---------|--------|
| Submit 1k lightweight tasks | ≥ 5,000 ops/s | ≥ 4,000 ops/s | 📋 Planned |
| Submit 1k CPU-bound tasks | ≥ 1,200 ops/s | ≥ 1,000 ops/s | 📋 Planned |

**Notes:** Measure using the `AsyncProcessFabric` facade over `AsyncProcessPool`. Benchmark harness to live in `benchmarks/async_fabric_benchmarks.py` (TBD). Collect metrics: ops/s, mean execution latency, worker saturation.

#### Queue Latency

| Scenario | Target | Maximum | Status |
|----------|--------|---------|--------|
| Publish→Consume round-trip (single producer/consumer) | < 5 ms | < 10 ms | 📋 Planned |
| Publish→Consume with channel filter (3 channels) | < 8 ms | < 15 ms | 📋 Planned |

**Notes:** Exercise `session.publish()` and `session.consume()` with and without channel filters. Capture P50/P95 latency and queue depth under load.

#### Shared Memory Operations

| Scenario | Target | Maximum | Status |
|----------|--------|---------|--------|
| Segment create+write (256 KB payload) | < 2 ms | < 5 ms | 📋 Planned |
| Segment reuse (read existing payload) | < 1 ms | < 3 ms | 📋 Planned |

**Notes:** Use `session.share()` / `session.release_shared()` to validate overhead for repeated attach/detach cycles.

**Benchmark harness:** [20260210-benchmark xwsystem async fabric](../benchmarks/20260210-benchmark%20xwsystem%20async%20fabric/) — run `scripts/run_benchmark_async_fabric.py` from xwsystem root.
- Emit results to `docs/logs/benchmarks/` and update `logs/benchmarks/INDEX.md` once execution completes.
- Integrate monitoring hooks (future iteration) to capture pool utilization during runs.

### At-Rest Encryption

Benchmark script: `benchmarks/encryption_benchmark.py`. Run from xwsystem root with `PYTHONPATH=src python benchmarks/encryption_benchmark.py [-o docs/encryption_benchmark_results.md]`.

**Throughput (MB/s) and latency (ms)** — see [encryption_benchmark_results.md](encryption_benchmark_results.md) for full tables. Summary: AES-256-GCM is fastest (e.g. ~1.5–4k MB/s at 100 KiB–10 MiB), XChaCha20-Poly1305 next, Fernet slowest; latency for 64 B–1 KiB is sub-millisecond for all.

**Safety ranking (documented):** Tier 1 (highest): AES-256-GCM, XChaCha20-Poly1305 (AEAD, 256-bit). Tier 2 (high): Fernet (CBC+HMAC). KDF: Argon2id > PBKDF2 when password is used.

| Algorithm | Key size | Mode | KDF | Tier |
|-----------|----------|------|-----|------|
| aes256-gcm | 256 | AEAD | PBKDF2/Argon2id | 1 (highest) |
| xchacha20-poly1305 | 256 | AEAD | PBKDF2/Argon2id | 1 (highest) |
| fernet | 128 (effective) | CBC+HMAC | PBKDF2 | 2 (high) |

---

### Memory Performance

#### Serialization Memory Overhead

| Format | Data Size | Peak Memory | Maximum | Status |
|--------|-----------|-------------|---------|--------|
| JSON | 1MB | 2.1MB | 3MB | ✅ Met (2.1x) |
| YAML | 1MB | 2.8MB | 3MB | ⚠️ Close (2.8x) |
| MessagePack | 1MB | 1.8MB | 3MB | ✅ Met (1.8x) |
| XML | 1MB | 2.5MB | 3MB | ✅ Met (2.5x) |

**Target:** Peak memory < 3x data size  
**Maximum:** Peak memory < 5x data size

#### Cache Memory Overhead

| Cache Type | Items | Data | Overhead | Maximum | Status |
|------------|-------|------|----------|---------|--------|
| LRU | 10K | 1MB | ~200KB | 500KB | ✅ Met |
| LFU | 10K | 1MB | ~300KB | 600KB | ✅ Met |
| TTL | 10K | 1MB | ~250KB | 550KB | ✅ Met |

**Target:** Overhead < 20% of data  
**Maximum:** Overhead < 50% of data

---

### Scaling Requirements

#### Linear Scaling (O(n))

These operations must scale linearly:

| Operation | 100 items | 1K items | 10K items | 100K items | Status |
|-----------|-----------|----------|-----------|------------|--------|
| Serialize list | ~5ms | ~50ms | ~500ms | ~5s | ✅ Verified |
| Deserialize list | ~6ms | ~60ms | ~600ms | ~6s | ✅ Verified |
| Cache lookups | ~10μs | ~100μs | ~1ms | ~10ms | ✅ Verified |

**Test Method:** Measure at each scale, verify slope ≈ 1.0 on log-log plot

#### Constant Time (O(1))

These operations must be constant time:

| Operation | Any Size | Maximum | Status |
|-----------|----------|---------|--------|
| Cache get | < 5μs | < 10μs | ✅ Met |
| Cache set | < 10μs | < 20μs | ✅ Met |
| Dict lookup | < 1μs | < 5μs | ✅ Met |

---

## Non-Functional Requirements (NFRs)

### NFR-001: Serialization Speed

**Requirement:** All serializers must serialize 1MB data in < 150ms

**Rationale:** Sub-150ms feels responsive for background operations

**Verification:**
```python
def test_nfr_001_serialization_speed(benchmark):
    """NFR-001: Serialize 1MB in < 150ms"""
    data = create_1mb_data()
    stats = benchmark(serializer.dumps, data)
    assert stats.mean < 0.150  # 150ms
```

**Status:** ✅ Met - All formats under 120ms

---

### NFR-002: Memory Efficiency

**Requirement:** Peak memory < 3x data size during serialization

**Rationale:** 3x allows for reasonable overhead while preventing memory issues

**Verification:**
```python
def test_nfr_002_memory_efficiency():
    """NFR-002: Memory < 3x data size"""
    data_size = 1_000_000  # 1MB
    data = create_test_data(data_size)
    
    tracemalloc.start()
    result = serializer.dumps(data)
    peak = tracemalloc.get_traced_memory()[1]
    tracemalloc.stop()
    
    assert peak < data_size * 3
```

**Status:** ✅ Met - Max 2.8x (YAML)

---

### NFR-003: Linear Scalability

**Requirement:** Operations must scale linearly O(n) to 1M items

**Rationale:** Predictable performance for any data size

**Verification:**
```python
@pytest.mark.parametrize("size", [100, 1_000, 10_000, 100_000, 1_000_000])
def test_nfr_003_linear_scaling(benchmark, size):
    """NFR-003: Linear scaling to 1M items"""
    data = create_list(size)
    benchmark(serializer.dumps, data)
    # Analyze: Plot time vs size, verify linear
```

**Status:** ✅ Met - Confirmed O(n)

---

### NFR-004: Cache Performance

**Requirement:** Cache operations must be O(1) constant time

**Rationale:** Cache should not degrade with size

**Verification:**
```python
@pytest.mark.parametrize("size", [100, 1_000, 10_000, 100_000])
def test_nfr_004_cache_constant_time(benchmark, size):
    """NFR-004: Cache ops are O(1)"""
    cache = LRUCache(maxsize=size)
    # Fill cache
    for i in range(size):
        cache[i] = i
    
    # Benchmark get (should be constant regardless of size)
    benchmark(cache.get, size // 2)
```

**Status:** ✅ Met - O(1) confirmed

---

### NFR-005: No Regressions

**Requirement:** New versions must not be > 10% slower than baseline

**Rationale:** Prevent gradual performance degradation

**Verification:**
```bash
pytest tests/3.advance/benchmarks/ \
  --benchmark-only \
  --benchmark-compare=baseline \
  --benchmark-compare-fail=mean:10%
```

**Status:** ✅ Met - No regressions detected

---

### NFR-006: Startup Time

**Requirement:** Module import < 100ms (lite mode)

**Rationale:** Fast startup for CLI tools

**Verification:**
```python
def test_nfr_006_import_time():
    """NFR-006: Import < 100ms"""
    import timeit
    
    time = timeit.timeit(
        'import exonware.xwsystem',
        number=1
    )
    assert time < 0.100  # 100ms
```

**Status:** ✅ Met - ~15ms import time

---

### NFR-007: Concurrent Performance

**Requirement:** Thread-safe operations with < 20% overhead

**Rationale:** Support concurrent use without major slowdown

**Verification:**
```python
def test_nfr_007_concurrent_performance(benchmark):
    """NFR-007: Thread-safe overhead < 20%"""
    # Benchmark single-threaded
    single = benchmark_single_thread()
    
    # Benchmark multi-threaded (4 threads)
    multi = benchmark_multi_thread(threads=4)
    
    # Overhead should be < 20%
    overhead = (multi / single - 1.0) * 100
    assert overhead < 20
```

**Status:** 🚧 Testing - Target for v0.0.2

---

## Benchmark Environment

### Standard Test Environment

**All benchmarks should use consistent environment:**

```yaml
Python: 3.9.7
CPU: Intel i7-9700K @ 3.6GHz (or equivalent)
RAM: 16GB DDR4
OS: Windows 10 / Ubuntu 22.04 / macOS 13
Disk: SSD
Network: Local (for HTTP tests)
Load: No other applications running
```

### Environment Documentation

**Template:** See [logs/benchmarks/TEMPLATE.md](logs/benchmarks/TEMPLATE.md) for benchmark report format including environment documentation requirements.

---

## Regression Thresholds

### Performance Change Thresholds

| Change | Threshold | Action |
|--------|-----------|--------|
| **Improvement** | > 5% faster | ✅ Document and celebrate |
| **Stable** | ±5% | ✅ Acceptable variation |
| **Warning** | 5-10% slower | ⚠️ Investigate cause |
| **Regression** | > 10% slower | ❌ Must fix before merge |

### Memory Change Thresholds

| Change | Threshold | Action |
|--------|-----------|--------|
| **Improvement** | > 10% less memory | ✅ Document |
| **Stable** | ±10% | ✅ Acceptable |
| **Warning** | 10-20% more | ⚠️ Investigate |
| **Regression** | > 20% more | ❌ Must fix |

---

## Benchmark Frequency

### Required Benchmarks

| Trigger | Benchmarks | Storage |
|---------|------------|---------|
| **Before release** | Full suite | BENCH_* document |
| **Performance change** | Affected operations | BENCH_* document |
| **Major refactor** | Full suite | BENCH_* document |
| **Weekly (CI)** | Core benchmarks | logs/benchmarks/INDEX.md |

### Optional Benchmarks

| Trigger | Benchmarks | Storage |
|---------|------------|---------|
| Bug fix | Related operations | logs/benchmarks/INDEX.md |
| Dependency update | Affected operations | logs/benchmarks/INDEX.md |
| On request | Specific operations | BENCH_* if significant |

---

## SLA Evolution

**For the process of updating SLAs:** See [guides/GUIDE_BENCH.md](guides/GUIDE_BENCH.md)

Track all SLA changes in this document's history. Major SLA changes should be noted in logs/benchmarks/INDEX.md and logs/SUMMARY_PROJECT.md.

---

## Performance Testing Matrix

### Coverage Requirements

Benchmark at least these scenarios:

#### Data Sizes

- **Small:** 1KB - 100KB
- **Medium:** 100KB - 1MB
- **Large:** 1MB - 10MB
- **Extra Large:** 10MB+ (select formats only)

#### Data Types

- **Simple:** Primitives (int, str, bool)
- **Nested:** Dicts, lists (3-5 levels)
- **Complex:** Mixed types, unicode, special chars

#### Conditions

- **Cold Start:** First run (no cache)
- **Warm:** After warmup rounds
- **Concurrent:** Multiple threads
- **Sequential:** Single thread

---

## Benchmark Baselines

### Overview

Baselines are snapshots of performance at specific points in time used for:
- Detecting performance regressions
- Tracking performance improvements over time
- Comparing versions objectively

### Current Baseline

**Version:** 0.0.1.387  
**Date:** 06-Nov-2025  
**File:** `logs/benchmarks/baseline/v0.0.1_baseline.md` (to be created)

### Historical Baselines

| Version | Date | File | Notes |
|---------|------|------|-------|
| 0.0.1.350 | 01-Nov-2025 | v0.0.1.350.md | First baseline |
| 0.0.1.387 | 06-Nov-2025 | v0.0.1_baseline.md | Current (pending) |

### When to Create Baselines

1. **Major milestones** - v1.0, v2.0, etc.
2. **Minor versions** - v0.1.0, v0.2.0, etc.
3. **After major optimizations** - Significant performance changes
4. **Before releases** - Capture pre-release performance

### Baseline Naming Convention

Format: `vX.Y.Z_baseline.md` or `vX.Y.Z.BUILD_baseline.md`

**Examples:**
- `v0.0.1_baseline.md` - Version 0.0.1 baseline
- `v1.0.0_baseline.md` - Version 1.0.0 baseline
- `v0.0.1.387_baseline.md` - Specific build baseline

### Using Baselines

**Save a Baseline:**
```bash
# Run benchmarks and save
pytest tests/3.advance/benchmarks/ --benchmark-only --benchmark-save=baseline_v0.0.1

# Move to baseline directory
mv .benchmarks/baseline_v0.0.1.json docs/logs/benchmarks/baseline/
```

**Compare to Baseline:**
```bash
# Compare current performance to baseline
pytest tests/3.advance/benchmarks/ --benchmark-only --benchmark-compare=baseline_v0.0.1

# Fail if > 10% slower
pytest tests/3.advance/benchmarks/ --benchmark-only \
  --benchmark-compare=baseline_v0.0.1 \
  --benchmark-compare-fail=mean:10%
```

### Baseline Document Format

Each baseline file should include:

1. **Version Information**
   - Version number
   - Date created
   - Git commit hash (if applicable)

2. **Environment Details**
   - Hardware specs (CPU, RAM)
   - Python version
   - OS version

3. **Benchmark Results**
   - All benchmark measurements
   - Mean, min, max, stddev
   - SLA compliance status

4. **Notes**
   - Any special conditions
   - Known issues
   - Changes since last baseline

---

## Related Documentation

**Guides:**
- [guides/GUIDE_BENCH.md](guides/GUIDE_BENCH.md) - How to write benchmarks
- [guides/GUIDE_TEST.md](guides/GUIDE_TEST.md) - Testing standards
- [guides/GUIDE_PLAN.md](guides/GUIDE_PLAN.md) - Development flow (Phase IV benchmarking)

**History:**
- [logs/benchmarks/INDEX.md](logs/benchmarks/INDEX.md) - Benchmark execution history
- [logs/benchmarks/](logs/benchmarks/) - Detailed benchmark results

**Requirements:**
- [REF_22_PROJECT.md](REF_22_PROJECT.md) - NFRs section

---

## Quick Reference

### SLA Summary

| Category | Key Metric | Target | Maximum |
|----------|-----------|--------|---------|
| **JSON Serialize** | 1MB | < 45ms | < 50ms |
| **JSON Deserialize** | 1MB | < 50ms | < 55ms |
| **MessagePack** | 1MB | < 25ms | < 30ms |
| **Cache Get** | Any | < 1μs | < 5μs |
| **Memory** | 1MB data | < 2.5MB | < 3MB |
| **Scaling** | To 1M items | O(n) | O(n) |

### Regression Limits

- **Performance:** Max 10% slower
- **Memory:** Max 20% more
- **Action:** Fix before merge

---

*Measure performance, meet SLAs, prevent regressions*



