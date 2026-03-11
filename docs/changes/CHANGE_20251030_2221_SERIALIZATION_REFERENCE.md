# ?? xSystem Serialization Guide - 24 Formats

**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Email:** connect@exonware.com  
**Version:** 0.0.1  
**Generation Date:** January 31, 2025

---

## ?? **Overview**

xSystem provides **24 serialization formats** with consistent APIs - more than any other Python library. All serializers follow the **production library principle**: we use established, well-tested libraries instead of custom implementations.

### **?? Format Categories**

- **?? Text Formats (8)**: Human-readable formats for configs and APIs
- **?? Binary Formats (9)**: High-performance binary formats for speed and size
- **?? ?? Schema-Based Enterprise Formats (7)**: Production-grade formats with schema evolution

---

## ?? **Text Formats (8 formats)**

### **1. JSON - Universal Standard**
```python
from exonware.xwsystem import JsonSerializer

js = JsonSerializer()
data = {"name": "John", "age": 30, "active": True}

# Serialize
json_str = js.dumps(data)  # '{"name":"John","age":30,"active":true}'

# Deserialize
parsed_data = js.loads(json_str)

# File operations
js.dump_to_file(data, "user.json")
loaded_data = js.load_from_file("user.json")
```

**Use Cases:** Web APIs, configuration files, data interchange  
**Library:** Built-in `json` module  
**Pros:** Universal support, human-readable  
**Cons:** Larger size than binary formats

### **2. YAML - Human-Readable Configs**
```python
from exonware.xwsystem import YamlSerializer

ys = YamlSerializer()
config = {
    "database": {
        "host": "localhost",
        "port": 5432,
        "credentials": {"user": "admin", "password": "secret"}
    },
    "features": ["auth", "api", "monitoring"]
}

yaml_str = ys.dumps(config)
# Output:
# database:
#   host: localhost
#   port: 5432
#   credentials:
#     user: admin
#     password: secret
# features:
# - auth
# - api
# - monitoring
```

**Use Cases:** Configuration files, Docker Compose, Kubernetes manifests  
**Library:** `PyYAML`  
**Pros:** Very human-readable, supports complex structures  
**Cons:** Slower parsing than JSON

### **3. TOML - Python Package Configs**
```python
from exonware.xwsystem import TomlSerializer

ts = TomlSerializer()
config = {
    "project": {"name": "myapp", "version": "1.0.0"},
    "dependencies": {"requests": ">=2.28.0", "pydantic": ">=1.10.0"}
}

toml_str = ts.dumps(config)
# Output:
# [project]
# name = "myapp"
# version = "1.0.0"
# 
# [dependencies]
# requests = ">=2.28.0"
# pydantic = ">=1.10.0"
```

**Use Cases:** `pyproject.toml`, configuration files  
**Library:** Built-in `tomllib` + `tomli-w`  
**Pros:** Clean syntax, good for configs  
**Cons:** Limited data types

### **4. XML - Structured Documents**
```python
from exonware.xwsystem import XmlSerializer

xs = XmlSerializer()
data = {
    "user": {
        "id": 123,
        "profile": {"name": "John", "email": "john@example.com"},
        "tags": ["admin", "active"]
    }
}

xml_str = xs.dumps(data)
# Output:
# <root>
#   <user>
#     <id>123</id>
#     <profile>
#       <name>John</name>
#       <email>john@example.com</email>
#     </profile>
#     <tags>admin</tags>
#     <tags>active</tags>
#   </user>
# </root>
```

**Use Cases:** SOAP APIs, configuration files, document storage  
**Library:** `dicttoxml` + `xmltodict` (secure parsing)  
**Pros:** Self-describing, widely supported  
**Cons:** Verbose, slower than JSON

### **5. CSV - Tabular Data**
```python
from exonware.xwsystem import CsvSerializer

cs = CsvSerializer()
tabular_data = [
    {"name": "John", "age": 30, "city": "New York"},
    {"name": "Jane", "age": 25, "city": "San Francisco"},
    {"name": "Bob", "age": 35, "city": "Chicago"}
]

csv_str = cs.dumps(tabular_data)
# Output:
# name,age,city
# John,30,New York
# Jane,25,San Francisco
# Bob,35,Chicago
```

**Use Cases:** Data analysis, Excel compatibility, reporting  
**Library:** Built-in `csv` module  
**Pros:** Excel compatibility, simple structure  
**Cons:** Limited data types, no nesting

