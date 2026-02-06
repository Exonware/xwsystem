// #exonware/xwsystem/rust/src/io/archive/base.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/base.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Base classes and registries for archive system.
//! 
//! Like codec/base.py: Contains abstracts + ArchiveFormatRegistry!
//! 
//! Priority 1 (Security): Safe base implementations
//! Priority 2 (Usability): Auto-detection like codec!
//! Priority 3 (Maintainability): Clean registry pattern
//! Priority 4 (Performance): Fast format lookup
//! Priority 5 (Extensibility): Easy to add 7z, RAR, etc.


use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use crate::base::{AFile};
use crate::codec::base::{ACodec};
use crate::contracts::{DecodeOptions, EncodeOptions, IArchiveFile, IArchiveFormat, IArchiveMetadata, IArchiver, ICompressor};
use crate::defs::{ArchiveFormat as ArchiveFormatEnum, CodecCapability, CompressionAlgorithm};

// ============================================================================

// ARCHIVE CODEC ABSTRACT BASE CLASS (I→A→XW Pattern)

// ============================================================================

// ============================================================================

// ARCHIVE FILE ABSTRACT BASE CLASS (I→A→XW Pattern)

// ============================================================================

// ============================================================================

// ARCHIVE FORMAT REGISTRY (Like CodecRegistry!)

// ============================================================================

// ============================================================================

// COMPRESSION REGISTRY

// ============================================================================

// Global registries

// Global registries

// Global registries

// Default: no MIME types
/// Abstract base for archive format handlers.
pub trait AArchiveFormat: IArchiveFormat {
    /// Unique format identifier.
    // Python decorators: @property
    fn format_id(&self) -> &str;

    /// Supported file extensions.
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

    /// Supported MIME types.
    // Python decorators: @property
    fn mime_types(&self) -> Vec<String> {
        vec![]  // Default: no MIME types
    }

}

/// Abstract base for compression algorithms.
pub trait ACompressor: ICompressor {
    /// Unique algorithm identifier.
    // Python decorators: @property
    fn algorithm_id(&self) -> &str;

    /// Supported file extensions.
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

}

/// Abstract base class for archive codecs - follows I→A→XW pattern.
///
/// I: IArchiver (interface)
/// A: AArchiver (abstract base - this class)
/// XW: XWArchiver implementations (ZipArchiver, TarArchiver, etc.)
///
/// Provides default implementations of compress()/extract() that delegate
/// to encode()/decode(). Subclasses only need to implement encode/decode
/// and metadata properties.
///
/// This is for in-memory archive operations - works on data in RAM, not files.
pub trait AArchiver: IArchiver {
    /// Encode data to archive bytes - must implement in subclass.
    fn encode(&self, value: serde_json::Value) -> Vec<u8>;

    /// Decode archive bytes to data - must implement in subclass.
    fn decode(&self, repr: Vec<u8>) -> serde_json::Value;

    /// Codec identifier (e.g., 'zip', 'tar').
    // Python decorators: @property
    fn codec_id(&self) -> &str;

    /// Supported MIME types.
    // Python decorators: @property
    fn media_types(&self) -> Vec<String>;

    /// Supported file extensions.
    // Python decorators: @property
    fn file_extensions(&self) -> Vec<String>;

    /// Archive codecs support bidirectional operations.
    // Python decorators: @property
    fn capabilities(&self) -> CodecCapability {
        CodecCapability::BIDIRECTIONAL
    }

    /// Default aliases from codec_id.
    // Python decorators: @property
    fn aliases(&self) -> Vec<String> {
        let id = self.codec_id();
        vec![id.to_lowercase(), id.to_uppercase()]
    }

    /// Archive codecs are archive type (default).
    /// Override in subclasses for specific types like:
    /// - ['archive', 'compression']: Archives with compression
    /// - ['compression']: Pure compression formats
    // Python decorators: @property
    fn codec_types(&self) -> Vec<String> {
        vec!["archive".to_string()]
    }

