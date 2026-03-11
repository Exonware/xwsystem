# BENCH 20260209_225421 — Serialization comparison (MsgPack, YAML, Pickle)

## Summary

| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |
|---------|---------|--------------|-------------|-------------|-------------|
| MsgPackSerializer (small) | — | 574052.28 | 862069.32 | 270124.41 | 301386.41 |
| MsgPackSerializer (medium) | — | 4747.3 | 7739.1 | 8787.96 | 5944.95 |
| MsgPackSerializer (large) | — | 354.28 | 628.66 | 282.69 | 360.14 |
| msgpack (small) | — | 948765.67 | 1272266.25 | 399360.95 | 410846.36 |
| msgpack (medium) | — | 4783.73 | 6241.57 | 5626.32 | 6590.57 |
| msgpack (large) | — | 644.94 | 379.08 | 517.61 | 426.84 |
| YamlSerializer (small) | — | 1389.41 | 1210.7 | 2182.89 | 2112.78 |
| YamlSerializer (medium) | — | 11.14 | 14.73 | 32.06 | 38.96 |
| PyYAML (small) | — | 2202.47 | 1925.11 | 4064.28 | 3352.44 |
| PyYAML (medium) | — | 14.46 | 13.25 | 36.87 | 38.48 |
| ruamel.yaml (small) | — | 613.18 | 512.74 | 919.62 | 884.9 |
| ruamel.yaml (medium) | — | 7.54 | 6.7 | 10.62 | 11.89 |
| PickleSerializer (small) | — | 406173.82 | 422296.99 | 507099.72 | 357398.1 |
| PickleSerializer (medium) | — | 17449.57 | 17731.75 | 10636.71 | 17152.07 |
| stdlib pickle (small) | — | 821018.09 | 1039500.67 | 740740.21 | 697350.42 |
| stdlib pickle (medium) | — | 11751.16 | 11034.6 | 18053.15 | 14010.31 |

## Raw data

- Full results: [results_20260209_225421.json](results_20260209_225421.json)