### **6. ConfigParser - INI Files**
```python
from exonware.xwsystem import ConfigParserSerializer

cps = ConfigParserSerializer()
ini_data = {
    "database": {"host": "localhost", "port": "5432", "name": "mydb"},
    "logging": {"level": "INFO", "file": "app.log"}
}

ini_str = cps.dumps(ini_data)
# Output:
# [database]
# host = localhost
# port = 5432
# name = mydb
# 
# [logging]
# level = INFO
# file = app.log
```

**Use Cases:** System configuration, legacy applications  
**Library:** Built-in `configparser` module  
**Pros:** Simple, widely supported  
**Cons:** Limited nesting, string values only

### **7. FormData - URL-Encoded Forms**
```python
from exonware.xwsystem import FormDataSerializer

fds = FormDataSerializer()
form_data = {
    "username": "john_doe",
    "password": "secret123",
    "remember_me": "true",
    "tags": ["user", "active"]
}

form_str = fds.dumps(form_data)
# Output: username=john_doe&password=secret123&remember_me=true&tags=user&tags=active
```

**Use Cases:** Web forms, API requests  
**Library:** Built-in `urllib.parse`  
**Pros:** Web standard, simple  
**Cons:** Limited data types, no nesting

### **8. Multipart - File Uploads**
```python
from exonware.xwsystem import MultipartSerializer

mps = MultipartSerializer()
multipart_data = {
    "file": open("document.pdf", "rb"),
    "description": "Important document",
    "tags": ["legal", "confidential"]
}

multipart_bytes = mps.dumps(multipart_data)
# Creates multipart/form-data suitable for HTTP file uploads
```

**Use Cases:** File uploads, complex web forms  
**Library:** Built-in `email.mime` modules  
**Pros:** Supports files and metadata  
**Cons:** Complex format, HTTP-specific

---

## ?? **Binary Formats (9 formats)**

### **1. BSON - MongoDB Binary JSON**
```python
from exonware.xwsystem import BsonSerializer

bs = BsonSerializer()
data = {
    "user_id": 12345,
    "name": "John Doe",
    "created_at": datetime.now(),
    "metadata": {"version": 1, "active": True}
}

bson_bytes = bs.dumps(data)  # Binary format
loaded_data = bs.loads(bson_bytes)
```

**Use Cases:** MongoDB integration, binary JSON  
**Library:** `pymongo.bson`  
**Pros:** Preserves types (dates, ObjectIds), MongoDB native  
**Cons:** Larger than MessagePack for simple data

### **2. MessagePack - Compact Binary**
```python
from exonware.xwsystem import MsgPackSerializer

mss = MsgPackSerializer()
data = {"users": list(range(1000)), "active": True}

# Compare sizes
json_size = len(JsonSerializer().dumps(data))      # e.g., 8,890 bytes
msgpack_size = len(mss.dumps(data))                # e.g., 4,700 bytes
size_reduction = (1 - msgpack_size/json_size) * 100  # ~47% smaller!

print(f"JSON: {json_size} bytes")
print(f"MessagePack: {msgpack_size} bytes")
print(f"Size reduction: {size_reduction:.1f}%")
```

**Use Cases:** High-performance APIs, data compression  
**Library:** `msgpack`  
**Pros:** 47% smaller than JSON, fast parsing  
**Cons:** Not human-readable

### **3. CBOR - RFC 8949 Standard**
```python
from exonware.xwsystem import CborSerializer

cbrs = CborSerializer()
data = {
    "sensor_id": 42,
    "readings": [23.5, 24.1, 23.8, 24.0],
    "timestamp": datetime.now(),
    "metadata": {"location": "warehouse_a", "calibrated": True}
}

cbor_bytes = cbrs.dumps(data)
parsed_data = cbrs.loads(cbor_bytes)
```

**Use Cases:** IoT devices, RFC-compliant binary format  
**Library:** `cbor2`  
**Pros:** Standards-compliant, compact, preserves types  
**Cons:** Less common than MessagePack

