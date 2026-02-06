#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/security/validator.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.4
Generation Date: 07-Jan-2025

Security validator implementation for XWSystem.
Implements ISecurityValidator protocol for comprehensive security validation.
"""

import re
import html
import urllib.parse
from typing import Any, Optional
from email.utils import parseaddr
from urllib.parse import urlparse

from .base import ASecurityValidatorBase
from .contracts import ISecurityValidator
from .defs import SecurityLevel
from ..config.logging_setup import get_logger

logger = get_logger("xwsystem.security.validator")


class SecurityValidator(ASecurityValidatorBase, ISecurityValidator):
    """
    Security validator implementation.
    
    Provides comprehensive security validation including:
    - Password strength validation
    - Input validation and sanitization
    - SQL injection detection
    - XSS detection
    - Certificate validation
    - Security headers validation
    """
    
    def __init__(self, security_level: SecurityLevel = SecurityLevel.MEDIUM):
        """
        Initialize security validator.
        
        Args:
            security_level: Security level for validation
        """
        super().__init__(security_level)
        self._validation_errors: list[str] = []
        
        # SQL injection patterns
        self._sql_patterns = [
            r"(\b(SELECT|INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|EXEC|EXECUTE|UNION|SCRIPT)\b)",
            r"(--|#|/\*|\*/|;|\||&)",
            r"(\bOR\b.*=.*|\bAND\b.*=.*)",
            r"('|(\\')|(;)|(--)|(\|)|(\*)|(%)|(\+)|(,)|(\\)|(\/)|(\[)|(\]))",
            r"(\bxp_\w+|\bsp_\w+)",  # SQL Server stored procedures
        ]
        
        # XSS patterns
        self._xss_patterns = [
            r"<script[^>]*>.*?</script>",
            r"<iframe[^>]*>.*?</iframe>",
            r"javascript:",
            r"on\w+\s*=",  # Event handlers like onclick, onload, etc.
            r"<img[^>]*src[^>]*=.*?javascript:",
            r"<svg[^>]*onload",
            r"<body[^>]*onload",
            r"<input[^>]*onfocus",
            r"eval\s*\(",
            r"expression\s*\(",
        ]
        
        # Security headers to check
        self._required_headers = {
            "X-Content-Type-Options": "nosniff",
            "X-Frame-Options": ["DENY", "SAMEORIGIN"],
            "X-XSS-Protection": "1; mode=block",
            "Strict-Transport-Security": None,  # Just presence required
            "Content-Security-Policy": None,  # Just presence required
        }
    
    def validate_password(self, password: str) -> tuple[bool, list[str]]:
        """
        Validate password strength.
        
        Args:
            password: Password to validate
            
        Returns:
            Tuple of (is_valid, error_messages)
        """
        errors: list[str] = []
        
        if not password:
            errors.append("Password cannot be empty")
            return False, errors
        
        if len(password) < 8:
            errors.append("Password must be at least 8 characters long")
        
        if len(password) > 128:
            errors.append("Password must be no more than 128 characters long")
        
        if not re.search(r"[a-z]", password):
            errors.append("Password must contain at least one lowercase letter")
        
        if not re.search(r"[A-Z]", password):
            errors.append("Password must contain at least one uppercase letter")
        
        if not re.search(r"\d", password):
            errors.append("Password must contain at least one digit")
        
        if not re.search(r"[!@#$%^&*(),.?\":{}|<>]", password):
            errors.append("Password must contain at least one special character")
        
        # Check for common weak passwords
        weak_passwords = [
            "password", "12345678", "qwerty", "abc123", "password123",
            "admin", "letmein", "welcome", "monkey", "1234567890"
        ]
        if password.lower() in weak_passwords:
            errors.append("Password is too common and easily guessable")
        
        # Check for repeated characters
        if re.search(r"(.)\1{3,}", password):
            errors.append("Password contains too many repeated characters")
        
        # Check for sequential characters
        if re.search(r"(012|123|234|345|456|567|678|789|890|abc|bcd|cde|def|efg|fgh|ghi|hij|ijk|jkl|klm|lmn|mno|nop|opq|pqr|qrs|rst|stu|tuv|uvw|vwx|wxy|xyz)", password.lower()):
            errors.append("Password contains sequential characters")
        
        is_valid = len(errors) == 0
        return is_valid, errors
    
    def validate_input(self, input_data: str, input_type: str) -> tuple[bool, list[str]]:
        """
        Validate input data based on type.
        
        Args:
            input_data: Input data to validate
            input_type: Type of input (email, url, etc.)
            
        Returns:
            Tuple of (is_valid, error_messages)
        """
        errors: list[str] = []
        
        if not isinstance(input_data, str):
            errors.append(f"Input must be a string, got {type(input_data).__name__}")
            return False, errors
        
        if input_type == "email":
            name, addr = parseaddr(input_data)
            if not addr or "@" not in addr:
                errors.append("Invalid email format")
            else:
                # Basic email regex
                email_pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
                if not re.match(email_pattern, addr):
                    errors.append("Invalid email format")
        
        elif input_type == "url":
            try:
                result = urlparse(input_data)
                if not all([result.scheme, result.netloc]):
                    errors.append("Invalid URL format")
            except Exception as e:
                errors.append(f"Invalid URL: {str(e)}")
        
        elif input_type == "integer":
            try:
                int(input_data)
            except ValueError:
                errors.append("Input must be a valid integer")
        
        elif input_type == "float":
            try:
                float(input_data)
            except ValueError:
                errors.append("Input must be a valid float")
        
        # Check for SQL injection
        if self.detect_sql_injection(input_data):
            errors.append("Potential SQL injection detected")
        
        # Check for XSS
        if self.detect_xss(input_data):
            errors.append("Potential XSS attack detected")
        
        is_valid = len(errors) == 0
        return is_valid, errors
    
    def sanitize_input(self, input_data: str) -> str:
        """
        Sanitize input data.
        
        Args:
            input_data: Input data to sanitize
            
        Returns:
            Sanitized data
        """
        if not isinstance(input_data, str):
            return str(input_data)
        
        # HTML escape
        sanitized = html.escape(input_data)
        
        # Remove null bytes
        sanitized = sanitized.replace("\x00", "")
        
        # Remove control characters except newlines and tabs
        sanitized = re.sub(r"[\x00-\x08\x0B-\x0C\x0E-\x1F\x7F]", "", sanitized)
        
        # URL decode to prevent double encoding attacks
        try:
            sanitized = urllib.parse.unquote(sanitized)
            sanitized = html.escape(sanitized)  # Re-escape after decode
        except Exception:
            pass
        
        return sanitized
    
    def detect_sql_injection(self, input_data: str) -> bool:
        """
        Detect SQL injection attempts.
        
        Args:
            input_data: Input data to check
            
        Returns:
            True if SQL injection detected
        """
        if not isinstance(input_data, str):
            input_data = str(input_data)
        
        input_upper = input_data.upper()
        
        for pattern in self._sql_patterns:
            if re.search(pattern, input_upper, re.IGNORECASE):
                logger.warning(f"SQL injection pattern detected: {pattern[:50]}")
                return True
        
        return False
    
    def detect_xss(self, input_data: str) -> bool:
        """
        Detect XSS attempts.
        
        Args:
            input_data: Input data to check
            
        Returns:
            True if XSS detected
        """
        if not isinstance(input_data, str):
            input_data = str(input_data)
        
        for pattern in self._xss_patterns:
            if re.search(pattern, input_data, re.IGNORECASE):
                logger.warning(f"XSS pattern detected: {pattern[:50]}")
                return True
        
        return False
    
    def validate_certificate(self, certificate: bytes) -> tuple[bool, str]:
        """
        Validate certificate.
        
        Args:
            certificate: Certificate data
            
        Returns:
            Tuple of (is_valid, error_message)
        """
        # Check if cryptography is available
        import importlib.util
        _cryptography_spec = importlib.util.find_spec('cryptography')
        if _cryptography_spec is None:
            return False, "cryptography library not available for certificate validation"
        
        try:
            from cryptography import x509
            from cryptography.hazmat.backends import default_backend
            
            # Try to parse the certificate
            cert = x509.load_pem_x509_certificate(certificate, default_backend())
            
            # Check if certificate is expired
            from datetime import datetime
            if cert.not_valid_after < datetime.now():
                return False, "Certificate has expired"
            
            # Check if certificate is not yet valid
            if cert.not_valid_before > datetime.now():
                return False, "Certificate is not yet valid"
            
            return True, "Certificate is valid"
        except Exception as e:
            return False, f"Invalid certificate: {str(e)}"
    
    def check_security_headers(self, headers: dict[str, str]) -> dict[str, bool]:
        """
        Check security headers.
        
        Args:
            headers: HTTP headers
            
        Returns:
            Dictionary of header validation results
        """
        results: dict[str, bool] = {}
        headers_upper = {k.upper(): v for k, v in headers.items()}
        
        for header_name, expected_value in self._required_headers.items():
            header_key = header_name.upper()
            
            if header_key not in headers_upper:
                results[header_name] = False
                continue
            
            if expected_value is None:
                # Just presence required
                results[header_name] = True
            elif isinstance(expected_value, list):
                # Value is one of the allowed values
                results[header_name] = headers_upper[header_key].upper() in [v.upper() for v in expected_value]
            else:
                # Value matches exactly
                results[header_name] = headers_upper[header_key].upper() == expected_value.upper()
        
        return results
    
    # Implementation of ASecurityValidatorBase abstract methods
    
    def validate_output(self, data: Any, output_type: str) -> bool:
        """
        Validate output data.
        
        Args:
            data: Output data to validate
            output_type: Type of output
            
        Returns:
            True if valid
        """
        # Basic validation - can be extended
        if output_type == "json":
            import json
            try:
                json.dumps(data)
                return True
            except (TypeError, ValueError):
                return False
        
        return True
    
    def validate_operation(self, operation: str, **kwargs) -> bool:
        """
        Validate operation.
        
        Args:
            operation: Operation name
            **kwargs: Operation parameters
            
        Returns:
            True if valid
        """
        # Check if operation is in allowed list
        allowed_operations = ["read", "write", "delete", "execute"]
        
        if operation.lower() not in allowed_operations:
            self._validation_errors.append(f"Operation '{operation}' is not allowed")
            return False
        
        # Validate parameters based on operation
        if operation.lower() == "delete" and self.security_level == SecurityLevel.HIGH:
            # High security: require confirmation
            if "confirm" not in kwargs or not kwargs.get("confirm"):
                self._validation_errors.append("Delete operation requires confirmation")
                return False
        
        return True
    
    def add_validation_rule(self, rule_name: str, rule_func: callable) -> None:
        """
        Add validation rule.
        
        Args:
            rule_name: Rule name
            rule_func: Rule function
        """
        self._validation_rules[rule_name] = rule_func
    
    def remove_validation_rule(self, rule_name: str) -> None:
        """
        Remove validation rule.
        
        Args:
            rule_name: Rule name
        """
        if rule_name in self._validation_rules:
            del self._validation_rules[rule_name]
    
    def get_validation_errors(self) -> list[str]:
        """
        Get validation errors.
        
        Returns:
            List of error messages
        """
        return self._validation_errors.copy()
    
    def clear_validation_errors(self) -> None:
        """Clear validation errors."""
        self._validation_errors.clear()
    
    def is_secure_operation(self, operation: str) -> bool:
        """
        Check if operation is secure.
        
        Args:
            operation: Operation name
            
        Returns:
            True if secure
        """
        # Check against known secure operations
        secure_operations = ["read", "validate", "check"]
        return operation.lower() in secure_operations
    
    def get_security_score(self) -> float:
        """
        Get security score.
        
        Returns:
            Security score (0.0 to 1.0)
        """
        # Calculate score based on security level and validation rules
        base_scores = {
            SecurityLevel.NONE: 0.0,
            SecurityLevel.LOW: 0.3,
            SecurityLevel.MEDIUM: 0.6,
            SecurityLevel.HIGH: 0.8,
            SecurityLevel.CRITICAL: 1.0,
        }
        
        base_score = base_scores.get(self.security_level, 0.5)
        
        # Add bonus for validation rules
        rule_bonus = min(len(self._validation_rules) * 0.05, 0.2)
        
        return min(base_score + rule_bonus, 1.0)
