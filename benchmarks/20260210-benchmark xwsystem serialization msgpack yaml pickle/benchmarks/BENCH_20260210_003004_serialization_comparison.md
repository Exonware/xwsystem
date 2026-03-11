# BENCH 20260210_003004 — Serialization comparison (MsgPack, YAML, Pickle)

## Summary

| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |
|---------|---------|--------------|-------------|-------------|-------------|
| MsgPackSerializer (small) | — | 685588.95 | 863110.67 | 305978.83 | 279970.87 |
| MsgPackSerializer (medium) | — | 11238.38 | 9084.85 | 8134.23 | 7820.37 |
| MsgPackSerializer (large) | — | 765.53 | 853.95 | 721.09 | 599.19 |
| msgpack (small) | — | 795544.93 | 819806.54 | 325245.57 | 372856.09 |
| msgpack (medium) | — | 8092.15 | 8018.54 | 9945.44 | 13289.14 |
| msgpack (large) | — | 1080.21 | 714.69 | 923.65 | 758.16 |
| YamlSerializer (small) | — | 960.52 | 2110.77 | 3142.45 | 2512.61 |
| YamlSerializer (medium) | — | 23.2 | 22.51 | 50.04 | 33.14 |
| YamlSerializer (large) | — | 0.85 | 0.95 | 2.14 | 2.75 |
| PyYAML (small) | — | 1491.12 | 1504.81 | 2717.62 | 2741.23 |
| PyYAML (medium) | — | 16.04 | 22.72 | 38.43 | 31.95 |
| PyYAML (large) | — | 1.07 | 1.18 | 2.03 | 2.05 |
| ruamel.yaml (small) | — | 1490.4 | 1566.05 | 2353.88 | 2497.15 |
| ruamel.yaml (medium) | — | 10.61 | 10.94 | 20.51 | 12.61 |
| ruamel.yaml (large) | — | 0.45 | 0.51 | 1.14 | 0.95 |
| PickleSerializer (small) | — | 718907.2 | 759762.95 | 663481.92 | 686530.32 |
| PickleSerializer (medium) | — | 20251.03 | 19915.4 | 15066.91 | 14935.27 |
| PickleSerializer (large) | — | 866.75 | 775.85 | 1045.79 | 1055.43 |
| stdlib pickle (small) | — | 945358.17 | 973709.7 | 762427.58 | 790014.16 |
| stdlib pickle (medium) | — | 16354.46 | 17495.61 | 15365.99 | 15795.54 |
| stdlib pickle (large) | — | 851.28 | 852.21 | 1074.68 | 1104.75 |

## Raw data

- Full results: [results_20260210_003004.json](results_20260210_003004.json)
