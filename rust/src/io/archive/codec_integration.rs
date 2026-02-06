// #exonware/xwsystem/rust/src/io/archive/codec_integration.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/codec_integration.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: November 1, 2025
//! 
//! Archive-Codec Integration - Register archivers with CodecRegistry.
//! 
//! This enables:
//! - file.convert("zip", "7z") - Convert between archive formats
//! - file.save_as(path, "7z") - Save with different format
//! - get_codec_by_id("7z") - Get archiver as codec
//! 
//! Priority 1 (Security): Safe format conversion
//! Priority 2 (Usability): Seamless integration
//! Priority 3 (Maintainability): Single registry
//! Priority 4 (Performance): Cached instances
//! Priority 5 (Extensibility): Auto-registration


use std::sync::Arc;
use crate::archive::archivers::{TarArchiver, ZipArchiver};
use crate::codec::registry::get_registry;
use crate::contracts::ICodec;

// Auto-register on import

// Register codec-based archivers with UniversalCodecRegistry
/// Register all archivers with the UniversalCodecRegistry.
///
/// This enables:
/// 1. get_registry().get_by_id("zip") → ZipArchiver
/// 2. get_registry().get_by_id("tar") → TarArchiver
/// 3. Unified codec discovery across all formats
///
/// NOTE: Archivers implement IArchiver (which extends ICodec) and follow
/// the I→A→XW pattern with full codec metadata support.
pub fn register_archivers_as_codecs() {
    let registry = get_registry();
    
    // Register codec-based archivers with UniversalCodecRegistry
    // Note: The registry.register() method signature may need adjustment
    // For now, we attempt to register - this will work once the registry is fully implemented
    
    // Register ZipArchiver
    let zip_archiver = Arc::new(ZipArchiver::new());
    // registry.register("ZipArchiver".to_string(), Some(Box::new(zip_archiver) as Box<dyn ICodec>), None, None);
    // TODO: Uncomment when registry.register() is fully implemented
    
    // Register TarArchiver
    let tar_archiver = Arc::new(TarArchiver::new());
    // registry.register("TarArchiver".to_string(), Some(Box::new(tar_archiver) as Box<dyn ICodec>), None, None);
    // TODO: Uncomment when registry.register() is fully implemented
    
    // Note: Registration is commented out because the codec registry's register method
    // is not yet fully implemented. Once it is, uncomment these lines.
}

// Auto-register on module load (when this module is imported)
// In Rust, we can't auto-execute on import, so this needs to be called explicitly
// or we can use a lazy_static/once_cell pattern
use once_cell::sync::OnceCell;

static REGISTERED: OnceCell<()> = OnceCell::new();

pub fn ensure_registered() {
    REGISTERED.get_or_init(|| {
        register_archivers_as_codecs();
    });
}
