# BENCH 20260209_224805 — Serialization comparison (MsgPack, YAML, Pickle)

## Summary

| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |
|---------|---------|--------------|-------------|-------------|-------------|
| MsgPackSerializer (small) | — | 509943.96 | 1100110.09 | 504286.44 | 386249.53 |
| MsgPackSerializer (medium) | — | 11134.99 | 12330.15 | 13823.04 | 14008.74 |
| MsgPackSerializer (large) | — | 974.31 | 1037.81 | 812.09 | 561.99 |
| msgpack (small) | — | 1297016.78 | 1524389.46 | 399840.18 | 464037.1 |
| msgpack (medium) | — | 8235.54 | 9230.37 | 7400.28 | 7729.53 |
| msgpack (large) | — | 995.24 | 632.36 | 662.8 | 677.45 |
| YamlSerializer (small) | — | 1465.64 | 1248.1 | 2677.91 | 2275.7 |
| YamlSerializer (medium) | — | 12.43 | 8.07 | 17.5 | 22.46 |
| PyYAML (small) | — | 747.53 | 1174.63 | 2313.57 | 2186.12 |
| PyYAML (medium) | — | 13.0 | 16.14 | 27.04 | 27.95 |
| ruamel.yaml (small) | — | 898.85 | 842.14 | 1455.32 | 1303.61 |
| ruamel.yaml (medium) | — | 6.35 | 6.64 | 11.62 | 12.97 |
| PickleSerializer (small) | — | 243546.04 | 262191.93 | 231267.33 | 249003.96 |
| PickleSerializer (medium) | — | 10527.31 | 9455.19 | 7691.3 | 8865.17 |
| stdlib pickle (small) | — | 354987.51 | 415627.65 | 348310.72 | 369685.82 |
| stdlib pickle (medium) | — | 11865.77 | 10019.94 | 8135.11 | 10397.28 |

## Raw data

- Full results: [results_20260209_224805.json](results_20260209_224805.json)
