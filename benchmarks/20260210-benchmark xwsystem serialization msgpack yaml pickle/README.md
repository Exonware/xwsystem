# 20260210 — Benchmark: xwsystem Serialization (MsgPack, YAML, Pickle) vs Others

**Campaign:** Serialization encode/decode comparison for MsgPack, YAML, Pickle  
**Date:** 10-Feb-2026  
**Project:** exonware/xwsystem  
**Guide:** [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)

---

## Goal

Compare **xwsystem serializers** (MsgPackSerializer, YamlSerializer, PickleSerializer) with **top Python libraries** for the same formats (msgpack, PyYAML/ruamel.yaml, stdlib pickle) on encode/decode throughput for small, medium, and large payloads.

---

## Description

This benchmark measures:

- **Encode:** Python object to bytes (MsgPack, Pickle) or str (YAML)
- **Decode:** bytes/str to Python object
- **Cold vs warm:** first-touch and steady-state throughput
- **Payload sizes:** small, medium, large (same shape as JSON benchmark)

Libraries in scope:

| Format  | xwsystem                         | External comparison        |
|---------|----------------------------------|----------------------------|
| MsgPack | MsgPackSerializer                | msgpack                    |
| YAML    | YamlSerializer                   | PyYAML, ruamel.yaml        |
| Pickle  | PickleSerializer                 | stdlib pickle              |

---

## Metrics

- **Throughput:** ops/sec for encode and decode (cold and warm)
- **Payload:** small / medium / large

---

## Structure

- **scripts/** — `run_benchmark_serialization.py`
- **data/** — Optional payload samples
- **benchmarks/** — BENCH_* reports, results_*.json, INDEX.md

---

## How to Run

From xwsystem root:

```bash
pip install msgpack pyyaml ruamel.yaml  # optional deps
python "benchmarks/20260210-benchmark xwsystem serialization msgpack yaml pickle/scripts/run_benchmark_serialization.py"
```

Or from the campaign scripts directory:

```bash
cd "benchmarks/20260210-benchmark xwsystem serialization msgpack yaml pickle/scripts"
python run_benchmark_serialization.py
```

---

## Results

**Last run:** 09-Feb-2026 (BENCH_20260209_225421)

**MsgPack (ops/sec)**

| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |
|---------|---------|-------------|-------------|-------------|-------------|
| MsgPackSerializer | small | 574k | 862k | 270k | 301k |
| MsgPackSerializer | medium | 4.7k | 7.7k | 8.8k | 5.9k |
| MsgPackSerializer | large | 354 | 629 | 283 | 360 |
| msgpack | small | 949k | 1.27M | 399k | 411k |
| msgpack | medium | 4.8k | 6.2k | 5.6k | 6.6k |
| msgpack | large | 645 | 379 | 518 | 427 |

**YAML (ops/sec)**

| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |
|---------|---------|-------------|-------------|-------------|-------------|
| YamlSerializer | small | 1.4k | 1.2k | 2.2k | 2.1k |
| YamlSerializer | medium | 11 | 15 | 32 | 39 |
| PyYAML | small | 2.2k | 1.9k | 4.1k | 3.4k |
| PyYAML | medium | 14 | 13 | 37 | 38 |
| ruamel.yaml | small | 613 | 513 | 920 | 885 |
| ruamel.yaml | medium | 8 | 7 | 11 | 12 |

**Pickle (ops/sec)**

| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |
|---------|---------|-------------|-------------|-------------|-------------|
| PickleSerializer | small | 406k | 422k | 507k | 357k |
| PickleSerializer | medium | 17.5k | 17.7k | 10.6k | 17.2k |
| stdlib pickle | small | 821k | 1.04M | 741k | 697k |
| stdlib pickle | medium | 11.8k | 11k | 18.1k | 14k |

**Report:** [benchmarks/BENCH_20260209_225421_serialization_comparison.md](benchmarks/BENCH_20260209_225421_serialization_comparison.md) · **Data:** [benchmarks/results_20260209_225421.json](benchmarks/results_20260209_225421.json)

---

## Related

- [GUIDE_54_BENCH.md](../../../docs/guides/GUIDE_54_BENCH.md)
- [../INDEX.md](../INDEX.md) — Benchmarks index
