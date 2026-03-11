# Architecture Reference — exonware-xwsystem

**Library:** exonware-xwsystem  
**Version:** 0.0.1.387  
**Last Updated:** 07-Feb-2026  
**Requirements source:** [REF_01_REQ.md](REF_01_REQ.md)

---

## 📊 Overview

exonware-xwsystem is the foundation library for the entire eXonware ecosystem, providing core functionality through a modular, extensible architecture.

**Design Philosophy:** Simple, composable modules with consistent APIs and production-grade implementations.

---

## 🏗️ System Architecture

### High-Level Structure

```
exonware-xwsystem/
+-- io/                  # Input/Output operations
�   +-- serialization/   # 24+ serialization formats
�   +-- archive/         # Archive formats (zip, tar, etc.)
�   +-- compression/     # Compression algorithms
�   +-- http/            # HTTP client
+-- codec/               # Codec architecture (NEW)
�   +-- base.py          # CodecBase[T, R]
�   +-- registry.py      # Global codec registry
�   +-- adapters.py      # Format adapters
+-- grammar/             # Grammar and syntax system
�   +-- bidirectional/   # Bidirectional grammar support
�   +-- monaco/          # Monaco editor integration
+-- security/            # Security utilities
+-- validation/          # Data validation
+-- caching/             # Caching implementations
+-- utils/               # Utility functions
+-- shared/              # Cross-cutting concerns
```

---

## 📦 Module Breakdown

### 1. IO Module (`io/`)

**Purpose:** All input/output operations

