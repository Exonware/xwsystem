# Codec Quick Start Guide

**Date:** October 30, 2025  
**Company:** eXonware.com

---

## ? 30-Second Overview

```python
from exonware.xwsystem.codec import CodecBase, MediaKey, register_codec

# 1. Create a codec (only implement encode/decode)
class MyCodec(CodecBase[dict, bytes]):
    codec_id = "myformat"
    media_types = ["application/x-myformat"]
    file_extensions = [".myformat"]
    
    def encode(self, value, *, options=None):
        return str(value).encode('utf-8')
    
    def decode(self, repr, *, options=None):
        return eval(repr.decode('utf-8'))
    
    def capabilities(self):
        return CodecCapability.BIDIRECTIONAL

# 2. Register it
register_codec(MyCodec)

# 3. Use it (8 method pairs available!)
codec = MyCodec()
codec.encode(data)         # Core
codec.dumps(data)          # Python convention
codec.serialize(data)      # Explicit
codec.save(data, path)     # File I/O
codec.export(data, path)   # Business term
codec.to_file(data, path)  # Explicit direction
codec.write(data, stream)  # Stream I/O
```

---

## ?? Package Structure

```
xwsystem/src/exonware/xwsystem/codec/
+-- __init__.py         Public API
+-- defs.py             CodecCapability flags
+-- errors.py           Error classes
+-- contracts.py        ICodec, Serializer, Formatter
+-- base.py             CodecBase, Registry, Adapters
```

---

## ?? Core Concepts

### ICodec[T, R]
- `T` = Model type (your data structure)
- `R` = Representation type (`bytes` or `str`)

### Specializations
- `Serializer[T]` = `ICodec[T, bytes]` - For storage/wire
- `Formatter[T]` = `ICodec[T, str]` - For language/syntax

---

## ?? Complete Method Reference

| Purpose | OUT (Model ? Format) | IN (Format ? Model) |
|---------|----------------------|---------------------|
| **Core** | `encode(value)` | `decode(repr)` |
| **Python** | `dumps(value)` | `loads(repr)` |
| **Explicit** | `serialize(value)` | `deserialize(repr)` |
| **File** | `save(value, path)` | `load(path)` |
| **Export** | `export(value, path)` | `import_(path)` |
| **Direction** | `to_file(value, path)` | `from_file(path)` |
| **Format** | `save_as(value, path, fmt)` | `load_as(path, fmt)` |
| **Stream** | `write(value, stream)` | `read(stream)` |

**All methods** except encode/decode are **automatic**!

---

## ?? Quick Examples

### Example 1: Bytes Codec (Serializer)

```python
class JsonCodec(CodecBase[dict, bytes]):
    codec_id = "json"
    media_types = ["application/json"]
    file_extensions = [".json"]
    
    def encode(self, value, *, options=None):
        import json
        return json.dumps(value).encode('utf-8')
    
    def decode(self, repr, *, options=None):
        import json
        return json.loads(repr.decode('utf-8'))
    
    def capabilities(self):
        return CodecCapability.BIDIRECTIONAL | CodecCapability.TEXT

# Use any method
codec = JsonCodec()
codec.encode({"a": 1})           # ? b'{"a":1}'
codec.dumps({"a": 1})            # ? b'{"a":1}'
codec.save({"a": 1}, "data.json")  # ? writes file
```

### Example 2: String Codec (Formatter)

```python
class SqlFormatter(CodecBase[dict, str]):
    codec_id = "sql"
    media_types = ["application/sql"]
    file_extensions = [".sql"]
    
    def encode(self, value, *, options=None):
        return f"SELECT {value['fields']} FROM {value['table']}"
    
    def decode(self, repr, *, options=None):
        # Parse SQL (simplified)
        parts = repr.split()
        return {'fields': parts[1], 'table': parts[3]}
    
    def capabilities(self):
        return CodecCapability.BIDIRECTIONAL | CodecCapability.TEXT

# Use any method
formatter = SqlFormatter()
query = {'fields': '*', 'table': 'users'}

formatter.encode(query)              # ? "SELECT * FROM users"
formatter.dumps(query)               # ? "SELECT * FROM users"
formatter.to_file(query, "q.sql")    # ? writes file
```

### Example 3: Registry

```python
from exonware.xwsystem.codec import get_codec, MediaKey

# Register once
register_codec(JsonCodec)

# Use anywhere
codec = get_codec(MediaKey("application/json"))
codec = get_codec_for_file("data.json")
codec = get_codec_by_id("json")
```

### Example 4: Adapters

```python
# SQL formatter ? bytes for file storage
formatter = SqlFormatter()  # Returns str
serializer = FormatterToSerializer(formatter)

bytes_data = serializer.encode(query)  # Returns bytes
with open("query.sql", "wb") as f:
    f.write(bytes_data)
```

---

## ? What's Tested

- ? Core encode/decode
- ? All 8 convenience method pairs
- ? MediaKey creation and lookup
- ? Registry registration and retrieval
- ? Registry caching
- ? Adapters (bytes ? str)
- ? Error handling
- ? End-to-end workflows
- ? Format conversion

**27/27 tests pass** ?

---

## ?? Design Philosophy

1. **Implement once** - Only `encode/decode` required
2. **Get 8 free** - All convenience methods automatic
3. **Self-describe** - Codec declares its own metadata
4. **NO HARDCODING** - Registry auto-discovers
5. **Type-safe** - Full generic type support

---

## ?? Documentation

- **Full Proposal:** `docs/CODEC_UNIFICATION_PROPOSAL.md`
- **Quick Reference:** `docs/CODEC_QUICK_REFERENCE.md`
- **Architecture:** `docs/CODEC_ARCHITECTURE_DIAGRAM.md`
- **Terminology:** `docs/CODEC_TERMINOLOGY_TABLE.md`
- **This Guide:** `xwsystem/codec_quick_start.md`

---

**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Date:** October 30, 2025


