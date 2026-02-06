// #exonware/xwsystem/rust/src/cli/errors.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! CLI module errors - exception classes for command-line interface functionality.


/// Base exception for CLI errors.
#[derive(Debug)]
pub struct CLIError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CLIError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CLIError {
    pub fn new(message: impl Into<String>) -> Self {
        CLIError {
            message: message.into(),
            source: None,
        }
    }

}

/// Raised when command line argument is invalid.
#[derive(Debug)]
pub struct ArgumentError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ArgumentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ArgumentError {
    pub fn new(message: impl Into<String>) -> Self {
        ArgumentError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ArgumentError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when command execution fails.
#[derive(Debug)]
pub struct CommandError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CommandError {
    pub fn new(message: impl Into<String>) -> Self {
        CommandError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        CommandError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when console operation fails.
#[derive(Debug)]
pub struct ConsoleError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ConsoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConsoleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ConsoleError {
    pub fn new(message: impl Into<String>) -> Self {
        ConsoleError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ConsoleError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when color operation fails.
#[derive(Debug)]
pub struct ColorError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ColorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ColorError {
    pub fn new(message: impl Into<String>) -> Self {
        ColorError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when progress bar operation fails.
#[derive(Debug)]
pub struct ProgressError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ProgressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ProgressError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ProgressError {
    pub fn new(message: impl Into<String>) -> Self {
        ProgressError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when table operation fails.
#[derive(Debug)]
pub struct TableError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for TableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TableError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl TableError {
    pub fn new(message: impl Into<String>) -> Self {
        TableError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when user prompt operation fails.
#[derive(Debug)]
pub struct PromptError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PromptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PromptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PromptError {
    pub fn new(message: impl Into<String>) -> Self {
        PromptError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when user input is invalid.
#[derive(Debug)]
pub struct InputError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl InputError {
    pub fn new(message: impl Into<String>) -> Self {
        InputError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when output operation fails.
#[derive(Debug)]
pub struct OutputError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl OutputError {
    pub fn new(message: impl Into<String>) -> Self {
        OutputError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when terminal operation fails.
#[derive(Debug)]
pub struct TerminalError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TerminalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl TerminalError {
    pub fn new(message: impl Into<String>) -> Self {
        TerminalError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when formatting operation fails.
#[derive(Debug)]
pub struct FormatError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FormatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FormatError {
    pub fn new(message: impl Into<String>) -> Self {
        FormatError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when CLI validation fails.
#[derive(Debug)]
pub struct CLIValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CLIValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CLIValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CLIValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        CLIValidationError {
            message: message.into(),
            source: None,
        }
    }
}
