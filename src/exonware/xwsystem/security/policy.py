#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/security/policy.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.10
Generation Date: 07-Jan-2025
Security policy implementation for XWSystem.
Implements ISecurityPolicy protocol for security policy management and enforcement.
"""

import time
from typing import Any
from datetime import datetime
from .base import ASecurityPolicyBase
from .contracts import ISecurityPolicy
from .defs import SecurityLevel
from ..config.logging_setup import get_logger
logger = get_logger("xwsystem.security.policy")


class SecurityPolicy(ASecurityPolicyBase, ISecurityPolicy):
    """
    Security policy implementation.
    Provides comprehensive security policy management including:
    - Policy definition and storage
    - Policy validation
    - Policy application
    - Policy violation tracking
    """

    def __init__(self, security_level: SecurityLevel = SecurityLevel.MEDIUM):
        """
        Initialize security policy manager.
        Args:
            security_level: Security level for policies
        """
        super().__init__()
        self.security_level = security_level
        self._policies: dict[str, dict[str, Any]] = {}
        self._policy_violations: list[dict[str, Any]] = []
        # Initialize default policies
        self._initialize_default_policies()

    def _initialize_default_policies(self) -> None:
        """Initialize default security policies."""
        # Password policy
        self._policies["password"] = {
            "min_length": 8,
            "max_length": 128,
            "require_uppercase": True,
            "require_lowercase": True,
            "require_digits": True,
            "require_special": True,
            "max_age_days": 90,
            "history_count": 5,
        }
        # Access control policy
        self._policies["access_control"] = {
            "max_failed_attempts": 5,
            "lockout_duration_minutes": 30,
            "session_timeout_minutes": 30,
            "require_mfa": False,
            "allowed_ip_ranges": [],
            "blocked_ip_ranges": [],
        }
        # Data protection policy
        self._policies["data_protection"] = {
            "encryption_required": True,
            "encryption_algorithm": "AES_256",
            "backup_required": True,
            "retention_days": 365,
            "anonymization_required": False,
        }
        # Audit policy
        self._policies["audit"] = {
            "log_all_access": True,
            "log_failed_attempts": True,
            "retention_days": 90,
            "alert_on_violations": True,
        }

    def get_policy(self, policy_name: str) -> dict[str, Any]:
        """
        Get security policy.
        Args:
            policy_name: Policy name
        Returns:
            Policy dictionary
        """
        return self._policies.get(policy_name, {}).copy()

    def set_policy(self, policy_name: str, policy: dict[str, Any]) -> None:
        """
        Set security policy.
        Args:
            policy_name: Policy name
            policy: Policy dictionary
        """
        # Validate policy before setting
        is_valid, errors = self.validate_policy(policy)
        if not is_valid:
            raise ValueError(f"Invalid policy: {', '.join(errors)}")
        self._policies[policy_name] = policy.copy()
        logger.info(f"Policy '{policy_name}' updated")

    def validate_policy(self, policy: dict[str, Any]) -> tuple[bool, list[str]]:
        """
        Validate security policy.
        Args:
            policy: Policy to validate
        Returns:
            Tuple of (is_valid, error_messages)
        """
        errors: list[str] = []
        if not isinstance(policy, dict):
            errors.append("Policy must be a dictionary")
            return False, errors
        # Check for required fields based on policy type
        if "password" in str(policy).lower():
            required_fields = ["min_length", "max_length"]
            for field in required_fields:
                if field not in policy:
                    errors.append(f"Missing required field: {field}")
        # Validate numeric fields
        numeric_fields = ["min_length", "max_length", "max_age_days", "retention_days"]
        for field in numeric_fields:
            if field in policy:
                if not isinstance(policy[field], (int, float)) or policy[field] < 0:
                    errors.append(f"Field '{field}' must be a non-negative number")
        # Validate boolean fields
        boolean_fields = ["require_uppercase", "require_lowercase", "require_digits", 
                         "require_special", "encryption_required", "backup_required"]
        for field in boolean_fields:
            if field in policy:
                if not isinstance(policy[field], bool):
                    errors.append(f"Field '{field}' must be a boolean")
        is_valid = len(errors) == 0
        return is_valid, errors

    def apply_policy(self, policy_name: str, context: dict[str, Any]) -> bool:
        """
        Apply security policy.
        Args:
            policy_name: Policy name
            context: Context data
        Returns:
            True if policy applied successfully
        """
        if policy_name not in self._policies:
            logger.warning(f"Policy '{policy_name}' not found")
            return False
        policy = self._policies[policy_name]
        context_type = context.get("type", "")
        try:
            if policy_name == "password":
                return self._apply_password_policy(policy, context)
            elif policy_name == "access_control":
                return self._apply_access_control_policy(policy, context)
            elif policy_name == "data_protection":
                return self._apply_data_protection_policy(policy, context)
            elif policy_name == "audit":
                return self._apply_audit_policy(policy, context)
            else:
                # Generic policy application
                return self._apply_generic_policy(policy, context)
        except Exception as e:
            logger.error(f"Error applying policy '{policy_name}': {str(e)}")
            self._record_violation(policy_name, context, f"Policy application error: {str(e)}")
            return False

    def list_policies(self) -> list[str]:
        """
        List all security policies.
        Returns:
            List of policy names
        """
        return list(self._policies.keys())

    def remove_policy(self, policy_name: str) -> bool:
        """
        Remove security policy.
        Args:
            policy_name: Policy name to remove
        Returns:
            True if removed
        """
        if policy_name in self._policies:
            del self._policies[policy_name]
            logger.info(f"Policy '{policy_name}' removed")
            return True
        return False

    def get_policy_violations(self) -> list[dict[str, Any]]:
        """
        Get policy violations.
        Returns:
            List of policy violations
        """
        return self._policy_violations.copy()

    def clear_policy_violations(self) -> None:
        """Clear policy violations."""
        self._policy_violations.clear()

    def _apply_password_policy(self, policy: dict[str, Any], context: dict[str, Any]) -> bool:
        """Apply password policy."""
        password = context.get("password", "")
        if len(password) < policy.get("min_length", 8):
            self._record_violation("password", context, "Password too short")
            return False
        if len(password) > policy.get("max_length", 128):
            self._record_violation("password", context, "Password too long")
            return False
        if policy.get("require_uppercase", True):
            if not any(c.isupper() for c in password):
                self._record_violation("password", context, "Password must contain uppercase")
                return False
        if policy.get("require_lowercase", True):
            if not any(c.islower() for c in password):
                self._record_violation("password", context, "Password must contain lowercase")
                return False
        if policy.get("require_digits", True):
            if not any(c.isdigit() for c in password):
                self._record_violation("password", context, "Password must contain digits")
                return False
        if policy.get("require_special", True):
            special_chars = "!@#$%^&*(),.?\":{}|<>"
            if not any(c in special_chars for c in password):
                self._record_violation("password", context, "Password must contain special characters")
                return False
        return True

    def _apply_access_control_policy(self, policy: dict[str, Any], context: dict[str, Any]) -> bool:
        """Apply access control policy."""
        user = context.get("user", "")
        ip_address = context.get("ip_address", "")
        failed_attempts = context.get("failed_attempts", 0)
        # Check failed attempts
        max_attempts = policy.get("max_failed_attempts", 5)
        if failed_attempts >= max_attempts:
            self._record_violation("access_control", context, 
                                 f"Exceeded max failed attempts: {failed_attempts}")
            return False
        # Check IP ranges
        blocked_ranges = policy.get("blocked_ip_ranges", [])
        if ip_address and any(ip_address.startswith(blocked) for blocked in blocked_ranges):
            self._record_violation("access_control", context, f"IP address blocked: {ip_address}")
            return False
        return True

    def _apply_data_protection_policy(self, policy: dict[str, Any], context: dict[str, Any]) -> bool:
        """Apply data protection policy."""
        data = context.get("data", "")
        is_encrypted = context.get("is_encrypted", False)
        if policy.get("encryption_required", True) and not is_encrypted:
            self._record_violation("data_protection", context, "Data encryption required")
            return False
        return True

    def _apply_audit_policy(self, policy: dict[str, Any], context: dict[str, Any]) -> bool:
        """Apply audit policy."""
        # Audit policy is typically about logging, so it usually passes
        # but we can check if logging is enabled
        log_all_access = policy.get("log_all_access", True)
        if log_all_access:
            event_type = context.get("type", "")
            logger.info(f"Audit log: {event_type} - {context.get('user', 'unknown')}")
        return True

    def _apply_generic_policy(self, policy: dict[str, Any], context: dict[str, Any]) -> bool:
        """Apply generic policy."""
        # Generic policy application - check if context matches policy rules
        for key, expected_value in policy.items():
            if key in context:
                actual_value = context[key]
                if actual_value != expected_value:
                    self._record_violation("generic", context, 
                                         f"Policy violation: {key} = {actual_value}, expected {expected_value}")
                    return False
        return True

    def _record_violation(self, policy_name: str, context: dict[str, Any], reason: str) -> None:
        """
        Record policy violation.
        Args:
            policy_name: Policy name
            context: Context data
            reason: Violation reason
        """
        violation = {
            "policy_name": policy_name,
            "reason": reason,
            "context": context.copy(),
            "timestamp": time.time(),
            "datetime": datetime.now().isoformat(),
        }
        self._policy_violations.append(violation)
        # Keep only last 1000 violations
        if len(self._policy_violations) > 1000:
            self._policy_violations = self._policy_violations[-1000:]
        logger.warning(f"Policy violation [{policy_name}]: {reason}")
        # Alert if configured
        policy = self._policies.get(policy_name, {})
        if policy.get("alert_on_violations", False):
            logger.error(f"SECURITY ALERT: Policy violation detected - {reason}")
