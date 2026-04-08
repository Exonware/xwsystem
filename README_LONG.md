# xwsystem (long README)

Hey — if you want the short version first, head to [README.md](README.md). This page is the tour: where things live, how to install, and where to dig deeper when you are building something serious.

**Company:** eXonware.com · **Author:** eXonware Backend Team · **Email:** connect@exonware.com  
**Version:** [version.py](src/exonware/xwsystem/version.py) or PyPI · **Last updated:** April 2026

---

## Why we’re glad this exists (and you might be too)

Shipping Python that touches files, networks, caches, and crypto means you usually glue together half a dozen libraries that all feel different. xwsystem is our answer: **one familiar surface** for the boring-but-critical stuff, so you spend brain budget on your product instead of reinventing safe IO for the ninth time.

You still get **real choices** — go **lite** and stay tiny, turn on **lazy** so optional pieces appear when code first needs them, or go **full** and let CI breathe easy. Same package; you pick the footprint.

We use this across eXonware. When we fix a serializer edge case or harden a path check, **every downstream library benefits**. That is the kind of leverage that actually feels good in production.

---

## What this is

The stack story in one breath: most eXonware Python packages import xwsystem instead of vendoring their own serializers, safe file helpers, caches, crypto wrappers, and HTTP glue. **One place to fix bugs, one set of patterns to learn.**

**Canonical copy (GUIDE_66)** — reuse these on PyPI, GitHub, or docs:

- **Tagline:** Shared systems layer for eXonware Python packages.
- **About line:** xwsystem is a Python systems library for eXonware packages that unifies serialization, IO, caching, security, validation, HTTP, IPC, and monitoring behind one import surface.
- **About paragraph:** xwsystem is a Python systems library that concentrates serialization, safe IO, caching, crypto, validation, console utilities, monitoring, IPC, and HTTP patterns in one package. Downstream eXonware libraries share the same APIs. Extras: `[lazy]` installs missing format backends on first use via xwlazy; `[full]` pre-installs common optional dependencies for CI.

**Neighboring packages (on purpose):** schema registry implementations live in **exonware-xwschema**. Full auth providers (OAuth2, JWT, SAML, and the rest) live in **xwauth**, built on xwsystem’s security contracts — so security stays sharp without turning this repo into a kitchen sink.

---

## Where to look in the source

All paths: `src/exonware/xwsystem/`. The package root pulls the common types up front; **reach into subpackages** when you want a slimmer import line or you are reading code and want to see who owns what.

**IO (`io/`)** — This is the workhorse: atomic reads and writes, locks, watchers, paging (byte / line / record), folders, codec-aware streams, a practical local filesystem view, and `source_reader` for “give me text from disk or HTTPS.” Serialization lives in `io/serialization` — formats, registry, `AutoSerializer`, `detect_format`, pools. Indexing is `io/indexing`. Archives: ZIP and TAR are always there; 7z, zstd, RAR, brotli, LZ4, ZPAQ, WIM, SquashFS, and friends **wake up when you use them** (and may ask for extra Python or OS bits). When you want one front door: `XWIO`.

**Indexing (`io/indexing/`)** — `XWIndex` loves JSONL/NDJSON: stream reads and updates, line offsets, the kind of workflows log and data pipelines need. Backends span in-memory, file-backed, an ordered B-tree-style store for range scans, and full-text variants — pick the tradeoff that matches your workload.

**Caching (`caching/`)** — From “just make it fast” to “make it observable”: LRU, LFU, TTL (sync and async), two-tier and memory-bounded caches, read/write/read-write-through, write-behind, tags, Bloom-style shortcuts, secure and observable variants, Prometheus export, and decorators `xwcached` / `xw_async_cached`. `CacheFactory` when you want batteries. Redis and friends show up when optional deps do. Facade: `XWCache`.

