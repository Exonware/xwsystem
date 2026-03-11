// #exonware/xwsystem/rust/src/io/archive/archive_files.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/archive_files.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Archive FILES - File persistence for archives.
//! 
//! IArchiveFile extends IFile and USES IArchiver for compression.
//! 
//! Composition Pattern:
//! - ZipFile extends XWFile
//! - ZipFile USES ZipArchiver internally
//! - Separates file I/O from data transformation
//! 
//! Priority 1 (Security): Safe file operations
//! Priority 2 (Usability): Simple add_files/extract_to API
//! Priority 3 (Maintainability): Clear separation of concerns
//! Priority 4 (Performance): Efficient delegation
//! Priority 5 (Extensibility): Easy to add new formats


use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

use base64::{Engine as _, engine::general_purpose};

use crate::archive::base::{AArchiveFile};
use crate::archivers::{TarArchiver, ZipArchiver};
use crate::contracts::{IArchiveFile, IArchiver, IFile};
use crate::errors::{ArchiveError};
use crate::base::AFile;
use crate::defs::FileMode;

// Compress using archiver (in RAM)
// Save to disk using direct write
// Load from disk using direct read
// Extract using archiver (in RAM)
// Write to destination folder
/// Zip archive FILE - follows I→A→XW pattern.
///
/// I: IArchiveFile (interface)
/// A: AArchiveFile (abstract base)
/// XW: ZipFile (concrete implementation)
///
/// USES XWZipArchiver internally (composition).
///
/// This handles:
/// - File I/O with .zip files on disk
/// - Adding files to archive
/// - Extracting archive to destination
///
/// Examples:
/// >>> # Create zip file
/// >>> zip_file = ZipFile("backup.zip")
/// >>>
/// >>> # Add files to archive
/// >>> zip_file.add_files([Path("file1.txt"), Path("file2.txt")])
/// >>>
/// >>> # Extract archive
/// >>> extracted = zip_file.extract_to(Path("output/"))
/// >>>
/// >>> # List contents
/// >>> files = zip_file.list_contents()
pub struct ZipFile {
    pub path: PathBuf,
    archiver: ZipArchiver,
}

impl ZipFile {
    /// Initialize zip archive file.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            archiver: ZipArchiver::new(),
        }
    }
}

impl IFile for ZipFile {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn exists(&self) -> bool {
        self.path.exists() && self.path.is_file()
    }

    fn size(&self) -> i64 {
        if self.exists() {
            if let Ok(metadata) = fs::metadata(&self.path) {
                return metadata.len() as i64;
            }
        }
        0
    }
}

impl IArchiveFile for ZipFile {
    fn add_files(&self, files: Vec<PathBuf>, options: HashMap<String, String>) {
        self.add_files_impl(&files, options).unwrap();
    }

    fn extract_to(&self, dest: PathBuf, options: HashMap<String, String>) -> Vec<PathBuf> {
        self.extract_to_impl(&dest, options).unwrap()
    }

    fn list_contents(&self) -> Vec<String> {
        self.list_contents_impl().unwrap()
    }

    fn get_archiver(&self) -> Box<dyn IArchiver> {
        Box::new(ZipArchiver::new())
    }
}

impl AFile for ZipFile {
    fn open(&self, _mode: crate::defs::FileMode) {
        // Archive files don't need explicit open
    }

    fn read(&self, _size: Option<i64>) -> String {
        // For archive files, use list_contents or extract_to instead
        String::new()
    }

    fn write(&self, _data: String) -> i64 {
        // For archive files, use add_files instead
        0
    }

    fn save(&self, _data: serde_json::Value) -> bool {
        // For archive files, use add_files instead
        false
    }

    fn load(&self) -> serde_json::Value {
        // For archive files, use extract_to instead
        serde_json::Value::Null
    }

    fn save_as(&self, _path: String, _data: serde_json::Value) -> bool {
        false
    }

    fn to_file(&self, _path: String) -> bool {
        false
    }

    fn from_file(&self, _path: String) -> String {
        String::new()
    }
}

impl AArchiveFile for ZipFile {
    fn get_archiver(&self) -> Box<dyn IArchiver> {
        Box::new(ZipArchiver::new())
    }

    fn add_files(&self, files: Vec<PathBuf>) {
        self.add_files_impl(&files, HashMap::new()).unwrap();
    }

    fn extract_to(&self, dest: PathBuf) -> Vec<PathBuf> {
        self.extract_to_impl(&dest, HashMap::new()).unwrap()
    }

    fn list_contents(&self) -> Vec<String> {
        self.list_contents_impl().unwrap()
    }
}

