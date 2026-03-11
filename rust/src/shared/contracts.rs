// #exonware/xwsystem/rust/src/shared/contracts.rs
//exonware/xwsystem/shared/contracts.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 10, 2025
//! 
//! Shared protocol interfaces (merged from the former core module).


use std::collections::HashMap;

use crate::defs::{CloneMode, CoreMode, CorePriority, CoreState, DataType};
use std::option::{Protocol, runtime_checkable};

// ============================================================================

// NATIVE DATA CONVERSION INTERFACES

// ============================================================================

// ============================================================================

// CORE OBJECT INTERFACE

// ============================================================================

// ============================================================================

// CLONING INTERFACES

// ============================================================================

// ============================================================================

// COMPARISON INTERFACES

// ============================================================================

// ============================================================================

// ITERATION INTERFACES

// ============================================================================

// ============================================================================

// CONTAINER INTERFACES

// ============================================================================

// ============================================================================

// METADATA INTERFACES

// ============================================================================

// ============================================================================

// LIFECYCLE INTERFACES

// ============================================================================

// ============================================================================

// FACTORY INTERFACES

// ============================================================================

// ============================================================================

// CORE INTERFACE

// ============================================================================

/// Interface for objects that have unique identification.
///
/// Enforces consistent ID management across XWSystem components.
pub trait IID {
    /// Get the primary identifier.
    /// Returns:
    /// Primary ID string
    // Python decorators: @property
    fn id(&self) -> &str;

    /// Get the unique identifier (UUID).
    /// Returns:
    /// UUID string
    // Python decorators: @property
    fn uid(&self) -> &str;

    /// Generate a new ID.
    /// Returns:
    /// New ID string
    fn generate_id(&self) -> String;

    /// Validate an ID format.
    /// Args:
    /// id_value: ID to validate
    /// Returns:
    /// True if valid
    fn validate_id(&self, id_value: String) -> bool;

    /// Check if this object has the same ID as another.
    /// Args:
    /// other: Another IID object
    /// Returns:
    /// True if same ID
    fn is_same_id(&self, other: String) -> bool;

}

/// Interface for objects that can convert to/from string representation.
///
/// Enforces consistent string conversion behavior across XWSystem.
pub trait IStringable {
    /// Convert object to string representation.
    /// Returns:
    /// String representation of the object
    fn to_string(&self) -> String;

    /// Initialize object from string representation.
    /// Args:
    /// string: String representation to parse
    /// Returns:
    /// True if parsing was successful, False otherwise
    fn from_string(&self, string: String) -> bool;

}

/// Interface for objects that can convert to/from native Python types.
///
/// Enforces consistent native data conversion across XWSystem.
pub trait INative {
    /// Convert to native Python object.
    /// Returns:
    /// Native Python object (dict, list, str, int, float, bool, etc.)
    fn to_native(&self) -> serde_json::Value;

    /// Create from native Python object.
    /// Args:
    /// data: Native Python object
    /// Returns:
    /// New instance created from native data
    fn from_native(&self, data: serde_json::Value) -> String;

    /// Check if data is compatible with native conversion.
    /// Args:
    /// data: Data to check
    /// Returns:
    /// True if compatible
    fn is_native_compatible(&self, data: serde_json::Value) -> bool;

    /// Get the native data type.
    /// Returns:
    /// DataType enum value
    fn get_native_type(&self) -> DataType;

}

/// Core interface for all objects in the eXonware ecosystem.
///
/// This is the foundational object interface that combines identity (IID),
/// native conversion (INative), timestamps, metadata (title/description),
/// serialization, and storage operations. This interface can be used by
/// xwauth, xwstorage, xwentity, and other libraries to ensure consistency.
///
/// Objects implementing IObject can be:
/// - Identified (id, uid)
/// - Converted to/from native Python types (to_native, from_native)
/// - Tracked with timestamps (created_at, updated_at)
/// - Described with metadata (title, description)
/// - Serialized (to_dict)
/// - Stored and loaded (save, load)
pub trait IObject: IID + INative {
    /// Get the creation timestamp.
    /// Returns:
    /// Creation datetime
    // Python decorators: @property
    fn created_at(&self) -> datetime;

