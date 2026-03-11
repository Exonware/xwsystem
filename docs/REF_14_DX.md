# Developer Experience Reference — xwsystem (REF_14_DX)

**Library:** exonware-xwsystem  
**Version:** 0.0.1  
**Last Updated:** 07-Feb-2026  
**Requirements source:** [REF_01_REQ.md](REF_01_REQ.md) sec. 5–6  
**Producing guide:** [GUIDE_14_DX.md](../../docs/guides/GUIDE_14_DX.md)

---

## Overview

This document is the **DX contract** for `xwsystem`: the few-line “happy paths”, the default behavior, and the error expectations for developer-facing surfaces.

**DX priorities:** Security → Usability → Maintainability → Performance → Extensibility.

---

## Quick Start (1–3 line happy paths)

These are the **key code** entry points for `xwsystem`. Common tasks should feel like this.

### Imports & System Facade

```python
from xwsystem import JsonSerializer, XWCache, XWIndex, XWArchive
```

```python
from exonware.xwsystem.facade import XWSystem
xw = XWSystem.create()  # io, caching, security, monitoring, threading, validation, http...
```

### Serialization & Files (default path)

```python
from xwsystem import quick_serialize, quick_deserialize
payload = quick_serialize({"user": 1}, "json")
user = quick_deserialize(payload, "auto")
```

```python
from xwsystem import XWSerializer
XWSerializer().save_file({"user": 1}, "data/user.json")
user = XWSerializer().load_file("data/user.json")
```

```python
from xwsystem import XWSerializer
XWSerializer().atomic_update_path("config.json", "settings.debug", True)
```

### Caching

```python
from xwsystem import XWCache
cache = XWCache.create(strategy="LRU", capacity=1000)
cache.put("user:1", user)
```

```python
from xwsystem import XWCache

@XWCache.cached(strategy="LRU", ttl=60)
def get_user(user_id: str) -> dict: ...
```

### Indexing (JSONL / NDJSON)

```python
from xwsystem import XWIndex
user = XWIndex("users.jsonl", id_field="id").get_by_id("user_123")
```

```python
idx = XWIndex("logs.jsonl", id_field="id")
page = idx.get_page(page=10, size=100)
```

### Archiving

```python
from xwsystem import XWArchive
XWArchive.create_archive("backup.zip", ["data/", "logs/"])
```

### Security & Crypto

```python
from xwsystem import quick_hash
digest = quick_hash("secret", algorithm="sha256")
```

```python
from xwsystem import quick_encrypt, quick_decrypt
encrypted, key = quick_encrypt("top-secret")
```

### HTTP

```python
from xwsystem import HttpClient
resp = HttpClient().get("https://api.example.com/data")
```

### Validation

```python
from xwsystem import XModel, Field

class User(XModel):
    id: int = Field(...)
```

### Monitoring & System Info

```python
from xwsystem import performance_monitor
with performance_monitor("load_user"):
    user = load_user()
```

### Threading & Async

```python
from xwsystem import ThreadSafeFactory
factory = ThreadSafeFactory()
factory.register("json", str, ["json"])
```

### Unified I/O & Atomic File Ops

```python
from xwsystem import XWIO, AtomicFileWriter, safe_write_text

data = XWIO().read_file("data/users.json")
with AtomicFileWriter("data/users.json") as f: f.write(data)
safe_write_text("logs/app.log", "started\n")
```

### System Monitoring & Health

```python
from xwsystem import get_system_info, SystemMonitor

info = get_system_info()
snapshot = SystemMonitor().snapshot()
```

### Date & Time Ergonomics

```python
from xwsystem import time_ago, humanize_timedelta

label = time_ago("2025-01-01T00:00:00Z")
duration = humanize_timedelta(seconds=90)
```

### CLI Output & Progress

```python
from xwsystem import CliConsole, CliProgressBar

CliConsole().print("Hello, xwsystem!")
bar = CliProgressBar(total=100); bar.update(10)
```

### Web Utilities

```python
from xwsystem import validate_url_accessible, extract_webpage_text

ok = validate_url_accessible("https://exonware.com")
text = extract_webpage_text("https://exonware.com")
```

### IPC & Processes

```python
from xwsystem import ProcessManager, MessageQueue

pm = ProcessManager()
queue = MessageQueue("jobs")
```

### Plugins & Runtime

