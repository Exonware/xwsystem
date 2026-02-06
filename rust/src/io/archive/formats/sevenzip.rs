// #exonware/xwsystem/rust/src/io/archive/formats/sevenzip.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/sevenzip.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! 7z archive format implementation - RANK #1 BEST COMPRESSION.
//! 
//! **Best overall ratio + AES-256 encryption + solid archiving**
//! 
//! Priority 1 (Security): AES-256 encryption support
//! Priority 2 (Usability): Best compression ratio
//! Priority 3 (Maintainability): Clean 7z handling
//! Priority 4 (Performance): LZMA2 compression
//! Priority 5 (Extensibility): Lazy installation of py7zr

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;

// Return extracted files
/// 7z archive format handler - RANK #1.
///
/// FormatAction naming: SevenZipArchiver
///
/// Best overall compression with:
/// - LZMA2 compression (best ratio)
/// - AES-256 encryption
/// - Solid archiving (better compression)
/// - Multi-volume archives
///
/// Examples:
/// >>> archiver = SevenZipArchiver()
/// >>> archiver.create([Path("data.txt")], Path("backup.7z"))
/// >>>
/// >>> # With encryption
/// >>> archiver.create(files, output, password="secret123")
/// >>>
/// >>> # With solid compression
/// >>> archiver.create(files, output, solid=True)
pub struct SevenZipArchiver;

impl IArchiveFormat for SevenZipArchiver {
    fn format_id(&self) -> &str {
        "7z"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec!["7z".to_string()]
    }

    fn mime_types(&self) -> Vec<String> {
        vec!["application/x-7z-compressed".to_string()]
    }

    fn create(&self, files: Vec<PathBuf>, output: PathBuf) -> () {
        self.create_with_opts(files, output, HashMap::new())
    }

    fn extract(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>) -> Vec<PathBuf> {
        self.extract_with_opts(archive, output_dir, members, HashMap::new())
    }

    fn list_contents(&self, archive: PathBuf) -> Vec<String> {
        self.list_contents_impl(&archive)
    }

    fn add_file(&self, archive: PathBuf, file: PathBuf, arcname: Option<String>) -> () {
        self.add_file_impl(&archive, &file, arcname)
    }
}

impl SevenZipArchiver {
    pub fn new() -> Self {
        SevenZipArchiver
    }

    /// Create 7z archive with optional encryption.
    ///
    /// Options:
    /// password: Encryption password (AES-256)
    /// solid: Solid archiving for better compression
    /// filters: Custom LZMA2 filters
    pub fn create_with_opts(&self, files: Vec<PathBuf>, output: PathBuf, opts: HashMap<String, String>) -> () {
        // Check if 7z command is available
        if Command::new("7z").arg("--help").output().is_err() {
            panic!("7z command not found. Please install 7-Zip (https://www.7-zip.org/)");
        }

        if let Some(parent) = output.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                panic!("Failed to create archive directory: {}", e);
            }
        }

        let mut cmd = Command::new("7z");
        cmd.arg("a"); // Add to archive
        cmd.arg("-y"); // Assume Yes on all queries
        
        if let Some(password) = opts.get("password") {
            cmd.arg(format!("-p{}", password));
        }
        
        if opts.get("solid").and_then(|s| s.parse().ok()).unwrap_or(false) {
            cmd.arg("-ms=on"); // Solid archive
        }
        
        cmd.arg(&output);
        
        for file in &files {
            cmd.arg(file);
        }

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute 7z command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("7z command failed: {}", stderr);
        }
    }

    // Return extracted files
    /// Extract 7z archive.
    ///
    /// Options:
    /// password: Decryption password
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, opts: HashMap<String, String>) -> Vec<PathBuf> {
        if Command::new("7z").arg("--help").output().is_err() {
            panic!("7z command not found. Please install 7-Zip (https://www.7-zip.org/)");
        }

        std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
            panic!("Failed to create output directory: {}", e);
        });

        let mut cmd = Command::new("7z");
        cmd.arg("x"); // Extract with full paths
        cmd.arg("-y"); // Assume Yes on all queries
        cmd.arg("-o").arg(&output_dir);
        
        if let Some(password) = opts.get("password") {
            cmd.arg(format!("-p{}", password));
        }
        
        if let Some(members_list) = members {
            for member in members_list {
                cmd.arg(member);
            }
        }
        
        cmd.arg(&archive);

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute 7z command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("7z extraction failed: {}", stderr);
        }

        // Return list of extracted files (simplified - would need to parse 7z output)
        vec![output_dir]
    }

    /// List 7z contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        if Command::new("7z").arg("--help").output().is_err() {
            panic!("7z command not found. Please install 7-Zip (https://www.7-zip.org/)");
        }

        let mut cmd = Command::new("7z");
        cmd.arg("l"); // List
        cmd.arg("-slt"); // Technical information
        cmd.arg(archive);

        let output = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute 7z command: {}", e);
        });

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("7z list failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut names = Vec::new();
        
        // Parse 7z output (simplified - would need proper parsing)
        for line in stdout.lines() {
            if line.starts_with("Path = ") {
                let path = line.strip_prefix("Path = ").unwrap_or("").to_string();
                if !path.is_empty() {
                    names.push(path);
                }
            }
        }

        names
    }

    /// Add file to 7z archive (append mode).
    pub fn add_file_impl(&self, archive: &Path, file: &Path, _arcname: Option<String>) -> () {
        if Command::new("7z").arg("--help").output().is_err() {
            panic!("7z command not found. Please install 7-Zip (https://www.7-zip.org/)");
        }

        let mut cmd = Command::new("7z");
        cmd.arg("a"); // Add to archive
        cmd.arg("-y"); // Assume Yes on all queries
        cmd.arg(archive);
        cmd.arg(file);

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute 7z command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("7z add failed: {}", stderr);
        }
    }
}

impl Default for SevenZipArchiver {
    fn default() -> Self {
        Self::new()
    }
}
