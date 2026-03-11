# 20260210 — Benchmark: xwsystem Atomic File I/O vs Plain Write

**Campaign:** Atomic file write + commit vs plain open/write/close  
**Date:** 10-Feb-2026  
**Project:** exonware/xwsystem  
**Guide:** [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)

---

## Goal

Measure throughput and latency of **AtomicFileWriter** (write + commit) vs **plain file write** (open, write, close) for the same payload sizes to quantify the cost of atomic, crash-safe writes.

---

## Description

- **Atomic:** xwsystem `AtomicFileWriter` (temp file + move to target)
- **Plain:** built-in `open(path, "wb").write(data); close`
- **Payload sizes:** 1 KB, 100 KB, 1 MB (binary)
- **Temp dir:** under campaign `data/` or system temp

---

## Metrics

- **Throughput:** writes per second (ops/s)
- **Latency:** mean time per write (optional)

---

## Structure

- **scripts/** — `run_benchmark_atomic_io.py`
- **data/** — used for temp/target files during run
- **benchmarks/** — BENCH_* reports, results_*.json, INDEX.md

---

## How to Run

From xwsystem root:

```bash
python "benchmarks/20260210-benchmark xwsystem atomic io/scripts/run_benchmark_atomic_io.py"
```

Or from the campaign scripts directory:

```bash
cd "benchmarks/20260210-benchmark xwsystem atomic io/scripts"
python run_benchmark_atomic_io.py
```

---

## Results

**Last run:** 09-Feb-2026 (BENCH_20260209_225334)

| Method | Size | Writes/sec |
|--------|------|------------|
| AtomicFileWriter | 1 KB | 205 |
| plain write | 1 KB | 743 |
| AtomicFileWriter | 100 KB | 357 |
| plain write | 100 KB | 504 |
| AtomicFileWriter | 1 MB | 231 |
| plain write | 1 MB | 344 |

**Report:** [benchmarks/BENCH_20260209_225334_atomic_io.md](benchmarks/BENCH_20260209_225334_atomic_io.md) · **Data:** [benchmarks/results_20260209_225334.json](benchmarks/results_20260209_225334.json)

---

## Related

- [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)
- [../INDEX.md](../INDEX.md)
