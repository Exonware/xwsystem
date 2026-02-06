// #exonware/xwsystem/rust/src/io/archive/formats/zstandard.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/zstandard.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! Zstandard (.zst) compression format - RANK #2 MODERN STANDARD.
//! 
//! **Fastest balanced modern compression - ideal for backups & databases**
//! 
//! Priority 1 (Security): Safe compression
//! Priority 2 (Usability): Fast compression/decompression
//! Priority 3 (Maintainability): Clean zstd handling
//! Priority 4 (Performance): Best speed/ratio balance
//! Priority 5 (Extensibility): Lazy installation of zstandard

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;
use zstd::stream::{Encoder, Decoder};

// Default: fast balanced
// Create tar.zst archive
/// Zstandard (.zst) archive format handler - RANK #2.
///
/// FormatAction naming: ZstandardArchiver
///
/// Modern compression with:
/// - Very fast compression/decompression
/// - Excellent compression ratio
/// - Streaming support
/// - Dictionary compression
/// - Ideal for databases and backups
///
/// Examples:
/// >>> archiver = ZstandardArchiver()
/// >>> archiver.create([Path("database.db")], Path("backup.tar.zst"))
/// >>>
/// >>> # High compression
/// >>> archiver.create(files, output, level=22)
/// >>>
/// >>> # Fast compression
/// >>> archiver.create(files, output, level=1)
pub struct ZstandardArchiver;

impl IArchiveFormat for ZstandardArchiver {
    fn format_id(&self) -> &str {
        "zstandard"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec!["zst".to_string(), "tar.zst".to_string(), "tzst".to_string()]
    }

    fn mime_types(&self) -> Vec<String> {
        vec!["application/zstd".to_string()]
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
        panic!("Zstandard format does not support append - use create() to add files");
    }
}

impl ZstandardArchiver {
    pub fn new() -> Self {
        ZstandardArchiver
    }

    // Default: fast balanced
    // Create tar.zst archive
    /// Create Zstandard-compressed tar archive.
    ///
    /// Options:
    /// level: Compression level (1-22, default: 3)
    /// threads: Number of threads for compression
    pub fn create_with_opts(&self, files: Vec<PathBuf>, output: PathBuf, opts: HashMap<String, String>) -> () {
        if let Some(parent) = output.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                panic!("Failed to create archive directory: {}", e);
            }
        }

        // First create a tar archive in memory
        let mut tar_data = Vec::new();
        {
            let mut tar = tar::Builder::new(&mut tar_data);
            for file_path in files {
                if file_path.is_file() {
                    let file_name = file_path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_else(|| panic!("Invalid file name"));
                    tar.append_path_with_name(&file_path, file_name).unwrap_or_else(|e| {
                        panic!("Failed to add file to tar: {}", e);
                    });
                } else if file_path.is_dir() {
                    tar.append_dir_all(file_path.file_name().unwrap().to_str().unwrap(), &file_path)
                        .unwrap_or_else(|e| {
                            panic!("Failed to add directory to tar: {}", e);
                        });
                }
            }
            tar.finish().unwrap_or_else(|e| {
                panic!("Failed to finalize tar: {}", e);
            });
        }

        // Compress with zstd
        let level: i32 = opts.get("level")
            .and_then(|s| s.parse().ok())
            .unwrap_or(3)
            .min(22)
            .max(1);

        let file = File::create(&output).unwrap_or_else(|e| {
            panic!("Failed to create zstd archive '{}': {}", output.display(), e);
        });

        let mut encoder = Encoder::new(file, level).unwrap_or_else(|e| {
            panic!("Failed to create zstd encoder: {}", e);
        });

        encoder.write_all(&tar_data).unwrap_or_else(|e| {
            panic!("Failed to compress with zstd: {}", e);
        });
        encoder.finish().unwrap_or_else(|e| {
            panic!("Failed to finish zstd compression: {}", e);
        });
    }

    /// Extract Zstandard archive.
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, _opts: HashMap<String, String>) -> Vec<PathBuf> {
        // Decompress zstd
        let file = File::open(&archive).unwrap_or_else(|e| {
            panic!("Failed to open zstd archive '{}': {}", archive.display(), e);
        });

        let mut decoder = Decoder::new(file).unwrap_or_else(|e| {
            panic!("Failed to create zstd decoder: {}", e);
        });

        let mut tar_data = Vec::new();
        decoder.read_to_end(&mut tar_data).unwrap_or_else(|e| {
            panic!("Failed to decompress zstd: {}", e);
        });

        // Extract tar
        let mut tar = tar::Archive::new(&tar_data[..]);
        
        std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
            panic!("Failed to create output directory: {}", e);
        });

        let members_set: std::collections::HashSet<String> = members.as_ref()
            .map(|m| m.iter().cloned().collect())
            .unwrap_or_default();

        let mut extracted = Vec::new();
        for entry in tar.entries().unwrap_or_else(|e| {
            panic!("Failed to read tar entries: {}", e);
        }) {
            let mut entry = entry.unwrap_or_else(|e| {
                panic!("Failed to read tar entry: {}", e);
            });
            
            let path = entry.path().unwrap_or_else(|e| {
                panic!("Failed to get entry path: {}", e);
            });
            let path_str = path.to_string_lossy().to_string();
            
            if !members_set.is_empty() && !members_set.contains(&path_str) {
                continue;
            }

            let out_path = output_dir.join(&path);
            
            if entry.header().entry_type().is_dir() {
                std::fs::create_dir_all(&out_path).unwrap_or_else(|e| {
                    panic!("Failed to create directory: {}", e);
                });
            } else {
                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                        panic!("Failed to create parent directory: {}", e);
                    });
                }
                
                let mut out_file = File::create(&out_path).unwrap_or_else(|e| {
                    panic!("Failed to create file '{}': {}", out_path.display(), e);
                });
                
                std::io::copy(&mut entry, &mut out_file).unwrap_or_else(|e| {
                    panic!("Failed to extract file: {}", e);
                });
            }
            
            extracted.push(out_path);
        }

        extracted
    }

    /// List Zstandard archive contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        let file = File::open(archive).unwrap_or_else(|e| {
            panic!("Failed to open zstd archive '{}': {}", archive.display(), e);
        });

        let mut decoder = Decoder::new(file).unwrap_or_else(|e| {
            panic!("Failed to create zstd decoder: {}", e);
        });

        let mut tar_data = Vec::new();
        decoder.read_to_end(&mut tar_data).unwrap_or_else(|e| {
            panic!("Failed to decompress zstd: {}", e);
        });

        let tar = tar::Archive::new(&tar_data[..]);
        let mut names = Vec::new();
        for entry in tar.entries().unwrap_or_else(|e| {
            panic!("Failed to read tar entries: {}", e);
        }) {
            let entry = entry.unwrap_or_else(|e| {
                panic!("Failed to read tar entry: {}", e);
            });
            if let Ok(path) = entry.path() {
                names.push(path.to_string_lossy().to_string());
            }
        }

        names
    }

    /// Not supported for streaming format - recreate archive instead.
    pub fn add_file_impl(&self, _archive: &Path, _file: &Path, _arcname: Option<String>) -> () {
        panic!("Zstandard format does not support append - use create() to add files");
    }
}

impl Default for ZstandardArchiver {
    fn default() -> Self {
        Self::new()
    }
}
