// #exonware/xwsystem/rust/src/io/archive/compression.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/compression.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Compression operations for gzip, bz2, and lzma.
//! 
//! Priority 1 (Security): Safe compression/decompression
//! Priority 2 (Usability): Auto-detect compression algorithm
//! Priority 3 (Maintainability): Clean compression logic
//! Priority 4 (Performance): Efficient compression with levels
//! Priority 5 (Extensibility): Easy to add new algorithms


use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression as Flate2Compression;
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression as Bzip2Compression;
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;
use xz2::stream::Stream as XzStream;
use std::io::{Read, Write};

use crate::contracts::{ICompression};
use crate::errors::ArchiveError;

// Determine output path
// Write compressed data
// Determine output path
// Remove compression extension
// Write decompressed data
/// Compression operations (gzip, bz2, lzma)
///
/// Use cases:
/// - Compress large files
/// - Network transfer
/// - Storage optimization
/// - Backup compression
///
/// Examples:
/// >>> comp = Compression()
/// >>>
/// >>> # Compress data
/// >>> data = b"Hello" * 1000
/// >>> compressed = comp.compress(data, algorithm='gzip')
/// >>>
/// >>> # Decompress
/// >>> original = comp.decompress(compressed)
/// >>>
/// >>> # Compress file
/// >>> comp.compress_file(Path("data.txt"))
/// >>> # Creates data.txt.gz
pub struct Compression;

impl Compression {
    pub fn new() -> Self {
        Compression
    }
}

impl Default for Compression {
    fn default() -> Self {
        Self::new()
    }
}

impl ICompression for Compression {
    fn compress(&self, data: Vec<u8>) -> Vec<u8> {
        // Default to gzip
        self.compress_impl(data, Some("gzip".to_string()), Some(6)).unwrap()
    }

    fn decompress(&self, data: Vec<u8>) -> Vec<u8> {
        // Auto-detect
        self.decompress_impl(data, None).unwrap()
    }
}

impl Compression {
    /// Compress bytes (public API matching Python signature).
    ///
    ///
    /// Args:
    /// data: Data to compress
    /// algorithm: Compression algorithm (gzip, bz2, lzma)
    /// level: Compression level (1-9, higher = more compression)
    ///
    ///
    /// Returns:
    /// Compressed bytes
    pub fn compress(&self, data: Vec<u8>, algorithm: Option<String>, level: Option<i64>) -> Result<Vec<u8>, ArchiveError> {
        self.compress_impl(data, algorithm, level)
    }

    /// Decompress bytes (public API matching Python signature).
    ///
    ///
    /// Args:
    /// data: Compressed data
    /// algorithm: Algorithm (None = auto-detect)
    ///
    ///
    /// Returns:
    /// Decompressed bytes
    pub fn decompress(&self, data: Vec<u8>, algorithm: Option<String>) -> Result<Vec<u8>, ArchiveError> {
        self.decompress_impl(data, algorithm)
    }

    /// Compress bytes (internal implementation).
    fn compress_impl(&self, data: Vec<u8>, algorithm: Option<String>, level: Option<i64>) -> Result<Vec<u8>, ArchiveError> {
        let algo = algorithm.as_deref().unwrap_or("gzip");
        let level = level.unwrap_or(6) as u32;
        
        match algo {
            "gzip" => {
                let mut encoder = GzEncoder::new(Vec::new(), Flate2Compression::new(level.min(9)));
                encoder.write_all(&data).map_err(|e| {
                    ArchiveError::new(format!("Failed to compress with gzip: {}", e))
                })?;
                encoder.finish().map_err(|e| {
                    ArchiveError::new(format!("Failed to finish gzip compression: {}", e))
                })
            },
            "bz2" => {
                let mut encoder = BzEncoder::new(Vec::new(), Bzip2Compression::new(level.min(9)));
                encoder.write_all(&data).map_err(|e| {
                    ArchiveError::new(format!("Failed to compress with bz2: {}", e))
                })?;
                encoder.finish().map_err(|e| {
                    ArchiveError::new(format!("Failed to finish bz2 compression: {}", e))
                })
            },
            "lzma" | "xz" => {
                let mut encoder = XzEncoder::new_stream(Vec::new(), &XzStream::new_easy_encoder(level.min(9) as u32, xz2::stream::Check::None).map_err(|e| {
                    ArchiveError::new(format!("Failed to create xz encoder: {}", e))
                })?);
                encoder.write_all(&data).map_err(|e| {
                    ArchiveError::new(format!("Failed to compress with xz: {}", e))
                })?;
                encoder.finish().map_err(|e| {
                    ArchiveError::new(format!("Failed to finish xz compression: {}", e))
                })
            },
            _ => Err(ArchiveError::new(format!("Unsupported compression algorithm: {}", algo)))
        }
    }

