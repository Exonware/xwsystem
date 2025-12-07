# Usage Guide - exonware-xwsystem

**Library:** exonware-xwsystem  
**Version:** 0.0.1.387  
**Last Updated:** 06-Nov-2025

---

## 🚀 Quick Start

### Installation

```bash
# Lite (Default) - Core Only
pip install exonware-xwsystem

# Lazy (Recommended for Development)
pip install exonware-xwsystem[lazy]

# Full (Recommended for Production)
pip install exonware-xwsystem[full]
```

### Basic Usage

```python
from exonware.xwsystem import JsonSerializer

# Create serializer
js = JsonSerializer()

# Serialize data
data = {"name": "Alice", "age": 30}
json_str = js.dumps(data)

# Deserialize
parsed = js.loads(json_str)
```

---

## 🔧 Core Capabilities

### 1. Serialization (24+ Formats)

xwsystem provides unified serialization across 24 formats:

**Text Formats:** JSON, YAML, TOML, XML, INI, CSV, Properties, Dotenv  
**Binary Formats:** MessagePack, BSON, CBOR, Pickle, Marshal, Protobuf, Avro, Thrift, Cap'n Proto, FlatBuffers  
**Columnar/Enterprise:** Parquet, ORC, Arrow, Feather, HDF5, NetCDF, FITS

### 2. HTTP Client

Advanced HTTP client with HTTP/2 support, streaming, retries, and mock transport

### 3. Validation

Pydantic-style data validation with type coercion

### 4. Caching

LRU, LFU, TTL caching with async support

### 5. CLI Utilities

Colors, progress bars, tables for beautiful CLI apps

---

## 💡 Common Use Cases

### Use Case 1: Data Serialization

```python
from exonware.xwsystem import JsonSerializer, YamlSerializer

# JSON for APIs
json_s = JsonSerializer()
api_data = {"status": "success", "data": {...}}
json_output = json_s.dumps(api_data)

# YAML for configs
yaml_s = YamlSerializer()
config = {
    "database": {"host": "localhost", "port": 5432},
    "features": ["auth", "api"]
}
yaml_output = yaml_s.dumps(config)
```

**Why this approach:**
- Consistent API across all formats
- Easy to switch formats
- Production-grade libraries under the hood

### Use Case 2: File I/O

```python
from exonware.xwsystem import JsonSerializer

js = JsonSerializer()

# Save to file
data = {"users": [...], "settings": {...}}
js.dump_to_file(data, "config.json")

# Load from file
loaded_data = js.load_from_file("config.json")
```

**Why:** Simple file operations with automatic encoding handling

### Use Case 3: HTTP Requests

```python
from exonware.xwsystem import AdvancedHttpClient

async with AdvancedHttpClient() as client:
    # GET request
    response = await client.get("https://api.example.com/data")
    data = response.json()
    
    # POST with JSON
    result = await client.post(
        "https://api.example.com/users",
        json={"name": "Alice", "email": "alice@example.com"}
    )
```

**Why:** Async-first, HTTP/2 support, automatic retries

---

## 🔧 Codec System Usage

### Creating Custom Codecs

```python
from exonware.xwsystem.codec import CodecBase, MediaKey, register_codec

class MyCodec(CodecBase[dict, bytes]):
    codec_id = "myformat"
    media_types = ["application/x-myformat"]
    file_extensions = [".myformat"]
    
    def encode(self, value, *, options=None):
        # Your encoding logic
        return str(value).encode('utf-8')
    
    def decode(self, repr, *, options=None):
        # Your decoding logic
        return eval(repr.decode('utf-8'))
    
    def capabilities(self):
        return CodecCapability.BIDIRECTIONAL

# Register codec
register_codec(MyCodec)

# Use it
codec = MyCodec()
encoded = codec.encode({"key": "value"})
decoded = codec.decode(encoded)
```

**Why:** 8 method pairs automatically generated from encode/decode

### Available Method Pairs

| Purpose | OUT (Model ? Format) | IN (Format ? Model) |
|---------|----------------------|---------------------|
| **Core** | `encode(value)` | `decode(repr)` |
| **Python** | `dumps(value)` | `loads(repr)` |
| **Explicit** | `serialize(value)` | `deserialize(repr)` |
| **File** | `save(value, path)` | `load(path)` |
| **Export** | `export(value, path)` | `import_(path)` |
| **Direction** | `to_file(value, path)` | `from_file(path)` |
| **Stream** | `write(value, stream)` | `read(stream)` |

---

## 📋 Core Module Interfaces

### IStringable - String Conversion

```python
from exonware.xwsystem import IStringable

class MyClass(IStringable):
    def __init__(self, name):
        self.name = name
    
    def to_string(self, **options) -> str:
        return f"MyClass(name={self.name})"

# Usage
obj = MyClass("example")
print(obj.to_string())  # MyClass(name=example)
```

**Why:** Consistent string conversion across xwsystem

---

## 🔒 Security Best Practices

### Input Validation

```python
from exonware.xwsystem import validate_path

# Always validate user input
safe_path = validate_path(user_provided_path, allowed_dir="/data")
data = load_file(safe_path)
```

**Why:** Prevents path traversal attacks (OWASP A5)

### Secure Serialization

```python
from exonware.xwsystem import JsonSerializer

js = JsonSerializer()

# Validate before serialization
if validate_data(user_data):
    json_output = js.dumps(user_data)
```

**Why:** Prevents injection attacks

---

## ? Performance Tips

### Tip 1: Use Binary Formats for Speed

```python
from exonware.xwsystem import MessagePackSerializer

# 3-5x faster than JSON
mps = MessagePackSerializer()
packed = mps.dumps(large_data)  # Faster encoding
unpacked = mps.loads(packed)     # Faster decoding
```

**Why:** Binary formats are significantly faster for large datasets

### Tip 2: Use Caching

```python
from exonware.xwsystem import LRUCache

cache = LRUCache(maxsize=1000)

@cache
def expensive_operation(param):
    # Cached result
    return compute(param)
```

**Why:** 80% performance improvement for repeated operations

---

## 🔍 Troubleshooting

### Issue: ImportError for Specific Format

```
ModuleNotFoundError: No module named 'yaml'
```

**Solution:**
```bash
# Install with full dependencies
pip install exonware-xwsystem[full]
```

**Why:** Lite installation doesn't include format-specific dependencies

### Issue: Slow Serialization

**Solution:**
```python
# Use binary format instead of text
from exonware.xwsystem import MessagePackSerializer

mps = MessagePackSerializer()  # 3-5x faster than JSON
```

**Why:** Binary formats optimize for speed over readability

---

## 📚 Related Documentation

- [GUIDE_MASTER.md](GUIDE_MASTER.md) - Master standards and shared constraints
- [REF_API.md](../REF_API.md) - Complete API reference
- [REF_ARCH.md](../REF_ARCH.md) - Architecture documentation
- [GUIDE_DEV.md](GUIDE_DEV.md) - Development standards
- [logs/SUMMARY_CHANGE.md](../logs/SUMMARY_CHANGE.md) - Version history

**Original Source Documents:**
- [SERIALIZATION Reference](../logs/changes/CHANGE_20251030_2221_SERIALIZATION_REFERENCE.md)
- [Codec Quick Start](../logs/changes/CHANGE_20251030_1911_CODEC_QUICKSTART.md)
- [Core Modules Analysis](../logs/changes/CHANGE_20251104_1919_CORE_MODULES.md)

---

*Last updated: 06-Nov-2025*


