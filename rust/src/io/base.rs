// #exonware/xwsystem/rust/src/io/base.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! IO module base classes - abstract classes for input/output functionality.


use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::contracts::{FileMode, FileType, IAsyncIO, IAtomicOperations, IBackupOperations, IFile, IFileManager, IFolder, IPath, IStream, ITemporaryOperations, IUnifiedIO, LockType, OperationResult, PathType, TextIO};
use crate::common::atomic::AtomicFileWriter;
use std::path::{Path};

// ============================================================================

// FOLDER ABSTRACT BASE CLASS

// ============================================================================

// ============================================================================

// PATH ABSTRACT BASE CLASS

// ============================================================================

// ============================================================================

// STREAM ABSTRACT BASE CLASS

// ============================================================================

// ============================================================================

// ASYNC I/O ABSTRACT BASE CLASS

// ============================================================================

// ============================================================================

// ATOMIC OPERATIONS ABSTRACT BASE CLASS

// ============================================================================

// ============================================================================

// BACKUP OPERATIONS ABSTRACT BASE CLASS

// ============================================================================

// ============================================================================

// TEMPORARY OPERATIONS ABSTRACT BASE CLASS

// ============================================================================

// ============================================================================

// UNIFIED I/O ABSTRACT BASE CLASS

// ============================================================================

// ============================================================================

// FILE MANAGER ABSTRACT BASE CLASS

// ============================================================================

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
// STATIC UTILITY METHODS (File Manager Features)
// ============================================================================
/// Abstract base class for file operations with both static and instance methods.
pub trait AFile: IFile {
    /// Open file with specified mode.
    fn open(&self, mode: FileMode) -> ();

    /// Read from file.
    fn read(&self, size: Option<i64>) -> String;

    /// Write to file.
    fn write(&self, data: String) -> i64;

    /// Close file.
    fn close(&self) {
        // Default implementation - subclasses should override if they have handles to close
    }

    /// Save data to file.
    fn save(&self, data: serde_json::Value) -> bool;

    /// Load data from file.
    fn load(&self) -> serde_json::Value;

    /// Save data to specific path.
    fn save_as(&self, path: String, data: serde_json::Value) -> bool;

    /// Write current object to file.
    fn to_file(&self, path: String) -> bool;

    /// Load object from file.
    fn from_file(&self, path: String) -> String;

    /// Check if file exists.
    // Python decorators: @staticmethod
    fn exists(path: String) -> bool {
        let path = Path::new(&path);
        path.exists() && path.is_file()
    }

    /// Get file size.
    // Python decorators: @staticmethod
    fn size(path: String) -> i64 {
        if Self::exists(path.clone()) {
            if let Ok(metadata) = fs::metadata(&path) {
                return metadata.len() as i64;
            }
        }
        0
    }

    /// Delete file.
    // Python decorators: @staticmethod
    fn delete(path: String) -> bool {
        if Self::exists(path.clone()) {
            fs::remove_file(&path).is_ok()
        } else {
            false
        }
    }

    /// Copy file.
    // Python decorators: @staticmethod
    fn copy(source: String, destination: String) -> bool {
        fs::copy(&source, &destination).is_ok()
    }

    /// Move file.
    // Python decorators: @staticmethod
    fn move(source: String, destination: String) -> bool {
        // Ensure destination directory exists
        if let Some(parent) = Path::new(&destination).parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::rename(&source, &destination).is_ok()
    }

    /// Rename file.
    // Python decorators: @staticmethod
    fn rename(old_path: String, new_path: String) -> bool {
        Self::move(old_path, new_path)
    }

