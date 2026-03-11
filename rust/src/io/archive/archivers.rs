// #exonware/xwsystem/rust/src/io/archive/archivers.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/archivers.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Archive codecs - In-memory archive processors.
//! 
//! Following FormatAction naming: ZipArchiver, TarArchiver, 7zArchiver
//! 
//! IArchiver extends ICodec:
//! - Works on ANY data in RAM (not just files!)
//! - compress() / extract() delegates to encode() / decode()
//! 
//! Priority 1 (Security): Safe archive operations with validation
//! Priority 2 (Usability): Simple compress/extract API
//! Priority 3 (Maintainability): Clean codec pattern
//! Priority 4 (Performance): Efficient in-memory operations
//! Priority 5 (Extensibility): Easy to add new formats (7z, RAR, etc.)


use std::collections::HashMap;
use std::io::{Read, Write, Cursor};
use std::fs::File;
use std::path::{Path, PathBuf};

use zip::write::{FileOptions, ZipWriter};
use zip::read::ZipArchive;
use zip::CompressionMethod;
use base64::{Engine as _, engine::general_purpose};

use tar::{Builder, Archive as TarArchive, Header};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression as Flate2Compression;
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression as Bzip2Compression;
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;
use xz2::stream::Stream as XzStream;

use crate::archive::base::{AArchiver};
use crate::contracts::{DecodeOptions, EncodeOptions, IArchiver, ICodec, IArchiveFormat};
use crate::defs::{ArchiveFormat, CodecCapability, CodecCategory};
use crate::errors::{ArchiveError, DecodeError, EncodeError};
use std::path::PathBuf;

// ----------------------------------------------------------------------
// File-based convenience API (used by archive facade + tests)
// ----------------------------------------------------------------------
// dict: keys are filenames, values are contents
// Raw bytes: create single file
// String: create single text file
// Other: serialize as string
// Extract zip from memory
/// Zip archive codec - operates in MEMORY.
///
/// Follows I→A→XW pattern:
/// - I: IArchiver (interface)
/// - A: AArchiver (abstract base)
/// - XW: XWZipArchiver (concrete implementation)
///
/// Can compress:
/// - bytes (raw data)
/// - str (text)
/// - dict/list (structured data)
/// - Any Python objects
///
/// NOT limited to files - works on data in RAM!
///
/// Examples:
/// >>> archiver = ZipArchiver()
/// >>>
/// >>> # Compress dict in RAM
/// >>> data = {"file1.txt": b"content1", "file2.txt": b"content2"}
/// >>> zip_bytes = archiver.compress(data)
/// >>>
/// >>> # Extract from RAM
/// >>> extracted = archiver.extract(zip_bytes)
/// >>>
/// >>> # Or use codec methods
/// >>> zip_bytes = archiver.encode(data)
/// >>> data = archiver.decode(zip_bytes)
pub struct ZipArchiver;

impl ZipArchiver {
    pub fn new() -> Self {
        ZipArchiver
    }
}

impl Default for ZipArchiver {
    fn default() -> Self {
        Self::new()
    }
}

impl ICodec for ZipArchiver {
    fn encode(&self, value: serde_json::Value) -> Vec<u8> {
        AArchiver::encode(self, value)
    }

    fn decode(&self, repr: Vec<u8>) -> serde_json::Value {
        AArchiver::decode(self, repr)
    }
}

impl AArchiver for ZipArchiver {
    fn encode(&self, value: serde_json::Value) -> Vec<u8> {
        Self::encode_impl(value)
    }

    fn decode(&self, repr: Vec<u8>) -> serde_json::Value {
        Self::decode_impl(repr)
    }

    fn codec_id(&self) -> &str {
        "zip"
    }

    fn media_types(&self) -> Vec<String> {
        vec!["application/zip".to_string(), "application/x-zip-compressed".to_string()]
    }

    fn file_extensions(&self) -> Vec<String> {
        vec![".zip".to_string()]
    }

    fn capabilities(&self) -> CodecCapability {
        CodecCapability::BIDIRECTIONAL
    }

