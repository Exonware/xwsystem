// #exonware/xwsystem/rust/src/patterns/defs.rs
//exonware/xwsystem/patterns/types.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Pattern types and enums for XWSystem design patterns.

/// Types of design patterns supported.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
    Creational,
    Structural,
    Behavioral,
    Concurrency,
    Architectural,
}

/// Types of handlers supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandlerType {
    Serialization,
    Validation,
    Transformation,
    Caching,
    Monitoring,
    Security,
}

/// Types of context managers supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextType {
    Resource,
    Transaction,
    Performance,
    Security,
    Logging,
}

/// Types of factories supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactoryType {
    Object,
    Handler,
    Strategy,
    Builder,
    Prototype,
}

/// Types of object pools supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolType {
    Thread,
    Connection,
    Memory,
    Cache,
    Resource,
}

/// Types of registries supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistryType {
    Handler,
    Strategy,
    Factory,
    Service,
    Component,
}

/// Types of strategies supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyType {
    Algorithm,
    Behavior,
    Policy,
    Rule,
    Heuristic,
}

/// Types of observers supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObserverType {
    Event,
    State,
    Data,
    Performance,
    Security,
}

/// Types of commands supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandType {
    Action,
    Query,
    Transaction,
    Batch,
    Async,
}

/// Types of states supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateType {
    Simple,
    Composite,
    Hierarchical,
    Concurrent,
    Persistent,
}

/// Types of builders supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuilderType {
    Object,
    Configuration,
    Query,
    Pipeline,
    Workflow,
}

/// Types of prototypes supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrototypeType {
    Shallow,
    Deep,
    Custom,
    Registry,
    Factory,
}

/// Types of adapters supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterType {
    Object,
    Class,
    Interface,
    Functional,
    Data,
}

/// Types of decorators supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecoratorType {
    Function,
    Class,
    Method,
    Property,
    Behavior,
}

/// Types of proxies supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyType {
    Virtual,
    Remote,
    Protection,
    Caching,
    Synchronization,
}

/// Types of facades supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FacadeType {
    Simple,
    Complex,
    Dynamic,
    Static,
    Hierarchical,
}

/// Types of chain handlers supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChainHandlerType {
    Sequential,
    Parallel,
    Conditional,
    Priority,
    Fallback,
}

/// Types of mediators supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediatorType {
    Simple,
    Complex,
    Hierarchical,
    Distributed,
    #[serde(rename = "event_driven")]
    EventDriven,
}

/// Types of mementos supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MementoType {
    Simple,
    Complex,
    Incremental,
    Compressed,
    Encrypted,
}

/// Types of visitors supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisitorType {
    Simple,
    Complex,
    Hierarchical,
    #[serde(rename = "multi_dispatch")]
    MultiDispatch,
    Functional,
}

/// Types of iterators supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IteratorType {
    Sequential,
    #[serde(rename = "random_access")]
    RandomAccess,
    Bidirectional,
    Filtered,
    Transformed,
}

/// Types of concurrency patterns supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcurrencyType {
    Lock,
    Semaphore,
    Mutex,
    Condition,
    Barrier,
}

/// Types of architectural patterns supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArchitecturalType {
    Mvc,
    Mvp,
    Mvvm,
    Layered,
    Microservice,
}

/// Types of specifications supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecificationType {
    Simple,
    Composite,
    Negation,
    Conjunction,
    Disjunction,
}

/// Types of value objects supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueObjectType {
    Immutable,
    Mutable,
    Composite,
    Primitive,
    Custom,
}

/// Types of aggregates supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregateType {
    Simple,
    Complex,
    Hierarchical,
    Distributed,
    #[serde(rename = "event_sourced")]
    EventSourced,
}
