// #exonware/xwsystem/rust/src/caching/integrity.rs
//exonware/xwsystem/caching/integrity.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Cache integrity verification - Security Priority #1.
//! Protects against cache poisoning and tampering.


use crate::caching::errors::CacheIntegrityError;
use crate::caching::utils::compute_checksum;

/// Cache entry with integrity verification.
///
/// Attributes:
/// key: Cache key
/// value: Cached value
/// checksum: Integrity checksum (SHA256 by default)
/// created_at: Creation timestamp
/// access_count: Number of accesses
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheEntry {
    pub key: serde_json::Value,
    pub value: serde_json::Value,
    pub checksum: String,
    pub created_at: f64,
    pub access_count: i64,
}

impl CacheEntry {
    /// Verify entry integrity.
    ///
    ///
    /// Returns:
    /// True if integrity check passes
    ///
    ///
    /// Raises:
    /// CacheIntegrityError: If integrity check fails
    pub fn verify_integrity(&self) -> Result<bool, CacheIntegrityError> {
        // Compute current checksum
        let current_checksum = compute_checksum(&self.value, Some("sha256"));
        
        if current_checksum != self.checksum {
            return Err(CacheIntegrityError::new(format!(
                "Cache entry integrity check failed. Expected checksum: {}, got: {}. Possible tampering detected.",
                self.checksum, current_checksum
            )));
        }
        
        Ok(true)
    }
    
    /// Increment access count.
    pub fn increment_access(&mut self) {
        self.access_count += 1;
    }
}

/// Create cache entry with integrity checksum.
///
///
/// Args:
/// key: Cache key
/// value: Value to cache
/// created_at: Creation timestamp
/// algorithm: Hash algorithm for checksum
///
///
/// Returns:
/// CacheEntry with integrity checksum
pub fn create_secure_entry(key: serde_json::Value, value: serde_json::Value, created_at: f64, algorithm: Option<&str>) -> CacheEntry {
    let algo = algorithm.unwrap_or("sha256");
    let checksum = compute_checksum(&value, Some(algo));
    
    CacheEntry {
        key,
        value,
        checksum,
        created_at,
        access_count: 0,
    }
}

/// Verify cache entry integrity.
///
///
/// Args:
/// entry: Cache entry to verify
///
///
/// Returns:
/// True if integrity check passes
///
///
/// Raises:
/// CacheIntegrityError: If integrity check fails
pub fn verify_entry_integrity(entry: &CacheEntry) -> Result<bool, CacheIntegrityError> {
    entry.verify_integrity()
}

/// Update entry checksum after value modification.
///
///
/// Args:
/// entry: Cache entry to update
///
///
/// Note:
/// Call this after modifying the entry value.
pub fn update_entry_checksum(entry: &mut CacheEntry) {
    entry.checksum = compute_checksum(&entry.value, Some("sha256"));
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
