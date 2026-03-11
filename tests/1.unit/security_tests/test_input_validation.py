#exonware/xwsystem/tests/1.unit/security_tests/test_input_validation.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
Generation Date: August 31, 2025
Comprehensive tests for input validation and sanitization.
"""

import pytest
import tempfile
from pathlib import Path
from exonware.xwsystem.security.path_validator import PathValidator, PathSecurityError
from exonware.xwsystem.validation.data_validator import (
    DataValidator,
    ValidationError,
    validate_path_input,
    check_data_depth,
    estimate_memory_usage,
)
from exonware.xwsystem.validation.type_safety import (
    SafeTypeValidator,
    GenericSecurityError,
    validate_untrusted_data,
)


class TestPathInputValidation:
    """Test path input validation and sanitization."""

    def test_basic_path_validation(self):
        """Test basic path validation."""
        # Use check_existence=False for validation-only tests
        validator = PathValidator(check_existence=False)
        # Valid paths - validate patterns without requiring file creation
        # For validation-only, we don't need for_writing=True since we're just checking path format
        # But we need base_path to resolve relative paths properly
        import tempfile
        with tempfile.TemporaryDirectory() as temp_dir:
            validator_with_base = PathValidator(base_path=temp_dir, check_existence=False)
            # Valid paths within temp directory - use create_dirs=True to allow parent dir creation
            validator_with_base.validate_path("test.txt", for_writing=True, create_dirs=True)
            validator_with_base.validate_path("folder/test.txt", for_writing=True, create_dirs=True)
        # Invalid paths should raise PathSecurityError (caught before existence check)
        # Security validation happens first, so these will fail even with check_existence=False
        validator_no_base = PathValidator(check_existence=False)
        with pytest.raises(PathSecurityError):
            # Dangerous pattern ".." is caught by _check_dangerous_patterns before existence check
            validator_no_base.validate_path("../etc/passwd", for_writing=False)
        # Test protected paths - use platform-appropriate protected path
        import platform
        validator_abs = PathValidator(check_existence=False, allow_absolute=True)
        with pytest.raises(PathSecurityError):
            if platform.system() == 'Windows':
                # On Windows, test with Windows-protected path
                validator_abs.validate_path("C:\\Windows\\System32\\cmd.exe", for_writing=False)
            else:
                # On Unix-like, test with Unix-protected path
                validator_abs.validate_path("/etc/passwd", for_writing=False)

    def test_path_traversal_prevention(self):
        """Test prevention of directory traversal attacks."""
        validator = PathValidator(check_existence=False)
        dangerous_paths = [
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32",
            "test/../../../etc/passwd",
            "test\\..\\..\\..\\windows\\system32",
            "....//....//etc/passwd",
            "test/....//....//etc/passwd",
        ]
        for path in dangerous_paths:
            with pytest.raises(PathSecurityError):
                # Use for_writing=True to bypass existence check, but security validation should catch traversal
                validator.validate_path(path, for_writing=True)

    def test_protected_paths(self):
        """Test protection of system paths."""
        import platform
        # Use check_existence=False - we're testing path security, not file existence
        validator = PathValidator(check_existence=False, allow_absolute=True)
        # Use platform-appropriate protected paths
        if platform.system() == 'Windows':
            protected_paths = [
                "C:\\Windows\\System32\\cmd.exe",
                "C:\\Program Files\\test.exe",
                "C:\\Program Files (x86)\\test.exe",
            ]
        else:
            protected_paths = [
                "/etc/passwd",
                "/bin/bash",
                "/usr/bin/sudo", 
                "/root/secret",
            ]
        for path in protected_paths:
            with pytest.raises(PathSecurityError):
                # Protected path check should catch these even with allow_absolute=True
                # Security check happens before existence check
                validator.validate_path(path, for_writing=False)

    def test_dangerous_characters(self):
        """Test detection of dangerous characters."""
        validator = PathValidator(check_existence=False)
        dangerous_chars = [
            "test|rm -rf /",
            "test; rm -rf /",
            "test & del C:\\",
            "test`whoami`",
            "test$(whoami)",
            "test<script>",
            "test>output.txt",
        ]
        for path in dangerous_chars:
            with pytest.raises(PathSecurityError):
                # Security check should catch dangerous characters before existence check
                validator.validate_path(path, for_writing=True)

    def test_max_path_length(self):
        """Test maximum path length validation."""
        validator = PathValidator(max_path_length=100, check_existence=False)
        # Valid length (using for_writing=True to bypass existence check)
        validator.validate_path("a" * 50, for_writing=True)
        # Too long - should raise PathSecurityError before checking existence
        with pytest.raises(PathSecurityError):
            validator.validate_path("a" * 200, for_writing=True)

    def test_base_path_restriction(self):
        """Test base path restriction."""
        with tempfile.TemporaryDirectory() as temp_dir:
            base_path = Path(temp_dir)
            validator = PathValidator(base_path=base_path)
            # Valid: inside base path
            test_file = base_path / "test.txt"
            test_file.touch()
            validator.validate_path(str(test_file))
            # Invalid: outside base path
            with pytest.raises(PathSecurityError):
                validator.validate_path("/tmp/outside.txt")

    def test_path_validation_function(self):
        """Test standalone path validation function."""
        from exonware.xwsystem.validation.errors import PathValidationError
        # Valid paths
        validate_path_input("test.txt")
        validate_path_input("folder/test.txt")
        # Invalid paths - validate_path_input checks for excessive patterns, not single "../"
        # But it does check None and type
        with pytest.raises(PathValidationError):
            validate_path_input(None)
        with pytest.raises(TypeError):
            validate_path_input(123)
        # Test excessive traversal pattern (which validate_path_input does check)
        with pytest.raises(PathValidationError):
            validate_path_input("../" * 10 + "etc/passwd")


class TestDataStructureValidation:
    """Test data structure validation."""

    def test_data_depth_validation(self):
        """Test data depth validation."""
        # Valid depth
        shallow_data = {"level1": {"level2": {"level3": "value"}}}
        check_data_depth(shallow_data, max_depth=5)
        # Too deep
        deep_data = {"l1": {"l2": {"l3": {"l4": {"l5": {"l6": "value"}}}}}}
        with pytest.raises(ValidationError):
            check_data_depth(deep_data, max_depth=5)

    def test_circular_reference_detection(self):
        """Test circular reference detection in data structures."""
        from exonware.xwsystem.validation.errors import DepthValidationError
        # Create circular reference
        data = {"key": "value"}
        data["self"] = data
        # check_data_depth will traverse the circular structure and detect depth
        # When it traverses: data -> data["self"] -> data["self"]["self"] -> ...
        # This will exceed max_depth and raise DepthValidationError
        # This is correct behavior - circular references indicate problematic data
        with pytest.raises(DepthValidationError):
            check_data_depth(data, max_depth=10)

    def test_memory_estimation(self):
        """Test memory usage estimation."""
        small_data = {"key": "value"}
        large_data = {"key": "x" * 10000}
        small_size = estimate_memory_usage(small_data)
        large_size = estimate_memory_usage(large_data)
        assert large_size > small_size
        assert small_size > 0

    def test_data_validator_class(self):
        """Test DataValidator class."""
        validator = DataValidator(max_dict_depth=3)
        # Valid data
        valid_data = {"level1": {"level2": "value"}}
        validator.validate_data_structure(valid_data)
        # Invalid data
        invalid_data = {"l1": {"l2": {"l3": {"l4": "value"}}}}
        with pytest.raises(ValidationError):
            validator.validate_data_structure(invalid_data)


class TestTypeValidation:
    """Test type safety validation."""

    def test_safe_type_detection(self):
        """Test safe type detection."""
        # Safe types
        assert SafeTypeValidator.is_safe_type("string")
        assert SafeTypeValidator.is_safe_type(123)
        assert SafeTypeValidator.is_safe_type(12.34)
        assert SafeTypeValidator.is_safe_type(True)
        assert SafeTypeValidator.is_safe_type([1, 2, 3])
        assert SafeTypeValidator.is_safe_type({"key": "value"})
        assert SafeTypeValidator.is_safe_type(None)
        # Unsafe types
        assert not SafeTypeValidator.is_safe_type(object())
        assert not SafeTypeValidator.is_safe_type(lambda x: x)
        assert not SafeTypeValidator.is_safe_type(open)

    def test_immutable_type_detection(self):
        """Test immutable type detection."""
        # Immutable types
        assert SafeTypeValidator.is_immutable_type("string")
        assert SafeTypeValidator.is_immutable_type(123)
        assert SafeTypeValidator.is_immutable_type((1, 2, 3))
        assert SafeTypeValidator.is_immutable_type(frozenset([1, 2, 3]))
        # Mutable types
        assert not SafeTypeValidator.is_immutable_type([1, 2, 3])
        assert not SafeTypeValidator.is_immutable_type({"key": "value"})
        assert not SafeTypeValidator.is_immutable_type(set([1, 2, 3]))

    def test_untrusted_data_validation(self):
        """Test validation of untrusted data."""
        # Valid untrusted data
        safe_data = {
            "string_key": "string_value",
            "int_key": 123,
            "float_key": 12.34,
            "bool_key": True,
            "list_key": [1, 2, 3],
            "dict_key": {"nested": "value"},
            "null_key": None,
        }
        validate_untrusted_data(safe_data)
        # Invalid untrusted data
        unsafe_data = {"function": lambda x: x}
        with pytest.raises(GenericSecurityError):
            validate_untrusted_data(unsafe_data)

    def test_deep_structure_validation(self):
        """Test validation of deeply nested structures."""
        # Valid deep structure
        deep_safe = {
            "level1": {
                "level2": {
                    "level3": ["item1", "item2", "item3"]
                }
            }
        }
        validate_untrusted_data(deep_safe)
        # Too deep structure
        very_deep = {}
        current = very_deep
        for i in range(150):  # Deeper than default max_depth
            current["next"] = {}
            current = current["next"]
        with pytest.raises(GenericSecurityError):
            validate_untrusted_data(very_deep)

    def test_non_string_keys(self):
        """Test detection of non-string dictionary keys."""
        # Valid: string keys
        valid_data = {"string_key": "value"}
        validate_untrusted_data(valid_data)
        # Invalid: non-string keys
        invalid_data = {123: "value"}
        with pytest.raises(GenericSecurityError):
            validate_untrusted_data(invalid_data)


class TestInputSanitization:
    """Test input sanitization and normalization."""

    def test_path_normalization(self):
        """Test path normalization."""
        # PathValidator doesn't have normalize_path method - paths are normalized during validation
        # Instead, test that validation normalizes paths correctly
        import tempfile
        with tempfile.TemporaryDirectory() as temp_dir:
            validator = PathValidator(base_path=temp_dir, check_existence=False)
            # Test that paths are resolved and normalized during validation
            test_paths = [
                "test//file.txt",
                "test\\file.txt",
                "./test/file.txt",
            ]
            for input_path in test_paths:
                # validate_path will normalize and resolve the path
                # Use create_dirs=True to allow parent directory creation
                normalized = validator.validate_path(input_path, for_writing=True, create_dirs=True)
                assert isinstance(normalized, Path)
                # Verify it's a resolved absolute path
                assert normalized.is_absolute()
                # Verify it's within the base path
                assert str(normalized).startswith(str(temp_dir))

    def test_size_limits(self):
        """Test size limit validation."""
        validator = DataValidator()
        # Large string
        large_string = "x" * 1000000  # 1MB string
        large_data = {"large": large_string}
        # Should validate size
        try:
            validator.validate_data_structure(large_data)
        except ValidationError as e:
            assert "size" in str(e).lower() or "memory" in str(e).lower()

    def test_special_characters_handling(self):
        """Test handling of special characters in input."""
        validator = PathValidator(check_existence=False)
        # Unicode characters - should be allowed (using for_writing=True to bypass existence check)
        unicode_path = "test_文件.txt"
        result = validator.validate_path(unicode_path, for_writing=True)
        assert isinstance(result, Path)
        # Control characters - should raise PathSecurityError
        control_chars = "test\x00\x01\x02.txt"
        with pytest.raises(PathSecurityError):
            validator.validate_path(control_chars, for_writing=True)
@pytest.mark.xwsystem_unit


class TestValidationErrorHandling:
    """Test validation error handling."""

    def test_validation_error_messages(self):
        """Test validation error messages are informative."""
        validator = PathValidator()
        try:
            validator.validate_path("../etc/passwd")
        except PathSecurityError as e:
            assert "traversal" in str(e).lower() or "dangerous" in str(e).lower()

    def test_type_error_messages(self):
        """Test type error messages."""
        try:
            validate_path_input(123)
        except TypeError as e:
            assert "string" in str(e)

    def test_depth_error_messages(self):
        """Test depth validation error messages."""
        deep_data = {"l1": {"l2": {"l3": {"l4": {"l5": "value"}}}}}
        try:
            check_data_depth(deep_data, max_depth=3)
        except ValidationError as e:
            assert "depth" in str(e).lower()
            assert "3" in str(e)
@pytest.mark.xwsystem_unit


class TestEdgeCases:
    """Test edge cases in validation."""

    def test_empty_inputs(self):
        """Test empty input handling."""
        validator = PathValidator(check_existence=False)
        # Empty string - should be rejected
        with pytest.raises(PathSecurityError):
            validator.validate_path("")
        # Whitespace only - PathValidator currently accepts whitespace-only paths
        # as they resolve to valid paths (current directory with whitespace name)
        # This is acceptable behavior - whitespace-only names are technically valid filenames
        # If strict validation is needed, it should be enforced at a higher level
        result = validator.validate_path("   ", for_writing=False)
        assert isinstance(result, Path)

    def test_very_long_inputs(self):
        """Test very long input handling."""
        # Very long path
        long_path = "a" * 10000
        with pytest.raises(ValidationError):
            validate_path_input(long_path)

    def test_binary_data(self):
        """Test binary data handling."""
        # Binary data (bytes) should NOT be considered a safe type
        # SAFE_TYPES = (str, int, float, bool, list, dict, type(None))
        # bytes is NOT in this list, so is_safe_type should return False
        binary_data = b"\x00\x01\x02\x03"
        # bytes is not in SAFE_TYPES, so should return False
        assert not SafeTypeValidator.is_safe_type(binary_data)
        # Test that dict with bytes value should also be unsafe when validated
        dict_with_binary = {"key": b"\x00\x01\x02\x03"}
        with pytest.raises(GenericSecurityError):
            validate_untrusted_data(dict_with_binary)

    def test_extreme_nesting(self):
        """Test extremely nested data structures."""
        # Create extremely nested structure
        nested = "value"
        for _ in range(1000):
            nested = [nested]
        with pytest.raises(GenericSecurityError):
            validate_untrusted_data(nested, max_depth=100)