    /// Compress data to archive bytes (user-friendly API).
    /// Delegates to encode().
    /// Args:
    /// data: Data to compress (dict, bytes, str, list, etc.)
    /// **options: Compression options
    /// Returns:
    /// Archive bytes
    fn compress(&self, data: serde_json::Value) -> Vec<u8> {
        self.encode(data)
    }

    /// Extract archive bytes to data (user-friendly API).
    /// Delegates to decode().
    /// Args:
    /// archive_bytes: Archive bytes to extract
    /// **options: Extraction options
    /// Returns:
    /// Extracted data
    fn extract(&self, archive_bytes: Vec<u8>) -> serde_json::Value {
        self.decode(archive_bytes)
    }

}

/// Abstract base class for archive file operations - follows I→A→XW pattern.
///
/// I: IArchiveFile (interface)
/// A: AArchiveFile (abstract base - this class)
/// XW: XWArchiveFile implementations (ZipFile, TarFile, etc.)
///
/// Extends AFile for file I/O and provides archive-specific methods.
/// Uses composition pattern: contains an IArchiver instance for compression.
///
/// This is for file-based archive operations - works on archive files on disk.
pub trait AArchiveFile: AFile + IArchiveFile {
    /// Get the underlying archiver codec - must implement in subclass.
    fn get_archiver(&self) -> IArchiver;

    /// Add files to archive - must implement in subclass.
    /// Should use self.get_archiver().compress() internally.
    fn add_files(&self, files: Vec<Path>) -> ();

    /// Extract archive to destination - must implement in subclass.
    /// Should use self.get_archiver().extract() internally.
    fn extract_to(&self, dest: Path) -> Vec<Path>;

    /// List files in archive - must implement in subclass.
    fn list_contents(&self) -> Vec<String>;

}

// Instantiate to get metadata
// Map extensions to format ID
// Check full suffix first (for .tar.gz, .tar.bz2, etc.)
/// Registry for archive formats - LIKE CodecRegistry!
///
/// Manages available archive formats and enables auto-detection.
///
/// Examples:
/// >>> registry = ArchiveFormatRegistry()
/// >>> registry.register(ZipArchiver)
/// >>> archiver = registry.get_by_extension("backup.zip")  # Auto-detect!
/// >>> archiver = registry.get_by_id("zip")
pub struct ArchiveFormatRegistry {
    formats: HashMap<String, Arc<dyn IArchiveFormat>>,
    extension_map: HashMap<String, String>,  // .zip -> "zip"
}

impl ArchiveFormatRegistry {
    /// Initialize registry.
    pub fn new() -> Self {
        Self {
            formats: HashMap::new(),
            extension_map: HashMap::new(),
        }
    }

    // Instantiate to get metadata
    // Map extensions to format ID
    /// Register an archive format.
    ///
    ///
    /// Args:
    /// format_instance: Archive format instance to register
    ///
    /// Example:
    /// >>> registry.register(Arc::new(ZipArchiver::new()))
    pub fn register(&mut self, format_instance: Arc<dyn IArchiveFormat>) {
        let format_id = format_instance.format_id().to_string();
        
        // Map extensions to format ID
        for ext in format_instance.file_extensions() {
            self.extension_map.insert(ext.to_lowercase(), format_id.clone());
        }
        
        // Store instance
        self.formats.insert(format_id, format_instance);
    }

    /// Get archiver by format ID.
    pub fn get_by_id(&self, format_id: &str) -> Option<Arc<dyn IArchiveFormat>> {
        self.formats.get(format_id).cloned()
    }

