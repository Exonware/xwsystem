// #exonware/xwsystem/rust/src/io/archive/formats/rar.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/rar.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! RAR5 archive format implementation - RANK #3 PROPRIETARY COMPRESSION.
//! 
//! **Strong compression + recovery record + multi-volume**
//! 
//! Priority 1 (Security): Strong encryption
//! Priority 2 (Usability): Recovery records
//! Priority 3 (Maintainability): Clean RAR handling
//! Priority 4 (Performance): Strong compression
//! Priority 5 (Extensibility): Optional rarfile dependency

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;

/// RAR5 archive format handler - RANK #3.
///
/// FormatAction naming: RarArchiver
///
/// NOTE: Extraction only (requires UnRAR binary for creation).
/// RAR is proprietary - use for extraction of existing RAR files.
///
/// Features:
/// - Strong compression
/// - Recovery records
/// - Multi-volume archives
/// - AES encryption
///
/// Examples:
/// >>> archiver = RarArchiver()
/// >>> # Extract existing RAR
/// >>> archiver.extract(Path("archive.rar"), Path("output/"))
/// >>>
/// >>> # With password
/// >>> archiver.extract(archive, output, password="secret")
pub struct RarArchiver;

impl IArchiveFormat for RarArchiver {
    fn format_id(&self) -> &str {
        "rar"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec!["rar".to_string(), "r00".to_string(), "r01".to_string()]
    }

    fn mime_types(&self) -> Vec<String> {
        vec!["application/x-rar-compressed".to_string(), "application/vnd.rar".to_string()]
    }

    fn create(&self, _files: Vec<PathBuf>, _output: PathBuf) -> () {
        panic!("RAR creation not supported (proprietary format). Use 7z or Zstandard for compression instead.");
    }

    fn extract(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>) -> Vec<PathBuf> {
        self.extract_with_opts(archive, output_dir, members, HashMap::new())
    }

    fn list_contents(&self, archive: PathBuf) -> Vec<String> {
        self.list_contents_impl(&archive)
    }

    fn add_file(&self, _archive: PathBuf, _file: PathBuf, _arcname: Option<String>) -> () {
        panic!("RAR append not supported (requires WinRAR binary).");
    }
}

impl RarArchiver {
    pub fn new() -> Self {
        RarArchiver
    }

    /// RAR creation not supported (proprietary).
    ///
    /// Use 7z or Zstandard for compression instead.
    pub fn create_with_opts(&self, _files: Vec<PathBuf>, _output: PathBuf, _opts: HashMap<String, String>) -> () {
        panic!("RAR creation not supported (proprietary format). Use 7z or Zstandard for compression instead.");
    }

    /// Extract RAR archive.
    ///
    /// Options:
    /// password: Decryption password
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, opts: HashMap<String, String>) -> Vec<PathBuf> {
        // Try unrar command (Windows/Linux)
        let unrar_cmd = if cfg!(windows) {
            "unrar"
        } else {
            "unrar"
        };

        if Command::new(unrar_cmd).arg("--help").output().is_err() {
            panic!("unrar command not found. Please install UnRAR (https://www.rarlab.com/rar_add.htm)");
        }

        std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
            panic!("Failed to create output directory: {}", e);
        });

        let mut cmd = Command::new(unrar_cmd);
        cmd.arg("x"); // Extract with full paths
        cmd.arg("-y"); // Assume Yes on all queries
        cmd.arg(&archive);
        cmd.arg(&output_dir);
        
        if let Some(password) = opts.get("password") {
            cmd.arg(format!("-p{}", password));
        }
        
        if let Some(members_list) = members {
            for member in members_list {
                cmd.arg(member);
            }
        }

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute unrar command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("unrar extraction failed: {}", stderr);
        }

        vec![output_dir]
    }

    /// List RAR contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        let unrar_cmd = if cfg!(windows) {
            "unrar"
        } else {
            "unrar"
        };

        if Command::new(unrar_cmd).arg("--help").output().is_err() {
            panic!("unrar command not found. Please install UnRAR");
        }

        let mut cmd = Command::new(unrar_cmd);
        cmd.arg("l"); // List
        cmd.arg(archive);

        let output = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute unrar command: {}", e);
        });

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("unrar list failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut names = Vec::new();
        
        // Parse unrar output (simplified)
        for line in stdout.lines() {
            if line.contains("  ") && !line.starts_with("Archive:") && !line.starts_with("----") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let name = parts[4..].join(" ");
                    if !name.is_empty() {
                        names.push(name);
                    }
                }
            }
        }

        names
    }

    /// RAR append not supported (requires WinRAR binary).
    pub fn add_file_impl(&self, _archive: &Path, _file: &Path, _arcname: Option<String>) -> () {
        panic!("RAR append not supported (requires WinRAR binary).");
    }
}

impl Default for RarArchiver {
    fn default() -> Self {
        Self::new()
    }
}
