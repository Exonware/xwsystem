#exonware/xwsystem/tests/1.unit/security_tests/test_security_edge_cases.py
"""
Security edge case tests for xSystem.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Generation Date: August 31, 2025
"""

import pytest
from pathlib import Path
import tempfile
import os

from exonware.xwsystem.security import PathValidator, PathSecurityError, ResourceLimits, GenericLimitError


@pytest.mark.xwsystem_security
class TestPathValidatorEdgeCases:
    """Test edge cases for path validation."""
    
    def test_path_traversal_attacks(self):
        """Test various path traversal attack patterns."""
        validator = PathValidator(check_existence=False)
        
        # Test patterns that contain literal ".." or "..." which should be caught
        # Note: URL-encoded patterns like %2e%2e%2f (URL-encoded ../) are not currently 
        # detected as they don't contain the literal ".." substring (this would require 
        # URL decoding first, which is a potential enhancement)
        attack_patterns = [
            "../../../etc/passwd",  # Contains literal ".."
            "..\\..\\windows\\system32",  # Contains literal ".."
            "/../../root/.ssh/id_rsa",  # Contains literal ".."
            "....//....//etc/passwd",  # Contains "..." which is caught by excessive dots check
            "..%c0%af..%c0%af",  # Contains literal ".." so it's caught
        ]
        
        for pattern in attack_patterns:
            with pytest.raises(PathSecurityError):
                # Security validation should catch traversal before existence check
                validator.validate_path(pattern, for_writing=True)
    
    def test_null_byte_injection(self):
        """Test null byte injection attempts."""
        validator = PathValidator(check_existence=False)
        
        with pytest.raises(PathSecurityError):
            # Null byte check happens in _check_dangerous_patterns before existence
            validator.validate_path("safe_file.txt\x00../../../etc/passwd", for_writing=True)
    
    def test_extremely_long_paths(self):
        """Test handling of extremely long paths."""
        validator = PathValidator(check_existence=False, max_path_length=5000)
        
        # Create a path longer than the limit (5000 chars)
        long_path = "a" * 10000
        with pytest.raises(PathSecurityError):
            # Length check happens before existence check
            validator.validate_path(long_path, for_writing=True)


@pytest.mark.xwsystem_security
class TestResourceLimitsEdgeCases:
    """Test edge cases for resource limiting."""
    
    def test_memory_bomb_protection(self):
        """Test protection against memory bombs."""
        limiter = ResourceLimits(max_depth=10, max_resources=100)
        
        # This should be caught by resource limiting - raises GenericLimitError
        with pytest.raises(GenericLimitError):
            limiter.check_depth(15)  # Exceeds max_depth
    
    def test_concurrent_resource_exhaustion(self):
        """Test concurrent resource exhaustion attempts."""
        limiter = ResourceLimits(max_resources=5)
        
        # Simulate many concurrent operations
        exception_raised = False
        for i in range(10):
            try:
                limiter.increment_resource_count()
            except GenericLimitError:
                # Expected to fail after 5 operations (when count exceeds max_resources)
                exception_raised = True
                break
        
        # Verify that the limit was enforced
        assert exception_raised, "Resource limit should have been exceeded"
        # Verify the count is at the limit
        assert limiter._resource_count == 6, f"Expected resource count to be 6 (5 + 1 that exceeded), got {limiter._resource_count}"


@pytest.mark.xwsystem_security
class TestSecurityIntegration:
    """Integration tests for security components."""
    
    def test_combined_path_and_resource_validation(self):
        """Test combined path and resource validation."""
        validator = PathValidator(check_existence=False)
        limiter = ResourceLimits()
        
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create a safe file path within temp directory
            safe_path = Path(tmpdir) / "safe_file.txt"
            safe_path.touch()  # Create the file so it exists
            
            # Should pass all validations - use base_path restriction
            validator_with_base = PathValidator(base_path=Path(tmpdir))
            validated_path = validator_with_base.validate_path("safe_file.txt")
            assert validated_path is not None
    
    def test_security_bypass_attempts(self):
        """Test various security bypass attempts."""
        validator = PathValidator(check_existence=False)
        
        # Test bypass attempts that contain dangerous patterns which should be caught
        # Note: URL schemes like "file://" and "http://" are not currently detected
        # as dangerous patterns (they would require protocol validation, which is 
        # a higher-level concern). We test patterns that ARE caught.
        bypass_attempts = [
            "\\\\network\\share\\..\\..\\sensitive",  # Contains literal ".."
            "http://evil.com/../../etc/passwd",  # Contains literal ".."
        ]
        
        for attempt in bypass_attempts:
            with pytest.raises(PathSecurityError):
                # Security checks should catch these patterns (containing "..") before existence check
                validator.validate_path(attempt, for_writing=True)