### **4. Pickle - Python Objects**
```python
from exonware.xwsystem import PickleSerializer

ps = PickleSerializer()

# Can serialize ANY Python object
class CustomClass:
    def __init__(self, name, data):
        self.name = name
        self.data = data

custom_obj = CustomClass("test", [1, 2, 3])
lambda_func = lambda x: x * 2
complex_data = {
    "object": custom_obj,
    "function": lambda_func,
    "set": {1, 2, 3, 4},
    "tuple": (1, "two", 3.0)
}

pickle_bytes = ps.dumps(complex_data)
restored_data = ps.loads(pickle_bytes)
```

**Use Cases:** Python-specific serialization, caching, inter-process communication  
**Library:** Built-in `pickle` module  
**Pros:** Serializes any Python object  
**Cons:** Python-only, security risks with untrusted data

### **5. Marshal - Python Internal**
```python
from exonware.xwsystem import MarshalSerializer

ms = MarshalSerializer()
data = {
    "numbers": [1, 2, 3, 4, 5],
    "text": "hello world",
    "flag": True,
    "nested": {"a": 1, "b": 2}
}

marshal_bytes = ms.dumps(data)
loaded_data = ms.loads(marshal_bytes)
```

**Use Cases:** Python bytecode, fastest Python serialization  
**Library:** Built-in `marshal` module  
**Pros:** Fastest Python serialization  
**Cons:** Python version dependent, limited types

### **6. SQLite3 - Embedded Database**
```python
from exonware.xwsystem import Sqlite3Serializer

s3s = Sqlite3Serializer()
records = [
    {"id": 1, "name": "John", "age": 30},
    {"id": 2, "name": "Jane", "age": 25},
    {"id": 3, "name": "Bob", "age": 35}
]

# Store in SQLite database
s3s.dumps_to_db("users.db", "users", records)

# Query data
results = s3s.query_db("users.db", "SELECT * FROM users WHERE age > 25")
print(results)  # [{"id": 1, "name": "John", "age": 30}, {"id": 3, "name": "Bob", "age": 35}]
```

**Use Cases:** Embedded databases, complex queries, data persistence  
**Library:** Built-in `sqlite3` module  
**Pros:** SQL queries, ACID transactions, indexes  
**Cons:** Overhead for simple data

### **7. DBM - Key-Value Database**
```python
from exonware.xwsystem import DbmSerializer

ds = DbmSerializer()
kv_data = {
    "user:1": {"name": "John", "email": "john@example.com"},
    "user:2": {"name": "Jane", "email": "jane@example.com"},
    "config:db": {"host": "localhost", "port": 5432}
}

# Store key-value pairs
ds.dumps_to_dbm("data.dbm", kv_data)

# Retrieve specific keys
user_data = ds.get_from_dbm("data.dbm", "user:1")
all_data = ds.loads_from_dbm("data.dbm")
```

**Use Cases:** Simple key-value storage, caching  
**Library:** Built-in `dbm` module  
**Pros:** Fast key-value access, persistent  
**Cons:** No complex queries, platform-dependent format

### **8. Shelve - Persistent Dictionary**
```python
from exonware.xwsystem import ShelveSerializer

ss = ShelveSerializer()
shelf_data = {
    "session:abc123": {"user_id": 1, "login_time": datetime.now()},
    "cache:popular_items": ["item1", "item2", "item3"],
    "settings": {"theme": "dark", "language": "en"}
}

# Use like a persistent dictionary
ss.dumps_to_shelf("app_data.shelf", shelf_data)

# Access individual keys
session_data = ss.get_from_shelf("app_data.shelf", "session:abc123")
```

**Use Cases:** Application state, caching, session storage  
**Library:** Built-in `shelve` module  
**Pros:** Dictionary-like interface, persistent  
**Cons:** Single-process access, slower than dedicated databases

### **9. Plistlib - Apple Property Lists**
```python
from exonware.xwsystem import PlistlibSerializer

pls = PlistlibSerializer()
plist_data = {
    "CFBundleName": "MyApp",
    "CFBundleVersion": "1.0.0",
    "LSMinimumSystemVersion": "10.15",
    "NSAppTransportSecurity": {
        "NSAllowsArbitraryLoads": False,
        "NSExceptionDomains": {"example.com": {"NSIncludesSubdomains": True}}
    }
}

plist_bytes = pls.dumps(plist_data)  # Binary plist format
plist_xml = pls.dumps(plist_data, format='xml')  # XML plist format
```

**Use Cases:** macOS/iOS applications, Apple ecosystem integration  
**Library:** Built-in `plistlib` module  
**Pros:** Native Apple format, supports binary and XML  
**Cons:** Apple-specific