    /// Get file modification time.
    // Python decorators: @staticmethod
    fn get_modified_time(path: String) -> f64 {
        if Self::exists(path.clone()) {
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                        return duration.as_secs_f64();
                    }
                }
            }
        }
        0.0
    }

    /// Get file creation time.
    // Python decorators: @staticmethod
    fn get_created_time(path: String) -> f64 {
        if Self::exists(path.clone()) {
            if let Ok(metadata) = fs::metadata(&path) {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    let ctime = metadata.ctime();
                    return ctime as f64;
                }
                #[cfg(windows)]
                {
                    use std::os::windows::fs::MetadataExt;
                    if let Ok(created) = metadata.created() {
                        if let Ok(duration) = created.duration_since(UNIX_EPOCH) {
                            return duration.as_secs_f64();
                        }
                    }
                }
                #[cfg(not(any(unix, windows)))]
                {
                    // Fallback to modified time
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                            return duration.as_secs_f64();
                        }
                    }
                }
            }
        }
        0.0
    }

    /// Get file permissions.
    // Python decorators: @staticmethod
    fn get_permissions(path: String) -> i64 {
        if Self::exists(path.clone()) {
            if let Ok(metadata) = fs::metadata(&path) {
                #[cfg(unix)]
                {
                    return metadata.permissions().mode() as i64;
                }
                #[cfg(not(unix))]
                {
                    // On non-Unix systems, return 0 as placeholder
                    return 0;
                }
            }
        }
        0
    }

    /// Check if file is readable.
    // Python decorators: @staticmethod
    fn is_readable(path: String) -> bool {
        Self::exists(path.clone()) && fs::File::open(&path).is_ok()
    }

    /// Check if file is writable.
    // Python decorators: @staticmethod
    fn is_writable(path: String) -> bool {
        if Self::exists(path.clone()) {
            fs::OpenOptions::new().write(true).open(&path).is_ok()
        } else {
            // Check if parent directory is writable
            if let Some(parent) = Path::new(&path).parent() {
                fs::OpenOptions::new().write(true).create(true).open(parent).is_ok()
            } else {
                false
            }
        }
    }

    /// Check if file is executable.
    // Python decorators: @staticmethod
    fn is_executable(path: String) -> bool {
        if !Self::exists(path.clone()) {
            return false;
        }
        #[cfg(unix)]
        {
            if let Ok(metadata) = fs::metadata(&path) {
                let mode = metadata.permissions().mode();
                return (mode & 0o111) != 0; // Check if any execute bit is set
            }
        }
        #[cfg(not(unix))]
        {
            // On non-Unix systems, check file extension
            let path = Path::new(&path);
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                return matches!(ext_str.as_str(), "exe" | "bat" | "cmd" | "com" | "scr");
            }
        }
        false
    }

    /// Read file as text.
    // Python decorators: @staticmethod
    fn read_text(path: String, encoding: String) -> String {
        // Note: Rust uses UTF-8 by default, encoding parameter is for compatibility
        fs::read_to_string(&path).unwrap_or_default()
    }

    /// Read file as bytes.
    // Python decorators: @staticmethod
    fn read_bytes(path: String) -> Vec<u8> {
        fs::read(&path).unwrap_or_default()
    }

    /// Write text to file.
    // Python decorators: @staticmethod
    fn write_text(path: String, content: String, encoding: String) -> bool {
        // Ensure parent directory exists
        if let Some(parent) = Path::new(&path).parent() {
            let _ = fs::create_dir_all(parent);
        }
        // Note: Rust uses UTF-8 by default, encoding parameter is for compatibility
        fs::write(&path, content).is_ok()
    }

    /// Write bytes to file.
    // Python decorators: @staticmethod
    fn write_bytes(path: String, content: Vec<u8>) -> bool {
        // Ensure parent directory exists
        if let Some(parent) = Path::new(&path).parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(&path, content).is_ok()
    }

    /// Safely read text file, returning None on error.
    // Python decorators: @staticmethod
    fn safe_read_text(path: String, encoding: String) -> Option<String> {
        fs::read_to_string(&path).ok()
    }

    /// Safely read binary file, returning None on error.
    // Python decorators: @staticmethod
    fn safe_read_bytes(path: String) -> Option<Vec<u8>> {
        fs::read(&path).ok()
    }

    /// Safely write text to file.
    // Python decorators: @staticmethod
    fn safe_write_text(path: String, content: String, encoding: String) -> bool {
        Self::write_text(path, content, encoding)
    }

    /// Safely write bytes to file.
    // Python decorators: @staticmethod
    fn safe_write_bytes(path: String, content: Vec<u8>) -> bool {
        Self::write_bytes(path, content)
    }

    /// Atomically write data to file (static version).
    // Python decorators: @staticmethod
    fn atomic_write(file_path: String, data: String, backup: bool) -> OperationResult {
        // Use simple atomic write pattern: write to temp, then rename
        let temp_path = format!("{}.tmp", file_path);
        match fs::write(&temp_path, data.as_bytes()) {
            Ok(_) => {
                // Create backup if requested
                if backup && Path::new(&file_path).exists() {
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let backup_path = format!("{}.backup.{}", file_path, timestamp);
                    let _ = fs::copy(&file_path, backup_path);
                }
                // Atomic move
                if fs::rename(&temp_path, &file_path).is_ok() {
                    OperationResult::Success
                } else {
                    let _ = fs::remove_file(&temp_path);
                    OperationResult::Failure
                }
            }
            Err(_) => OperationResult::Failure,
        }
    }

    /// Atomically copy file (static version).
    // Python decorators: @staticmethod
    fn atomic_copy(source: String, destination: String) -> OperationResult {
        match fs::read(&source) {
            Ok(data) => {
                let temp_path = format!("{}.tmp", destination);
                match fs::write(&temp_path, data) {
                    Ok(_) => {
                        if fs::rename(&temp_path, &destination).is_ok() {
                            OperationResult::Success
                        } else {
                            let _ = fs::remove_file(&temp_path);
                            OperationResult::Failure
                        }
                    }
                    Err(_) => OperationResult::Failure,
                }
            }
            Err(_) => OperationResult::Failure,
        }
    }

    /// Atomically move file (static version).
    // Python decorators: @staticmethod
    fn atomic_move(source: String, destination: String) -> OperationResult {
        let result = Self::atomic_copy(source.clone(), destination);
        if result == OperationResult::Success {
            if fs::remove_file(&source).is_ok() {
                OperationResult::Success
            } else {
                OperationResult::Failure
            }
        } else {
            OperationResult::Failure
        }
    }

    /// Atomically delete file (static version).
    // Python decorators: @staticmethod
    fn atomic_delete(file_path: String, backup: bool) -> OperationResult {
        let target = Path::new(&file_path);
        if backup && target.exists() {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if let Some(extension) = target.extension() {
                let backup_path = target.with_extension(
                    format!("{}.backup.{}", extension.to_string_lossy(), timestamp)
                );
                let _ = fs::copy(target, backup_path);
            }
        }
        if target.exists() {
            if fs::remove_file(target).is_ok() {
                OperationResult::Success
            } else {
                OperationResult::Failure
            }
        } else {
            OperationResult::Success
        }
    }

    /// Create backup of file (static version).
    // Python decorators: @staticmethod
    fn create_backup(source: String, backup_dir: String) -> Option<PathBuf> {
        let source_path = Path::new(&source);
        let backup_path = Path::new(&backup_dir);
        
        if fs::create_dir_all(backup_path).is_err() {
            return None;
        }
        
        if source_path.is_file() {
            if let Some(file_name) = source_path.file_name() {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let backup_file = backup_path.join(
                    format!("{}.backup.{}", file_name.to_string_lossy(), timestamp)
                );
                if fs::copy(source_path, &backup_file).is_ok() {
                    return Some(backup_file);
                }
            }
        }
        None
    }

    /// Restore from backup (static version).
    // Python decorators: @staticmethod
    fn restore_backup(backup_path: String, target: String) -> OperationResult {
        // Ensure target directory exists
        if let Some(parent) = Path::new(&target).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if fs::copy(&backup_path, &target).is_ok() {
            OperationResult::Success
        } else {
            OperationResult::Failure
        }
    }

    /// Create temporary file (static version).
    // Python decorators: @staticmethod
    fn create_temp_file(suffix: Option<String>, prefix: Option<String>) -> PathBuf {
        let mut path = std::env::temp_dir();
        let prefix = prefix.unwrap_or_else(|| "tmp".to_string());
        let suffix = suffix.unwrap_or_else(|| "".to_string());
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("{}{}{}", prefix, timestamp, suffix));
        // Create empty file
        let _ = fs::File::create(&path);
        path
    }

    /// Create temporary directory (static version).
    // Python decorators: @staticmethod
    fn create_temp_directory(suffix: Option<String>, prefix: Option<String>) -> PathBuf {
        let mut path = std::env::temp_dir();
        let prefix = prefix.unwrap_or_else(|| "tmp".to_string());
        let suffix = suffix.unwrap_or_else(|| "".to_string());
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("{}{}{}", prefix, timestamp, suffix));
        let _ = fs::create_dir_all(&path);
        path
    }

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Abstract base class for folder operations with both static and instance methods.
pub trait AFolder: IFolder {
    /// Create directory.
    fn create(&self, parents: bool, exist_ok: bool) -> bool;