```python
from xwsystem import PluginManager, EnvironmentManager

plugins = PluginManager().discover("myapp.plugins")
env = EnvironmentManager().get_env_summary()
```

### Windows UTF-8 console (default path)

```python
from exonware.xwsystem.console.cli import ensure_utf8_console
ensure_utf8_console()
```

### Lazy installation (default path)

Install:

```bash
pip install exonware-xwsystem[lazy]
```

Use normal imports (no wrappers):

```python
from xwsystem import JsonSerializer
```

---

## DX rules for xwsystem public APIs

### One obvious default for common tasks

- **Serialization & files**: use `quick_serialize` / `quick_deserialize`, `XWSerializer` helpers (`save_file`, `load_file`, `atomic_update_path`, etc.), and the high-level `save_file` / `load_file` functions exposed by the public facade (see `REF_15_API.md`).
- **Caching**: use `XWCache` (including `XWCache.create` and `@XWCache.cached`) instead of wiring low-level cache classes directly.
- **Indexing**: use `XWIndex` for JSONL/NDJSON indexing, paging, and ID-based lookups.
- **Archiving**: use `XWArchive.create_archive` / `XWArchive.extract_archive` for archive operations.
- **Security & crypto**: use `quick_hash`, `quick_encrypt` / `quick_decrypt`, and `hash_password` / `verify_password` for common security tasks.
- **HTTP**: use `HttpClient` / `AdvancedHttpClient` with `RetryConfig` for network calls.
- **Validation**: use `XModel` and `DataValidator` for schema-based and ad-hoc validation.
- **Monitoring & system info**: use `performance_monitor`, `SystemMonitor`, and helpers like `get_system_info`.
- **Console**: use `ensure_utf8_console()` on Windows; do not manually wrap stdio encoders.
- **Config**: configuration should have one obvious source of truth (documented in this library’s usage and API reference documentation).

### Unified Facades (`XW*` family)

- **System composition**: `XWSystem.create()` is the **default** way to bundle IO, caching, security, monitoring, threading, validation, and HTTP for apps that want a single entry point.
- **I/O**: `XWIO` is the **default facade** for safe file operations and atomic I/O (backed by `AtomicFileWriter`, `safe_read_*`, `safe_write_*`).
- **Caching**: `XWCache` is the **default caching facade** (LRU/LFU/TTL, async, secure, decorators, named caches).
- **Indexing**: `XWIndex` is the **default facade** for JSONL/NDJSON indexing, random access, and paging.
- **Archiving**: `XWArchive` is the **default facade** for archive formats and backup flows.
- **Security**: `XWSecurity` / `XWCrypto` are the **default facades** for higher-level security operations; `quick_hash`, `quick_encrypt`, and password helpers are the one-liner DX surface on top.
- **HTTP**: `XWHTTP` is the **unified HTTP facade**; `HttpClient` / `AdvancedHttpClient` are the underlying clients for more control.
- **Validation**: `XWValidator` is the **facade** on top of `XModel`, `DataValidator`, and type-safety helpers.
- **Monitoring & concurrency**: `XWMonitor` and `XWConcurrency` are the **facades** for performance monitoring, limits, and threading/async primitives.

### Error expectations (actionable by default)

- Errors must be **specific** and **actionable**:
  - what failed
  - why it failed
  - what the developer should do next
- “Fast but dangerous” paths must be explicit and clearly documented when they exist.

---

## Developer-facing documentation expectations

- Every major subsystem must have:
  - a minimal example (copy-pasteable)
  - a slightly-advanced example (realistic)
- Examples must match real APIs and stay in sync with `REF_15_API.md`.

---

## Common DX bugs (what to capture and how to act)

- **Confusing API surface / too many options** → capture in this DX reference and create an implementation or documentation task.
- **Unclear errors** → add a minimal reproduction, improve error messages, and log the change in `docs/logs/changes/`.
- **Slow workflow or performance regression** → capture in this DX reference and update benchmarks / SLAs in `REF_54_BENCH.md` and benchmark evidence under `docs/logs/benchmarks/`.

---

## Links

- Requirements: [REF_01_REQ.md](REF_01_REQ.md) sec. 5–6
- API reference: [REF_15_API.md](REF_15_API.md)
- Planning: [REF_21_PLAN.md](REF_21_PLAN.md) (milestones)
- Usage: [GUIDE_01_USAGE.md](GUIDE_01_USAGE.md)
- QA reference: [REF_50_QA.md](REF_50_QA.md)

