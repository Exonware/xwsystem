# BENCH 20260209_230855 — Serialization comparison (MsgPack, YAML, Pickle)

## Summary

| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |
|---------|---------|--------------|-------------|-------------|-------------|
| MsgPackSerializer (small) | — | 550357.42 | 689654.9 | 303582.32 | 407830.34 |
| MsgPackSerializer (medium) | — | 14862.37 | 9001.71 | 11490.55 | 13488.36 |
| MsgPackSerializer (large) | — | 896.09 | 963.8 | 783.77 | 756.22 |
| msgpack (small) | — | 944287.14 | 1044931.75 | 448833.13 | 458085.38 |
| msgpack (medium) | — | 10759.51 | 12264.52 | 11659.63 | 11863.66 |
| msgpack (large) | — | 1133.24 | 838.73 | 751.89 | 723.8 |
| YamlSerializer (small) | — | 1783.54 | 2656.08 | 4877.62 | 5508.43 |
| YamlSerializer (medium) | — | 22.85 | 21.12 | 49.5 | 47.72 |
| YamlSerializer (large) | — | 0.78 | 0.66 | 1.9 | 2.41 |
| PyYAML (small) | — | 2205.58 | 2076.57 | 3499.1 | 3689.76 |
| PyYAML (medium) | — | 26.98 | 20.84 | 60.23 | 40.71 |
| PyYAML (large) | — | 0.87 | 0.76 | 2.86 | 2.28 |
| ruamel.yaml (small) | — | 690.8 | 698.73 | 1190.87 | 1295.92 |
| ruamel.yaml (medium) | — | 6.94 | 12.78 | 17.88 | 18.8 |
| ruamel.yaml (large) | — | 0.46 | 0.45 | 1.04 | 0.81 |
| PickleSerializer (small) | — | 439367.34 | 478927.29 | 419287.09 | 448832.9 |
| PickleSerializer (medium) | — | 17031.13 | 16423.87 | 13779.23 | 19748.8 |
| PickleSerializer (large) | — | 711.06 | 1066.99 | 737.43 | 1051.17 |
| stdlib pickle (small) | — | 499999.98 | 511770.79 | 429184.5 | 385059.69 |
| stdlib pickle (medium) | — | 18345.26 | 20820.32 | 18021.27 | 15103.46 |
| stdlib pickle (large) | — | 663.87 | 730.84 | 957.35 | 949.1 |

## Raw data

- Full results: [results_20260209_230855.json](results_20260209_230855.json)