    /// Delete directory.
    fn delete(&self, recursive: bool) -> bool;

    /// List files in directory.
    fn list_files(&self, pattern: Option<String>, recursive: bool) -> Vec<PathBuf> {
        // Default implementation delegates to static method
        // Subclasses should override with instance-specific path
        Vec::new()
    }

    /// List subdirectories.
    fn list_directories(&self, recursive: bool) -> Vec<PathBuf> {
        // Default implementation - subclasses should override
        Vec::new()
    }

    /// Walk directory tree.
    fn walk(&self) -> Vec<(PathBuf, Vec<String>, Vec<String>)> {
        // Default implementation - subclasses should override
        Vec::new()
    }

    /// Get directory size.
    fn get_size(&self) -> i64 {
        // Default implementation - subclasses should override
        0
    }

    /// Check if directory is empty.
    fn is_empty(&self) -> bool {
        // Default implementation - subclasses should override
        true
    }

    /// Copy directory to destination.
    fn copy_to(&self, destination: String) -> bool {
        // Default implementation - subclasses should override
        false
    }

    /// Move directory to destination.
    fn move_to(&self, destination: String) -> bool {
        // Default implementation - subclasses should override
        false
    }

    /// Check if directory exists.
    // Python decorators: @staticmethod
    fn exists(path: String) -> bool {
        let path = Path::new(&path);
        path.exists() && path.is_dir()
    }

    /// Create directory.
    // Python decorators: @staticmethod
    fn create_dir(path: String, parents: bool, exist_ok: bool) -> bool {
        let path = Path::new(&path);
        if path.exists() {
            return exist_ok;
        }
        if parents {
            fs::create_dir_all(path).is_ok()
        } else {
            fs::create_dir(path).is_ok()
        }
    }

    /// Delete directory.
    // Python decorators: @staticmethod
    fn delete_dir(path: String, recursive: bool) -> bool {
        let path = Path::new(&path);
        if recursive {
            fs::remove_dir_all(path).is_ok()
        } else {
            fs::remove_dir(path).is_ok()
        }
    }

