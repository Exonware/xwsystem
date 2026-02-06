#!/usr/bin/env python3
#exonware/xwsystem/examples/basic_examples.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1.3
Generation Date: August 31, 2025

Basic usage examples extracted from the original examples.py file.
"""

import json
import tempfile
from pathlib import Path

from exonware.xwsystem import (
    ThreadSafeFactory,
    PathValidator,
    AtomicFileWriter,
    CircularReferenceDetector,
    GenericHandlerFactory
)

def example_thread_safe_factory():
    """Example of ThreadSafeFactory usage."""
    print("=== ThreadSafeFactory Example ===")
    
    factory = ThreadSafeFactory()
    factory.register("json", json.JSONEncoder, ["json"])
    factory.register("text", str, ["txt"])
    
    print(f"Available formats: {factory.get_available_formats()}")
    print(f"JSON handler: {factory.get_handler('json')}")
    print()

def example_path_validator():
    """Example of PathValidator usage."""
    print("=== PathValidator Example ===")
    
    with tempfile.TemporaryDirectory() as temp_dir:
        validator = PathValidator(base_path=temp_dir)
        
        # Valid path
        try:
            safe_path = validator.validate_path("config/settings.json")
            print(f"✅ Valid path: {safe_path}")
        except ValueError as e:
            print(f"❌ Error: {e}")
        
        # Invalid path (directory traversal)
        try:
            validator.validate_path("../../etc/passwd")
        except ValueError as e:
            print(f"✅ Security violation caught: {e}")
    print()

def example_atomic_file_writer():
    """Example of AtomicFileWriter usage."""
    print("=== AtomicFileWriter Example ===")
    
    with tempfile.TemporaryDirectory() as temp_dir:
        test_file = Path(temp_dir) / "test.json"
        from exonware.xwsystem.version import __version__
        test_data = {"message": "Hello from xSystem!", "version": __version__}
        
        # Write data atomically
        with AtomicFileWriter(str(test_file)) as f:
            json.dump(test_data, f, indent=2)
        
        # Verify it was written
        if test_file.exists():
            with open(test_file, 'r') as f:
                loaded_data = json.load(f)
            print(f"✅ File written atomically: {loaded_data['message']}")
        else:
            print("❌ File was not created")
    print()

def example_circular_reference_detector():
    """Example of CircularReferenceDetector usage."""
    print("=== CircularReferenceDetector Example ===")
    
    detector = CircularReferenceDetector()
    
    # Safe data
    safe_data = {"level1": {"level2": {"value": "safe"}}}
    print(f"✅ Safe data has circular refs: {detector.is_circular(safe_data)}")
    
    # Circular data
    circular_data = {"name": "root"}
    circular_data["self_ref"] = circular_data
    print(f"✅ Circular data has circular refs: {detector.is_circular(circular_data)}")
    print()

def example_generic_handler_factory():
    """Example of GenericHandlerFactory usage."""
    print("=== GenericHandlerFactory Example ===")
    
    with tempfile.TemporaryDirectory() as temp_dir:
        factory = GenericHandlerFactory(base_path=temp_dir, enable_security=True)
        factory.register_safe("json", json.JSONEncoder, ["json"])
        
        print(f"✅ Handler registered: {factory.get_handler('json')}")
        
        # Safe file operation
        def dummy_operation(file_path):
            return f"Processed: {file_path}"
        
        result = factory.safe_file_operation("test.json", dummy_operation)
        print(f"✅ Safe operation result: {result}")
    print()

if __name__ == "__main__":
    """Run all basic examples."""
    print("🚀 xSystem Basic Examples\n")
    
    example_thread_safe_factory()
    example_path_validator()
    example_atomic_file_writer()
    example_circular_reference_detector()
    example_generic_handler_factory()
    
    print("🎉 All basic examples completed!")
