// #exonware/xwsystem/rust/src/io/archive/formats/zpaq_format.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/zpaq_format.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! ZPAQ journaled compression format - RANK #8 EXTREME COMPRESSION.
//! 
//! **Extreme compression ratio, slow, archival only**
//! 
//! Priority 1 (Security): Journaled integrity
//! Priority 2 (Usability): Extreme compression
//! Priority 3 (Maintainability): Clean zpaq handling
//! Priority 4 (Performance): Slow (archival only)
//! Priority 5 (Extensibility): Lazy installation of zpaq

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;

// ZPAQ supports wildcard extraction
// Return extracted files
// Parse zpaq list output
/// ZPAQ archive format handler - RANK #8.
///
/// FormatAction naming: ZpaqArchiver
///
/// Extreme archival compression with:
/// - Best compression ratio (PAQ algorithm)
/// - Journaled incremental backups
/// - Deduplication
/// - Very slow (archival only)
///
/// NOTE: Requires zpaq binary installed on system.
///
/// Examples:
/// >>> archiver = ZpaqArchiver()
/// >>> # Create archive (very slow!)
/// >>> archiver.create([Path("data/")], Path("archive.zpaq"))
/// >>>
/// >>> # Extract
/// >>> archiver.extract(Path("archive.zpaq"), Path("output/"))
pub struct ZpaqArchiver;

impl IArchiveFormat for ZpaqArchiver {
    fn format_id(&self) -> &str {
        "zpaq"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec!["zpaq".to_string()]
    }

    fn mime_types(&self) -> Vec<String> {
        vec!["application/x-zpaq".to_string()]
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

impl ZpaqArchiver {
    pub fn new() -> Self {
        ZpaqArchiver
    }

    /// Check if zpaq binary is available.
    pub fn _check_zpaq(&self) -> PathBuf {
        if Command::new("zpaq").arg("--help").output().is_err() {
            panic!("zpaq command not found. Please install zpaq (http://mattmahoney.net/dc/zpaq.html)");
        }
        PathBuf::from("zpaq")
    }

    /// Create ZPAQ archive.
    ///
    /// Options:
    /// method: Compression method (0-5, default: 3)
    /// incremental: Enable incremental backup
    pub fn create_with_opts(&self, files: Vec<PathBuf>, output: PathBuf, opts: HashMap<String, String>) -> () {
        self._check_zpaq();

        if let Some(parent) = output.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                panic!("Failed to create archive directory: {}", e);
            }
        }

        let method: u32 = opts.get("method")
            .and_then(|s| s.parse().ok())
            .unwrap_or(3)
            .min(5);

        let mut cmd = Command::new("zpaq");
        cmd.arg("add");
        cmd.arg(&output);
        
        if opts.get("incremental").and_then(|s| s.parse().ok()).unwrap_or(false) {
            cmd.arg("-method").arg(method.to_string());
        } else {
            cmd.arg("-method").arg(method.to_string());
        }
        
        for file in &files {
            cmd.arg(file);
        }

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute zpaq command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("zpaq creation failed: {}", stderr);
        }
    }

    // ZPAQ supports wildcard extraction
    // Return extracted files
    /// Extract ZPAQ archive.
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, _opts: HashMap<String, String>) -> Vec<PathBuf> {
        self._check_zpaq();

        std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
            panic!("Failed to create output directory: {}", e);
        });

        let mut cmd = Command::new("zpaq");
        cmd.arg("extract");
        cmd.arg(&archive);
        cmd.arg(&output_dir);
        
        if let Some(members_list) = members {
            for member in members_list {
                cmd.arg(member);
            }
        }

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute zpaq command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("zpaq extraction failed: {}", stderr);
        }

        vec![output_dir]
    }

    // Parse zpaq list output
    /// List ZPAQ contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        self._check_zpaq();

        let mut cmd = Command::new("zpaq");
        cmd.arg("list");
        cmd.arg(archive);

        let output = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute zpaq command: {}", e);
        });

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("zpaq list failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut names = Vec::new();
        
        // Parse zpaq list output
        for line in stdout.lines() {
            if !line.starts_with("zpaq") && !line.starts_with("----") && !line.is_empty() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    // Extract filename from zpaq output format
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if !parts.is_empty() {
                        names.push(parts.last().unwrap().to_string());
                    }
                }
            }
        }

        names
    }

    /// Add file to ZPAQ archive (incremental).
    pub fn add_file_impl(&self, archive: &Path, file: &Path, _arcname: Option<String>) -> () {
        self._check_zpaq();

        let mut cmd = Command::new("zpaq");
        cmd.arg("add");
        cmd.arg(archive);
        cmd.arg(file);

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute zpaq command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("zpaq add failed: {}", stderr);
        }
    }
}

impl Default for ZpaqArchiver {
    fn default() -> Self {
        Self::new()
    }
}
