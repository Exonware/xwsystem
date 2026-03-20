#exonware/xwsystem/src/exonware/xwsystem/validation/facade.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.14
Generation Date: January 2026
XWValidator - Unified Validation Facade
Simplified API for validation operations:
- Quick validation
- Declarative models
- Path validation
- Data validation (depth, size)
"""

from typing import Any
from pathlib import Path
from .declarative import XModel, Field, ValidationError
from .data_validator import DataValidator, validate_path_input, check_data_depth, estimate_memory_usage
from .type_safety import validate_untrusted_data, SafeTypeValidator
from ..security.path_validator import PathValidator
from ..config.logging_setup import get_logger
logger = get_logger(__name__)


class XWValidator:
    """
    Unified validation facade - simple API for validation operations.
    Examples:
        >>> # Quick validation
        >>> XWValidator.validate(data, rules={"age": "int", "name": "required"})
        >>> # Declarative models
        >>> class User(XWValidator.Model):
        ...     name: str = XWValidator.Field(required=True, min_length=3)
        ...     age: int = XWValidator.Field(min=0, max=150)
        >>> user = User(name="John", age=30)  # Validates automatically
        >>> # Path validation
        >>> XWValidator.validate_path("/safe/dir/file.txt", base_path="/safe")
        >>> # Data validation
        >>> XWValidator.validate_depth(data, max_depth=100)
        >>> XWValidator.validate_size(data, max_size_mb=10)
    """
    # Re-export XModel and Field for convenience
    Model = XModel
    Field = Field
    @staticmethod

    def validate(data: Any, rules: dict[str, Any]) -> bool:
        """
        Quick validation with rules dictionary.
        Args:
            data: Data to validate
            rules: Validation rules (e.g., {"age": "int", "name": "required"})
        Returns:
            True if valid
        Raises:
            ValidationError: If validation fails
        """
        # Simple rule-based validation
        if not isinstance(data, dict):
            raise ValidationError("Data must be a dictionary")
        for key, rule in rules.items():
            if rule == "required" and key not in data:
                raise ValidationError(f"Required field missing: {key}")
            elif key in data:
                value = data[key]
                if rule == "int" and not isinstance(value, int):
                    raise ValidationError(f"Field {key} must be int, got {type(value).__name__}")
                elif rule == "str" and not isinstance(value, str):
                    raise ValidationError(f"Field {key} must be str, got {type(value).__name__}")
        return True
    @staticmethod

    def validate_path(path: str, base_path: str | None = None, must_exist: bool = False) -> str:
        """
        Validate and sanitize file path.
        Args:
            path: Path to validate
            base_path: Base directory (prevents directory traversal)
            must_exist: Whether path must exist (default: False)
        Returns:
            Validated path
        Raises:
            ValidationError: If path is invalid
        """
        try:
            validator = PathValidator(base_path) if base_path else PathValidator()
            validated = validator.validate_path(path)
            # Check existence if required
            if must_exist and not Path(validated).exists():
                raise ValidationError(f"Path does not exist: {validated}")
            return validated
        except ValidationError:
            raise
        except Exception as e:
            raise ValidationError(f"Path validation failed: {e}") from e
    @staticmethod

    def validate_depth(data: Any, max_depth: int = 100) -> bool:
        """
        Validate data depth.
        Args:
            data: Data to validate
            max_depth: Maximum allowed depth
        Returns:
            True if depth is within limits
        Raises:
            ValidationError: If depth exceeds limit
        """
        # check_data_depth raises exception if depth exceeds, returns None otherwise
        try:
            check_data_depth(data, max_depth=max_depth)
            return True
        except Exception as e:
            raise ValidationError(f"Data depth validation failed: {e}") from e
    @staticmethod

    def validate_size(data: Any, max_size_mb: int = 10) -> bool:
        """
        Validate data size.
        Args:
            data: Data to validate
            max_size_mb: Maximum size in MB
        Returns:
            True if size is within limits
        Raises:
            ValidationError: If size exceeds limit
        """
        size_mb = estimate_memory_usage(data) / (1024 * 1024)
        if size_mb > max_size_mb:
            raise ValidationError(f"Data size {size_mb:.2f}MB exceeds maximum {max_size_mb}MB")
        return True
    @staticmethod

    def validate_untrusted(data: Any, max_depth: int = 100) -> bool:
        """
        Validate untrusted data with security checks.
        Args:
            data: Untrusted data to validate
            max_depth: Maximum allowed depth
        Returns:
            True if valid
        Raises:
            ValidationError: If validation fails
        """
        return validate_untrusted_data(data, max_depth=max_depth)
    @classmethod

    def create_model(cls, **fields) -> type[XModel]:
        """
        Create a model class dynamically.
        Args:
            **fields: Field definitions
        Returns:
            Model class
        """
        # Create a dynamic model class
        class DynamicModel(XModel):
            pass
        for name, field_def in fields.items():
            if isinstance(field_def, Field):
                setattr(DynamicModel, name, field_def)
            else:
                setattr(DynamicModel, name, Field(default=field_def))
        return DynamicModel
