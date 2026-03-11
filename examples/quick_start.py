#!/usr/bin/env python3
#exonware/xwsystem/examples/quick_start.py
"""
xSystem Quick Start Examples
This file demonstrates the most common xSystem usage patterns.
Perfect for new users to get started quickly.
"""

from exonware.xwsystem import (
    # Convenience functions - easiest way to get started
    quick_serialize, quick_deserialize, quick_hash, quick_encrypt, quick_decrypt,
    # Core serialization
    JsonSerializer, YamlSerializer, MsgPackSerializer,
    # Security
    SecureHash, SymmetricEncryption, PathValidator,
    # HTTP client
    HttpClient, RetryConfig,
    # Performance monitoring
    PerformanceMonitor, CircuitBreaker,
    # Threading
    ThreadSafeFactory, AsyncLock
)


def demo_convenience_functions():
    """Demonstrate the new convenience functions - easiest way to use xSystem."""
    print("🚀 CONVENIENCE FUNCTIONS DEMO")
    print("=" * 50)
    # Quick serialization - auto-detects format
    data = {"name": "xSystem", "version": "0.0.1.3", "features": ["fast", "secure"]}
    # JSON
    json_str = quick_serialize(data, "json")
    print(f"JSON: {json_str}")
    # YAML  
    yaml_str = quick_serialize(data, "yaml")
    print(f"YAML:\n{yaml_str}")
    # MessagePack (binary, 47% smaller)
    msgpack_bytes = quick_serialize(data, "msgpack")
    print(f"MessagePack: {len(msgpack_bytes)} bytes (vs {len(json_str)} JSON chars)")
    # Auto-detection deserialization
    parsed_json = quick_deserialize(json_str)
    parsed_yaml = quick_deserialize(yaml_str, "yaml")  
    parsed_msgpack = quick_deserialize(msgpack_bytes, "msgpack")
    print(f"All equal: {parsed_json == parsed_yaml == parsed_msgpack == data}")
    # Quick hashing
    password = "super_secure_password"
    sha256_hash = quick_hash(password, "sha256")
    blake2b_hash = quick_hash(password, "blake2b")
    print(f"SHA256: {sha256_hash[:16]}...")
    print(f"BLAKE2b: {blake2b_hash[:16]}...")
    # Quick encryption
    secret_data = "This is confidential information"
    encrypted, key = quick_encrypt(secret_data)
    decrypted = quick_decrypt(encrypted, key)
    print(f"Encrypted: {len(encrypted)} bytes")
    print(f"Decrypted: {decrypted}")
    print(f"Round trip successful: {secret_data == decrypted}")


def demo_serialization():
    """Demonstrate 24 serialization formats with consistent API."""
    print("\n📦 SERIALIZATION DEMO")  
    print("=" * 50)
    # Complex data structure
    data = {
        "users": [
            {"id": 1, "name": "Alice", "active": True},
            {"id": 2, "name": "Bob", "active": False}
        ],
        "metadata": {
            "created": "2025-01-31",
            "version": 2.1,
            "tags": ["production", "api"]
        }
    }
    # Text formats
    json_serializer = JsonSerializer()
    yaml_serializer = YamlSerializer()
    json_output = json_serializer.dumps(data)
    yaml_output = yaml_serializer.dumps(data)
    print(f"JSON size: {len(json_output)} chars")
    print(f"YAML size: {len(yaml_output)} chars")
    # Binary format (more efficient)
    msgpack_serializer = MsgPackSerializer()
    msgpack_output = msgpack_serializer.dumps(data)
    print(f"MessagePack size: {len(msgpack_output)} bytes ({100 - len(msgpack_output)/len(json_output)*100:.1f}% smaller)")
    # All deserialize to the same data
    json_parsed = json_serializer.loads(json_output)
    yaml_parsed = yaml_serializer.loads(yaml_output)
    msgpack_parsed = msgpack_serializer.loads(msgpack_output)
    print(f"All formats equal: {json_parsed == yaml_parsed == msgpack_parsed == data}")


