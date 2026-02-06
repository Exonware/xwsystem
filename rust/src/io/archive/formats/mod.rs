// #exonware/xwsystem/rust/src/io/archive/formats/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/formats/__init__.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! Archive format implementations - COMPREHENSIVE 2025 SUPPORT.
//! 
//! Pluggable archive formats registered with ArchiveFormatRegistry.
//! 
//! SUPPORTED FORMATS (Ranked by compression quality):
//! 
//! | Rank | Format         | Type       | Compression       | Use Case                        |
//! |------|----------------|------------|-------------------|---------------------------------|
//! | 1    | 7z             | Container  | LZMA2             | Best ratio + AES-256            |
//! | 2    | Zstandard      | Stream     | Zstd              | Fast modern (backups/DBs)       |
//! | 3    | RAR5           | Container  | Proprietary       | Strong + recovery               |
//! | 4    | ZIP/ZIPX       | Container  | Deflate/LZMA      | Widely supported                |
//! | 5    | tar.zst/tar.xz | Container  | Zstd/LZMA2        | Linux backups                   |
//! | 6    | Brotli         | Stream     | Brotli            | Web & text assets               |
//! | 7    | LZ4            | Stream     | LZ4               | Ultra fast real-time            |
//! | 8    | ZPAQ           | Journaled  | PAQ               | Extreme compression (archival)  |
//! | 9    | WIM            | Container  | LZX               | Windows system images           |
//! | 10   | SquashFS       | Filesystem | LZMA/LZ4          | Embedded systems                |
//! 
//! Priority 1 (Security): Safe format operations
//! Priority 2 (Usability): Auto-registration + lazy install
//! Priority 3 (Maintainability): Modular formats
//! Priority 4 (Performance): Efficient format handling
//! Priority 5 (Extensibility): Easy to add more formats

// Standard formats (always available)
pub mod tar;
pub mod zip;

// Compression formats
pub mod brotli_format;
pub mod lz4_format;
pub mod zstandard;

// Complex formats (may require external binaries)
pub mod sevenzip;
pub mod rar;
pub mod squashfs_format;
pub mod wim_format;
pub mod zpaq_format;

pub use tar::TarArchiver;
pub use zip::ZipArchiver;
pub use brotli_format::BrotliArchiver;
pub use lz4_format::Lz4Archiver;
pub use zstandard::ZstandardArchiver;
pub use sevenzip::SevenZipArchiver;
pub use rar::RarArchiver;
pub use squashfs_format::SquashfsArchiver;
pub use wim_format::WimArchiver;
pub use zpaq_format::ZpaqArchiver;

use std::sync::Arc;
use once_cell::sync::OnceCell;
use crate::archive::base::{get_global_archive_registry};
use crate::contracts::IArchiveFormat;

// Auto-register built-in formats
static INITIALIZED: OnceCell<()> = OnceCell::new();

fn ensure_registered() {
    INITIALIZED.get_or_init(|| {
        let registry = get_global_archive_registry();
        let mut reg = registry.lock().unwrap();
        // Standard formats
        reg.register(Arc::new(ZipArchiver::new()));
        reg.register(Arc::new(TarArchiver::new()));
        // Compression formats
        reg.register(Arc::new(BrotliArchiver::new()));
        reg.register(Arc::new(Lz4Archiver::new()));
        reg.register(Arc::new(ZstandardArchiver::new()));
        // Complex formats
        reg.register(Arc::new(SevenZipArchiver::new()));
        reg.register(Arc::new(RarArchiver::new()));
        reg.register(Arc::new(SquashfsArchiver::new()));
        reg.register(Arc::new(WimArchiver::new()));
        reg.register(Arc::new(ZpaqArchiver::new()));
    });
}

/// Get archiver by file extension (auto-detection!).
///
/// Examples:
/// >>> get_archiver_for_file("backup.zip")  # Returns ZipArchiver
/// >>> get_archiver_for_file("data.tar.gz")  # Returns TarArchiver
pub fn get_archiver_for_file(path: &str) -> Option<Arc<dyn IArchiveFormat>> {
    ensure_registered();
    let registry = get_global_archive_registry();
    let reg = registry.lock().unwrap();
    reg.get_by_extension(path)
}

/// Get archiver by format ID.
///
/// Examples:
/// >>> get_archiver_by_id("zip")  # Returns ZipArchiver
/// >>> get_archiver_by_id("tar")  # Returns TarArchiver
pub fn get_archiver_by_id(format_id: &str) -> Option<Arc<dyn IArchiveFormat>> {
    ensure_registered();
    let registry = get_global_archive_registry();
    let reg = registry.lock().unwrap();
    reg.get_by_id(format_id)
}

/// Register a custom archive format.
pub fn register_archive_format(format_instance: Arc<dyn IArchiveFormat>) {
    let registry = get_global_archive_registry();
    let mut reg = registry.lock().unwrap();
    reg.register(format_instance);
}
