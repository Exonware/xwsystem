# BENCH 20260209_212735 — Caching comparison

## Summary

| Cache | Group | Get (ops/s) | Put (ops/s) | Mixed (ops/s) |
|-------|-------|-------------|-------------|---------------|
| FunctoolsLRUCache (default) | xwsystem | 1549786.9 | 1398034.37 | 1351862.87 |
| CacheboxCache | xwsystem | 2677734.61 | 801371.95 | 1428326.57 |
| CachetoolsLRUCache | xwsystem | 841502.92 | 655686.11 | 1104191.52 |
| CachetoolsLFUCache | xwsystem | 696582.57 | 663878.38 | 741059.12 |
| CachetoolsTTLCache | xwsystem | 505397.65 | 259092.19 | 440059.5 |
| CachetoolsRRCache | xwsystem | 1485663.34 | 988288.78 | 1157675.39 |
| LRUCache | xwsystem | 832604.8 | 658128.55 | 844494.74 |
| LFUCache | xwsystem | 1263743.22 | 1357496.78 | 1489048.07 |
| OptimizedLFUCache | xwsystem | 708611.76 | 862336.59 | 607485.44 |
| TTLCache | xwsystem | 938676.28 | 153226.88 | 55387.82 |
| MemoryBoundedLRUCache | xwsystem | 742445.62 | 258321.84 | 451899.79 |
| MemoryBoundedLFUCache | xwsystem | 705542.03 | 422143.98 | 540663.29 |
| FluentLRUCache | xwsystem | 703274.45 | 791746.83 | 806809.47 |
| ObservableLRUCache | xwsystem | 492734.63 | 601623.18 | 466413.56 |
| TwoTierCache | xwsystem | — | — | 'TwoTierCache' object has no attribute 'put' |
| diskcache | external | 109805.76 | 6068.28 | 30075.87 |
| cacheout.LRUCache | external | 620609.19 | 750739.48 | 567118.47 |
| pylru | external | 1487541.84 | 1766722.02 | 2028973.78 |
| Rust (Moka/QuickCache/DashMap) | rust | — | — | rust bindings not built (maturin develop --features external-caches,python) |

## Raw data

- Full results: [results_20260209_212735.json](results_20260209_212735.json)

- Legacy baseline: [data/baseline/_legacy/baseline_v0.0.1.387.json](../data/baseline/_legacy/baseline_v0.0.1.387.json)
