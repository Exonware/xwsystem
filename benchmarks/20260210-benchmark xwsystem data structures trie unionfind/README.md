# 20260210 — Benchmark: xwsystem Data Structures (Trie, UnionFind)

**Campaign:** TrieNode-based trie and UnionFind throughput  
**Date:** 10-Feb-2026  
**Project:** exonware/xwsystem  
**Guide:** [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)

---

## Goal

Measure **throughput (ops/s)** of xwsystem **TrieNode**-based trie (insert, lookup) and **UnionFind** (make_set, find, union) vs plain **dict**/set for comparable operations.

---

## Description

- **Trie:** Build trie from TrieNode; insert keys; lookup keys; optional prefix walk
- **UnionFind:** make_set for N elements; repeated union and find
- **Comparison:** dict lookup/set membership where applicable

---

## Metrics

- **Throughput:** operations per second

---

## Structure

- **scripts/** — `run_benchmark_data_structures.py`
- **data/** — optional key lists
- **benchmarks/** — BENCH_* reports, results_*.json, INDEX.md

---

## How to Run

From xwsystem root:

```bash
python "benchmarks/20260210-benchmark xwsystem data structures trie unionfind/scripts/run_benchmark_data_structures.py"
```

---

## Results

**Last run:** 09-Feb-2026 (BENCH_20260209_225437)

| Structure | ops/sec |
|-----------|--------|
| trie_lookup (TrieNode) | 899,135 |
| dict_lookup | 26,096,025 |
| trie_insert (TrieNode batch) | 48,629 |
| UnionFind (make_set+union+find) | 573 |

**Report:** [benchmarks/BENCH_20260209_225437_data_structures.md](benchmarks/BENCH_20260209_225437_data_structures.md) · **Data:** [benchmarks/results_20260209_225437.json](benchmarks/results_20260209_225437.json)

---

## Related

- [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)
- [../INDEX.md](../INDEX.md)