**Sub-modules:**
- **serialization/** - 24 format serializers
- **archive/** - Archive operations (zip, tar, gzip)
- **compression/** - Compression algorithms
- **http/** - Advanced HTTP client with HTTP/2

**Design Pattern:** Strategy pattern for format handling

**Why this structure:** Separates concerns by I/O type

### 2. Codec Module (`codec/`)

**Purpose:** Universal codec architecture

**Key Components:**
- `CodecBase[T, R]` - Base codec class with 8 auto-generated method pairs
- `CodecRegistry` - Global codec registration and discovery
- `MediaKey` - Media type identification
- `Serializer[T]` - Specialization for bytes
- `Formatter[T]` - Specialization for strings

**Design Pattern:** 
- Template Method (CodecBase provides algorithm skeleton)
- Registry pattern (global codec management)
- Adapter pattern (format conversion)

**Why:** Reduces boilerplate by 400+ lines, consistent API

### 3. Grammar Module (`grammar/`)

**Purpose:** Syntax parsing and grammar systems

**Features:**
- Bidirectional grammar support
- Multiformat grammar (JSON, YAML, TOML, etc.)
- Monaco editor integration
- Built-in grammars

**Design Pattern:** Interpreter pattern

### 4. Security Module (`security/`)

**Purpose:** Security utilities and validation

**Features:**
- Path traversal protection
- Input sanitization
- Cryptographic operations
- Security validation

**Design Pattern:** Defense-in-depth

### 5. Validation Module (`validation/`)

**Purpose:** Pydantic-style data validation

**Features:**
- Type coercion
- Field validation
- Custom validators
- Error aggregation

**Design Pattern:** Specification pattern

### 6. Caching Module (`caching/`)

**Purpose:** Performance caching

**Implementations:**
- `LRUCache` - Least Recently Used
- `LFUCache` - Least Frequently Used
- `TTLCache` - Time-To-Live
- `AsyncLRUCache` - Async LRU

**Design Pattern:** Decorator pattern

### 7. IPC Module (`ipc/`)

**Purpose:** Inter-process communication primitives (process pools, message queues, shared memory managers).

**Key Components:**
- `process_pool.py` – Synchronous + async process orchestration with worker management.
- `message_queue.py` – Queue-based messaging for processes or threads.
- `shared_memory.py` – Shared buffers with synchronization helpers.
- `async_fabric.py` (NEW) – Async Process Fabric facade coordinating the above primitives with unified lifecycle hooks.

**Design Notes:**
- `AsyncProcessFabric` composes the existing primitives, wrapping them in an async-friendly orchestration API that enforces consistent startup/shutdown semantics.
- The facade exposes task submission, async iteration, and shared resource helpers while delegating execution logic to existing modules. This preserves backwards compatibility.
- Lifecycle instrumentation plugs into monitoring hooks (emitters to be wired in subsequent iterations) and surfaces consistent error handling using `ipc.errors`.

**Design Patterns:** Facade + Mediator (AsyncProcessFabric mediates coordination between queues, pools, and shared memory).

---

## 🎨 Design Patterns

### 1. Strategy Pattern (Serialization)

```
Context: SerializationManager
+-- Strategy: JsonSerializer
+-- Strategy: YamlSerializer
+-- Strategy: MessagePackSerializer
+-- ... (21 more strategies)
```

**Why:** Interchangeable serialization algorithms

### 2. Facade Pattern (Main API)

```python
# Facade hides complexity
from exonware.xwsystem import serialize, deserialize

# Simple API for complex subsystems
data = serialize(obj, format="json")
obj = deserialize(data, format="json")
```

**Why:** Simplified API hiding implementation complexity

### 3. Registry Pattern (Codec System)

```
CodecRegistry
+-- register(codec_class)
+-- get_codec(codec_id)
+-- find_by_media_type(media_type)
+-- find_by_extension(extension)
```

**Why:** Dynamic codec discovery and registration

### 4. Template Method (CodecBase)

```
CodecBase (defines skeleton)
+-- encode() - Must implement
+-- decode() - Must implement
+-- dumps() - Auto-generated from encode
+-- loads() - Auto-generated from decode
+-- save() - Auto-generated from encode + file I/O
+-- load() - Auto-generated from decode + file I/O
+-- ... (14 total methods, only 2 implemented)
```

**Why:** Reduces code duplication, ensures consistency

---

## 🔄 Data Flow

### Serialization Flow

```
Python Object
    ?
[Format Detector]  ? Auto-detect or explicit format
    ?
[Codec Registry]   ? Find appropriate codec
    ?
[Codec.encode()]   ? Format-specific encoding
    ?
Serialized Data (bytes/str)
```

### Deserialization Flow

```
Serialized Data
    ?
[Format Detector]  ? Detect from extension/media type/content
    ?
[Codec Registry]   ? Find appropriate codec
    ?
[Codec.decode()]   ? Format-specific decoding
    ?
Python Object
```

---

## 🔌 Extensibility Points

### 1. Custom Codecs

Extend `CodecBase[T, R]` to add new formats

### 2. Custom Validators

Implement custom validation logic

### 3. Custom Cache Policies

Extend cache classes for custom eviction policies

### 4. HTTP Transports

Implement custom HTTP transports (e.g., mock, retry logic)

---

## 🔒 Security Architecture

### Defense-in-Depth Layers

1. **Input Validation** - All external input validated
2. **Path Validation** - Prevent path traversal
3. **Type Checking** - Runtime type validation
4. **Sanitization** - Input sanitization before processing
5. **Error Handling** - Safe error messages (no info leakage)

**Why:** Multiple layers prevent single points of failure

---

## ? Performance Architecture

### Optimization Strategies

1. **Lazy Loading** - Import modules on first use
2. **Caching** - Aggressive caching of repeated operations
3. **Async-First** - All I/O operations support async
4. **Zero-Copy** - Minimize data copying where possible
5. **Connection Pooling** - HTTP client connection reuse

**Result:** 20-100x performance improvement in lazy mode

---

## 🧪 Testing Architecture

### 4-Layer Test Hierarchy

```
tests/
+-- runner.py         # Main orchestrator
+-- 0.core/          # 80/20 rule - high-value tests
+-- 1.unit/          # Component isolation
+-- 2.integration/   # Cross-module scenarios
+-- 3.advance/       # Excellence (v1.0.0+)
```

**Why:** Fast feedback, comprehensive coverage, hierarchical execution

---

## 🌐 Ecosystem Integration

### Current Status

**xwsystem** (complete) ? Foundation for:
- **xwnode** (planned) - Node structures
- **xwdata** (planned) - 50+ formats
- **xwschema** (planned) - Schema validation
- **xwaction** (planned) - Function decoration
- **xwentity** (planned) - Entity management

### Dependency Flow

```
xwsystem (foundation)
    ?
xwnode (uses xwsystem serialization)
    ?
xwdata (uses xwnode structures)
    ?
xwschema (uses xwdata)
    ?
xwaction (uses xwschema)
    ?
xwentity (uses xwschema + xwaction + xwdata)
```

---

## 📐 Architectural Principles

### 1. Single Responsibility

Each module has one clear purpose:
- `io/` - Input/output
- `codec/` - Format conversion
- `security/` - Security
- `caching/` - Performance caching

### 2. Dependency Inversion

High-level modules depend on abstractions (interfaces), not concrete implementations

### 3. Open/Closed Principle

Open for extension (add new codecs) but closed for modification (core API stable)

### 4. Interface Segregation

Small, focused interfaces (`IStringable`, `ICodec[T, R]`)

### 5. Liskov Substitution

All serializers interchangeable through common interface

---

## 🔢 Versioning Architecture

### Overview

All eXonware projects use a **centralized, single-source-of-truth** versioning system powered by xwsystem's `VersionManager`.

### Architecture Components

**1. Core Implementation (Single Source of Truth)**

```
xwsystem/src/exonware/xwsystem/config/version_manager.py
+-- VersionManager class (master implementation)
    +-- bump_major() / bump_minor() / bump_patch() / bump_build()
    +-- get_version_info() / get_version_dict()
    +-- is_dev_version() / is_release_version()
```

**2. Project-Specific Version Files**

Each project delegates to xwsystem's VersionManager:

```python
# Example: xwnode/src/exonware/xwnode/version.py
from exonware.xwsystem.config.version_manager import VersionManager

__version__ = "0.0.1.5"
_version_manager = VersionManager(__version__, "xwnode")

def get_version() -> str:
    return _version_manager.version_string
```

**3. CI/CD Integration**

Local shims allow CI tools to import VersionManager without circular dependencies.

### Benefits

- **No Code Duplication** - Single VersionManager implementation
- **CI/CD Compatible** - Local shims for tool imports
- **Consistent Versioning** - Same format across all projects
- **Maintainable** - Update logic in one place

### Version Manager Usage

```python
from exonware.xwsystem.config.version_manager import VersionManager

vm = VersionManager("0.0.1.5", "project")

# Version bumping
vm.bump_major()    # ? "1.0.0"
vm.bump_minor()    # ? "0.1.0"
vm.bump_patch()    # ? "0.0.2"
vm.bump_build()    # ? "0.0.1.6"

# Version checks
vm.is_dev_version()      # ? True (has build number)
vm.is_release_version()  # ? False
```

---

## ? Async/Sync Patterns

### The Problem

Python's asyncio has challenges when mixing synchronous and asynchronous code:
- Event loop conflicts (can't call `asyncio.run()` when loop is running)
- Nested event loops causing RuntimeError
- Event loops not properly closed

### The Solution: New Event Loop Pattern

**? Correct Pattern for Sync Wrappers:**

```python
def sync_wrapper(self, *args, **kwargs):
    """Synchronous wrapper for async method."""
    import asyncio
    
    # Create a NEW event loop
    new_loop = asyncio.new_event_loop()
    try:
        asyncio.set_event_loop(new_loop)
        return new_loop.run_until_complete(self.async_method(*args, **kwargs))
    finally:
        # Always clean up
        new_loop.close()
        asyncio.set_event_loop(None)
```

**? Incorrect Patterns (Don't Use):**

```python
# ? WRONG - Fails if loop is already running
def sync_wrapper(self):
    return asyncio.run(self.async_method())

# ? WRONG - Can cause conflicts
def sync_wrapper(self):
    loop = asyncio.get_event_loop()
    return loop.run_until_complete(self.async_method())
```

### When to Use Async vs Sync

**Use Async When:**
- Performing I/O operations (file, network, database)
- Operations are naturally asynchronous
- Need to run multiple operations concurrently

**Use Sync When:**
- Simple, immediate operations
- Utility functions
- Pure data transformations

**Provide Both When:**
- Library API (support both sync and async users)
- File operations (common use case)
- Data loading/saving

### Pattern Template

```python
def sync_wrapper_method(self, *args, **kwargs):
    """
    Synchronous wrapper for async_method.
    
    Creates a new event loop to avoid conflicts.
    Properly cleans up resources after execution.
    """
    import asyncio
    
    new_loop = asyncio.new_event_loop()
    try:
        asyncio.set_event_loop(new_loop)
        return new_loop.run_until_complete(
            self.async_method(*args, **kwargs)
        )
    finally:
        new_loop.close()
        asyncio.set_event_loop(None)
```

### Key Principles

**? Do:**
1. Create new event loops for sync wrappers
2. Always use try/finally for cleanup
3. Close loops with `loop.close()` and `set_event_loop(None)`
4. Use sync I/O in sync contexts when possible
5. Test event loop cleanup in integration tests

**? Don't:**
1. Don't use `asyncio.get_event_loop()` in sync wrappers
2. Don't check `is_running()` and branch
3. Don't forget to cleanup event loops
4. Don't use `async_safe_*` functions unnecessarily in sync wrappers

---

## Additional architecture docs (archived)

Large-file support (1KB–10GB+), advanced serialization roadmap, and cache configuration were documented in former standalone docs; their value was absorbed. Design decisions are reflected in this reference and [REF_15_API.md](REF_15_API.md).

---

## 📚 Related Documentation

- [guides/GUIDE_ARCH.md](guides/GUIDE_ARCH.md) - Architecture playbook (cross-project standards)
- [guides/GUIDE_COMP.md](guides/GUIDE_COMP.md) - Compliance program and Mars Standard roadmap
- [guides/GUIDE_DEV.md](guides/GUIDE_DEV.md) - Development standards
- [REF_15_API.md](REF_15_API.md) - API reference
- [REF_54_BENCH.md](REF_54_BENCH.md) - Performance SLAs
- [REF_22_PROJECT.md#project-status-overview](REF_22_PROJECT.md#project-status-overview) - Project status

**Architectural Changes:**
- [IO Redesign](logs/changes/CHANGE_20251030_2324_IO_REDESIGN.md)
- [IO Final State](logs/changes/CHANGE_20251030_2308_IO_FINAL_STATE.md)
- [Codec Implementation](logs/changes/CHANGE_20251030_1911_CODEC_IMPLEMENTATION.md)
- [Infrastructure Build](logs/changes/CHANGE_20251030_2048_INFRASTRUCTURE.md)
- [Version Management](logs/changes/CHANGE_20250914_0223_VERSION_MANAGEMENT.md)

---

*Last updated: 06-Nov-2025*