    fn aliases(&self) -> Vec<String> {
        vec!["zip".to_string(), "ZIP".to_string()]
    }

    fn codec_types(&self) -> Vec<String> {
        vec!["archive".to_string()]
    }

    fn compress(&self, data: serde_json::Value) -> Vec<u8> {
        self.encode(data)
    }

    fn extract(&self, archive_bytes: Vec<u8>) -> serde_json::Value {
        self.decode(archive_bytes)
    }
}

impl IArchiveFormat for ZipArchiver {
    fn format_id(&self) -> &str {
        self.codec_id()
    }

    fn file_extensions(&self) -> Vec<String> {
        AArchiver::file_extensions(self)
    }

    fn mime_types(&self) -> Vec<String> {
        AArchiver::media_types(self)
    }

    fn create(&self, files: Vec<PathBuf>, output: PathBuf) {
        let file_refs: Vec<&std::path::Path> = files.iter().map(|p| p.as_path()).collect();
        let _ = self.create(file_refs, &output, None);
    }

    fn extract(&self, archive: PathBuf, output_dir: PathBuf, _members: Option<Vec<String>>) -> Vec<PathBuf> {
        // For now, extract all members (members filtering not implemented)
        let result = self.extract(archive.to_string_lossy().to_string(), Some(&output_dir), HashMap::new());
        match result {
            Ok(serde_json::Value::Array(paths)) => {
                paths.iter()
                    .filter_map(|p| p.as_str())
                    .map(|s| PathBuf::from(s))
                    .collect()
            },
            _ => vec![],
        }
    }

    fn list_contents(&self, archive: PathBuf) -> Vec<String> {
        self.list_contents(&archive).unwrap_or_default()
    }

    fn add_file(&self, archive: PathBuf, file: PathBuf, _arcname: Option<String>) {
        let _ = self.add_file(&archive, &file, None);
    }
}

impl ZipArchiver {
    /// Check capability support.
    pub fn supports_capability(&self, capability: CodecCapability) -> bool {
        capability.contains(CodecCapability::BIDIRECTIONAL) || 
        capability.contains(CodecCapability::COMPRESSION)
    }

    /// Format identifier alias (compat with archive facade/tests).
    pub fn format_id(&self) -> &str {
        self.codec_id()
    }

    /// Codec category: ARCHIVE.
    pub fn category(&self) -> CodecCategory {
        CodecCategory::Archive
    }