    /// List files in directory.
    // Python decorators: @staticmethod
    fn list_files_static(path: String, pattern: Option<String>, recursive: bool) -> Vec<PathBuf> {
        if !Self::exists(path.clone()) {
            return Vec::new();
        }
        let path = Path::new(&path);
        let mut files = Vec::new();
        
        if recursive {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        if let Some(ref pat) = pattern {
                            if entry_path.to_string_lossy().contains(pat) {
                                files.push(entry_path);
                            }
                        } else {
                            files.push(entry_path);
                        }
                    } else if entry_path.is_dir() {
                        // Recursively list files in subdirectory
                        let sub_files = Self::list_files_static(
                            entry_path.to_string_lossy().to_string(),
                            pattern.clone(),
                            true
                        );
                        files.extend(sub_files);
                    }
                }
            }
        } else {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        if let Some(ref pat) = pattern {
                            if entry_path.to_string_lossy().contains(pat) {
                                files.push(entry_path);
                            }
                        } else {
                            files.push(entry_path);
                        }
                    }
                }
            }
        }
        files
    }

    /// List subdirectories.
    // Python decorators: @staticmethod
    fn list_directories_static(path: String, recursive: bool) -> Vec<PathBuf> {
        if !Self::exists(path.clone()) {
            return Vec::new();
        }
        let path = Path::new(&path);
        let mut dirs = Vec::new();
        
        if recursive {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() && entry_path != path {
                        dirs.push(entry_path.clone());
                        // Recursively list subdirectories
                        let sub_dirs = Self::list_directories_static(
                            entry_path.to_string_lossy().to_string(),
                            true
                        );
                        dirs.extend(sub_dirs);
                    }
                }
            }
        } else {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        dirs.push(entry_path);
                    }
                }
            }
        }
        dirs
    }

    /// Walk directory tree.
    // Python decorators: @staticmethod
    fn walk_static(path: String) -> Vec<(PathBuf, Vec<String>, Vec<String>)> {
        if !Self::exists(path.clone()) {
            return Vec::new();
        }
        let mut result = Vec::new();
        let path = Path::new(&path);
        
        if let Ok(entries) = fs::read_dir(path) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();
            
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    dirs.push(entry_path.file_name().unwrap().to_string_lossy().to_string());
                } else if entry_path.is_file() {
                    files.push(entry_path.file_name().unwrap().to_string_lossy().to_string());
                }
            }
            
            result.push((path.to_path_buf(), dirs, files));
            
            // Recursively walk subdirectories
            for entry in fs::read_dir(path).unwrap().flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    let sub_results = Self::walk_static(entry_path.to_string_lossy().to_string());
                    result.extend(sub_results);
                }
            }
        }
        result
    }

    /// Get directory size.
    // Python decorators: @staticmethod
    fn get_size_static(path: String) -> i64 {
        if !Self::exists(path.clone()) {
            return 0;
        }
        let path = Path::new(&path);
        let mut total_size = 0i64;
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Ok(metadata) = entry_path.metadata() {
                        total_size += metadata.len() as i64;
                    }
                } else if entry_path.is_dir() {
                    total_size += Self::get_size_static(entry_path.to_string_lossy().to_string());
                }
            }
        }
        total_size
    }

    /// Check if directory is empty.
    // Python decorators: @staticmethod
    fn is_empty_static(path: String) -> bool {
        if !Self::exists(path.clone()) {
            return true;
        }
        let path = Path::new(&path);
        if let Ok(mut entries) = fs::read_dir(path) {
            entries.next().is_none()
        } else {
            true
        }
    }

    /// Copy directory.
    // Python decorators: @staticmethod
    fn copy_dir(source: String, destination: String) -> bool {
        let source_path = Path::new(&source);
        let dest_path = Path::new(&destination);
        
        if !source_path.exists() || !source_path.is_dir() {
            return false;
        }
        
        // Create destination directory
        if fs::create_dir_all(dest_path).is_err() {
            return false;
        }
        
        // Copy all files and subdirectories
        if let Ok(entries) = fs::read_dir(source_path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let dest_entry = dest_path.join(entry_path.file_name().unwrap());
                
                if entry_path.is_file() {
                    if fs::copy(&entry_path, &dest_entry).is_err() {
                        return false;
                    }
                } else if entry_path.is_dir() {
                    if !Self::copy_dir(
                        entry_path.to_string_lossy().to_string(),
                        dest_entry.to_string_lossy().to_string()
                    ) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Move directory.
    // Python decorators: @staticmethod
    fn move_dir(source: String, destination: String) -> bool {
        // Try atomic move first
        if fs::rename(&source, &destination).is_ok() {
            return true;
        }
        // Fallback to copy + delete
        if Self::copy_dir(source.clone(), destination) {
            Self::delete_dir(source, true)
        } else {
            false
        }
    }

    /// Get directory permissions.
    // Python decorators: @staticmethod
    fn get_permissions(path: String) -> i64 {
        if Self::exists(path.clone()) {
            if let Ok(metadata) = fs::metadata(&path) {
                #[cfg(unix)]
                {
                    return metadata.permissions().mode() as i64;
                }
                #[cfg(not(unix))]
                {
                    return 0;
                }
            }
        }
        0
    }

    /// Check if directory is readable.
    // Python decorators: @staticmethod
    fn is_readable(path: String) -> bool {
        Self::exists(path.clone()) && fs::read_dir(&path).is_ok()
    }

    /// Check if directory is writable.
    // Python decorators: @staticmethod
    fn is_writable(path: String) -> bool {
        if !Self::exists(path.clone()) {
            return false;
        }
        // Try to create a test file
        let test_file = Path::new(&path).join(".write_test");
        let result = fs::File::create(&test_file).is_ok();
        let _ = fs::remove_file(&test_file);
        result
    }

    /// Check if directory is executable.
    // Python decorators: @staticmethod
    fn is_executable(path: String) -> bool {
        if !Self::exists(path.clone()) {
            return false;
        }
        #[cfg(unix)]
        {
            if let Ok(metadata) = fs::metadata(&path) {
                let mode = metadata.permissions().mode();
                return (mode & 0o111) != 0;
            }
        }
        #[cfg(not(unix))]
        {
            // On non-Unix systems, directories are generally "executable" (traversable)
            return true;
        }
        false
    }

}

// ============================================================================
// ============================================================================
/// Abstract base class for path operations with static methods.
pub trait APath: IPath {
    /// Normalize path.
    // Python decorators: @staticmethod
    fn normalize(path: String) -> PathBuf {
        Path::new(&path).canonicalize().unwrap_or_else(|_| Path::new(&path).to_path_buf())
    }

    /// Resolve path.
    // Python decorators: @staticmethod
    fn resolve(path: String) -> PathBuf {
        Path::new(&path).canonicalize().unwrap_or_else(|_| Path::new(&path).to_path_buf())
    }

    /// Get absolute path.
    // Python decorators: @staticmethod
    fn absolute(path: String) -> PathBuf {
        let path = Path::new(&path);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(path)
        }
    }

    /// Get relative path.
    // Python decorators: @staticmethod
    fn relative(path: String, start: Option<String>) -> PathBuf {
        let path = Path::new(&path).canonicalize().unwrap_or_else(|_| Path::new(&path).to_path_buf());
        let start_path = if let Some(s) = start {
            Path::new(&s).canonicalize().unwrap_or_else(|_| Path::new(&s).to_path_buf())
        } else {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        };
        
        // Simple relative path calculation
        if path.starts_with(&start_path) {
            path.strip_prefix(&start_path)
                .unwrap_or(&path)
                .to_path_buf()
        } else {
            path
        }
    }

    /// Join paths.
    // Python decorators: @staticmethod
    fn join() -> PathBuf {
        // Rust doesn't support variadic functions like Python, so this returns current dir
        // In practice, use PathBuf::join() directly
        PathBuf::from(".")
    }

    /// Split path into directory and filename.
    // Python decorators: @staticmethod
    fn split(path: String) -> (PathBuf, String) {
        let path = Path::new(&path);
        let parent = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "".to_string());
        (parent, name)
    }

    /// Get file extension.
    // Python decorators: @staticmethod
    fn get_extension(path: String) -> String {
        Path::new(&path)
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_else(|| "".to_string())
    }

    /// Get file stem (name without extension).
    // Python decorators: @staticmethod
    fn get_stem(path: String) -> String {
        Path::new(&path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "".to_string())
    }

    /// Get file/directory name.
    // Python decorators: @staticmethod
    fn get_name(path: String) -> String {
        Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "".to_string())
    }

    /// Get parent directory.
    // Python decorators: @staticmethod
    fn get_parent(path: String) -> PathBuf {
        Path::new(&path)
            .parent()
            .unwrap_or(Path::new("."))
            .to_path_buf()
    }

    /// Check if path is absolute.
    // Python decorators: @staticmethod
    fn is_absolute(path: String) -> bool {
        Path::new(&path).is_absolute()
    }

    /// Check if path is relative.
    // Python decorators: @staticmethod
    fn is_relative(path: String) -> bool {
        !Path::new(&path).is_absolute()
    }

    /// Get path parts.
    // Python decorators: @staticmethod
    fn get_parts(path: String) {
        // Note: Rust doesn't have a direct equivalent to Python's path.parts
        // This method signature returns () as it's not commonly used in Rust
        // In practice, use Path::components() directly
    }

    /// Check if path matches pattern.
    // Python decorators: @staticmethod
    fn match(path: String, pattern: String) -> bool {
        // Simple glob-like matching (basic implementation)
        // For full glob support, use a crate like glob
        let path_str = Path::new(&path).to_string_lossy();
        path_str.contains(&pattern)
    }

    /// Get path with new suffix.
    // Python decorators: @staticmethod
    fn with_suffix(path: String, suffix: String) -> PathBuf {
        Path::new(&path).with_extension(suffix)
    }

    /// Get path with new name.
    // Python decorators: @staticmethod
    fn with_name(path: String, name: String) -> PathBuf {
        Path::new(&path).with_file_name(name)
    }

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Abstract base class for stream operations with both static and instance methods.
pub trait AStream: IStream {
    /// Read from stream.
    fn read(&self, size: Option<i64>) -> String;

