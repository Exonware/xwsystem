# xSystem Test Suite

## Overview

The xSystem test suite is organized into three main categories:

- **Core Tests**: Basic functionality and integration tests
- **Unit Tests**: Individual component tests organized by module
- **Integration Tests**: Cross-module interaction tests

## Test Structure

```
tests/
+-- runner.py                    # Main test runner
+-- core/                        # Core functionality tests
�   +-- runner.py               # Core test runner
�   +-- test_core.py            # Core utility tests
�   +-- conftest.py             # Core test fixtures
+-- unit/                        # Unit tests by module
�   +-- runner.py               # Unit test runner
�   +-- config_tests/           # Configuration tests
�   +-- performance_tests/      # Performance tests
�   +-- security_tests/         # Security tests
�   +-- threading_tests/        # Threading tests
�   +-- io_tests/               # I/O tests
�   +-- structures_tests/       # Data structure tests
�   +-- patterns_tests/         # Design pattern tests
�   +-- serialization_tests/    # ?? Serialization tests (24 formats)
+-- integration/                 # Integration tests
    +-- runner.py               # Integration test runner
    +-- test_module_interactions.py
```

## Running Tests

### Run All Tests
```bash
python tests/runner.py
```

### Run Specific Categories
```bash
# Run only core tests
python tests/runner.py core

# Run only unit tests
python tests/runner.py unit

# Run only integration tests
python tests/runner.py integration
```

### Run Specific Unit Test Categories
```bash
# Run specific unit test category
python tests/runner.py unit config_tests
python tests/runner.py unit performance_tests
python tests/runner.py unit security_tests
python tests/runner.py unit serialization_tests  # ?? Test all 24 serialization formats
```

### Direct Runner Usage
```bash
# Run core tests directly
python tests/core/runner.py

# Run unit tests directly
python tests/unit/runner.py

# Run integration tests directly
python tests/integration/runner.py
```

## Test Markers

Tests are categorized using pytest markers:

- `@pytest.mark.xwsystem_core`: Core functionality tests
- `@pytest.mark.xwsystem_unit`: Unit tests for individual components
- `@pytest.mark.xwsystem_integration`: Integration tests between components
- `@pytest.mark.xwsystem_config`: Configuration and setup tests
- `@pytest.mark.xwsystem_performance`: Performance and benchmarking tests
- `@pytest.mark.xwsystem_security`: Security validation tests
- `@pytest.mark.xwsystem_threading`: Threading and concurrency tests
- `@pytest.mark.xwsystem_io`: I/O operations tests
- `@pytest.mark.xwsystem_structures`: Data structure tests
- `@pytest.mark.xwsystem_patterns`: Design pattern tests
- `@pytest.mark.xwsystem_monitoring`: Monitoring and metrics tests
- `@pytest.mark.xwsystem_serialization`: ?? Serialization tests (24 formats)

## Test Coverage

The test suite provides comprehensive coverage for:

### Core Tests
- ThreadSafeFactory functionality
- PathValidator security
- AtomicFileWriter operations
- CircularReferenceDetector
- GenericHandlerFactory

### Unit Tests
- Configuration management
- Performance monitoring
- Security validation
- Threading utilities
- I/O operations
- Data structures
- Design patterns
- ?? **Serialization (24 formats)**: JSON, YAML, TOML, XML, CSV, ConfigParser, FormData, Multipart, BSON, MessagePack, CBOR, Pickle, Marshal, SQLite3, DBM, Shelve, Plistlib, Apache Avro, Protocol Buffers, Apache Thrift, Apache Parquet, Apache ORC, Cap'n Proto, FlatBuffers

### Integration Tests
- Cross-module interactions
- Concurrent operations
- Error recovery scenarios
- Performance integration

## Adding New Tests

### For Core Tests
1. Add test file to `tests/core/`
2. Use `@pytest.mark.xwsystem_core` marker
3. Import from `exonware.xwsystem`

### For Unit Tests
1. Add test file to appropriate subdirectory in `tests/unit/`
2. Use `@pytest.mark.xwsystem_unit` marker
3. Test specific module functionality

### For Integration Tests
1. Add test file to `tests/integration/`
2. Use `@pytest.mark.xwsystem_integration` marker
3. Test interactions between multiple modules

