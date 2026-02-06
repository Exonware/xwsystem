// #exonware/xwsystem/rust/src/io/errors.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! IO module errors - ALL exceptions in ONE place.
//! 
//! Consolidated from all submodules for maintainability.


use std::path::{Path};

// From common

// From common

// From common

// From common

// From folder

// From stream

// From stream

// From filesystem

// From archive

// From archive

// From archive

// From archive

// From archive

// From archive

// From manager

/// Raised when file is not found.
#[derive(Debug)]
pub struct FileNotFoundError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileNotFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileNotFoundError {
    pub fn new(message: impl Into<String>) -> Self {
        FileNotFoundError {
            message: message.into(),
            path: None,
            source: None,
        }
    }

    pub fn with_path(message: impl Into<String>, path: String) -> Self {
        FileNotFoundError {
            message: message.into(),
            path: Some(path),
            source: None,
        }
    }
}

/// Type alias for FileNotFoundError (Python compatibility)
pub type XWFileNotFoundError = FileNotFoundError;

/// Raised when file permission is denied.
#[derive(Debug)]
pub struct FilePermissionError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FilePermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FilePermissionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FilePermissionError {
    pub fn new(message: impl Into<String>) -> Self {
        FilePermissionError {
            message: message.into(),
            path: None,
            source: None,
        }
    }

    pub fn with_path(message: impl Into<String>, path: String) -> Self {
        FilePermissionError {
            message: message.into(),
            path: Some(path),
            source: None,
        }
    }
}

/// Type alias for FilePermissionError (Python compatibility)
pub type XWPermissionError = FilePermissionError;

/// Raised when file lock operation fails.
#[derive(Debug)]
pub struct FileLockError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileLockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileLockError {
    pub fn new(message: impl Into<String>) -> Self {
        FileLockError {
            message: message.into(),
            path: None,
            source: None,
        }
    }

    pub fn with_path(message: impl Into<String>, path: String) -> Self {
        FileLockError {
            message: message.into(),
            path: Some(path),
            source: None,
        }
    }
}

/// Raised when file read operation fails.
#[derive(Debug)]
pub struct FileReadError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileReadError {
    pub fn new(message: impl Into<String>) -> Self {
        FileReadError {
            message: message.into(),
            path: None,
            source: None,
        }
    }

    pub fn with_path(message: impl Into<String>, path: String) -> Self {
        FileReadError {
            message: message.into(),
            path: Some(path),
            source: None,
        }
    }
}

/// Raised when file write operation fails.
#[derive(Debug)]
pub struct FileWriteError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileWriteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileWriteError {
    pub fn new(message: impl Into<String>) -> Self {
        FileWriteError {
            message: message.into(),
            path: None,
            source: None,
        }
    }

    pub fn with_path(message: impl Into<String>, path: String) -> Self {
        FileWriteError {
            message: message.into(),
            path: Some(path),
            source: None,
        }
    }
}

