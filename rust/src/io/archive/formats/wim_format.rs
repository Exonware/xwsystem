// #exonware/xwsystem/rust/src/io/archive/formats/wim_format.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/wim_format.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! WIM (Windows Imaging) format - RANK #9 SYSTEM IMAGES.
//! 
//! **Used by Microsoft for system images**
//! 
//! Priority 1 (Security): System image integrity
//! Priority 2 (Usability): Windows deployment
//! Priority 3 (Maintainability): Clean WIM handling
//! Priority 4 (Performance): LZX compression
//! Priority 5 (Extensibility): Lazy installation of wimlib

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;

/// WIM archive format handler - RANK #9.
///
/// FormatAction naming: WimArchiver
///
/// Windows Imaging format with:
/// - LZX compression
/// - Hardware-independent imaging
/// - Single-instance storage
/// - Bootable images
///
/// Examples:
/// >>> archiver = WimArchiver()
/// >>> # Create system image
/// >>> archiver.create([Path("C:\")], Path("system.wim"))
/// >>>
/// >>> # Extract image
/// >>> archiver.extract(Path("system.wim"), Path("restore/"))
pub struct WimArchiver;

impl IArchiveFormat for WimArchiver {
    fn format_id(&self) -> &str {
        "wim"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec!["wim".to_string(), "esd".to_string(), "swm".to_string()]
    }

    fn mime_types(&self) -> Vec<String> {
        vec!["application/x-ms-wim".to_string()]
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

impl WimArchiver {
    pub fn new() -> Self {
        WimArchiver
    }

    /// Create WIM image.
    ///
    /// Options:
    /// compression: Compression type (none, xpress, lzx, lzms)
    /// boot: Make image bootable
    pub fn create_with_opts(&self, files: Vec<PathBuf>, output: PathBuf, opts: HashMap<String, String>) -> () {
        // Try wimlib-imagex (cross-platform) or DISM (Windows)
        let wim_cmd = if cfg!(windows) {
            // Try DISM first, then wimlib-imagex
            if Command::new("dism").arg("/?").output().is_ok() {
                "dism"
            } else if Command::new("wimlib-imagex").arg("--help").output().is_ok() {
                "wimlib-imagex"
            } else {
                panic!("WIM tools not found. Please install wimlib (https://wimlib.net/) or use Windows DISM");
            }
        } else {
            if Command::new("wimlib-imagex").arg("--help").output().is_err() {
                panic!("wimlib-imagex not found. Please install wimlib (https://wimlib.net/)");
            }
            "wimlib-imagex"
        };

        if let Some(parent) = output.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                panic!("Failed to create archive directory: {}", e);
            }
        }

        if wim_cmd == "dism" {
            // DISM command (Windows)
            let mut cmd = Command::new("dism");
            cmd.arg("/Capture-Image");
            cmd.arg("/ImageFile").arg(&output);
            cmd.arg("/CaptureDir").arg(&files[0]);
            cmd.arg("/Name").arg("Image");
            
            if let Some(compression) = opts.get("compression") {
                cmd.arg("/Compress").arg(compression);
            }
            
            let output_result = cmd.output().unwrap_or_else(|e| {
                panic!("Failed to execute DISM command: {}", e);
            });

            if !output_result.status.success() {
                let stderr = String::from_utf8_lossy(&output_result.stderr);
                panic!("DISM failed: {}", stderr);
            }
        } else {
            // wimlib-imagex command
            let mut cmd = Command::new("wimlib-imagex");
            cmd.arg("capture");
            
            for file in &files {
                cmd.arg(file);
            }
            
            cmd.arg(&output);
            cmd.arg("Image");
            
            if let Some(compression) = opts.get("compression") {
                cmd.arg("--compress").arg(compression);
            }
            
            if opts.get("boot").and_then(|s| s.parse().ok()).unwrap_or(false) {
                cmd.arg("--boot");
            }

            let output_result = cmd.output().unwrap_or_else(|e| {
                panic!("Failed to execute wimlib-imagex command: {}", e);
            });

            if !output_result.status.success() {
                let stderr = String::from_utf8_lossy(&output_result.stderr);
                panic!("wimlib-imagex failed: {}", stderr);
            }
        }
    }

