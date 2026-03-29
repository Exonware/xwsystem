# 20260210 - Benchmark: xwsystem Operations (Diff, Merge, Patch)

**Campaign:** Throughput of DiffOperation, MergeOperation, PatchOperation  
**Date:** 10-Feb-2026  
**Project:** exonware/xwsystem  
**Guide:** [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)

---

## Goal

Measure **throughput (ops/s)** of xwsystem operations: **DiffOperation** (structural/content/full), **MergeOperation** (deep/shallow/overwrite), and **PatchOperationImpl** (apply_patch) on small, medium, and large nested structures.

---

## Description

- **Diff:** diff(original, modified, mode) for DiffMode.STRUCTURAL, CONTENT, FULL
- **Merge:** merge(target, source, strategy) for MergeStrategy.DEEP, SHALLOW, OVERWRITE
- **Patch:** apply_patch(data, operations) with RFC 6902-style ops
- **Fixtures:** nested dict/list (small/medium/large)

---

## Metrics

- **Throughput:** operations per second

---

## Structure

- **scripts/** - `run_benchmark_operations.py`
- **data/** - optional fixture JSON
- **benchmarks/** - BENCH_* reports, results_*.json, INDEX.md

---

## How to Run

From xwsystem root:

```bash
python "benchmarks/20260210-benchmark xwsystem operations diff merge patch/scripts/run_benchmark_operations.py"
```

---

## Results

**Last run:** 09-Feb-2026 (BENCH_20260209_225415)

| Operation | Payload | ops/sec |
|-----------|---------|--------|
| diff_structural | small | 35,561 |
| diff_content | small | 48,212 |
| diff_full | small | 45,365 |
| merge_deep | small | 69,720 |
| merge_shallow | small | 58,814 |
| merge_overwrite | small | 66,399 |
| diff_structural | medium | 1,178 |
| diff_content | medium | 975 |
| diff_full | medium | 1,324 |
| merge_deep | medium | 1,286 |
| merge_shallow | medium | 1,012 |
| merge_overwrite | medium | 1,146 |
| diff_structural | large | 60 |
| diff_content | large | 75 |
| diff_full | large | 73 |
| merge_deep | large | 82 |
| merge_shallow | large | 72 |
| merge_overwrite | large | 67 |
| patch_apply | small | 63,872 |

**Report:** [benchmarks/BENCH_20260209_225415_operations.md](benchmarks/BENCH_20260209_225415_operations.md) · **Data:** [benchmarks/results_20260209_225415.json](benchmarks/results_20260209_225415.json)

---

## Related

- [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)
- [../INDEX.md](../INDEX.md)