    /// Create a ZIP archive on disk from a list of files.
    pub fn create(&self, files: Vec<&Path>, archive_path: &Path, compression: Option<i64>) -> Result<PathBuf, ArchiveError> {
        // Create parent directory if needed
        if let Some(parent) = archive_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                ArchiveError::new(format!("Failed to create archive directory: {}", e))
            })?;
        }

        let file = File::create(archive_path).map_err(|e| {
            ArchiveError::new(format!("Failed to create zip archive '{}': {}", archive_path.display(), e))
        })?;

        let compression_method = if compression.unwrap_or(8) > 0 {
            CompressionMethod::Deflated
        } else {
            CompressionMethod::Stored
        };
        let options = FileOptions::default().compression_method(compression_method);

        let mut zip = ZipWriter::new(file);
        
        for file_path in files {
            if file_path.is_file() {
                let file_name = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| ArchiveError::new("Invalid file name"))?;
                
                zip.start_file(file_name, options).map_err(|e| {
                    ArchiveError::new(format!("Failed to add file to zip: {}", e))
                })?;
                
                let mut file_content = File::open(file_path).map_err(|e| {
                    ArchiveError::new(format!("Failed to read file '{}': {}", file_path.display(), e))
                })?;
                
                std::io::copy(&mut file_content, &mut zip).map_err(|e| {
                    ArchiveError::new(format!("Failed to write file to zip: {}", e))
                })?;
            }
        }

        zip.finish().map_err(|e| {
            ArchiveError::new(format!("Failed to finalize zip archive: {}", e))
        })?;

        Ok(archive_path.to_path_buf())
    }

    /// List contents of a ZIP archive on disk.
    pub fn list_contents(&self, archive_path: &Path) -> Result<Vec<String>, ArchiveError> {
        let file = File::open(archive_path).map_err(|e| {
            ArchiveError::new(format!("Failed to open zip archive '{}': {}", archive_path.display(), e))
        })?;

        let mut zip = ZipArchive::new(file).map_err(|e| {
            ArchiveError::new(format!("Failed to read zip archive '{}': {}", archive_path.display(), e))
        })?;

        let mut names = Vec::new();
        for i in 0..zip.len() {
            let file = zip.by_index(i).map_err(|e| {
                ArchiveError::new(format!("Failed to read zip entry: {}", e))
            })?;
            names.push(file.name().to_string());
        }

        Ok(names)
    }

    /// Add a file to an existing ZIP archive.
    pub fn add_file(&self, archive_path: &Path, file_path: &Path, compression: Option<i64>) -> Result<(), ArchiveError> {
        // Read existing archive
        let file = File::open(archive_path).map_err(|e| {
            ArchiveError::new(format!("Failed to open zip archive '{}': {}", archive_path.display(), e))
        })?;

        let mut zip = ZipArchive::new(file).map_err(|e| {
            ArchiveError::new(format!("Failed to read zip archive '{}': {}", archive_path.display(), e))
        })?;

        // Create new archive with existing files + new file
        let new_file = File::create(archive_path).map_err(|e| {
            ArchiveError::new(format!("Failed to create zip archive '{}': {}", archive_path.display(), e))
        })?;

        let compression_method = if compression.unwrap_or(8) > 0 {
            CompressionMethod::Deflated
        } else {
            CompressionMethod::Stored
        };
        let options = FileOptions::default().compression_method(compression_method);

        let mut new_zip = ZipWriter::new(new_file);

        // Copy existing files
        for i in 0..zip.len() {
            let mut file_entry = zip.by_index(i).map_err(|e| {
                ArchiveError::new(format!("Failed to read zip entry: {}", e))
            })?;
            
            new_zip.start_file(file_entry.name(), options).map_err(|e| {
                ArchiveError::new(format!("Failed to add existing file to zip: {}", e))
            })?;
            
            std::io::copy(&mut file_entry, &mut new_zip).map_err(|e| {
                ArchiveError::new(format!("Failed to copy file to new zip: {}", e))
            })?;
        }

        // Add new file
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| ArchiveError::new("Invalid file name"))?;
        
        new_zip.start_file(file_name, options).map_err(|e| {
            ArchiveError::new(format!("Failed to add file to zip: {}", e))
        })?;
        
        let mut file_content = File::open(file_path).map_err(|e| {
            ArchiveError::new(format!("Failed to read file '{}': {}", file_path.display(), e))
        })?;
        
        std::io::copy(&mut file_content, &mut new_zip).map_err(|e| {
            ArchiveError::new(format!("Failed to write file to zip: {}", e))
        })?;

        new_zip.finish().map_err(|e| {
            ArchiveError::new(format!("Failed to finalize zip archive: {}", e))
        })?;

        Ok(())
    }

    // dict: keys are filenames, values are contents
    // Raw bytes: create single file
    // String: create single text file
    // Other: serialize as string
    /// Encode data to zip bytes (in RAM).
    ///
    ///
    /// Args:
    /// value: Data to archive (dict, bytes, list, etc.)
    /// options: Compression options
    ///
    ///
    /// Returns:
    /// Zip archive bytes
    fn encode_impl(value: serde_json::Value) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut zip = ZipWriter::new(Cursor::new(&mut buffer));
        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

        match value {
            serde_json::Value::Object(map) => {
                // dict: keys are filenames, values are contents
                for (filename, content) in map {
                    let content_bytes = match content {
                        serde_json::Value::String(s) => s.into_bytes(),
                        serde_json::Value::Array(arr) => {
                            serde_json::to_vec(&arr).unwrap_or_default()
                        },
                        serde_json::Value::Object(obj) => {
                            serde_json::to_vec(&obj).unwrap_or_default()
                        },
                        _ => content.to_string().into_bytes(),
                    };
                    
                    zip.start_file(&filename, options).unwrap();
                    zip.write_all(&content_bytes).unwrap();
                }
            },
            serde_json::Value::String(s) => {
                // String: create single text file
                zip.start_file("data.txt", options).unwrap();
                zip.write_all(s.as_bytes()).unwrap();
            },
            serde_json::Value::Array(arr) => {
                // Array: serialize as JSON
                let content = serde_json::to_string(&arr).unwrap_or_default();
                zip.start_file("data.json", options).unwrap();
                zip.write_all(content.as_bytes()).unwrap();
            },
            _ => {
                // Other: serialize as string
                let content = value.to_string();
                zip.start_file("data", options).unwrap();
                zip.write_all(content.as_bytes()).unwrap();
            },
        }

        zip.finish().unwrap();
        buffer
    }

    // Extract zip from memory
    /// Decode zip bytes to data (in RAM).
    ///
    ///
    /// Args:
    /// repr: Zip archive bytes
    /// options: Extraction options
    ///
    ///
    /// Returns:
    /// Extracted data (dict of filename → content)
    fn decode_impl(repr: Vec<u8>) -> serde_json::Value {
        let cursor = Cursor::new(repr);
        let mut zip = ZipArchive::new(cursor).unwrap();
        let mut result = serde_json::Map::new();

        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let filename = file.name().to_string();
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();
            
            // Try to parse as JSON, otherwise store as string
            match serde_json::from_slice::<serde_json::Value>(&contents) {
                Ok(json_value) => {
                    result.insert(filename, json_value);
                },
                Err(_) => {
                    // Store as string if not valid JSON
                    match String::from_utf8(contents) {
                        Ok(s) => {
                            result.insert(filename, serde_json::Value::String(s));
                        },
                        Err(_) => {
                            // Store as base64 if not valid UTF-8
                            result.insert(filename, serde_json::Value::String(
                                general_purpose::STANDARD.encode(&contents)
                            ));
                        },
                    }
                },
            }
        }

        serde_json::Value::Object(result)
    }

    /// User-friendly: Compress data to zip bytes.
    ///
    /// Delegates to encode().
    pub fn compress(&self, data: serde_json::Value, _options: HashMap<String, String>) -> Vec<u8> {
        self.encode(data)
    }

    /// Extract ZIP from either in-memory bytes or a file path.
    ///
    /// - If source is bytes: returns dict[str, bytes] via decode()
    /// - If source is Path: extracts to extract_dir and returns list[Path]
    pub fn extract(&self, source: String, extract_dir: Option<&Path>, _options: HashMap<String, String>) -> Result<serde_json::Value, ArchiveError> {
        // Check if source is a valid file path
        let path = Path::new(&source);
        if path.exists() && path.is_file() {
            // Treat as file path
            let extract_dir = extract_dir.ok_or_else(|| {
                ArchiveError::new("extract_dir is required when extracting from a file path".to_string())
            })?;

        std::fs::create_dir_all(extract_dir).map_err(|e| {
            ArchiveError::new(format!("Failed to create extract directory: {}", e))
        })?;

        let file = File::open(path).map_err(|e| {
            ArchiveError::new(format!("Failed to open zip archive '{}': {}", path.display(), e))
        })?;

        let mut zip = ZipArchive::new(file).map_err(|e| {
            ArchiveError::new(format!("Failed to read zip archive '{}': {}", path.display(), e))
        })?;

        let mut extracted_paths = Vec::new();

        for i in 0..zip.len() {
            let mut file_entry = zip.by_index(i).map_err(|e| {
                ArchiveError::new(format!("Failed to read zip entry: {}", e))
            })?;

            let file_path = extract_dir.join(file_entry.name());
            
            // Create parent directory if needed
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    ArchiveError::new(format!("Failed to create directory: {}", e))
                })?;
            }

            let mut outfile = File::create(&file_path).map_err(|e| {
                ArchiveError::new(format!("Failed to create file '{}': {}", file_path.display(), e))
            })?;

            std::io::copy(&mut file_entry, &mut outfile).map_err(|e| {
                ArchiveError::new(format!("Failed to extract file: {}", e))
            })?;

            extracted_paths.push(file_path.to_string_lossy().to_string());
        }

            Ok(serde_json::json!(extracted_paths))
        } else {
            // Try to parse as bytes (base64 encoded or raw)
            // First try base64 decode
            if let Ok(bytes) = general_purpose::STANDARD.decode(&source) {
                Ok(self.decode(bytes))
            } else {
                // Try as raw bytes (UTF-8 string converted to bytes)
                Ok(self.decode(source.into_bytes()))
            }
        }
    }

}

