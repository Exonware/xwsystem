# Universal Serialization Options

**Company**: eXonware.com  
**Author**: eXonware Backend Team  
**Version**: 0.0.1.387  
**Date**: October 27, 2025

## Overview

Universal serialization options provide a **format-agnostic** way to control serialization across all text-based formats (JSON, YAML, XML, TOML, CSV, etc.) in the xwsystem library.

Instead of learning format-specific options for each serializer, you can use a consistent set of universal options that are automatically mapped to the appropriate format-specific parameters.

## Philosophy

**?? Write once, serialize anywhere**

```python
# Same options work for ANY format
serializer.detect_and_serialize(data, format_hint='JSON', pretty=True, sorted=True)
serializer.detect_and_serialize(data, format_hint='YAML', pretty=True, sorted=True)
serializer.detect_and_serialize(data, format_hint='XML', pretty=True, sorted=True)
```

## Universal Options Reference

### `pretty: bool = False`

Human-readable formatting with indentation and newlines.

**Maps to**:
- **JSON**: `indent=2, use_orjson=False`
- **YAML**: `default_flow_style=False, indent=2`
- **XML**: `pretty_print=True`
- **TOML**: `pretty=True`

**Example**:
```python
# Pretty formatting
serializer.detect_and_serialize(data, format_hint='JSON', pretty=True)
# Output:
# {
#   "name": "example",
#   "value": 42
# }
```

### `compact: bool = False`

Minimal formatting with no unnecessary whitespace. **Overrides `pretty` option**.

**Maps to**:
- **JSON**: `indent=None, use_orjson=True`
- **YAML**: `default_flow_style=True`
- **XML**: `pretty_print=False`
- **TOML**: `pretty=False`

**Example**:
```python
# Compact formatting
serializer.detect_and_serialize(data, format_hint='JSON', compact=True)
# Output: {"name":"example","value":42}
```

### `sorted: bool = False`

Sort dictionary keys alphabetically (where applicable).