    /// Write to stream.
    fn write(&self, data: String) -> i64;

    /// Seek stream position.
    fn seek(&self, position: i64, whence: i64) -> i64;

    /// Get current stream position.
    fn tell(&self) -> i64;

    /// Flush stream buffer.
    fn flush(&self) -> ();

    /// Close stream.
    fn close(&self) {
        // Default implementation - subclasses should override if they have streams to close
    }

    /// Open file as stream.
    // Python decorators: @staticmethod
    fn open_file(path: String, mode: String, encoding: Option<String>) -> TextIO {
        // Note: encoding parameter is for compatibility, Rust uses UTF-8 by default
        fs::OpenOptions::new()
            .read(mode.contains('r'))
            .write(mode.contains('w') || mode.contains('a'))
            .append(mode.contains('a'))
            .create(mode.contains('w') || mode.contains('a'))
            .truncate(mode.contains('w'))
            .open(&path)
            .unwrap_or_else(|_| fs::File::create(&path).unwrap())
    }

    /// Check if stream is closed.
    // Python decorators: @staticmethod
    fn is_closed(_stream: TextIO) -> bool {
        // In Rust, we can't easily check if a File is closed without trying to use it
        // This is a placeholder - actual implementation would need to track state
        false
    }

    /// Check if stream is readable.
    // Python decorators: @staticmethod
    fn readable(_stream: TextIO) -> bool {
        // In Rust, File handles are readable if opened with read permissions
        // This is a placeholder - actual implementation would check metadata
        true
    }