---

## ?? ?? **Schema-Based Enterprise Formats (7 formats)**

### **1. Apache Avro - Schema Evolution**
```python
from exonware.xwsystem import AvroSerializer

avs = AvroSerializer()

# Define schema v1
schema_v1 = {
    "type": "record",
    "name": "User",
    "fields": [
        {"name": "id", "type": "int"},
        {"name": "name", "type": "string"},
        {"name": "email", "type": "string"}
    ]
}

# Serialize with schema v1
user_v1 = {"id": 1, "name": "John", "email": "john@example.com"}
avro_bytes = avs.dumps(user_v1, schema=schema_v1)

# Later: Schema v2 adds optional field (backward compatible)
schema_v2 = {
    "type": "record",
    "name": "User",
    "fields": [
        {"name": "id", "type": "int"},
        {"name": "name", "type": "string"},
        {"name": "email", "type": "string"},
        {"name": "phone", "type": ["null", "string"], "default": None}  # Optional field
    ]
}

# Old data still works with new schema!
user_data = avs.loads(avro_bytes, schema=schema_v2)
print(user_data)  # {"id": 1, "name": "John", "email": "john@example.com", "phone": None}
```

**Use Cases:** Data pipelines, streaming systems, schema evolution  
**Library:** `fastavro`  
**Pros:** Schema evolution, cross-language, compact binary  
**Cons:** Requires schema management

### **2. Protocol Buffers - Google's Format**
```python
from exonware.xwsystem import ProtobufSerializer

pbs = ProtobufSerializer()

# Define .proto schema (simplified for example)
proto_schema = """
syntax = "proto3";

message User {
    int32 id = 1;
    string name = 2;
    string email = 3;
    repeated string tags = 4;
}
"""

user_data = {
    "id": 123,
    "name": "John Doe",
    "email": "john@example.com",
    "tags": ["admin", "active"]
}

# Serialize to protobuf binary
protobuf_bytes = pbs.dumps(user_data, schema=proto_schema)

# Deserialize
loaded_user = pbs.loads(protobuf_bytes, schema=proto_schema)

# Cross-language compatibility - this binary can be read by Java, C++, Go, etc.
```

**Use Cases:** Microservices, gRPC APIs, cross-language data exchange  
**Library:** `protobuf`  
**Pros:** Cross-language, compact, fast, backward compatible  
**Cons:** Requires .proto files, learning curve

### **3. Apache Thrift - Cross-Language RPC**
```python
from exonware.xwsystem import ThriftSerializer

trs = ThriftSerializer()

# Define Thrift IDL (simplified)
thrift_schema = """
struct UserService {
    1: required i32 id,
    2: required string name,
    3: optional string email,
    4: list<string> roles
}
"""

user_data = {
    "id": 456,
    "name": "Jane Smith",
    "email": "jane@example.com",
    "roles": ["user", "moderator"]
}

thrift_bytes = trs.dumps(user_data, schema=thrift_schema)
loaded_data = trs.loads(thrift_bytes, schema=thrift_schema)
```

**Use Cases:** Cross-language RPC, distributed systems, Facebook ecosystem  
**Library:** `thrift`  
**Pros:** RPC framework included, cross-language, mature  
**Cons:** More complex than simple serialization

### **4. Apache Parquet - Columnar Analytics**
```python
from exonware.xwsystem import ParquetSerializer

pqs = ParquetSerializer()

# Analytics data - perfect for Parquet's columnar format
analytics_data = [
    {"timestamp": "2025-01-01T00:00:00", "user_id": 1, "page": "/home", "duration": 45.2},
    {"timestamp": "2025-01-01T00:01:00", "user_id": 1, "page": "/products", "duration": 120.5},
    {"timestamp": "2025-01-01T00:02:00", "user_id": 2, "page": "/home", "duration": 30.1},
    # ... millions of records
]

# Serialize to Parquet (columnar format)
parquet_bytes = pqs.dumps(analytics_data)

# Efficient queries on specific columns
loaded_data = pqs.loads(parquet_bytes)

# Parquet is optimized for analytical queries:
# - Column-based compression
# - Predicate pushdown
# - Schema evolution
# - Integration with Spark, Pandas, etc.
```

**Use Cases:** Big data analytics, data warehousing, Apache Spark  
**Library:** `pyarrow`  
**Pros:** Excellent compression, fast analytical queries, Spark integration  
**Cons:** Optimized for batch processing, not single records

