# Usage Reference - exonware-xwsystem

**Library:** exonware-xwsystem  
**Version:** 1.0.1  
**Last Updated:** 08-Nov-2025

---

## 📖 Audience & Purpose

Authoritative usage patterns for `exonware-xwsystem`. Start here to see how installation, dependency orchestration, codecs, configuration, and runtime services come together in real projects.

---

## 🚀 Quick Start

### 1. Install Once

```bash
pip install exonware-xwsystem[lazy]
```

### 2. Import Normally

```python
# No special loaders required
import cv2  # Installed on-demand if missing
from exonware.xwsystem import JsonSerializer
```

**Why:** `lazy` activates the dependency supervisor. Missing packages are resolved automatically the first time you import or access them—no manual `pip install` or helper wrappers required.

---

## 🤖 Dependency Autopilot

### How it Works

- Watches `import` statements and resolves unavailable packages through configured indexes.
- Honors per-project policies defined in `xwconfig.toml` (see Configuration section).
- Caches downloads for offline replays and CI reproducibility.

### When to Use `xwimport`

```python
from exonware.xwsystem import xwimport

cv2 = xwimport("cv2")
```

**Recommended only for:** interactive notebooks, diagnostics, or forcing installation without executing an `import`. In typical applications, stick with plain Python imports—the supervisor does the rest.

---

## ⚙️ Configuration System

### Central Contract

`xwsystem` looks for `xwconfig.toml` (or environment overrides) to control dependency, codec, IO, and security policies.

```toml
[project]
name = "vision-lab"
environment = "dev"

[dependencies]
prefer_binary = true
allow_pre_release = false

[codecs]
default_format = "json"
allow_eval_codecs = false

[paths]
data_root = "./data"
cache_dir = "~/.cache/xwsystem"
```

### Runtime Access

```python
from exonware.xwsystem import settings

cfg = settings.current()
cache_dir = cfg.paths.cache_dir
policy = cfg.dependencies.prefer_binary
```

**Why:** Single source of truth that drives dependency rules, serialization defaults, and security controls across your services, notebooks, and automation.

---

## 🔧 Codec Powerhouse

### Use Built-in Codecs

```python
from exonware.xwsystem import JsonSerializer, MessagePackSerializer

api_serializer = JsonSerializer(indent=2)
binary_serializer = MessagePackSerializer()

payload = {"status": "ok", "items": [1, 2, 3]}
json_repr = api_serializer.dumps(payload)
binary_repr = binary_serializer.dumps(payload)
```

### Author Custom Codecs in Minutes

```python
from exonware.xwsystem.codec import CodecBase, register_codec, CodecCapability

class MetricsCodec(CodecBase[dict, bytes]):
    codec_id = "metrics"
    media_types = ["application/x-metrics"]
    file_extensions = [".metrics"]

    def encode(self, value, *, options=None) -> bytes:
        return ";".join(f"{k}:{v}" for k, v in value.items()).encode()

    def decode(self, repr, *, options=None) -> dict:
        return dict(item.split(":") for item in repr.decode().split(";"))

    def capabilities(self):
        return CodecCapability.BIDIRECTIONAL

register_codec(MetricsCodec)
```

**Why:** Implement `encode` and `decode`; `xwsystem` fabricates the other 6 method pairs (file, stream, explicit aliases) automatically.

### Take Advantage of Media Keys

- Map codecs to media types, file extensions, or arbitrary IDs.
- Compose pipelines (`encode` ? compress ? encrypt) via registry metadata.

---

## 📝 Smart Serialization Workflows

### Files & Streams

```python
from exonware.xwsystem import JsonSerializer

serializer = JsonSerializer()
serializer.dump_to_file(data, "report.json")

with serializer.stream("report.json", mode="r") as stream:
    processed = serializer.read(stream)
```

### Format Negotiation

```python
from exonware.xwsystem import negotiate_codec

codec = negotiate_codec(media_type="application/json", prefer_binary=False)
representation = codec.dumps(data)
```

**Why:** Uniform API for 24+ formats means you swap transports without rewriting business logic.

---

## 🎯 Runtime Services Tour

- **Advanced HTTP Client** – HTTP/2, retry, streaming, and mock transport for tests.
- **Validation (xModel)** – Pydantic-style models with coercion, schema export, and config-driven constraints.
- **Caching Stack** – LRU/LFU/TTL decorators for sync/async workloads, with optional disk-backed layers.
- **Async Process Fabric** – Unified async facade over process pools, message queues, and shared memory helpers.
- **CLI Helpers** – Rich text, progress, and table utilities for operational tooling.
- **Metrics & Logging Hooks** – Structured logging adapters and observability meters keyed off `xwconfig`.

```python
from exonware.xwsystem import AdvancedHttpClient, LRUCache, xModel

class User(xModel):
    id: int
    email: str

cache = LRUCache(maxsize=1000)

@cache
async def fetch_user(user_id: int):
    async with AdvancedHttpClient() as client:
        resp = await client.get(f"https://api.example.com/users/{user_id}")
        return User(**resp.json())
```

### Async Process Fabric Example

```python
from exonware.xwsystem.ipc import AsyncProcessFabric

async def main(dataset_ids):
    fabric = AsyncProcessFabric()

    async with fabric.session() as session:
        job = await session.submit("tasks.dataset.ingest", dataset_ids)

        async for result in session.iter_results(job):
            print("dataset complete:", result["dataset"])

        await session.publish("events.sync", {"status": "ingest-complete"})
        message = await session.consume("events.sync")
        print("sync event:", message)

if __name__ == "__main__":
    import asyncio
    asyncio.run(main(["customers", "orders"]))
```

**Why:** `AsyncProcessFabric` gives you batteries-included orchestration for IO-bound workloads—tying together pools, queues, and shared buffers with consistent async lifecycle management and future-ready monitoring hooks.

---

## 🛡️ Operations & Governance

- **Dependency Audit:** `xwsystem` records provenance for auto-installed packages; export via `xw deps report`.
- **Sandbox Modes:** Restrict auto-installation to approved registries or freeze to curated mirrors.
- **Reproducibility:** Cache manifests and lockfiles integrate with CI pipelines (see `tools/ci/quick_release.py`).

---

## 🔍 Troubleshooting Checklist

| Scenario | Quick Fix |
|----------|-----------|
| Package fails to auto-install | Confirm registry access in `xwconfig.toml` and rerun import |
| Codec not found | Ensure `register_codec` executed during startup or declare via entry point |
| Configuration missing | Run `xw config init` to scaffold defaults |
| Lazy install disabled | Verify `xwsystem.lazy.enabled = true` in environment or config |

---

## 📚 Related Resources

- [GUIDE_USAGE.md](guides/GUIDE_USAGE.md) – Narrative walkthroughs and tutorials
- [REF_API.md](REF_API.md) – Public API surface
- [REF_ARCH.md](REF_ARCH.md) – Architecture and subsystem diagrams
- [logs/SUMMARY_CHANGE.md](logs/SUMMARY_CHANGE.md) – Release & feature history

---

*Practical usage reference for exonware-xwsystem version 1.0.1*



