// #exonware/xwsystem/rust/src/io/archive/formats/squashfs_format.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/squashfs_format.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! SquashFS filesystem format - RANK #10 EMBEDDED SYSTEMS.
//! 
//! **Embedded & system image use**
//! 
//! Priority 1 (Security): Read-only filesystem security
//! Priority 2 (Usability): Embedded systems
//! Priority 3 (Maintainability): Clean squashfs handling
//! Priority 4 (Performance): LZMA/LZ4 compression
//! Priority 5 (Extensibility): Binary tool integration

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;

// Build mksquashfs command
// Extract specific files
// Return extracted files
// Parse unsquashfs output
// Skip header lines and parse file list
/// SquashFS filesystem format handler - RANK #10.
///
/// FormatAction naming: SquashfsArchiver
///
/// Read-only compressed filesystem with:
/// - LZMA/LZ4/LZO compression
/// - Block deduplication
/// - Extended attributes
/// - Used in embedded systems, live CDs
///
/// NOTE: Requires mksquashfs/unsquashfs binaries.
///
/// Examples:
/// >>> archiver = SquashfsArchiver()
/// >>> # Create filesystem image
/// >>> archiver.create([Path("rootfs/")], Path("system.squashfs"))
/// >>>
/// >>> # Extract filesystem
/// >>> archiver.extract(Path("system.squashfs"), Path("output/"))
pub struct SquashfsArchiver;

impl IArchiveFormat for SquashfsArchiver {
    fn format_id(&self) -> &str {
        "squashfs"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec!["squashfs".to_string(), "sqfs".to_string()]
    }

    fn mime_types(&self) -> Vec<String> {
        vec!["application/x-squashfs".to_string()]
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

    fn add_file(&self, _archive: PathBuf, _file: PathBuf, _arcname: Option<String>) -> () {
        panic!("SquashFS doesn't support append (read-only FS)");
    }
}

impl SquashfsArchiver {
    pub fn new() -> Self {
        SquashfsArchiver
    }

    /// Check if squashfs tools are available.
    pub fn _check_tools(&self) -> () {
        if Command::new("mksquashfs").arg("--help").output().is_err() {
            panic!("mksquashfs command not found. Please install squashfs-tools");
        }
        if Command::new("unsquashfs").arg("--help").output().is_err() {
            panic!("unsquashfs command not found. Please install squashfs-tools");
        }
    }

    // Build mksquashfs command
    /// Create SquashFS filesystem.
    ///
    /// Options:
    /// comp: Compression algorithm (gzip, lzma, lzo, lz4, xz, zstd)
    /// block_size: Block size (default: 128K)
    pub fn create_with_opts(&self, files: Vec<PathBuf>, output: PathBuf, opts: HashMap<String, String>) -> () {
        self._check_tools();

        if let Some(parent) = output.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                panic!("Failed to create archive directory: {}", e);
            }
        }

        let mut cmd = Command::new("mksquashfs");
        
        for file in &files {
            cmd.arg(file);
        }
        
        cmd.arg(&output);
        
        if let Some(comp) = opts.get("comp") {
            cmd.arg("-comp").arg(comp);
        }
        
        if let Some(block_size) = opts.get("block_size") {
            cmd.arg("-b").arg(block_size);
        }

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute mksquashfs command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("mksquashfs failed: {}", stderr);
        }
    }

    // Extract specific files
    // Return extracted files
    /// Extract SquashFS filesystem.
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, _opts: HashMap<String, String>) -> Vec<PathBuf> {
        self._check_tools();

        std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
            panic!("Failed to create output directory: {}", e);
        });

        let mut cmd = Command::new("unsquashfs");
        cmd.arg("-d").arg(&output_dir);
        
        if let Some(members_list) = members {
            for member in members_list {
                cmd.arg("-e").arg(member);
            }
        }
        
        cmd.arg(&archive);

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute unsquashfs command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("unsquashfs failed: {}", stderr);
        }

        vec![output_dir]
    }

    // Parse unsquashfs output
    // Skip header lines and parse file list
    /// List SquashFS contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        self._check_tools();

        let mut cmd = Command::new("unsquashfs");
        cmd.arg("-l").arg(archive);

        let output = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute unsquashfs command: {}", e);
        });

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("unsquashfs list failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut names = Vec::new();
        
        // Parse unsquashfs output - skip header lines
        for line in stdout.lines() {
            if !line.starts_with("squashfs") && !line.starts_with("----") && !line.is_empty() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    names.push(trimmed.to_string());
                }
            }
        }

        names
    }

    /// SquashFS doesn't support append (read-only FS).
    pub fn add_file_impl(&self, _archive: &Path, _file: &Path, _arcname: Option<String>) -> () {
        panic!("SquashFS doesn't support append (read-only filesystem)");
    }
}

impl Default for SquashfsArchiver {
    fn default() -> Self {
        Self::new()
    }
}
