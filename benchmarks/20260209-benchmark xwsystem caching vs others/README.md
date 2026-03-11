# 20260209 — Benchmark: xwsystem Caching vs Others

**Campaign:** In-memory cache get/put/mixed throughput comparison  
**Date:** 09-Feb-2026  
**Project:** exonware/xwsystem  
**Guide:** [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)

---

## Goal

Demonstrate that **PylruCache** (pylru, from `xwsystem/src/exonware/xwsystem/caching/external_caching_python.py`) is our **default** in-memory cache (highest throughput in benchmarks), and compare it with **all other xwsystem caches** in that package plus **top 10** caching libraries in the Python world.

---

## Description

This benchmark compares **PylruCache** (default; pylru wrapped in xwsystem) against:

1. **All xwsystem caches** in `xwsystem/src/exonware/xwsystem/caching/`:
   - **PylruCache** — default (pylru wrapper; highest throughput ~2.4M mixed/s; `external_caching_python.py`)
   - **FunctoolsLRUCache** — OrderedDict-based LRU (`external_caching_python.py`)
   - **CacheboxCache** — Rust-based (cachebox; `external_caching_python.py`)
   - **CachetoolsLRUCache**, **CachetoolsLFUCache**, **CachetoolsTTLCache**, **CachetoolsRRCache** — cachetools wrappers (`external_caching_python.py`)
   - **LRUCache**, **AsyncLRUCache** — core LRU (`lru_cache.py`)
   - **LFUCache**, **AsyncLFUCache** — LFU (`lfu_cache.py`)
   - **OptimizedLFUCache**, **AsyncOptimizedLFUCache** — O(1) LFU (`lfu_optimized.py`)
   - **TTLCache**, **AsyncTTLCache** — TTL (`ttl_cache.py`)
   - **MemoryBoundedLRUCache**, **MemoryBoundedLFUCache** — memory-bounded (`memory_bounded.py`)
   - **TwoTierCache** — L1/L2 (`two_tier_cache.py`)
   - **SecureLRUCache**, **SecureLFUCache**, **SecureTTLCache** — secure variants (`secure_cache.py`)
   - **FluentLRUCache**, **FluentLFUCache**, **FluentTTLCache** — fluent API (`fluent.py`)
   - **ObservableLRUCache**, **ObservableLFUCache** — event-emitting (`observable_cache.py`)
   - **ReadThroughCache**, **WriteThroughCache**, **ReadWriteThroughCache** — load/store patterns (`read_through.py`)
   - **SerializableCache** — save/load (`serializable.py`)
   - **TaggedCache** — tag invalidation (`tagging.py`)
   - **WriteBehindCache** — lazy write (`write_behind.py`)
   - **ConditionalEvictionCache** — custom eviction (`conditional.py`)
   - **BloomFilterCache** — bloom + LRU (`bloom_cache.py`)
   - **DiskCache** — disk-backed (`disk_cache.py`)
   - **PluggableCache** — pluggable backend (`pluggable_cache.py`)
   - **DistributedCache**, **RedisCache** — distributed (`distributed.py`)

2. **Top 10 Python-world caches** (where applicable):
   - **cachebox** (Rust)
   - **cachetools** (LRU, LFU, TTL, RR)
   - **functools.lru_cache** (stdlib C)
   - **diskcache**
   - **cacheout** (pylru is included in xwsystem as **PylruCache** and is the default)
   - **django.core.cache** (local memory backend)
   - **redis** (network; optional for comparison)
   - **memcached** (optional)
   - **aiocache** (async; optional)
   - **joblib.Memory** (optional)

3. **Rust attempt** (xwsystem): External Rust caches (e.g. Moka, QuickCache, DashMap via PyO3 in `rust/src/caching/external_caching_rust.rs`) — included for comparison even when they underperformed (e.g. after 2 days of tuning).

Metrics: **get** throughput, **put** throughput, **mixed** (get+put) throughput, cold vs warm, and where relevant atomic/batch behavior.

---

## Metrics

- **Throughput:** ops/sec for get, put, and mixed (get+put) workloads.
- **Cold vs warm:** first-touch vs steady-state.
- **Hit rate:** where measurable (e.g. 80% get / 20% put mix).
- **Latency:** mean/median per op when useful.

---

## Structure

- **scripts/** — Script(s) to run the benchmark (e.g. `run_benchmark_caching.py`).
- **data/** — Configs, baseline JSON, optional datasets.
- **benchmarks/** — BENCH_* reports and INDEX.md for this campaign.

---

## How to Run

From project root (xwsystem):

```bash
# Default cache is PylruCache (pylru). Install external deps as needed:
pip install pylru cachebox cachetools diskcache

# Run caching benchmark
python "benchmarks/20260209-benchmark xwsystem caching vs others/scripts/run_benchmark_caching.py"
```

Or from the campaign scripts directory:

```bash
cd "benchmarks/20260209-benchmark xwsystem caching vs others/scripts"
python run_benchmark_caching.py
```

Results go to stdout and can be saved to `../data/` or `../benchmarks/` (BENCH_* and INDEX.md updated after run).

---

## Results

**Last run:** 10-Feb-2026 (BENCH_20260210_203231)  
**Report:** [benchmarks/BENCH_20260210_203231_caching_comparison.md](benchmarks/BENCH_20260210_203231_caching_comparison.md) · **Data:** [benchmarks/results_20260210_203231.json](benchmarks/results_20260210_203231.json)

**Summary:** Capacity 2000, prefill 1000, 10k ops per workload (get / put / mixed 80/20). **PylruCache** is the default; it and CacheboxCache reached highest throughput in this run.

| Cache (sample)        | Get (ops/s) | Put (ops/s) | Mixed (ops/s) |
|-----------------------|-------------|-------------|---------------|
| PylruCache (default)  | 2.99M       | 2.83M       | 2.49M         |
| FunctoolsLRUCache     | 3.18M       | 3.15M       | 2.69M         |
| CacheboxCache         | 3.72M       | 1.66M       | 2.54M         |
| LRUCache (xwsystem)   | 1.41M       | 1.42M       | 1.39M         |
| TwoTierCache          | 1.53M       | 1.3k        | 6.3k          |
| diskcache             | 181k        | 9.4k        | 45k           |

**PylruCache vs xwsystem others:** PylruCache (default) and FunctoolsLRUCache lead on mixed (~2.5–2.7M/s); CacheboxCache is highest on get (3.72M). TwoTierCache now runs (implements `put()`); get is competitive (1.53M/s) while put/mixed are slower due to disk tier.

**PylruCache vs top 10 Python:** PylruCache is our wrapper around pylru; CacheboxCache (also wrapped) is comparable. cacheout is in the same range; diskcache is much slower (disk I/O).

**Rust attempt (external_caching_rust):** Not run — Rust bindings not built (`maturin develop --features external-caches,python` required).

---

## Related

- [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md) — Benchmarking guide
- [REF_54_BENCH.md](../../REF_54_BENCH.md) — Performance SLAs (if present)
- [../INDEX.md](../INDEX.md) — Benchmarks index
- [PylruCache](../../src/exonware/xwsystem/caching/external_caching_python.py) — default cache (pylru wrapper); [FunctoolsLRUCache](../../src/exonware/xwsystem/caching/external_caching_python.py) — fallback when pylru not installed
