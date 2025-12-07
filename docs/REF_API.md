# API Reference - exonware-xwsystem

**Library:** exonware-xwsystem  
**Version:** 0.0.1.387  
**Last Updated:** 06-Nov-2025

---

## 📊 Overview

Complete API reference for exonware-xwsystem. This document provides technical specifications for all public APIs.

---

## 🔧 Core Modules

### Serialization Module

**Import:** `from exonware.xwsystem import JsonSerializer, YamlSerializer, ...`

**Available Serializers:**
- `JsonSerializer` - JSON format
- `YamlSerializer` - YAML format
- `TomlSerializer` - TOML format
- `XmlSerializer` - XML format
- `MessagePackSerializer` - MessagePack binary
- `BsonSerializer` - BSON format
- `CborSerializer` - CBOR format
- `PickleSerializer` - Python pickle
- ... (24 total formats)

**Common API (all serializers):**
```python
serializer = JsonSerializer()

# Serialize
json_str = serializer.dumps(data)
serializer.dump_to_file(data, "file.json")

# Deserialize
data = serializer.loads(json_str)
data = serializer.load_from_file("file.json")
```

### Codec Module

**Import:** `from exonware.xwsystem.codec import CodecBase, register_codec`

**Base Class:** `CodecBase[T, R]`
- `T`: Model type (your data structure)
- `R`: Representation type (`bytes` or `str`)

**Key Classes:**
- `CodecBase[T, R]` - Base codec class
- `Serializer[T]` - Codec with bytes representation
- `Formatter[T]` - Codec with str representation
- `MediaKey` - Media type identification
- `CodecRegistry` - Global codec registry

**Methods (all codecs):**

| Method | Purpose | Direction |
|--------|---------|-----------|
| `encode(value)` | Core encoding | Model ? Format |
| `decode(repr)` | Core decoding | Format ? Model |
| `dumps(value)` | Python-style | Model ? Format |
| `loads(repr)` | Python-style | Format ? Model |
| `serialize(value)` | Explicit term | Model ? Format |
| `deserialize(repr)` | Explicit term | Format ? Model |
| `save(value, path)` | File save | Model ? File |
| `load(path)` | File load | File ? Model |
| `export(value, path)` | Business term | Model ? File |
| `import_(path)` | Business term | File ? Model |
| `to_file(value, path)` | Explicit direction | Model ? File |
| `from_file(path)` | Explicit direction | File ? Model |
| `write(value, stream)` | Stream write | Model ? Stream |
| `read(stream)` | Stream read | Stream ? Model |

### HTTP Client Module

**Import:** `from exonware.xwsystem import AdvancedHttpClient`

**Class:** `AdvancedHttpClient`

**Methods:**
```python
async with AdvancedHttpClient() as client:
    # GET
    response = await client.get(url)
    
    # POST
    response = await client.post(url, json=data)
    
    # PUT
    response = await client.put(url, json=data)
    
    # DELETE
    response = await client.delete(url)
    
    # Stream download
    async for chunk in client.stream_download(url):
        process(chunk)
```

**Features:**
- HTTP/2 support
- Automatic retries
- Connection pooling
- Stream support
- Mock transport for testing

### Validation Module

**Import:** `from exonware.xwsystem import xModel, Field, ValidationError`

**Pydantic-style validation:**
```python
class User(xModel):
    name: str
    email: str
    age: int = Field(ge=0, le=150)

# Validate
user = User(name="Alice", email="alice@example.com", age=30)

# Auto-coercion
user = User(name="Bob", email="bob@example.com", age="25")  # age coerced to int
```

### Caching Module

**Import:** `from exonware.xwsystem import LRUCache, AsyncLRUCache`

**Classes:**
- `LRUCache` - Sync LRU cache
- `AsyncLRUCache` - Async LRU cache  
- `LFUCache` - Least Frequently Used
- `TTLCache` - Time-based expiration

**Usage:**
```python
from exonware.xwsystem import LRUCache

cache = LRUCache(maxsize=1000)

@cache
def expensive_function(param):
    return compute(param)
```

### IPC Module

**Import:** `from exonware.xwsystem.ipc import AsyncProcessFabric`

**Class:** `AsyncProcessFabric`

**Constructor Parameters:**
- `pool_factory` *(optional)* – Callable returning a process pool instance (defaults to `ProcessPool`).
- `queue_factory` *(optional)* – Callable returning a message queue instance.
- `shared_memory_factory` *(optional)* – Callable returning a shared memory manager.
- `logger` *(optional)* – Logger instance for lifecycle events.

**Primary API:**
```python
from exonware.xwsystem.ipc import AsyncProcessFabric

fabric = AsyncProcessFabric()

async with fabric.session() as session:
    # Submit async-friendly callables to the process pool
    task = await session.submit("tasks.transform", payload)

    # Iterate over results as they complete
    async for result in session.iter_results(task):
        handle(result)

    # Publish / consume messages through the shared queue facade
    await session.publish("events.ingest", {"dataset": "customers"})
    event = await session.consume("events.ingest")
```

