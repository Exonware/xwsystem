// #exonware/xwsystem/rust/src/io/archive/formats/zip.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/zip.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! ZIP archive format implementation.
//! 
//! Priority 1 (Security): Safe ZIP operations
//! Priority 2 (Usability): Simple ZIP API
//! Priority 3 (Maintainability): Clean ZIP handling
//! Priority 4 (Performance): Efficient ZIP operations
//! Priority 5 (Extensibility): Registered via registry

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::contracts::IArchiveFormat;
use crate::errors::ArchiveError;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;
use zip::ZipArchive;

// Add directory recursively
/// ZIP archive format handler.
pub struct ZipArchiver;

impl IArchiveFormat for ZipArchiver {
    fn format_id(&self) -> &str {
        "zip"
    }

    fn file_extensions(&self) -> Vec<String> {
        vec!["zip".to_string(), "zipx".to_string()]
    }

    fn mime_types(&self) -> Vec<String> {
        vec!["application/zip".to_string(), "application/x-zip-compressed".to_string()]
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

impl ZipArchiver {
    pub fn new() -> Self {
        ZipArchiver
    }

    // Add directory recursively
    /// Create ZIP archive.
    pub fn create_with_opts(&self, files: Vec<PathBuf>, output: PathBuf, opts: HashMap<String, String>) -> () {
        if let Some(parent) = output.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                panic!("Failed to create archive directory: {}", e);
            }
        }

        let file = File::create(&output).unwrap_or_else(|e| {
            panic!("Failed to create zip archive '{}': {}", output.display(), e);
        });

        let compression_level: i64 = opts.get("compression")
            .and_then(|s| s.parse().ok())
            .unwrap_or(8);
        
        let compression_method = if compression_level > 0 {
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
                    .unwrap_or_else(|| panic!("Invalid file name"));
                
                zip.start_file(file_name, options).unwrap_or_else(|e| {
                    panic!("Failed to add file to zip: {}", e);
                });
                
                let mut file_content = File::open(&file_path).unwrap_or_else(|e| {
                    panic!("Failed to read file '{}': {}", file_path.display(), e);
                });
                
                std::io::copy(&mut file_content, &mut zip).unwrap_or_else(|e| {
                    panic!("Failed to write file to zip: {}", e);
                });
            } else if file_path.is_dir() {
                // Add directory recursively
                self.add_directory_recursive(&mut zip, &file_path, &file_path, &options);
            }
        }

