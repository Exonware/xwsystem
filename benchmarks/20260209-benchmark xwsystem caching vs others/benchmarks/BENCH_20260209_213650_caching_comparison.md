# BENCH 20260209_213650 — Caching comparison

## Summary

| Cache | Group | Get (ops/s) | Put (ops/s) | Mixed (ops/s) |
|-------|-------|-------------|-------------|---------------|
| FunctoolsLRUCache (default) | xwsystem | 1885049.67 | 1568799.71 | 1931210.3 |
| CacheboxCache | xwsystem | 2802847.75 | 1177398.65 | 2049180.37 |
| CachetoolsLRUCache | xwsystem | 1549402.7 | 774251.49 | 1085010.58 |
| CachetoolsLFUCache | xwsystem | 1090203.43 | 768669.05 | 852347.8 |
| CachetoolsTTLCache | xwsystem | 628622.43 | 353639.3 | 451648.75 |
| CachetoolsRRCache | xwsystem | 1874730.51 | 1079400.72 | 1682736.81 |
| LRUCache | xwsystem | 998073.71 | 1072995.92 | 833861.45 |
| LFUCache | xwsystem | 1341975.65 | 1741007.67 | 1499407.75 |
| OptimizedLFUCache | xwsystem | 741322.82 | 900487.16 | 749776.95 |
| TTLCache | xwsystem | 1035486.1 | 161116.73 | 71258.31 |
| MemoryBoundedLRUCache | xwsystem | 768161.25 | 266118.82 | 723081.48 |
| MemoryBoundedLFUCache | xwsystem | 851136.27 | 564831.34 | 516558.27 |
| FluentLRUCache | xwsystem | 845165.65 | 938570.56 | 821996.63 |
| ObservableLRUCache | xwsystem | 601091.58 | 590151.55 | 524317.86 |
| TwoTierCache | xwsystem | — | — | 'TwoTierCache' object has no attribute 'put' |
| diskcache | external | 112733.6 | 3115.85 | 14710.18 |
| cacheout.LRUCache | external | 804421.1 | 913984.88 | 753539.75 |
| pylru | external | 2709439.74 | 2401479.35 | 2357767.7 |
| Rust (Moka/QuickCache/DashMap) | rust | — | — | rust bindings not built (maturin develop --features external-caches,python) |

## Raw data

- Full results: [results_20260209_213650.json](results_20260209_213650.json)

- Legacy baseline: [data/baseline/_legacy/baseline_v0.0.1.387.json](../data/baseline/_legacy/baseline_v0.0.1.387.json)
