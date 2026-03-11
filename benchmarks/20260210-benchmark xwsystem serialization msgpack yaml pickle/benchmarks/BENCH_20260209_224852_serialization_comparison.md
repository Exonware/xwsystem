# BENCH 20260209_224852 — Serialization comparison (MsgPack, YAML, Pickle)

## Summary

| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |
|---------|---------|--------------|-------------|-------------|-------------|
| MsgPackSerializer (small) | — | 595237.69 | 990098.32 | 622664.78 | 692521.09 |
| MsgPackSerializer (medium) | — | 14055.21 | 16334.0 | 15229.04 | 15930.67 |
| MsgPackSerializer (large) | — | 824.02 | 1029.81 | 572.62 | 272.31 |
| msgpack (small) | — | 328299.42 | 248880.02 | 181488.2 | 187195.85 |
| msgpack (medium) | — | 4340.05 | 4145.77 | 4500.77 | 4799.85 |
| msgpack (large) | — | 502.43 | 387.28 | 434.69 | 504.94 |
| YamlSerializer (small) | — | 1918.74 | 1674.02 | 2508.96 | 3206.94 |
| YamlSerializer (medium) | — | 17.14 | 18.18 | 31.34 | 30.27 |
| PyYAML (small) | — | 1976.69 | 1876.35 | 3201.95 | 3460.85 |
| PyYAML (medium) | — | 19.21 | 19.97 | 37.23 | 34.81 |
| ruamel.yaml (small) | — | 845.45 | 850.51 | 1215.56 | 1354.5 |
| ruamel.yaml (medium) | — | 8.21 | 7.35 | 12.57 | 12.86 |
| PickleSerializer (small) | — | 451671.07 | 628141.43 | 508129.82 | 551267.96 |
| PickleSerializer (medium) | — | 15513.5 | 16437.1 | 13285.86 | 13845.81 |
| stdlib pickle (small) | — | 819672.15 | 980391.41 | 716332.9 | 758724.93 |
| stdlib pickle (medium) | — | 16961.8 | 17127.39 | 14142.67 | 14743.61 |

## Raw data

- Full results: [results_20260209_224852.json](results_20260209_224852.json)
