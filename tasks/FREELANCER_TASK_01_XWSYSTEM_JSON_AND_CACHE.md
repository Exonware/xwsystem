# XWSYSTEM – Freelancer Task 01: JSON Serializer & Cache Competition

## 1. Overview

This task is a **backend performance & architecture challenge** on top of **XWSYSTEM**.

You will implement:

- **A new JSON serializer** that competes with and **outperforms** the existing `JsonSerializer`.
- **A new in‑memory cache** that competes with and **outperforms** the existing `OptimizedLFUCache`.

Your work will be validated by an **official benchmark script** in this folder.

---

## 2. Objectives

- **Serializer objective**: Implement `CandidateJsonSerializer` so that it:
  - Correctly round‑trips representative JSON payloads (no data loss / corruption).
  - **Beats `JsonSerializer` in encode+decode throughput** on the provided benchmark.

- **Cache objective**: Implement `CandidateCache` so that it:
  - Behaves as a correct, deterministic in‑memory cache (predictable eviction).
  - **Beats `OptimizedLFUCache` in operations/second** on the hot read/write benchmark.

This task is designed to test:

- Your ability to read and respect an existing architecture (`xwsystem`).
- Your ability to design **high‑performance, low‑level data structures**.
- Your ability to write **clean, production‑grade Python**, not just micro‑bench code.

---

## 3. Code locations

All task‑specific files live under:

- `xwsystem/tasks/`

You will work in the following Python files:

- `xwsystem/tasks/candidate_json.py`
  - Defines `CandidateJsonSerializer` – a new JSON serializer.
- `xwsystem/tasks/candidate_cache.py`
  - Defines `CandidateCache` – a new in‑memory cache.
- `xwsystem/tasks/json_and_cache_competition_benchmark.py`
  - Official benchmark script comparing baseline vs your implementations.

The **existing production implementations** you are competing against live inside the installed `exonware.xwsystem` package (do not modify them):

- `exonware.xwsystem.io.serialization.formats.text.json.JsonSerializer`
- `exonware.xwsystem.caching.lfu_optimized.OptimizedLFUCache`

---

## 4. Part A – JSON Serializer Task

### 4.1. File & class

- File: `xwsystem/tasks/candidate_json.py`
- Class: `CandidateJsonSerializer`

The file is intentionally mostly empty and documents the **rules and expectations** for your implementation. You must implement:

- `__init__(...)`
- `encode(self, value, *, options: Optional[EncodeOptions] = None) -> bytes | str`
- `decode(self, repr: bytes | str, *, options: Optional[DecodeOptions] = None) -> Any`

### 4.2. Constraints

You **MUST NOT**:

- Subclass or wrap the existing `JsonSerializer`.
- Call `JsonSerializer.encode` or `JsonSerializer.decode`.
- Re‑use the existing parser registry in `exonware.xwsystem.io.serialization.parsers.registry.get_parser`.
- Copy‑paste code from `json.py` or other serializers – design your own approach.

You **MAY**:

- Use alternative high‑performance JSON libraries (e.g. `pysimdjson`, `python-rapidjson`, etc.).
- Implement your own specialized fast path for typical payloads used in the benchmark.
- Add extra tuning parameters to `__init__` if needed (e.g. buffer sizes, flags).

### 4.3. Behavioural requirements

- API must be **compatible** with `ASerialization` and the expectations in `json.py`:
  - Accept `EncodeOptions` / `DecodeOptions` (at least the subset used in the benchmark).
  - Support text JSON (returning `str` is recommended; `bytes` is allowed).
- Must **round‑trip** the benchmark payloads correctly (no missing keys, no wrong types).
- Must raise `SerializationError`‑compatible errors on failures (see existing serializers for patterns).

### 4.4. Performance expectations

In `json_and_cache_competition_benchmark.py`, your serializer is compared against:

- `JsonSerializer (baseline)`
- `CandidateJsonSerializer`

You should aim for **strictly higher `operations_per_second`** for your candidate on the serializer section of the benchmark, on realistic hardware.

---

## 5. Part B – Cache Task

### 5.1. File & class

- File: `xwsystem/tasks/candidate_cache.py`
- Class: `CandidateCache`

The file is intentionally mostly empty and documents the **rules and expectations** for your cache. You must implement all abstract methods from `ACache`:

- `get`, `put`, `delete`, `clear`, `size`, `is_full`, `evict`, `keys`, `values`, `items`, `get_stats`.

### 5.2. Constraints

You **MUST NOT**:

- Wrap, delegate to, or subclass `OptimizedLFUCache` or any other existing cache in `exonware.xwsystem.caching`.
- Implement a cache that simply proxies calls to an existing implementation.
- Copy‑paste internal data‑structure logic from `lfu_optimized.py` or other caches.

You **MAY**:

- Design your own algorithm (e.g. segmented LRU, TinyLFU‑style admission, ARC‑like, CLOCK, etc.).
- Optimize for the read/write pattern used in the benchmark (hot keys, mixed workload).
- Trade off memory for speed, as long as behaviour is deterministic and well‑defined.

### 5.3. Behavioural requirements

- **Correctness**:
  - `get` must never raise for missing keys; return the provided default instead.
  - `put` must enforce the `capacity` limit and trigger eviction deterministically.
  - `delete` must be safe on missing keys and return a boolean status.
  - `clear`, `size`, `is_full`, `keys`, `values`, `items` must behave as expected from their names.
- **Statistics**:
  - `get_stats` should expose at least:
    - `capacity`
    - `size`
    - total puts / gets
    - hit rate
    - eviction count

### 5.4. Performance expectations

In `json_and_cache_competition_benchmark.py`, your cache is compared against:

- `OptimizedLFUCache (baseline)`
- `CandidateCache`

You should aim for **strictly higher `operations_per_second`** for your candidate on the cache section of the benchmark, for realistic key‑space and iteration counts.

---

## 6. Benchmark – How to Run

The official benchmark lives in:

- `xwsystem/tasks/json_and_cache_competition_benchmark.py`

It will:

- Build representative JSON payloads.
- Run encode+decode cycles for both serializers.
- Run a mixed put/get workload for both caches.
- Print a human‑readable summary and raw result dictionaries.

### 6.1. Basic run

From the root of the `exonware` repo (where `xwsystem/` lives), you can typically run:

```bash
cd xwsystem/tasks
python json_and_cache_competition_benchmark.py
```

Make sure you have the project installed/available on `PYTHONPATH` so imports like
`exonware.xwsystem...` resolve correctly (the same setup you use to run existing tests).

---

## 7. Evaluation Criteria

We will look at:

- **Correctness**: All benchmarks complete without errors; round‑trips and cache behaviour are correct.
- **Performance**: Your implementations provide a **clear performance win** over the baselines.
- **Code quality**:
  - Clean, well‑structured, and well‑commented where it matters.
  - Follows XWSYSTEM conventions and patterns as much as practical.
  - No unnecessary complexity – performance should be justified and measured.

Deliver your work as standard Python code changes (no binary artifacts) and be ready to explain your design choices, trade‑offs, and how you tuned for the benchmark.

