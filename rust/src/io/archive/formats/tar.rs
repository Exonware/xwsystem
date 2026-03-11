// #exonware/xwsystem/rust/src/io/archive/formats/tar.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/tar.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! TAR archive format implementation.
//! 
//! Priority 1 (Security): Safe TAR operations
//! Priority 2 (Usability): Simple TAR API
//! Priority 3 (Maintainability): Clean TAR handling
//! Priority 4 (Performance): Efficient TAR operations
//! Priority 5 (Extensibility): Registered via registry

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, Seek};
use std::path::{Path, PathBuf};

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;
use tar::{Builder, Archive as TarArchive};
use flate2::read::GzDecoder;
use bzip2::read::BzDecoder;
use xz2::read::XzDecoder;

// Auto-detect compression on read
// Only support append to uncompressed TAR
/// TAR archive format handler (supports tar, tar.gz, tar.bz2, tar.xz).
pub struct TarArchiver;

impl IArchiveFormat for TarArchiver {
    fn format_id(&self) -> &str {
        "tar"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec![
            "tar".to_string(),
            "tar.gz".to_string(),
            "tgz".to_string(),
            "tar.bz2".to_string(),
            "tbz2".to_string(),
            "tar.xz".to_string(),
            "txz".to_string(),
        ]
    }

    fn mime_types(&self) -> Vec<String> {
        vec![
            "application/x-tar".to_string(),
            "application/gzip".to_string(),
            "application/x-bzip2".to_string(),
            "application/x-xz".to_string(),
        ]
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

impl TarArchiver {
    pub fn new() -> Self {
        TarArchiver
    }

    // Auto-detect compression on read
    /// Determine TAR mode from file extension.
    pub fn _determine_mode(&self, path: &Path, _write: Option<bool>) -> String {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name.ends_with(".tar.gz") || file_name.ends_with(".tgz") {
            "gz".to_string()
        } else if file_name.ends_with(".tar.bz2") || file_name.ends_with(".tbz2") {
            "bz2".to_string()
        } else if file_name.ends_with(".tar.xz") || file_name.ends_with(".txz") {
            "xz".to_string()
        } else if ext == "gz" {
            "gz".to_string()
        } else if ext == "bz2" {
            "bz2".to_string()
        } else if ext == "xz" {
            "xz".to_string()
        } else {
            "".to_string()
        }
    }

    /// Create TAR archive.
    pub fn create_with_opts(&self, files: Vec<PathBuf>, output: PathBuf, opts: HashMap<String, String>) -> () {
        if let Some(parent) = output.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                panic!("Failed to create archive directory: {}", e);
            }
        }

        let file = File::create(&output).unwrap_or_else(|e| {
            panic!("Failed to create tar archive '{}': {}", output.display(), e);
        });

        let mode = opts.get("mode").cloned().unwrap_or_else(|| {
            self._determine_mode(&output, Some(true))
        });

        let mut tar: Box<dyn Write> = match mode.as_str() {
            "gz" => {
                Box::new(flate2::write::GzEncoder::new(file, flate2::Compression::default()))
            },
            "bz2" => {
                Box::new(bzip2::write::BzEncoder::new(file, bzip2::Compression::default()))
            },
            "xz" => {
                Box::new(xz2::write::XzEncoder::new(file, 6))
            },
            _ => Box::new(file),
        };

        let mut builder = Builder::new(tar);
        
        for file_path in files {
            if file_path.is_file() {
                let file_name = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_else(|| panic!("Invalid file name"));
                
                builder.append_path_with_name(&file_path, file_name).unwrap_or_else(|e| {
                    panic!("Failed to add file to tar: {}", e);
                });
            } else if file_path.is_dir() {
                builder.append_dir_all(file_path.file_name().unwrap().to_str().unwrap(), &file_path)
                    .unwrap_or_else(|e| {
                        panic!("Failed to add directory to tar: {}", e);
                    });
            }
        }

        builder.finish().unwrap_or_else(|e| {
            panic!("Failed to finalize tar archive: {}", e);
        });
    }

    /// Extract TAR archive.
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, _opts: HashMap<String, String>) -> Vec<PathBuf> {
        let mut archive_reader = Self::open_tar_archive(&archive).unwrap_or_else(|e| {
            panic!("Failed to read tar archive: {}", e);
        });

        std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
            panic!("Failed to create output directory: {}", e);
        });

        let members_set: std::collections::HashSet<String> = members.as_ref()
            .map(|m| m.iter().cloned().collect())
            .unwrap_or_default();

        let mut extracted = Vec::new();
        for entry in archive_reader.entries().unwrap_or_else(|e| {
            panic!("Failed to read tar archive entries: {}", e);
        }) {
            let mut entry = entry.unwrap_or_else(|e| {
                panic!("Failed to read tar entry: {}", e);
            });
            
            let path = entry.path().unwrap_or_else(|e| {
                panic!("Failed to get entry path: {}", e);
            });
            let path_str = path.to_string_lossy().to_string();
            
            // Filter by members if specified
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

    /// List TAR contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        let mut archive_reader = Self::open_tar_archive(archive).unwrap_or_else(|e| {
            panic!("Failed to read tar archive: {}", e);
        });

        let mut names = Vec::new();
        for entry in archive_reader.entries().unwrap_or_else(|e| {
            panic!("Failed to read tar archive entries: {}", e);
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

    // Only support append to uncompressed TAR
    /// Add file to TAR archive.
    pub fn add_file_impl(&self, archive: &Path, file: &Path, arcname: Option<String>) -> () {
        // TAR doesn't support true append - need to recreate
        // For now, just create a new archive with the new file
        // This is a limitation of the tar crate
        panic!("Append mode not fully supported for TAR - use create() to add files");
    }

    // Helper to open tar archive with compression detection
    fn open_tar_archive(path: &Path) -> Result<TarArchive<Box<dyn Read>>, ArchiveError> {
        use std::io::BufReader;
        
        let file = File::open(path).map_err(|e| {
            ArchiveError::new(format!("Failed to open file: {}", e))
        })?;
        
        let mut reader = BufReader::new(&file);
        let mut buffer = [0u8; 6];
        let _ = reader.read_exact(&mut buffer);
        
        // Reopen file for the decoder
        let file = File::open(path).map_err(|e| {
            ArchiveError::new(format!("Failed to reopen file: {}", e))
        })?;

        // Check compression type
        if buffer[0..2] == [0x1f, 0x8b] {
            // gzip
            Ok(TarArchive::new(Box::new(GzDecoder::new(file))))
        } else if buffer[0..2] == [b'B', b'Z'] {
            // bzip2
            Ok(TarArchive::new(Box::new(BzDecoder::new(file))))
        } else if buffer[0..6] == [0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00] {
            // xz
            Ok(TarArchive::new(Box::new(XzDecoder::new(file))))
        } else {
            // No compression - reopen to reset position
            let file = File::open(path).map_err(|e| {
                ArchiveError::new(format!("Failed to reopen file: {}", e))
            })?;
            Ok(TarArchive::new(Box::new(file)))
        }
    }
}

impl Default for TarArchiver {
    fn default() -> Self {
        Self::new()
    }
}
