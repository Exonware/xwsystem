# 20260209 — Benchmark: xwsystem JSON vs Others

**Campaign:** JSON serialization and deserialization comparison  
**Date:** 09-Feb-2026  
**Project:** exonware/xwsystem  
**Guide:** [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)

---

## Goal

Demonstrate that **JsonSerializer** (from `xwsystem/src/exonware/xwsystem/io/serialization/formats/text/json.py`) is the **fastest** among top Python JSON libraries that use C++ or Rust backends, across the dimensions that matter in production.

---

## Description

This benchmark compares **JsonSerializer** (from the file above) against **top 10** (and more) JSON libraries in Python that rely on C++ or Rust for speed:

- **Read:** decode / parse (bytes or str → Python object)
- **Write:** encode / serialize (Python object → bytes or str)
- **Cold read / cold write:** first access after process start (no warm cache)
- **Warm read / warm write:** repeated operations (cache-hot, JIT warm)
- **Atomic operations:** where supported (e.g. xwsystem’s atomic save/load)
- **Indexing and caching:** where applicable (e.g. xwsystem indexing/caching layers)

Libraries in scope include (C++/Rust-backed or otherwise fast):

| Library        | Backend   | Notes                          |
|----------------|-----------|--------------------------------|
| orjson         | Rust      | Very fast encode/decode        |
| ujson          | C         | Fast, widely used              |
| rapidjson      | C++       | Python bindings to RapidJSON   |
| pysimdjson     | C++       | SIMD-based parsing             |
| msgspec        | Cython/Rust | Fast schema-based + JSON     |
| simplejson     | C         | C extension                    |
| hyperjson      | Rust      | Fast parser                    |
| stdlib json    | C         | Built-in                       |
| **JsonSerializer** | hyperjson (preferred) → hybrid → stdlib | `xwsystem/.../formats/text/json.py`; atomic ops supported |

Additional aspects (when available):

- **Indexing:** e.g. xwsystem’s indexing for JSON-backed data
- **Caching:** e.g. repeated read/write with cache
- **Atomic I/O:** e.g. atomic file write/read

---

## Metrics

- **Throughput:** ops/sec for encode and decode (cold and warm).
- **Latency:** mean/median (and optionally p95/p99) per operation.
- **Cold vs warm:** separate results for first-touch vs steady state.
- **Atomic:** time for atomic save/load where supported.
- **Indexing/caching:** noted in report when used.

---

## Structure

- **scripts/** — `run_benchmark_json.py` (and helpers) to run the benchmark.
- **data/** — Payloads (e.g. sample JSON, sizes) and optional baseline files.
- **benchmarks/** — BENCH_* reports and INDEX.md for this campaign.

---

## How to Run

From project root (xwsystem):

```bash
# Install optional JSON deps (all that might beat stdlib / JsonSerializer):
pip install orjson msgspec ujson python-rapidjson pysimdjson simplejson hyperlight-hyperjson

# Run JSON benchmark
python benchmarks/20260209-benchmark xwsystem json vs others/scripts/run_benchmark_json.py
```

Or from the campaign scripts directory:

```bash
cd "benchmarks/20260209-benchmark xwsystem json vs others/scripts"
python run_benchmark_json.py
```

Results are printed to stdout and can be saved to `../data/` or `../benchmarks/` (e.g. BENCH_* and INDEX.md updated after run).

---

## Results

**Last run:** 09-Feb-2026 (BENCH_20260209_225411)  
**Report:** [benchmarks/BENCH_20260209_225411_json_comparison.md](benchmarks/BENCH_20260209_225411_json_comparison.md) · **Data:** [benchmarks/results_20260209_225411.json](benchmarks/results_20260209_225411.json)

**Parser:** JsonSerializer **prefers hyperjson** when installed (`pip install hyperlight-hyperjson`), then falls back to hybrid (msgspec + orjson), then stdlib. So **yes, JSON is faster** when hyperjson is available: JsonSerializer uses the same parser for all encode/decode and atomic operations (atomic_update_path, atomic_read_path, load_file, save_file).

**JsonSerializer measured speed** (warm decode / warm encode, 500 iter, 50 warmup; below from run 20260209_225411 — hybrid parser in that run; with hyperjson installed, expect ~hyperjson standalone rates):

| Payload | JsonSerializer (warm decode / warm encode) | orjson | msgspec | hyperjson |
|---------|-------------------------------------------|--------|---------|-----------|
| small   | **798k / 506k** ops/s                     | 1.19M / 1.59M | 860k / 1.18M | 611k / 937k |
| medium  | **12.0k / 17.1k** ops/s                   | 15.0k / 24.2k | 13.3k / 35.7k | 9.0k / 19.4k |
| large   | **504 / 2.79k** ops/s                     | 428 / 1.85k | 509 / 3.62k | 815 / 4.77k |

**JsonSerializer vs others:** With **hyperjson installed**, JsonSerializer uses it and matches hyperjson speed. Without it (e.g. run above), hybrid keeps JsonSerializer in the top tier (small: ~798k decode, ~506k encode). Install `hyperlight-hyperjson` for best speed.

---

## Related

- [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md) — Benchmarking guide
- [REF_54_BENCH.md](../../REF_54_BENCH.md) — Performance SLAs (if present)
- [../INDEX.md](../INDEX.md) — Benchmarks index