    /// Check if stream is writable.
    // Python decorators: @staticmethod
    fn writable(_stream: TextIO) -> bool {
        // In Rust, File handles are writable if opened with write permissions
        // This is a placeholder - actual implementation would check metadata
        true
    }

    /// Check if stream is seekable.
    // Python decorators: @staticmethod
    fn seekable(_stream: TextIO) -> bool {
        // Regular files are seekable in Rust
        true
    }

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
// Lazy installation system will handle aiofiles if missing
// Lazy installation system will handle aiofiles if missing
// Lazy installation system will handle aiofiles if missing
// Lazy installation system will handle aiofiles if missing
// Lazy installation system will handle aiofiles if missing
/// Abstract base class for async I/O operations with both static and instance methods.
pub trait AAsyncIO: IAsyncIO {
    /// Async read operation.
    async fn aread(&self, size: Option<i64>) -> String;

    /// Async write operation.
    async fn awrite(&self, data: String) -> i64;

    /// Async seek operation.
    async fn aseek(&self, position: i64, whence: i64) -> i64;

    /// Async tell operation.
    async fn atell(&self) -> i64;

    /// Async flush operation.
    async fn aflush(&self) -> ();

    /// Async close operation.
    async fn aclose(&self) {
        // Default implementation - subclasses should override if they have async streams to close
    }

    /// Async open file.
    // Python decorators: @staticmethod
    async fn aopen_file(path: String, mode: String, encoding: Option<String>) -> serde_json::Value {
        // Note: For async file operations, use tokio::fs or async-std
        // This is a placeholder that returns a JSON representation
        serde_json::json!({
            "path": path,
            "mode": mode,
            "encoding": encoding
        })
    }

    /// Async read text file.
    // Python decorators: @staticmethod
    async fn aread_text(path: String, encoding: String) -> String {
        // Note: For actual async implementation, use tokio::fs::read_to_string
        // This is a synchronous fallback
        fs::read_to_string(&path).unwrap_or_default()
    }

    /// Async read binary file.
    // Python decorators: @staticmethod
    async fn aread_bytes(path: String) -> Vec<u8> {
        // Note: For actual async implementation, use tokio::fs::read
        // This is a synchronous fallback
        fs::read(&path).unwrap_or_default()
    }

    /// Async write text to file.
    // Python decorators: @staticmethod
    async fn awrite_text(path: String, content: String, encoding: String) -> bool {
        // Note: For actual async implementation, use tokio::fs::write
        // This is a synchronous fallback
        if let Some(parent) = Path::new(&path).parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(&path, content).is_ok()
    }

    /// Async write bytes to file.
    // Python decorators: @staticmethod
    async fn awrite_bytes(path: String, content: Vec<u8>) -> bool {
        // Note: For actual async implementation, use tokio::fs::write
        // This is a synchronous fallback
        if let Some(parent) = Path::new(&path).parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(&path, content).is_ok()
    }

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Abstract base class for atomic operations with both static and instance methods.
pub trait AAtomicOperations: IAtomicOperations {
    /// Atomically write data to file.
    fn atomic_write(&self, file_path: String, data: String, backup: bool) -> OperationResult;

    /// Atomically copy file.
    fn atomic_copy(&self, source: String, destination: String) -> OperationResult;

    /// Atomically move file.
    fn atomic_move(&self, source: String, destination: String) -> OperationResult;

    /// Atomically delete file.
    fn atomic_delete(&self, file_path: String, backup: bool) -> OperationResult;