### ?? For Serialization Tests
1. Add test file to `tests/unit/serialization_tests/`
2. Use `@pytest.mark.xwsystem_serialization` marker
3. Test specific serialization format functionality
4. Follow the comprehensive test pattern for all 24 formats

## Test Fixtures

Common test fixtures are available in `conftest.py` files:

- `clean_env`: Clean environment for testing
- `temp_log_dir`: Temporary logging directory
- `performance_data`: Sample performance data
- `circular_data`: Sample data with circular references

## Continuous Integration

Tests are automatically run in CI/CD pipelines with:

- Coverage reporting
- Performance benchmarking
- Security validation
- Cross-platform compatibility

## ?? **Serialization Testing (24 Formats)**

### **Test Structure**
```
tests/unit/serialization_tests/
+-- runner.py                           # Serialization test runner
+-- conftest.py                         # Serialization test fixtures
+-- test_all_serializers.py             # Comprehensive tests for all 24 formats
+-- test_complete_optimization.py       # Performance optimization tests
+-- test_optimization_progress.py       # Optimization progress tracking
+-- multilingual_emoji_test.py          # Unicode and multilingual support
```

### **Comprehensive Format Testing**

Each of the 24 serialization formats is tested with:

#### **Text Formats (8 formats)**
- **JSON**: Standard compliance, Unicode support, performance
- **YAML**: Complex structures, human readability, security
- **TOML**: Configuration format compliance, type preservation
- **XML**: Secure parsing, structure validation, namespace support
- **CSV**: Tabular data integrity, delimiter handling, encoding
- **ConfigParser**: INI format compliance, section handling
- **FormData**: URL encoding, special character handling
- **Multipart**: File upload simulation, boundary handling

#### **Binary Formats (9 formats)**
- **BSON**: MongoDB compatibility, type preservation, ObjectId support
- **MessagePack**: Size optimization (47% reduction), speed benchmarks
- **CBOR**: RFC 8949 compliance, type fidelity, compact encoding
- **Pickle**: Python object serialization, security warnings
- **Marshal**: Python internal format, version compatibility
- **SQLite3**: Database operations, query functionality, ACID properties
- **DBM**: Key-value operations, persistence, platform compatibility
- **Shelve**: Dictionary interface, persistence, concurrent access
- **Plistlib**: Apple format compliance, binary/XML variants

#### **?? Schema-Based Enterprise Formats (7 formats)**
- **Apache Avro**: Schema evolution, backward compatibility, fastavro integration
- **Protocol Buffers**: Cross-language compatibility, protobuf integration, performance
- **Apache Thrift**: RPC framework integration, cross-language serialization
- **Apache Parquet**: Columnar storage, analytics optimization, pyarrow integration
- **Apache ORC**: Row columnar format, compression efficiency, pyorc integration
- **Cap'n Proto**: Zero-copy deserialization, infinite speed claims (optional)
- **FlatBuffers**: Zero-copy access, memory efficiency, game development use cases

### **Test Categories**

#### **1. Functionality Tests**
```python
@pytest.mark.xwsystem_serialization
def test_serializer_roundtrip(serializer_class, test_data):
    """Test that data can be serialized and deserialized correctly."""
    serializer = serializer_class()
    
    # Serialize
    serialized = serializer.dumps(test_data)
    
    # Deserialize
    deserialized = serializer.loads(serialized)
    
    # Verify integrity
    assert deserialized == test_data
```

#### **2. Performance Tests**
```python
@pytest.mark.xwsystem_serialization
def test_serialization_performance(serializer_class, large_dataset):
    """Test serialization performance and size optimization."""
    serializer = serializer_class()
    
    # Measure serialization time
    start_time = time.time()
    serialized = serializer.dumps(large_dataset)
    serialize_time = time.time() - start_time
    
    # Measure deserialization time
    start_time = time.time()
    deserialized = serializer.loads(serialized)
    deserialize_time = time.time() - start_time
    
    # Verify performance benchmarks
    assert serialize_time < MAX_SERIALIZE_TIME
    assert deserialize_time < MAX_DESERIALIZE_TIME
    assert len(serialized) < MAX_SIZE_THRESHOLD
```

