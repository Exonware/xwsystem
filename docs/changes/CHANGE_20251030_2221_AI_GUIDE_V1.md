# ?? xSystem: AI-Friendly Complete Guide

**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Email:** connect@exonware.com  
**Version:** 0.0.1  
**Date:** January 31, 2025

---

## ?? **Quick Reference for AI Assistants**

This guide is specifically designed for AI assistants to understand xSystem's capabilities, patterns, and usage. All examples are complete and runnable.

### **?? What xSystem Solves**

xSystem eliminates the need for 50+ Python dependencies by providing a unified, production-grade framework with:

- **24 serialization formats** (JSON, YAML, TOML, XML, BSON, MessagePack, CBOR, Pickle, Avro, Protobuf, Parquet, etc.)
- **Async/sync dual APIs** for all I/O operations
- **Pydantic-style validation** with type coercion
- **Advanced HTTP client** with HTTP/2, streaming, mock transport
- **Production caching** (LRU, LFU, TTL) with async support
- **CLI utilities** (colors, progress bars, tables)
- **Low-level cryptography** (AEAD, key exchange, signatures)
- **System monitoring** (processes, hardware, network)
- **Human-friendly datetime** utilities
- **Thread-safe everything** by default

---

## ?? **Installation & Import Patterns**

```python
# Single import for everything
from exonware.xwsystem import *

# Or specific imports
from exonware.xwsystem import (
    # Serialization
    JsonSerializer, YamlSerializer, BsonSerializer,
    
    # HTTP
    AdvancedHttpClient, MockTransport,
    
    # Validation
    xModel, Field, ValidationError,
    
    # Caching
    LRUCache, AsyncLRUCache,
    
    # CLI
    colorize, Colors, ProgressBar,
    
    # Async I/O
    async_safe_write_text, async_safe_read_text,
    
    # Security
    AES_GCM, X25519_KeyExchange, Ed25519_Signature,
    
    # System
    get_cpu_usage, list_processes, get_hardware_info,
    
    # DateTime
    humanize_timedelta, time_ago, parse_human_duration
)
```

---

## ?? **Serialization: 24 Formats, One API**

### **Pattern: Consistent API Across All Formats**

```python
# Same API for any format
from exonware.xwsystem import JsonSerializer, YamlSerializer, BsonSerializer

data = {"users": [{"name": "Alice", "age": 30}], "count": 1}

# JSON
json_ser = JsonSerializer()
json_str = json_ser.dumps(data)
parsed = json_ser.loads(json_str)

# YAML - identical API
yaml_ser = YamlSerializer()
yaml_str = yaml_ser.dumps(data)
parsed = yaml_ser.loads(yaml_str)

# Binary formats - same pattern
bson_ser = BsonSerializer()
bson_bytes = bson_ser.dumps(data)
parsed = bson_ser.loads(bson_bytes)
```

### **Enterprise Schema Formats**

```python
from exonware.xwsystem import AvroSerializer, ProtobufSerializer, ParquetSerializer

# Apache Avro with schema evolution
avro = AvroSerializer()
avro_data = avro.dumps(data)

# Protocol Buffers (Google's format)
protobuf = ProtobufSerializer()
pb_data = protobuf.dumps(data)

# Apache Parquet (columnar analytics)
parquet = ParquetSerializer()
parquet_data = parquet.dumps(data)
```

---

## ? **Async Foundation: Everything is Async-Ready**

### **Pattern: Dual Sync/Async APIs**

```python
import asyncio
from exonware.xwsystem import (
    safe_write_text, async_safe_write_text,
    LRUCache, AsyncLRUCache,
    HttpClient, AdvancedHttpClient
)

# Sync version
safe_write_text("config.json", '{"key": "value"}')
content = safe_read_text("config.json")

# Async version - same API pattern
async def async_example():
    await async_safe_write_text("config.json", '{"key": "value"}')
    content = await async_safe_read_text("config.json")
    
    # Async caching
    cache = AsyncLRUCache(capacity=100)
    await cache.put("key", "value")
    result = await cache.get("key")
    
    # Async HTTP
    async with AdvancedHttpClient() as client:
        response = await client.get("https://api.example.com/data")
        data = response.json()

asyncio.run(async_example())
```