**Maps to**:
- **JSON**: `sort_keys=True`
- **YAML**: `sort_keys=True`
- **TOML**: `sort_keys=True`
- **XML**: *(ignored - XML doesn't have key ordering)*

**Example**:
```python
data = {'z': 1, 'a': 2, 'm': 3}
serializer.detect_and_serialize(data, format_hint='JSON', sorted=True)
# Output: {"a": 2, "m": 3, "z": 1}
```

### `canonical: bool = False`

Deterministic output - same input always produces identical bytes. Useful for hashing, checksums, and comparisons.

**Implies**: `sorted=True`, `ensure_ascii=True`, minimal formatting

**Maps to**:
- **JSON**: `sort_keys=True, ensure_ascii=True, indent=None, canonical=True`
- **YAML**: `canonical=True, sort_keys=True`
- **XML**: `canonical=True`
- **TOML**: `sort_keys=True`

**Example**:
```python
# Canonical - always same output for same input
serializer.detect_and_serialize(data, format_hint='JSON', canonical=True)
```

### `indent: int = 2`

Indentation level when `pretty=True`.

**Maps to**:
- **JSON**: `indent=N`
- **YAML**: `indent=N`

**Example**:
```python
# 4-space indentation
serializer.detect_and_serialize(data, format_hint='JSON', pretty=True, indent=4)
```

### `line_by_line: bool = False`

Alternative formatting with one element per line (format-specific).

**Maps to**:
- **CSV**: Line-by-line processing
- **JSONL**: Newline-delimited JSON

### `declaration: bool = True`

Include format declaration (XML-specific).

**Maps to**:
- **XML**: `xml_declaration=True` ? `<?xml version="1.0" encoding="UTF-8"?>`

**Example**:
```python
# With declaration (default)
serializer.detect_and_serialize(data, format_hint='XML', declaration=True)
# Output: <?xml version="1.0"?><root>...</root>

# Without declaration
serializer.detect_and_serialize(data, format_hint='XML', declaration=False)
# Output: <root>...</root>
```

### `ensure_ascii: bool = False`

Control ASCII escaping of non-ASCII characters.

- `False` (default): Keep Unicode characters as-is (recommended)
- `True`: Escape non-ASCII characters (e.g., `"hello"` ? `"\\u0068\\u0065\\u006C\\u006C\\u006F"`)

**Maps to**:
- **JSON**: `ensure_ascii=False`
- **YAML**: `allow_unicode=True`

**Example**:
```python
data = {'message': 'Hello ?? ??'}

# Keep Unicode (default)
serializer.detect_and_serialize(data, format_hint='JSON', ensure_ascii=False)
# Output: {"message": "Hello ?? ??"}

# Escape Unicode
serializer.detect_and_serialize(data, format_hint='JSON', ensure_ascii=True)
# Output: {"message": "Hello \\u4e16\\u754c \\ud83c\\udf0d"}
```

### `encoding: str = "utf-8"`

Character encoding for output.

**Maps to**:
- **XML**: `encoding="utf-8"`
- **CSV**: `encoding="utf-8"`

**Example**:
```python
serializer.detect_and_serialize(data, format_hint='XML', encoding='utf-16')
```

## Format Compatibility Table

| Universal Option | JSON | YAML | XML | TOML | CSV | Binary |
|-----------------|------|------|-----|------|-----|--------|
| `pretty`        | ?   | ?   | ?  | ?   | ?  | ?     |
| `compact`       | ?   | ?   | ?  | ?   | ?  | ?     |
| `sorted`        | ?   | ?   | ?  | ?   | ?  | ?     |
| `canonical`     | ?   | ?   | ?  | ?   | ?  | ??     |
| `indent`        | ?   | ?   | ?  | ?   | ?  | ?     |
| `line_by_line`  | ??   | ?   | ?  | ?   | ?  | ?     |
| `declaration`   | ?   | ?   | ?  | ?   | ?  | ?     |
| `ensure_ascii`  | ?   | ?   | ?  | ?   | ?  | ?     |
| `encoding`      | ??   | ??   | ?  | ?   | ?  | ?     |

? = Fully supported  
?? = Partial support  
? = Not applicable

## Usage Examples

### Basic Usage

```python
from exonware.xwsystem.serialization.auto_serializer import AutoSerializer

serializer = AutoSerializer()

# Pretty JSON
result = serializer.detect_and_serialize(
    data,
    format_hint='JSON',
    pretty=True
)

# Compact YAML
result = serializer.detect_and_serialize(
    data,
    format_hint='YAML',
    compact=True
)

# Sorted XML
result = serializer.detect_and_serialize(
    data,
    format_hint='XML',
    pretty=True
)
```

### File Operations

```python
# Save with universal options
serializer.auto_save_file(
    data,
    'output.json',
    pretty=True,
    sorted=True
)

# Or let format be detected from extension
serializer.auto_save_file(
    data,
    'output.yaml',
    compact=True
)
```

### Integration with xwdata

xwdata automatically uses universal options:

```python
from exonware.xwdata import XWData

data = XWData({'key': 'value'})

# Universal options work directly
data.to_file('output.json', pretty=True, sorted=True)
data.to_file('output.yaml', compact=True)
data.to_file('output.xml', declaration=False)
```

### Canonical Serialization

For checksums, hashing, or comparison:

```python
# Always same output for same input
canonical_output = serializer.detect_and_serialize(
    data,
    format_hint='JSON',
    canonical=True
)

# Use for hashing
import hashlib
hash_value = hashlib.sha256(canonical_output.encode()).hexdigest()
```

## Best Practices

### ? Do

1. **Use universal options** for format-agnostic code
2. **Use `pretty=True`** for human-readable output (logs, debugging)
3. **Use `compact=True`** for network/storage optimization
4. **Use `canonical=True`** for hashing and comparisons
5. **Use `ensure_ascii=False`** to preserve Unicode (modern systems)

### ? Don't

1. **Don't mix universal and format-specific options** (confusing)
2. **Don't use `ensure_ascii=True`** unless required for legacy systems
3. **Don't forget** that `compact` overrides `pretty`
4. **Don't use binary format options** with text formats

## Migration from Format-Specific Options

### Before (format-specific)

```python
# JSON
json_serializer = JsonSerializer(indent=2, sort_keys=True, ensure_ascii=False)
result = json_serializer.dumps(data)

# YAML
yaml_serializer = YamlSerializer(default_flow_style=False, sort_keys=True)
result = yaml_serializer.dumps(data)

# XML
xml_serializer = XmlSerializer(pretty_print=True)
result = xml_serializer.dumps(data)
```

### After (universal options)

```python
# Works for ALL formats!
serializer = AutoSerializer()

result = serializer.detect_and_serialize(data, format_hint='JSON', pretty=True, sorted=True)
result = serializer.detect_and_serialize(data, format_hint='YAML', pretty=True, sorted=True)
result = serializer.detect_and_serialize(data, format_hint='XML', pretty=True)
```

## Implementation Details

Universal options are mapped via `exonware.xwsystem.serialization.universal_options.map_universal_options()`:

```python
from exonware.xwsystem.serialization.universal_options import map_universal_options

# Map universal to format-specific
json_opts = map_universal_options('JSON', pretty=True, sorted=True)
# Returns: {'indent': 2, 'use_orjson': False, 'sort_keys': True}

yaml_opts = map_universal_options('YAML', pretty=True)
# Returns: {'default_flow_style': False, 'indent': 2}
```

## See Also

- xwsystem Serialization Guide (see SERIALIZATION.md)
- xwdata Documentation (see ../../xwdata/docs/README.md)
- [GUIDELINES_DEV.md](../../guides/GUIDE_DEV.md) - Development patterns


