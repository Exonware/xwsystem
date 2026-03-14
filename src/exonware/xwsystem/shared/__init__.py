#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/shared/__init__.py
#exonware/xwsystem/shared/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.7
Generation Date: 10-Sep-2025
Shared types and utilities for XWSystem modules.
"""

from .defs import (
    AuthType,
    CloneMode,
    CoreMode,
    CorePriority,
    CoreState,
    DataType,
    LogLevel,
    LockType,
    OperationResult,
    PathType,
    PerformanceLevel,
    ValidationLevel,
)
from .base import (
    AConfigurationBase,
    ACoreBase,
    AObject,
    AOperationBase,
    AResourceManagerBase,
    AValidationBase,
    BaseCore,
)
from .xwobject import XWObject
from .contracts import (
    ICloneable,
    IComparable,
    IContainer,
    ICore,
    IID,
    IFactory,
    IMetadata,
    INative,
    IObject,
    IStringable,
    ILifecycle,
    IIterable,
    # Provider interfaces (for xwauth/xwstorage decoupling)
    IBasicProviderAuth,
    IBasicProviderStorage,
)
from .errors import (
    CoreConfigurationError,
    CoreDependencyError,
    CoreError,
    CoreInitializationError,
    CoreOperationError,
    CorePermissionError,
    CoreResourceError,
    CoreShutdownError,
    CoreStateError,
    CoreTimeoutError,
    CoreValidationError,
)
__all__ = [
    # Shared enums
    "ValidationLevel",
    "PerformanceLevel",
    "AuthType",
    "LockType",
    "PathType",
    "LogLevel",
    "OperationResult",
    "DataType",
    "CloneMode",
    "CoreState",
    "CoreMode",
    "CorePriority",
    # Base classes
    "ACoreBase",
    "AResourceManagerBase",
    "AConfigurationBase",
    "AValidationBase",
    "AOperationBase",
    "AObject",
    "BaseCore",
    "XWObject",
    # Contracts
    "IID",
    "IStringable",
    "INative",
    "IObject",
    "ICloneable",
    "IComparable",
    "IIterable",
    "IContainer",
    "IMetadata",
    "ILifecycle",
    "IFactory",
    "ICore",
    # Provider interfaces
    "IBasicProviderAuth",
    "IBasicProviderStorage",
    # Errors
    "CoreError",
    "CoreInitializationError",
    "CoreShutdownError",
    "CoreStateError",
    "CoreDependencyError",
    "CoreConfigurationError",
    "CoreResourceError",
    "CoreTimeoutError",
    "CorePermissionError",
    "CoreValidationError",
    "CoreOperationError",
]