    /// Get the last update timestamp.
    /// Returns:
    /// Last update datetime
    // Python decorators: @property
    fn updated_at(&self) -> datetime;

    /// Get the object title.
    /// Returns:
    /// Title string or None if not set
    // Python decorators: @property
    fn title(&self) -> Option<String>;

    /// Get the object description.
    /// Returns:
    /// Description string or None if not set
    // Python decorators: @property
    fn description(&self) -> Option<String>;

    /// Export object as dictionary.
    /// Should include id, uid, created_at, updated_at, title, description,
    /// and any object-specific data.
    /// Returns:
    /// Dictionary representation of the object
    fn to_dict(&self) -> HashMap<String, serde_json::Value>;

    /// Save object to storage.
    /// Subclasses implement object-specific storage logic. This method
    /// can be decorated with @XWAction to enable action-based execution,
    /// validation, and authorization.
    /// Args:
    /// *args: Positional arguments (implementation-specific)
    /// **kwargs: Keyword arguments (implementation-specific)
    fn save(&self) -> ();

    /// Load object from storage.
    /// Subclasses implement object-specific loading logic. This method
    /// can be decorated with @XWAction to enable action-based execution,
    /// validation, and authorization.
    /// Args:
    /// *args: Positional arguments (implementation-specific)
    /// **kwargs: Keyword arguments (implementation-specific)
    fn load(&self) -> ();

}

/// Interface for objects that can be cloned.
///
/// Enforces consistent cloning behavior across XWSystem.
pub trait ICloneable {
    /// Create a clone of this object.
    /// Args:
    /// mode: Cloning mode
    /// Returns:
    /// Cloned object
    fn clone(&self, mode: CloneMode) -> String;

    /// Create a deep clone.
    /// Returns:
    /// Deep cloned object
    fn deep_clone(&self) -> String;

    /// Create a shallow clone.
    /// Returns:
    /// Shallow cloned object
    fn shallow_clone(&self) -> String;

    /// Create a reference clone (same object, different reference).
    /// Returns:
    /// Reference cloned object
    fn reference_clone(&self) -> String;

    /// Check if object can be cloned in given mode.
    /// Args:
    /// mode: Cloning mode to check
    /// Returns:
    /// True if cloneable
    fn is_cloneable(&self, mode: CloneMode) -> bool;

}

/// Interface for objects that can be compared.
///
/// Enforces consistent comparison behavior across XWSystem.
pub trait IComparable {
    /// Check if this object equals another.
    /// Args:
    /// other: Object to compare
    /// Returns:
    /// True if equal
    fn equals(&self, other: serde_json::Value) -> bool;

    /// Compare this object to another.
    /// Args:
    /// other: Object to compare
    /// Returns:
    /// -1 if less than, 0 if equal, 1 if greater than
    fn compare_to(&self, other: serde_json::Value) -> i64;

    /// Get hash code for this object.
    /// Returns:
    /// Hash code
    fn hash_code(&self) -> i64;

    /// Check if this object can be compared to another.
    /// Args:
    /// other: Object to check
    /// Returns:
    /// True if comparable
    fn is_comparable(&self, other: serde_json::Value) -> bool;

}

/// Interface for objects that can be iterated.
///
/// Enforces consistent iteration behavior across XWSystem.
pub trait IIterable {
    /// Check if object is iterable.
    /// Returns:
    /// True if iterable
    fn is_iterable(&self) -> bool;

    /// Get the type of iterator this object provides.
    /// Returns:
    /// Iterator type name
    fn get_iterator_type(&self) -> String;

}

/// Interface for objects that act as containers.
///
/// Enforces consistent container behavior across XWSystem.
pub trait IContainer {
    /// Add item to container.
    /// Args:
    /// item: Item to add
    /// Returns:
    /// True if added successfully
    fn add(&self, item: serde_json::Value) -> bool;

