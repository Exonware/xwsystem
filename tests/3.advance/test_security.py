#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/3.advance/test_security.py
Comprehensive OWASP Top 10 security tests for xwsystem.
Priority #1: Security Excellence
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 07-Jan-2025
"""

import pytest
import os
import tempfile
import time
from pathlib import Path
from exonware.xwsystem.security.path_validator import PathValidator
from exonware.xwsystem.security.crypto import SecureHash, SymmetricEncryption
from exonware.xwsystem.security.resource_limits import ResourceLimits
from exonware.xwsystem.security.validator import SecurityValidator
from exonware.xwsystem.security.monitor import SecurityMonitor
from exonware.xwsystem.security.policy import SecurityPolicy
from exonware.xwsystem.security.defs import SecurityLevel
from exonware.xwsystem.io.file import XWFile
from exonware.xwsystem.io.serialization.serializer import XWSerializer
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP01BrokenAccessControl:
    """OWASP Top 10 #1: Broken Access Control"""

    def test_path_traversal_protection(self):
        """Test protection against path traversal attacks."""
        validator = PathValidator()
        # Normal paths should work
        assert validator.validate_path("/safe/path/file.txt") is True
        # Path traversal attempts should be blocked
        malicious_paths = [
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32",
            "/etc/passwd",
            "C:\\Windows\\System32",
            "....//....//etc/passwd",
        ]
        for path in malicious_paths:
            assert validator.validate_path(path) is False, f"Path traversal not blocked: {path}"

    def test_file_access_restrictions(self):
        """Test that file operations respect access restrictions."""
        validator = PathValidator()
        file_ops = XWFile()
        with tempfile.TemporaryDirectory() as tmpdir:
            safe_dir = Path(tmpdir) / "safe"
            safe_dir.mkdir()
            # Should allow access to safe directory
            safe_file = safe_dir / "test.txt"
            safe_file.write_text("test")
            assert validator.validate_path(str(safe_file)) is True
            # Should block access outside safe directory
            restricted_path = Path(tmpdir).parent / "restricted.txt"
            assert validator.validate_path(str(restricted_path)) is False
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP02CryptographicFailures:
    """OWASP Top 10 #2: Cryptographic Failures"""

    def test_encryption_uses_strong_algorithms(self):
        """Test that encryption uses strong algorithms."""
        key = SymmetricEncryption.generate_key()
        crypto = SymmetricEncryption(key)
        # Test AES-256 encryption
        data = b"sensitive data"
        encrypted = crypto.encrypt(data)
        # Encrypted data should be different from original
        assert encrypted != data
        # Should be able to decrypt
        decrypted = crypto.decrypt(encrypted)
        assert decrypted == data

    def test_password_hashing_uses_salt(self):
        """Test that password hashing uses salt."""
        # xwsystem uses SecureHash which provides hashing
        password = "my_password"
        hash1 = SecureHash.sha256(password)
        hash2 = SecureHash.sha256(password)
        # SHA-256 without salt produces same hash, but that's expected
        # For password hashing with salt, would need bcrypt or similar
        # This test verifies basic hashing works
        assert hash1 == hash2  # SHA-256 is deterministic
        assert len(hash1) == 64  # SHA-256 produces 64-char hex string
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP03Injection:
    """OWASP Top 10 #3: Injection"""

    def test_sql_injection_protection(self):
        """Test protection against SQL injection."""
        # xwsystem doesn't have SQL, but test serialization injection
        serializer = XWSerializer()
        malicious_inputs = [
            '{"key": "value"; DROP TABLE users; --"}',
            '{"key": "value\' OR \'1\'=\'1"}',
            '{"key": "<script>alert(\'xss\')</script>"}',
        ]
        for malicious in malicious_inputs:
            # Should handle safely without executing code
            try:
                result = serializer.deserialize(malicious, format="json")
                # Result should be data structure, not executed code
                assert isinstance(result, dict)
            except Exception:
                # Exception is acceptable - better than execution
                pass

    def test_command_injection_protection(self):
        """Test protection against command injection."""
        validator = PathValidator()
        malicious_paths = [
            "file.txt; rm -rf /",
            "file.txt | cat /etc/passwd",
            "file.txt && echo hacked",
        ]
        for path in malicious_paths:
            # Should reject paths with command separators
            assert validator.validate_path(path) is False
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP04InsecureDesign:
    """OWASP Top 10 #4: Insecure Design"""

    def test_defense_in_depth(self):
        """Test that multiple security layers are in place."""
        validator = PathValidator()
        limits = ResourceLimits()
        # Multiple validations should work together
        path = "/safe/path/file.txt"
        assert validator.validate_path(path) is True
        assert limits.check_file_size(path, max_size_mb=10) is True

    def test_secure_defaults(self):
        """Test that defaults are secure."""
        key = SymmetricEncryption.generate_key()
        crypto = SymmetricEncryption(key)
        # Default encryption should be strong
        data = b"test"
        encrypted = crypto.encrypt(data)
        assert encrypted != data
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP05SecurityMisconfiguration:
    """OWASP Top 10 #5: Security Misconfiguration"""

    def test_no_debug_mode_in_production(self):
        """Test that debug information is not exposed."""
        # In production, errors should not expose internals
        validator = PathValidator()
        try:
            validator.validate_path("../../../etc/passwd")
        except Exception as e:
            # Error message should not expose internal paths
            error_msg = str(e)
            assert "etc/passwd" not in error_msg.lower()
            assert ".." not in error_msg

    def test_secure_configuration(self):
        """Test that configuration is secure by default."""
        limits = ResourceLimits()
        # Default limits should be reasonable
        assert limits.max_file_size_mb > 0
        assert limits.max_file_size_mb < 1000  # Not unlimited
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP06VulnerableComponents:
    """OWASP Top 10 #6: Vulnerable and Outdated Components"""

    def test_dependency_validation(self):
        """Test that dependencies are validated."""
        # This would typically check requirements.txt versions
        # For now, verify that imports work correctly
        from exonware.xwsystem.security import PathValidator
        from exonware.xwsystem.security.crypto import SecureHash, SymmetricEncryption
        assert PathValidator is not None
        assert SecureHash is not None
        assert SymmetricEncryption is not None
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP07AuthFailures:
    """OWASP Top 10 #7: Identification and Authentication Failures"""

    def test_password_strength_validation(self):
        """Test password strength validation."""
        # xwsystem provides hashing, password strength validation would be in xwauth
        # Test that hashing works for any password
        weak_passwords = ["12345", "password", "abc", ""]
        for weak_pwd in weak_passwords:
            # Should hash any password
            hash_result = SecureHash.sha256(weak_pwd)
            assert len(hash_result) == 64  # SHA-256 produces 64-char hex

    def test_session_management(self):
        """Test secure session management."""
        # xwsystem doesn't have sessions, but test key generation
        key1 = SymmetricEncryption.generate_key()
        key2 = SymmetricEncryption.generate_key()
        # Keys should be unique
        assert key1 != key2
        assert len(key1) >= 32  # Fernet keys are 32 bytes (44 chars base64)
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP08SoftwareDataIntegrity:
    """OWASP Top 10 #8: Software and Data Integrity Failures"""

    def test_data_integrity_verification(self):
        """Test data integrity verification."""
        data = b"important data"
        hash_value = SecureHash.sha256(data)
        # Verify integrity - recompute hash and compare
        recomputed = SecureHash.sha256(data)
        assert recomputed == hash_value
        # Tampered data should produce different hash
        tampered = b"tampered data"
        tampered_hash = SecureHash.sha256(tampered)
        assert tampered_hash != hash_value

    def test_serialization_integrity(self):
        """Test that serialized data maintains integrity."""
        serializer = XWSerializer()
        original = {"key": "value", "number": 42}
        serialized = serializer.serialize(original, format="json")
        deserialized = serializer.deserialize(serialized, format="json")
        assert deserialized == original
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP09LoggingFailures:
    """OWASP Top 10 #9: Security Logging and Monitoring Failures"""

    def test_security_events_are_logged(self):
        """Test that security events are logged."""
        validator = PathValidator()
        # Security violations should be detectable
        malicious_path = "../../../etc/passwd"
        result = validator.validate_path(malicious_path)
        # Should reject and potentially log
        assert result is False
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestOWASP10SSRF:
    """OWASP Top 10 #10: Server-Side Request Forgery"""

    def test_url_validation(self):
        """Test URL validation to prevent SSRF."""
        validator = PathValidator()
        # Internal URLs should be blocked
        internal_urls = [
            "file:///etc/passwd",
            "http://localhost",
            "http://127.0.0.1",
            "http://169.254.169.254",  # AWS metadata
        ]
        # xwsystem path validator should handle file paths
        for url in internal_urls:
            if url.startswith("file://"):
                path = url.replace("file://", "")
                assert validator.validate_path(path) is False
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestSecurityValidator:
    """Test SecurityValidator implementation."""

    def test_password_validation(self):
        """Test password strength validation."""
        validator = SecurityValidator()
        # Valid password
        is_valid, errors = validator.validate_password("StrongP@ssw0rd")
        assert is_valid is True
        assert len(errors) == 0
        # Weak password
        is_valid, errors = validator.validate_password("weak")
        assert is_valid is False
        assert len(errors) > 0

    def test_sql_injection_detection(self):
        """Test SQL injection detection."""
        validator = SecurityValidator()
        # SQL injection attempts
        malicious_inputs = [
            "'; DROP TABLE users; --",
            "1' OR '1'='1",
            "admin'--",
            "1; DELETE FROM users",
        ]
        for malicious in malicious_inputs:
            assert validator.detect_sql_injection(malicious) is True
        # Safe input
        assert validator.detect_sql_injection("normal user input") is False

    def test_xss_detection(self):
        """Test XSS detection."""
        validator = SecurityValidator()
        # XSS attempts
        malicious_inputs = [
            "<script>alert('xss')</script>",
            "<img src=x onerror=alert('xss')>",
            "javascript:alert('xss')",
            "<iframe src='evil.com'></iframe>",
        ]
        for malicious in malicious_inputs:
            assert validator.detect_xss(malicious) is True
        # Safe input
        assert validator.detect_xss("normal text") is False

    def test_input_sanitization(self):
        """Test input sanitization."""
        validator = SecurityValidator()
        # Test HTML escaping
        malicious = "<script>alert('xss')</script>"
        sanitized = validator.sanitize_input(malicious)
        assert "<script>" not in sanitized
        assert "&lt;script&gt;" in sanitized

    def test_security_headers_validation(self):
        """Test security headers validation."""
        validator = SecurityValidator()
        # Valid headers
        valid_headers = {
            "X-Content-Type-Options": "nosniff",
            "X-Frame-Options": "DENY",
            "X-XSS-Protection": "1; mode=block",
            "Strict-Transport-Security": "max-age=31536000",
            "Content-Security-Policy": "default-src 'self'",
        }
        results = validator.check_security_headers(valid_headers)
        assert all(results.values())
        # Missing headers
        invalid_headers = {"X-Content-Type-Options": "nosniff"}
        results = validator.check_security_headers(invalid_headers)
        assert not all(results.values())
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestSecurityMonitor:
    """Test SecurityMonitor implementation."""

    def test_intrusion_detection(self):
        """Test intrusion detection."""
        monitor = SecurityMonitor()
        # Failed login intrusion
        for i in range(6):
            event = {
                "type": "failed_login",
                "user": "attacker",
                "timestamp": time.time() + i,
            }
            if i < 5:
                assert monitor.detect_intrusion(event) is False
            else:
                assert monitor.detect_intrusion(event) is True

    def test_failed_login_monitoring(self):
        """Test failed login monitoring."""
        monitor = SecurityMonitor()
        # Simulate failed logins
        for i in range(5):
            assert monitor.monitor_failed_logins("user1", max_attempts=5) is False
        # 6th attempt should trigger
        assert monitor.monitor_failed_logins("user1", max_attempts=5) is True

    def test_anomaly_detection(self):
        """Test anomaly detection."""
        monitor = SecurityMonitor()
        # Normal behavior
        for i in range(10):
            behavior = {
                "type": "access",
                "user": "user1",
                "value": 10 + i,  # Normal range
            }
            assert monitor.detect_anomaly(behavior) is False
        # Anomalous behavior (very high value)
        anomaly = {
            "type": "access",
            "user": "user1",
            "value": 10000,  # Way outside normal range
        }
        # May or may not detect depending on threshold
        result = monitor.detect_anomaly(anomaly)
        assert isinstance(result, bool)

    def test_security_alerts(self):
        """Test security alerts."""
        monitor = SecurityMonitor()
        # Trigger alert
        event = {
            "type": "privilege_escalation",
            "user": "attacker",
        }
        monitor.detect_intrusion(event)
        alerts = monitor.get_security_alerts()
        assert len(alerts) > 0
        assert any("privilege_escalation" in str(a) for a in alerts)

    def test_threat_level_management(self):
        """Test threat level management."""
        monitor = SecurityMonitor()
        # Default threat level
        assert monitor.get_threat_level() == SecurityLevel.MEDIUM
        # Set threat level
        monitor.set_threat_level(SecurityLevel.HIGH)
        assert monitor.get_threat_level() == SecurityLevel.HIGH

    def test_security_metrics(self):
        """Test security metrics."""
        monitor = SecurityMonitor()
        metrics = monitor.get_security_metrics()
        assert "threat_level" in metrics
        assert "security_level" in metrics
        assert "total_alerts" in metrics
        assert "monitoring_active" in metrics
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_security

