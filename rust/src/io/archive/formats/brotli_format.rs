// #exonware/xwsystem/rust/src/io/archive/formats/brotli_format.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/brotli_format.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! Brotli (.br) compression format - RANK #6 WEB COMPRESSION.
//! 
//! **Excellent for web & text assets**
//! 
//! Priority 1 (Security): Safe compression
//! Priority 2 (Usability): Excellent text compression
//! Priority 3 (Maintainability): Clean brotli handling
//! Priority 4 (Performance): Optimized for web
//! Priority 5 (Extensibility): Lazy installation of brotli

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;
use brotli::enc::BrotliEncoderParams;
use brotli::CompressorWriter;
use brotli::Decompressor;

// Max quality for static assets
// Then compress with brotli
/// Brotli (.br) archive format handler - RANK #6.
///
/// FormatAction naming: BrotliArchiver
///
/// Web-optimized compression with:
/// - Excellent text/HTML/JSON compression
/// - Dictionary support
/// - Quality levels 0-11
/// - Widely supported in browsers
///
/// Examples:
/// >>> archiver = BrotliArchiver()
/// >>> archiver.create([Path("index.html")], Path("website.tar.br"))
/// >>>
/// >>> # Maximum compression for static assets
/// >>> archiver.create(files, output, quality=11)
/// >>>
/// >>> # Fast compression for dynamic content
/// >>> archiver.create(files, output, quality=4)
pub struct BrotliArchiver;

impl IArchiveFormat for BrotliArchiver {
    fn format_id(&self) -> &str {
        "brotli"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec!["br".to_string(), "tar.br".to_string()]
    }

    fn mime_types(&self) -> Vec<String> {
        vec!["application/x-brotli".to_string()]
    }

    fn create(&self, files: Vec<PathBuf>, output: PathBuf) -> () {
        self.create_with_opts(files, output, HashMap::new())
    }

    fn extract(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>) -> Vec<PathBuf> {
        self.extract_with_opts(archive, output_dir, members, HashMap::new())
    }

    fn list_contents(&self, archive: PathBuf) -> Vec<String> {
        // Brotli is a stream format, list by extracting to temp and reading tar
        self.list_contents_impl(&archive)
    }

    fn add_file(&self, archive: PathBuf, file: PathBuf, arcname: Option<String>) -> () {
        // Not supported - recreate archive instead
        panic!("Brotli format does not support append - use create() to add files");
    }
}

impl BrotliArchiver {
    pub fn new() -> Self {
        BrotliArchiver
    }

    // Max quality for static assets
    // Then compress with brotli
    /// Create Brotli-compressed tar archive.
    ///
    /// Options:
    /// quality: Compression quality (0-11, default: 11 for static)
    /// mode: Compression mode (generic, text, font)
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

        // Compress with brotli
        let quality: u32 = opts.get("quality")
            .and_then(|s| s.parse().ok())
            .unwrap_or(11)
            .min(11);

        let mut params = BrotliEncoderParams::default();
        params.quality = quality as i32;

        let file = File::create(&output).unwrap_or_else(|e| {
            panic!("Failed to create brotli archive '{}': {}", output.display(), e);
        });

        let mut compressor = CompressorWriter::with_params(file, 4096, &params);
        compressor.write_all(&tar_data).unwrap_or_else(|e| {
            panic!("Failed to compress with brotli: {}", e);
        });
        compressor.flush().unwrap_or_else(|e| {
            panic!("Failed to flush brotli compressor: {}", e);
        });
        drop(compressor); // Ensure compressor is closed
    }

    /// Extract Brotli archive.
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, _opts: HashMap<String, String>) -> Vec<PathBuf> {
        // Decompress brotli
        let file = File::open(&archive).unwrap_or_else(|e| {
            panic!("Failed to open brotli archive '{}': {}", archive.display(), e);
        });

        let mut decompressor = Decompressor::new(file, 4096);
        let mut tar_data = Vec::new();
        decompressor.read_to_end(&mut tar_data).unwrap_or_else(|e| {
            panic!("Failed to decompress brotli: {}", e);
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

    /// List Brotli archive contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        // Decompress and list tar contents
        let file = File::open(archive).unwrap_or_else(|e| {
            panic!("Failed to open brotli archive '{}': {}", archive.display(), e);
        });

        let mut decompressor = Decompressor::new(file, 4096);
        let mut tar_data = Vec::new();
        decompressor.read_to_end(&mut tar_data).unwrap_or_else(|e| {
            panic!("Failed to decompress brotli: {}", e);
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
        panic!("Brotli format does not support append - use create() to add files");
    }
}

impl Default for BrotliArchiver {
    fn default() -> Self {
        Self::new()
    }
}