#### **3. Security Tests**
```python
@pytest.mark.xwsystem_serialization
def test_serializer_security(serializer_class):
    """Test security constraints and malicious input handling."""
    serializer = serializer_class()
    
    # Test with malicious input
    malicious_inputs = [
        "../../../etc/passwd",  # Path traversal
        "<script>alert('xss')</script>",  # XSS attempt
        "A" * 10000000,  # Large input
        {"depth": {"very": {"deep": {"nested": "data"}}}},  # Deep nesting
    ]
    
    for malicious_input in malicious_inputs:
        try:
            result = serializer.dumps(malicious_input)
            # Should not crash or expose security vulnerabilities
            assert result is not None
        except (SerializationError, ValidationError):
            # Expected for malicious input
            pass
```

#### **4. Error Handling Tests**
```python
@pytest.mark.xwsystem_serialization
def test_serializer_error_handling(serializer_class):
    """Test proper error handling for invalid data."""
    serializer = serializer_class()
    
    invalid_inputs = [
        lambda x: x,  # Non-serializable function
        object(),     # Generic object
        float('inf'), # Infinity
        float('nan'), # NaN
    ]
    
    for invalid_input in invalid_inputs:
        with pytest.raises(SerializationError):
            serializer.dumps(invalid_input)
```

#### **5. Unicode and Multilingual Tests**
```python
@pytest.mark.xwsystem_serialization
def test_unicode_support(serializer_class):
    """Test Unicode and multilingual character support."""
    serializer = serializer_class()
    
    multilingual_data = {
        "english": "Hello World",
        "chinese": "????",
        "arabic": "????? ???????",
        "emoji": "??????????",
        "special": "Special chars: ��� � � � �",
        "mixed": "Mixed: Hello ?? ?? ?????"
    }
    
    # Test roundtrip with Unicode data
    serialized = serializer.dumps(multilingual_data)
    deserialized = serializer.loads(serialized)
    
    assert deserialized == multilingual_data
```

### **Running Serialization Tests**

```bash
# Run all serialization tests
python tests/unit/serialization_tests/runner.py

# Run specific format tests
pytest tests/unit/serialization_tests/ -k "json"
pytest tests/unit/serialization_tests/ -k "avro"
pytest tests/unit/serialization_tests/ -k "protobuf"

# Run performance tests only
pytest tests/unit/serialization_tests/ -k "performance"

# Run with verbose output
python tests/unit/serialization_tests/runner.py -v
```

### **Test Data Sets**

The serialization tests use comprehensive test datasets:

```python
# Simple data
SIMPLE_DATA = {"key": "value", "number": 42, "boolean": True}

# Complex nested data
COMPLEX_DATA = {
    "users": [
        {"id": 1, "name": "John", "settings": {"theme": "dark"}},
        {"id": 2, "name": "Jane", "settings": {"theme": "light"}}
    ],
    "metadata": {"version": "1.0", "created": datetime.now()}
}

# Large dataset for performance testing
LARGE_DATA = {"items": [{"id": i, "value": f"item_{i}"} for i in range(10000)]}

# Edge cases
EDGE_CASES = [
    {},  # Empty dict
    [],  # Empty list
    None,  # None value
    "",  # Empty string
    0,  # Zero
    False,  # False boolean
]
```

### **Verification Script**

The `tests/verify_installation.py` script provides comprehensive verification:

```bash
# Verify all 24 formats are working
python tests/verify_installation.py

# Expected output:
# ?? Testing exonware.xwsystem serialization formats...
# ? JSON
# ? Apache Avro
# ? Protocol Buffers
# ? Apache Thrift
# ? Apache Parquet
# ? Apache ORC
# ? Cap'n Proto
# ? FlatBuffers
# 
# ?? SUCCESS! exonware.xwsystem is ready to use!
# You have access to enterprise-grade serialization with 24 formats!
```

## Best Practices

1. **Test Isolation**: Each test should be independent
2. **Descriptive Names**: Use clear, descriptive test names
3. **Proper Markers**: Always use appropriate pytest markers
4. **Error Handling**: Test both success and failure scenarios
5. **Performance**: Include performance tests for critical paths
6. **Security**: Validate security constraints and edge cases
7. **?? Format Coverage**: Test all 24 serialization formats comprehensively
8. **?? Schema Evolution**: Test backward compatibility for schema-based formats
9. **?? Cross-Platform**: Verify serialization works across different platforms

