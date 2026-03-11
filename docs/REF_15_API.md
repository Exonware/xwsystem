# API Reference — exonware-xwsystem

**Library:** exonware-xwsystem  
**Version:** 0.0.1.387  
**Last Updated:** 20-Feb-2026  
**Requirements source:** [REF_01_REQ.md](REF_01_REQ.md) sec. 6

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
serializer.save_file(data, "file.json")

# Deserialize
data = serializer.loads(json_str)
data = serializer.load_file("file.json")
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

**Cross-library reuse:** xwschema **SchemaCatalog**, xwstorage **json_utils** and **CompositeIndex**, and xwsystem **SerializableCache** (format='json'), **io/indexing** file-backed backends, and **io/serialization** (sqlite3, dbm, etc.) all use **get_serializer(JsonSerializer)** for JSON so one parser and one file format are shared. xwschema **ConfluentSchemaRegistry** uses **create_cache** when `cache_size > 0` for get_schema/get_latest_schema (LRU). xwstorage **StorageCompression** extends **io.archive.compression.Compression** for gzip/bz2/lzma and adds zstd/snappy/lz4; used by WAL, deduplication, and time-series compression.

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
- `SerializableCache` - LRU cache with `save_to_file` / `load_from_file` (formats: `pickle`, `json`). When format is `json`, uses **get_serializer(JsonSerializer)** for persistence (same stack as indexing and xwstorage/json_utils).

**Usage:**
```python
from exonware.xwsystem import LRUCache

cache = LRUCache(maxsize=1000)

@cache
def expensive_function(param):
    return compute(param)
```

### Indexing Module (io/indexing)

**Import:** `from exonware.xwsystem.io.indexing import IIndexBackend, InMemoryIndexBackend, FileBackedIndexBackend, BTreeIndexBackend, FullTextIndexBackend, FileBackedFullTextIndexBackend`

Pluggable index backends for general-purpose physical indexes. Used by file-based stores (e.g. XWJSON) and xwstorage for key-based, ordered, and full-text access. **For file-backed indexing**, the stack provides: **key-value** (InMemoryIndexBackend, FileBackedIndexBackend), **ordered range scan** (BTreeIndexBackend with `range_scan`), and **full-text term index** (FullTextIndexBackend, FileBackedFullTextIndexBackend with `index`/`search`/`search_any`). All are available in xwsystem for file-backed use; indexing criterion is satisfied.

| Backend | Use case | Production note |
|---------|----------|-----------------|
| **InMemoryIndexBackend** | Tests, small indexes | In-memory dict; no persistence. |
| **FileBackedIndexBackend** | File-backed key/value index | Persists to a single JSON file. |
| **BTreeIndexBackend** | Ordered keys, range scans | Keys kept in sort order; `list_keys("")` yields sorted keys; `range_scan(start_key, end_key)` yields `(key, value)` in order for `[start_key, end_key]`. Use when you need B-tree–like ordered iteration or range queries. |
| **FullTextIndexBackend** | Term index, text search | In-memory term→doc_ids index. `index(doc_id, text)`, `search(query)` (AND), `search_any(query)` (OR), `remove_document(doc_id)`. No ranking; tokenization is lowercase alphanumeric. Use for simple full-text matching; for ranking use a dedicated search engine. |
| **FileBackedFullTextIndexBackend** | Persistent full-text index | Same API as FullTextIndexBackend; persists to JSON (load on init, save on index/remove_document). Use when the term index must survive process restarts. |

**IIndexBackend** protocol: `get(key)`, `put(key, value)`, `delete(key)`, `list_keys(prefix="")`, `contains(key)`. InMemory, FileBacked, and BTree backends implement it. **BTreeIndexBackend** additionally provides `range_scan(start_key, end_key)`. **FullTextIndexBackend** and **FileBackedFullTextIndexBackend** have a different API (index/search) and do not implement IIndexBackend.

**Reusability:** FileBackedIndexBackend and FileBackedFullTextIndexBackend use xwsystem **JsonSerializer** (via `get_serializer(JsonSerializer)`) for load/save so persistence shares the same serialization stack and fast parser (e.g. orjson when available).

**Tests:** `tests/1.unit/io_tests/indexing_tests/test_indexing_backend.py` (all backends including FileBackedFullTextIndexBackend; QA: tokenization, unicode, persistence).

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

## 🔐 Security — At-Rest Encryption

**Import:** `from exonware.xwsystem.security import AES256GCMAtRest, XChaCha20Poly1305AtRest, FernetAtRest, get_at_rest_encryption, is_envelope, build_envelope, parse_envelope`