def demo_security():
    """Demonstrate security features."""
    print("\n🔐 SECURITY DEMO")
    print("=" * 50)
    # Secure hashing
    password = "user_password_123"
    hashed = SecureHash.sha256(password)
    print(f"Password hash: {hashed[:32]}...")
    # Symmetric encryption
    sensitive_data = "Credit card: 4111-1111-1111-1111"
    encryption = SymmetricEncryption()
    encrypted = encryption.encrypt(sensitive_data)
    decrypted = encryption.decrypt(encrypted)
    print(f"Original: {sensitive_data}")
    print(f"Encrypted: {encrypted[:32]}...")
    print(f"Decrypted: {decrypted}")
    print(f"Encryption successful: {sensitive_data == decrypted}")
    # Path validation (security against directory traversal)
    validator = PathValidator(base_path="/safe/directory")
    try:
        safe_path = validator.validate_path("config/settings.json")  # OK
        print(f"Safe path: {safe_path}")
        dangerous_path = validator.validate_path("../../../etc/passwd")  # Blocked
        print(f"This shouldn't print: {dangerous_path}")
    except Exception as e:
        print(f"Blocked dangerous path: {e}")


def demo_http_client():
    """Demonstrate HTTP client with retry logic."""
    print("\n🌐 HTTP CLIENT DEMO")
    print("=" * 50)
    # Create HTTP client with retry configuration
    retry_config = RetryConfig(
        max_retries=3,
        base_delay=1.0,
        retry_on_status=[500, 502, 503, 504]
    )
    client = HttpClient(
        timeout=10.0,
        retry_config=retry_config,
        default_headers={"User-Agent": f"xSystem/0.0.1.3"}
    )
    try:
        # Example API call (this will work if you have internet)
        print("Making HTTP request to httpbin.org...")
        response = client.get("https://httpbin.org/json")
        if response.status_code == 200:
            data = response.json()
            print(f"Response: {data}")
        else:
            print(f"HTTP {response.status_code}: {response.text[:100]}...")
    except Exception as e:
        print(f"HTTP request failed (this is normal if offline): {e}")


def demo_performance_monitoring():
    """Demonstrate performance monitoring."""
    print("\n📊 PERFORMANCE MONITORING DEMO")
    print("=" * 50)
    import time
    # Performance monitor
    monitor = PerformanceMonitor()
    # Simulate some work
    with monitor.measure("data_processing"):
        # Simulate processing
        time.sleep(0.1)
        # Nested operation
        with monitor.measure("database_query"):
            time.sleep(0.05)
    # Get statistics
    stats = monitor.get_stats()
    print(f"Operations performed: {stats.operations_count}")
    print(f"Total time: {stats.total_processing_time:.3f}s")
    # Circuit breaker pattern
    @CircuitBreaker(failure_threshold=3, recovery_timeout=5)
    def unreliable_operation():
        import random
        if random.random() < 0.7:  # 70% failure rate
            raise Exception("Simulated failure")
        return "Success!"
    # Test circuit breaker
    for i in range(5):
        try:
            result = unreliable_operation()
            print(f"Attempt {i+1}: {result}")
        except Exception as e:
            print(f"Attempt {i+1}: Failed - {e}")


def demo_threading():
    """Demonstrate thread-safe operations."""
    print("\n🧵 THREADING DEMO")
    print("=" * 50)
    import threading
    import time
    # Thread-safe factory
    factory = ThreadSafeFactory()
    # Register a handler
    class DataProcessor:
        def __init__(self, name):
            self.name = name
        def process(self, data):
            return f"{self.name} processed: {data}"
    factory.register("processor", DataProcessor, ["data"])
    # Use from multiple threads
    results = []
    def worker(thread_id):
        handler = factory.get_handler("processor", name=f"Worker-{thread_id}")
        result = handler.process(f"data-{thread_id}")
        results.append(result)
    # Create multiple threads
    threads = []
    for i in range(3):
        t = threading.Thread(target=worker, args=(i,))
        threads.append(t)
        t.start()
    # Wait for completion
    for t in threads:
        t.join()
    print("Thread results:")
    for result in results:
        print(f"  {result}")
if __name__ == "__main__":
    print("🚀 xSystem Quick Start Examples")
    print("=" * 60)
    # Run all demos
    demo_convenience_functions()
    demo_serialization()
    demo_security()
    demo_http_client()
    demo_performance_monitoring()
    demo_threading()
    print("\n✅ All demos completed!")
    print("\n📚 Next steps:")
    print("  - Explore the 24 serialization formats")
    print("  - Try enterprise features (schema registry, tracing)")
    print("  - Use async versions of operations")
    print("  - Check out the comprehensive documentation")