**Security (`security/`)** — Path validation, symmetric and asymmetric crypto (sync and async), password hashing, secure random and storage, `XWSecurity` / `XWCrypto`, and `hazmat` when `cryptography` is on the path. Contracts for tokens and auth live here; **implementations** ship in **xwauth** so this layer stays composable.

**Validation (`validation/`)** — Declarative `XModel` / `Field`, `DataValidator`, guards on depth, paths, and memory, plus `SafeTypeValidator` when the internet sends you weird data. Facade: `XWValidator`.

**HTTP (`http_client/`)** — `HttpClient`, `AsyncHttpClient`, `RetryConfig`, and `AdvancedHttpClient` (HTTP/2 options, streaming, mock transport). Enough rope to call real services without assembling a pile of one-off wrappers. Facade: `XWHTTP`.

**Monitoring (`monitoring/`)** — Performance and memory tracking, circuit breaker, error recovery, performance budgets, tracing hooks (OpenTelemetry and friends when installed). `SystemMonitor` and OS-style probes want `psutil`. Facade: `XWMonitor`.

**IPC (`ipc/`)** — Processes, shared memory, queues, pools, pipes — sync and async — for when your architecture steps beyond a single process.

**Threading (`threading/`)** — `ThreadSafeFactory`, `EnhancedRLock`, fast lock helpers, async locks, semaphores, queues, RW locks, resource pools. Facade: `XWConcurrency`.

**Console (`console/`)** — Colors, styles, argument parsing, progress, tables, `CliConsole`. On Windows, `console.cli.ensure_utf8_console` saves a lot of “why is this mojibake” afternoons.

**Config (`config/`)** — Shared defaults (limits, encodings, delimiters), `setup_logging` / `get_logger`, `DefaultConfig`, `PerformanceConfig` with `get_performance_config` / `set_performance_config`. Pair with `xwconfig.toml` as [docs/GUIDE_01_USAGE.md](docs/GUIDE_01_USAGE.md) describes.

**Smaller packages (still useful)** — `runtime/` (`EnvironmentManager`, `ReflectionUtils`). `patterns/` (context logging, singleton, handler factory, import registration, `ObjectPool`). `structures/` (circular reference detection, `TreeWalker`). `data_structures/` (`TrieNode`, `UnionFind`). `operations/` (merge, diff, `apply_patch`). `query/` (provider contract + registry so **xwquery** / **xwnode** integrate without import spaghetti). `observability/` (correlation headers, telemetry version, default HTTP timeouts). `plugins/` (manager + registry). `shared/` (enums, `XWObject`, `IStringable`, provider hooks). `utils/` (dates in `utils.dt`, strings, small web helpers).

**Facades on the root import** — `XWCache`, `XWArchive`, `XWIndex`, `XWSecurity`, `XWCrypto`, `XWHTTP`, `XWValidator`, `XWMonitor`, `XWConcurrency`, plus `XWIO` and the serializers you already know by name. Nice when you want the “just give me the thing” API.

**Extra codecs** — Install siblings like **xwformats**, **xwjson**, or **xwsyntax** and they can plug into the same serialization story. Lite stays whisper-quiet; `[lazy]` and `[full]` only change what is already sitting on disk.

**Lazy installs** — With `exonware-xwlazy` installed, unless `XWSTACK_SKIP_XWLAZY_INIT` is set, import time enables lazy install for this package in `smart` mode — a missing optional backend can be pulled in the first time your code actually touches it. Handy when you are exploring formats without pre-building a giant venv.

---

## Install

Python **3.12+**. One package, three moods:

| Install | What you get |
|--------|----------------|
| `pip install exonware-xwsystem` or `pip install xwsystem` | **Lite** — only `typing-extensions` required at runtime. Start here if you like minimal. |
| `pip install exonware-xwsystem[lazy]` | **Lazy** — adds xwlazy; missing format libraries can install when first used. Great for iterative work. |
| `pip install exonware-xwsystem[full]` | **Full** — common optionals upfront (crypto, HTTP stack, YAML, msgpack, OpenTelemetry pieces, Windows helpers, …). CI and “I want it all” machines. |

