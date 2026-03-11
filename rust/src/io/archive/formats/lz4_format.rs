// #exonware/xwsystem/rust/src/io/archive/formats/lz4_format.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/lz4_format.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! LZ4 compression format - RANK #7 FASTEST COMPRESSION.
//! 
//! **Very fast, lower ratio, real-time archiving**
//! 
//! Priority 1 (Security): Safe compression
//! Priority 2 (Usability): Extremely fast
//! Priority 3 (Maintainability): Clean lz4 handling
//! Priority 4 (Performance): Best speed (slower ratio)
//! Priority 5 (Extensibility): Lazy installation of lz4

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;
use lz4::{Decoder, EncoderBuilder};

// Then compress with LZ4
/// LZ4 archive format handler - RANK #7.
///
/// FormatAction naming: Lz4Archiver
///
/// Ultra-fast compression with:
/// - Extremely fast compression/decompression
/// - Lower compression ratio (trade-off for speed)
/// - Ideal for real-time archiving
/// - Log rotation, streaming backups
///
/// Examples:
/// >>> archiver = Lz4Archiver()
/// >>> archiver.create([Path("logs.txt")], Path("logs.tar.lz4"))
/// >>>
/// >>> # High compression
/// >>> archiver.create(files, output, compression_level=12)
/// >>>
/// >>> # Ultra fast (default)
/// >>> archiver.create(files, output, compression_level=0)
pub struct Lz4Archiver;

impl IArchiveFormat for Lz4Archiver {
    fn format_id(&self) -> &str {
        "lz4"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec!["lz4".to_string(), "tar.lz4".to_string()]
    }

    fn mime_types(&self) -> Vec<String> {
        vec!["application/x-lz4".to_string()]
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
        panic!("LZ4 format does not support append - use create() to add files");
    }
}

impl Lz4Archiver {
    pub fn new() -> Self {
        Lz4Archiver
    }

    // Then compress with LZ4
    /// Create LZ4-compressed tar archive.
    ///
    /// Options:
    /// compression_level: 0 (fastest) to 12 (better ratio)
    /// block_size: Block size for compression
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

        // Compress with LZ4
        let compression_level: u32 = opts.get("compression_level")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
            .min(12);

        let file = File::create(&output).unwrap_or_else(|e| {
            panic!("Failed to create lz4 archive '{}': {}", output.display(), e);
        });

        let mut encoder = EncoderBuilder::new()
            .level(compression_level)
            .build(file)
            .unwrap_or_else(|e| {
                panic!("Failed to create LZ4 encoder: {}", e);
            });

        encoder.write_all(&tar_data).unwrap_or_else(|e| {
            panic!("Failed to compress with LZ4: {}", e);
        });
        let (_file, result) = encoder.finish();
        result.unwrap_or_else(|e| {
            panic!("Failed to finish LZ4 compression: {}", e);
        });
    }

    /// Extract LZ4 archive.
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, _opts: HashMap<String, String>) -> Vec<PathBuf> {
        // Decompress LZ4
        let file = File::open(&archive).unwrap_or_else(|e| {
            panic!("Failed to open lz4 archive '{}': {}", archive.display(), e);
        });

        let mut decoder = Decoder::new(file).unwrap_or_else(|e| {
            panic!("Failed to create LZ4 decoder: {}", e);
        });

        let mut tar_data = Vec::new();
        decoder.read_to_end(&mut tar_data).unwrap_or_else(|e| {
            panic!("Failed to decompress LZ4: {}", e);
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

    /// List LZ4 archive contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        let file = File::open(archive).unwrap_or_else(|e| {
            panic!("Failed to open lz4 archive '{}': {}", archive.display(), e);
        });

        let mut decoder = Decoder::new(file).unwrap_or_else(|e| {
            panic!("Failed to create LZ4 decoder: {}", e);
        });

        let mut tar_data = Vec::new();
        decoder.read_to_end(&mut tar_data).unwrap_or_else(|e| {
            panic!("Failed to decompress LZ4: {}", e);
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

    /// Not supported - recreate archive instead.
    pub fn add_file_impl(&self, _archive: &Path, _file: &Path, _arcname: Option<String>) -> () {
        panic!("LZ4 format does not support append - use create() to add files");
    }
}

impl Default for Lz4Archiver {
    fn default() -> Self {
        Self::new()
    }
}