// ----------------------------------------------------------------------
// File-based convenience API (used by archive facade + tests)
// ----------------------------------------------------------------------
// '', 'gz', 'bz2', 'xz'
/// Tar archive codec - operates in MEMORY.
///
/// Follows I→A→XW pattern:
/// - I: IArchiver (interface)
/// - A: AArchiver (abstract base)
/// - XW: XWTarArchiver (concrete implementation)
///
/// Similar to XWZipArchiver but uses tar format.
/// Supports compression: gzip, bz2, xz
pub struct TarArchiver;

impl TarArchiver {
    pub fn new() -> Self {
        TarArchiver
    }
}

impl Default for TarArchiver {
    fn default() -> Self {
        Self::new()
    }
}

impl ICodec for TarArchiver {
    fn encode(&self, value: serde_json::Value) -> Vec<u8> {
        AArchiver::encode(self, value)
    }

    fn decode(&self, repr: Vec<u8>) -> serde_json::Value {
        AArchiver::decode(self, repr)
    }
}

impl AArchiver for TarArchiver {
    fn encode(&self, value: serde_json::Value) -> Vec<u8> {
        Self::encode_impl(value, "")
    }

    fn decode(&self, repr: Vec<u8>) -> serde_json::Value {
        Self::decode_impl(repr)
    }