### **5. Apache ORC - Optimized Row Columnar**
```python
from exonware.xwsystem import OrcSerializer

ors = OrcSerializer()

# Similar to Parquet but with different optimizations
orc_data = [
    {"product_id": 1, "category": "electronics", "price": 299.99, "in_stock": True},
    {"product_id": 2, "category": "electronics", "price": 199.99, "in_stock": False},
    {"product_id": 3, "category": "books", "price": 19.99, "in_stock": True},
    # ... large dataset
]

orc_bytes = ors.dumps(orc_data)
loaded_data = ors.loads(orc_bytes)

# ORC features:
# - Built-in indexes
# - Column statistics
# - Predicate pushdown
# - ACID transactions (in Hive)
```

**Use Cases:** Apache Hive, data warehousing, Hadoop ecosystem  
**Library:** `pyorc`  
**Pros:** Excellent for Hadoop/Hive, built-in indexes, ACID support  
**Cons:** Less common than Parquet outside Hadoop

### **6. Cap'n Proto - Infinite Speed (Optional)**
```python
from exonware.xwsystem import CapnProtoSerializer

try:
    cps = CapnProtoSerializer()
    
    # Cap'n Proto schema (simplified)
    capnp_schema = """
    struct Person {
        id @0 :UInt32;
        name @1 :Text;
        email @2 :Text;
        friends @3 :List(Person);
    }
    """
    
    person_data = {
        "id": 789,
        "name": "Alice Johnson",
        "email": "alice@example.com",
        "friends": [
            {"id": 790, "name": "Bob", "email": "bob@example.com", "friends": []}
        ]
    }
    
    # Serialize with zero-copy deserialization capability
    capnp_bytes = cps.dumps(person_data, schema=capnp_schema)
    
    # Deserialization is "infinitely fast" - no parsing needed!
    loaded_person = cps.loads(capnp_bytes, schema=capnp_schema)
    
except ImportError:
    print("Cap'n Proto requires pycapnp which needs C++ build tools")
    print("Install with: pip install pycapnp")
    print("Note: This is optional - you have 23 other formats available!")
```

**Use Cases:** High-performance systems, real-time applications, gaming  
**Library:** `pycapnp` (requires C++ build tools)  
**Pros:** Zero-copy deserialization, extremely fast, cross-language  
**Cons:** Requires C++ compilation, complex setup

### **7. FlatBuffers - Zero-Copy Access**
```python
from exonware.xwsystem import FlatBuffersSerializer

fbs = FlatBuffersSerializer()

# FlatBuffers schema (simplified)
flatbuf_schema = """
table GameState {
    player_id: uint32;
    position: Vec3;
    health: float;
    inventory: [string];
}

table Vec3 {
    x: float;
    y: float;
    z: float;
}
"""

game_state = {
    "player_id": 12345,
    "position": {"x": 10.5, "y": 20.3, "z": 5.1},
    "health": 100.0,
    "inventory": ["sword", "shield", "health_potion", "magic_scroll"]
}

# Serialize for zero-copy access
flatbuf_bytes = fbs.dumps(game_state, schema=flatbuf_schema)

# Access data without deserialization - perfect for games!
game_data = fbs.loads(flatbuf_bytes, schema=flatbuf_schema)

# In real usage, you can access fields directly from the buffer
# without creating Python objects - extremely memory efficient
```

**Use Cases:** Game development, mobile apps, memory-constrained systems  
**Library:** `flatbuffers`  
**Pros:** Zero-copy access, memory efficient, fast  
**Cons:** More complex than simple formats, requires schema

---

## ?? **Consistent API Across All 24 Formats**

Every serializer follows the same interface:

```python
# All serializers support these methods
serializer = AnySerializer()  # Any of the 24 serializers

# Core serialization methods
output = serializer.dumps(data)           # Serialize to string/bytes
data = serializer.loads(output)           # Deserialize from string/bytes

# File operations
serializer.dump_to_file(data, "output.ext")     # Save to file
data = serializer.load_from_file("input.ext")   # Load from file

# Validation
is_valid = serializer.validate(data)            # Check if data is serializable

# Error handling (consistent exceptions)
try:
    result = serializer.dumps(invalid_data)
except SerializationError as e:
    print(f"Serialization failed: {e}")
    print(f"Error type: {e.error_type}")        # Format-specific error type
    print(f"Original error: {e.original_error}") # Underlying library error
```