impl ZipFile {
    // Compress using archiver (in RAM)
    // Save to disk using direct write
    /// Add files to zip archive.
    ///
    /// Uses archiver internally:
    /// 1. Read files from disk
    /// 2. Use ZipArchiver.compress() to create archive bytes
    /// 3. Save to disk using XWFile.save()
    fn add_files_impl(&self, files: &[PathBuf], options: HashMap<String, String>) -> Result<(), ArchiveError> {
        // Read files
        let mut file_data = serde_json::Map::new();
        for file_path in files {
            if file_path.is_file() {
                let filename = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| ArchiveError::new("Invalid file name"))?;
                
                let content = fs::read(file_path).map_err(|e| {
                    ArchiveError::new(format!("Failed to read file '{}': {}", file_path.display(), e))
                })?;
                
                file_data.insert(filename.to_string(), serde_json::Value::String(
                    general_purpose::STANDARD.encode(&content)
                ));
            }
        }

        // Compress using archiver (in RAM)
        let zip_bytes = self.archiver.compress(serde_json::Value::Object(file_data), options);

        // Save to disk using direct write
        fs::write(&self.path, zip_bytes).map_err(|e| {
            ArchiveError::new(format!("Failed to write zip archive: {}", e))
        })?;

        Ok(())
    }

    // Load from disk using direct read
    // Extract using archiver (in RAM)
    // Write to destination folder
    /// Extract zip archive to destination.
    ///
    /// Uses archiver internally:
    /// 1. Load from disk using XWFile.load()
    /// 2. Use ZipArchiver.extract() to get data
    /// 3. Write files to destination
    fn extract_to_impl(&self, dest: &Path, options: HashMap<String, String>) -> Result<Vec<PathBuf>, ArchiveError> {
        // Load from disk using direct read
        let zip_bytes = fs::read(&self.path).map_err(|e| {
            ArchiveError::new(format!("Failed to read zip archive: {}", e))
        })?;

        // Extract using archiver (in RAM)
        let file_data = self.archiver.decode(zip_bytes);

        // Write to destination folder
        fs::create_dir_all(dest).map_err(|e| {
            ArchiveError::new(format!("Failed to create destination directory: {}", e))
        })?;

        let mut extracted_files = Vec::new();

        if let serde_json::Value::Object(map) = file_data {
            for (filename, content) in map {
                let file_path = dest.join(&filename);
                
                // Create parent directory if needed
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).map_err(|e| {
                        ArchiveError::new(format!("Failed to create directory: {}", e))
                    })?;
                }

                // Decode content (base64 encoded)
                let content_bytes = if let serde_json::Value::String(s) = content {
                    general_purpose::STANDARD.decode(&s).map_err(|e| {
                        ArchiveError::new(format!("Failed to decode file content: {}", e))
                    })?
                } else {
                    return Err(ArchiveError::new("Invalid content format".to_string()));
                };

                fs::write(&file_path, content_bytes).map_err(|e| {
                    ArchiveError::new(format!("Failed to write file '{}': {}", file_path.display(), e))
                })?;

                extracted_files.push(file_path);
            }
        }

        Ok(extracted_files)
    }

    /// List files in zip archive.
    fn list_contents_impl(&self) -> Result<Vec<String>, ArchiveError> {
        self.archiver.list_contents(&self.path)
    }
}

// Compress using archiver (in RAM)
// Save to disk using direct write
// Load from disk using direct read
// Extract using archiver (in RAM)
// Write to destination folder
/// Tar archive FILE - follows I→A→XW pattern.
///
/// I: IArchiveFile (interface)
/// A: AArchiveFile (abstract base)
/// XW: TarFile (concrete implementation)
///
/// USES XWTarArchiver internally (composition).
///
/// Similar to ZipFile but for tar format.
/// Supports compression: gzip, bz2, xz
pub struct TarFile {
    pub path: PathBuf,
    pub compression: String,
    archiver: TarArchiver,
}

impl TarFile {
    /// Initialize tar archive file.
    ///
    ///
    /// Args:
    /// path: Archive file path
    /// compression: Compression type ('', 'gz', 'bz2', 'xz')
    pub fn new(path: impl Into<PathBuf>, compression: Option<String>) -> Self {
        Self {
            path: path.into(),
            compression: compression.unwrap_or_default(),
            archiver: TarArchiver::new(),
        }
    }
}

impl IFile for TarFile {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn exists(&self) -> bool {
        self.path.exists() && self.path.is_file()
    }

    fn size(&self) -> i64 {
        if self.exists() {
            if let Ok(metadata) = fs::metadata(&self.path) {
                return metadata.len() as i64;
            }
        }
        0
    }
}

impl IArchiveFile for TarFile {
    fn add_files(&self, files: Vec<PathBuf>, options: HashMap<String, String>) {
        self.add_files_impl(&files, options).unwrap();
    }