    fn codec_id(&self) -> &str {
        "tar"
    }

    fn media_types(&self) -> Vec<String> {
        vec!["application/x-tar".to_string(), "application/x-gtar".to_string()]
    }

    fn file_extensions(&self) -> Vec<String> {
        vec![".tar".to_string(), ".tar.gz".to_string(), ".tgz".to_string(), 
             ".tar.bz2".to_string(), ".tar.xz".to_string()]
    }

    fn capabilities(&self) -> CodecCapability {
        CodecCapability::BIDIRECTIONAL
    }

    fn aliases(&self) -> Vec<String> {
        vec!["tar".to_string(), "TAR".to_string()]
    }

    fn codec_types(&self) -> Vec<String> {
        vec!["archive".to_string()]
    }

    fn compress(&self, data: serde_json::Value) -> Vec<u8> {
        self.encode(data)
    }

    fn extract(&self, archive_bytes: Vec<u8>) -> serde_json::Value {
        self.decode(archive_bytes)
    }
}

impl IArchiveFormat for TarArchiver {
    fn format_id(&self) -> &str {
        self.codec_id()
    }

    fn file_extensions(&self) -> Vec<String> {
        AArchiver::file_extensions(self)
    }

    fn mime_types(&self) -> Vec<String> {
        AArchiver::media_types(self)
    }

    fn create(&self, files: Vec<PathBuf>, output: PathBuf) {
        let file_refs: Vec<&std::path::Path> = files.iter().map(|p| p.as_path()).collect();
        let _ = self.create(file_refs, &output, None);
    }

    fn extract(&self, archive: PathBuf, output_dir: PathBuf, _members: Option<Vec<String>>) -> Vec<PathBuf> {
        // For now, extract all members (members filtering not implemented)
        let result = self.extract(archive.to_string_lossy().to_string(), Some(&output_dir), HashMap::new());
        match result {
            Ok(serde_json::Value::Array(paths)) => {
                paths.iter()
                    .filter_map(|p| p.as_str())
                    .map(|s| PathBuf::from(s))
                    .collect()
            },
            _ => vec![],
        }
    }

