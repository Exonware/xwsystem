#exonware/xwsystem/src/exonware/xwsystem/shared/base.py
#exonware/xwsystem/shared/base.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.16
Generation Date: September 04, 2025
Shared base classes (merged from the former core module).
"""

from abc import ABC, abstractmethod
from typing import Any
from datetime import datetime
import uuid
from .contracts import CoreMode, CorePriority, CoreState, IObject
from .defs import DataType


class ACoreBase(ABC):
    """Abstract base class for core functionality."""

    def __init__(self, mode: CoreMode = CoreMode.DEVELOPMENT):
        """
        Initialize core base.
        Args:
            mode: Core operation mode
        """
        self.mode = mode
        self.state = CoreState.INITIALIZING
        self._dependencies: list[str] = []
        self._resources: dict[str, Any] = {}
    @abstractmethod

    def initialize(self) -> None:
        """Initialize core functionality."""
        pass
    @abstractmethod

    def shutdown(self) -> None:
        """Shutdown core functionality."""
        pass
    @abstractmethod

    def get_state(self) -> CoreState:
        """Get current core state."""
        pass
    @abstractmethod

    def set_state(self, state: CoreState) -> None:
        """Set core state."""
        pass
    @abstractmethod

    def is_initialized(self) -> bool:
        """Check if core is initialized."""
        pass
    @abstractmethod

    def is_shutdown(self) -> bool:
        """Check if core is shutdown."""
        pass
    @abstractmethod

    def add_dependency(self, dependency: str) -> None:
        """Add core dependency."""
        pass
    @abstractmethod

    def remove_dependency(self, dependency: str) -> None:
        """Remove core dependency."""
        pass
    @abstractmethod

    def get_dependencies(self) -> list[str]:
        """Get all dependencies."""
        pass
    @abstractmethod

    def check_dependencies(self) -> bool:
        """Check if all dependencies are satisfied."""
        pass


class AResourceManagerBase(ABC):
    """Abstract base class for resource management."""

    def __init__(self, max_resources: int = 100):
        """
        Initialize resource manager.
        Args:
            max_resources: Maximum number of resources
        """
        self.max_resources = max_resources
        self._resources: dict[str, Any] = {}
        self._resource_locks: dict[str, bool] = {}
    @abstractmethod

    def acquire_resource(
        self, resource_id: str, priority: CorePriority = CorePriority.NORMAL
    ) -> Any:
        """Acquire resource."""
        pass
    @abstractmethod

    def release_resource(self, resource_id: str) -> None:
        """Release resource."""
        pass
    @abstractmethod

    def is_resource_available(self, resource_id: str) -> bool:
        """Check if resource is available."""
        pass
    @abstractmethod

    def get_resource_count(self) -> int:
        """Get number of managed resources."""
        pass
    @abstractmethod

    def list_resources(self) -> list[str]:
        """List all resource IDs."""
        pass
    @abstractmethod

    def cleanup_resources(self) -> None:
        """Cleanup all resources."""
        pass


class AConfigurationBase(ABC):
    """Abstract base class for core configuration."""

    def __init__(self):
        """Initialize configuration base."""
        self._config: dict[str, Any] = {}
        self._defaults: dict[str, Any] = {}
    @abstractmethod

    def load_config(self, config_data: dict[str, Any]) -> None:
        """Load configuration data."""
        pass
    @abstractmethod

    def get_config_value(self, key: str, default: Any = None) -> Any:
        """Get configuration value."""
        pass
    @abstractmethod

    def set_config_value(self, key: str, value: Any) -> None:
        """Set configuration value."""
        pass
    @abstractmethod

    def validate_config(self) -> bool:
        """Validate configuration."""
        pass
    @abstractmethod

    def reset_to_defaults(self) -> None:
        """Reset configuration to defaults."""
        pass
    @abstractmethod

    def export_config(self) -> dict[str, Any]:
        """Export configuration as dictionary."""
        pass


class AValidationBase(ABC):
    """Abstract base class for core validation."""
    @abstractmethod

    def validate_input(self, data: Any) -> bool:
        """Validate input data."""
        pass
    @abstractmethod

    def validate_output(self, data: Any) -> bool:
        """Validate output data."""
        pass
    @abstractmethod

    def validate_operation(self, operation: str, **kwargs) -> bool:
        """Validate operation."""
        pass
    @abstractmethod

    def get_validation_errors(self) -> list[str]:
        """Get validation errors."""
        pass
    @abstractmethod

    def clear_validation_errors(self) -> None:
        """Clear validation errors."""
        pass


class AOperationBase(ABC):
    """Abstract base class for core operations."""

    def __init__(self, timeout: int | None = None):
        """
        Initialize operation base.
        Args:
            timeout: Operation timeout in seconds
        """
        self.timeout = timeout
        self._start_time: float | None = None
        self._end_time: float | None = None
    @abstractmethod

    def execute(self, **kwargs) -> Any:
        """Execute operation."""
        pass
    @abstractmethod

    def is_running(self) -> bool:
        """Check if operation is running."""
        pass
    @abstractmethod

    def is_completed(self) -> bool:
        """Check if operation is completed."""
        pass
    @abstractmethod

    def is_failed(self) -> bool:
        """Check if operation failed."""
        pass
    @abstractmethod

    def get_duration(self) -> float | None:
        """Get operation duration."""
        pass
    @abstractmethod

    def cancel(self) -> None:
        """Cancel operation."""
        pass
    @abstractmethod

    def get_result(self) -> Any:
        """Get operation result."""
        pass
    @abstractmethod

    def get_error(self) -> Exception | None:
        """Get operation error."""
        pass


class BaseCore(ACoreBase):
    """Base implementation of core functionality."""

    def __init__(self, mode: CoreMode = CoreMode.DEVELOPMENT):
        """Initialize base core."""
        super().__init__(mode)
        self._initialized = False
        self._shutdown = False

    def initialize(self) -> None:
        """Initialize core functionality."""
        self.state = CoreState.INITIALIZING
        self._initialized = True
        self.state = CoreState.INITIALIZED

    def shutdown(self) -> None:
        """Shutdown core functionality."""
        self.state = CoreState.SHUTTING_DOWN
        self._shutdown = True
        self.state = CoreState.SHUTDOWN

    def get_state(self) -> CoreState:
        """Get current core state."""
        return self.state

    def set_state(self, state: CoreState) -> None:
        """Set core state."""
        self.state = state

    def is_initialized(self) -> bool:
        """Check if core is initialized."""
        return self._initialized

    def is_shutdown(self) -> bool:
        """Check if core is shutdown."""
        return self._shutdown

    def add_dependency(self, dependency: str) -> None:
        """Add core dependency."""
        if dependency not in self._dependencies:
            self._dependencies.append(dependency)

    def remove_dependency(self, dependency: str) -> None:
        """Remove core dependency."""
        if dependency in self._dependencies:
            self._dependencies.remove(dependency)

    def get_dependencies(self) -> list[str]:
        """Get all dependencies."""
        return self._dependencies.copy()

    def check_dependencies(self) -> bool:
        """Check if all dependencies are satisfied."""
        # Basic implementation - can be overridden
        return True
# ============================================================================
# OBJECT BASE CLASSES
# ============================================================================


class AObject(IObject, ABC):
    """
    Abstract base class for all objects in the eXonware ecosystem.
    Provides common functionality for objects across xwauth, xwstorage, xwentity,
    and other libraries. Extends IObject interface. All object types share:
    - Identity (both required): id = user/programmer-set, for finding and storing;
      uid = system auto-generated, ensures uniqueness. They must not be the same.
    - Timestamps (created_at, updated_at)
    - Metadata (title, description)
    - Native conversion (to_native, from_native from INative)
    - Serialization (to_dict)
    - Storage operations (save, load)
    Subclasses must implement:
    - id property (returns object identifier)
    - created_at property
    - updated_at property
    - to_dict() method (should include title and description)
    - save() method (object-specific storage logic)
    - load() method (object-specific loading logic)
    """

    def __init__(self, object_id: str | None = None):
        """
        Initialize abstract object.
        Args:
            object_id: Optional id value (user/programmer-set). uid is always auto-generated.
        """
        # Timestamps are initialized by subclasses
        # Set self._created_at and self._updated_at in __init__
        # Title and description are optional and initialized by subclasses
        pass
    @property
    @abstractmethod

    def id(self) -> str:
        """User/programmer-set identifier for finding and storing. Not the same as uid."""
        pass
    @property

    def uid(self) -> str:
        """
        System auto-generated unique identifier. Ensures id is never duplicated.
        XWObject generates a UUID on creation; must not be the same as id.
        """
        # Default implementation - subclasses can override
        if not hasattr(self, '_uid'):
            self._uid = str(uuid.uuid4())
        return self._uid
    @property
    @abstractmethod

    def created_at(self) -> datetime:
        """Get the creation timestamp."""
        pass
    @property
    @abstractmethod

    def updated_at(self) -> datetime:
        """Get the last update timestamp."""
        pass
    @property

    def title(self) -> str | None:
        """
        Get the object title.
        Default implementation returns None. Subclasses should override
        to provide title support, storing it in self._title.
        """
        return getattr(self, '_title', None)
    @property

    def description(self) -> str | None:
        """
        Get the object description.
        Default implementation returns None. Subclasses should override
        to provide description support, storing it in self._description.
        """
        return getattr(self, '_description', None)
    @abstractmethod

    def to_dict(self) -> dict[str, Any]:
        """
        Export object as dictionary.
        Must include both id and uid (and created_at, updated_at, title, description,
        and any object-specific data). id and uid are distinct.
        """
        pass

    def to_native(self) -> Any:
        """
        Get object as native representation.
        Default implementation returns to_dict(). Subclasses can override
        if they need a different native representation.
        """
        return self.to_dict()

    def from_native(self, data: dict[str, Any]) -> "IObject":
        """
        Create from native Python object.
        Default implementation raises NotImplementedError. Subclasses should
        override to provide from_native support.
        Args:
            data: Native Python object (typically a dict)
        Returns:
            Self (for chaining) - returns IObject to match protocol
        """
        raise NotImplementedError("Subclasses must implement from_native method")

    def is_native_compatible(self, data: Any) -> bool:
        """
        Check if data is compatible with native conversion.
        Default implementation checks if data is a dict. Subclasses can override
        for more specific validation.
        """
        return isinstance(data, dict)

    def get_native_type(self) -> DataType:
        """
        Get the native data type.
        Default implementation returns DataType.DICT since objects typically
        serialize to dictionaries. Subclasses can override for custom types.
        Returns:
            DataType enum value
        """
        return DataType.DICT
    @abstractmethod

    def save(self, *args, **kwargs) -> None:
        """
        Save object to storage.
        Subclasses must implement this with object-specific storage logic.
        This method can be decorated with @XWAction to enable action-based
        execution, validation, and authorization.
        """
        pass
    @abstractmethod

    def load(self, *args, **kwargs) -> None:
        """
        Load object from storage.
        Subclasses must implement this with object-specific loading logic.
        This method can be decorated with @XWAction to enable action-based
        execution, validation, and authorization.
        """
        pass

    def generate_id(self) -> str:
        """
        Generate a new ID.
        Default implementation generates a UUID. Subclasses can override
        for custom ID generation logic.
        Returns:
            New ID string
        """
        return str(uuid.uuid4())

    def validate_id(self, id_value: str) -> bool:
        """
        Validate an ID format.
        Default implementation checks if ID is a non-empty string.
        Subclasses can override for custom validation logic.
        Args:
            id_value: ID to validate
        Returns:
            True if valid
        """
        return isinstance(id_value, str) and len(id_value) > 0

    def is_same_id(self, other: "IObject") -> bool:
        """
        Check if this object has the same ID as another.
        Args:
            other: Another IObject instance
        Returns:
            True if same ID
        """
        if not isinstance(other, IObject):
            return False
        return self.id == other.id

    def __getitem__(self, key: str) -> Any:
        """
        Get object property using dictionary-style access.
        Supports accessing properties like:
        - obj["uid"] -> returns uid property
        - obj["title"] -> returns title property
        - obj["description"] -> returns description property
        - obj["id"] -> returns id property (if implemented)
        - obj["created_at"] -> returns created_at property (if implemented)
        - obj["updated_at"] -> returns updated_at property (if implemented)
        - obj["property_name"] -> returns any other attribute
        Args:
            key: Property name to access
        Returns:
            Property value
        Raises:
            KeyError: If property doesn't exist
        Example:
            >>> obj = MyObject()
            >>> obj._title = "My Title"
            >>> print(obj["title"])  # "My Title"
            >>> print(obj["uid"])    # UUID string
        """
        # Try to get via property methods first
        if key == "uid":
            return self.uid
        elif key == "title":
            return self.title
        elif key == "description":
            return self.description
        elif key == "id":
            # Try to get id property (may be abstract in some cases)
            try:
                return self.id
            except (AttributeError, NotImplementedError):
                # Fall back to _id attribute if it exists
                if hasattr(self, "_id"):
                    return self._id
                raise KeyError(f"Property 'id' not available")
        elif key == "created_at":
            # Try to get created_at property (may be abstract)
            try:
                return self.created_at
            except (AttributeError, NotImplementedError):
                # Fall back to _created_at attribute if it exists
                if hasattr(self, "_created_at"):
                    return self._created_at
                raise KeyError(f"Property 'created_at' not available")
        elif key == "updated_at":
            # Try to get updated_at property (may be abstract)
            try:
                return self.updated_at
            except (AttributeError, NotImplementedError):
                # Fall back to _updated_at attribute if it exists
                if hasattr(self, "_updated_at"):
                    return self._updated_at
                raise KeyError(f"Property 'updated_at' not available")
        else:
            # Try to get as attribute
            if hasattr(self, key):
                return getattr(self, key)
            # Try with underscore prefix (private attributes)
            if hasattr(self, f"_{key}"):
                return getattr(self, f"_{key}")
            raise KeyError(f"Property '{key}' not found")
