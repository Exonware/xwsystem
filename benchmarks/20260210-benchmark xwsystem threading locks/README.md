# 20260210 - Benchmark: xwsystem Threading Locks

**Campaign:** EnhancedRLock vs stdlib RLock throughput  
**Date:** 10-Feb-2026  
**Project:** exonware/xwsystem  
**Guide:** [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)

---

## Goal

Measure **throughput (acquire+release/s)** of **EnhancedRLock** vs **threading.RLock** in single-thread and optionally under contention.

---

## Description

- **Single-thread:** repeated with lock: pass (acquire + release)
- **Comparison:** xwsystem EnhancedRLock vs stdlib RLock

---

## Metrics

- **Throughput:** acquire+release cycles per second

---

## Structure

- **scripts/** - `run_benchmark_locks.py`
- **data/** - not required
- **benchmarks/** - BENCH_* reports, results_*.json, INDEX.md

---

## How to Run

From xwsystem root:

```bash
python "benchmarks/20260210-benchmark xwsystem threading locks/scripts/run_benchmark_locks.py"
```

---

## Results

**Last run:** 10-Feb-2026 (BENCH_20260210_203220)

| Lock | ops/sec |
|------|--------|
| EnhancedRLock (track_stats=True) | 1,530,585 |
| EnhancedRLock (track_stats=False) | 3,368,977 |
| threading.RLock (with) | 7,139,593 |

**Report:** [benchmarks/BENCH_20260210_203220_locks.md](benchmarks/BENCH_20260210_203220_locks.md) · **Data:** [benchmarks/results_20260210_203220.json](benchmarks/results_20260210_203220.json)

Use `EnhancedRLock(track_stats=False)` for maximum throughput when timeout/stats are not needed (see REF_54_BENCH trade-offs).

---

## Related

- [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)
- [../INDEX.md](../INDEX.md)
