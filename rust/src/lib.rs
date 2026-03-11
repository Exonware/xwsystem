// #exonware/xwsystem/rust/src/lib.rs
//exonware/xwsystem/__init__.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 10, 2025
//! 
//! XWSystem - Enterprise-grade Python framework with AI-powered performance optimization.
//! 
//! Minimal build for caching module only.

// Caching module
pub mod caching;

// Re-export caching types
pub use caching::{
    AsyncLFUCache, AsyncLRUCache, CacheConfig, CacheManager, CacheStats, 
    LFUCache, LRUCache, AsyncTTLCache, TTLCache
};

// Python bindings module (only compiled when python feature is enabled)
#[cfg(feature = "python")]
pub mod python_bindings;

// Register the main Python module
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn _xwsystem_rust(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    use python_bindings::caching_module;
    
    // Create caching submodule
    let caching_submodule = PyModule::new_bound(_py, "caching")?;
    caching_module(_py, &caching_submodule)?;
    m.add_submodule(&caching_submodule)?;
    
    Ok(())
}