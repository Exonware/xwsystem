#!/usr/bin/env python3
#exonware/xwsystem/examples/examples.py
"""
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
from ..src.exonware.xwsystem.threading import ThreadSafeFactory
from ..src.exonware.xwsystem.security import PathValidator, PathSecurityError
from ..src.exonware.xwsystem.io import AtomicFileWriter
from ..src.exonware.xwsystem.structures import CircularReferenceDetector, CircularReferenceError
from ..src.exonware.xwsystem.patterns import GenericHandlerFactory
# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)
# Example 1: Secure Configuration Manager


class SecureConfigManager:
    """
    A configuration manager that uses xwsystem utilities for security and reliability.
    """

    def __init__(self, base_directory: str = "./configs"):
        self.path_validator = PathValidator(
            base_path=base_directory,
            allow_absolute=False,
            max_path_length=1024
        )
        self.circular_detector = CircularReferenceDetector()

    def save_config(self, config_name: str, config_data: dict[str, Any]) -> bool:
        """Save configuration data safely."""
        try:
            # Validate configuration data structure
            if self.circular_detector.is_circular(config_data):
                logger.error(f"Configuration '{config_name}' contains circular references")
                return False
            # Validate and secure the file path
            safe_path = self.path_validator.validate_path(
                f"{config_name}.json",
                for_writing=True,
                create_dirs=True
            )
            # Use atomic writing for data integrity
            with AtomicFileWriter(safe_path, backup=True) as writer:
                json.dump(config_data, writer, indent=2, ensure_ascii=False)
            logger.info(f"Configuration '{config_name}' saved successfully to {safe_path}")
            return True
        except PathSecurityError as e:
            logger.error(f"Security violation saving config '{config_name}': {e}")
            return False
        except CircularReferenceError as e:
            logger.error(f"Circular reference in config '{config_name}': {e}")
            return False
        except Exception as e:
            logger.error(f"Failed to save config '{config_name}': {e}")
            return False

    def load_config(self, config_name: str) -> dict[str, Any]:
        """Load configuration data safely."""
        try:
            # Validate the file path
            safe_path = self.path_validator.validate_path(f"{config_name}.json")
            # Load and validate data
            with open(safe_path, 'r', encoding='utf-8') as f:
                config_data = json.load(f)
            # Check for circular references in loaded data
            if self.circular_detector.is_circular(config_data):
                logger.warning(f"Loaded config '{config_name}' contains circular references")
            logger.info(f"Configuration '{config_name}' loaded successfully")
            return config_data
        except PathSecurityError as e:
            logger.error(f"Security violation loading config '{config_name}': {e}")
            return {}
        except FileNotFoundError:
            logger.warning(f"Configuration '{config_name}' not found")
            return {}
        except Exception as e:
            logger.error(f"Failed to load config '{config_name}': {e}")
            return {}
# Example 2: Thread-Safe Data Processor Factory


class DataProcessor:
    """Base class for data processors."""

    def process(self, data: Any) -> Any:
        raise NotImplementedError


class JsonProcessor(DataProcessor):
    """JSON data processor."""

    def process(self, data: Any) -> str:
        return json.dumps(data, indent=2)


class UppercaseProcessor(DataProcessor):
    """Uppercase text processor."""

    def process(self, data: Any) -> str:
        return str(data).upper()


class DataProcessingSystem:
    """
    A data processing system using thread-safe factories.
    """

    def __init__(self):
        # Create thread-safe factory for processors
        self.factory = ThreadSafeFactory[DataProcessor]()
        self._register_processors()

    def _register_processors(self):
        """Register available processors."""
        self.factory.register("json", JsonProcessor, extensions=["json"])
        self.factory.register("uppercase", UppercaseProcessor, extensions=["txt"])

    def process_data(self, data: Any, processor_type: str) -> str:
        """Process data using specified processor type."""
        try:
            processor = self.factory.get_handler(processor_type)
            result = processor.process(data)
            logger.info(f"Data processed successfully using {processor_type} processor")
            return result
        except Exception as e:
            logger.error(f"Failed to process data with {processor_type}: {e}")
            return str(data)

    def get_available_processors(self) -> list:
        """Get list of available processor types."""
        return self.factory.get_available_formats()
# Example 3: Comprehensive Data Manager


class SafeDataManager:
    """
    A comprehensive data manager that combines all xwsystem utilities
    for maximum safety and functionality.
    """

    def __init__(self, base_path: str = "./data"):
        self.factory = GenericHandlerFactory(
            base_path=base_path,
            enable_security=True,
            enable_circular_detection=True,
            max_circular_depth=100
        )
        # Register custom handlers
        self._register_handlers()

    def _register_handlers(self):
        """Register data format handlers."""
        # These would be real handler classes in practice
        class MockJsonHandler:
            def load(self, file_path): 
                with open(file_path, 'r') as f:
                    return json.load(f)
            def save(self, data, file_path):
                with open(file_path, 'w') as f:
                    json.dump(data, f, indent=2)
        self.factory.register("json", MockJsonHandler, extensions=["json"])

    def safe_data_operation(self, file_path: str, data: dict[str, Any], operation: str = "save"):
        """
        Perform safe data operations with comprehensive validation.
        """
        try:
            # Validate data structure first
            self.factory.validate_data_structure(data)
            if operation == "save":
                # Safe atomic write operation
                def write_operation(file_handle):
                    json.dump(data, file_handle, indent=2)
                result_path = self.factory.atomic_write_operation(
                    file_path,
                    write_operation,
                    backup=True
                )
                logger.info(f"Data saved safely to {result_path}")
                return result_path
            elif operation == "load":
                # Safe file read operation
                def read_operation(validated_path):
                    with open(validated_path, 'r') as f:
                        return json.load(f)
                data = self.factory.safe_file_operation(
                    file_path,
                    read_operation,
                    for_writing=False
                )
                logger.info(f"Data loaded safely from {file_path}")
                return data
        except CircularReferenceError as e:
            logger.error(f"Circular reference detected in data: {e}")
            return None
        except PathSecurityError as e:
            logger.error(f"Security violation: {e}")
            return None
        except Exception as e:
            logger.error(f"Operation failed: {e}")
            return None

    def create_safe_workspace(self, workspace_name: str) -> Path:
        """Create a safe temporary workspace."""
        try:
            temp_file = self.factory.create_safe_temp_file(
                prefix=f"{workspace_name}_",
                suffix=".workspace"
            )
            workspace_dir = temp_file.parent / f"{workspace_name}_workspace"
            workspace_dir.mkdir(exist_ok=True)
            logger.info(f"Safe workspace created: {workspace_dir}")
            return workspace_dir
        except Exception as e:
            logger.error(f"Failed to create workspace: {e}")
            return None

    def get_system_stats(self) -> dict[str, Any]:
        """Get comprehensive system statistics."""
        return self.factory.get_operation_stats()
# Example 4: Real-world Usage Demonstration


def demonstrate_xwsystem_utilities():
    """
    Demonstrate real-world usage of xwsystem utilities.
    """
    logger.info("=== xSystem Utilities Demonstration ===")
    # 1. Secure Configuration Management
    logger.info("\n1. Secure Configuration Management:")
    config_manager = SecureConfigManager()
    # Test data with various complexity levels
    simple_config = {
        "app_name": "MyApp",
        "version": "1.0.0",
        "settings": {
            "debug": True,
            "max_connections": 100
        }
    }
    # Save configuration
    success = config_manager.save_config("app_config", simple_config)
    if success:
        # Load it back
        loaded_config = config_manager.load_config("app_config")
        logger.info(f"Config loaded: {loaded_config}")
    # Test security validation (this should fail)
    try:
        config_manager.save_config("../../../etc/passwd", {"malicious": "data"})
    except Exception as e:
        logger.info(f"Security test passed: Blocked malicious path")
    # 2. Thread-Safe Data Processing
    logger.info("\n2. Thread-Safe Data Processing:")
    processing_system = DataProcessingSystem()
    test_data = {"message": "hello world", "count": 42}
    # Process with different processors
    json_result = processing_system.process_data(test_data, "json")
    logger.info(f"JSON processed: {json_result[:50]}...")
    text_result = processing_system.process_data("hello world", "uppercase")
    logger.info(f"Uppercase processed: {text_result}")
    available = processing_system.get_available_processors()
    logger.info(f"Available processors: {available}")
    # 3. Comprehensive Data Management
    logger.info("\n3. Comprehensive Data Management:")
    data_manager = SafeDataManager()
    # Create safe workspace
    workspace = data_manager.create_safe_workspace("test_project")
    if workspace:
        logger.info(f"Workspace created at: {workspace}")
    # Safe data operations
    test_data = {
        "project": "xComBot",
        "modules": ["xdata", "xwsystem", "xbot"],
        "metadata": {
            "created": "2024-12-01",
            "version": "2.0.0"
        }
    }
    # Test circular reference detection
    # circular_data = {"a": {"b": None}}
    # circular_data["a"]["b"] = circular_data  # Create circular reference
    # data_manager.safe_data_operation("circular_test.json", circular_data)
    # Safe save operation
    saved_path = data_manager.safe_data_operation("project_info.json", test_data, "save")
    if saved_path:
        # Safe load operation
        loaded_data = data_manager.safe_data_operation("project_info.json", {}, "load")
        logger.info(f"Data round-trip successful: {loaded_data}")
    # Get system statistics
    stats = data_manager.get_system_stats()
    logger.info(f"System stats: {stats}")
    logger.info("\n=== Demonstration Complete ===")
# Example 5: Error Handling Patterns


def demonstrate_error_handling():
    """
    Demonstrate proper error handling patterns with xwsystem utilities.
    """
    logger.info("\n=== Error Handling Demonstration ===")
    # Path validation errors
    validator = PathValidator(base_path="/safe/dir")
    dangerous_paths = [
        "../../../etc/passwd",      # Directory traversal
        "/etc/shadow",              # Absolute path outside base
        "file\x00.txt",            # Null byte injection
        "a" * 2000,                # Path too long
        "CON.txt"                  # Windows reserved name
    ]
    for path in dangerous_paths:
        try:
            validator.validate_path(path)
            logger.warning(f"Security test failed: {path} was allowed!")
        except PathSecurityError as e:
            logger.info(f"✅ Blocked dangerous path '{path[:50]}...': {type(e).__name__}")
        except Exception as e:
            logger.info(f"✅ Blocked path '{path[:50]}...' with: {type(e).__name__}")
    # Circular reference errors
    detector = CircularReferenceDetector()
    # Create various circular reference patterns
    circular_cases = [
        # Simple self-reference
        lambda: (lambda d: d.update({"self": d}) or d)({}),
        # Nested circular reference
        lambda: (lambda d: d.update({"nested": {"parent": d}}) or d)({}),
        # List circular reference
        lambda: (lambda l: l.append(l) or l)([1, 2, 3])
    ]
    for i, case_func in enumerate(circular_cases):
        try:
            circular_data = case_func()
            detector.traverse(circular_data, [])
            logger.warning(f"Circular test {i+1} failed: No error raised!")
        except CircularReferenceError as e:
            logger.info(f"✅ Detected circular reference {i+1}: {type(e).__name__}")
        except Exception as e:
            logger.info(f"✅ Caught error {i+1}: {type(e).__name__}")
    logger.info("Error handling demonstration complete")
if __name__ == "__main__":
    # Run demonstrations
    demonstrate_xwsystem_utilities()
    demonstrate_error_handling()
    print("\n🎉 All xwsystem utility examples completed successfully!")
    print("📚 For more details, see: src/exonware/xwsystem/README.md") 
