# 20260210 — Benchmark: xwsystem Validation

**Campaign:** check_data_depth and validate_path_input throughput  
**Date:** 10-Feb-2026  
**Project:** exonware/xwsystem  
**Guide:** [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)

---

## Goal

Measure **throughput (calls/s)** of xwsystem validation: **check_data_depth** (deep vs shallow structures) and **validate_path_input** (safe path strings).

---

## Description

- **check_data_depth:** data structure depth check; fixtures: shallow and deep nested dict/list
- **validate_path_input:** path string validation; safe paths only (no exceptions)

---

## Metrics

- **Throughput:** validations per second

---

## Structure

- **scripts/** — `run_benchmark_validation.py`
- **data/** — optional
- **benchmarks/** — BENCH_* reports, results_*.json, INDEX.md

---

## How to Run

From xwsystem root:

```bash
python "benchmarks/20260210-benchmark xwsystem validation/scripts/run_benchmark_validation.py"
```

---

## Results

**Last run:** 09-Feb-2026 (BENCH_20260209_225438)

| Check | ops/sec |
|-------|--------|
| check_data_depth (shallow) | 1,108,672 |
| check_data_depth (deep) | 229,014 |
| validate_path_input (safe) | 2,583,779 |

**Report:** [benchmarks/BENCH_20260209_225438_validation.md](benchmarks/BENCH_20260209_225438_validation.md) · **Data:** [benchmarks/results_20260209_225438.json](benchmarks/results_20260209_225438.json)

---

## Related

- [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)
- [../INDEX.md](../INDEX.md)