    /// Remove item from container.
    /// Args:
    /// item: Item to remove
    /// Returns:
    /// True if removed successfully
    fn remove(&self, item: serde_json::Value) -> bool;

    /// Clear all items from container.
    fn clear(&self) -> ();

    /// Check if container is empty.
    /// Returns:
    /// True if empty
    fn is_empty(&self) -> bool;

    /// Get size of container.
    /// Returns:
    /// Number of items
    fn size(&self) -> i64;

    /// Check if container contains item.
    /// Args:
    /// item: Item to check
    /// Returns:
    /// True if contains
    fn contains(&self, item: serde_json::Value) -> bool;

}

/// Interface for objects that have metadata.
///
/// Enforces consistent metadata handling across XWSystem.
pub trait IMetadata {
    /// Get metadata value by key.
    /// Args:
    /// key: Metadata key
    /// Returns:
    /// Metadata value
    fn get_metadata(&self, key: String) -> serde_json::Value;

    /// Set metadata value by key.
    /// Args:
    /// key: Metadata key
    /// value: Metadata value
    fn set_metadata(&self, key: String, value: serde_json::Value) -> ();

    /// Check if metadata key exists.
    /// Args:
    /// key: Metadata key
    /// Returns:
    /// True if exists
    fn has_metadata(&self, key: String) -> bool;

    /// Remove metadata by key.
    /// Args:
    /// key: Metadata key
    /// Returns:
    /// True if removed
    fn remove_metadata(&self, key: String) -> bool;

    /// Get all metadata.
    /// Returns:
    /// Dictionary of all metadata
    fn get_all_metadata(&self) -> HashMap<String, serde_json::Value>;

    /// Clear all metadata.
    fn clear_metadata(&self) -> ();

}

/// Interface for objects with lifecycle management.
///
/// Enforces consistent lifecycle behavior across XWSystem.
pub trait ILifecycle {
    /// Initialize the object.
    fn initialize(&self) -> ();

    /// Start the object.
    fn start(&self) -> ();

    /// Stop the object.
    fn stop(&self) -> ();

    /// Shutdown the object.
    fn shutdown(&self) -> ();

    /// Check if object is initialized.
    /// Returns:
    /// True if initialized
    fn is_initialized(&self) -> bool;

    /// Check if object is running.
    /// Returns:
    /// True if running
    fn is_running(&self) -> bool;

    /// Get current state.
    /// Returns:
    /// Current state string
    fn get_state(&self) -> String;

}

/// Interface for factory objects.
///
/// Enforces consistent factory behavior across XWSystem.
pub trait IFactory {
    /// Create a new instance.
    /// Args:
    /// *args: Positional arguments
    /// **kwargs: Keyword arguments
    /// Returns:
    /// New instance
    fn create(&self) -> serde_json::Value;

    /// Create instance from configuration.
    /// Args:
    /// config: Configuration dictionary
    /// Returns:
    /// New instance
    fn create_from_config(&self, config: HashMap<String, serde_json::Value>) -> serde_json::Value;

    /// Get list of supported types.
    /// Returns:
    /// List of supported type names
    fn get_supported_types(&self) -> Vec<String>;

    /// Check if factory can create type.
    /// Args:
    /// type_name: Type name to check
    /// Returns:
    /// True if can create
    fn can_create(&self, type_name: String) -> bool;

}

/// Interface for core functionality.
pub trait ICore {
    /// Get core mode.
    // Python decorators: @property
    fn mode(&self) -> CoreMode;

    /// Get core state.
    // Python decorators: @property
    fn state(&self) -> CoreState;

    /// Initialize core functionality.
    fn initialize(&self) -> ();

    /// Shutdown core functionality.
    fn shutdown(&self) -> ();

    /// Check if core is initialized.
    fn is_initialized(&self) -> bool;

    /// Check if core is shutdown.
    fn is_shutdown(&self) -> bool;

    /// Get all dependencies.
    fn get_dependencies(&self) -> Vec<String>;

    /// Check if all dependencies are satisfied.
    fn check_dependencies(&self) -> bool;

}
