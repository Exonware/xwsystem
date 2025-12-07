# Advanced Serialization Features - Requirements & Implementation Plan

**Status:** Planning  
**Date:** 2025-01-XX  
**Author:** Eng. Muhammad AlShehri  
**Email:** connect@exonware.com  
**Version:** 0.0.1.409

---

## Executive Summary

This document catalogs all advanced serialization features that should be available in `xwsystem` serialization system, identifies what's currently implemented vs. missing, and provides implementation proposals for each feature across all supported formats.

**Key Finding:** During migration to the new code structure, many advanced features were lost. This document serves as the roadmap to restore and enhance them.

---

## Table of Contents

1. [Feature Categories](#feature-categories)
2. [Format-Specific Library Capabilities](#format-specific-library-capabilities)
3. [Implementation Status Matrix](#implementation-status-matrix)
4. [Implementation Proposals](#implementation-proposals)
5. [Priority Implementation Plan](#priority-implementation-plan)

---

## Feature Categories

### 1. Partial Access (Without Full Deserialization)

**Purpose:** Read/write specific paths without loading entire file into memory.

| Method | Description | Use Case |
|--------|-------------|----------|
| `get_at(path)` | Read value at path without full deserialization | Extract single field from 10GB file |
| `set_at(path, value)` | Update value at path without full load/save | Update one record in huge dataset |
| `iter_path(path_pattern)` | Stream values matching path pattern | Process filtered subset of large file |

**Status:** ❌ **MISSING** - Documented in examples but not implemented in serializers

---

### 2. Atomic Operations

**Purpose:** Thread-safe, atomic read/write operations on file paths.

| Method | Description | Use Case |
|--------|-------------|----------|
| `atomic_read_path(file_path, path)` | Atomic read at specific path | Thread-safe partial reads |
| `atomic_update_path(file_path, path, value)` | Atomic update at specific path | Thread-safe partial updates |
| `atomic_search(file_path, query)` | Atomic search/query operation | Thread-safe queries |

**Status:** ⚠️ **PARTIAL** - `atomic_read_path()` and `atomic_update_path()` exist but **load entire file** (should use streaming)

---

### 3. Streaming Operations

**Purpose:** Memory-efficient processing of large datasets.

| Method | Description | Use Case |
|--------|-------------|----------|
| `iter_serialize(data, chunk_size)` | Stream serialization in chunks | Serialize large objects incrementally |
| `iter_deserialize(stream)` | Stream deserialization from chunks | Deserialize large files incrementally |
| `iter_path(file_path, path_pattern)` | Stream items matching pattern | Process filtered data without full load |
| `stream_serialize_async()` | Async streaming serialization | Async large file processing |
| `stream_deserialize_async()` | Async streaming deserialization | Async large file processing |

**Status:** ⚠️ **PARTIAL** - Interface exists but implementations need true streaming parsers

---

### 4. Patching Operations

**Purpose:** Apply structured updates without full deserialization.

| Method | Description | Use Case |
|--------|-------------|----------|
| `apply_patch(data, patch, rfc="6902")` | Apply JSON Patch (RFC 6902) | Database-like atomic updates |
| `apply_merge_patch(data, patch)` | Apply JSON Merge Patch (RFC 7386) | Simple merge operations |
| `create_patch(old_data, new_data)` | Generate patch from diff | Version control, change tracking |

**Status:** ❌ **MISSING** - Documented in examples but not implemented

---

### 5. Query/Search Operations

**Purpose:** Query data using format-specific query languages.

| Method | Description | Use Case |
|--------|-------------|----------|
| `query(file_path, query_expr)` | Query using format-specific language | JSONPath, XPath queries |
| `search(file_path, pattern)` | Search with pattern matching | Text search in structured data |
| `filter(file_path, predicate)` | Filter items by predicate | Functional filtering |

**Status:** ✅ **PARTIAL** - `query()` exists for JSON (JSONPath) but needs expansion to other formats

---

### 6. Canonical Operations

**Purpose:** Deterministic serialization and content-based operations.

| Method | Description | Use Case |
|--------|-------------|----------|
| `canonicalize(data)` | Canonical serialization (deterministic) | Same output regardless of input order |
| `hash_stable(data, algorithm="sha256")` | Content-based hashing | Content-based caching, deduplication |
| `checksum(data)` | Data integrity checksum | Integrity verification |

**Status:** ❌ **MISSING** - Documented in examples but not implemented

---

### 7. Incremental Operations

**Purpose:** True incremental save/load without collecting all items.

| Method | Description | Use Case |
|--------|-------------|----------|
| `incremental_save(items, file_path)` | Save items incrementally | Append to file as items arrive |
| `incremental_load(file_path)` | Load items incrementally (generator) | Process items one at a time |
| `append(item, file_path)` | Append item to file | Simple append operation |

**Status:** ⚠️ **PARTIAL** - Interface exists but needs true streaming implementations

---

### 8. Schema Validation

**Purpose:** Validate data against schemas.

| Method | Description | Use Case |
|--------|-------------|----------|
| `validate_with_schema(data, schema)` | Validate against schema | Data integrity checks |
| `validate_schema(data, schema)` | Alternative validation method | Format-specific validation |

**Status:** ⚠️ **PARTIAL** - Interface exists but needs format-specific implementations

---

### 9. Merge Operations

**Purpose:** Merge updates into existing files.

| Method | Description | Use Case |
|--------|-------------|----------|
| `merge(file_path, updates, deep=True)` | Merge updates into file | Configuration updates |
| `deep_merge(base, updates)` | Deep merge operation | Nested structure updates |

**Status:** ✅ **IMPLEMENTED** - Basic implementation exists

---

### 10. Key-Value Operations (Database Formats)

**Purpose:** Database-like operations for key-value formats.

| Method | Description | Use Case |
|--------|-------------|----------|
| `put(key, value, file_path)` | Put key-value pair | O(1) insertions |
| `get(key, file_path)` | Get value by key | O(1) lookups |
| `delete(key, file_path)` | Delete key | O(1) deletions |
| `keys(file_path, prefix=None)` | List keys (with optional prefix) | Key enumeration |
| `items(file_path, prefix=None)` | Iterate key-value pairs | Full scan operations |

**Status:** ✅ **IMPLEMENTED** - Only for LMDB/LevelDB; not in base serializers

---

### 11. Paging Operations

**Purpose:** Paginated access to large datasets.

| Method | Description | Use Case |
|--------|-------------|----------|
| `page(file_path, page_num, page_size)` | Get paginated results | UI pagination |
| `paginate(file_path, page_size)` | Iterator over pages | Batch processing |

**Status:** ❌ **MISSING** - Not implemented

---

### 12. Lazy Loading

**Purpose:** Defer parsing until data is accessed.

| Method | Description | Use Case |
|--------|-------------|----------|
| `lazy_load(file_path)` | Lazy loader that defers parsing | Memory-efficient loading |
| `lazy_access(file_path, path)` | Lazy access to specific paths | On-demand path access |

**Status:** ❌ **MISSING** - `supports_lazy_loading` property exists but implementation missing

---

## Format-Specific Library Capabilities

### JSON

| Library | Streaming | Partial Access | Query | Patch | Canonical | Recommendation |
|---------|-----------|----------------|-------|-------|-----------|----------------|
| `json` (stdlib) | ❌ | ❌ | ❌ | ❌ | ❌ | Basic only |
| `orjson` | ❌ | ❌ | ❌ | ❌ | ✅ | Fast, canonical support |
| `ijson` | ✅ | ✅ | ❌ | ❌ | ❌ | **Use for streaming** |
| `jsonpointer` | ❌ | ✅ | ❌ | ❌ | ❌ | **Use for partial access** |
| `jsonpatch` | ❌ | ❌ | ❌ | ✅ | ❌ | **Use for patching** |
| `jsonpath-ng` | ❌ | ❌ | ✅ | ❌ | ❌ | **Use for queries** |
| `ujson` | ❌ | ❌ | ❌ | ❌ | ❌ | Fast, C-based |

**Recommended Stack:**
- **Streaming:** `ijson>=3.2.0` (already in requirements)
- **Partial Access:** `jsonpointer>=2.0.0` (already in requirements)
- **Patching:** `jsonpatch>=1.33.0` (already in requirements)
- **Query:** `jsonpath-ng>=1.6.0` (already in requirements)
- **Canonical:** `orjson>=3.8.0` (already in requirements)

---

### XML

| Library | Streaming | Partial Access | Query | Schema | Canonical | Recommendation |
|---------|-----------|----------------|-------|--------|-----------|----------------|
| `xml.etree.ElementTree` | ⚠️ | ⚠️ | ⚠️ | ❌ | ❌ | Basic iterparse |
| `lxml` | ✅ | ✅ | ✅ | ✅ | ✅ | **Use for all advanced features** |
| `xmltodict` | ❌ | ❌ | ❌ | ❌ | ❌ | Dict conversion only |
| `defusedxml` | ⚠️ | ⚠️ | ⚠️ | ❌ | ❌ | Security-focused |
| `xmlschema` | ❌ | ❌ | ❌ | ✅ | ❌ | Schema validation |

**Recommended Stack:**
- **Streaming:** `lxml.iterparse()` (already in requirements: `lxml>=4.9.0`)
- **Partial Access:** `lxml.etree` with XPath
- **Query:** `lxml.etree` with XPath (native support)
- **Schema:** `xmlschema>=2.0.0` (already in requirements)
- **Canonical:** `lxml.etree.tostring()` with C14N

---

### YAML

| Library | Streaming | Partial Access | Query | Multi-doc | Round-trip | Recommendation |
|---------|-----------|----------------|-------|-----------|-----------|----------------|
| `PyYAML` | ⚠️ | ❌ | ❌ | ✅ | ❌ | Basic support |
| `ruamel.yaml` | ✅ | ⚠️ | ❌ | ✅ | ✅ | **Use for advanced features** |
| `yaml` (stdlib wrapper) | ❌ | ❌ | ❌ | ❌ | ❌ | Minimal |

**Recommended Stack:**
- **Streaming:** `ruamel.yaml.safe_load_all()` (already in requirements: `ruamel.yaml>=0.17.0`)
- **Partial Access:** Custom dot-notation parser (simple implementation)
- **Multi-doc:** `ruamel.yaml` native support
- **Round-trip:** `ruamel.yaml` native support

---

### TOML

| Library | Streaming | Partial Access | Query | Notes |
|---------|-----------|----------------|-------|-------|
| `tomli` | ❌ | ❌ | ❌ | Reading only |
| `tomli-w` | ❌ | ❌ | ❌ | Writing only |
| `rtoml` | ❌ | ❌ | ❌ | Fast Rust-based |

**Note:** TOML is not designed for streaming. Partial access via dot notation is possible with custom implementation.

**Recommended Approach:**
- **Partial Access:** Custom dot-notation parser (hard-coded, simple)
- **Streaming:** Not applicable (TOML is not streamable by design)

---

### CSV

| Library | Streaming | Partial Access | Query | Type Inference | Recommendation |
|---------|-----------|----------------|-------|----------------|----------------|
| `csv` (stdlib) | ✅ | ⚠️ | ❌ | ❌ | Basic streaming |
| `pandas` | ✅ | ✅ | ✅ | ✅ | **Use for advanced features** |
| `csvkit` | ✅ | ⚠️ | ✅ | ✅ | Command-line tools |

**Recommended Stack:**
- **Streaming:** `csv.reader()` (stdlib) for basic, `pandas.read_csv(chunksize=)` for advanced
- **Partial Access:** `pandas` DataFrame indexing
- **Query:** `pandas` DataFrame filtering
- **Type Inference:** `pandas` automatic type detection

**Note:** `pandas` is large (~200MB). Consider making it optional or using `csv` (stdlib) for basic operations.

---

### BSON

| Library | Streaming | Partial Access | Query | Notes |
|---------|-----------|----------------|-------|-------|
| `bson` (pymongo) | ⚠️ | ⚠️ | ❌ | MongoDB format |
| `bson` (standalone) | ❌ | ❌ | ❌ | Basic only |

**Recommended Approach:**
- **Streaming:** Custom implementation using `bson.decode_file_iter()` from pymongo
- **Partial Access:** Reuse JSONPointer logic (BSON is JSON-like)
- **Query:** Reuse JSONPath logic (BSON is JSON-like)

---

### MessagePack / CBOR

| Format | Streaming | Partial Access | Query | Recommendation |
|--------|-----------|----------------|-------|---------------|
| MessagePack | ⚠️ | ❌ | ❌ | Binary format - limited |
| CBOR | ⚠️ | ❌ | ❌ | Binary format - limited |

**Recommended Approach:**
- **Streaming:** Custom implementation using `msgpack.Unpacker()` / `cbor2.CBORDecoder()`
- **Partial Access:** Not practical (binary format, no path support)
- **Query:** Not practical (binary format, no query language)

**Note:** These formats are optimized for speed, not advanced features. Focus on basic streaming only.

---

## Implementation Status Matrix

| Feature | JSON | XML | YAML | TOML | CSV | BSON | Status |
|---------|------|-----|------|------|-----|------|--------|
| `get_at()` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | **MISSING** |
| `set_at()` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | **MISSING** |
| `iter_path()` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | **MISSING** |
| `atomic_read_path()` | ⚠️ | ❌ | ❌ | ❌ | ❌ | ❌ | **Loads full file** |
| `atomic_update_path()` | ⚠️ | ❌ | ❌ | ❌ | ❌ | ❌ | **Loads full file** |
| `atomic_search()` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | **MISSING** |
| `iter_serialize()` | ⚠️ | ⚠️ | ⚠️ | ❌ | ✅ | ⚠️ | **Basic only** |
| `iter_deserialize()` | ⚠️ | ⚠️ | ⚠️ | ❌ | ✅ | ⚠️ | **Basic only** |
| `apply_patch()` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | **MISSING** |
| `query()` | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | **JSON only** |
| `canonicalize()` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | **MISSING** |
| `hash_stable()` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | **MISSING** |
| `incremental_save()` | ⚠️ | ⚠️ | ⚠️ | ❌ | ✅ | ⚠️ | **Basic only** |
| `incremental_load()` | ⚠️ | ⚠️ | ❌ | ❌ | ✅ | ⚠️ | **Basic only** |
| `validate_with_schema()` | ⚠️ | ⚠️ | ❌ | ❌ | ❌ | ❌ | **Interface only** |
| `merge()` | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | **Implemented** |
| `page()` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | **MISSING** |
| `lazy_load()` | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | **MISSING** |

**Legend:**
- ✅ = Implemented
- ⚠️ = Partial (needs improvement)
- ❌ = Missing

---

## Implementation Proposals

### 1. Partial Access (`get_at()`, `set_at()`, `iter_path()`)

#### JSON
- **Approach:** Use external libraries
- **Libraries:**
  - `ijson>=3.2.0` for streaming `get_at()` (already in requirements)
  - `jsonpointer>=2.0.0` for `get_at()` / `set_at()` (already in requirements)
  - Custom `iter_path()` using `ijson` with path filtering
- **Implementation:** Format-specific in `JsonSerializer`
- **Priority:** **CRITICAL** (Phase 1)

#### XML
- **Approach:** Use external library
- **Library:** `lxml>=4.9.0` (already in requirements)
  - `lxml.etree.iterparse()` for streaming
  - `lxml.etree.XPath()` for path access
- **Implementation:** Format-specific in `XmlSerializer`
- **Priority:** **HIGH** (Phase 2)

#### YAML
- **Approach:** Custom implementation (hard-coded)
- **Reason:** Simple dot-notation parser, no external library needed
- **Implementation:** Format-specific in `YamlSerializer`
- **Priority:** **MEDIUM** (Phase 3)

#### TOML
- **Approach:** Custom implementation (hard-coded)
- **Reason:** Simple dot-notation parser, no external library needed
- **Implementation:** Format-specific in `TomlSerializer`
- **Priority:** **LOW** (Phase 4)

#### CSV
- **Approach:** Use external library (optional)
- **Library:** `pandas` (optional, large dependency)
- **Fallback:** Custom implementation using `csv` (stdlib)
- **Implementation:** Format-specific in `CsvSerializer`
- **Priority:** **MEDIUM** (Phase 3)

#### BSON
- **Approach:** Reuse JSON logic
- **Reason:** BSON is JSON-like, can reuse `jsonpointer` logic
- **Implementation:** Format-specific in `BsonSerializer`, delegate to JSON utilities
- **Priority:** **LOW** (Phase 4)

---

### 2. True Atomic Operations (Fix `atomic_read_path()`, `atomic_update_path()`, Add `atomic_search()`)

#### JSON
- **Approach:** Use external libraries
- **Libraries:**
  - `ijson>=3.2.0` for streaming reads (already in requirements)
  - `jsonpointer>=2.0.0` for path navigation (already in requirements)
  - Custom streaming writer for updates
- **Implementation:** 
  - `atomic_read_path()`: Use `ijson` to stream to path, return value
  - `atomic_update_path()`: Use streaming reader + writer, update in-place
  - `atomic_search()`: Use `ijson` with path filtering
- **Priority:** **CRITICAL** (Phase 1)

#### XML
- **Approach:** Use external library
- **Library:** `lxml>=4.9.0` (already in requirements)
  - `lxml.etree.iterparse()` for streaming reads
  - `lxml.etree.XPath()` for path access
  - Custom streaming writer for updates
- **Implementation:** Format-specific in `XmlSerializer`
- **Priority:** **HIGH** (Phase 2)

#### YAML / TOML / CSV / BSON
- **Approach:** Custom streaming implementations
- **Reason:** Format-specific streaming parsers needed
- **Implementation:** Format-specific in each serializer
- **Priority:** **MEDIUM** (Phase 3)

---

### 3. Patching Operations (`apply_patch()`, `apply_merge_patch()`, `create_patch()`)

#### JSON
- **Approach:** Use external library
- **Library:** `jsonpatch>=1.33.0` (already in requirements)
  - RFC 6902 (JSON Patch)
  - RFC 7386 (JSON Merge Patch) - custom implementation
- **Implementation:** Format-specific in `JsonSerializer`
- **Priority:** **HIGH** (Phase 2)

#### XML
- **Approach:** Custom implementation (hard-coded)
- **Reason:** XML patching is format-specific, no standard library
- **Implementation:** Format-specific in `XmlSerializer` using XPath updates
- **Priority:** **MEDIUM** (Phase 3)

#### YAML / TOML / CSV / BSON
- **Approach:** Custom implementation (hard-coded)
- **Reason:** Format-specific patching logic needed
- **Implementation:** Format-specific in each serializer
- **Priority:** **LOW** (Phase 4)

---

### 4. Canonical Operations (`canonicalize()`, `hash_stable()`, `checksum()`)

#### Universal Approach
- **Approach:** Universal implementation in base class
- **Reason:** Canonical serialization is format-agnostic (sorted keys, deterministic)
- **Implementation:** 
  - `canonicalize()`: Format-specific canonical serialization
  - `hash_stable()`: Universal hashing on canonical form
  - `checksum()`: Universal checksum on canonical form
- **Libraries:**
  - `orjson>=3.8.0` for JSON canonical (already in requirements)
  - `lxml.etree.tostring()` with C14N for XML
  - Custom for YAML/TOML (sorted keys)
- **Priority:** **MEDIUM** (Phase 3)

---

### 5. Query Operations (Expand `query()` to all formats)

#### JSON
- **Approach:** Use external library
- **Library:** `jsonpath-ng>=1.6.0` (already in requirements)
- **Status:** ✅ Already implemented
- **Priority:** **N/A** (Done)

#### XML
- **Approach:** Use external library
- **Library:** `lxml>=4.9.0` (already in requirements)
  - `lxml.etree.XPath()` for XPath queries
- **Implementation:** Format-specific in `XmlSerializer`
- **Priority:** **HIGH** (Phase 2)

#### YAML / TOML / CSV / BSON
- **Approach:** Custom implementation (hard-coded)
- **Reason:** Format-specific query languages needed
- **Implementation:** Format-specific in each serializer
- **Priority:** **LOW** (Phase 4)

---

### 6. Paging Operations (`page()`, `paginate()`)

#### Universal Approach
- **Approach:** Universal implementation in base class
- **Reason:** Pagination is format-agnostic (slice data, return page)
- **Implementation:** 
  - `page()`: Load data, slice, return page
  - `paginate()`: Generator yielding pages
- **Optimization:** For large files, use streaming + pagination
- **Priority:** **LOW** (Phase 4)

---

### 7. Lazy Loading (`lazy_load()`, `lazy_access()`)

#### JSON
- **Approach:** Use external library
- **Library:** `ijson>=3.2.0` (already in requirements)
- **Implementation:** Lazy loader that defers parsing until access
- **Priority:** **MEDIUM** (Phase 3)

#### XML
- **Approach:** Use external library
- **Library:** `lxml>=4.9.0` (already in requirements)
  - `lxml.etree.iterparse()` for lazy parsing
- **Implementation:** Format-specific in `XmlSerializer`
- **Priority:** **MEDIUM** (Phase 3)

#### YAML / TOML / CSV / BSON
- **Approach:** Custom implementation (hard-coded)
- **Reason:** Format-specific lazy loading needed
- **Implementation:** Format-specific in each serializer
- **Priority:** **LOW** (Phase 4)

---

### 8. True Streaming Operations (Fix `iter_serialize()`, `iter_deserialize()`)

#### JSON
- **Approach:** Use external library
- **Library:** `ijson>=3.2.0` for deserialization (already in requirements)
- **Implementation:** Custom streaming serializer for serialization
- **Priority:** **HIGH** (Phase 2)

#### XML
- **Approach:** Use external library
- **Library:** `lxml>=4.9.0` (already in requirements)
  - `lxml.etree.iterparse()` for deserialization
  - Custom streaming serializer for serialization
- **Implementation:** Format-specific in `XmlSerializer`
- **Priority:** **HIGH** (Phase 2)

#### YAML
- **Approach:** Use external library
- **Library:** `ruamel.yaml>=0.17.0` (already in requirements)
  - `ruamel.yaml.safe_load_all()` for multi-doc streaming
- **Implementation:** Format-specific in `YamlSerializer`
- **Priority:** **MEDIUM** (Phase 3)

#### CSV
- **Approach:** Use standard library
- **Library:** `csv` (stdlib)
  - `csv.reader()` / `csv.writer()` for streaming
- **Status:** ✅ Already works
- **Priority:** **N/A** (Done)

#### TOML / BSON
- **Approach:** Custom implementation (hard-coded)
- **Reason:** Format-specific streaming needed
- **Implementation:** Format-specific in each serializer
- **Priority:** **LOW** (Phase 4)

---

## Priority Implementation Plan

### Phase 1: Core Partial Access (CRITICAL) - **Weeks 1-2**

**Goal:** Enable partial access without loading entire files.

1. **JSON `get_at()` / `set_at()` / `iter_path()`**
   - Use `ijson>=3.2.0` for streaming reads
   - Use `jsonpointer>=2.0.0` for path navigation
   - Custom `iter_path()` using `ijson` with filtering
   - **Approach:** External libraries (already in requirements)
   - **Files:** `xwsystem/src/exonware/xwsystem/io/serialization/formats/text/json.py`

2. **Fix JSON `atomic_read_path()` / `atomic_update_path()`**
   - Replace full-file load with streaming
   - Use `ijson` for reads, custom streaming writer for updates
   - **Approach:** External libraries (already in requirements)
   - **Files:** `xwsystem/src/exonware/xwsystem/io/serialization/formats/text/json.py`

**Deliverables:**
- ✅ JSON partial access working
- ✅ JSON atomic operations using streaming
- ✅ Tests for all features

---

### Phase 2: XML & Query Expansion (HIGH) - **Weeks 3-4**

**Goal:** Add XML support and expand queries to XML.

1. **XML `get_at()` / `set_at()` / `iter_path()`**
   - Use `lxml>=4.9.0` for streaming and XPath
   - **Approach:** External library (already in requirements)
   - **Files:** `xwsystem/src/exonware/xwsystem/io/serialization/formats/text/xml.py`

2. **Fix XML `atomic_read_path()` / `atomic_update_path()`**
   - Use `lxml.etree.iterparse()` for streaming
   - **Approach:** External library (already in requirements)
   - **Files:** `xwsystem/src/exonware/xwsystem/io/serialization/formats/text/xml.py`

3. **XML `query()` with XPath**
   - Use `lxml.etree.XPath()` for queries
   - **Approach:** External library (already in requirements)
   - **Files:** `xwsystem/src/exonware/xwsystem/io/serialization/formats/text/xml.py`

4. **JSON `apply_patch()`**
   - Use `jsonpatch>=1.33.0` for RFC 6902
   - Custom implementation for RFC 7386 (JSON Merge Patch)
   - **Approach:** External library (already in requirements)
   - **Files:** `xwsystem/src/exonware/xwsystem/io/serialization/formats/text/json.py`

**Deliverables:**
- ✅ XML partial access working
- ✅ XML atomic operations using streaming
- ✅ XML queries with XPath
- ✅ JSON patching operations

---

### Phase 3: Canonical, Lazy Loading, YAML (MEDIUM) - **Weeks 5-6**

**Goal:** Add canonical operations, lazy loading, and YAML support.

1. **Universal `canonicalize()` / `hash_stable()` / `checksum()`**
   - Format-specific canonical serialization
   - Universal hashing on canonical form
   - **Approach:** Universal base class + format-specific
   - **Files:** 
     - `xwsystem/src/exonware/xwsystem/io/serialization/base.py` (universal)
     - Format-specific serializers (canonical serialization)

2. **JSON / XML `lazy_load()` / `lazy_access()`**
   - Use `ijson` for JSON, `lxml.iterparse()` for XML
   - **Approach:** External libraries (already in requirements)
   - **Files:** Format-specific serializers

3. **YAML `get_at()` / `set_at()` / `iter_path()`**
   - Custom dot-notation parser
   - Use `ruamel.yaml` for multi-doc streaming
   - **Approach:** Custom implementation (hard-coded)
   - **Files:** `xwsystem/src/exonware/xwsystem/io/serialization/formats/text/yaml.py`

4. **Fix Streaming Operations**
   - True streaming for JSON/XML/YAML
   - **Approach:** External libraries (already in requirements)
   - **Files:** Format-specific serializers

**Deliverables:**
- ✅ Canonical operations working
- ✅ Lazy loading for JSON/XML
- ✅ YAML partial access
- ✅ True streaming operations

---

### Phase 4: Remaining Formats & Features (LOW) - **Weeks 7-8**

**Goal:** Complete remaining formats and features.

1. **TOML / CSV / BSON Partial Access**
   - Custom implementations
   - **Approach:** Custom implementation (hard-coded)
   - **Files:** Format-specific serializers

2. **Paging Operations**
   - Universal implementation in base class
   - **Approach:** Universal base class
   - **Files:** `xwsystem/src/exonware/xwsystem/io/serialization/base.py`

3. **Remaining Format Features**
   - Complete missing features for all formats
   - **Approach:** Mix of external libraries and custom implementations
   - **Files:** Format-specific serializers

**Deliverables:**
- ✅ All formats have partial access
- ✅ Paging operations available
- ✅ Complete feature set

---

## Summary of Implementation Approaches

| Feature Category | Approach | Rationale |
|-----------------|----------|-----------|
| **JSON Partial Access** | External libraries (`ijson`, `jsonpointer`) | Mature, well-tested libraries |
| **XML Partial Access** | External library (`lxml`) | Industry-standard, full-featured |
| **YAML Partial Access** | Custom implementation | Simple dot-notation, no library needed |
| **TOML Partial Access** | Custom implementation | Simple dot-notation, no library needed |
| **CSV Partial Access** | Optional external (`pandas`) or custom | `pandas` is large, custom is simple |
| **BSON Partial Access** | Reuse JSON logic | BSON is JSON-like |
| **Patching (JSON)** | External library (`jsonpatch`) | RFC 6902 standard |
| **Patching (Other)** | Custom implementation | Format-specific, no standard |
| **Canonical Operations** | Universal base + format-specific | Format-agnostic concept |
| **Query (JSON)** | External library (`jsonpath-ng`) | Already implemented |
| **Query (XML)** | External library (`lxml` XPath) | Industry-standard |
| **Query (Other)** | Custom implementation | Format-specific |
| **Paging** | Universal base class | Format-agnostic concept |
| **Lazy Loading** | External libraries (`ijson`, `lxml`) | Mature streaming parsers |
| **Streaming** | External libraries + custom | Mix of libraries and custom writers |

---

## Dependencies Summary

### Already in Requirements
- ✅ `ijson>=3.2.0` - JSON streaming
- ✅ `jsonpointer>=2.0.0` - JSON path access
- ✅ `jsonpatch>=1.33.0` - JSON patching
- ✅ `jsonpath-ng>=1.6.0` - JSON queries
- ✅ `orjson>=3.8.0` - JSON canonical
- ✅ `lxml>=4.9.0` - XML all features
- ✅ `ruamel.yaml>=0.17.0` - YAML advanced features
- ✅ `xmlschema>=2.0.0` - XML schema validation

### No New Dependencies Needed
All required libraries are already in `xwsystem/requirements.txt` and `xwsystem/pyproject.toml`.

---

## Next Steps

1. **Review this document** with the team
2. **Prioritize Phase 1** (JSON partial access) - **CRITICAL**
3. **Create implementation tickets** for each phase
4. **Start Phase 1 implementation** following `GUIDE_DEV.md` and `GUIDE_TEST.md`
5. **Update ADR-001** with implementation decisions

---

**Company:** eXonware.com  
**Author:** Eng. Muhammad AlShehri  
**Email:** connect@exonware.com  
**Version:** 0.0.1.409  
**Last Updated:** 2025-01-XX

