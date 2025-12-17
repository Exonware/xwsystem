# 📚 xSystem - Complete Documentation

**Company:** eXonware.com  
**Author:** Eng. Muhammad AlShehri  
**Email:** connect@exonware.com  
**Version:** 0.0.1  
**Last Updated:** September 2, 2025

---

## 📑 **Table of Contents**

1. [Overview](#overview)
2. [Project Phases](#project-phases)
3. [Installation](#installation)
4. [Serialization (24 Formats)](#serialization-24-formats)
5. [Security & Cryptography](#security--cryptography)
6. [Threading Utilities](#threading-utilities)
7. [I/O Operations](#io-operations)
8. [HTTP Client](#http-client)
9. [Data Structures](#data-structures)
10. [Design Patterns](#design-patterns)
11. [Performance Monitoring](#performance-monitoring)
12. [Runtime Utilities](#runtime-utilities)
13. [Plugin System](#plugin-system)
14. [Configuration](#configuration)
15. [Error Handling](#error-handling)
16. [Best Practices](#best-practices)

---

## 📊 **Overview**

xSystem is the **all-in-one Python library** that replaces 50+ dependencies with a single, production-grade package. It provides:

- **24 serialization formats** with consistent APIs (including 7 enterprise schema-based formats)
- **Enterprise-grade security** utilities
- **Thread-safe operations** by default
- **Atomic I/O operations** for data integrity
- **Modern HTTP client** with smart retries
- **Performance monitoring** and optimization
- **Plugin system** for extensibility
- **AI-powered performance optimization** with adaptive learning

### **📦 Module Structure**

```
src/exonware/xwsystem/
+-- __init__.py                    # Main module exports (277 lines)
+-- serialization/                # 24 serialization formats
�   +-- json.py                   # JSON serialization
�   +-- yaml.py                   # YAML serialization  
�   +-- toml.py                   # TOML serialization
�   +-- xml.py                    # XML serialization (secure)
�   +-- bson.py                   # BSON/MongoDB serialization
�   +-- msgpack.py                # MessagePack binary
�   +-- cbor.py                   # CBOR binary
�   +-- csv.py                    # CSV tabular data
�   +-- pickle.py                 # Python object serialization
�   +-- marshal.py                # Python internal serialization
�   +-- configparser.py           # INI-style configs
�   +-- formdata.py               # URL-encoded forms
�   +-- multipart.py              # HTTP multipart uploads
�   +-- sqlite3.py                # SQLite database
�   +-- dbm.py                    # Key-value database
�   +-- shelve.py                 # Persistent dictionaries
�   +-- plistlib.py               # Apple property lists
�   +-- avro.py                   # 📊 Apache Avro (schema evolution)
�   +-- protobuf.py               # 📦 Protocol Buffers (Google)
�   +-- thrift.py                 # 🔄 Apache Thrift (cross-language RPC)
�   +-- parquet.py                # 📊 Apache Parquet (columnar analytics)
�   +-- orc.py                    # 📊 Apache ORC (optimized row columnar)
�   +-- capnproto.py              # ⚡ Cap'n Proto (infinite speed - optional)
�   +-- flatbuffers.py            # 🚀 FlatBuffers (zero-copy access)
+-- security/                     # Security suite
�   +-- crypto.py                 # Encryption & hashing
�   +-- path_validator.py         # Path security
�   +-- resource_limits.py        # Resource protection
+-- threading/                    # Thread-safe utilities
�   +-- safe_factory.py           # Thread-safe factories
�   +-- locks.py                  # Enhanced locking
+-- io/                          # I/O operations
�   +-- atomic_file.py            # Atomic file operations
�   +-- path_manager.py           # Path management
+-- http/                        # HTTP client
�   +-- client.py                 # Modern HTTP with retries
+-- structures/                   # Data structures
�   +-- circular_detector.py      # Circular reference detection
�   +-- tree_walker.py            # Tree traversal utilities
+-- patterns/                     # Design patterns
�   +-- handler_factory.py        # Generic factories
�   +-- context_manager.py        # Context utilities
�   +-- object_pool.py            # Object pooling
+-- monitoring/                   # Performance monitoring
�   +-- performance_monitor.py    # Performance tracking
�   +-- memory_monitor.py         # Memory monitoring
�   +-- metrics.py                # Metrics collection
+-- runtime/                      # Runtime utilities
�   +-- env.py                    # Environment detection
�   +-- reflection.py             # Dynamic introspection
+-- plugins/                      # Plugin system
�   +-- base.py                   # Plugin management
+-- config/                       # Configuration
�   +-- defaults.py               # Default settings
�   +-- performance.py            # Performance config
�   +-- logging_setup.py          # Logging setup
+-- validation/                   # Data validation
    +-- data_validator.py         # Input validation
    +-- type_safety.py            # Type checking
```

---

## 🚀 **Project Phases**

xSystem follows a structured 5-phase development approach designed to deliver enterprise-grade functionality while maintaining rapid iteration and continuous improvement.

### **Current Phase: 🧪 Version 0 - Experimental Stage**
- **Focus:** Fast applications & usage, refactoring to perfection of software patterns and design
- **Status:** ✅ **ACTIVE** - Foundation complete with 24 serialization formats, enterprise-grade security, and comprehensive testing

### **Development Roadmap:**
- **Version 1 (Q1 2026):** Production Ready - Enterprise deployment and hardening
- **Version 2 (Q2 2026):** Mars Standard Draft Implementation - Cross-platform interoperability
- **Version 3 (Q3 2026):** RUST Core & Facades - High-performance multi-language support
- **Version 4 (Q4 2026):** Mars Standard Implementation - Full compliance and enterprise deployment

?? **[View Complete Project Phases Documentation](logs/changes/CHANGE_20251030_2221_PROJECT_PHASES.md)**

---

## 📦 **Installation**

### **Quick Start**
```bash
# Get everything (recommended)
pip install exonware-xwsystem[all]

# Or minimal install
pip install exonware-xwsystem
```

### **Feature-Specific Installation**
```bash
# Serialization formats
pip install exonware-xwsystem[yaml]      # YAML support
pip install exonware-xwsystem[toml]      # TOML support  
pip install exonware-xwsystem[xml]       # Secure XML
pip install exonware-xwsystem[bson]      # MongoDB BSON
pip install exonware-xwsystem[msgpack]   # MessagePack binary
pip install exonware-xwsystem[cbor]      # CBOR binary

# Additional features
pip install exonware-xwsystem[http]      # HTTP client
pip install exonware-xwsystem[crypto]    # Cryptography
```

---

## 📄 **Serialization (24 Formats)**

The crown jewel of xSystem - **24 serialization formats with consistent APIs**.

### **🎯 Core Principle: Production-Grade Libraries Only**

xSystem uses **established, well-tested libraries** for all serialization:
- **JSON**: Built-in `json` module
- **YAML**: `PyYAML` library
- **TOML**: Built-in `tomllib` + `tomli-w`
- **XML**: `dicttoxml` + `xmltodict`
- **BSON**: `pymongo.bson`
- **MessagePack**: `msgpack`
- **CBOR**: `cbor2`
- **📊 Apache Avro**: `fastavro` library
- **📦 Protocol Buffers**: `protobuf` library
- **🔄 Apache Thrift**: `thrift` library
- **📊 Apache Parquet**: `pyarrow` library
- **📊 Apache ORC**: `pyorc` library
- **⚡ Cap'n Proto**: `pycapnp` library (optional)
- **🚀 FlatBuffers**: `flatbuffers` library
- And 10 more formats...

### **📝 Text Formats (8 formats)**

```python
from exonware.xwsystem import (
    JsonSerializer, YamlSerializer, TomlSerializer, XmlSerializer,
    CsvSerializer, ConfigParserSerializer, FormDataSerializer, MultipartSerializer
)

data = {"users": 1000, "active": True, "tags": ["fast", "reliable"]}

# JSON - Universal standard
js = JsonSerializer()
json_str = js.dumps(data)  # {"users":1000,"active":true,"tags":["fast","reliable"]}

# YAML - Human-readable configs
ys = YamlSerializer()
yaml_str = ys.dumps(data)  # users: 1000\nactive: true\ntags:\n- fast\n- reliable

# TOML - Python package configs
ts = TomlSerializer()
toml_str = ts.dumps(data)  # users = 1000\nactive = true\ntags = ["fast", "reliable"]

# XML - Structured documents
xs = XmlSerializer()
xml_str = xs.dumps(data)   # <root><users>1000</users><active>true</active>...

# CSV - Tabular data
cs = CsvSerializer()
csv_str = cs.dumps([{"name": "John", "age": 30}, {"name": "Jane", "age": 25}])

# ConfigParser - INI files
cps = ConfigParserSerializer()
ini_str = cps.dumps({"section1": {"key1": "value1", "key2": "value2"}})

# FormData - URL-encoded forms
fds = FormDataSerializer()
form_str = fds.dumps({"username": "john", "password": "secret"})

# Multipart - File uploads
mps = MultipartSerializer()
multipart_data = mps.dumps({"file": open("data.txt", "rb"), "metadata": "info"})
```

### **💾 Binary Formats (9 formats)**

```python
from exonware.xwsystem import (
    BsonSerializer, MsgPackSerializer, CborSerializer,
    PickleSerializer, MarshalSerializer, Sqlite3Serializer,
    DbmSerializer, ShelveSerializer, PlistlibSerializer
)

data = {"users": 1000, "active": True, "performance": 95.7}

# BSON - MongoDB compatibility
bs = BsonSerializer()
bson_bytes = bs.dumps(data)  # Binary format for MongoDB

# MessagePack - Compact binary (47% smaller than JSON!)
mss = MsgPackSerializer()
msgpack_bytes = mss.dumps(data)  # Ultra-compact binary

# CBOR - RFC 8949 standard
cbrs = CborSerializer()
cbor_bytes = cbrs.dumps(data)  # Standards-compliant binary

# Pickle - Python objects (any type)
ps = PickleSerializer()
pickle_bytes = ps.dumps(data)  # Serialize any Python object

# Marshal - Python internal (fastest)
ms = MarshalSerializer()
marshal_bytes = ms.dumps(data)  # Fastest Python serialization

# SQLite3 - Embedded database
s3s = Sqlite3Serializer()
s3s.dumps_to_db("data.db", "table_name", [data])  # Store in SQLite

# DBM - Key-value database
ds = DbmSerializer()
ds.dumps_to_dbm("data.dbm", {"key1": data})  # Persistent key-value

# Shelve - Persistent dictionary
ss = ShelveSerializer()
ss.dumps_to_shelf("data.shelf", {"data_key": data})  # Dict-like storage

# Plistlib - Apple property lists
pls = PlistlibSerializer()
plist_bytes = pls.dumps(data)  # Apple plist format
```

### **🏢 Schema-Based Enterprise Formats (7 formats)**

```python
from exonware.xwsystem import (
    AvroSerializer, ProtobufSerializer, ThriftSerializer,
    ParquetSerializer, OrcSerializer, CapnProtoSerializer, FlatBuffersSerializer
)

data = {"user_id": 12345, "name": "John Doe", "email": "john@example.com", "score": 95.7}

# Apache Avro - Schema evolution support
avs = AvroSerializer()
avro_bytes = avs.dumps(data)  # Compact binary with schema evolution
loaded_data = avs.loads(avro_bytes)

# Protocol Buffers - Google's language-neutral format
pbs = ProtobufSerializer()
protobuf_bytes = pbs.dumps(data)  # Efficient cross-language serialization

# Apache Thrift - Cross-language RPC framework
trs = ThriftSerializer()
thrift_bytes = trs.dumps(data)  # RPC-ready binary format

# Apache Parquet - Columnar storage for analytics
pqs = ParquetSerializer()
parquet_data = pqs.dumps([data, data, data])  # Optimized for analytics queries

# Apache ORC - Optimized Row Columnar format
ors = OrcSerializer()
orc_data = ors.dumps([data])  # High-performance columnar storage

# Cap'n Proto - Infinitely fast data interchange (optional)
try:
    cps = CapnProtoSerializer()
    capnp_bytes = cps.dumps(data)  # Zero-copy deserialization
except ImportError:
    print("Cap'n Proto requires pycapnp - install with: pip install pycapnp")

# FlatBuffers - Zero-copy serialization for games/performance
fbs = FlatBuffersSerializer()
flatbuf_bytes = fbs.dumps(data)  # Memory-efficient with zero-copy access
```

### **💼 Enterprise Format Use Cases**

```python
# Use Case 1: Data Pipeline with Schema Evolution
avro_serializer = AvroSerializer()
# Schema v1: {"name": "string", "age": "int"}
user_v1 = {"name": "John", "age": 30}
serialized = avro_serializer.dumps(user_v1)

# Later: Schema v2 adds "email" field - backward compatible!
# {"name": "string", "age": "int", "email": "string"}
user_v2 = avro_serializer.loads(serialized)  # Works seamlessly

# Use Case 2: High-Performance Analytics
parquet_serializer = ParquetSerializer()
analytics_data = [
    {"timestamp": "2025-01-01T00:00:00", "user_id": 1, "revenue": 100.0},
    {"timestamp": "2025-01-01T01:00:00", "user_id": 2, "revenue": 150.0},
    # ... millions of records
]
parquet_file = parquet_serializer.dumps(analytics_data)  # Columnar compression

# Use Case 3: Cross-Language Microservices
protobuf_serializer = ProtobufSerializer()
api_request = {"service": "user", "method": "get_profile", "user_id": 123}
protobuf_bytes = protobuf_serializer.dumps(api_request)
# Send to Java/C++/Go services - they can decode natively

# Use Case 4: Real-Time Gaming (FlatBuffers)
flatbuf_serializer = FlatBuffersSerializer()
game_state = {
    "player_id": 456,
    "position": {"x": 10.5, "y": 20.3, "z": 5.1},
    "health": 100,
    "inventory": ["sword", "shield", "potion"]
}
game_bytes = flatbuf_serializer.dumps(game_state)  # Zero-copy access for speed
```

### **🔄 Consistent API Across All Formats**

Every serializer follows the same interface:

```python
# All serializers support these methods
serializer = AnySerializer()

# Serialize to string/bytes
output = serializer.dumps(data)

# Deserialize from string/bytes  
data = serializer.loads(output)

# File operations
serializer.dump_to_file(data, "output.ext")
data = serializer.load_from_file("input.ext")

# Validation (built-in)
is_valid = serializer.validate(data)

# Error handling (consistent exceptions)
try:
    result = serializer.dumps(invalid_data)
except SerializationError as e:
    print(f"Serialization failed: {e}")
```

### **⚡ Performance Comparison**

```python
import time
from exonware.xwsystem import JsonSerializer, MsgPackSerializer, CborSerializer

data = {"users": list(range(1000)), "metadata": {"version": "1.0", "active": True}}

# JSON (text format)
js = JsonSerializer()
start = time.time()
json_result = js.dumps(data)
json_time = time.time() - start
json_size = len(json_result)

# MessagePack (binary format)
mss = MsgPackSerializer()
start = time.time()
msgpack_result = mss.dumps(data)
msgpack_time = time.time() - start
msgpack_size = len(msgpack_result)

print(f"JSON: {json_size} bytes, {json_time:.4f}s")
print(f"MessagePack: {msgpack_size} bytes, {msgpack_time:.4f}s")
print(f"Size reduction: {(1 - msgpack_size/json_size)*100:.1f}%")
# Output: Size reduction: 47.3%
```

---

## 🔒 **Security & Cryptography**

Enterprise-grade security utilities for production applications.

### **🔒 Path Security**

```python
from exonware.xwsystem import PathValidator, PathSecurityError

# Create validator with security policies
validator = PathValidator(
    base_path="/app/data",           # Restrict to base directory
    allow_absolute=False,            # Block absolute paths
    max_path_length=1024,           # Limit path length
    allow_symlinks=False,           # Block symbolic links
    reserved_names=["CON", "PRN"]   # Block Windows reserved names
)

try:
    # Validate suspicious paths
    safe_path = validator.validate_path("../../etc/passwd")  # Blocked!
except PathSecurityError as e:
    print(f"Security violation: {e}")

# Safe path validation
safe_path = validator.validate_path("config/settings.json")  # ✅ Allowed
validated_path = validator.resolve_path(safe_path)           # Get absolute path
```

### **🔐 Cryptography**

```python
from exonware.xwsystem import (
    SymmetricEncryption, AsymmetricEncryption, SecureHash,
    SecureRandom, generate_api_key, hash_password, verify_password
)

# Symmetric encryption (AES)
key = SecureRandom.generate_key(32)  # 256-bit key
encrypted = SymmetricEncryption.encrypt("sensitive data", key)
decrypted = SymmetricEncryption.decrypt(encrypted, key)

# Asymmetric encryption (RSA)
public_key, private_key = AsymmetricEncryption.generate_keypair(2048)
encrypted = AsymmetricEncryption.encrypt("secret message", public_key)
decrypted = AsymmetricEncryption.decrypt(encrypted, private_key)

# Secure hashing
password_hash = hash_password("user_password")  # bcrypt with salt
is_valid = verify_password("user_password", password_hash)

# API keys and tokens
api_key = generate_api_key(32)      # Cryptographically secure
session_token = SecureRandom.generate_token(64)

# File integrity
file_hash = SecureHash.file_sha256("important_file.txt")
content_hash = SecureHash.sha256("content to hash")
```

### **✅ Input Validation**

```python
from exonware.xwsystem import DataValidator, ValidationError

validator = DataValidator()

# Validate data types and constraints
try:
    # Email validation
    email = validator.validate_email("user@example.com")
    
    # URL validation
    url = validator.validate_url("https://secure-site.com/api")
    
    # Custom validation
    age = validator.validate_range(25, min_val=0, max_val=150)
    
    # String sanitization
    safe_string = validator.sanitize_string("<script>alert('xss')</script>")
    
except ValidationError as e:
    print(f"Validation failed: {e}")
```

---

## 🔄 **Threading Utilities**

Thread-safe utilities for concurrent applications.

### **🔒 Thread-Safe Factory**

```python
from exonware.xwsystem import ThreadSafeFactory

# Create thread-safe handler factory
factory = ThreadSafeFactory()

# Register handlers (thread-safe)
factory.register("json", JsonHandler, extensions=["json"])
factory.register("yaml", YamlHandler, extensions=["yaml", "yml"])

# Retrieve handlers (thread-safe)
handler = factory.get_handler("json")
all_formats = factory.get_available_formats()

# Use in multithreaded environment
import threading

def worker(format_name):
    handler = factory.get_handler(format_name)  # Thread-safe
    result = handler.process(data)
    return result

threads = [
    threading.Thread(target=worker, args=("json",)),
    threading.Thread(target=worker, args=("yaml",)),
]
```

### **🔐 Enhanced Locks**

```python
from exonware.xwsystem import EnhancedRLock

# Create lock with timeout and statistics
lock = EnhancedRLock(timeout=5.0, name="DataProcessingLock")

# Use with automatic timeout
try:
    with lock.timeout_context(timeout=2.0):
        # Critical section - automatically released
        process_shared_data()
except TimeoutError:
    print("Could not acquire lock within timeout")

# Get lock statistics
stats = lock.get_stats()
print(f"Lock acquired {stats['acquisition_count']} times")
print(f"Average hold time: {stats['average_hold_time']:.4f}s")
```

### **⚙️ Method Generation**

```python
from exonware.xwsystem import MethodGenerator

# Dynamically generate methods on classes
def export_template(self, format_name: str, **kwargs):
    handler = self.factory.get_handler(format_name)
    return handler.export(self.data, **kwargs)

# Generate export methods for all registered formats
MethodGenerator.generate_export_methods(
    target_class=DataProcessor,
    factory=my_factory,
    method_template=export_template,
    method_name_pattern="export_to_{format}",
    method_doc_pattern="Export data to {format} format with validation."
)

# Now DataProcessor has: export_to_json(), export_to_yaml(), etc.
processor = DataProcessor()
json_data = processor.export_to_json(indent=2)
yaml_data = processor.export_to_yaml(default_flow_style=False)
```

---

## 💾 **I/O Operations**

Atomic and safe file operations for data integrity.

### **🔒 Atomic File Operations**

```python
from exonware.xwsystem import AtomicFileWriter, safe_write_text, safe_read_text

# Method 1: Context manager class
with AtomicFileWriter(
    target_path="critical_data.json",
    mode='w',
    encoding='utf-8',
    backup=True,           # Create backup of existing file
    permissions=0o644      # Set file permissions
) as writer:
    writer.write(json.dumps(important_data, indent=2))
    # File is atomically written on successful exit

# Method 2: Convenience functions
safe_write_text("config.yaml", yaml_content)
safe_write_bytes("binary_data.bin", binary_content)

# Method 3: With error handling
try:
    content = safe_read_text("settings.json")
    data = json.loads(content)
except FileOperationError as e:
    print(f"File operation failed: {e}")
    # Original file is preserved on failure
```

### **📁 Path Management**

```python
from exonware.xwsystem import PathManager

# Advanced path operations
pm = PathManager(base_path="/app/data")

# Safe path operations
safe_path = pm.join_path("config", "database.json")
temp_file = pm.create_temp_file(suffix=".json")
backup_path = pm.create_backup_path("important.txt")

# Directory operations
pm.ensure_directory("logs/2025/09")
pm.clean_empty_directories("temp")

# File operations with validation
if pm.is_safe_to_write("output.txt"):
    pm.atomic_write("output.txt", content)
```

---

## 🌐 **HTTP Client**

Modern HTTP client with smart retries and configuration.

### **🌐 HTTP Operations**

```python
from exonware.xwsystem import HttpClient, RetryConfig

# Create client with retry configuration
retry_config = RetryConfig(
    max_retries=3,
    backoff_factor=0.5,
    retry_on_status=[500, 502, 503, 504],
    timeout=30
)

client = HttpClient(
    base_url="https://api.example.com",
    retry_config=retry_config,
    headers={"Authorization": "Bearer token"}
)

# Async operations
async def fetch_data():
    # GET with automatic retries
    response = await client.get("/users", params={"page": 1})
    users = response.json()
    
    # POST with data
    new_user = {"name": "John", "email": "john@example.com"}
    response = await client.post("/users", json=new_user)
    
    # File upload
    with open("profile.jpg", "rb") as f:
        response = await client.post("/upload", files={"image": f})
    
    return users

# Synchronous operations
response = client.sync_get("/health")
status = response.json()
```

### **⚡ Advanced Features**

```python
# Session management
with client.session() as session:
    # Login
    login_response = await session.post("/login", json=credentials)
    token = login_response.json()["token"]
    
    # Authenticated requests (token automatically included)
    session.headers.update({"Authorization": f"Bearer {token}"})
    user_data = await session.get("/profile")

# Request/response middleware
@client.middleware
async def log_requests(request, call_next):
    start_time = time.time()
    response = await call_next(request)
    duration = time.time() - start_time
    print(f"{request.method} {request.url} - {response.status_code} ({duration:.2f}s)")
    return response
```

---

## 📊 **Data Structures**

Advanced data structure utilities for complex data handling.

### **🔄 Circular Reference Detection**

```python
from exonware.xwsystem import CircularReferenceDetector, CircularReferenceError

detector = CircularReferenceDetector(max_depth=100)

# Create circular reference
data = {"level1": {"level2": {"level3": None}}}
data["level1"]["level2"]["level3"] = data  # Circular!

# Detection methods
if detector.is_circular(data):
    print("Circular reference detected!")

# Get detailed path information
try:
    detector.traverse(data, [])
except CircularReferenceError as e:
    print(f"Circular path: {e.path}")
    print(f"At depth: {e.depth}")

# Safe traversal with circular handling
safe_data = detector.resolve_circular_refs(data, placeholder="<CIRCULAR>")
```

### **🌳 Tree Walking**

```python
from exonware.xwsystem import TreeWalker

walker = TreeWalker()

# Walk complex nested structures
nested_data = {
    "users": [
        {"name": "John", "settings": {"theme": "dark", "lang": "en"}},
        {"name": "Jane", "settings": {"theme": "light", "lang": "es"}}
    ],
    "config": {"version": "1.0", "features": ["auth", "api"]}
}

# Find all values matching criteria
themes = walker.find_values(nested_data, key="theme")
# Result: ["dark", "light"]

# Transform values
def uppercase_names(path, key, value):
    if key == "name":
        return value.upper()
    return value

transformed = walker.transform(nested_data, uppercase_names)

# Flatten nested structure
flat_data = walker.flatten(nested_data, separator=".")
# Result: {"users.0.name": "JOHN", "users.0.settings.theme": "dark", ...}
```

---

## 🎨 **Design Patterns**

Reusable design patterns for better code organization.

### **🏭 Generic Handler Factory**

```python
from exonware.xwsystem import GenericHandlerFactory

# Create enhanced factory with all xwsystem features
factory = GenericHandlerFactory(
    base_path="/safe/directory",
    enable_security=True,
    enable_circular_detection=True,
    max_circular_depth=100
)

# Safe registration with validation
class JsonHandler:
    def process(self, data):
        return json.dumps(data)

try:
    factory.register_safe(
        "json", 
        JsonHandler, 
        extensions=["json"],
        validate_class=True  # Check for circular references in class
    )
except CircularReferenceError:
    print("Handler class has circular references!")

# Thread-safe retrieval
handler = factory.get_handler("json")
```

### **📦 Context Managers**

```python
from exonware.xwsystem import (
    ContextualLogger, ThreadSafeSingleton, 
    combine_contexts, enhanced_error_context
)

# Contextual logging
with ContextualLogger("data_processing") as logger:
    logger.info("Starting data processing")
    process_data()
    logger.info("Data processing completed")

# Combine multiple contexts
with combine_contexts([
    enhanced_error_context("Critical operation"),
    ContextualLogger("operation"),
    lock.timeout_context(5.0)
]):
    # All contexts are active
    critical_operation()

# Thread-safe singleton
@ThreadSafeSingleton
class DatabaseConnection:
    def __init__(self):
        self.connection = create_connection()

# Always returns the same instance (thread-safe)
db1 = DatabaseConnection()
db2 = DatabaseConnection()
assert db1 is db2
```

### **🏊 Object Pool**

```python
from exonware.xwsystem import ObjectPool

# Create pool for expensive objects
class DatabaseConnection:
    def __init__(self):
        self.connection = create_expensive_connection()
    
    def reset(self):
        self.connection.reset()

pool = ObjectPool(
    factory=DatabaseConnection,
    max_size=10,
    reset_method="reset"
)

# Use pooled objects
with pool.get_object() as conn:
    result = conn.connection.execute("SELECT * FROM users")
    # Object automatically returned to pool
```

---

## ⚡ **Performance Monitoring**

Built-in performance monitoring and optimization.

### **📊 Performance Tracking**

```python
from exonware.xwsystem import PerformanceMonitor, MemoryMonitor

# Monitor performance
monitor = PerformanceMonitor()

@monitor.track_performance
def expensive_operation(data):
    # Process data
    return result

# Get performance statistics
stats = monitor.get_stats("expensive_operation")
print(f"Average execution time: {stats['avg_time']:.4f}s")
print(f"Total calls: {stats['call_count']}")
print(f"Memory usage: {stats['avg_memory_mb']:.2f} MB")

# Memory monitoring
memory_monitor = MemoryMonitor()
memory_monitor.start_monitoring()

# Your application code here

memory_stats = memory_monitor.get_memory_stats()
print(f"Peak memory usage: {memory_stats['peak_mb']:.2f} MB")
```

### **✅ Performance Validation**

```python
from exonware.xwsystem import PerformanceValidator

validator = PerformanceValidator(
    max_execution_time=1.0,    # 1 second max
    max_memory_mb=100,         # 100 MB max
    max_cpu_percent=80         # 80% CPU max
)

@validator.validate_performance
def monitored_function(data):
    # Function execution is monitored
    return process_data(data)

# Performance violations are logged and can trigger alerts
```

---

## ⚙️ **Runtime Utilities**

Environment detection and runtime introspection.

### **🌍 Environment Management**

```python
from exonware.xwsystem import EnvironmentManager

env = EnvironmentManager()

# Environment detection
print(f"Platform: {env.get_platform()}")      # linux, windows, darwin
print(f"Python version: {env.get_python_version()}")
print(f"Architecture: {env.get_architecture()}")  # x64, arm64, etc.

# Resource information
print(f"CPU cores: {env.get_cpu_count()}")
print(f"Available memory: {env.get_available_memory_mb()} MB")
print(f"Disk space: {env.get_disk_space_gb('/')} GB")

# Environment variables (with validation)
database_url = env.get_env_var("DATABASE_URL", required=True)
debug_mode = env.get_env_bool("DEBUG", default=False)
max_workers = env.get_env_int("MAX_WORKERS", default=4, min_val=1, max_val=16)
```

### **🔍 Reflection Utilities**

```python
from exonware.xwsystem import ReflectionUtils

reflection = ReflectionUtils()

# Class introspection
class_info = reflection.analyze_class(MyClass)
print(f"Methods: {class_info['methods']}")
print(f"Properties: {class_info['properties']}")
print(f"Inheritance: {class_info['inheritance']}")

# Dynamic method calling
result = reflection.call_method(obj, "method_name", arg1="value", arg2=42)

# Module discovery
modules = reflection.discover_modules("mypackage")
classes = reflection.get_classes_in_module("mypackage.submodule")
```

---

## 🔌 **Plugin System**

Dynamic plugin discovery and management.

### **🔌 Plugin Management**

```python
from exonware.xwsystem import PluginManager, PluginBase

# Create plugin manager
plugin_manager = PluginManager(
    plugin_directory="plugins",
    entry_point_group="myapp.plugins"
)

# Define plugin interface
class DataProcessorPlugin(PluginBase):
    @abstractmethod
    def process(self, data):
        pass

# Register plugin types
plugin_manager.register_plugin_type("data_processor", DataProcessorPlugin)

# Discover and load plugins
plugin_manager.discover_plugins()
loaded_plugins = plugin_manager.load_plugins()

# Use plugins
processor_plugins = plugin_manager.get_plugins_by_type("data_processor")
for plugin in processor_plugins:
    result = plugin.process(my_data)
```

### **🔧 Plugin Development**

```python
# Example plugin implementation
class JsonProcessorPlugin(DataProcessorPlugin):
    name = "json_processor"
    version = "1.0.0"
    description = "JSON data processing plugin"
    
    def process(self, data):
        return json.dumps(data, indent=2)
    
    def validate(self, data):
        try:
            json.dumps(data)
            return True
        except (TypeError, ValueError):
            return False

# Plugin registration (in plugin file)
def register_plugin():
    return JsonProcessorPlugin()
```

---

## ⚙️ **Configuration**

Flexible configuration system for all xwsystem components.

### **⚡ Performance Configuration**

```python
from exonware.xwsystem import (
    PerformanceConfig, configure_performance, 
    get_performance_config
)

# Configure performance settings
configure_performance(
    mode="production",           # development, testing, production
    max_memory_mb=512,
    max_file_size_mb=100,
    enable_monitoring=True,
    cache_size=1000
)

# Get current configuration
config = get_performance_config()
print(f"Current mode: {config.mode}")
print(f"Memory limit: {config.max_memory_mb} MB")

# Per-component configuration
from exonware.xwsystem import SerializationLimits, NetworkLimits

serialization_limits = SerializationLimits(
    max_object_depth=50,
    max_string_length=1000000,
    max_collection_size=10000
)

network_limits = NetworkLimits(
    max_request_size_mb=10,
    connection_timeout=30,
    read_timeout=60
)
```

### **📝 Logging Configuration**

```python
from exonware.xwsystem import setup_logging, get_logger

# Setup logging
setup_logging(
    level="INFO",
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    file_path="app.log",
    max_file_size_mb=10,
    backup_count=5
)

# Use logger
logger = get_logger(__name__)
logger.info("Application started")
logger.error("An error occurred", exc_info=True)

# Contextual logging
with ContextualLogger("operation_name") as ctx_logger:
    ctx_logger.info("Operation started")
    # Logs include operation context
```

---

## ⚠️ **Error Handling**

Comprehensive error handling and recovery mechanisms.

### **⚠️ Exception Hierarchy**

```python
from exonware.xwsystem import (
    SerializationError, PathSecurityError, FileOperationError,
    CircularReferenceError, CryptoError, ValidationError,
    HttpError, PerformanceError
)

try:
    # Operations that might fail
    data = JsonSerializer().loads(invalid_json)
    safe_path = PathValidator().validate_path("../../../etc/passwd")
    encrypted = SymmetricEncryption.encrypt(data, weak_key)
    
except SerializationError as e:
    logger.error(f"Serialization failed: {e}")
except PathSecurityError as e:
    logger.warning(f"Path security violation: {e}")
except CryptoError as e:
    logger.error(f"Cryptographic operation failed: {e}")
except ValidationError as e:
    logger.warning(f"Validation failed: {e}")
```

### **🔄 Error Recovery**

```python
from exonware.xwsystem import ErrorRecovery, CircuitBreaker

# Circuit breaker for external services
circuit_breaker = CircuitBreaker(
    failure_threshold=5,
    recovery_timeout=30,
    expected_exception=HttpError
)

@circuit_breaker
async def call_external_api():
    response = await http_client.get("/api/data")
    return response.json()

# Retry mechanism
recovery = ErrorRecovery(
    max_retries=3,
    backoff_factor=2.0,
    retry_exceptions=[HttpError, TimeoutError]
)

@recovery.retry
def flaky_operation():
    # Operation that might fail
    return process_data()
```

---

## ✨ **Best Practices**

### **🔒 Security Best Practices**

1. **Always validate paths** before file operations:
```python
validator = PathValidator(base_path="/safe/directory")
safe_path = validator.validate_path(user_input)
```

2. **Use atomic operations** for critical data:
```python
with AtomicFileWriter("critical.json") as writer:
    writer.write(json.dumps(important_data))
```

3. **Encrypt sensitive data** at rest and in transit:
```python
encrypted_data = SymmetricEncryption.encrypt(sensitive_data, key)
```

4. **Validate all input** before processing:
```python
validator = DataValidator()
clean_email = validator.validate_email(user_email)
```

### **⚡ Performance Best Practices**

1. **Use binary formats** for large data:
```python
# Instead of JSON for large datasets
msgpack_data = MsgPackSerializer().dumps(large_dataset)  # 47% smaller
```

2. **Monitor performance** in production:
```python
@PerformanceMonitor().track_performance
def critical_function():
    return expensive_operation()
```

3. **Use object pools** for expensive resources:
```python
with database_pool.get_object() as conn:
    result = conn.execute(query)
```

4. **Enable appropriate performance mode**:
```python
configure_performance(mode="production")  # Optimized for production
```

### **🔄 Threading Best Practices**

1. **Use thread-safe factories** for shared resources:
```python
factory = ThreadSafeFactory()
handler = factory.get_handler("json")  # Thread-safe
```

2. **Use enhanced locks** with timeouts:
```python
with EnhancedRLock(timeout=5.0).timeout_context():
    # Critical section with automatic timeout
    shared_resource.update()
```

3. **Avoid circular references** in threaded code:
```python
detector = CircularReferenceDetector()
if not detector.is_circular(data):
    process_in_thread(data)
```

### **💾 File I/O Best Practices**

1. **Always use atomic writes** for important files:
```python
safe_write_text("config.json", json.dumps(config))
```

2. **Create backups** before modifying critical files:
```python
with AtomicFileWriter("data.txt", backup=True) as writer:
    writer.write(new_content)
```

3. **Handle file operation errors** gracefully:
```python
try:
    content = safe_read_text("file.txt")
except FileOperationError as e:
    logger.error(f"Failed to read file: {e}")
    content = get_default_content()
```

### **📄 Serialization Best Practices**

1. **Choose the right format** for your use case:
   - **JSON**: Web APIs, configuration files
   - **YAML**: Human-readable configs
   - **MessagePack**: High-performance binary
   - **BSON**: MongoDB integration
   - **CSV**: Data analysis, Excel compatibility

2. **Validate data** before serialization:
```python
serializer = JsonSerializer()
if serializer.validate(data):
    result = serializer.dumps(data)
```

3. **Handle serialization errors** appropriately:
```python
try:
    result = serializer.dumps(complex_data)
except SerializationError as e:
    logger.error(f"Serialization failed: {e}")
    # Fallback to simpler format or error response
```

---

## 🤝 **Contributing**

### **Adding New Features**

1. **Create feature** in appropriate module
2. **Add comprehensive tests** with pytest
3. **Update documentation** with examples
4. **Add to `__init__.py`** exports
5. **Follow type hints** and docstring conventions

### **Code Style**

- **Type hints** for all public APIs
- **Comprehensive docstrings** with examples
- **PEP 8** style guidelines
- **Error handling** with specific exceptions
- **Performance considerations** documented

---

## 📄 **License**

MIT License - see LICENSE file for details.

---

**?? xSystem: Because life's too short for dependency hell.**

---

*Built with ?? by eXonware.com*