    fn extract_to(&self, dest: PathBuf, options: HashMap<String, String>) -> Vec<PathBuf> {
        self.extract_to_impl(&dest, options).unwrap()
    }

    fn list_contents(&self) -> Vec<String> {
        self.list_contents_impl().unwrap()
    }

    fn get_archiver(&self) -> Box<dyn IArchiver> {
        Box::new(TarArchiver::new())
    }
}

impl AFile for TarFile {
    fn open(&self, _mode: FileMode) {
        // Archive files don't need explicit open
    }

    fn read(&self, _size: Option<i64>) -> String {
        String::new()
    }

    fn write(&self, _data: String) -> i64 {
        0
    }

    fn save(&self, _data: serde_json::Value) -> bool {
        false
    }

    fn load(&self) -> serde_json::Value {
        serde_json::Value::Null
    }

    fn save_as(&self, _path: String, _data: serde_json::Value) -> bool {
        false
    }

    fn to_file(&self, _path: String) -> bool {
        false
    }

    fn from_file(&self, _path: String) -> String {
        String::new()
    }
}

impl AArchiveFile for TarFile {
    fn get_archiver(&self) -> Box<dyn IArchiver> {
        Box::new(TarArchiver::new())
    }

    fn add_files(&self, files: Vec<PathBuf>) {
        let mut options = HashMap::new();
        options.insert("compression".to_string(), self.compression.clone());
        self.add_files_impl(&files, options).unwrap();
    }

    fn extract_to(&self, dest: PathBuf) -> Vec<PathBuf> {
        self.extract_to_impl(&dest, HashMap::new()).unwrap()
    }

    fn list_contents(&self) -> Vec<String> {
        self.list_contents_impl().unwrap()
    }
}

impl TarFile {
    // Compress using archiver (in RAM)
    // Save to disk using direct write
    /// Add files to tar archive (uses archiver internally).
    fn add_files_impl(&self, files: &[PathBuf], mut options: HashMap<String, String>) -> Result<(), ArchiveError> {
        // Read files
        let mut file_data = serde_json::Map::new();
        for file_path in files {
            if file_path.is_file() {
                let filename = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| ArchiveError::new("Invalid file name"))?;
                
                let content = fs::read(file_path).map_err(|e| {
                    ArchiveError::new(format!("Failed to read file '{}': {}", file_path.display(), e))
                })?;
                
                file_data.insert(filename.to_string(), serde_json::Value::String(
                    general_purpose::STANDARD.encode(&content)
                ));
            }
        }

        // Compress using archiver (in RAM)
        options.insert("compression".to_string(), self.compression.clone());
        let tar_bytes = self.archiver.compress(serde_json::Value::Object(file_data), options);

        // Save to disk using direct write
        fs::write(&self.path, tar_bytes).map_err(|e| {
            ArchiveError::new(format!("Failed to write tar archive: {}", e))
        })?;

        Ok(())
    }

    // Load from disk using direct read
    // Extract using archiver (in RAM)
    // Write to destination folder
    /// Extract tar archive to destination (uses archiver internally).
    fn extract_to_impl(&self, dest: &Path, _options: HashMap<String, String>) -> Result<Vec<PathBuf>, ArchiveError> {
        // Load from disk using direct read
        let tar_bytes = fs::read(&self.path).map_err(|e| {
            ArchiveError::new(format!("Failed to read tar archive: {}", e))
        })?;

        // Extract using archiver (in RAM)
        let file_data = self.archiver.decode(tar_bytes);

        // Write to destination folder
        fs::create_dir_all(dest).map_err(|e| {
            ArchiveError::new(format!("Failed to create destination directory: {}", e))
        })?;

        let mut extracted_files = Vec::new();

        if let serde_json::Value::Object(map) = file_data {
            for (filename, content) in map {
                let file_path = dest.join(&filename);
                
                // Create parent directory if needed
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).map_err(|e| {
                        ArchiveError::new(format!("Failed to create directory: {}", e))
                    })?;
                }

                // Decode content (base64 encoded)
                let content_bytes = if let serde_json::Value::String(s) = content {
                    general_purpose::STANDARD.decode(&s).map_err(|e| {
                        ArchiveError::new(format!("Failed to decode file content: {}", e))
                    })?
                } else {
                    return Err(ArchiveError::new("Invalid content format".to_string()));
                };

                fs::write(&file_path, content_bytes).map_err(|e| {
                    ArchiveError::new(format!("Failed to write file '{}': {}", file_path.display(), e))
                })?;

                extracted_files.push(file_path);
            }
        }

        Ok(extracted_files)
    }

    /// List files in tar archive.
    fn list_contents_impl(&self) -> Result<Vec<String>, ArchiveError> {
        self.archiver.list_contents(&self.path)
    }
}