    fn list_contents(&self, archive: PathBuf) -> Vec<String> {
        self.list_contents(&archive).unwrap_or_default()
    }

    fn add_file(&self, archive: PathBuf, file: PathBuf, _arcname: Option<String>) {
        let _ = self.add_file(&archive, &file);
    }
}

impl TarArchiver {
    /// Check capability support.
    pub fn supports_capability(&self, capability: CodecCapability) -> bool {
        capability.contains(CodecCapability::BIDIRECTIONAL) || 
        capability.contains(CodecCapability::COMPRESSION)
    }

    /// Format identifier alias (compat with archive facade/tests).
    pub fn format_id(&self) -> &str {
        self.codec_id()
    }

    /// Codec category: ARCHIVE.
    pub fn category(&self) -> CodecCategory {
        CodecCategory::Archive
    }

    /// Create a TAR archive on disk from a list of files.
    pub fn create(&self, files: Vec<&Path>, archive_path: &Path, mode: Option<String>) -> Result<PathBuf, ArchiveError> {
        // Create parent directory if needed
        if let Some(parent) = archive_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                ArchiveError::new(format!("Failed to create archive directory: {}", e))
            })?;
        }

        let file = File::create(archive_path).map_err(|e| {
            ArchiveError::new(format!("Failed to create tar archive '{}': {}", archive_path.display(), e))
        })?;

        let mut tar = Builder::new(file);
        
        for file_path in files {
            if file_path.is_file() {
                let file_name = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| ArchiveError::new("Invalid file name"))?;
                
                tar.append_path_with_name(file_path, file_name).map_err(|e| {
                    ArchiveError::new(format!("Failed to add file to tar: {}", e))
                })?;
            }
        }

        tar.finish().map_err(|e| {
            ArchiveError::new(format!("Failed to finalize tar archive: {}", e))
        })?;

        Ok(archive_path.to_path_buf())
    }

    /// List contents of a TAR archive on disk.
    pub fn list_contents(&self, archive_path: &Path) -> Result<Vec<String>, ArchiveError> {
        let file = File::open(archive_path).map_err(|e| {
            ArchiveError::new(format!("Failed to open tar archive '{}': {}", archive_path.display(), e))
        })?;

        // Try to detect compression and decompress
        let mut archive = Self::open_tar_archive(file)?;

        let mut names = Vec::new();
        for entry in archive.entries().map_err(|e| {
            ArchiveError::new(format!("Failed to read tar archive '{}': {}", archive_path.display(), e))
        })? {
            let entry = entry.map_err(|e| {
                ArchiveError::new(format!("Failed to read tar entry: {}", e))
            })?;
            if let Some(name) = entry.path().ok().and_then(|p| p.to_str()) {
                names.push(name.to_string());
            }
        }

        Ok(names)
    }

    /// Add a file to an existing TAR archive (append mode).
    pub fn add_file(&self, archive_path: &Path, file_path: &Path) -> Result<(), ArchiveError> {
        let file = File::open(archive_path).map_err(|e| {
            ArchiveError::new(format!("Failed to open tar archive '{}': {}", archive_path.display(), e))
        })?;

        let mut archive = Self::open_tar_archive(file)?;
        
        // Note: tar crate doesn't support append mode directly
        // We need to read all entries, then write a new archive
        // For now, we'll create a new archive with the new file
        // This is a limitation of the tar crate
        Err(ArchiveError::new("Append mode not fully supported - use create() to add files".to_string()))
    }

    // Helper to open tar archive with compression detection
    fn open_tar_archive(file: File) -> Result<TarArchive<Box<dyn Read>>, ArchiveError> {
        // Try to detect compression by reading first few bytes
        let mut buffer = [0u8; 2];
        let mut file_reader = std::io::BufReader::new(file);
        file_reader.read_exact(&mut buffer).map_err(|_| {
            ArchiveError::new("Failed to read archive header".to_string())
        })?;

        // Reset file position
        let file = file_reader.into_inner();
        let file = file.into_inner();

        // Check compression type
        if buffer == [0x1f, 0x8b] {
            // gzip
            Ok(TarArchive::new(Box::new(GzDecoder::new(file))))
        } else if buffer.starts_with(b"BZ") {
            // bzip2
            Ok(TarArchive::new(Box::new(BzDecoder::new(file))))
        } else if buffer == [0xfd, 0x37] {
            // xz (check more bytes)
            let mut more_buffer = [0u8; 4];
            let mut file_reader = std::io::BufReader::new(file);
            file_reader.read_exact(&mut more_buffer).ok();
            if more_buffer == [0x7a, 0x58, 0x5a, 0x00] {
                let file = file_reader.into_inner().into_inner();
                Ok(TarArchive::new(Box::new(XzDecoder::new(file))))
            } else {
                let file = file_reader.into_inner().into_inner();
                Ok(TarArchive::new(Box::new(file)))
            }
        } else {
            // No compression
            Ok(TarArchive::new(Box::new(file)))
        }
    }

    // '', 'gz', 'bz2', 'xz'
    /// Encode data to tar bytes (in RAM).
    fn encode_impl(value: serde_json::Value, compression: &str) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut tar = Builder::new(&mut buffer);

        match value {
            serde_json::Value::Object(map) => {
                for (filename, content) in map {
                    let content_bytes = match content {
                        serde_json::Value::String(s) => s.into_bytes(),
                        serde_json::Value::Array(arr) => {
                            serde_json::to_vec(&arr).unwrap_or_default()
                        },
                        serde_json::Value::Object(obj) => {
                            serde_json::to_vec(&obj).unwrap_or_default()
                        },
                        _ => content.to_string().into_bytes(),
                    };
                    
                    let mut header = tar::Header::new_gnu();
                    header.set_path(&filename).unwrap();
                    header.set_size(content_bytes.len() as u64);
                    header.set_cksum();
                    tar.append(&header, &content_bytes[..]).unwrap();
                }
            },
            serde_json::Value::String(s) => {
                let content = s.into_bytes();
                let mut header = tar::Header::new_gnu();
                header.set_path("data.txt").unwrap();
                header.set_size(content.len() as u64);
                header.set_cksum();
                tar.append(&header, &content[..]).unwrap();
            },
            _ => {
                let content = value.to_string().into_bytes();
                let mut header = tar::Header::new_gnu();
                header.set_path("data").unwrap();
                header.set_size(content.len() as u64);
                header.set_cksum();
                tar.append(&header, &content[..]).unwrap();
            },
        }

        tar.finish().unwrap();

        // Apply compression if specified
        match compression {
            "gz" => {
                let mut encoder = GzEncoder::new(Vec::new(), Flate2Compression::default());
                encoder.write_all(&buffer).unwrap();
                encoder.finish().unwrap()
            },
            "bz2" => {
                let mut encoder = BzEncoder::new(Vec::new(), Bzip2Compression::default());
                encoder.write_all(&buffer).unwrap();
                encoder.finish().unwrap()
            },
            "xz" => {
                let mut encoder = XzEncoder::new_stream(Vec::new(), &XzStream::new_easy_encoder(6, xz2::stream::Check::None).unwrap());
                encoder.write_all(&buffer).unwrap();
                encoder.finish().unwrap()
            },
            _ => buffer,
        }
    }

    /// Decode tar bytes to data (in RAM).
    fn decode_impl(repr: Vec<u8>) -> serde_json::Value {
        let cursor = Cursor::new(repr);
        // Try to detect compression
        let archive: TarArchive<Box<dyn Read>> = if repr.len() >= 2 && repr[0] == 0x1f && repr[1] == 0x8b {
            TarArchive::new(Box::new(GzDecoder::new(cursor)))
        } else if repr.len() >= 2 && repr.starts_with(b"BZ") {
            TarArchive::new(Box::new(BzDecoder::new(cursor)))
        } else if repr.len() >= 6 && repr[0] == 0xfd && repr[1] == 0x37 && repr[2] == 0x7a && repr[3] == 0x58 && repr[4] == 0x5a && repr[5] == 0x00 {
            TarArchive::new(Box::new(XzDecoder::new(cursor)))
        } else {
            TarArchive::new(Box::new(cursor))
        };

        let mut result = serde_json::Map::new();

        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();
            if entry.header().entry_type().is_file() {
                let path = entry.path().unwrap();
                let filename = path.to_string_lossy().to_string();
                let mut contents = Vec::new();
                entry.read_to_end(&mut contents).unwrap();
                
                // Try to parse as JSON, otherwise store as string
                match serde_json::from_slice::<serde_json::Value>(&contents) {
                    Ok(json_value) => {
                        result.insert(filename, json_value);
                    },
                    Err(_) => {
                        match String::from_utf8(contents) {
                            Ok(s) => {
                                result.insert(filename, serde_json::Value::String(s));
                            },
                            Err(_) => {
                                result.insert(filename, serde_json::Value::String(
                                    general_purpose::STANDARD.encode(&contents)
                                ));
                            },
                        }
                    },
                }
            }
        }

        serde_json::Value::Object(result)
    }

    /// User-friendly: Compress data to tar bytes.
    pub fn compress(&self, data: serde_json::Value, options: HashMap<String, String>) -> Vec<u8> {
        let compression = options.get("compression").map(|s| s.as_str()).unwrap_or("");
        Self::encode_impl(data, compression)
    }

    /// Extract TAR from either in-memory bytes or a file path.
    ///
    /// - If source is bytes: returns dict[str, bytes] via decode()
    /// - If source is Path: extracts to extract_dir and returns list[Path]
    pub fn extract(&self, source: String, extract_dir: Option<&Path>, _options: HashMap<String, String>) -> Result<serde_json::Value, ArchiveError> {
        // Check if source is a valid file path
        let path = Path::new(&source);
        if path.exists() && path.is_file() {
            // Treat as file path
            let extract_dir = extract_dir.ok_or_else(|| {
                ArchiveError::new("extract_dir is required when extracting from a file path".to_string())
            })?;

            std::fs::create_dir_all(extract_dir).map_err(|e| {
                ArchiveError::new(format!("Failed to create extract directory: {}", e))
            })?;

            let file = File::open(path).map_err(|e| {
                ArchiveError::new(format!("Failed to open tar archive '{}': {}", path.display(), e))
            })?;

            let mut archive = Self::open_tar_archive(file)?;

            let mut extracted_paths = Vec::new();

            for entry in archive.entries().map_err(|e| {
                ArchiveError::new(format!("Failed to read tar archive '{}': {}", path.display(), e))
            })? {
                let mut entry = entry.map_err(|e| {
                    ArchiveError::new(format!("Failed to read tar entry: {}", e))
                })?;

                let file_path = extract_dir.join(entry.path().map_err(|e| {
                    ArchiveError::new(format!("Failed to get entry path: {}", e))
                })?);
                
                // Create parent directory if needed
                if let Some(parent) = file_path.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        ArchiveError::new(format!("Failed to create directory: {}", e))
                    })?;
                }

                if entry.header().entry_type().is_file() {
                    let mut outfile = File::create(&file_path).map_err(|e| {
                        ArchiveError::new(format!("Failed to create file '{}': {}", file_path.display(), e))
                    })?;

                    std::io::copy(&mut entry, &mut outfile).map_err(|e| {
                        ArchiveError::new(format!("Failed to extract file: {}", e))
                    })?;

                    extracted_paths.push(file_path.to_string_lossy().to_string());
                }
            }

            Ok(serde_json::json!(extracted_paths))
        } else {
            // Try to parse as bytes (base64 encoded or raw)
            if let Ok(bytes) = general_purpose::STANDARD.decode(&source) {
                Ok(self.decode(bytes))
            } else {
                Ok(self.decode(source.into_bytes()))
            }
        }
    }

}