        zip.finish().unwrap_or_else(|e| {
            panic!("Failed to finalize zip archive: {}", e);
        });
    }

    fn add_directory_recursive(&self, zip: &mut ZipWriter<File>, root: &Path, dir: &Path, options: &FileOptions) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let relative_path = path.strip_prefix(root)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .replace('\\', "/");
                    
                    zip.start_file(&relative_path, *options).unwrap_or_else(|e| {
                        panic!("Failed to add file to zip: {}", e);
                    });
                    
                    let mut file_content = File::open(&path).unwrap_or_else(|e| {
                        panic!("Failed to read file '{}': {}", path.display(), e);
                    });
                    
                    std::io::copy(&mut file_content, zip).unwrap_or_else(|e| {
                        panic!("Failed to write file to zip: {}", e);
                    });
                } else if path.is_dir() {
                    self.add_directory_recursive(zip, root, &path, options);
                }
            }
        }
    }

    /// Extract ZIP archive.
    pub fn extract_with_opts(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, _opts: HashMap<String, String>) -> Vec<PathBuf> {
        let file = File::open(&archive).unwrap_or_else(|e| {
            panic!("Failed to open zip archive '{}': {}", archive.display(), e);
        });

        let mut zip = ZipArchive::new(file).unwrap_or_else(|e| {
            panic!("Failed to read zip archive '{}': {}", archive.display(), e);
        });

        std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
            panic!("Failed to create output directory: {}", e);
        });

        let mut extracted = Vec::new();
        let members_set: std::collections::HashSet<String> = members.as_ref()
            .map(|m| m.iter().cloned().collect())
            .unwrap_or_default();

        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap_or_else(|e| {
                panic!("Failed to read zip entry: {}", e);
            });
            
            let name = file.name().to_string();
            
            // Filter by members if specified
            if !members_set.is_empty() && !members_set.contains(&name) {
                continue;
            }

            let out_path = output_dir.join(&name);
            
            if name.ends_with('/') {
                // Directory
                std::fs::create_dir_all(&out_path).unwrap_or_else(|e| {
                    panic!("Failed to create directory: {}", e);
                });
            } else {
                // File
                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                        panic!("Failed to create parent directory: {}", e);
                    });
                }
                
                let mut out_file = File::create(&out_path).unwrap_or_else(|e| {
                    panic!("Failed to create file '{}': {}", out_path.display(), e);
                });
                
                std::io::copy(&mut file, &mut out_file).unwrap_or_else(|e| {
                    panic!("Failed to extract file: {}", e);
                });
            }
            
            extracted.push(out_path);
        }

        extracted
    }

    /// List ZIP contents.
    pub fn list_contents_impl(&self, archive: &Path) -> Vec<String> {
        let file = File::open(archive).unwrap_or_else(|e| {
            panic!("Failed to open zip archive '{}': {}", archive.display(), e);
        });

        let mut zip = ZipArchive::new(file).unwrap_or_else(|e| {
            panic!("Failed to read zip archive '{}': {}", archive.display(), e);
        });

        let mut names = Vec::new();
        for i in 0..zip.len() {
            let file = zip.by_index(i).unwrap_or_else(|e| {
                panic!("Failed to read zip entry: {}", e);
            });
            names.push(file.name().to_string());
        }

        names
    }

    /// Add file to ZIP archive.
    pub fn add_file_impl(&self, archive: &Path, file: &Path, arcname: Option<String>) -> () {
        // Read existing archive
        let existing_file = File::open(archive).unwrap_or_else(|e| {
            panic!("Failed to open zip archive '{}': {}", archive.display(), e);
        });

        let mut zip = ZipArchive::new(existing_file).unwrap_or_else(|e| {
            panic!("Failed to read zip archive '{}': {}", archive.display(), e);
        });

        // Create new archive with existing files + new file
        let new_file = File::create(archive).unwrap_or_else(|e| {
            panic!("Failed to create zip archive '{}': {}", archive.display(), e);
        });

        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
        let mut new_zip = ZipWriter::new(new_file);

        // Copy existing files
        for i in 0..zip.len() {
            let mut file_entry = zip.by_index(i).unwrap_or_else(|e| {
                panic!("Failed to read zip entry: {}", e);
            });
            
            new_zip.start_file(file_entry.name(), options).unwrap_or_else(|e| {
                panic!("Failed to add existing file to zip: {}", e);
            });
            
            std::io::copy(&mut file_entry, &mut new_zip).unwrap_or_else(|e| {
                panic!("Failed to copy file to new zip: {}", e);
            });
        }

        // Add new file
        let file_name = arcname.unwrap_or_else(|| {
            file.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| panic!("Invalid file name"))
        });
        
        new_zip.start_file(&file_name, options).unwrap_or_else(|e| {
            panic!("Failed to add file to zip: {}", e);
        });
        
        let mut file_content = File::open(file).unwrap_or_else(|e| {
            panic!("Failed to read file '{}': {}", file.display(), e);
        });
        
        std::io::copy(&mut file_content, &mut new_zip).unwrap_or_else(|e| {
            panic!("Failed to write file to zip: {}", e);
        });

        new_zip.finish().unwrap_or_else(|e| {
            panic!("Failed to finalize zip archive: {}", e);
        });
    }
}

impl Default for ZipArchiver {
    fn default() -> Self {
        Self::new()
    }
}
