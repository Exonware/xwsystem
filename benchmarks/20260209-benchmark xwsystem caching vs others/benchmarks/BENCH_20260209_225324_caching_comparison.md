# BENCH 20260209_225324 — Caching comparison

## Summary

| Cache | Group | Get (ops/s) | Put (ops/s) | Mixed (ops/s) |
|-------|-------|-------------|-------------|---------------|
| PylruCache (default) | xwsystem | 1481810.76 | 1638511.57 | 1451737.01 |
| FunctoolsLRUCache | xwsystem | 1689046.52 | 1674845.51 | 701419.67 |
| CacheboxCache | xwsystem | 1236139.78 | 474809.01 | 894870.6 |
| CachetoolsLRUCache | xwsystem | 694743.57 | 477320.13 | 477081.03 |
| CachetoolsLFUCache | xwsystem | 329533.81 | 278009.45 | 226527.76 |
| CachetoolsTTLCache | xwsystem | 250272.8 | 148262.58 | 204151.21 |
| CachetoolsRRCache | xwsystem | 821989.88 | 347166.77 | 788575.12 |
| LRUCache | xwsystem | 529725.55 | 486272.53 | 371765.18 |
| LFUCache | xwsystem | 761185.62 | 624711.07 | 857647.65 |
| OptimizedLFUCache | xwsystem | 510514.04 | 404350.82 | 357665.3 |
| TTLCache | xwsystem | 550830.65 | 187197.91 | 24851.28 |
| MemoryBoundedLRUCache | xwsystem | 499877.53 | 102147.13 | 235691.74 |
| MemoryBoundedLFUCache | xwsystem | 371250.37 | 175677.46 | 313285.17 |
| FluentLRUCache | xwsystem | 272919.06 | 351915.65 | 241677.24 |
| ObservableLRUCache | xwsystem | 162653.73 | 203861.96 | 157211.94 |
| TwoTierCache | xwsystem | — | — | 'TwoTierCache' object has no attribute 'put' |
| diskcache | external | 27167.62 | 2049.54 | 10356.77 |
| cacheout.LRUCache | external | 282440.4 | 400533.51 | 245846.42 |
| pylru | external | 1113759.38 | 1136725.33 | 1038227.54 |
| Rust (Moka/QuickCache/DashMap) | rust | — | — | rust bindings not built (maturin develop --features external-caches,python) |

## Raw data

- Full results: [results_20260209_225324.json](results_20260209_225324.json)

- Legacy baseline: [data/baseline/_legacy/baseline_v0.0.1.387.json](../data/baseline/_legacy/baseline_v0.0.1.387.json)