---

## ?? **Format Selection Guide**

### **Choose Based on Use Case:**

| **Use Case** | **Recommended Formats** | **Why** |
|--------------|------------------------|---------|
| **Web APIs** | JSON, MessagePack | Universal support, compact binary option |
| **Configuration Files** | YAML, TOML | Human-readable, easy to edit |
| **Data Analytics** | Parquet, ORC | Columnar compression, fast queries |
| **Cross-Language Services** | Protocol Buffers, Avro, Thrift | Schema evolution, language neutral |
| **High-Performance Gaming** | FlatBuffers, Cap'n Proto | Zero-copy access, minimal latency |
| **Python-Specific** | Pickle, Marshal | Native Python object support |
| **Database Storage** | SQLite3, BSON | Query capabilities, MongoDB integration |
| **Caching** | MessagePack, DBM, Shelve | Fast serialization, persistent storage |
| **Mobile/IoT** | CBOR, MessagePack | Compact size, standards-compliant |
| **Legacy Systems** | XML, CSV, ConfigParser | Wide compatibility, simple formats |

### **Performance Comparison (Approximate)**

| **Format** | **Speed** | **Size** | **Human Readable** | **Schema Evolution** |
|------------|-----------|----------|-------------------|---------------------|
| JSON | Fast | Medium | ? | ? |
| MessagePack | Very Fast | Small | ? | ? |
| Protocol Buffers | Very Fast | Small | ? | ? |
| Avro | Fast | Small | ? | ? |
| Cap'n Proto | Extreme | Small | ? | ? |
| FlatBuffers | Extreme | Small | ? | ? |
| Parquet | Medium | Very Small | ? | ? |
| YAML | Slow | Large | ? | ? |
| XML | Slow | Large | ? | ? |

---

## ??? **Security Considerations**

### **Safe Formats (No Code Execution Risk)**
- JSON, YAML, TOML, XML, CSV, ConfigParser, FormData, Multipart
- MessagePack, CBOR, BSON
- Avro, Protocol Buffers, Thrift, Parquet, ORC, FlatBuffers

### **Potentially Unsafe Formats (Trusted Data Only)**
- **Pickle**: Can execute arbitrary code during deserialization
- **Marshal**: Can contain bytecode, version-dependent
- **Cap'n Proto**: Requires careful schema validation

### **Best Practices**
```python
# Always validate data from untrusted sources
if not serializer.validate(untrusted_data):
    raise ValidationError("Data validation failed")

# Use safe formats for external data
json_serializer = JsonSerializer()  # Safe choice
safe_data = json_serializer.loads(external_input)

# Reserve Pickle for trusted, internal data only
pickle_serializer = PickleSerializer()
internal_data = pickle_serializer.loads(trusted_internal_data)
```

---

## ?? **Performance Tips**

### **1. Choose the Right Format**
```python
# For large datasets - use binary formats
large_data = [{"id": i, "value": f"item_{i}"} for i in range(100000)]

json_size = len(JsonSerializer().dumps(large_data))        # ~2.8MB
msgpack_size = len(MsgPackSerializer().dumps(large_data))  # ~1.5MB (47% smaller!)
```

### **2. Use Streaming for Large Files**
```python
# For very large files, process in chunks
def process_large_file(filename):
    with open(filename, 'rb') as f:
        while chunk := f.read(8192):  # 8KB chunks
            data = MsgPackSerializer().loads(chunk)
            process_chunk(data)
```

### **3. Cache Serializers**
```python
# Reuse serializer instances
class DataProcessor:
    def __init__(self):
        self.json_serializer = JsonSerializer()
        self.msgpack_serializer = MsgPackSerializer()
    
    def process(self, data, format_type):
        if format_type == "json":
            return self.json_serializer.dumps(data)
        elif format_type == "msgpack":
            return self.msgpack_serializer.dumps(data)
```

### **4. Enable Compression**
```python
# For network transmission, combine with compression
import gzip

data = large_dataset
msgpack_bytes = MsgPackSerializer().dumps(data)
compressed = gzip.compress(msgpack_bytes)

print(f"Original: {len(data)} objects")
print(f"MessagePack: {len(msgpack_bytes)} bytes")
print(f"Compressed: {len(compressed)} bytes")
# Often achieves 70-80% additional size reduction
```