---

## ?? **Pydantic-Style Validation: Type Hints + Auto-Coercion**

### **Pattern: Declarative Models with Automatic Type Conversion**

```python
from exonware.xwsystem import xModel, Field, ValidationError
from typing import Optional, List
from datetime import datetime
from enum import Enum

class Priority(Enum):
    LOW = "low"
    MEDIUM = "medium" 
    HIGH = "high"

class User(xModel):
    # Basic types with auto-coercion
    name: str
    age: int = Field(ge=0, le=150)  # Constraints
    email: str = Field(pattern=r'^[^@]+@[^@]+\.[^@]+$')
    
    # Optional fields
    bio: Optional[str] = None
    
    # Collections
    tags: List[str] = Field(min_length=0, max_length=10)
    
    # Enums (auto-coerced from strings)
    priority: Priority = Priority.MEDIUM
    
    # Dates (auto-parsed from strings)
    created_at: datetime
    
    # Field with alias
    full_name: str = Field(alias="fullName")

# Auto type coercion in action
user_data = {
    "name": "Alice",
    "age": "30",  # String -> int
    "email": "alice@example.com",
    "tags": "python,web,api",  # String -> List[str]
    "priority": "high",  # String -> Enum
    "created_at": "2023-01-15T10:30:00",  # String -> datetime
    "fullName": "Alice Johnson"  # Alias mapping
}

user = User.model_validate(user_data)
print(f"Age: {user.age} (type: {type(user.age)})")  # Age: 30 (type: <class 'int'>)

# JSON Schema generation
schema = User.model_json_schema()
# Produces OpenAPI-compatible schema

# Serialization
json_data = user.model_dump_json()
dict_data = user.model_dump(exclude={"email"})
```

---

## ?? **Advanced HTTP: HTTP/2, Streaming, Mock Testing**

### **Pattern: Modern HTTP with Full Feature Set**

```python
from exonware.xwsystem import (
    AdvancedHttpClient, AdvancedHttpConfig, Http2Config, 
    StreamingConfig, MockTransport, MockResponse
)

# Production HTTP/2 client
config = AdvancedHttpConfig(
    http2=Http2Config(enabled=True, max_concurrent_streams=100),
    streaming=StreamingConfig(chunk_size=8192, max_content_length=1024*1024*10),
    verify_ssl=True,
    follow_redirects=True
)

async def http_examples():
    async with AdvancedHttpClient(config=config) as client:
        # Standard requests
        response = await client.get("https://api.example.com/users")
        users = response.json()
        
        # Streaming large files
        async with client.stream("GET", "https://example.com/large-file.zip") as response:
            async for chunk in response.aiter_bytes(8192):
                process_chunk(chunk)
        
        # File download with progress
        await client.download_file(
            "https://example.com/file.zip",
            "/tmp/downloaded.zip",
            progress_callback=lambda downloaded, total: print(f"{downloaded}/{total}")
        )

# Mock testing
def test_api_client():
    mock_responses = {
        "https://api.service.com/users": {
            "status_code": 200,
            "content": b'[{"id": 1, "name": "Alice"}]',
            "headers": {"Content-Type": "application/json"}
        }
    }
    
    transport = MockTransport(mock_responses)
    client = AdvancedHttpClient(transport=transport)
    
    response = await client.get("https://api.service.com/users")
    assert response.status_code == 200
    assert len(response.json()) == 1
```

---

## ??? **Caching: Production-Grade Performance**

### **Pattern: Multiple Cache Types with Consistent APIs**

```python
from exonware.xwsystem import LRUCache, LFUCache, TTLCache, AsyncLRUCache

# LRU Cache (Least Recently Used)
lru = LRUCache(capacity=1000, ttl=300)  # Optional TTL
lru.put("user:123", {"name": "Alice", "email": "alice@example.com"})
user = lru.get("user:123")

# LFU Cache (Least Frequently Used)
lfu = LFUCache(capacity=500)
lfu.put("popular:item", expensive_computation_result())

# TTL Cache (Time To Live)
ttl = TTLCache(capacity=200, ttl=60)  # 1 minute expiry
ttl.put("session:abc123", session_data)

# Async caching for high-concurrency
async def cache_example():
    cache = AsyncLRUCache(capacity=10000)
    
    # Batch operations
    tasks = []
    for i in range(100):
        tasks.append(cache.put(f"key_{i}", f"value_{i}"))
    await asyncio.gather(*tasks)
    
    # Statistics
    stats = await cache.get_stats()
    print(f"Hit rate: {stats['hit_rate']:.2%}")

# Dictionary-like interface
lru["key"] = "value"
value = lru["key"]
del lru["key"]
```

