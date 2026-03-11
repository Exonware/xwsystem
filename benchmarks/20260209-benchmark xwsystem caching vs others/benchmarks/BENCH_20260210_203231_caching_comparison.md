# BENCH 20260210_203231 — Caching comparison

## Summary

| Cache | Group | Get (ops/s) | Put (ops/s) | Mixed (ops/s) |
|-------|-------|-------------|-------------|---------------|
| PylruCache (default) | xwsystem | 2988196.62 | 2833583.64 | 2490101.84 |
| FunctoolsLRUCache | xwsystem | 3184003.56 | 3145742.23 | 2689690.42 |
| CacheboxCache | xwsystem | 3718854.59 | 1661902.55 | 2539102.17 |
| CachetoolsLRUCache | xwsystem | 2191060.47 | 1494343.91 | 1924150.01 |
| CachetoolsLFUCache | xwsystem | 1908469.79 | 1196172.25 | 1261209.0 |
| CachetoolsTTLCache | xwsystem | 914620.2 | 587582.04 | 847931.89 |
| CachetoolsRRCache | xwsystem | 3456261.02 | 1603386.35 | 2641798.54 |
| LRUCache | xwsystem | 1410457.13 | 1420212.46 | 1394233.45 |
| LFUCache | xwsystem | 2347472.94 | 2446363.48 | 2288905.67 |
| OptimizedLFUCache | xwsystem | 1277694.02 | 1385790.11 | 1166738.62 |
| TTLCache | xwsystem | 1476755.86 | 210304.06 | 94447.61 |
| MemoryBoundedLRUCache | xwsystem | 1316846.42 | 481319.97 | 1068022.34 |
| MemoryBoundedLFUCache | xwsystem | 1239357.02 | 805756.32 | 1093912.38 |
| FluentLRUCache | xwsystem | 1611915.28 | 1510163.4 | 1381081.94 |
| ObservableLRUCache | xwsystem | 793808.3 | 881127.14 | 770238.0 |
| TwoTierCache | xwsystem | 1534919.42 | 1270.26 | 6260.66 |
| diskcache | external | 180807.63 | 9360.4 | 45142.98 |
| cacheout.LRUCache | external | 1073560.36 | 1121189.36 | 904396.27 |
| pylru | external | 2378404.09 | 2911462.42 | 2507711.22 |
| Rust (Moka/QuickCache/DashMap) | rust | — | — | rust bindings not built (maturin develop --features external-caches,python) |

## Raw data

- Full results: [results_20260210_203231.json](results_20260210_203231.json)

- Legacy baseline: [data/baseline/_legacy/baseline_v0.0.1.387.json](../data/baseline/_legacy/baseline_v0.0.1.387.json)
