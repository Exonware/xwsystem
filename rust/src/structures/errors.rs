// #exonware/xwsystem/rust/src/structures/errors.rs
//exonware/xwsystem/structures/errors.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Structures module errors - exception classes for data structure functionality.


/// Base exception for structure errors.
#[derive(Debug)]
pub struct StructureError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StructureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StructureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StructureError {
    pub fn new(message: impl Into<String>) -> Self {
        StructureError {
            message: message.into(),
            source: None,
        }
    }

}

/// Raised when tree operation fails.
#[derive(Debug)]
pub struct TreeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TreeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl TreeError {
    pub fn new(message: impl Into<String>) -> Self {
        TreeError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        TreeError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when tree node operation fails.
#[derive(Debug)]
pub struct TreeNodeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for TreeNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TreeNodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl TreeNodeError {
    pub fn new(message: impl Into<String>) -> Self {
        TreeNodeError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        TreeNodeError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when tree traversal fails.
#[derive(Debug)]
pub struct TreeTraversalError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for TreeTraversalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TreeTraversalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl TreeTraversalError {
    pub fn new(message: impl Into<String>) -> Self {
        TreeTraversalError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        TreeTraversalError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when tree validation fails.
#[derive(Debug)]
pub struct TreeValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for TreeValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TreeValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl TreeValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        TreeValidationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        TreeValidationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when graph operation fails.
#[derive(Debug)]
pub struct GraphError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GraphError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl GraphError {
    pub fn new(message: impl Into<String>) -> Self {
        GraphError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        GraphError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when graph node operation fails.
#[derive(Debug)]
pub struct GraphNodeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for GraphNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GraphNodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl GraphNodeError {
    pub fn new(message: impl Into<String>) -> Self {
        GraphNodeError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        GraphNodeError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when graph edge operation fails.
#[derive(Debug)]
pub struct GraphEdgeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for GraphEdgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GraphEdgeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl GraphEdgeError {
    pub fn new(message: impl Into<String>) -> Self {
        GraphEdgeError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        GraphEdgeError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when graph traversal fails.
#[derive(Debug)]
pub struct GraphTraversalError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for GraphTraversalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GraphTraversalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl GraphTraversalError {
    pub fn new(message: impl Into<String>) -> Self {
        GraphTraversalError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        GraphTraversalError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when graph validation fails.
#[derive(Debug)]
pub struct GraphValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for GraphValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GraphValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl GraphValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        GraphValidationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        GraphValidationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when circular reference is detected.
#[derive(Debug)]
pub struct CircularReferenceError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CircularReferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CircularReferenceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CircularReferenceError {
    pub fn new(message: impl Into<String>) -> Self {
        CircularReferenceError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CircularReferenceError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when circular reference detection fails.
#[derive(Debug)]
pub struct CircularDetectionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CircularDetectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CircularDetectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CircularDetectionError {
    pub fn new(message: impl Into<String>) -> Self {
        CircularDetectionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CircularDetectionError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when circular reference breaking fails.
#[derive(Debug)]
pub struct CircularBreakError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CircularBreakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CircularBreakError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CircularBreakError {
    pub fn new(message: impl Into<String>) -> Self {
        CircularBreakError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CircularBreakError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when structure validation fails.
#[derive(Debug)]
pub struct StructureValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StructureValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StructureValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StructureValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        StructureValidationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        StructureValidationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when structure type is invalid.
#[derive(Debug)]
pub struct StructureTypeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StructureTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StructureTypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StructureTypeError {
    pub fn new(message: impl Into<String>) -> Self {
        StructureTypeError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        StructureTypeError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when structure size is invalid.
#[derive(Debug)]
pub struct StructureSizeError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StructureSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StructureSizeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StructureSizeError {
    pub fn new(message: impl Into<String>) -> Self {
        StructureSizeError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        StructureSizeError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when structure operation fails.
#[derive(Debug)]
pub struct StructureOperationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StructureOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StructureOperationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StructureOperationError {
    pub fn new(message: impl Into<String>) -> Self {
        StructureOperationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        StructureOperationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when structure index is invalid.
#[derive(Debug)]
pub struct StructureIndexError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StructureIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StructureIndexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StructureIndexError {
    pub fn new(message: impl Into<String>) -> Self {
        StructureIndexError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        StructureIndexError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when structure key is invalid.
#[derive(Debug)]
pub struct StructureKeyError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StructureKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StructureKeyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StructureKeyError {
    pub fn new(message: impl Into<String>) -> Self {
        StructureKeyError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        StructureKeyError {
            message: message.into(),
            source: Some(source),
        }
    }
}