---

## ?? **CLI: Beautiful Terminal Applications**

### **Pattern: Cross-Platform Terminal Utilities**

```python
from exonware.xwsystem import (
    colorize, Colors, Style, print_colored,
    ProgressBar, Table, TableFormatter
)

# Colored output
print_colored("? Success!", Colors.GREEN, Style.BOLD)
print_colored("? Warning!", Colors.YELLOW)
print_colored("? Error!", Colors.RED, Style.BOLD)

# Advanced coloring
text = colorize("Important message", Colors.CYAN, Style.UNDERLINE)
print(text)

# Progress bars
progress = ProgressBar(total=100, description="Processing")
for i in range(100):
    time.sleep(0.01)
    progress.update(i + 1)
progress.close()

# Tables
table = Table()
table.add_column("Name", style=Colors.CYAN)
table.add_column("Age", justify="right")
table.add_column("Status", style=Colors.GREEN)

table.add_row("Alice", "30", "Active")
table.add_row("Bob", "25", "Inactive")
print(table)

# Automatic color detection (respects NO_COLOR, terminal capabilities)
if supports_color():
    print(colorize("Colorful!", Colors.RAINBOW))
else:
    print("Plain text fallback")
```

---

## ?? **Security: Military-Grade Cryptography**

### **Pattern: High-Level + Low-Level Crypto Access**

```python
from exonware.xwsystem import (
    # High-level (easy to use)
    SymmetricEncryption, AsymmetricEncryption, SecureHash,
    
    # Low-level (hazmat - for experts)
    AES_GCM, ChaCha20Poly1305_Cipher, X25519_KeyExchange, 
    Ed25519_Signature, HKDF_Expand, X509Certificate
)

# High-level encryption (recommended for most users)
key = SymmetricEncryption.generate_key()
encrypted = SymmetricEncryption.encrypt("sensitive data", key)
decrypted = SymmetricEncryption.decrypt(encrypted, key)

# Low-level AEAD encryption (for experts)
key = AES_GCM.generate_key(256)  # 256-bit key
cipher = AES_GCM(key)
nonce = AES_GCM.generate_nonce()
encrypted = cipher.encrypt(nonce, b"secret message", b"associated data")
decrypted = cipher.decrypt(nonce, encrypted, b"associated data")

# Modern key exchange
alice_kx = X25519_KeyExchange()
bob_kx = X25519_KeyExchange()

alice_public = alice_kx.get_public_key()
bob_public = bob_kx.get_public_key()

alice_shared = alice_kx.exchange(bob_public)
bob_shared = bob_kx.exchange(alice_public)
assert alice_shared == bob_shared  # Same shared secret

# Digital signatures
signer = Ed25519_Signature()
message = b"Important document"
signature = signer.sign(message)
public_key = signer.get_public_key()

is_valid = Ed25519_Signature.verify(public_key, signature, message)

# X.509 certificates
cert = X509Certificate.load_from_file("server.crt")
print(f"Subject: {cert.get_subject()}")
print(f"Valid until: {cert.get_not_valid_after()}")
print(f"Is valid now: {cert.is_valid_now()}")
```

---

## ?? **System Monitoring: Hardware & Process Insights**

### **Pattern: Cross-Platform System Information**

