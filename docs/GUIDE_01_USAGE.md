# Usage Guide — xwsystem (GUIDE_01_USAGE)

**Library:** exonware-xwsystem  
**Version:** 0.0.1  
**Last Updated:** 07-Feb-2026  
**Producing guide:** GUIDE_41_DOCS (project-local usage)

---

## Purpose

Help developers successfully use `xwsystem` in real projects with **copy‑pasteable** minimal examples that align with the DX contract ([REF_14_DX.md](REF_14_DX.md)) and the public API contract ([REF_15_API.md](REF_15_API.md)). Requirements: [REF_01_REQ.md](REF_01_REQ.md).

---

## Installation

`xwsystem` supports three install modes:

```bash
# Lite (core only)
pip install exonware-xwsystem

# Lazy (recommended for most dev workflows)
pip install exonware-xwsystem[lazy]

# Full (pre-install common optional deps)
pip install exonware-xwsystem[full]
```

---

## Quick Start (Python)

### Serialization (happy path)

```python
from exonware.xwsystem import JsonSerializer

s = JsonSerializer()
text = s.dumps({"name": "Alice", "age": 30})
data = s.loads(text)
```

### Windows console UTF‑8 (happy path)

```python
from exonware.xwsystem.console.cli import ensure_utf8_console

ensure_utf8_console()
```

---

## Lazy installation and xwimport

When installed as `exonware-xwsystem[lazy]`, dependency orchestration is enabled. Use normal imports; missing packages are resolved on first use. For interactive use or forcing install without importing:

```python
from exonware.xwsystem import xwimport

cv2 = xwimport("cv2")
```

---

## Configuration

Use `xwconfig.toml` at the project root for dependency, codec, and path policies:

```toml
[project]
name = "example"
environment = "dev"

[dependencies]
prefer_binary = true

[codecs]
default_format = "json"

[paths]
cache_dir = "~/.cache/xwsystem"
```

Runtime access: `from exonware.xwsystem import settings` then `settings.current()`.

---

## Codecs and serialization

```python
from exonware.xwsystem import JsonSerializer, MessagePackSerializer, negotiate_codec

# Built-in codecs
s = JsonSerializer(indent=2)
s.dump_to_file(data, "report.json")

# Format negotiation
codec = negotiate_codec(media_type="application/json", prefer_binary=False)
```

See [REF_15_API.md](REF_15_API.md) for the full codec API and custom codec registration.

---

## Caching

Use `create_cache()` for the benchmark-backed default (PylruCache when pylru is installed, else FunctoolsLRUCache). Use `LRUCache`, `LFUCache`, TTL decorators, and disk-backed layers from `exonware.xwsystem.caching`. Full caching patterns and security options are in [REF_15_API.md](REF_15_API.md). See [REF_54_BENCH.md](REF_54_BENCH.md) for trade-offs (e.g. TwoTierCache for memory+disk).

---

## Runtime services (summary)

- **HTTP:** `AdvancedHttpClient` (HTTP/2, retry, streaming).
- **Validation:** `xModel` (Pydantic-style) and config-driven constraints.
- **Async:** `AsyncProcessFabric` for process pools, queues, and shared memory (see [REF_22_PROJECT.md](REF_22_PROJECT.md) for session patterns).
- **Locking:** Use `fast_lock()` or `EnhancedRLock(track_stats=False)` for hot-path reentrant locks when you don't need timeout/stats (benchmark: ~3.4M ops/s). See [REF_54_BENCH.md](REF_54_BENCH.md).

---

## Production and real-world examples

Production deployment and extended real-world example value was absorbed into this guide. Use this guide plus [REF_15_API.md](REF_15_API.md) and `xwsystem/examples/` for current patterns.

---

## Related

- [REF_01_REQ.md](REF_01_REQ.md) — Requirements (when present)
- [REF_12_IDEA.md](REF_12_IDEA.md) — Idea context (when present)
- [REF_22_PROJECT.md](REF_22_PROJECT.md) — Vision and milestones (when present)
- [REF_14_DX.md](REF_14_DX.md) — DX contract  
- [REF_15_API.md](REF_15_API.md) — API reference  
- [REF_21_PLAN.md](REF_21_PLAN.md) — Milestones and roadmap (when present)
- [REF_50_QA.md](REF_50_QA.md) — QA gates  
- [INDEX.md](INDEX.md) — Full REF set and navigation
- Test evidence: `docs/logs/tests/INDEX.md`

