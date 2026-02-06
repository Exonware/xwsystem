# Usage Guide — xwsystem

**Library:** exonware-xwsystem  
**Version:** 0.0.1  
**Last Updated:** 29-Jan-2026  

---

## Purpose

Help developers successfully use `xwsystem` in real projects with **copy‑pasteable** minimal examples that align with the DX contract (`REF_DX.md`) and the public API contract (`REF_API.md`).

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

## Lazy installation (how to use it correctly)

When installed as `exonware-xwsystem[lazy]`, dependency orchestration is enabled. In typical applications you should use normal imports (no wrapper helpers).

For advanced scenarios (forcing installation without executing an import), see `REF_USAGE.md`.

---

## Configuration (single source of truth)

If you use configuration-driven behavior, keep configuration centralized and documented. A common pattern is an `xwconfig.toml` at the project root:

```toml
[project]
name = "example"
environment = "dev"

[codecs]
default_format = "json"
```

---

## Subsystems

### Caching

For deep caching usage (LRU/LFU/TTL, decorators, security caching, patterns), see:
- `USAGE_GUIDE.md` (caching-focused usage guide)

---

## Examples

Working examples are under:
- `xwsystem/examples/`

---

## Links

- DX contract: `REF_DX.md`
- API reference: `REF_API.md`
- QA gates: `REF_QA.md`
- Test evidence: `docs/logs/tests/INDEX.md`

