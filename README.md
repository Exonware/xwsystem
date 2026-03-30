# xwsystem

**One install instead of 50+.** Serialization (24+ formats), caching, security, validation, HTTP, IPC, monitoring - same APIs everywhere. The base every other eXonware package builds on.

**Company:** eXonware.com · **Author:** eXonware Backend Team · **Email:** connect@exonware.com  

[![Status](https://img.shields.io/badge/status-beta-blue.svg)](https://exonware.com)
[![Python](https://img.shields.io/badge/python-3.12%2B-blue.svg)](https://www.python.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

---

## Install

**Requires Python 3.12+.** (See [pyproject.toml](pyproject.toml) `requires-python`.)

| Install | What you get | When to use |
|---------|--------------|-------------|
| `pip install exonware-xwsystem` | **Lite** - core only, zero optional deps | Minimal footprint, or you install what you need. |
| `pip install exonware-xwsystem[lazy]` | **Lazy** - core + xwlazy; missing deps install on first import | Development; optional formats without pre-installing everything. |
| `pip install exonware-xwsystem[full]` | **Full** - core + common optionals pre-installed | Production or CI when you want all formats and features up front. |

Same package; `[lazy]` and `[full]` are extras. You can also install as `xwsystem` (same thing).

---

## Quick start

**Serialization (same API for every format):**

```python
from exonware.xwsystem import JsonSerializer

s = JsonSerializer()
text = s.dumps({"name": "Alice", "age": 30})
data = s.loads(text)
```

**Lazy install:** With `[lazy]`, use normal imports; if a format backend is missing, xwlazy installs it on first use. No try/except import blocks.

**Windows console UTF-8:**

```python
from exonware.xwsystem.console.cli import ensure_utf8_console
ensure_utf8_console()
```

**Config:** `xwconfig.toml` at project root; in code: `from exonware.xwsystem import settings` then `settings.current()`.

---

## What you get

| Area | What’s in it |
|------|----------------|
| **One dependency** | Replaces dozens of imports (json, yaml, msgpack, crypto, requests, path validation, caching, …) with one API surface. |
| **24+ serialization formats** | Text (JSON, YAML, TOML, XML, CSV, INI, properties, .env), binary (MsgPack, BSON, CBOR, Pickle), database (Sqlite3, Dbm, Shelve), tabular (DataFrame, Excel, CSV dialects). Enterprise/scientific (~10 formats: Avro, Protobuf, Parquet, ORC, Arrow, etc.) integrate via xwformats. |
| **20+ caching variants** | LRU, LFU, TTL (sync + async), read-through/write-through/write-behind, two-tier (memory+disk), secure/encrypted caches, observable caches, Prometheus export. |
| **Security** | Path validation, file security, audit helpers, crypto utilities, validators, policies. |
| **Validation** | Pydantic-style XModel, schema discovery. |
| **Runtime** | Config (performance modes, logging), console (CLI, colors, progress, tables), monitoring (memory, performance, tracing), IPC (process fabric, queues, pipes, shared memory), HTTP client (HTTP/2, retries, streaming). |
| **Patterns** | Circuit breakers, retry, object pool, context managers. Plus: operations (diff, merge, patch), structures (tree walker, circular ref detection), threading (locks, thread-safe factory), utils (datetime, paths, string, web), data structures (trie, union-find), grammar/syntax (50+ languages, Monaco), plugins, query registry. |
| **Indexing** | Pluggable index backends (`io/indexing`): InMemory, FileBacked, **BTreeIndexBackend** (ordered, range_scan), **FullTextIndexBackend** and **FileBackedFullTextIndexBackend** (term index, persistent). See [docs/REF_15_API.md](docs/REF_15_API.md) (Indexing module). |

Lite = zero optional deps. Lazy = xwlazy installs format backends on first use. Full = common optionals pre-installed.

---

## Why xwsystem? (Highlights)

**Automatic format detection (no guessing)** - One API for any file. `AutoSerializer` and `detect_format` pick the right codec from **file extension**, **magic bytes**, or **content patterns** (confidence-scored). No need to know whether it’s JSON, YAML, or MessagePack; pass a path or bytes and get the right serializer.

```python
from exonware.xwsystem import AutoSerializer, detect_format
from pathlib import Path

# By path: extension tells us the format
fmt = detect_format(Path("data.json"))   # → JSON

# One serializer for everything: format from path (extension)
auto = AutoSerializer()
data = auto.load_file(Path("config.yaml"))
auto.save_file(data, Path("out.toml"))
```

**JSON: 3x+ faster than stdlib** - Pluggable parsers: **orjson** (3-4x faster), optional **hybrid** (msgspec for read, orjson for write). Measured: 1MB serialize &lt;45ms, deserialize &lt;52ms (see [REF_54_BENCH](docs/REF_54_BENCH.md)).

**Caching: O(1), benchmark-backed** - LRU/LFU/TTL with sub-microsecond get/set (10K items). Default uses the fastest Python cache baseline (~33k GET ops/sec, ~3.3k MIXED in our benchmarks). SLAs and NFRs in [REF_54_BENCH](docs/REF_54_BENCH.md).

**Serialization benchmark (1MB data)** - From [REF_15_API](docs/REF_15_API.md) / [REF_54_BENCH](docs/REF_54_BENCH.md):

| Format     | Serialize | Deserialize |
|-----------|-----------|-------------|
| MessagePack | 23 ms   | 28 ms       |
| JSON       | 45 ms   | 52 ms       |
| YAML       | 123 ms  | 145 ms      |

**Benchmarks** - Campaigns live in [benchmarks/](benchmarks/) (per [GUIDE_54_BENCH](../docs/guides/GUIDE_54_BENCH.md)). Run from project root; full index: [benchmarks/INDEX.md](benchmarks/INDEX.md).

**Benchmarks summary (10-Feb-2026)** - Condensed from latest runs. All campaigns: JSON, caching, serialization, atomic I/O, operations, data structures, object pool, validation, threading locks, async fabric.

| Area | Best / representative result |
|------|------------------------------|
| **JSON** | JsonSerializer (hybrid/hyperjson): small ~798k decode / 506k encode ops/s; with hyperjson installed matches hyperjson. orjson/msgspec in same tier. |
| **Caching** | PylruCache (default): get 2.99M/s, put 2.83M/s, mixed 2.49M/s. CacheboxCache get 3.72M/s. TwoTierCache now included (put/get contract). |
| **Serialization** | MsgPackSerializer small: 862k decode / 301k encode. YamlSerializer medium: 15 decode / 39 encode. PickleSerializer small: 422k decode / 357k encode. Competitive with msgpack/PyYAML/stdlib pickle. |
| **Atomic I/O** | AtomicFileWriter: 205-357 writes/s (1KB-100KB). Plain write: 504-743 writes/s. Atomic adds ~2-3x latency for crash-safe commit. |
| **Operations** | diff (structural/content/full) small: 35k-48k ops/s. merge (deep/shallow/overwrite) small: 59k-70k ops/s. patch_apply small: 64k ops/s. |
| **Data structures** | TrieNode lookup 899k ops/s; dict lookup 26M ops/s. UnionFind (make_set+union+find) 573 runs/s. trie insert batch 49k ops/s. |
| **Object pool** | ObjectPool get+release (thread_safe=True) 350k ops/s; (thread_safe=False) 287k ops/s. direct SimpleObj() 2.93M ops/s. Pool overhead ~8x for reuse. |
| **Validation** | check_data_depth shallow 1.11M ops/s, deep 229k ops/s. validate_path_input (safe) 2.58M ops/s. |
| **Locks** | EnhancedRLock (track_stats=True) 1.53M ops/s; (track_stats=False) 3.37M ops/s. threading.RLock 7.14M ops/s. Use track_stats=False for fast path. |
| **Async fabric** | Submit 1k tasks ~3.27k ops/s; queue latency ~0.02 ms; shared memory reuse read ~0.006 ms (see benchmarks/20260210-benchmark xwsystem async fabric). |

**Using these results:** `create_cache()` already defaults to **PylruCache** when pylru is installed (~2.5-3M mixed ops/s). For hot-path locking without timeout or stats, use **`fast_lock()`** or **`EnhancedRLock(track_stats=False)`** (~3.4M ops/s). Use **TwoTierCache** when you need memory+disk; use **ObjectPool** when object creation cost outweighs pool overhead. See [REF_54_BENCH](docs/REF_54_BENCH.md) trade-offs table.

---

## Rust and the rest of the stack

**Rust:** xwsystem is the reference implementation. Multi-language (TypeScript, Rust, Go) is planned via contracts/specs. Right now Python performance is comparable to what we would expect from a Rust port on the hot paths, so no conversion yet. Getting here took several rewrites; the current design is the one we kept.

**Ecosystem:** xwsystem is the foundation for 12+ eXonware projects (xwstorage, xwformats, xwjson, xwnode, xwdata, xwauth, xwquery, xwchat, xwui, *-server). They get one dependency instead of 50+, same APIs everywhere, and Lite/Lazy/Full so each package can ship lean or full. One place to fix bugs and add features for the whole stack.

---

## Full feature list and examples

For a complete feature tour, code samples, and platform notes see **[README_LONG.md](README_LONG.md)**.

---

## Docs and tests

Content in this README is aligned with the project REFs and [docs/GUIDE_01_USAGE.md](docs/GUIDE_01_USAGE.md) (per [GUIDE_63_README](../../docs/guides/GUIDE_63_README.md)).

- **Start:** [docs/INDEX.md](docs/INDEX.md) - doc index and quick links.
- **Use it:** [docs/GUIDE_01_USAGE.md](docs/GUIDE_01_USAGE.md) - installation, codecs, caching, runtime, production.
- **Requirements and status:** [docs/REF_01_REQ.md](docs/REF_01_REQ.md), [docs/REF_22_PROJECT.md](docs/REF_22_PROJECT.md).
- **API and design:** [docs/REF_15_API.md](docs/REF_15_API.md), [docs/REF_13_ARCH.md](docs/REF_13_ARCH.md).
- **DX and quality:** [docs/REF_14_DX.md](docs/REF_14_DX.md), [docs/REF_50_QA.md](docs/REF_50_QA.md), [docs/REF_54_BENCH.md](docs/REF_54_BENCH.md), [docs/REF_51_TEST.md](docs/REF_51_TEST.md).
- **Ideas and planning:** [docs/REF_12_IDEA.md](docs/REF_12_IDEA.md), [docs/REF_21_PLAN.md](docs/REF_21_PLAN.md).
- **Compliance:** [docs/compliance/](docs/compliance/). **Evidence:** [docs/logs/](docs/logs/) (changes, tests, benchmarks, plans).
- **Benchmarks:** [benchmarks/INDEX.md](benchmarks/INDEX.md) - run scripts in `benchmarks/<campaign>/scripts/` (JSON, caching, serialization, atomic I/O, operations, data structures, object pool, validation, locks). See [REF_54_BENCH](docs/REF_54_BENCH.md).

**Tests:** 4-layer suite (0.core, 1.unit, 2.integration, 3.advance). Run via project test runner or pytest. See [docs/REF_51_TEST.md](docs/REF_51_TEST.md).

---

## Where xwsystem fits

`xwsystem` provides the shared infrastructure layer for other eXonware packages (xwstorage, xwformats, xwjson, xwnode, xwdata, xwauth, xwquery, xwchat, xwui, *-server). They depend on its serializers, caches, security helpers, IPC layer, and runtime services instead of re-implementing them.

Approximate surface area (as of 2026-02-07, see REFs and benchmarks for exact lists):

- **Formats:** 24+ core serialization formats in this package, plus ~10 enterprise/scientific formats via xwformats.
- **Caching:** 20+ cache variants built from core implementations and wrappers (LRU/LFU/TTL, two-tier, observable, secure, async, read-through/write-through/write-behind).
- **Grammar/syntax:** 50+ language grammars with Monaco integration (see `grammar/` and [REF_13_ARCH](docs/REF_13_ARCH.md)).
- **IPC:** 4+ IPC primitives (process pool, message queue, shared memory, async fabric facade) wired into a single Async Process Fabric.
- **Benchmarks:** 10+ benchmark campaigns (JSON, caching, serialization, atomic I/O, operations, data structures, object pool, validation, locks, async fabric) tracked in `benchmarks/` and [REF_54_BENCH](docs/REF_54_BENCH.md).

Downstream libraries consume these services via stable APIs exported from `exonware.xwsystem`, so fixes and performance improvements in xwsystem flow through the rest of the ecosystem.

---

## License and links

MIT - see [LICENSE](LICENSE).

- **Homepage:** https://exonware.com  
- **Repository:** https://github.com/exonware/xwsystem  

Part of the eXonware ecosystem - one foundation for all of it.

## Async Support

<!-- async-support:start -->
- xwsystem includes asynchronous execution paths in production code.
- Source validation: 284 async def definitions and 210 await usages under src/.
- Use async APIs for I/O-heavy or concurrent workloads to improve throughput and responsiveness.
<!-- async-support:end -->
Version: 0.9.0.29 | Updated: 31-Mar-2026

*Built with ❤️ by eXonware.com - Revolutionizing Python Development Since 2025*
