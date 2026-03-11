# BENCH 20260209_225242 — Caching comparison

## Summary

| Cache | Group | Get (ops/s) | Put (ops/s) | Mixed (ops/s) |
|-------|-------|-------------|-------------|---------------|
| PylruCache (default) | xwsystem | 1953697.38 | 2201237.14 | 1856251.84 |
| FunctoolsLRUCache | xwsystem | 1084516.36 | 1205240.37 | 1458938.18 |
| CacheboxCache | xwsystem | 2988911.05 | 925386.11 | 1685345.91 |
| CachetoolsLRUCache | xwsystem | 954936.55 | 441949.88 | 596477.2 |
| CachetoolsLFUCache | xwsystem | 480392.77 | 407118.05 | 493454.33 |
| CachetoolsTTLCache | xwsystem | 347480.42 | 201603.96 | 276212.57 |
| CachetoolsRRCache | xwsystem | 1292507.33 | 447954.42 | 785836.1 |
| LRUCache | xwsystem | 483718.05 | 482739.64 | 480459.71 |
| LFUCache | xwsystem | 770665.39 | 838771.37 | 928212.08 |
| OptimizedLFUCache | xwsystem | 466324.39 | 428654.71 | 434776.94 |
| TTLCache | xwsystem | 631556.35 | 125712.32 | 40633.77 |
| MemoryBoundedLRUCache | xwsystem | 705243.49 | 209707.35 | 463430.69 |
| MemoryBoundedLFUCache | xwsystem | 478082.32 | 280358.63 | 385405.47 |
| FluentLRUCache | xwsystem | 571810.87 | 495329.05 | 568886.46 |
| ObservableLRUCache | xwsystem | 366536.67 | 318215.95 | 288230.95 |
| TwoTierCache | xwsystem | — | — | 'TwoTierCache' object has no attribute 'put' |
| diskcache | external | 85049.49 | 1408.18 | 7948.14 |
| cacheout.LRUCache | external | 530197.39 | 710878.58 | 730636.31 |
| pylru | external | 1457364.8 | 1524041.78 | 1518879.66 |
| Rust (Moka/QuickCache/DashMap) | rust | — | — | rust bindings not built (maturin develop --features external-caches,python) |

## Raw data

- Full results: [results_20260209_225242.json](results_20260209_225242.json)

- Legacy baseline: [data/baseline/_legacy/baseline_v0.0.1.387.json](../data/baseline/_legacy/baseline_v0.0.1.387.json)
