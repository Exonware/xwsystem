# Real-World Examples with xwsystem

**Company:** eXonware.com  
**Author:** Eng. Muhammad AlShehri  
**Email:** connect@exonware.com  
**Version:** 1.0.0

## Overview

This document provides real-world examples demonstrating xwsystem in various production scenarios. Each example includes complete code, explanations, and best practices.

## Table of Contents

1. [E-Commerce API](#e-commerce-api)
2. [ML Pipeline with Performance Monitoring](#ml-pipeline-with-performance-monitoring)
3. [Enterprise Application with Security](#enterprise-application-with-security)
4. [High-Performance Caching Scenarios](#high-performance-caching-scenarios)
5. [Data Processing Workflow](#data-processing-workflow)

## E-Commerce API

### Scenario

Build a REST API for an e-commerce platform that handles:
- Product catalog (JSON, YAML, MessagePack)
- Order processing with multiple formats
- Secure payment data storage
- Performance monitoring
- Circuit breakers for external services

### Implementation

```python
from fastapi import FastAPI, HTTPException
from exonware.xwsystem import (
    JsonSerializer,
    MsgPackSerializer,
    SecureStorage,
    PathValidator,
    CircuitBreaker,
    PerformanceMonitor,
    MemoryMonitor,
    AtomicFileWriter,
)

app = FastAPI()
json_ser = JsonSerializer()
msgpack_ser = MsgPackSerializer()
secure_storage = SecureStorage()
path_validator = PathValidator(base_path="/data/ecommerce")
perf_monitor = PerformanceMonitor()
memory_monitor = MemoryMonitor(enable_auto_cleanup=True)

# Circuit breaker for payment gateway
payment_breaker = CircuitBreaker(failure_threshold=5, recovery_timeout=30)

@app.post("/products")
async def create_product(product_data: dict):
    """Create product with format selection."""
    # Validate input
    if not product_data.get("name"):
        raise HTTPException(400, "Product name required")
    
    # Serialize based on client preference
    format_type = product_data.pop("format", "json")
    serializer = json_ser if format_type == "json" else msgpack_ser
    
    with perf_monitor.measure("product_creation"):
        # Store product
        product_id = product_data.get("id")
        safe_path = path_validator.validate_path(f"products/{product_id}.{format_type}")
        
        serialized = serializer.dumps(product_data)
        with AtomicFileWriter(safe_path) as writer:
            writer.write(serialized)
    
    return {"status": "success", "product_id": product_id}

@app.post("/orders")
async def create_order(order_data: dict):
    """Create order with payment processing."""
    try:
        # Process order
        order_id = order_data.get("id")
        
        # Store order securely
        secure_storage.store(f"order_{order_id}", order_data)
        
        # Process payment (with circuit breaker)
        @payment_breaker
        async def process_payment():
            # Payment processing logic
            return {"status": "paid"}
        
        payment_result = await process_payment()
        
        return {
            "status": "success",
            "order_id": order_id,
            "payment": payment_result
        }
    except Exception as e:
        raise HTTPException(500, str(e))

@app.get("/metrics")
async def get_metrics():
    """Get performance metrics."""
    return {
        "performance": perf_monitor.get_stats(),
        "memory": memory_monitor.get_current_usage(),
        "circuit_breaker": {
            "state": payment_breaker.state,
            "failures": payment_breaker.failure_count
        }
    }
```

### Key Features

- **Multi-format support**: JSON and MessagePack
- **Secure storage**: Payment data encrypted
- **Performance monitoring**: Track all operations
- **Circuit breakers**: Protect against payment gateway failures
- **Atomic operations**: Prevent data corruption

## ML Pipeline with Performance Monitoring

### Scenario

Build a machine learning pipeline that:
- Processes data in multiple formats
- Monitors performance at each stage
- Handles large datasets efficiently
- Tracks memory usage
- Optimizes serialization format

### Implementation

```python
import asyncio
from exonware.xwsystem import (
    JsonSerializer,
    AvroSerializer,
    ParquetSerializer,
    PerformanceMonitor,
    MemoryMonitor,
    CircularReferenceDetector,
)

class MLPipeline:
    def __init__(self):
        self.json_ser = JsonSerializer()
        self.avro_ser = AvroSerializer()
        self.parquet_ser = ParquetSerializer()
        self.perf_monitor = PerformanceMonitor()
        self.memory_monitor = MemoryMonitor(enable_auto_cleanup=True)
        self.circular_detector = CircularReferenceDetector()
    
    async def process_dataset(self, data_path: str):
        """Process ML dataset through pipeline."""
        self.perf_monitor.start()
        self.memory_monitor.start_monitoring()
        
        try:
            # Stage 1: Load and validate
            with self.perf_monitor.measure("load_data"):
                data = self.json_ser.loads_from_file(data_path)
                
                # Check for circular references
                if self.circular_detector.is_circular(data):
                    data = self.circular_detector.resolve_circular_refs(data)
            
            # Stage 2: Transform to Avro (schema evolution)
            with self.perf_monitor.measure("transform_avro"):
                avro_data = self.avro_ser.dumps(data)
            
            # Stage 3: Convert to Parquet (analytics)
            with self.perf_monitor.measure("convert_parquet"):
                # Load from Avro first
                avro_obj = self.avro_ser.loads(avro_data)
                parquet_data = self.parquet_ser.dumps(avro_obj)
            
            # Get performance stats
            stats = self.perf_monitor.get_stats()
            memory = self.memory_monitor.get_current_usage()
            
            return {
                "status": "success",
                "stages": {
                    "load": stats.get("load_data", {}),
                    "transform": stats.get("transform_avro", {}),
                    "convert": stats.get("convert_parquet", {})
                },
                "memory": memory,
                "sizes": {
                    "json": len(self.json_ser.dumps(data)),
                    "avro": len(avro_data),
                    "parquet": len(parquet_data)
                }
            }
        finally:
            self.perf_monitor.stop()
            self.memory_monitor.stop_monitoring()

# Usage
pipeline = MLPipeline()
result = asyncio.run(pipeline.process_dataset("dataset.json"))
print(f"Pipeline completed: {result}")
```

### Key Features

- **Format optimization**: JSON → Avro → Parquet
- **Performance tracking**: Monitor each stage
- **Memory management**: Auto-cleanup enabled
- **Circular reference handling**: Prevents infinite loops
- **Size comparison**: Compare format efficiency

## Enterprise Application with Security

### Scenario

Build an enterprise application with:
- Multi-level security
- Audit logging
- Secure data storage
- Access control
- Compliance features

### Implementation

```python
from exonware.xwsystem import (
    SecureStorage,
    SecureHash,
    SymmetricEncryption,
    AsymmetricEncryption,
    PathValidator,
    AtomicFileWriter,
    SafeTypeValidator,
    ResourceLimits,
)

class EnterpriseApp:
    def __init__(self):
        self.secure_storage = SecureStorage()
        self.path_validator = PathValidator(base_path="/secure/enterprise")
        self.type_validator = SafeTypeValidator()
        self.resource_limits = ResourceLimits(
            max_file_size=50 * 1024 * 1024,  # 50MB
            max_memory=500 * 1024 * 1024,     # 500MB
            max_depth=50
        )
        
        # Encryption
        self.symmetric = SymmetricEncryption()
        self.asymmetric, self.private_key, self.public_key = \
            AsymmetricEncryption.generate_key_pair(4096)
    
    def store_sensitive_document(self, doc_id: str, content: str, user_id: str):
        """Store sensitive document with encryption and audit."""
        # Validate input
        if not self.type_validator.validate_untrusted_data({"content": content}):
            raise ValueError("Invalid content")
        
        # Encrypt content
        encrypted = self.symmetric.encrypt(content)
        
        # Create audit log
        audit_entry = {
            "action": "store_document",
            "doc_id": doc_id,
            "user_id": user_id,
            "timestamp": "2025-01-01T00:00:00Z",
            "hash": SecureHash.sha256(content)
        }
        
        # Sign audit entry
        audit_str = str(audit_entry)
        signature = self.asymmetric.sign(audit_str, self.private_key)
        
        # Store document
        self.secure_storage.store(f"doc_{doc_id}", {
            "encrypted_content": encrypted,
            "user_id": user_id,
            "audit": audit_entry,
            "signature": signature
        })
        
        return {"status": "success", "doc_id": doc_id}
    
    def retrieve_document(self, doc_id: str, user_id: str):
        """Retrieve document with access control."""
        stored = self.secure_storage.retrieve(f"doc_{doc_id}")
        
        # Verify access
        if stored["user_id"] != user_id:
            raise PermissionError("Access denied")
        
        # Verify signature
        audit_str = str(stored["audit"])
        is_valid = self.asymmetric.verify(
            audit_str,
            stored["signature"],
            self.public_key
        )
        
        if not is_valid:
            raise ValueError("Signature verification failed")
        
        # Decrypt content
        content = self.symmetric.decrypt(stored["encrypted_content"])
        
        return {
            "content": content,
            "audit": stored["audit"],
            "verified": is_valid
        }

# Usage
app = EnterpriseApp()
app.store_sensitive_document("doc_123", "Sensitive content", "user_456")
document = app.retrieve_document("doc_123", "user_456")
```

### Key Features

- **Multi-layer encryption**: Symmetric and asymmetric
- **Audit logging**: Complete audit trail
- **Digital signatures**: Verify data integrity
- **Access control**: User-based permissions
- **Input validation**: Safe type validation

## High-Performance Caching Scenarios

### Scenario

Implement high-performance caching for:
- API response caching
- Database query caching
- Session management
- Real-time data caching

### Implementation

```python
from exonware.xwsystem import (
    LRUCache,
    LFUCache,
    TTLCache,
    TwoTierCache,
    ReadThroughCache,
    SerializableCache,
)

# API response cache (LRU)
api_cache = LRUCache(max_size=1000)

# Database query cache (LFU - frequently used queries)
db_cache = LFUCache(max_size=500)

# Session cache (TTL - expire after 1 hour)
session_cache = TTLCache(max_size=10000, default_ttl=3600)

# Two-tier cache (memory + disk)
two_tier = TwoTierCache(
    l1_cache=LRUCache(max_size=100),
    l2_cache=SerializableCache(max_size=1000, storage_path="/cache/disk")
)

# Read-through cache (auto-load on miss)
def load_user(user_id):
    # Database query
    return {"id": user_id, "name": "User"}

read_through = ReadThroughCache(
    cache=LRUCache(max_size=500),
    loader=load_user
)

# Usage examples
# API caching
def get_api_response(endpoint):
    if endpoint in api_cache:
        return api_cache[endpoint]
    response = fetch_from_api(endpoint)
    api_cache[endpoint] = response
    return response

# Database query caching
def get_user_queries(user_id):
    cache_key = f"queries_{user_id}"
    if cache_key in db_cache:
        return db_cache[cache_key]
    queries = fetch_from_db(user_id)
    db_cache[cache_key] = queries
    return queries

# Session management
def get_session(session_id):
    return session_cache.get(session_id)

def set_session(session_id, data):
    session_cache[session_id] = data

# Two-tier caching
def get_cached_data(key):
    return two_tier.get(key)

# Read-through caching
user = read_through.get("user_123")  # Auto-loads if not cached
```

### Key Features

- **Multiple cache strategies**: LRU, LFU, TTL
- **Two-tier caching**: Memory + disk
- **Read-through pattern**: Auto-load on miss
- **Serializable cache**: Persist to disk
- **Performance optimized**: Fast lookups

## Data Processing Workflow

### Scenario

Build a data processing workflow that:
- Handles multiple data sources
- Converts between formats
- Processes in parallel
- Monitors progress
- Handles errors gracefully

### Implementation

```python
import asyncio
from exonware.xwsystem import (
    JsonSerializer,
    YamlSerializer,
    MsgPackSerializer,
    PerformanceMonitor,
    MemoryMonitor,
    AtomicFileWriter,
    PathValidator,
)

class DataWorkflow:
    def __init__(self):
        self.serializers = {
            "json": JsonSerializer(),
            "yaml": YamlSerializer(),
            "msgpack": MsgPackSerializer(),
        }
        self.perf_monitor = PerformanceMonitor()
        self.memory_monitor = MemoryMonitor(enable_auto_cleanup=True)
        self.path_validator = PathValidator(base_path="/data/workflow")
    
    async def process_multiple_sources(self, sources: list):
        """Process multiple data sources in parallel."""
        self.perf_monitor.start()
        self.memory_monitor.start_monitoring()
        
        try:
            tasks = [self.process_source(source) for source in sources]
            results = await asyncio.gather(*tasks, return_exceptions=True)
            
            # Aggregate results
            successful = [r for r in results if not isinstance(r, Exception)]
            failed = [r for r in results if isinstance(r, Exception)]
            
            return {
                "successful": len(successful),
                "failed": len(failed),
                "results": successful,
                "errors": [str(e) for e in failed],
                "performance": self.perf_monitor.get_stats(),
                "memory": self.memory_monitor.get_current_usage()
            }
        finally:
            self.perf_monitor.stop()
            self.memory_monitor.stop_monitoring()
    
    async def process_source(self, source: dict):
        """Process a single data source."""
        source_path = source["path"]
        input_format = source.get("format", "json")
        output_format = source.get("output_format", "msgpack")
        
        with self.perf_monitor.measure(f"process_{source_path}"):
            # Load data
            input_ser = self.serializers[input_format]
            data = input_ser.loads_from_file(source_path)
            
            # Transform (example: add metadata)
            data["processed"] = True
            data["timestamp"] = "2025-01-01T00:00:00Z"
            
            # Save in output format
            output_ser = self.serializers[output_format]
            output_data = output_ser.dumps(data)
            
            output_path = self.path_validator.validate_path(
                f"output/{source_path}.{output_format}"
            )
            with AtomicFileWriter(output_path) as writer:
                writer.write(output_data)
            
            return {
                "source": source_path,
                "output": str(output_path),
                "size": len(output_data)
            }

# Usage
workflow = DataWorkflow()
sources = [
    {"path": "data1.json", "format": "json", "output_format": "msgpack"},
    {"path": "data2.yaml", "format": "yaml", "output_format": "json"},
    {"path": "data3.json", "format": "json", "output_format": "msgpack"},
]

result = asyncio.run(workflow.process_multiple_sources(sources))
print(f"Workflow completed: {result}")
```

### Key Features

- **Parallel processing**: Multiple sources simultaneously
- **Format conversion**: Flexible input/output formats
- **Error handling**: Graceful failure handling
- **Performance tracking**: Monitor all operations
- **Memory management**: Auto-cleanup enabled

## Best Practices Summary

1. **Always use PathValidator** for file operations
2. **Enable performance monitoring** in production
3. **Use appropriate serialization formats** for your use case
4. **Implement circuit breakers** for external services
5. **Enable memory monitoring** with auto-cleanup
6. **Use secure storage** for sensitive data
7. **Validate all inputs** before processing
8. **Handle errors gracefully** with proper logging
9. **Monitor performance metrics** regularly
10. **Test error scenarios** before deployment

## Additional Resources

- [Production Deployment Guide](PRODUCTION_GUIDE.md)
- [Complete API Reference](REF_API.md)
- [Examples Directory](../examples/)
- [Performance Benchmarks](REF_BENCH.md)

---

**Last Updated:** January 2025
