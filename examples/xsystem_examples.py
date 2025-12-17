#!/usr/bin/env python3
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1.3
Generation Date: August 31, 2025

xSystem Utilities - Practical Usage Examples

This file demonstrates real-world usage patterns for the xwsystem utilities.
These examples show how to integrate multiple utilities for safe, robust operations.
"""

import json
import time
import logging
from pathlib import Path
from typing import Any

# Import xwsystem utilities
from exonware.xwsystem import (
    ThreadSafeFactory,
    PathValidator,
    AtomicFileWriter,
    CircularReferenceDetector,
    GenericHandlerFactory
)

def basic_usage_examples():
    """Basic usage examples for each utility."""
    
    print("=== xSystem Basic Usage Examples ===\n")
    
    # 1. ThreadSafeFactory Example
    print("1. ThreadSafeFactory - Thread-safe handler registration")
    factory = ThreadSafeFactory()
    factory.register("json", json.JSONEncoder, ["json"])
    factory.register("text", str, ["txt", "log"])
    
    print(f"Available formats: {factory.get_available_formats()}")
    print(f"JSON handler: {factory.get_handler('json')}")
    print()
    
    # 2. PathValidator Example
    print("2. PathValidator - Secure path validation")
    validator = PathValidator(base_path="./safe_zone", allow_absolute=False)
    
    try:
        safe_path = validator.validate_path("config/settings.json")
        print(f"Valid path: {safe_path}")
    except ValueError as e:
        print(f"Invalid path: {e}")
    
    try:
        # This should fail
        validator.validate_path("../../../etc/passwd")
    except ValueError as e:
        print(f"Security violation caught: {e}")
    print()
    
    # 3. AtomicFileWriter Example
    print("3. AtomicFileWriter - Safe file operations")
    test_file = Path("./temp_test.json")
    
    # Write data atomically
    test_data = {"message": "Hello from xSystem!", "timestamp": time.time()}
    with AtomicFileWriter(str(test_file)) as f:
        json.dump(test_data, f, indent=2)
    
    print(f"File written atomically: {test_file.exists()}")
    if test_file.exists():
        with open(test_file, 'r') as f:
            loaded_data = json.load(f)
        print(f"Content: {loaded_data['message']}")
        test_file.unlink()  # Cleanup
    print()
    
    # 4. CircularReferenceDetector Example
    print("4. CircularReferenceDetector - Detect circular references")
    detector = CircularReferenceDetector()
    
    # Safe data
    safe_data = {"level1": {"level2": {"value": "safe"}}}
    print(f"Safe data has circular refs: {detector.is_circular(safe_data)}")
    
    # Circular data
    circular_data = {"name": "root"}
    circular_data["self_ref"] = circular_data  # Create circular reference
    print(f"Circular data has circular refs: {detector.is_circular(circular_data)}")
    print()

def advanced_integration_example():
    """Advanced example showing integration of multiple utilities."""
    
    print("=== Advanced Integration Example ===\n")
    
    # Create a safe file processor using multiple utilities
    class SafeFileProcessor:
        def __init__(self, base_path: str):
            self.factory = GenericHandlerFactory(base_path=base_path, enable_security=True)
            self.detector = CircularReferenceDetector()
            
        def process_json_safely(self, file_path: str, data: dict[str, Any]) -> bool:
            """Process JSON data with full safety checks."""
            
            # 1. Check for circular references
            if self.detector.is_circular(data):
                print("❌ Circular reference detected in data")
                return False
            
            # 2. Use safe file operation through the factory
            def write_operation(validated_path: str):
                with AtomicFileWriter(validated_path) as f:
                    json.dump(data, f, indent=2)
                return True
            
            try:
                result = self.factory.safe_file_operation(
                    file_path, 
                    write_operation, 
                    for_writing=True
                )
                print(f"✅ Data written safely to: {file_path}")
                return result
            except Exception as e:
                print(f"❌ Operation failed: {e}")
                return False
    
    # Demo the integrated processor
    processor = SafeFileProcessor(base_path="./safe_zone")
    
    # Test with safe data
    safe_data = {
        "config": {
            "version": "1.0",
            "settings": {
                "debug": True,
                "max_connections": 100
            }
        },
        "timestamp": time.time()
    }
    
    print("Processing safe data:")
    success = processor.process_json_safely("config/app_settings.json", safe_data)
    print(f"Result: {success}\n")
    
    # Test with circular data
    circular_data = {"root": {}}
    circular_data["root"]["parent"] = circular_data  # Circular reference
    
    print("Processing circular data:")
    success = processor.process_json_safely("config/bad_settings.json", circular_data)
    print(f"Result: {success}")

def performance_monitoring_example():
    """Example showing performance monitoring capabilities."""
    
    print("\n=== Performance Monitoring Example ===\n")
    
    # Simulate some work with timing
    import time
    
    factory = ThreadSafeFactory()
    start_time = time.time()
    
    # Register multiple handlers
    for i in range(1000):
        factory.register(f"handler_{i}", str, [f"ext_{i}"])
    
    registration_time = time.time() - start_time
    print(f"Registered 1000 handlers in {registration_time:.4f} seconds")
    
    # Test retrieval performance
    start_time = time.time()
    for i in range(0, 1000, 10):  # Test every 10th handler
        handler = factory.get_handler(f"handler_{i}")
    
    retrieval_time = time.time() - start_time
    print(f"Retrieved 100 handlers in {retrieval_time:.4f} seconds")
    
    print(f"Total handlers available: {len(factory.get_available_formats())}")

if __name__ == "__main__":
    """Run all examples."""
    
    # Setup logging
    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
    
    try:
        # Run examples
        basic_usage_examples()
        advanced_integration_example()
        performance_monitoring_example()
        
        print("\n🎉 All examples completed successfully!")
        
    except Exception as e:
        print(f"\n❌ Example failed: {e}")
        import traceback
        traceback.print_exc()