```python
from exonware.xwsystem import (
    get_cpu_usage, get_memory_usage, get_hardware_info,
    list_processes, get_process, SystemMonitor
)

# Quick system stats
cpu_percent = get_cpu_usage(interval=1.0)
memory = get_memory_usage()
print(f"CPU: {cpu_percent:.1f}%, Memory: {memory['percent']:.1f}%")

# Hardware information
hardware = get_hardware_info()
print(f"CPU cores: {hardware['cpu']['logical_cores']}")
print(f"Total RAM: {hardware['memory']['total_gb']} GB")

# Process monitoring
processes = list_processes()
python_procs = [p for p in processes if 'python' in p.name.lower()]

for proc in python_procs:
    print(f"PID {proc.pid}: {proc.name} - {proc.cpu_percent:.1f}% CPU")

# Detailed system monitor
monitor = SystemMonitor()
if monitor.is_available():
    # Get specific process
    current_proc = monitor.get_process(os.getpid())
    print(f"Current process memory: {current_proc.memory_rss / 1024 / 1024:.1f} MB")
    
    # Network interfaces
    interfaces = monitor.get_network_interfaces()
    for iface in interfaces:
        if iface.is_up:
            print(f"{iface.interface}: {iface.bytes_recv / 1024 / 1024:.1f} MB received")
```

---

## ?? **DateTime: Human-Friendly Time Handling**

### **Pattern: Natural Language Time Operations**

```python
from exonware.xwsystem import (
    humanize_timedelta, time_ago, time_until, 
    parse_human_duration, duration_to_human
)
from datetime import datetime, timedelta

# Humanize time differences
now = datetime.now()
past = now - timedelta(hours=2, minutes=30)
future = now + timedelta(days=3, hours=4)

print(time_ago(past))        # "2 hours ago"
print(time_until(future))    # "in 3 days"

# Natural duration parsing
duration1 = parse_human_duration("2 hours 30 minutes")
duration2 = parse_human_duration("1h 30m 45s")
duration3 = parse_human_duration("2:30:00")  # HH:MM:SS

# Duration to human readable
seconds = 7384  # Some duration in seconds
human = duration_to_human(seconds)  # "2 hours, 3 minutes"

# Smart formatting based on context
def smart_format(dt):
    diff = abs((dt - datetime.now()).total_seconds())
    
    if diff < 60:
        return "just now"
    elif diff < 3600:
        return f"{int(diff // 60)} minutes ago"
    elif dt.date() == datetime.now().date():
        return dt.strftime("%I:%M %p")  # "2:30 PM"
    else:
        return dt.strftime("%b %d, %Y")  # "Jan 15, 2023"

# Timezone-aware operations
from exonware.xwsystem import convert_timezone, get_local_timezone

utc_time = datetime.utcnow()
local_time = convert_timezone(utc_time, get_local_timezone())
```

---

## ?? **Threading: Async Primitives & Synchronization**

### **Pattern: Modern Async Concurrency**

```python
from exonware.xwsystem import (
    AsyncLock, AsyncSemaphore, AsyncEvent, AsyncQueue,
    AsyncCondition, AsyncResourcePool
)

async def concurrency_examples():
    # Async lock for critical sections
    lock = AsyncLock(name="database-lock")
    async with lock:
        # Critical section
        await update_database()
    
    # Semaphore for rate limiting
    semaphore = AsyncSemaphore(value=5, name="api-calls")
    async with semaphore:
        # Only 5 concurrent API calls
        response = await make_api_call()
    
    # Event for coordination
    event = AsyncEvent(name="data-ready")
    
    # Producer
    async def producer():
        await prepare_data()
        event.set()  # Signal that data is ready
    
    # Consumer
    async def consumer():
        await event.wait()  # Wait for data
        await process_data()
    
    # Queue for work distribution
    queue = AsyncQueue(maxsize=100, name="work-queue")
    
    # Producer-consumer pattern
    await queue.put(work_item)
    work_item = await queue.get()
    
    # Resource pool for connection management
    db_connections = [create_db_connection() for _ in range(10)]
    pool = AsyncResourcePool(db_connections, name="db-pool")
    
    async with pool.get_resource() as conn:
        result = await conn.execute("SELECT * FROM users")
```

---

## ?? **Best Practices for AI Assistants**

### **1. Import Patterns**

```python
# ? Recommended: Import what you need
from exonware.xwsystem import LRUCache, xModel, Field, colorize

# ? Also good: Star import for comprehensive usage
from exonware.xwsystem import *

# ? Avoid: Deep imports (may change)
# from exonware.xwsystem.caching.lru_cache import LRUCache
```

### **2. Error Handling Patterns**

