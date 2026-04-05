#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/security/audit.py
Generic Security Audit Utilities for xwsystem.
Provides generic security auditing that can be used by any library:
- Security issue detection
- Security audit reporting
- Generic security level definitions
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.33
Generation Date: 26-Jan-2025
"""

from typing import Any
from dataclasses import dataclass
from enum import Enum
from exonware.xwsystem import get_logger
logger = get_logger(__name__)


class SecurityLevel(Enum):
    """Security issue severity levels."""
    CRITICAL = "critical"
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"
    INFO = "info"
@dataclass

class SecurityIssue:
    """A security issue found during audit."""
    level: SecurityLevel
    category: str
    description: str
    recommendation: str
    location: str | None = None


class SecurityAuditor:
    """
    Generic security audit utilities.
    Can be used by any library to audit security issues.
    """
    @staticmethod

    def audit_object(obj: Any, object_type: str = "object") -> list[SecurityIssue]:
        """
        Audit an object for security issues.
        Args:
            obj: Object instance to audit
            object_type: Type name of the object (e.g., "strategy", "node", "component")
        Returns:
            List of SecurityIssue objects
        """
        issues = []
        obj_name = getattr(obj, '__class__', {}).__name__ if hasattr(obj, '__class__') else 'Unknown'
        # Check for input validation
        if not hasattr(obj, 'validate_input'):
            issues.append(SecurityIssue(
                level=SecurityLevel.MEDIUM,
                category="Input Validation",
                description=f"{object_type.capitalize()} {obj_name} does not have explicit input validation",
                recommendation="Add validate_input method to check inputs before processing",
                location=f"{obj_name}.validate_input"
            ))
        # Check for bounds checking (if applicable)
        if hasattr(obj, 'get') and not hasattr(obj, '_check_bounds'):
            issues.append(SecurityIssue(
                level=SecurityLevel.LOW,
                category="Bounds Checking",
                description=f"{object_type.capitalize()} {obj_name} may not check bounds on get operations",
                recommendation="Ensure all index/key access operations validate bounds",
                location=f"{obj_name}.get"
            ))
        # Check for error handling
        methods = [m for m in dir(obj) if not m.startswith('_') and callable(getattr(obj, m, None))]
        error_handling_count = sum(1 for m in methods if 'error' in m.lower() or 'exception' in m.lower())
        if error_handling_count == 0:
            issues.append(SecurityIssue(
                level=SecurityLevel.MEDIUM,
                category="Error Handling",
                description=f"{object_type.capitalize()} {obj_name} may lack comprehensive error handling",
                recommendation="Add explicit error handling for edge cases and invalid inputs",
                location=f"{obj_name}"
            ))
        # Check for data sanitization (if applicable)
        if hasattr(obj, 'put') or hasattr(obj, 'set'):
            issues.append(SecurityIssue(
                level=SecurityLevel.INFO,
                category="Data Sanitization",
                description=f"{object_type.capitalize()} {obj_name} should sanitize data before storage",
                recommendation="Consider adding data sanitization for user-provided inputs",
                location=f"{obj_name}.put/set"
            ))
        return issues
    @staticmethod

    def generate_report(issues: list[SecurityIssue]) -> dict[str, Any]:
        """
        Generate a security audit report.
        Args:
            issues: List of security issues
        Returns:
            Dictionary with report data
        """
        by_level = {}
        by_category = {}
        for issue in issues:
            # Group by level
            level = issue.level.value
            if level not in by_level:
                by_level[level] = []
            by_level[level].append(issue)
            # Group by category
            if issue.category not in by_category:
                by_category[issue.category] = []
            by_category[issue.category].append(issue)
        return {
            'total_issues': len(issues),
            'by_level': {k: len(v) for k, v in by_level.items()},
            'by_category': {k: len(v) for k, v in by_category.items()},
            'issues': [
                {
                    'level': issue.level.value,
                    'category': issue.category,
                    'description': issue.description,
                    'recommendation': issue.recommendation,
                    'location': issue.location
                }
                for issue in issues
            ]
        }


def audit_security(obj: Any, object_type: str = "object") -> dict[str, Any]:
    """
    Convenience function to audit an object's security.
    Args:
        obj: Object instance to audit
        object_type: Type name of the object
    Returns:
        Security audit report
    """
    issues = SecurityAuditor.audit_object(obj, object_type)
    return SecurityAuditor.generate_report(issues)