    /// Atomically rename file.
    fn atomic_rename(&self, old_path: String, new_path: String) -> OperationResult;

    /// Atomically write data to file.
    // Python decorators: @staticmethod
    fn atomic_write_static(file_path: String, data: String, backup: bool) -> OperationResult {
        AFile::atomic_write(file_path, data, backup)
    }

    /// Atomically copy file.
    // Python decorators: @staticmethod
    fn atomic_copy_static(source: String, destination: String) -> OperationResult {
        AFile::atomic_copy(source, destination)
    }

    /// Atomically move file.
    // Python decorators: @staticmethod
    fn atomic_move_static(source: String, destination: String) -> OperationResult {
        AFile::atomic_move(source, destination)
    }

    /// Atomically delete file.
    // Python decorators: @staticmethod
    fn atomic_delete_static(file_path: String, backup: bool) -> OperationResult {
        AFile::atomic_delete(file_path, backup)
    }

    /// Atomically rename file.
    // Python decorators: @staticmethod
    fn atomic_rename_static(old_path: String, new_path: String) -> OperationResult {
        AFile::atomic_move(old_path, new_path)
    }

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Abstract base class for backup operations with both static and instance methods.
pub trait ABackupOperations: IBackupOperations {
    /// Create backup of file or directory.
    fn create_backup(&self, source: String, backup_dir: String) -> Option<PathBuf>;

    /// Restore from backup.
    fn restore_backup(&self, backup_path: String, target: String) -> OperationResult;

    /// List available backups.
    fn list_backups(&self, backup_dir: String) -> Vec<PathBuf> {
        Self::list_backups_static(backup_dir)
    }

    /// Cleanup old backups.
    fn cleanup_backups(&self, backup_dir: String, max_age_days: i64) -> i64 {
        Self::cleanup_backups_static(backup_dir, max_age_days)
    }

    /// Verify backup integrity.
    fn verify_backup(&self, backup_path: String) -> bool {
        Self::verify_backup_static(backup_path)
    }

    /// Create backup of file or directory.
    // Python decorators: @staticmethod
    fn create_backup_static(source: String, backup_dir: String) -> Option<PathBuf> {
        AFile::create_backup(source, backup_dir)
    }

    /// Restore from backup.
    // Python decorators: @staticmethod
    fn restore_backup_static(backup_path: String, target: String) -> OperationResult {
        AFile::restore_backup(backup_path, target)
    }

    /// List available backups.
    // Python decorators: @staticmethod
    fn list_backups_static(backup_dir: String) -> Vec<PathBuf> {
        let backup_path = Path::new(&backup_dir);
        if !backup_path.exists() {
            return Vec::new();
        }
        let mut backups = Vec::new();
        if let Ok(entries) = fs::read_dir(backup_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.to_string_lossy().contains(".backup.") {
                    backups.push(path);
                }
            }
        }
        backups
    }