Unified at-rest encryption for serialization/XWJSON: all strategies implement the same interface (`IAtRestEncryption`) and use a common envelope format (XWJE magic) so they are swappable and benchmarkable.

**Interface (IAtRestEncryption):**
- `encrypt(data: bytes, *, key=None, password=None, salt=None) -> bytes` — returns envelope bytes (XWJE + nonce + salt + ciphertext).
- `decrypt(payload: bytes, *, key=None, password=None) -> bytes` — parses envelope and returns plaintext.
- `algorithm_id() -> str` — e.g. `"aes256-gcm"`, `"xchacha20-poly1305"`, `"fernet"`.
- `supports_password() -> bool` — True if KDF is used when password is provided.

**Implementations:**
- `AES256GCMAtRest` — AEAD, C/OpenSSL (primary).
- `XChaCha20Poly1305AtRest` — AEAD, good when AES-NI unavailable.
- `FernetAtRest` — AES-128-CBC + HMAC (backward compatibility).

**Factory:** `get_at_rest_encryption(algorithm_id: str, key: Optional[bytes] = None) -> IAtRestEncryption`  
Supported ids: `aes256-gcm`, `xchacha20-poly1305`, `fernet`.

**Envelope helpers:** `build_envelope(ciphertext, algo_byte, nonce, salt=None)`, `parse_envelope(payload) -> (algo_byte, nonce, salt, ciphertext)`, `is_envelope(payload) -> bool`.

**Example:**
```python
from exonware.xwsystem.security import AES256GCMAtRest, get_at_rest_encryption

key = os.urandom(32)
enc = get_at_rest_encryption("aes256-gcm", key=key)
payload = enc.encrypt(b"secret data", key=key)
plain = enc.decrypt(payload, key=key)
# Or with password (KDF used):
enc2 = AES256GCMAtRest()
payload2 = enc2.encrypt(b"data", password="mypass")
plain2 = enc2.decrypt(payload2, password="mypass")
```

**Key derivation (optional):** `from exonware.xwsystem.security import derive_key_pbkdf2, derive_key_argon2id, derive_key_from_password` — PBKDF2 always available; Argon2id when `argon2-cffi` is installed.

---

## 📦 Serialization — Pipeline Services

**Import:** `from exonware.xwsystem.io.serialization import EncryptionService, ArchiveService, BinaryService, apply_pipeline_save, apply_pipeline_load`

Composable bytes-in/bytes-out services applied after encode / before decode. Any format (XWJSON, JSON, YAML, etc.) can use them via `save_file` / `load_file` options.

**Services:**
- **EncryptionService** — `encrypt(data, key=..., password=..., algorithm="aes256-gcm")`, `decrypt(payload, key=..., password=...)`, `is_encrypted(payload)`.
- **ArchiveService** — `compress(data, format="gzip"|"zst"|"lz4")`, `decompress(data, format=None)` (auto-detect when format is None).
- **BinaryService** — `wrap(data, use_length_prefix=False)`, `unwrap(data, use_length_prefix=False)`.

**Pipeline:** `apply_pipeline_save(repr_data, options)` and `apply_pipeline_load(raw_bytes, options)` where `options` may contain:
- `encryption` — dict with `key`, `password`, `algorithm` (e.g. `aes256-gcm`).
- `archive` — str (e.g. `"zst"`, `"gzip"`) or `True` for auto-detect on load.
- `binary_framing` — bool for length-prefix wrap/unwrap.

**save_file / load_file options (any serializer):** Pass `encryption=...`, `archive=...`, `binary_framing=...` (and for encryption, `key=...` or `password=...`) so the base serialization layer applies the pipeline. Default: no encryption, no archive, binary only if the codec is already binary.

**Example (XWJSON with encryption):**
```python
from exonware.xwjson import XWJSONSerializer
ser = XWJSONSerializer()
ser.save_file(data, "out.xwjson", password="secret")
data = ser.load_file("out.xwjson", password="secret")
# With archive:
ser.save_file(data, "out.xwjson.zst", password="secret", archive="zst")
```

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
- [REF_13_ARCH.md](REF_13_ARCH.md) - Architecture reference

**Detailed Format Documentation:**
- [Serialization Reference](logs/changes/CHANGE_20251030_2221_SERIALIZATION_REFERENCE.md)
- [Codec Implementation](logs/changes/CHANGE_20251030_1911_CODEC_IMPLEMENTATION.md)
- [Codec Quick Start](logs/changes/CHANGE_20251030_1911_CODEC_QUICKSTART.md)

---

*Complete API reference for exonware-xwsystem version 0.0.1.387*


