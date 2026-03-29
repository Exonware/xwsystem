# xwsystem (long README)

Long-form companion to [README.md](README.md): same facts, more context and examples.

**Company:** eXonware.com · **Author:** eXonware Backend Team · **Email:** connect@exonware.com  
**Version:** See [version.py](src/exonware/xwsystem/version.py) or PyPI.

---

## Role in the stack

xwsystem is the shared systems layer for eXonware packages. It concentrates serialization, IO, caching, security helpers, validation, configuration, console utilities, monitoring, IPC, HTTP client usage patterns, and related building blocks so downstream libraries do not each vendor their own copies.

---

## Installation (Python 3.12+)

Same package, three extras (see [pyproject.toml](pyproject.toml) `requires-python`):

| Command | Role |
|---------|------|
| `pip install exonware-xwsystem` or `pip install xwsystem` | **Lite** - core only, no optional format backends. |
| `pip install exonware-xwsystem[lazy]` | **Lazy** - pulls in xwlazy so missing format libraries can be installed when first needed. |
| `pip install exonware-xwsystem[full]` | **Full** - common optional dependencies pre-installed for CI or all-in-one environments. |

### Lazy / on-demand backends

With `[lazy]`, normal imports and serializer use stay the same; if a codec needs a package that is not installed, the lazy layer can install it and continue. Successful imports stay on the fast path. Details: [docs/GUIDE_01_USAGE.md](docs/GUIDE_01_USAGE.md) and lazy-system docs if present under `docs/`.

---

## Quick starts (from README)

**Serialization**

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

**Config**

Use `xwconfig.toml` at the project root, or `from exonware.xwsystem import settings` and `settings.current()` in code.

---

## Surface area (summary)

- **Serialization** - 24+ formats in this repo (text, binary, light DB bindings, tabular paths). Heavier enterprise/scientific codecs often route through xwformats while keeping the same usage style.
- **Caching** - LRU/LFU/TTL, async variants, disk and two-tier combinations, secure and observable wrappers, optional metrics export.
- **Security and validation** - Path checks, crypto helpers, audit-oriented utilities, XModel-style validation where exposed.
- **IO** - Atomic writers, filesystem helpers, archives, streaming helpers, format autodetection (`AutoSerializer`, `detect_format`).
- **Runtime** - Config, logging setup, console/CLI helpers, performance and memory monitors, tracing hooks.
- **IPC and HTTP** - Process fabric, queues/shared memory facades, HTTP client with retries and modern options.
- **Indexing** - Pluggable backends under `io/indexing` (in-memory, file-backed, B-tree ordered, full-text). See [docs/REF_15_API.md](docs/REF_15_API.md).

For the full table and feature matrix, use [README.md](README.md) and the REFs below.

---

## Benchmarks

Campaigns live under [benchmarks/](benchmarks/) with an index in [benchmarks/INDEX.md](benchmarks/INDEX.md). Methodology and numbers: [docs/REF_54_BENCH.md](docs/REF_54_BENCH.md). Treat micro-benchmarks as guidance, not guarantees on your hardware.

---

## Rust and other languages

This tree is the **reference Python implementation**. Other language facades are planned around shared contracts; there is experimental Rust under [rust/](rust/). Today, hot paths are tuned in Python first; a wholesale Rust port is not required for the performance targets documented in REF_54.

---

## Platform notes

- **Windows, Linux, macOS** are supported; CI and primary dev include Windows 10/11 and common Linux/macOS releases.
- **Windows:** optional `exonware-xwsystem[windows]` (see `pyproject.toml`) for native helpers where documented; reserved device names and worker limits are handled where the stdlib requires it.
- **Async pipes on Windows:** some async pipe entry points are limited; prefer synchronous pipes or multiprocessing where noted in module docs.
- **WIM archives:** non-Windows hosts may need `wimlib` installed separately.

See [CROSS_PLATFORM_COMPATIBILITY_REPORT.md](CROSS_PLATFORM_COMPATIBILITY_REPORT.md) if present for extended notes.

---

## Documentation

- [docs/INDEX.md](docs/INDEX.md) - start here  
- [docs/GUIDE_01_USAGE.md](docs/GUIDE_01_USAGE.md) - install, codecs, caching, production  
- [docs/REF_01_REQ.md](docs/REF_01_REQ.md), [docs/REF_22_PROJECT.md](docs/REF_22_PROJECT.md) - requirements and status  
- [docs/REF_15_API.md](docs/REF_15_API.md), [docs/REF_13_ARCH.md](docs/REF_13_ARCH.md) - API and architecture  
- [docs/REF_14_DX.md](docs/REF_14_DX.md), [docs/REF_50_QA.md](docs/REF_50_QA.md), [docs/REF_54_BENCH.md](docs/REF_54_BENCH.md), [docs/REF_51_TEST.md](docs/REF_51_TEST.md) - DX, QA, benchmarks, tests  
- [docs/compliance/](docs/compliance/) - compliance scaffolding  
- [docs/logs/](docs/logs/) - change and test logs where used  

Optional or legacy guides may also exist under `docs/`; prefer INDEX.md for the canonical list.

---

## Tests

Four layers (0.core, 1.unit, 2.integration, 3.advance) via the project runner or pytest. See [docs/REF_51_TEST.md](docs/REF_51_TEST.md).

```bash
pip install -e .
pytest
```

---

## Downstream packages

xwstorage, xwformats, xwjson, xwnode, xwdata, xwauth, xwquery, xwchat, xwui, and related `*-server` packages depend on xwsystem for shared IO, security, caching, and configuration patterns so fixes land once.

---

## License and links

MIT - see [LICENSE](LICENSE).

- **Homepage:** https://exonware.com  
- **Repository:** https://github.com/exonware/xwsystem  
- **Version:** `from exonware.xwsystem import __version__` or check `exonware.xwsystem.version`  