    // Check full suffix first (for .tar.gz, .tar.bz2, etc.)
    /// Get archiver by file extension (AUTO-DETECTION!).
    ///
    ///
    /// Args:
    /// path: File path
    ///
    ///
    /// Returns:
    /// Archiver instance or None
    ///
    /// Example:
    /// >>> archiver = registry.get_by_extension("backup.zip")
    /// >>> archiver = registry.get_by_extension("data.tar.gz")
    pub fn get_by_extension(&self, path: &str) -> Option<Arc<dyn IArchiveFormat>> {
        let path_obj = Path::new(path);
        
        // Get all suffixes
        let mut suffixes: Vec<String> = path_obj.extension()
            .map(|e| e.to_string_lossy().to_string())
            .into_iter()
            .collect();
        
        // Check if file_stem has additional extension (e.g., .tar.gz)
        if let Some(stem) = path_obj.file_stem() {
            let stem_path = Path::new(&stem.to_string_lossy());
            if let Some(stem_ext) = stem_path.extension() {
                suffixes.insert(0, stem_ext.to_string_lossy().to_string());
            }
        }
        
        // Check full suffix first (for .tar.gz, .tar.bz2, etc.)
        if suffixes.len() >= 2 {
            let full_suffix = format!(".{}.{}", suffixes[0], suffixes[1]);
            if let Some(format_id) = self.extension_map.get(&full_suffix.to_lowercase()) {
                return self.get_by_id(format_id);
            }
        }
        
        // Check single suffix
        if let Some(suffix) = path_obj.extension() {
            let suffix_str = format!(".{}", suffix.to_string_lossy());
            if let Some(format_id) = self.extension_map.get(&suffix_str.to_lowercase()) {
                return self.get_by_id(format_id);
            }
        }
        
        None
    }

    /// List all registered format IDs.
    pub fn list_formats(&self) -> Vec<String> {
        self.formats.keys().cloned().collect()
    }
}

/// Registry for compression algorithms.
pub struct CompressionRegistry {
    algorithms: HashMap<String, Arc<dyn ICompressor>>,
}

impl CompressionRegistry {
    /// Initialize registry.
    pub fn new() -> Self {
        Self {
            algorithms: HashMap::new(),
        }
    }

    /// Register a compression algorithm.
    pub fn register(&mut self, compressor_instance: Arc<dyn ICompressor>) {
        let algo_id = compressor_instance.algorithm_id().to_string();
        self.algorithms.insert(algo_id, compressor_instance);
    }

    /// Get compressor by algorithm ID.
    pub fn get(&self, algorithm_id: &str) -> Option<Arc<dyn ICompressor>> {
        self.algorithms.get(algorithm_id).cloned()
    }

    /// Auto-detect compression algorithm from data.
    pub fn auto_detect(&self, data: &[u8]) -> Option<Arc<dyn ICompressor>> {
        // Check magic bytes
        if data.len() >= 2 {
            if data[0] == 0x1f && data[1] == 0x8b {
                // gzip
                return self.get("gzip");
            } else if data.len() >= 2 && data.starts_with(b"BZ") {
                // bz2
                return self.get("bz2");
            } else if data.len() >= 6 && data[0] == 0xfd && data[1] == 0x37 && 
                     data[2] == 0x7a && data[3] == 0x58 && data[4] == 0x5a && data[5] == 0x00 {
                // xz/lzma
                return self.get("lzma");
            }
        }
        
        // Try each compressor's can_handle method if available
        for compressor in self.algorithms.values() {
            // Note: ICompressor doesn't have can_handle in the trait, so we skip this for now
            // In a full implementation, we'd add this method to the trait
        }
        
        None
    }

    /// List all registered algorithm IDs.
    pub fn list_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }
}

// Auto-register built-in formats (will be done in formats/__init__.py)
/// Get or create global archive format registry.
pub fn get_global_archive_registry() -> Arc<Mutex<ArchiveFormatRegistry>> {
    static REGISTRY: OnceLock<Arc<Mutex<ArchiveFormatRegistry>>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        Arc::new(Mutex::new(ArchiveFormatRegistry::new()))
    }).clone()
}

// Auto-register built-in compressors (will be done in compression/__init__.py)
/// Get or create global compression registry.
pub fn get_global_compression_registry() -> Arc<Mutex<CompressionRegistry>> {
    static REGISTRY: OnceLock<Arc<Mutex<CompressionRegistry>>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        Arc::new(Mutex::new(CompressionRegistry::new()))
    }).clone()
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Traits and structs are already public, no need for pub use
// Functions are module-level, accessible via module path