    /// Decompress bytes.
    ///
    ///
    /// Args:
    /// data: Compressed data
    /// algorithm: Algorithm (None = auto-detect)
    ///
    ///
    /// Returns:
    /// Decompressed bytes
    pub fn decompress_impl(&self, data: Vec<u8>, algorithm: Option<String>) -> Result<Vec<u8>, ArchiveError> {
        let algo = if let Some(alg) = algorithm {
            Some(alg.as_str())
        } else {
            // Auto-detect
            if data.len() >= 2 && data[0] == 0x1f && data[1] == 0x8b {
                Some("gzip")
            } else if data.len() >= 2 && data.starts_with(b"BZ") {
                Some("bz2")
            } else if data.len() >= 6 && data[0] == 0xfd && data[1] == 0x37 && 
                     data[2] == 0x7a && data[3] == 0x58 && data[4] == 0x5a && data[5] == 0x00 {
                Some("lzma")
            } else {
                // Try gzip first
                None
            }
        };

        match algo.unwrap_or("gzip") {
            "gzip" => {
                let mut decoder = GzDecoder::new(&data[..]);
                let mut result = Vec::new();
                decoder.read_to_end(&mut result).map_err(|e| {
                    ArchiveError::new(format!("Failed to decompress with gzip: {}", e))
                })?;
                Ok(result)
            },
            "bz2" => {
                let mut decoder = BzDecoder::new(&data[..]);
                let mut result = Vec::new();
                decoder.read_to_end(&mut result).map_err(|e| {
                    ArchiveError::new(format!("Failed to decompress with bz2: {}", e))
                })?;
                Ok(result)
            },
            "lzma" | "xz" => {
                let mut decoder = XzDecoder::new(&data[..]);
                let mut result = Vec::new();
                decoder.read_to_end(&mut result).map_err(|e| {
                    ArchiveError::new(format!("Failed to decompress with xz: {}", e))
                })?;
                Ok(result)
            },
            _ => Err(ArchiveError::new(format!("Unsupported compression algorithm: {:?}", algo)))
        }
    }

    // Determine output path
    // Write compressed data
    /// Compress file.
    ///
    ///
    /// Args:
    /// path: File to compress
    /// algorithm: Compression algorithm
    /// level: Compression level
    /// **opts: Algorithm-specific options (output path)
    ///
    ///
    /// Returns:
    /// Path to compressed file (e.g., file.txt.gz)
    pub fn compress_file(&self, path: &Path, algorithm: Option<String>, level: Option<i64>, opts: HashMap<String, String>) -> Result<PathBuf, ArchiveError> {
        // Determine output path
        let output = opts.get("output")
            .map(|s| PathBuf::from(s))
            .unwrap_or_else(|| {
                let algo = algorithm.as_deref().unwrap_or("gzip");
                let ext = match algo {
                    "gzip" => ".gz",
                    "bz2" => ".bz2",
                    "lzma" | "xz" => ".xz",
                    _ => "",
                };
                path.with_extension(format!("{}{}", path.extension().and_then(|e| e.to_str()).unwrap_or(""), ext))
            });

        // Read and compress
        let data = fs::read(path).map_err(|e| {
            ArchiveError::new(format!("Failed to read file '{}': {}", path.display(), e))
        })?;
        
        let compressed = self.compress_impl(data, algorithm, level)?;

        // Write compressed data
        fs::write(&output, compressed).map_err(|e| {
            ArchiveError::new(format!("Failed to write compressed file '{}': {}", output.display(), e))
        })?;

        Ok(output)
    }

    // Determine output path
    // Remove compression extension
    // Write decompressed data
    /// Decompress file.
    ///
    ///
    /// Args:
    /// path: Compressed file
    /// output: Output path (None = auto-generate from input)
    ///
    ///
    /// Returns:
    /// Path to decompressed file
    pub fn decompress_file(&self, path: &Path, output: Option<PathBuf>) -> Result<PathBuf, ArchiveError> {
        // Determine output path
        let output = output.unwrap_or_else(|| {
            // Remove compression extension
            if let Some(suffix) = path.extension().and_then(|e| e.to_str()) {
                if suffix == "gz" || suffix == "bz2" || suffix == "xz" {
                    path.with_extension("")
                } else {
                    path.with_extension(format!("{}.decompressed", suffix))
                }
            } else {
                path.with_extension("decompressed")
            }
        });

        // Read and decompress
        let data = fs::read(path).map_err(|e| {
            ArchiveError::new(format!("Failed to read compressed file '{}': {}", path.display(), e))
        })?;
        
        let decompressed = self.decompress_impl(data, None)?;

        // Write decompressed data
        fs::write(&output, decompressed).map_err(|e| {
            ArchiveError::new(format!("Failed to write decompressed file '{}': {}", output.display(), e))
        })?;

        Ok(output)
    }
}
