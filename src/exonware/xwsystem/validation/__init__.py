#exonware/xwsystem/src/exonware/xwsystem/validation/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.10
Generation Date: September 04, 2025
xwsystem Validation Package
Declarative validation with type hints, automatic coercion, and Pydantic-style models.
"""

from .declarative import XModel, Field, ValidationError
from .type_safety import validate_untrusted_data
from .contracts import ISchemaProvider
from .schema_discovery import (
    DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP,
    SchemaValidatorDiscoveryResult,
    discover_schema_validators,
    get_schema_validator,
    set_schema_validator,
    available_schema_validators,
)
# Unified Facade
from .facade import XWValidator
__all__ = [
    # Unified Facade
    "XWValidator",
    # Core Classes
    "XModel",
    "Field", 
    "ValidationError",
    "validate_untrusted_data",
    # Schema validation contracts + discovery (optional providers)
    "ISchemaProvider",
    "DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP",
    "SchemaValidatorDiscoveryResult",
    "discover_schema_validators",
    "get_schema_validator",
    "set_schema_validator",
    "available_schema_validators",
]
