# BENCH 20260210_002609 — Serialization comparison (MsgPack, YAML, Pickle)

## Summary

| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |
|---------|---------|--------------|-------------|-------------|-------------|
| MsgPackSerializer (small) | — | 629485.09 | 960984.04 | 390106.86 | 548245.64 |
| MsgPackSerializer (medium) | — | 11951.72 | 15535.62 | 16182.7 | 14363.81 |
| MsgPackSerializer (large) | — | 1324.56 | 1552.19 | 842.51 | 1121.5 |
| msgpack (small) | — | 2651112.92 | 2357377.47 | 1042318.02 | 1037775.13 |
| msgpack (medium) | — | 20612.86 | 17108.23 | 20457.76 | 20924.18 |
| msgpack (large) | — | 1019.16 | 1364.26 | 1155.56 | 859.56 |
| YamlSerializer (small) | — | 2638.4 | 3372.11 | 6255.72 | 5790.18 |
| YamlSerializer (medium) | — | 24.57 | 22.13 | 44.6 | 41.47 |
| YamlSerializer (large) | — | 0.89 | 0.95 | 2.09 | 2.47 |
| PyYAML (small) | — | 2907.58 | 2678.16 | 4230.68 | 3935.57 |
| PyYAML (medium) | — | 24.97 | 25.19 | 45.23 | 47.41 |
| PyYAML (large) | — | 1.11 | 1.17 | 2.42 | 1.89 |
| ruamel.yaml (small) | — | 804.86 | 773.2 | 1134.49 | 1190.73 |
| ruamel.yaml (medium) | — | 7.76 | 7.88 | 20.41 | 17.74 |
| ruamel.yaml (large) | — | 0.44 | 0.52 | 1.07 | 1.05 |
| PickleSerializer (small) | — | 885425.9 | 1018744.88 | 956754.65 | 889046.8 |
| PickleSerializer (medium) | — | 26997.26 | 24609.69 | 25371.05 | 20918.66 |
| PickleSerializer (large) | — | 1202.38 | 1091.83 | 1561.04 | 1525.12 |
| stdlib pickle (small) | — | 659891.82 | 700280.15 | 588166.13 | 585274.5 |
| stdlib pickle (medium) | — | 20081.05 | 27143.38 | 21191.74 | 18351.59 |
| stdlib pickle (large) | — | 1080.95 | 1023.0 | 1404.34 | 2072.81 |

## Raw data

- Full results: [results_20260210_002609.json](results_20260210_002609.json)
