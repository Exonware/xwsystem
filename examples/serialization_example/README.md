# xwsystem Serialization Examples

Comprehensive examples demonstrating xwsystem's advanced serialization features.

## Structure

```
serialization_example/
├── x0_common/              # Shared utilities
├── x1_basic_formats/       # Basic serialization (JSON, YAML, XML, etc.)
├── x2_path_access/         # Path-based partial access (get_at, set_at)
├── x3_streaming/           # Streaming operations (iter_serialize, iter_path)
├── x4_patching/            # JSON Patch operations (RFC 6902, 7386)
├── x5_key_value/           # Key-value operations (LMDB, SQLite)
└── x6_canonical/           # Canonical serialization and hashing
```

## Examples

### x1_basic_formats - Basic Serialization

Tests basic serialization/deserialization with 9 formats:
- JSON, YAML, XML, MessagePack, Pickle, CBOR, BSON, CSV, TOML
- Measures speed and file size
- Compares performance across formats

```bash
cd x1_basic_formats
python benchmark.py
```

### x2_path_access - Path-Based Access

Demonstrates reading/writing specific nodes WITHOUT full deserialization:
- JSON Pointer (RFC 6901): `/users/0/name`
- XPath for XML: `//user[@id='123']`
- Dot notation for YAML: `company.departments.engineering.manager`

```bash
cd x2_path_access
python benchmark.py
```

**Use cases**: Update single record in 10GB file, random access, partial updates

### x3_streaming - Streaming Operations

Memory-efficient processing of large datasets:
- `iter_serialize()`: Chunk-based serialization
- `iter_deserialize()`: Streaming deserialization  
- `iter_path()`: Streaming with filtering

```bash
cd x3_streaming
python benchmark.py
```

**Use cases**: Process files larger than RAM, progressive loading

### x4_patching - JSON Patch Operations

Atomic updates without full deserialization:
- RFC 6902 (JSON Patch): `[{"op": "replace", "path": "/users/0/age", "value": 31}]`
- RFC 7386 (JSON Merge Patch)
- Batch operations

```bash
cd x4_patching
python benchmark.py
```

**Use cases**: Database-like updates, version control, diff operations

### x5_key_value - Key-Value Operations

Database-backed serializers with direct key access:
- LMDB: Memory-mapped key-value store (very fast reads)
- SQLite3: SQL database operations
- `put()`, `get()`, `delete()`, `keys()`, `items()`
- Prefix-based scanning

```bash
cd x5_key_value
python benchmark.py
```

**Use cases**: O(1) lookups, database-like operations on files

### x6_canonical - Canonical Serialization

Deterministic serialization and hashing:
- `canonicalize()`: Ensures same output regardless of input order
- `hash_stable()`: Content-based hashing (SHA256, XXH3)
- `checksum()`: Data integrity verification

```bash
cd x6_canonical
python benchmark.py
```

**Use cases**: Content-based caching, deduplication, integrity verification

## Key Features

### 1. Partial Access (get_at, set_at)
```python
serializer = JsonSerializer()
json_data = serializer.dumps(large_dataset)

# Get value without full deserialization
value = serializer.get_at(json_data, "/users/12345/name")

# Update value without full load/save
updated = serializer.set_at(json_data, "/users/12345/age", 31)
```

### 2. Streaming
```python
# Process 10GB file without loading all into memory
for user in serializer.iter_path(huge_json, "users.item"):
    process(user)  # Memory-efficient!
```

### 3. JSON Patch
```python
patch = [
    {"op": "replace", "path": "/status", "value": "active"},
    {"op": "add", "path": "/tags/-", "value": "verified"}
]
patched = serializer.apply_patch(data, patch, rfc="6902")
```

### 4. Key-Value
```python
lmdb = LmdbSerializer()
lmdb.put("user:123", user_data, "./db")
user = lmdb.get("user:123", "./db")  # O(1) lookup!
```

### 5. Canonical Hashing
```python
hash1 = serializer.hash_stable({"b": 2, "a": 1})
hash2 = serializer.hash_stable({"a": 1, "b": 2})
assert hash1 == hash2  # Same content, same hash!
```

## Why xwsystem is Unique

Most serialization libraries only provide:
```python
dumps(data) / loads(data)
```

xwsystem provides **database-like operations** on serialized data:
- ✅ Partial access (get_at, set_at)
- ✅ Streaming (iter_path)
- ✅ Atomic updates (apply_patch)
- ✅ Key-value operations (LMDB, SQLite)
- ✅ Canonical hashing
- ✅ 30+ serialization formats

**ONE import replaces 50+ dependencies!**

## Quick Start

```bash
# Run all examples
for dir in x*; do
    cd $dir
    python benchmark.py
    cd ..
done
```

## Output

Each benchmark saves its output to a `data/` folder:
- Serialized files in various formats
- Before/after comparisons
- Performance metrics

## Requirements

```python
pip install exonware-xwsystem
```

All dependencies are handled automatically by xwsystem.

---

**Company**: eXonware.com  
**Author**: eXonware Backend Team  
**Email**: connect@exonware.com  
**Version**: 0.0.1  
**Date**: October 12, 2025