```python
from exonware.xwsystem import ValidationError, FileOperationError, HttpError

# Validation errors
try:
    user = User(name="", age=-5)
except ValidationError as e:
    print(f"Validation failed: {e}")
    for error in e.errors:
        print(f"  {error['field']}: {error['message']}")

# File operation errors
try:
    content = await async_safe_read_text("nonexistent.txt")
except FileOperationError as e:
    print(f"File error: {e}")

# HTTP errors
try:
    response = await client.get("https://api.example.com/data")
except HttpError as e:
    print(f"HTTP error {e.status_code}: {e}")
```

### **3. Async/Await Patterns**

```python
# ? Always use async context managers for async resources
async with AsyncLRUCache(100) as cache:
    await cache.put("key", "value")

async with AdvancedHttpClient() as client:
    response = await client.get("https://api.example.com")

# ? Batch async operations
tasks = [
    async_safe_write_text(f"file_{i}.txt", f"content_{i}")
    for i in range(10)
]
await asyncio.gather(*tasks)
```

### **4. Configuration Patterns**

```python
# ? Use configuration objects for complex setups
config = AdvancedHttpConfig(
    http2=Http2Config(enabled=True),
    streaming=StreamingConfig(chunk_size=4096),
    retry=RetryConfig(max_retries=3, base_delay=1.0)
)

client = AdvancedHttpClient(config=config)

# ? Use Field constraints for validation
class APIConfig(xModel):
    url: str = Field(pattern=r'^https?://')
    timeout: float = Field(gt=0, le=300)
    retries: int = Field(ge=0, le=10)
```

---

## ?? **Migration from Common Libraries**

### **From Pydantic**

```python
# Old Pydantic code
from pydantic import BaseModel, Field as PydanticField

class OldUser(BaseModel):
    name: str
    age: int = PydanticField(ge=0)

# New xSystem code (drop-in replacement)
from exonware.xwsystem import xModel, Field

class NewUser(xModel):
    name: str
    age: int = Field(ge=0)

# Same API: model_validate, model_dump, model_json_schema
```

### **From requests/httpx**

```python
# Old requests/httpx
import requests
import httpx

response = requests.get("https://api.example.com")
async with httpx.AsyncClient() as client:
    response = await client.get("https://api.example.com")

# New xSystem (enhanced features)
from exonware.xwsystem import AdvancedHttpClient

async with AdvancedHttpClient() as client:
    response = await client.get("https://api.example.com")
    # Now with HTTP/2, streaming, mock support, etc.
```

### **From functools.lru_cache**

```python
# Old functools
from functools import lru_cache

@lru_cache(maxsize=128)
def expensive_function(arg):
    return complex_computation(arg)

# New xSystem (more features)
from exonware.xwsystem import LRUCache

cache = LRUCache(capacity=128, ttl=300)  # With TTL!

def expensive_function(arg):
    result = cache.get(arg)
    if result is None:
        result = complex_computation(arg)
        cache.put(arg, result)
    return result
```

---

## ?? **Performance Tips**

1. **Use async APIs for I/O-bound operations**
2. **Configure cache sizes based on memory constraints**
3. **Use streaming for large HTTP responses**
4. **Enable HTTP/2 for better performance**
5. **Use TTL caches for time-sensitive data**
6. **Batch async operations with asyncio.gather()**
7. **Use appropriate serialization formats (binary for performance, text for debugging)**

---

## ?? **Troubleshooting Common Issues**

### **Import Errors**
```python
# If you get import errors, ensure dependencies are installed
pip install aiofiles httpx psutil cryptography
```

### **Async Context Warnings**
```python
# ? Always use proper async context managers
async with client:  # Good
    await client.get(...)

# ? Don't forget to close resources
client = AdvancedHttpClient()
await client.get(...)  # Missing close - may warn
```

### **Validation Errors**
```python
# Check field constraints and types
class User(xModel):
    age: int = Field(ge=0)  # Must be >= 0

try:
    User(age=-1)  # Will raise ValidationError
except ValidationError as e:
    print(e.errors)  # Shows specific field errors
```

---

This guide provides AI assistants with comprehensive patterns and examples for using xSystem effectively. All code examples are production-ready and demonstrate best practices.