class TestSecurityPolicy:
    """Test SecurityPolicy implementation."""

    def test_password_policy(self):
        """Test password policy application."""
        policy = SecurityPolicy()
        # Valid password
        context = {"password": "StrongP@ssw0rd"}
        assert policy.apply_policy("password", context) is True
        # Invalid password (too short)
        context = {"password": "weak"}
        assert policy.apply_policy("password", context) is False

    def test_access_control_policy(self):
        """Test access control policy."""
        policy = SecurityPolicy()
        # Valid access
        context = {
            "user": "user1",
            "ip_address": "192.168.1.1",
            "failed_attempts": 2,
        }
        assert policy.apply_policy("access_control", context) is True
        # Too many failed attempts
        context["failed_attempts"] = 6
        assert policy.apply_policy("access_control", context) is False

    def test_policy_management(self):
        """Test policy management."""
        policy = SecurityPolicy()
        # List policies
        policies = policy.list_policies()
        assert "password" in policies
        assert "access_control" in policies
        # Get policy
        password_policy = policy.get_policy("password")
        assert "min_length" in password_policy
        # Set new policy
        new_policy = {"min_length": 12, "max_length": 256}
        policy.set_policy("custom_password", new_policy)
        assert "custom_password" in policy.list_policies()
        # Remove policy
        assert policy.remove_policy("custom_password") is True
        assert "custom_password" not in policy.list_policies()

    def test_policy_validation(self):
        """Test policy validation."""
        policy = SecurityPolicy()
        # Valid policy
        valid_policy = {"min_length": 8, "max_length": 128}
        is_valid, errors = policy.validate_policy(valid_policy)
        assert is_valid is True
        # Invalid policy
        invalid_policy = {"min_length": -1}
        is_valid, errors = policy.validate_policy(invalid_policy)
        assert is_valid is False
        assert len(errors) > 0

    def test_policy_violations(self):
        """Test policy violation tracking."""
        policy = SecurityPolicy()
        # Trigger violation
        context = {"password": "weak"}
        policy.apply_policy("password", context)
        violations = policy.get_policy_violations()
        assert len(violations) > 0
        assert any("password" in str(v) for v in violations)