Full dependency list: [pyproject.toml](pyproject.toml).

---

## Copy-paste starters

**JSON**

```python
from exonware.xwsystem import JsonSerializer

s = JsonSerializer()
text = s.dumps({"name": "Alice", "age": 30})
data = s.loads(text)
```

**Windows console UTF-8**

```python
from exonware.xwsystem.console.cli import ensure_utf8_console
ensure_utf8_console()
```

**Config** — Drop `xwconfig.toml` where your app expects it. From code, start with `exonware.xwsystem.config` (`DefaultConfig`, `get_performance_config`, `setup_logging`); [docs/GUIDE_01_USAGE.md](docs/GUIDE_01_USAGE.md) walks the happy path.

**One-liners on the package root**

```python
from exonware.xwsystem import quick_serialize, quick_deserialize, list_available_formats
```

---

## Benchmarks

We keep campaigns under [benchmarks/](benchmarks/) with [benchmarks/INDEX.md](benchmarks/INDEX.md). How to read them: [docs/REF_54_BENCH.md](docs/REF_54_BENCH.md). Numbers are **directional** — your CPU, disk, and workload get the final vote.

---

## Rust

This tree is the **Python reference** you can ship today. Experimental Rust lives under [rust/](rust/). Performance targets and methodology are in REF_54; you are not waiting on a from-scratch rewrite to hit the goals documented there.

---

## Platforms

We run this on **Windows, Linux, and macOS** day to day. `[full]` may bring in Windows-specific helpers (see `pyproject.toml`). A heads-up: some async pipe paths are weaker on Windows — follow module docs and reach for sync pipes or multiprocessing where they tell you to. WIM archives on non-Windows hosts often want **wimlib** installed by you. Extra notes: [CROSS_PLATFORM_COMPATIBILITY_REPORT.md](CROSS_PLATFORM_COMPATIBILITY_REPORT.md) when that file is in the repo.

---

## Docs

| Doc | Use it for |
|-----|------------|
| [docs/INDEX.md](docs/INDEX.md) | The map — start here when you are lost |
| [docs/GUIDE_01_USAGE.md](docs/GUIDE_01_USAGE.md) | Install, codecs, caching, production |
| [docs/REF_15_API.md](docs/REF_15_API.md) | API tables, indexing detail |
| [docs/REF_13_ARCH.md](docs/REF_13_ARCH.md) | Architecture |
| [docs/REF_01_REQ.md](docs/REF_01_REQ.md), [docs/REF_22_PROJECT.md](docs/REF_22_PROJECT.md) | Scope and status |
| [docs/REF_14_DX.md](docs/REF_14_DX.md) | DX notes |
| [docs/REF_50_QA.md](docs/REF_50_QA.md), [docs/REF_51_TEST.md](docs/REF_51_TEST.md), [docs/REF_54_BENCH.md](docs/REF_54_BENCH.md) | QA, tests, benchmarks |
| [docs/compliance/](docs/compliance/), [docs/logs/](docs/logs/) | Compliance and logs where used |

If two pages disagree, **INDEX.md** wins.

---

## Tests

Layers **0.core → 3.advance**; project runner or plain `pytest`. Story of the suite: [docs/REF_51_TEST.md](docs/REF_51_TEST.md).

```bash
pip install -e .
pytest
```

---

## Who depends on this

xwstorage, xwformats, xwjson, xwnode, xwdata, xwauth, xwquery, xwchat, xwui, and related `*-server` packages sit on xwsystem so IO, security, caching, and config behave the same everywhere — **fix once, benefit across the stack.**

---

## License and links

Apache License 2.0 — [LICENSE](LICENSE).

- https://exonware.com  
- https://github.com/exonware/xwsystem  
- Version: `from exonware.xwsystem import __version__`

If something here sparks an idea or a bug report, we are building in the open — docs and repo above. Happy shipping.