    /// Extract WIM image.
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, _opts: HashMap<String, String>) -> Vec<PathBuf> {
        let wim_cmd = if cfg!(windows) {
            if Command::new("dism").arg("/?").output().is_ok() {
                "dism"
            } else if Command::new("wimlib-imagex").arg("--help").output().is_ok() {
                "wimlib-imagex"
            } else {
                panic!("WIM tools not found");
            }
        } else {
            if Command::new("wimlib-imagex").arg("--help").output().is_err() {
                panic!("wimlib-imagex not found");
            }
            "wimlib-imagex"
        };

        std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
            panic!("Failed to create output directory: {}", e);
        });

        if wim_cmd == "dism" {
            let mut cmd = Command::new("dism");
            cmd.arg("/Apply-Image");
            cmd.arg("/ImageFile").arg(&archive);
            cmd.arg("/ApplyDir").arg(&output_dir);
            cmd.arg("/Index").arg("1");

            let output_result = cmd.output().unwrap_or_else(|e| {
                panic!("Failed to execute DISM command: {}", e);
            });

            if !output_result.status.success() {
                let stderr = String::from_utf8_lossy(&output_result.stderr);
                panic!("DISM extraction failed: {}", stderr);
            }
        } else {
            let mut cmd = Command::new("wimlib-imagex");
            cmd.arg("apply");
            cmd.arg(&archive);
            cmd.arg("1"); // Image index
            cmd.arg(&output_dir);

            let output_result = cmd.output().unwrap_or_else(|e| {
                panic!("Failed to execute wimlib-imagex command: {}", e);
            });

            if !output_result.status.success() {
                let stderr = String::from_utf8_lossy(&output_result.stderr);
                panic!("wimlib-imagex extraction failed: {}", stderr);
            }
        }

        vec![output_dir]
    }

    /// List WIM contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        let wim_cmd = if cfg!(windows) {
            if Command::new("dism").arg("/?").output().is_ok() {
                "dism"
            } else if Command::new("wimlib-imagex").arg("--help").output().is_ok() {
                "wimlib-imagex"
            } else {
                panic!("WIM tools not found");
            }
        } else {
            if Command::new("wimlib-imagex").arg("--help").output().is_err() {
                panic!("wimlib-imagex not found");
            }
            "wimlib-imagex"
        };

        if wim_cmd == "dism" {
            let mut cmd = Command::new("dism");
            cmd.arg("/Get-ImageInfo");
            cmd.arg("/ImageFile").arg(archive);

            let output = cmd.output().unwrap_or_else(|e| {
                panic!("Failed to execute DISM command: {}", e);
            });

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                panic!("DISM list failed: {}", stderr);
            }

            // Parse DISM output (simplified)
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut names = Vec::new();
            for line in stdout.lines() {
                if line.contains("Index") || line.contains("Name") {
                    names.push(line.trim().to_string());
                }
            }
            names
        } else {
            let mut cmd = Command::new("wimlib-imagex");
            cmd.arg("info");
            cmd.arg(archive);

            let output = cmd.output().unwrap_or_else(|e| {
                panic!("Failed to execute wimlib-imagex command: {}", e);
            });

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                panic!("wimlib-imagex list failed: {}", stderr);
            }

            // Parse wimlib-imagex output (simplified)
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut names = Vec::new();
            for line in stdout.lines() {
                if line.contains("Image Index") || line.contains("Image Name") {
                    names.push(line.trim().to_string());
                }
            }
            names
        }
    }

    /// Add image to WIM.
    pub fn add_file_impl(&self, archive: &Path, file: &Path, _arcname: Option<String>) -> () {
        let wim_cmd = if cfg!(windows) {
            if Command::new("wimlib-imagex").arg("--help").output().is_ok() {
                "wimlib-imagex"
            } else {
                panic!("wimlib-imagex not found for append operation");
            }
        } else {
            if Command::new("wimlib-imagex").arg("--help").output().is_err() {
                panic!("wimlib-imagex not found");
            }
            "wimlib-imagex"
        };

        let mut cmd = Command::new(wim_cmd);
        cmd.arg("append");
        cmd.arg(archive);
        cmd.arg(file);
        cmd.arg("NewImage");

        let output_result = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute wimlib-imagex command: {}", e);
        });

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            panic!("wimlib-imagex append failed: {}", stderr);
        }
    }
}

impl Default for WimArchiver {
    fn default() -> Self {
        Self::new()
    }
}
