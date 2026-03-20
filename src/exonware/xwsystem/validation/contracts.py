#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/validation/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.16
Generation Date: September 04, 2025
Validation protocol interfaces for XWSystem.
"""

from typing import Any, Protocol, runtime_checkable

from collections.abc import Iterator, Callable
# Import enums from types module
from .defs import (
    ValidationLevel,
    ValidationType,
    ValidationResult
)
# ============================================================================
# VALIDATION INTERFACES
# ============================================================================
@runtime_checkable

class IValidatable(Protocol):
    """
    Interface for objects that can be validated.
    Enforces consistent validation behavior across XWSystem.
    """

    def validate(self) -> bool:
        """
        Validate this object.
        Returns:
            True if valid
        """
        ...

    def is_valid(self) -> bool:
        """
        Check if object is valid.
        Returns:
            True if valid
        """
        ...

    def get_errors(self) -> list[str]:
        """
        Get validation errors.
        Returns:
            List of error messages
        """
        ...

    def clear_errors(self) -> None:
        """
        Clear validation errors.
        """
        ...

    def has_errors(self) -> bool:
        """
        Check if object has validation errors.
        Returns:
            True if has errors
        """
        ...

    def add_error(self, error: str) -> None:
        """
        Add validation error.
        Args:
            error: Error message
        """
        ...
# ============================================================================
# VALIDATION MANAGER INTERFACES
# ============================================================================
@runtime_checkable

class IValidationManager(Protocol):
    """
    Interface for validation management.
    Enforces consistent validation management across XWSystem.
    """

    def add_validator(self, name: str, validator: Callable[[Any], bool]) -> None:
        """
        Add validator function.
        Args:
            name: Validator name
            validator: Validator function
        """
        ...

    def remove_validator(self, name: str) -> bool:
        """
        Remove validator.
        Args:
            name: Validator name
        Returns:
            True if removed
        """
        ...

    def validate_object(self, obj: Any, validators: list[str]) -> tuple[bool, list[str]]:
        """
        Validate object with specified validators.
        Args:
            obj: Object to validate
            validators: List of validator names
        Returns:
            Tuple of (is_valid, error_messages)
        """
        ...

    def get_validators(self) -> list[str]:
        """
        Get list of available validators.
        Returns:
            List of validator names
        """
        ...
# ============================================================================
# SCHEMA VALIDATION INTERFACES (Schema Provider)
# ============================================================================
@runtime_checkable

class ISchemaProvider(Protocol):
    """
    Interface for schema validation (schema provider).
    Enforces consistent schema validation across XWSystem.
    Implementations are discovered via entry point xwsystem.schema_validators.
    """

    def validate_schema(self, data: Any, schema: dict[str, Any]) -> tuple[bool, list[str]]:
        """
        Validate data against schema.
        Args:
            data: Data to validate
            schema: Schema definition
        Returns:
            Tuple of (is_valid, error_messages)
        """
        ...

    def create_schema(self, data: Any) -> dict[str, Any]:
        """
        Create schema from data.
        Args:
            data: Data to create schema from
        Returns:
            Schema definition
        """
        ...

    def validate_type(self, data: Any, expected_type: str) -> bool:
        """
        Validate data type.
        Args:
            data: Data to validate
            expected_type: Expected type name
        Returns:
            True if type matches
        """
        ...

    def validate_range(self, data: Any, min_value: Any, max_value: Any) -> bool:
        """
        Validate data range.
        Args:
            data: Data to validate
            min_value: Minimum value
            max_value: Maximum value
        Returns:
            True if in range
        """
        ...

    def validate_pattern(self, data: str, pattern: str) -> bool:
        """
        Validate string pattern.
        Args:
            data: String to validate
            pattern: Regex pattern
        Returns:
            True if pattern matches
        """
        ...
# ============================================================================
# VALIDATION PROTOCOLS
# ============================================================================
@runtime_checkable

class IValidatableSimple(Protocol):
    """Protocol for objects that support data validation (simpler interface than IValidatable)."""

    def validate(self, data: Any, **kwargs: Any) -> bool:
        """Validate data against rules."""
        ...

    def get_errors(self) -> list[dict[str, Any]]:
        """Get validation errors."""
        ...
