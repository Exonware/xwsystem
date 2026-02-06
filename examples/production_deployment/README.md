# Production Deployment Examples

This directory contains production-ready examples demonstrating xwsystem in real-world scenarios.

## Examples

### 1. Web API Example (`web_api_example.py`)

A production-ready REST API demonstrating:
- Multiple serialization formats (JSON, YAML, MessagePack)
- Security validation with PathValidator
- Performance monitoring
- Circuit breakers for external API calls
- Memory leak prevention
- Health checks and metrics

**Features:**
- FastAPI-based REST API
- Format-agnostic data endpoints
- Secure file storage
- Real-time monitoring
- Circuit breaker protection

**Usage:**
```bash
python web_api_example.py
# API available at http://localhost:8000
```

### 2. Microservices Example (`microservices_example.py`)

A microservices architecture demonstrating:
- Circuit breakers for service resilience
- Inter-service communication
- Performance monitoring across services
- Graceful degradation
- Service health tracking

**Features:**
- Multiple service clients
- Circuit breaker per service
- Performance metrics aggregation
- Error handling and recovery

**Usage:**
```bash
# Start services (user, order, payment)
python microservices_example.py
# Gateway available at http://localhost:8000
```

### 3. Data Pipeline Example (`data_pipeline_example.py`)

A data processing pipeline demonstrating:
- Multi-stage data transformation
- Format conversion (JSON → MessagePack → Avro → Parquet)
- Performance monitoring per stage
- Memory management
- Circular reference detection

**Features:**
- Configurable pipeline stages
- Format conversion
- Atomic file operations
- Performance tracking
- Error recovery

**Usage:**
```bash
python data_pipeline_example.py
```

### 4. Security-Hardened Example (`security_hardened_example.py`)

A security-hardened application demonstrating:
- Path validation and sanitization
- Secure storage with encryption
- Data signing and verification
- Resource limits
- Type validation

**Features:**
- Symmetric and asymmetric encryption
- Secure file operations
- Data validation
- Resource limits
- Security monitoring

**Usage:**
```bash
python security_hardened_example.py
```

## Common Patterns

### Circuit Breakers
```python
from exonware.xwsystem import CircuitBreaker

breaker = CircuitBreaker(failure_threshold=5, recovery_timeout=30)

@breaker
async def call_service():
    # Service call protected by circuit breaker
    pass
```

### Performance Monitoring
```python
from exonware.xwsystem import PerformanceMonitor

monitor = PerformanceMonitor()
monitor.start()

with monitor.measure("operation_name"):
    # Your operation
    pass

stats = monitor.get_stats()
```

### Secure Storage
```python
from exonware.xwsystem import SecureStorage, PathValidator

storage = SecureStorage()
validator = PathValidator(base_path="/safe/directory")

safe_path = validator.validate_path("user/data.json")
storage.store("key", {"data": "value"})
```

### Format Conversion
```python
from exonware.xwsystem import JsonSerializer, MsgPackSerializer

json_ser = JsonSerializer()
msgpack_ser = MsgPackSerializer()

data = {"key": "value"}
json_data = json_ser.dumps(data)
msgpack_data = msgpack_ser.dumps(data)
```

## Production Considerations

1. **Security:**
   - Always use PathValidator for file operations
   - Enable encryption for sensitive data
   - Use SecureStorage for credentials
   - Validate all inputs

2. **Performance:**
   - Enable performance monitoring
   - Use appropriate serialization formats
   - Monitor memory usage
   - Use circuit breakers for external calls

3. **Reliability:**
   - Implement circuit breakers
   - Use atomic file operations
   - Monitor and log errors
   - Implement health checks

4. **Monitoring:**
   - Track performance metrics
   - Monitor memory usage
   - Log all operations
   - Set up alerts

## Deployment

### Docker Example
```dockerfile
FROM python:3.12-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
COPY . .
CMD ["python", "web_api_example.py"]
```

### Environment Variables
```bash
# Security
XWSYSTEM_BASE_PATH=/secure/data
XWSYSTEM_ENCRYPTION_KEY=your-key-here

# Performance
XWSYSTEM_MONITORING_ENABLED=true
XWSYSTEM_MEMORY_LIMIT=100MB
```

## Testing

Each example includes error handling and can be tested with:

```bash
# Test web API
curl http://localhost:8000/health

# Test data pipeline
python -m pytest data_pipeline_example.py

# Test security
python security_hardened_example.py
```

## License

MIT License - see LICENSE file in parent directory.
