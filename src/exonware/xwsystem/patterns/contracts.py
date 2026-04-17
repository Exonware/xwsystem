#exonware/xwsystem/src/exonware/xwsystem/patterns/contracts.py
#exonware/xwsystem/patterns/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.40
Generation Date: September 04, 2025
Pattern contracts and interfaces for XWSystem design patterns.
"""

from __future__ import annotations
from typing import Any, Protocol, runtime_checkable

from collections.abc import Callable
# Root cause: Migrating to Python 3.12 built-in generic syntax for consistency
# Priority #3: Maintainability - Modern type annotations improve code clarity
# Import enums from types module
from .defs import (
    PatternType,
    HandlerType,
    ContextType,
    FactoryType,
    PoolType,
    RegistryType,
    StrategyType,
    ObserverType,
    CommandType,
    StateType,
    BuilderType,
    PrototypeType,
    AdapterType,
    DecoratorType,
    ProxyType,
    FacadeType,
    ChainHandlerType,
    MediatorType,
    MementoType,
    VisitorType,
    IteratorType,
    ConcurrencyType,
    ArchitecturalType,
    SpecificationType,
    ValueObjectType,
    AggregateType
)
# ============================================================================
# CORE INTERFACES
# ============================================================================
@runtime_checkable

class IHandler[T](Protocol):
    """Interface for handlers in the chain of responsibility pattern."""

    def handle(self, request: T) -> T:
        """Handle the request."""
        ...

    def can_handle(self, request: T) -> bool:
        """Check if this handler can handle the request."""
        ...

    def set_next(self, handler: IHandler[T]) -> IHandler[T]:
        """Set the next handler in the chain."""
        ...
@runtime_checkable

class IHandlerFactory[T](Protocol):
    """Interface for handler factories."""

    def create_handler(self, handler_type: str, **kwargs) -> T:
        """Create a handler of the specified type."""
        ...

    def register_handler(self, handler_type: str, handler_class: type[T]) -> None:
        """Register a handler class."""
        ...

    def unregister_handler(self, handler_type: str) -> None:
        """Unregister a handler class."""
        ...

    def list_handlers(self) -> list[str]:
        """List all registered handler types."""
        ...

    def has_handler(self, handler_type: str) -> bool:
        """Check if a handler type is registered."""
        ...
@runtime_checkable

class IContextManager(Protocol):
    """Interface for context managers."""

    def __enter__(self) -> IContextManager:
        """Enter the context."""
        ...

    def __exit__(self, exc_type: type[BaseException] | None, 
                exc_val: BaseException | None, 
                exc_tb: Any | None) -> bool:
        """Exit the context."""
        ...

    def is_active(self) -> bool:
        """Check if context is active."""
        ...

    def get_context_data(self) -> dict[str, Any]:
        """Get context data."""
        ...
@runtime_checkable

class IObjectPool[T](Protocol):
    """Interface for object pools."""

    def get(self, obj_type: type[T], *args, **kwargs) -> T:
        """Get an object from the pool."""
        ...

    def release(self, obj: T) -> None:
        """Release an object back to the pool."""
        ...

    def clear(self, obj_type: type[T] | None = None) -> None:
        """Clear objects from the pool."""
        ...

    def get_stats(self) -> dict[str, Any]:
        """Get pool statistics."""
        ...

    def is_empty(self, obj_type: type[T]) -> bool:
        """Check if pool is empty for a type."""
        ...
@runtime_checkable

class IRegistry[K, V](Protocol):
    """Interface for registries."""

    def register(self, key: K, value: V) -> None:
        """Register a value with a key."""
        ...

    def unregister(self, key: K) -> V:
        """Unregister a value by key."""
        ...

    def get(self, key: K) -> V | None:
        """Get a value by key."""
        ...

    def has(self, key: K) -> bool:
        """Check if key exists."""
        ...

    def list_keys(self) -> list[K]:
        """List all keys."""
        ...

    def list_values(self) -> list[V]:
        """List all values."""
        ...

    def clear(self) -> None:
        """Clear all entries."""
        ...
@runtime_checkable

class IGenericRegistry[T](Protocol):
    """
    Interface for generic registry implementations with metadata support.
    Root cause: Adding generic type parameter for better type safety.
    Priority #3: Maintainability - Generic types improve code clarity and type checking.
    """

    def register(self, name: str, item: T, metadata: dict[str, Any] | None = None) -> bool:
        """Register an item with optional metadata."""
        ...

    def unregister(self, name: str) -> bool:
        """Unregister an item."""
        ...

    def get(self, name: str) -> T | None:
        """Get an item by name."""
        ...

    def list_names(self) -> list[str]:
        """List all registered names."""
        ...

    def exists(self, name: str) -> bool:
        """Check if an item exists."""
        ...

    def clear(self) -> bool:
        """Clear all registrations."""
        ...
@runtime_checkable

class IStrategy(Protocol):
    """Interface for strategies."""

    def execute(self, context: Any) -> Any:
        """Execute the strategy."""
        ...

    def can_handle(self, context: Any) -> bool:
        """Check if strategy can handle context."""
        ...

    def get_name(self) -> str:
        """Get strategy name."""
        ...
@runtime_checkable

class IObserver(Protocol):
    """Interface for observers."""

    def update(self, subject: ISubject, event: Any) -> None:
        """Update the observer."""
        ...

    def get_id(self) -> str:
        """Get observer ID."""
        ...
@runtime_checkable

class ISubject(Protocol):
    """Interface for subjects."""

    def attach(self, observer: IObserver) -> None:
        """Attach an observer."""
        ...

    def detach(self, observer: IObserver) -> None:
        """Detach an observer."""
        ...

    def notify(self, event: Any) -> None:
        """Notify all observers."""
        ...
@runtime_checkable

class ICommand(Protocol):
    """Interface for commands."""

    def execute(self) -> Any:
        """Execute the command."""
        ...

    def undo(self) -> Any:
        """Undo the command."""
        ...

    def can_undo(self) -> bool:
        """Check if command can be undone."""
        ...

    def get_description(self) -> str:
        """Get command description."""
        ...
@runtime_checkable

class IState(Protocol):
    """Interface for states."""

    def enter(self, context: Any) -> None:
        """Enter the state."""
        ...

    def exit(self, context: Any) -> None:
        """Exit the state."""
        ...

    def handle(self, context: Any, event: Any) -> None:
        """Handle an event in this state."""
        ...

    def get_name(self) -> str:
        """Get state name."""
        ...
@runtime_checkable

class IBuilder[T](Protocol):
    """Interface for builders."""

    def build(self) -> T:
        """Build the object."""
        ...

    def reset(self) -> IBuilder[T]:
        """Reset the builder."""
        ...

    def is_valid(self) -> bool:
        """Check if builder is in valid state."""
        ...
@runtime_checkable

class IPrototype[T](Protocol):
    """Interface for prototypes."""

    def clone(self) -> T:
        """Clone the object."""
        ...

    def deep_clone(self) -> T:
        """Create a deep clone."""
        ...

    def shallow_clone(self) -> T:
        """Create a shallow clone."""
        ...
@runtime_checkable

class IAdapter(Protocol):
    """Interface for adapters."""

    def adapt(self, source: Any) -> Any:
        """Adapt source to target."""
        ...

    def can_adapt(self, source: Any) -> bool:
        """Check if source can be adapted."""
        ...

    def get_source_type(self) -> type:
        """Get source type."""
        ...

    def get_target_type(self) -> type:
        """Get target type."""
        ...
@runtime_checkable

class IDecorator[T](Protocol):
    """Interface for decorators."""

    def decorate(self, target: T) -> T:
        """Decorate the target."""
        ...

    def undecorate(self, target: T) -> T:
        """Remove decoration from target."""
        ...

    def is_decorated(self, target: T) -> bool:
        """Check if target is decorated."""
        ...
@runtime_checkable

class IProxy[T](Protocol):
    """Interface for proxies."""

    def get_real_object(self) -> T:
        """Get the real object."""
        ...

    def is_accessible(self) -> bool:
        """Check if real object is accessible."""
        ...

    def get_proxy_type(self) -> str:
        """Get proxy type."""
        ...
@runtime_checkable

class IFacade(Protocol):
    """Interface for facades."""

    def execute(self, operation: str, *args, **kwargs) -> Any:
        """Execute an operation through the facade."""
        ...

    def get_available_operations(self) -> list[str]:
        """Get list of available operations."""
        ...

    def is_operation_supported(self, operation: str) -> bool:
        """Check if operation is supported."""
        ...
@runtime_checkable

class IDynamicFacade(Protocol):
    """Interface for dynamic facades."""

    def get_available_formats(self) -> list[str]:
        """Get list of available formats."""
        ...

    def has_format(self, format_name: str) -> bool:
        """Check if format is available."""
        ...

    def load(self, source: Any, format_name: str, **kwargs) -> Any:
        """Load data using specified format."""
        ...

    def save(self, data: Any, target: Any, format_name: str, **kwargs) -> None:
        """Save data using specified format."""
        ...
@runtime_checkable

class IChainHandler(Protocol):
    """Interface for chain handlers."""

    def handle(self, request: Any) -> Any:
        """Handle the request."""
        ...

    def set_next(self, handler: IChainHandler) -> IChainHandler:
        """Set the next handler."""
        ...

    def can_handle(self, request: Any) -> bool:
        """Check if can handle request."""
        ...
@runtime_checkable

class IMediator(Protocol):
    """Interface for mediators."""

    def register_colleague(self, colleague_id: str, colleague: Any) -> None:
        """Register a colleague."""
        ...

    def unregister_colleague(self, colleague_id: str) -> None:
        """Unregister a colleague."""
        ...

    def send_message(self, sender_id: str, receiver_id: str, message: Any) -> None:
        """Send a message between colleagues."""
        ...

    def broadcast_message(self, sender_id: str, message: Any) -> None:
        """Broadcast a message to all colleagues."""
        ...
@runtime_checkable

class IMemento(Protocol):
    """Interface for mementos."""

    def get_state(self) -> Any:
        """Get the saved state."""
        ...

    def get_timestamp(self) -> float:
        """Get creation timestamp."""
        ...

    def get_description(self) -> str:
        """Get memento description."""
        ...
@runtime_checkable

class IOriginator(Protocol):
    """Interface for originators."""

    def create_memento(self) -> IMemento:
        """Create a memento."""
        ...

    def restore_from_memento(self, memento: IMemento) -> None:
        """Restore from memento."""
        ...
@runtime_checkable

class IVisitor(Protocol):
    """Interface for visitors."""

    def visit(self, element: Any) -> Any:
        """Visit an element."""
        ...

    def can_visit(self, element: Any) -> bool:
        """Check if can visit element."""
        ...
@runtime_checkable

class IElement(Protocol):
    """Interface for elements that accept visitors."""

    def accept(self, visitor: IVisitor) -> Any:
        """Accept a visitor."""
        ...
@runtime_checkable

class IIterator[T](Protocol):
    """Interface for iterators."""

    def __next__(self) -> T:
        """Get next item."""
        ...

    def __iter__(self) -> IIterator[T]:
        """Get iterator."""
        ...

    def has_next(self) -> bool:
        """Check if has next item."""
        ...

    def reset(self) -> None:
        """Reset iterator."""
        ...
@runtime_checkable

class IConcurrencyControl(Protocol):
    """Interface for concurrency control."""

    def acquire(self) -> None:
        """Acquire the lock."""
        ...

    def release(self) -> None:
        """Release the lock."""
        ...

    def is_locked(self) -> bool:
        """Check if locked."""
        ...

    def try_acquire(self, timeout: float | None = None) -> bool:
        """Try to acquire with timeout."""
        ...
@runtime_checkable

class IArchitecturalPattern(Protocol):
    """Interface for architectural patterns."""

    def initialize(self) -> None:
        """Initialize the pattern."""
        ...

    def shutdown(self) -> None:
        """Shutdown the pattern."""
        ...

    def is_initialized(self) -> bool:
        """Check if initialized."""
        ...

    def get_components(self) -> list[str]:
        """Get list of components."""
        ...
@runtime_checkable

class ISpecification(Protocol):
    """Interface for specifications."""

    def is_satisfied_by(self, candidate: Any) -> bool:
        """Check if candidate satisfies specification."""
        ...

    def and_specification(self, other: ISpecification) -> ISpecification:
        """Create AND specification."""
        ...

    def or_specification(self, other: ISpecification) -> ISpecification:
        """Create OR specification."""
        ...

    def not_specification(self) -> ISpecification:
        """Create NOT specification."""
        ...
@runtime_checkable

class IValueObject(Protocol):
    """Interface for value objects."""

    def equals(self, other: Any) -> bool:
        """Check if equal to other."""
        ...

    def get_hash(self) -> int:
        """Get hash code."""
        ...

    def to_string(self) -> str:
        """Convert to string."""
        ...
@runtime_checkable

class IAggregate(Protocol):
    """Interface for aggregates in domain-driven design."""

    def get_id(self) -> str:
        """Get the aggregate ID."""
        ...

    def get_version(self) -> int:
        """Get the aggregate version."""
        ...

    def get_uncommitted_events(self) -> list[Any]:
        """Get uncommitted events."""
        ...

    def mark_events_as_committed(self) -> None:
        """Mark events as committed."""
        ...
@runtime_checkable

class IPattern(Protocol):
    """Interface for design patterns."""

    def get_pattern_type(self) -> PatternType:
        """Get pattern type."""
        ...

    def get_name(self) -> str:
        """Get pattern name."""
        ...

    def get_description(self) -> str:
        """Get pattern description."""
        ...

    def is_applicable(self, context: Any) -> bool:
        """Check if pattern is applicable to context."""
        ...

    def apply(self, context: Any) -> Any:
        """Apply the pattern to context."""
        ...

    def validate(self, data: Any) -> bool:
        """Validate data for pattern application."""
        ...
