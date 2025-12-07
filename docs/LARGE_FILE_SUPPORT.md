# Large File Support (1KB - 10GB+)

**Company:** eXonware.com  
**Author:** Eng. Muhammad AlShehri  
**Version:** 0.0.1  
**Date:** 2025-11-02

## Overview

All serializers in `xwsystem/io` and `xwformats` are designed to handle both **small files (1KB)** and **large files (10GB+)** efficiently through:

1. **Safety Validation System** (inherited from `ACodec` base class)
2. **File-Size-Aware Validation** (automatically skips size checks for large files)
3. **Lazy Loading Support** (for formats that support it)
4. **Atomic Operations** (work with large files without full validation)

## Architecture

### Base Class Hierarchy

```
ACodec (base class)
  ├── Safety validation (depth + size limits)
  ├── Caching for depth/size calculations
  └── File-size-aware validation (skips size check for files > 1GB)

ASerialization (inherits from ACodec)
  ├── All ACodec features
  ├── File I/O operations
  ├── Atomic path operations (skip size check for large files)
  └── supports_lazy_loading property

All Serializers (inherit from ASerialization)
  ├── Automatic safety validation
  ├── Large file support (via base class)
  └── Format-specific optimizations
```

## Safety Validation System

### Depth Validation (ALWAYS performed)
- **Purpose:** Prevents infinite recursion (security issue)
- **Default limit:** 100 levels of nesting
- **When:** Always checked, even for large files
- **Why:** Deep nesting causes parser hangs, not file size

### Size Validation (Conditional)
- **Small files (< 1GB):** Full validation (depth + size)
- **Large files (> 1GB):** Only depth validation (size check skipped)
- **Why:** Large files are expected and use lazy loading/streaming

### File-Size Detection

The system automatically detects file size and adjusts validation:

```python
# In ACodec._validate_data_limits()
if file_path:
    path_obj = Path(file_path)
    if path_obj.exists():
        file_size_mb = path_obj.stat().st_size / (1024 * 1024)
        if file_size_mb > 1024:  # 1GB threshold
            return  # Skip size check for large files
```

## Serializer Support Matrix

### xwsystem/io Serializers

| Format | Inherits Safety | Lazy Loading | Large File Ready |
|--------|----------------|--------------|------------------|
| JSON | ✅ Yes | ✅ Yes | ✅ Yes |
| JSON5 | ✅ Yes | ⚠️ Limited* | ✅ Yes |
| YAML | ✅ Yes | ✅ Yes | ✅ Yes |
| XML | ✅ Yes | ✅ Yes | ✅ Yes |
| TOML | ✅ Yes | ⚠️ Limited | ✅ Yes |
| BSON | ✅ Yes | ✅ Yes | ✅ Yes |
| MessagePack | ✅ Yes | ✅ Yes | ✅ Yes |
| CBOR | ✅ Yes | ✅ Yes | ✅ Yes |
| Pickle | ✅ Yes | ✅ Yes | ✅ Yes |
| Marshal | ✅ Yes | ✅ Yes | ✅ Yes |
| CSV | ✅ Yes | ✅ Yes | ✅ Yes |
| JSONL | ✅ Yes | ✅ Yes | ✅ Yes |
| ConfigParser | ✅ Yes | ⚠️ Limited | ✅ Yes |
| SQLite3 | ✅ Yes | ✅ Yes | ✅ Yes |
| Shelve | ✅ Yes | ✅ Yes | ✅ Yes |
| DBM | ✅ Yes | ✅ Yes | ✅ Yes |

*JSON5 has stricter limits (50 depth, 10MB) due to parser limitations

### xwformats Serializers

| Format | Inherits Safety | Lazy Loading | Large File Ready |
|--------|----------------|--------------|------------------|
| Protobuf | ✅ Yes | ✅ Yes | ✅ Yes |
| Avro | ✅ Yes | ✅ Yes | ✅ Yes |
| Thrift | ✅ Yes | ✅ Yes | ✅ Yes |
| FlatBuffers | ✅ Yes | ✅ Yes | ✅ Yes |
| Cap'n Proto | ✅ Yes | ✅ Yes | ✅ Yes |
| Parquet | ✅ Yes | ✅ Yes | ✅ Yes |
| ORC | ✅ Yes | ✅ Yes | ✅ Yes |
| LevelDB | ✅ Yes | ✅ Yes | ✅ Yes |
| LMDB | ✅ Yes | ✅ Yes | ✅ Yes |
| HDF5 | ✅ Yes | ✅ Yes | ✅ Yes |
| Feather | ✅ Yes | ✅ Yes | ✅ Yes |
| Zarr | ✅ Yes | ✅ Yes | ✅ Yes |
| NetCDF | ✅ Yes | ✅ Yes | ✅ Yes |
| UBJSON | ✅ Yes | ✅ Yes | ✅ Yes |

## Usage Examples

### Small Files (1KB - 1MB)

```python
from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer

serializer = JsonSerializer()

# Full validation (depth + size)
data = {"key": "value"}
serializer.save_file(data, "small.json")  # ✅ Validates both depth and size
```

### Large Files (10GB+)

```python
# Automatic: Size validation skipped for files > 1GB
serializer.save_file(large_data, "large.json")  # ✅ Only validates depth

# Atomic operations automatically skip size checks
serializer.atomic_read_path("large.json", "/path/to/value")  # ✅ Works!
serializer.atomic_update_path("large.json", "/path/to/value", new_value)  # ✅ Works!
```

### Explicit Control

```python
# Skip size validation explicitly (for atomic operations)
serializer.load_file("large.json", skip_size_check=True)  # ✅ Only depth check

# Skip all validation (not recommended)
serializer.save_file(data, "file.json", skip_validation=True)  # ⚠️ Use with caution
```

## Implementation Details

### Automatic File-Size Detection

The system checks file size automatically:

1. **Before encoding:** Checks if target file exists and is large
2. **During atomic operations:** Automatically skips size check
3. **During load:** Checks source file size

### Caching System

Depth and size calculations use caching to avoid reprocessing:

```python
# In ACodec
self._depth_cache: Dict[int, int] = {}  # obj_id -> depth
self._size_cache: Dict[int, float] = {}  # obj_id -> size_mb
```

This ensures:
- Same object processed only once
- Fast validation for repeated structures
- Memory efficient for large datasets

## Testing

All serializers are tested for:

1. ✅ Small file handling (1KB)
2. ✅ Large file handling (10GB+)
3. ✅ Depth validation (prevents infinite recursion)
4. ✅ Size validation (skipped for large files)
5. ✅ Atomic operations (work with large files)

## Root Cause Analysis

### Problem
- Serializers could hang on very large/deep nested structures
- Size validation blocked legitimate large files (10GB+)
- Atomic operations failed on large files

### Solution
1. **Moved safety validation to `ACodec`** - applies to all codecs
2. **File-size-aware validation** - skips size check for files > 1GB
3. **Depth validation always performed** - prevents infinite recursion
4. **Atomic operations skip size checks** - work with large files
5. **Caching system** - avoids reprocessing same objects

### Priority Alignment
- **Priority #1 (Security):** Depth validation prevents DoS via excessive nesting
- **Priority #4 (Performance):** Size validation skipped for large files, caching improves performance

## Conclusion

✅ **All serializers in `xwsystem/io` and `xwformats` are ready to handle:**
- Small files (1KB) with full validation
- Large files (10GB+) with lazy loading support
- Atomic operations on large files
- Efficient caching to avoid reprocessing

The system automatically adapts validation based on file size, ensuring both security (depth checks) and usability (large file support).