    /// Cleanup old backups.
    // Python decorators: @staticmethod
    fn cleanup_backups_static(backup_dir: String, max_age_days: i64) -> i64 {
        let backup_path = Path::new(&backup_dir);
        if !backup_path.exists() {
            return 0;
        }
        let max_age_secs = max_age_days * 24 * 60 * 60;
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - max_age_secs as u64;
        
        let mut deleted = 0;
        if let Ok(entries) = fs::read_dir(backup_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.to_string_lossy().contains(".backup.") {
                    if let Ok(metadata) = path.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(age) = modified.duration_since(UNIX_EPOCH) {
                                if age.as_secs() < cutoff_time {
                                    if fs::remove_file(&path).is_ok() {
                                        deleted += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        deleted
    }

    /// Verify backup integrity.
    // Python decorators: @staticmethod
    fn verify_backup_static(backup_path: String) -> bool {
        let path = Path::new(&backup_path);
        path.exists() && path.is_file() && path.metadata().is_ok()
    }

}

// ============================================================================
// ============================================================================
// ============================================================================
// ============================================================================
/// Abstract base class for temporary operations with both static and instance methods.
pub trait ATemporaryOperations: ITemporaryOperations {
    /// Create temporary file.
    fn create_temp_file(&self, suffix: Option<String>, prefix: Option<String>) -> PathBuf;

    /// Create temporary directory.
    fn create_temp_directory(&self, suffix: Option<String>, prefix: Option<String>) -> PathBuf;

    /// Cleanup temporary file or directory.
    fn cleanup_temp(&self, path: String) -> bool {
        Self::cleanup_temp_static(path)
    }

    /// Cleanup all temporary files and directories.
    fn cleanup_all_temp(&self) -> i64 {
        // Default implementation - subclasses should track and cleanup their temp files
        0
    }

    /// Create temporary file.
    // Python decorators: @staticmethod
    fn create_temp_file_static(suffix: Option<String>, prefix: Option<String>) -> PathBuf {
        AFile::create_temp_file(suffix, prefix)
    }

    /// Create temporary directory.
    // Python decorators: @staticmethod
    fn create_temp_directory_static(suffix: Option<String>, prefix: Option<String>) -> PathBuf {
        AFile::create_temp_directory(suffix, prefix)
    }

    /// Cleanup temporary file or directory.
    // Python decorators: @staticmethod
    fn cleanup_temp_static(path: String) -> bool {
        let path = Path::new(&path);
        if path.is_file() {
            fs::remove_file(path).is_ok()
        } else if path.is_dir() {
            fs::remove_dir_all(path).is_ok()
        } else {
            false
        }
    }

    /// Get temporary base directory.
    // Python decorators: @staticmethod
    fn get_temp_base_dir() -> PathBuf {
        std::env::temp_dir()
    }

    /// Check if path is temporary.
    // Python decorators: @staticmethod
    fn is_temp(path: String) -> bool {
        let path = Path::new(&path);
        let temp_base = Self::get_temp_base_dir();
        path.starts_with(&temp_base)
    }

}

// Initialize parent classes
// Initialize without file path for general I/O operations
// Initialize xwsystem utilities
// Will be set by subclasses
// Will be set by subclasses
// Will be set by subclasses
// Track temporary files for cleanup
/// Abstract base class for unified I/O operations combining all existing I/O capabilities.
///
/// This abstract class combines all existing I/O abstract classes into a single,
/// comprehensive abstract base that provides complete I/O functionality with
/// xwsystem integration for security, validation, and monitoring.
///
/// Features:
/// - File operations (AFile)
/// - Directory operations (AFolder)
/// - Path operations (APath)
/// - Stream operations (AStream)
/// - Async operations (AAsyncIO)
/// - Atomic operations (AAtomicOperations)
/// - Backup operations (ABackupOperations)
/// - Temporary operations (ATemporaryOperations)
/// - xwsystem integration (security, validation, monitoring)
pub trait AUnifiedIO: AFile + AFolder + APath + AStream + AAsyncIO + AAtomicOperations + ABackupOperations + ATemporaryOperations {
}

// Initialize parent classes
// Initialize without base path for general file operations
// Initialize xwsystem utilities
// Will be set by subclasses
// Will be set by subclasses
// Will be set by subclasses
// Track temporary files for cleanup
/// Abstract base class for file manager operations combining all file-related capabilities.
///
/// This abstract class combines all file-related abstract classes into a single,
/// comprehensive abstract base that provides complete file management functionality
/// with xwsystem integration for security, validation, and monitoring.
///
/// Features:
/// - File operations (AFile)
/// - Directory operations (AFolder)
/// - Path operations (APath)
/// - Atomic operations (AAtomicOperations)
/// - Backup operations (ABackupOperations)
/// - Temporary operations (ATemporaryOperations)
/// - xwsystem integration (security, validation, monitoring)
/// - Format detection and intelligent handling
pub trait AFileManager: AFile + AFolder + APath + AAtomicOperations + ABackupOperations + ATemporaryOperations {
    /// Detect file type from extension and content.
    /// Args:
    /// file_path: Path to file
    /// Returns:
    /// Detected file type
    fn detect_file_type(&self, file_path: String) -> String {
        let path = Path::new(&file_path);
        if let Some(ext) = path.extension() {
            ext.to_string_lossy().to_lowercase()
        } else {
            "unknown".to_string()
        }
    }

    /// Get comprehensive file information.
    /// Args:
    /// file_path: Path to file
    /// Returns:
    /// File information dictionary
    fn get_file_info(&self, file_path: String) -> HashMap<String, serde_json::Value> {
        let mut info = HashMap::new();
        let path = Path::new(&file_path);
        
        info.insert("path".to_string(), serde_json::Value::String(file_path.clone()));
        info.insert("exists".to_string(), serde_json::Value::Bool(path.exists()));
        
        if let Ok(metadata) = fs::metadata(path) {
            info.insert("size".to_string(), serde_json::Value::Number(metadata.len().into()));
            info.insert("is_file".to_string(), serde_json::Value::Bool(metadata.is_file()));
            info.insert("is_dir".to_string(), serde_json::Value::Bool(metadata.is_dir()));
            
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                    info.insert("modified_time".to_string(), serde_json::Value::Number(
                        serde_json::Number::from_f64(duration.as_secs_f64()).unwrap()
                    ));
                }
            }
        }
        
        if let Some(ext) = path.extension() {
            info.insert("extension".to_string(), serde_json::Value::String(
                ext.to_string_lossy().to_string()
            ));
        }
        
        info
    }

    /// Check if file is safe to process (not too large, accessible, etc.).
    /// Args:
    /// file_path: Path to file
    /// Returns:
    /// True if safe to process
    fn is_safe_to_process(&self, file_path: String) -> bool {
        let path = Path::new(&file_path);
        if !path.exists() || !path.is_file() {
            return false;
        }
        
        // Check file size (limit to 1GB for safety)
        if let Ok(metadata) = path.metadata() {
            let max_size = 1024 * 1024 * 1024; // 1GB
            if metadata.len() > max_size {
                return false;
            }
        }
        
        // Check if readable
        fs::File::open(path).is_ok()
    }

}