/// Raised when file delete operation fails.
#[derive(Debug)]
pub struct FileDeleteError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileDeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileDeleteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileDeleteError {
    pub fn new(message: impl Into<String>) -> Self {
        FileDeleteError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when file copy operation fails.
#[derive(Debug)]
pub struct FileCopyError {
    message: String,
    source_path: Option<String>,
    dest_path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileCopyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileCopyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileCopyError {
    pub fn new(message: impl Into<String>) -> Self {
        FileCopyError {
            message: message.into(),
            source_path: None,
            dest_path: None,
            source: None,
        }
    }
}

/// Raised when file move operation fails.
#[derive(Debug)]
pub struct FileMoveError {
    message: String,
    source_path: Option<String>,
    dest_path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileMoveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileMoveError {
    pub fn new(message: impl Into<String>) -> Self {
        FileMoveError {
            message: message.into(),
            source_path: None,
            dest_path: None,
            source: None,
        }
    }
}

/// Raised when directory operation fails.
#[derive(Debug)]
pub struct DirectoryError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DirectoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DirectoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl DirectoryError {
    pub fn new(message: impl Into<String>) -> Self {
        DirectoryError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when directory is not found.
#[derive(Debug)]
pub struct DirectoryNotFoundError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DirectoryNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DirectoryNotFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl DirectoryNotFoundError {
    pub fn new(message: impl Into<String>) -> Self {
        DirectoryNotFoundError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when directory creation fails.
#[derive(Debug)]
pub struct DirectoryCreateError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DirectoryCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DirectoryCreateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl DirectoryCreateError {
    pub fn new(message: impl Into<String>) -> Self {
        DirectoryCreateError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when directory deletion fails.
#[derive(Debug)]
pub struct DirectoryDeleteError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DirectoryDeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DirectoryDeleteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl DirectoryDeleteError {
    pub fn new(message: impl Into<String>) -> Self {
        DirectoryDeleteError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when path operation fails.
#[derive(Debug)]
pub struct PathError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PathError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PathError {
    pub fn new(message: impl Into<String>) -> Self {
        PathError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when path validation fails.
#[derive(Debug)]
pub struct PathValidationError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PathValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PathValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PathValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        PathValidationError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when path resolution fails.
#[derive(Debug)]
pub struct PathResolutionError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PathResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PathResolutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PathResolutionError {
    pub fn new(message: impl Into<String>) -> Self {
        PathResolutionError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when stream operation fails.
#[derive(Debug)]
pub struct StreamError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StreamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StreamError {
    pub fn new(message: impl Into<String>) -> Self {
        StreamError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when stream opening fails.
#[derive(Debug)]
pub struct StreamOpenError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StreamOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StreamOpenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StreamOpenError {
    pub fn new(message: impl Into<String>) -> Self {
        StreamOpenError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when stream closing fails.
#[derive(Debug)]
pub struct StreamCloseError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StreamCloseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StreamCloseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StreamCloseError {
    pub fn new(message: impl Into<String>) -> Self {
        StreamCloseError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when stream read fails.
#[derive(Debug)]
pub struct StreamReadError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StreamReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StreamReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StreamReadError {
    pub fn new(message: impl Into<String>) -> Self {
        StreamReadError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when stream write fails.
#[derive(Debug)]
pub struct StreamWriteError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for StreamWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StreamWriteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl StreamWriteError {
    pub fn new(message: impl Into<String>) -> Self {
        StreamWriteError {
            message: message.into(),
            source: None,
        }
    }
}

/// Raised when atomic operation fails.
#[derive(Debug)]
pub struct AtomicOperationError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for AtomicOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AtomicOperationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl AtomicOperationError {
    pub fn new(message: impl Into<String>) -> Self {
        AtomicOperationError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when backup operation fails.
#[derive(Debug)]
pub struct BackupError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BackupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl BackupError {
    pub fn new(message: impl Into<String>) -> Self {
        BackupError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Raised when temporary file operation fails.
#[derive(Debug)]
pub struct TemporaryFileError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for TemporaryFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TemporaryFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl TemporaryFileError {
    pub fn new(message: impl Into<String>) -> Self {
        TemporaryFileError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Base exception for common IO utility errors.
#[derive(Debug)]
pub struct CommonIOError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CommonIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CommonIOError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CommonIOError {
    pub fn new(message: impl Into<String>) -> Self {
        CommonIOError {
            message: message.into(),
            source: None,
        }
    }

}

/// Error in file watcher operations.
#[derive(Debug)]
pub struct WatcherError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for WatcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for WatcherError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl WatcherError {
    pub fn new(message: impl Into<String>) -> Self {
        WatcherError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Error in file lock operations.
#[derive(Debug)]
pub struct LockError {
    message: String,
    path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for LockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl LockError {
    pub fn new(message: impl Into<String>) -> Self {
        LockError {
            message: message.into(),
            path: None,
            source: None,
        }
    }
}

/// Lock acquisition timeout.
pub struct LockTimeoutError {
    pub message: String,
    pub path: Option<Path>,
    pub timeout: Option<f64>,
}

impl std::fmt::Display for LockTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LockTimeoutError {
}

impl LockTimeoutError {
    /// Constructor
    pub fn new(
        message: String,
        path: Option<Path>,
        timeout: Option<f64>
    ) -> Self {
        Self {
            message,
            path,
            timeout,
        }
    }

}

/// Base exception for file operations.
#[derive(Debug)]
pub struct FileError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileError {
    pub fn new(message: impl Into<String>) -> Self {
        FileError {
            message: message.into(),
            source: None,
        }
    }

}

/// Error in file data source operations.
#[derive(Debug)]
pub struct FileSourceError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileSourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileSourceError {
    pub fn new(message: impl Into<String>) -> Self {
        FileSourceError {
            message: message.into(),
            source: None,
        }
    }
}

/// Error in paged file source operations.
#[derive(Debug)]
pub struct PagedSourceError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PagedSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PagedSourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PagedSourceError {
    pub fn new(message: impl Into<String>) -> Self {
        PagedSourceError {
            message: message.into(),
            source: None,
        }
    }
}

/// Error in paging strategy operations.
pub struct PagingStrategyError {
    pub message: String,
    pub strategy_id: Option<String>,
    pub path: Option<Path>,
    pub original_error: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PagingStrategyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PagingStrategyError {
}

impl PagingStrategyError {
    /// Constructor
    pub fn new(
        message: String,
        strategy_id: Option<String>,
        path: Option<Path>,
        original_error: Option<Box<dyn std::error::Error + Send + Sync + 'static>>
    ) -> Self {
        Self {
            message,
            strategy_id,
            path,
            original_error,
        }
    }

}

/// Base exception for folder operations.
#[derive(Debug)]
pub struct FolderError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FolderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FolderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FolderError {
    pub fn new(message: impl Into<String>) -> Self {
        FolderError {
            message: message.into(),
            source: None,
        }
    }

}

/// Error in codec I/O operations.
#[derive(Debug)]
pub struct CodecIOError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CodecIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CodecIOError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CodecIOError {
    pub fn new(message: impl Into<String>) -> Self {
        CodecIOError {
            message: message.into(),
            source: None,
        }
    }
}

/// Error in async I/O operations.
#[derive(Debug)]
pub struct AsyncIOError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for AsyncIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AsyncIOError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl AsyncIOError {
    pub fn new(message: impl Into<String>) -> Self {
        AsyncIOError {
            message: message.into(),
            source: None,
        }
    }
}

/// Base exception for filesystem operations.
#[derive(Debug)]
pub struct FileSystemError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileSystemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl FileSystemError {
    pub fn new(message: impl Into<String>) -> Self {
        FileSystemError {
            message: message.into(),
            source: None,
        }
    }

}

/// Base exception for archive operations.
#[derive(Debug)]
pub struct ArchiveError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ArchiveError {
    pub fn new(message: impl Into<String>) -> Self {
        ArchiveError {
            message: message.into(),
            source: None,
        }
    }

}

/// Error when archive format is unsupported or invalid.
pub struct ArchiveFormatError {
    pub message: String,
    pub format_id: Option<String>,
    pub archive_path: Option<Path>,
}

impl std::fmt::Display for ArchiveFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ArchiveFormatError {
}

impl ArchiveFormatError {
    /// Constructor
    pub fn new(
        message: String,
        format_id: Option<String>,
        archive_path: Option<Path>
    ) -> Self {
        Self {
            message,
            format_id,
            archive_path,
        }
    }

}

/// Error when archiver lookup fails.
#[derive(Debug)]
pub struct ArchiveNotFoundError {
    message: String,
    format_id: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ArchiveNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ArchiveNotFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ArchiveNotFoundError {
    pub fn new(message: impl Into<String>) -> Self {
        ArchiveNotFoundError {
            message: message.into(),
            format_id: None,
            source: None,
        }
    }
}

/// Error during archive extraction.
#[derive(Debug)]
pub struct ExtractionError {
    message: String,
    archive_path: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ExtractionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ExtractionError {
    pub fn new(message: impl Into<String>) -> Self {
        ExtractionError {
            message: message.into(),
            archive_path: None,
            source: None,
        }
    }
}

/// Error during compression operations.
#[derive(Debug)]
pub struct CompressionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CompressionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CompressionError {
    pub fn new(message: impl Into<String>) -> Self {
        CompressionError {
            message: message.into(),
            source: None,
        }
    }

}

/// Error during decompression operations.
#[derive(Debug)]
pub struct DecompressionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DecompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DecompressionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl DecompressionError {
    pub fn new(message: impl Into<String>) -> Self {
        DecompressionError {
            message: message.into(),
            source: None,
        }
    }
}

/// Base exception for manager operations.
#[derive(Debug)]
pub struct ManagerError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for ManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ManagerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl ManagerError {
    pub fn new(message: impl Into<String>) -> Self {
        ManagerError {
            message: message.into(),
            source: None,
        }
    }

}

/// Base exception for codec operations.
#[derive(Debug)]
pub struct CodecError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CodecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CodecError {
    pub fn new(message: impl Into<String>) -> Self {
        CodecError {
            message: message.into(),
            source: None,
        }
    }

}

/// Base exception for serialization operations.
///
/// Root cause fixed: Added missing SerializationError class that was being
/// imported by serialization/base.py but didn't exist.
///
/// Root cause fixed: Added __init__ to accept format_name and original_error
/// parameters that are used throughout the serialization codebase.
#[derive(Debug)]
pub struct SerializationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SerializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl SerializationError {
    pub fn new(message: impl Into<String>) -> Self {
        SerializationError {
            message: message.into(),
            source: None,
        }
    }

}

/// Raised when encoding fails.
#[derive(Debug)]
pub struct EncodeError {
    message: String,
    codec_id: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl EncodeError {
    pub fn new(message: impl Into<String>) -> Self {
        EncodeError {
            message: message.into(),
            codec_id: None,
            source: None,
        }
    }
}

/// Raised when decoding fails.
#[derive(Debug)]
pub struct DecodeError {
    message: String,
    codec_id: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl DecodeError {
    pub fn new(message: impl Into<String>) -> Self {
        DecodeError {
            message: message.into(),
            codec_id: None,
            source: None,
        }
    }
}

/// Raised when codec is not found.
#[derive(Debug)]
pub struct CodecNotFoundError {
    message: String,
    codec_id: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CodecNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CodecNotFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CodecNotFoundError {
    pub fn new(message: impl Into<String>) -> Self {
        CodecNotFoundError {
            message: message.into(),
            codec_id: None,
            source: None,
        }
    }
}

/// Raised when codec registration fails.
#[derive(Debug)]
pub struct CodecRegistrationError {
    message: String,
    codec_id: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CodecRegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CodecRegistrationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CodecRegistrationError {
    pub fn new(message: impl Into<String>) -> Self {
        CodecRegistrationError {
            message: message.into(),
            codec_id: None,
            source: None,
        }
    }
}
