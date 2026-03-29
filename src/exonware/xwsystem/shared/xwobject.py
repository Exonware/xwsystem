#exonware/xwsystem/src/exonware/xwsystem/shared/xwobject.py
#exonware/xwsystem/shared/xwobject.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.23
Generation Date: September 04, 2025
XWObject - Concrete base class for all objects in the eXonware ecosystem.
"""

from typing import Any
from datetime import datetime
import uuid
from .base import AObject
from .contracts import IObject


class XWObject(AObject):
    """
    Concrete base class for all objects in the eXonware ecosystem.
    Provides common functionality shared by objects across xwauth, xwstorage,
    xwentity, and other libraries. Both id and uid are always present and used:
    - id: User/programmer-set, for finding and storing what they want. Set via
      object_id in constructor or from_native; never the same as uid.
    - uid: System auto-generated (UUID), ensures id is never duplicated.
    - Timestamp tracking (created_at, updated_at - abstract, must be set by subclasses)
    - Metadata (title, description - optional)
    - Serialization (to_dict, to_native) must include both id and uid.
    - Storage operations (save, load - abstract, to be implemented by subclasses)
    Subclasses must implement:
    - created_at property
    - updated_at property
    - to_dict() method (must include both id and uid, plus created_at, updated_at, title, description)
    - save() method (object-specific storage logic)
    - load() method (object-specific loading logic)
    Example:
        >>> class MyEntity(XWObject):
        ...     def __init__(self):
        ...         super().__init__(object_id="my_entity")   # id set by programmer
        ...         self._created_at = datetime.now()
        ...         self._updated_at = self._created_at
        ...         self._title = "My Entity"
        ...     # id and uid come from XWObject; both are always present and used
        ...     @property
        ...     def created_at(self) -> datetime:
        ...         return self._created_at
        ...
        ...     @property
        ...     def updated_at(self) -> datetime:
        ...         return self._updated_at
    """

    def __init__(self, object_id: str | None = None):
        """
        Initialize XWObject base class.
        uid is auto-generated (UUID). id is set by programmer when object_id is passed.
        Both id and uid are always present and used; they are never the same.
        Subclasses should call super().__init__(object_id=...) and then set:
        - self._created_at, self._updated_at
        - self._title, self._description (optional)
        Args:
            object_id: Optional id value (user/programmer-set, for finding/storing).
                       uid is always auto-generated and distinct from id.
        """
        super().__init__(object_id)
        # Instance identity: GUID only
        self._uid = str(uuid.uuid4())
        # Semantic id: never equal to uid
        self._id: str | None = object_id if object_id else None
        # Timestamps are initialized by subclasses
        self._title: str | None = None
        self._description: str | None = None
    @property

    def id(self) -> str:
        """
        User/programmer-set identifier for finding and storing. Set via object_id or from_native.
        Never the same as uid. If unset, "".
        """
        return self._id if self._id is not None else ""
    @property

    def uid(self) -> str:
        """
        System auto-generated unique identifier (UUID). Ensures id is never duplicated.
        Always present; never the same as id.
        """
        return self._uid
    @property

    def title(self) -> str | None:
        """
        Get the object title.
        Returns:
            Title string or None if not set
        """
        return self._title
    @property

    def description(self) -> str | None:
        """
        Get the object description.
        Returns:
            Description string or None if not set
        """
        return self._description

    def _update_timestamp(self) -> None:
        """
        Update the updated_at timestamp.
        Subclasses should call this method when modifying the object.
        This is a helper method that subclasses should implement by updating
        self._updated_at. The default implementation does nothing.
        """
        # Subclasses implement this by updating self._updated_at
        # This is a helper method that can be overridden
        pass

    def to_native(self) -> Any:
        """
        Get object as native representation.
        Default implementation returns to_dict(). Subclasses can override
        if they need a different native representation.
        """
        return self.to_dict()

    def __str__(self) -> str:
        """
        String representation as JSON-like output (reusable by XWSchema, XWData, etc.).
        Uses xwsystem JsonSerializer; to_native() so subclasses get their own content.
        """
        try:
            from exonware.xwsystem.io.serialization import JsonSerializer
            out = JsonSerializer().encode(
                self.to_native(),
                options={"indent": 2, "ensure_ascii": False, "default": str}
            )
            return out if isinstance(out, str) else out.decode("utf-8")
        except Exception:
            return super().__str__()
    @classmethod

    def from_string(cls, s: str) -> "IObject":
        """
        Create instance from JSON string (reusable by subclasses).
        Uses xwsystem JsonSerializer. Default: decode, create instance, call from_native(data).
        Subclasses (e.g. XWSchema) may override to construct from schema/data dict.
        """
        from exonware.xwsystem.io.serialization import JsonSerializer
        data = JsonSerializer().decode(s)
        obj = cls()
        obj.from_native(data)
        return obj

    def from_native(self, data: dict[str, Any]) -> "IObject":
        """
        Populate object from native Python dictionary.
        This method takes a dictionary and populates the object's properties.
        It handles:
        - uid: Sets _uid if present in dict
        - title: Sets _title if present in dict
        - description: Sets _description if present in dict
        - created_at: Sets _created_at if present (as datetime or ISO string)
        - updated_at: Sets _updated_at if present (as datetime or ISO string)
        - id: Sets _id if present (subclasses may use this)
        - Any other attributes: Sets them directly on the object
        Args:
            data: Dictionary containing object data
        Returns:
            Self (for chaining) - returns IObject to match protocol
        Example:
            >>> obj = XWObject()
            >>> obj.from_native({
            ...     "uid": "123e4567-e89b-12d3-a456-426614174000",
            ...     "title": "My Object",
            ...     "description": "Description",
            ...     "created_at": "2025-01-25T10:00:00",
            ...     "updated_at": "2025-01-25T11:00:00"
            ... })
        """
        if not isinstance(data, dict):
            raise TypeError(f"from_native expects a dict, got {type(data).__name__}")
        # Handle uid
        if "uid" in data:
            self._uid = str(data["uid"])
        # Handle title
        if "title" in data:
            self._title = data["title"] if data["title"] is not None else None
        # Handle description
        if "description" in data:
            self._description = data["description"] if data["description"] is not None else None
        # Handle created_at (if _created_at attribute exists)
        if "created_at" in data and hasattr(self, "_created_at"):
            created_at_value = data["created_at"]
            if isinstance(created_at_value, str):
                try:
                    self._created_at = datetime.fromisoformat(created_at_value.replace("Z", "+00:00"))
                except (ValueError, AttributeError):
                    # If parsing fails, try to set as string (subclass may handle it)
                    self._created_at = created_at_value
            elif isinstance(created_at_value, datetime):
                self._created_at = created_at_value
        # Handle updated_at (if _updated_at attribute exists)
        if "updated_at" in data and hasattr(self, "_updated_at"):
            updated_at_value = data["updated_at"]
            if isinstance(updated_at_value, str):
                try:
                    self._updated_at = datetime.fromisoformat(updated_at_value.replace("Z", "+00:00"))
                except (ValueError, AttributeError):
                    # If parsing fails, try to set as string (subclass may handle it)
                    self._updated_at = updated_at_value
            elif isinstance(updated_at_value, datetime):
                self._updated_at = updated_at_value
        # Handle id (semantic identifier; never overwrite uid)
        if "id" in data:
            self._id = str(data["id"]) if data["id"] else None
        # Handle any other attributes in the dict
        # Set them directly on the object (subclasses may use this)
        for key, value in data.items():
            if key not in ("uid", "title", "description", "created_at", "updated_at", "id"):
                setattr(self, key, value)
        return self

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
            >>> obj = XWObject()
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
