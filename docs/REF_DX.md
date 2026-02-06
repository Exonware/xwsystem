# DX Reference — xwsystem

**Library:** exonware-xwsystem  
**Version:** 0.0.1  
**Last Updated:** 29-Jan-2026  

---

## Overview

This document is the **DX contract** for `xwsystem`: the few-line “happy paths”, the default behavior, and the error expectations for developer-facing surfaces.

**DX priorities:** Security → Usability → Maintainability → Performance → Extensibility.

---

## Quick Start (1–3 line happy paths)

### Serialization (default path)

```python
from exonware.xwsystem import JsonSerializer

s = JsonSerializer()
payload = s.loads(s.dumps({"ok": True}))
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
from exonware.xwsystem import JsonSerializer
```

---

## DX rules for xwsystem public APIs

### One obvious default for common tasks

- **Serialization**: use the canonical serializers exposed by the public facade (see `REF_API.md`).
- **Console**: use `ensure_utf8_console()` on Windows; do not manually wrap stdio encoders.
- **Config**: configuration should have one obvious source of truth (documented in this library’s usage and API reference documentation).

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
- Examples must match real APIs and stay in sync with `REF_API.md`.

---

## Common DX bugs (what to capture and how to act)

- **Confusing API surface / too many options** → capture in this DX reference and create an implementation or documentation task.
- **Unclear errors** → add a minimal reproduction, improve error messages, and log the change in `docs/changes/`.
- **Slow workflow or performance regression** → capture in this DX reference and update benchmarks / SLAs in `REF_BENCH.md` and benchmark evidence under `docs/logs/benchmarks/`.

---

## Links

- API reference: `REF_API.md`
- Usage documentation: `GUIDE_USAGE.md`
- QA reference: `REF_QA.md`

