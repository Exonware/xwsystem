// #exonware/xwsystem/rust/src/patterns/contracts.rs
//exonware/xwsystem/patterns/contracts.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Pattern contracts and interfaces for XWSystem design patterns.


use std::collections::HashMap;

use crate::defs::{AdapterType, AggregateType, ArchitecturalType, BuilderType, ChainHandlerType, CommandType, ConcurrencyType, ContextType, DecoratorType, FacadeType, FactoryType, HandlerType, IteratorType, MediatorType, MementoType, ObserverType, PatternType, PoolType, PrototypeType, ProxyType, RegistryType, SpecificationType, StateType, StrategyType, ValueObjectType, VisitorType};

/// Interface for handlers in the chain of responsibility pattern.
pub trait IHandler {
    /// Handle the request.
    fn handle(&self, request: T) -> T;

    /// Check if this handler can handle the request.
    fn can_handle(&self, request: T) -> bool;

    /// Set the next handler in the chain.
    fn set_next(&self, handler: String) -> String;

}

/// Interface for handler factories.
pub trait IHandlerFactory {
    /// Create a handler of the specified type.
    fn create_handler(&self, handler_type: String) -> T;

    /// Register a handler class.
    fn register_handler(&self, handler_type: String, handler_class: String) -> ();

    /// Unregister a handler class.
    fn unregister_handler(&self, handler_type: String) -> ();

    /// List all registered handler types.
    fn list_handlers(&self) -> Vec<String>;

    /// Check if a handler type is registered.
    fn has_handler(&self, handler_type: String) -> bool;

}

/// Interface for context managers.
pub trait IContextManager {
    /// Check if context is active.
    fn is_active(&self) -> bool;

    /// Get context data.
    fn get_context_data(&self) -> HashMap<String, serde_json::Value>;

}

/// Interface for object pools.
pub trait IObjectPool {
    /// Get an object from the pool.
    fn get(&self, obj_type: String) -> T;

    /// Release an object back to the pool.
    fn release(&self, obj: T) -> ();

    /// Clear objects from the pool.
    fn clear(&self, obj_type: Option<String>) -> ();

    /// Get pool statistics.
    fn get_stats(&self) -> HashMap<String, serde_json::Value>;

    /// Check if pool is empty for a type.
    fn is_empty(&self, obj_type: String) -> bool;

}

/// Interface for registries.
pub trait IRegistry {
    /// Register a value with a key.
    fn register(&self, key: K, value: V) -> ();

    /// Unregister a value by key.
    fn unregister(&self, key: K) -> V;

    /// Get a value by key.
    fn get(&self, key: K) -> Option<V>;

    /// Check if key exists.
    fn has(&self, key: K) -> bool;

    /// List all keys.
    fn list_keys(&self) -> Vec<K>;

    /// List all values.
    fn list_values(&self) -> Vec<V>;

    /// Clear all entries.
    fn clear(&self) -> ();

}

/// Interface for generic registry implementations with metadata support.
///
/// Root cause: Adding generic type parameter for better type safety.
/// Priority #3: Maintainability - Generic types improve code clarity and type checking.
pub trait IGenericRegistry {
    /// Register an item with optional metadata.
    fn register(&self, name: String, item: T, metadata: Option<HashMap<String, serde_json::Value>>) -> bool;

    /// Unregister an item.
    fn unregister(&self, name: String) -> bool;

    /// Get an item by name.
    fn get(&self, name: String) -> Option<T>;

    /// List all registered names.
    fn list_names(&self) -> Vec<String>;

    /// Check if an item exists.
    fn exists(&self, name: String) -> bool;

    /// Clear all registrations.
    fn clear(&self) -> bool;

}

/// Interface for strategies.
pub trait IStrategy {
    /// Execute the strategy.
    fn execute(&self, context: serde_json::Value) -> serde_json::Value;

    /// Check if strategy can handle context.
    fn can_handle(&self, context: serde_json::Value) -> bool;

    /// Get strategy name.
    fn get_name(&self) -> String;

}

