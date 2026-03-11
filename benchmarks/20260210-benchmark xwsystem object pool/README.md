# 20260210 — Benchmark: xwsystem Object Pool

**Campaign:** ObjectPool acquire/release vs direct instantiation  
**Date:** 10-Feb-2026  
**Project:** exonware/xwsystem  
**Guide:** [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)

---

## Goal

Measure **throughput (ops/s)** of **ObjectPool** get + release (thread-safe and non–thread-safe) vs **direct instantiation** of the same type.

---

## Description

- **Pool:** get(obj_type) then release(obj) in a loop
- **Direct:** obj_type() in a loop
- **Config:** fixed max_size, num_ops

---

## Metrics

- **Throughput:** acquire+release or instantiate per second

---

## Structure

- **scripts/** — `run_benchmark_object_pool.py`
- **data/** — not required
- **benchmarks/** — BENCH_* reports, results_*.json, INDEX.md

---

## How to Run

From xwsystem root:

```bash
python "benchmarks/20260210-benchmark xwsystem object pool/scripts/run_benchmark_object_pool.py"
```

---

## Results

**Last run:** 09-Feb-2026 (BENCH_20260209_225437)

| Method | ops/sec |
|--------|--------|
| ObjectPool get+release (thread_safe=True) | 350,095 |
| ObjectPool get+release (thread_safe=False) | 287,462 |
| direct SimpleObj() | 2,932,121 |

**Report:** [benchmarks/BENCH_20260209_225437_object_pool.md](benchmarks/BENCH_20260209_225437_object_pool.md) · **Data:** [benchmarks/results_20260209_225437.json](benchmarks/results_20260209_225437.json)

---

## Related

- [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)
- [../INDEX.md](../INDEX.md)