---

## ?? **Advanced Usage**

### **Custom Serialization Options**
```python
# JSON with custom formatting
js = JsonSerializer(indent=2, sort_keys=True)
formatted_json = js.dumps(data)

# YAML with custom flow style
ys = YamlSerializer(default_flow_style=False)
readable_yaml = ys.dumps(data)

# MessagePack with custom types
mss = MsgPackSerializer(use_bin_type=True)
binary_msgpack = mss.dumps(data)
```

### **Error Handling**
```python
from exonware.xwsystem import SerializationError

try:
    # Attempt serialization
    result = serializer.dumps(complex_data)
except SerializationError as e:
    print(f"Format: {e.format_name}")
    print(f"Error: {e.message}")
    print(f"Original: {e.original_error}")
    
    # Fallback to different format
    fallback_result = JsonSerializer().dumps(simplified_data)
```

### **Batch Operations**
```python
# Process multiple items efficiently
def serialize_batch(items, serializer):
    results = []
    for item in items:
        try:
            result = serializer.dumps(item)
            results.append(result)
        except SerializationError:
            results.append(None)  # Mark failed items
    return results

batch_results = serialize_batch(data_items, MsgPackSerializer())
```

---

## ?? **Examples by Industry**

### **Web Development**
```python
# API responses
api_response = {"status": "success", "data": user_data, "timestamp": datetime.now()}
json_response = JsonSerializer().dumps(api_response)

# High-performance API with binary format
msgpack_response = MsgPackSerializer().dumps(api_response)  # 47% smaller
```

### **Data Engineering**
```python
# ETL pipeline with schema evolution
avro_serializer = AvroSerializer()
pipeline_data = extract_from_source()
avro_bytes = avro_serializer.dumps(pipeline_data, schema=current_schema)
load_to_destination(avro_bytes)
```

### **Game Development**
```python
# Real-time game state
game_serializer = FlatBuffersSerializer()
game_state = {
    "players": player_positions,
    "objects": world_objects,
    "events": recent_events
}
game_bytes = game_serializer.dumps(game_state)  # Zero-copy access
```

### **IoT/Embedded Systems**
```python
# Sensor data with minimal bandwidth
sensor_data = {"device_id": "sensor_01", "readings": [23.5, 24.1, 23.8]}
cbor_bytes = CborSerializer().dumps(sensor_data)  # Compact, standards-compliant
```

### **Enterprise Integration**
```python
# Cross-system communication
protobuf_serializer = ProtobufSerializer()
service_request = {"service": "inventory", "method": "check_stock", "params": {"sku": "ABC123"}}
protobuf_bytes = protobuf_serializer.dumps(service_request)
# Send to Java/C#/Go services
```

---

## ?? **Migration Guide**

### **From Single Format to xSystem**
```python
# Before: Using multiple libraries
import json
import yaml
import msgpack
import pickle

# After: One import for everything
from exonware.xwsystem import JsonSerializer, YamlSerializer, MsgPackSerializer, PickleSerializer

# Same functionality, consistent API
js = JsonSerializer()
ys = YamlSerializer()
mss = MsgPackSerializer()
ps = PickleSerializer()
```

### **Gradual Migration**
```python
# Step 1: Start with existing format
def serialize_data(data, format="json"):
    if format == "json":
        return JsonSerializer().dumps(data)
    # Add other formats gradually

# Step 2: Add performance-critical formats
def serialize_data(data, format="json"):
    serializers = {
        "json": JsonSerializer(),
        "msgpack": MsgPackSerializer(),  # Added for performance
        "avro": AvroSerializer(),        # Added for schema evolution
    }
    return serializers[format].dumps(data)
```

---

## ?? **Best Practices Summary**

1. **?? Security First**: Use safe formats for external data, validate all inputs
2. **? Performance**: Choose binary formats for large data, cache serializers
3. **?? Consistency**: Use the same API across all formats for maintainability
4. **?? Scalability**: Consider schema-based formats for long-term data evolution
5. **??? Error Handling**: Always handle SerializationError exceptions
6. **?? Monitoring**: Track serialization performance in production
7. **?? Testing**: Test with real data sizes and edge cases

---

**?? xSystem Serialization: 24 formats, one API, infinite possibilities.**

*Built with ?? by eXonware.com*