/// Interface for observers.
pub trait IObserver {
    /// Update the observer.
    fn update(&self, subject: String, event: serde_json::Value) -> ();

    /// Get observer ID.
    fn get_id(&self) -> String;

}

/// Interface for subjects.
pub trait ISubject {
    /// Attach an observer.
    fn attach(&self, observer: IObserver) -> ();

    /// Detach an observer.
    fn detach(&self, observer: IObserver) -> ();

    /// Notify all observers.
    fn notify(&self, event: serde_json::Value) -> ();

}

/// Interface for commands.
pub trait ICommand {
    /// Execute the command.
    fn execute(&self) -> serde_json::Value;

    /// Undo the command.
    fn undo(&self) -> serde_json::Value;

    /// Check if command can be undone.
    fn can_undo(&self) -> bool;

    /// Get command description.
    fn get_description(&self) -> String;

}

/// Interface for states.
pub trait IState {
    /// Enter the state.
    fn enter(&self, context: serde_json::Value) -> ();

    /// Exit the state.
    fn exit(&self, context: serde_json::Value) -> ();

    /// Handle an event in this state.
    fn handle(&self, context: serde_json::Value, event: serde_json::Value) -> ();

    /// Get state name.
    fn get_name(&self) -> String;

}

/// Interface for builders.
pub trait IBuilder {
    /// Build the object.
    fn build(&self) -> T;

    /// Reset the builder.
    fn reset(&self) -> String;

    /// Check if builder is in valid state.
    fn is_valid(&self) -> bool;

}

/// Interface for prototypes.
pub trait IPrototype {
    /// Clone the object.
    fn clone(&self) -> T;

    /// Create a deep clone.
    fn deep_clone(&self) -> T;

    /// Create a shallow clone.
    fn shallow_clone(&self) -> T;

}

/// Interface for adapters.
pub trait IAdapter {
    /// Adapt source to target.
    fn adapt(&self, source: serde_json::Value) -> serde_json::Value;

    /// Check if source can be adapted.
    fn can_adapt(&self, source: serde_json::Value) -> bool;

    /// Get source type.
    fn get_source_type(&self) -> String;

    /// Get target type.
    fn get_target_type(&self) -> String;

}

/// Interface for decorators.
pub trait IDecorator {
    /// Decorate the target.
    fn decorate(&self, target: T) -> T;

    /// Remove decoration from target.
    fn undecorate(&self, target: T) -> T;

    /// Check if target is decorated.
    fn is_decorated(&self, target: T) -> bool;

}

/// Interface for proxies.
pub trait IProxy {
    /// Get the real object.
    fn get_real_object(&self) -> T;

    /// Check if real object is accessible.
    fn is_accessible(&self) -> bool;

    /// Get proxy type.
    fn get_proxy_type(&self) -> String;

}

/// Interface for facades.
pub trait IFacade {
    /// Execute an operation through the facade.
    fn execute(&self, operation: String) -> serde_json::Value;

    /// Get list of available operations.
    fn get_available_operations(&self) -> Vec<String>;

    /// Check if operation is supported.
    fn is_operation_supported(&self, operation: String) -> bool;

}

/// Interface for dynamic facades.
pub trait IDynamicFacade {
    /// Get list of available formats.
    fn get_available_formats(&self) -> Vec<String>;

    /// Check if format is available.
    fn has_format(&self, format_name: String) -> bool;

    /// Load data using specified format.
    fn load(&self, source: serde_json::Value, format_name: String) -> serde_json::Value;

    /// Save data using specified format.
    fn save(&self, data: serde_json::Value, target: serde_json::Value, format_name: String) -> ();

}

/// Interface for chain handlers.
pub trait IChainHandler {
    /// Handle the request.
    fn handle(&self, request: serde_json::Value) -> serde_json::Value;

    /// Set the next handler.
    fn set_next(&self, handler: String) -> String;

    /// Check if can handle request.
    fn can_handle(&self, request: serde_json::Value) -> bool;

}

/// Interface for mediators.
pub trait IMediator {
    /// Register a colleague.
    fn register_colleague(&self, colleague_id: String, colleague: serde_json::Value) -> ();