**Helper Methods:**
- `session()` – Async context manager that provisions shared pool, queue, and shared memory handles.
- `submit(func_ref, *args, **kwargs)` – Enqueue work on the underlying process pool.
- `iter_results(token)` – Async iterator yielding process results (supports backpressure).
- `publish(channel, message)` / `consume(channel)` – Message queue façade wrapping `MessageQueue`.
- `share(name, allocator)` – Lazily create shared memory buffers with a user-provided allocator.
- `shutdown()` – Explicitly tear down managed resources.

**Usage Notes:**
- Designed to be additive; existing direct imports (`ProcessPool`, `MessageQueue`, etc.) continue to work.
- Integrates with future monitoring hooks to emit lifecycle telemetry.
- Exceptions bubble through `ipc.errors`, preserving existing error taxonomy.

---

## 📄 Serialization Formats

See [logs/changes/CHANGE_20251030_2221_SERIALIZATION_REFERENCE.md](logs/changes/CHANGE_20251030_2221_SERIALIZATION_REFERENCE.md) for detailed format specifications.

### Text Formats (8)

1. **JSON** - Universal standard for APIs
2. **YAML** - Human-readable configs
3. **TOML** - Python package configs
4. **XML** - Legacy enterprise systems
5. **INI** - Simple configuration files
6. **CSV** - Tabular data exchange
7. **Properties** - Java-style properties
8. **Dotenv** - Environment variables

### Binary Formats (9)

1. **MessagePack** - Fast binary JSON alternative
2. **BSON** - MongoDB's binary JSON
3. **CBOR** - Concise binary object representation
4. **Pickle** - Python native serialization
5. **Marshal** - Python internal format
6. **Protobuf** - Google's data interchange
7. **Avro** - Hadoop ecosystem format
8. **Thrift** - Apache cross-language RPC
9. **Cap'n Proto** - Zero-copy serialization

### Enterprise Formats (7)

1. **Parquet** - Columnar storage for analytics
2. **ORC** - Optimized row columnar
3. **Arrow** - In-memory columnar format
4. **Feather** - Fast on-disk format
5. **HDF5** - Hierarchical data format
6. **NetCDF** - Network Common Data Form
7. **FITS** - Flexible Image Transport System

---

## 🔧 Codec System

### Understanding Codec Architecture

**Why CodecBase exists:**
- Provides 8 method pairs from 2 implementations (encode/decode)
- Consistent API across all formats
- Automatic file I/O and streaming

### Creating Custom Codecs

```python
from exonware.xwsystem.codec import CodecBase

class CustomCodec(CodecBase[MyModel, bytes]):
    codec_id = "custom"
    media_types = ["application/x-custom"]
    file_extensions = [".custom"]
    
    def encode(self, value: MyModel, *, options=None) -> bytes:
        # Implement encoding
        return bytes(str(value), 'utf-8')
    
    def decode(self, repr: bytes, *, options=None) -> MyModel:
        # Implement decoding
        return MyModel.parse(repr.decode('utf-8'))
    
    def capabilities(self):
        return CodecCapability.BIDIRECTIONAL
```

**Why minimal implementation:**
- Only implement encode/decode
- All other methods auto-generated
- Reduces boilerplate by 400+ lines

---

## 🔒 Security

### Path Validation

```python
from exonware.xwsystem import validate_path

# Validate user-provided paths
safe_path = validate_path(
    user_path,
    allowed_dir="/data",
    must_exist=True
)
```

**Why:** Prevents path traversal attacks

### Input Sanitization

```python
# Always validate before processing
from exonware.xwsystem import sanitize_input

clean_data = sanitize_input(user_input)
```

---

## ? Performance

### Choosing the Right Format

**For Speed:** MessagePack > CBOR > BSON > JSON  
**For Size:** Protobuf > MessagePack > CBOR > JSON  
**For Readability:** YAML > TOML > JSON > XML

### Benchmark Comparison (1MB data)

| Format | Serialize | Deserialize | Size |
|--------|-----------|-------------|------|
| MessagePack | 23 ms | 28 ms | 850 KB |
| JSON | 45 ms | 52 ms | 1.2 MB |
| YAML | 123 ms | 145 ms | 1.3 MB |

**Recommendation:** Use MessagePack for performance, JSON for compatibility

---

## 📚 Related Documentation

- [GUIDE_DEV.md](guides/GUIDE_DEV.md) - Development standards
- [GUIDE_TEST.md](guides/GUIDE_TEST.md) - Testing standards
- [logs/SUMMARY_CHANGE.md](logs/SUMMARY_CHANGE.md) - Version history
- [REF_ARCH.md](REF_ARCH.md) - Architecture reference

**Detailed Format Documentation:**
- [Serialization Reference](logs/changes/CHANGE_20251030_2221_SERIALIZATION_REFERENCE.md)
- [Codec Implementation](logs/changes/CHANGE_20251030_1911_CODEC_IMPLEMENTATION.md)
- [Codec Quick Start](logs/changes/CHANGE_20251030_1911_CODEC_QUICKSTART.md)

---

*Complete API reference for exonware-xwsystem version 0.0.1.387*