    /// Unregister a colleague.
    fn unregister_colleague(&self, colleague_id: String) -> ();

    /// Send a message between colleagues.
    fn send_message(&self, sender_id: String, receiver_id: String, message: serde_json::Value) -> ();

    /// Broadcast a message to all colleagues.
    fn broadcast_message(&self, sender_id: String, message: serde_json::Value) -> ();

}

/// Interface for mementos.
pub trait IMemento {
    /// Get the saved state.
    fn get_state(&self) -> serde_json::Value;

    /// Get creation timestamp.
    fn get_timestamp(&self) -> f64;

    /// Get memento description.
    fn get_description(&self) -> String;

}

/// Interface for originators.
pub trait IOriginator {
    /// Create a memento.
    fn create_memento(&self) -> IMemento;

    /// Restore from memento.
    fn restore_from_memento(&self, memento: IMemento) -> ();

}

/// Interface for visitors.
pub trait IVisitor {
    /// Visit an element.
    fn visit(&self, element: serde_json::Value) -> serde_json::Value;

    /// Check if can visit element.
    fn can_visit(&self, element: serde_json::Value) -> bool;

}

/// Interface for elements that accept visitors.
pub trait IElement {
    /// Accept a visitor.
    fn accept(&self, visitor: IVisitor) -> serde_json::Value;

}

/// Interface for iterators.
pub trait IIterator {
    /// Check if has next item.
    fn has_next(&self) -> bool;

    /// Reset iterator.
    fn reset(&self) -> ();

}

/// Interface for concurrency control.
pub trait IConcurrencyControl {
    /// Acquire the lock.
    fn acquire(&self) -> ();

    /// Release the lock.
    fn release(&self) -> ();

    /// Check if locked.
    fn is_locked(&self) -> bool;

    /// Try to acquire with timeout.
    fn try_acquire(&self, timeout: Option<f64>) -> bool;

}

/// Interface for architectural patterns.
pub trait IArchitecturalPattern {
    /// Initialize the pattern.
    fn initialize(&self) -> ();

    /// Shutdown the pattern.
    fn shutdown(&self) -> ();

    /// Check if initialized.
    fn is_initialized(&self) -> bool;

    /// Get list of components.
    fn get_components(&self) -> Vec<String>;

}

/// Interface for specifications.
pub trait ISpecification {
    /// Check if candidate satisfies specification.
    fn is_satisfied_by(&self, candidate: serde_json::Value) -> bool;

    /// Create AND specification.
    fn and_specification(&self, other: String) -> String;

    /// Create OR specification.
    fn or_specification(&self, other: String) -> String;

    /// Create NOT specification.
    fn not_specification(&self) -> String;

}

/// Interface for value objects.
pub trait IValueObject {
    /// Check if equal to other.
    fn equals(&self, other: serde_json::Value) -> bool;

    /// Get hash code.
    fn get_hash(&self) -> i64;

    /// Convert to string.
    fn to_string(&self) -> String;

}

/// Interface for aggregates in domain-driven design.
pub trait IAggregate {
    /// Get the aggregate ID.
    fn get_id(&self) -> String;

    /// Get the aggregate version.
    fn get_version(&self) -> i64;

    /// Get uncommitted events.
    fn get_uncommitted_events(&self) -> Vec<serde_json::Value>;

    /// Mark events as committed.
    fn mark_events_as_committed(&self) -> ();

}

/// Interface for design patterns.
pub trait IPattern {
    /// Get pattern type.
    fn get_pattern_type(&self) -> PatternType;

    /// Get pattern name.
    fn get_name(&self) -> String;

    /// Get pattern description.
    fn get_description(&self) -> String;

    /// Check if pattern is applicable to context.
    fn is_applicable(&self, context: serde_json::Value) -> bool;

    /// Apply the pattern to context.
    fn apply(&self, context: serde_json::Value) -> serde_json::Value;

    /// Validate data for pattern application.
    fn validate(&self, data: serde_json::Value) -> bool;

}
