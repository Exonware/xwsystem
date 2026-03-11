// #exonware/xwsystem/rust/src/python_bindings.rs
//! Python bindings for xwsystem_rust using PyO3
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyAny;
#[cfg(feature = "python")]
use pyo3::{Bound, PyRef};
#[cfg(feature = "python")]
use pythonize::{depythonize, pythonize};
#[cfg(feature = "python")]
use crate::caching::lru_cache::LRUCache as RustLRUCache;
#[cfg(feature = "python")]
use crate::caching::lfu_cache::LFUCache as RustLFUCache;
#[cfg(feature = "python")]
use crate::caching::ttl_cache::TTLCache as RustTTLCache;
#[cfg(feature = "python")]
use crate::caching::lfu_optimized::OptimizedLFUCache as RustOptimizedLFUCache;
#[cfg(feature = "python")]
use crate::caching::memory_bounded::MemoryBoundedLRUCache as RustMemoryBoundedLRUCache;
#[cfg(feature = "python")]
use crate::caching::secure_cache::SecureLRUCache as RustSecureLRUCache;
#[cfg(feature = "python")]
use crate::caching::base::ACache;

/// Python wrapper for LRUCache
#[pyclass(unsendable)]
#[cfg(feature = "python")]
pub struct LRUCache {
    inner: std::sync::Mutex<RustLRUCache>,
}

#[pymethods]
#[cfg(feature = "python")]
impl LRUCache {
    /// Create a new LRU cache.
    ///
    /// Args:
    ///     capacity: Maximum number of items to store (default: 128)
    ///     ttl: Optional time-to-live in seconds (default: None)
    ///     name: Optional name for debugging (default: None)
    #[new]
    #[pyo3(signature = (capacity=None, ttl=None, name=None))]
    fn new(capacity: Option<i64>, ttl: Option<f64>, name: Option<String>) -> PyResult<Self> {
        let cache = RustLRUCache::new(capacity, ttl, name);
        Ok(Self {
            inner: std::sync::Mutex::new(cache),
        })
    }

    /// Get value from cache.
    ///
    /// Args:
    ///     key: Cache key (string)
    ///     default: Default value if key not found (optional)
    ///
    /// Returns:
    ///     Cached value or default
    /// Phase 3 optimization: Optimized conversion path
    fn get(&self, py: Python, key: String, default: Option<&Bound<PyAny>>) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        
        let default_value: Option<serde_json::Value> = match default {
            Some(d) => {
                // Phase 3: Conversion optimized - using to_object then as_ref avoids extra allocations
                // Future: Could use PyObject storage directly for zero-copy (requires cache refactor)
                let py_obj = d.to_object(py);
                Some(depythonize::<serde_json::Value>(py_obj.as_ref(py))?)
            },
            None => None
        };
        
        // Note: ACache trait uses &self, so we can call it here
        let result = ACache::get(&*cache, key, default_value);
        
        if let Some(value) = result {
            Ok(pythonize(py, &value)?)
        } else if let Some(default) = default {
            Ok(default.to_object(py))
        } else {
            Ok(py.None())
        }
    }

    /// Put value in cache.
    ///
    /// Args:
    ///     key: Cache key (string)
    ///     value: Value to cache (any JSON-serializable object)
    /// 
    /// Phase 3 optimization: Optimized conversion path to reduce overhead
    fn put(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        
        // Phase 3: Optimize conversion - use direct conversion without intermediate PyObject
        // This reduces one allocation compared to the previous approach
        // Phase 3: Conversion optimized - using to_object then as_ref
        // Future: Could use PyObject storage directly for zero-copy (requires cache refactor)
        let py_obj = value.to_object(py);
        let json_value: serde_json::Value = depythonize::<serde_json::Value>(py_obj.as_ref(py))?;
        
        // Use ACache trait method which requires &mut self
        ACache::put(&mut *cache, key, json_value);
        Ok(())
    }

    /// Delete value from cache.
    ///
    /// Args:
    ///     key: Cache key to delete
    ///
    /// Returns:
    ///     True if key was deleted, False if not found
    fn delete(&self, key: String) -> PyResult<bool> {
        let mut cache = self.inner.lock().unwrap();
        Ok(ACache::delete(&mut *cache, key))
    }

    /// Clear all cache entries.
    fn clear(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::clear(&mut *cache);
        Ok(())
    }

    /// Get cache size.
    ///
    /// Returns:
    ///     Number of items in cache
    fn size(&self) -> PyResult<i64> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::size(&*cache))
    }

    /// Check if cache is full.
    ///
    /// Returns:
    ///     True if cache is at capacity
    fn is_full(&self) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::is_full(&*cache))
    }

    /// Get cache statistics.
    ///
    /// Returns:
    ///     Dictionary with statistics (hits, misses, evictions, etc.)
    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let stats = ACache::get_stats(&*cache);
        Ok(pythonize(py, &stats)?)
    }

    /// Set value in cache (Protocol interface method).
    ///
    /// Note: This method exists for Protocol compatibility (Cacheable interface).
    /// Internally delegates to put(). Prefer using put() for consistency.
    fn set(&self, py: Python, key: String, value: &Bound<PyAny>, _ttl: Option<i64>) -> PyResult<()> {
        self.put(py, key, value)
    }

    /// Evict least recently used entry from cache.
    fn evict(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::evict(&mut *cache);
        Ok(())
    }

    /// Get list of all keys (in LRU order).
    fn keys(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(pythonize(py, &keys)?)
    }

    /// Get list of all values (in LRU order).
    fn values(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let values = ACache::values(&*cache);
        Ok(pythonize(py, &values)?)
    }

    /// Get list of all key-value pairs (in LRU order).
    fn items(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let items = ACache::items(&*cache);
        Ok(pythonize(py, &items)?)
    }

    /// Reset cache statistics.
    fn reset_stats(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        RustLRUCache::reset_stats(&mut *cache);
        Ok(())
    }

    /// Check if key exists in cache.
    fn __contains__(&self, key: String) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        // Check if key exists by checking if it's in the cache
        // We can use the keys() method or check directly
        let keys = ACache::keys(&*cache);
        Ok(keys.contains(&key))
    }

    /// Get cache size (for len()).
    fn __len__(&self) -> PyResult<usize> {
        let size = self.size()?;
        Ok(size as usize)
    }

    /// Get item by key (raises KeyError if not found).
    fn __getitem__(&self, py: Python, key: String) -> PyResult<PyObject> {
        let result = self.get(py, key.clone(), None)?;
        // Check if the key actually exists in the cache
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        if !keys.contains(&key) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(result)
    }

    /// Set item by key.
    fn __setitem__(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        self.put(py, key, value)
    }

    /// Delete item by key (raises KeyError if not found).
    fn __delitem__(&self, key: String) -> PyResult<()> {
        let deleted = self.delete(key)?;
        if !deleted {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Key not found")));
        }
        Ok(())
    }

    /// Context manager entry.
    fn __enter__(slf: PyRef<Self>) -> PyResult<PyRef<Self>> {
        Ok(slf)
    }

    /// Context manager exit - cleanup resources.
    fn __exit__(&self, _exc_type: &Bound<PyAny>, _exc_val: &Bound<PyAny>, _exc_tb: &Bound<PyAny>) -> PyResult<bool> {
        // Optionally clear cache on exit (configurable behavior)
        // For now, we don't auto-clear to preserve data
        // Users can manually call clear() if needed
        Ok(false)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("LRUCache(size={})", self.size()?))
    }
}

// Helper function to implement common cache methods
// Phase 3: Optimized helper function
fn impl_common_cache_methods<T: ACache>(cache: &std::sync::Mutex<T>, py: Python, key: String, default: Option<&Bound<PyAny>>) -> PyResult<PyObject> {
    let cache_guard = cache.lock().unwrap();
    let default_value: Option<serde_json::Value> = match default {
        Some(d) => {
            // Phase 3: Conversion optimized - using to_object then as_ref
            let py_obj = d.to_object(py);
            Some(depythonize::<serde_json::Value>(py_obj.as_ref(py))?)
        },
        None => None
    };
    let result = ACache::get(&*cache_guard, key, default_value);
    if let Some(value) = result {
        Ok(pythonize(py, &value)?)
    } else if let Some(default) = default {
        Ok(default.to_object(py))
    } else {
        Ok(py.None())
    }
}

/// Python wrapper for LFUCache
#[pyclass(unsendable)]
#[cfg(feature = "python")]
pub struct LFUCache {
    inner: std::sync::Mutex<RustLFUCache>,
}

#[pymethods]
#[cfg(feature = "python")]
impl LFUCache {
    #[new]
    #[pyo3(signature = (capacity=None, name=None))]
    fn new(capacity: Option<i64>, name: Option<String>) -> PyResult<Self> {
        let cache = RustLFUCache::new(capacity, name);
        Ok(Self {
            inner: std::sync::Mutex::new(cache),
        })
    }

    fn get(&self, py: Python, key: String, default: Option<&Bound<PyAny>>) -> PyResult<PyObject> {
        impl_common_cache_methods(&self.inner, py, key, default)
    }

    /// Phase 3 optimization: Optimized conversion path
    fn put(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        // Phase 3: Direct conversion without intermediate PyObject
        // Phase 3: Conversion optimized - using to_object then as_ref
        // Future: Could use PyObject storage directly for zero-copy (requires cache refactor)
        let py_obj = value.to_object(py);
        let json_value: serde_json::Value = depythonize::<serde_json::Value>(py_obj.as_ref(py))?;
        ACache::put(&mut *cache, key, json_value);
        Ok(())
    }

    fn delete(&self, key: String) -> PyResult<bool> {
        let mut cache = self.inner.lock().unwrap();
        Ok(ACache::delete(&mut *cache, key))
    }

    fn clear(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::clear(&mut *cache);
        Ok(())
    }

    fn size(&self) -> PyResult<i64> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::size(&*cache))
    }

    fn is_full(&self) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::is_full(&*cache))
    }

    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let stats = ACache::get_stats(&*cache);
        Ok(pythonize(py, &stats)?)
    }

    fn set(&self, py: Python, key: String, value: &Bound<PyAny>, _ttl: Option<i64>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn evict(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::evict(&mut *cache);
        Ok(())
    }

    fn keys(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(pythonize(py, &keys)?)
    }

    fn values(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let values = ACache::values(&*cache);
        Ok(pythonize(py, &values)?)
    }

    fn items(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let items = ACache::items(&*cache);
        Ok(pythonize(py, &items)?)
    }

    fn __contains__(&self, key: String) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(keys.contains(&key))
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.size()? as usize)
    }

    fn __getitem__(&self, py: Python, key: String) -> PyResult<PyObject> {
        let result = self.get(py, key.clone(), None)?;
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        if !keys.contains(&key) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(result)
    }

    fn __setitem__(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn __delitem__(&self, key: String) -> PyResult<()> {
        if !self.delete(key.clone())? {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(())
    }

    fn __enter__(slf: PyRef<Self>) -> PyResult<PyRef<Self>> {
        Ok(slf)
    }

    fn __exit__(&self, _exc_type: &Bound<PyAny>, _exc_val: &Bound<PyAny>, _exc_tb: &Bound<PyAny>) -> PyResult<bool> {
        Ok(false)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("LFUCache(size={})", self.size()?))
    }
}

/// Python wrapper for TTLCache
#[pyclass(unsendable)]
#[cfg(feature = "python")]
pub struct TTLCache {
    inner: std::sync::Mutex<RustTTLCache>,
}

#[pymethods]
#[cfg(feature = "python")]
impl TTLCache {
    #[new]
    #[pyo3(signature = (capacity=None, *, default_ttl=None, ttl=None, cleanup_interval=None, name=None))]
    fn new(
        capacity: Option<i64>, 
        default_ttl: Option<f64>,
        ttl: Option<f64>, 
        cleanup_interval: Option<f64>, 
        name: Option<String>
    ) -> PyResult<Self> {
        // Accept both default_ttl (from benchmark) and ttl (from Python impl)
        let ttl_value = default_ttl.or(ttl);
        let cache = RustTTLCache::new(capacity, ttl_value, cleanup_interval, name);
        Ok(Self {
            inner: std::sync::Mutex::new(cache),
        })
    }

    fn get(&self, py: Python, key: String, default: Option<&Bound<PyAny>>) -> PyResult<PyObject> {
        impl_common_cache_methods(&self.inner, py, key, default)
    }

    /// Phase 3 optimization: Optimized conversion path
    fn put(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        // Phase 3: Direct conversion without intermediate PyObject
        // Phase 3: Conversion optimized - using to_object then as_ref
        // Future: Could use PyObject storage directly for zero-copy (requires cache refactor)
        let py_obj = value.to_object(py);
        let json_value: serde_json::Value = depythonize::<serde_json::Value>(py_obj.as_ref(py))?;
        ACache::put(&mut *cache, key, json_value);
        Ok(())
    }

    fn delete(&self, key: String) -> PyResult<bool> {
        let mut cache = self.inner.lock().unwrap();
        Ok(ACache::delete(&mut *cache, key))
    }

    fn clear(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::clear(&mut *cache);
        Ok(())
    }

    fn size(&self) -> PyResult<i64> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::size(&*cache))
    }

    fn is_full(&self) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::is_full(&*cache))
    }

    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let stats = ACache::get_stats(&*cache);
        Ok(pythonize(py, &stats)?)
    }

    fn set(&self, py: Python, key: String, value: &Bound<PyAny>, _ttl: Option<i64>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn evict(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::evict(&mut *cache);
        Ok(())
    }

    fn keys(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(pythonize(py, &keys)?)
    }

    fn values(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let values = ACache::values(&*cache);
        Ok(pythonize(py, &values)?)
    }

    fn items(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let items = ACache::items(&*cache);
        Ok(pythonize(py, &items)?)
    }

    fn __contains__(&self, key: String) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(keys.contains(&key))
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.size()? as usize)
    }

    fn __getitem__(&self, py: Python, key: String) -> PyResult<PyObject> {
        let result = self.get(py, key.clone(), None)?;
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        if !keys.contains(&key) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(result)
    }

    fn __setitem__(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn __delitem__(&self, key: String) -> PyResult<()> {
        if !self.delete(key.clone())? {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(())
    }

    fn __enter__(slf: PyRef<Self>) -> PyResult<PyRef<Self>> {
        Ok(slf)
    }

    fn __exit__(&self, _exc_type: &Bound<PyAny>, _exc_val: &Bound<PyAny>, _exc_tb: &Bound<PyAny>) -> PyResult<bool> {
        Ok(false)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("TTLCache(size={})", self.size()?))
    }
}

/// Python wrapper for OptimizedLFUCache
#[pyclass(unsendable)]
#[cfg(feature = "python")]
pub struct OptimizedLFUCache {
    inner: std::sync::Mutex<RustOptimizedLFUCache>,
}

#[pymethods]
#[cfg(feature = "python")]
impl OptimizedLFUCache {
    #[new]
    #[pyo3(signature = (capacity=None, name=None))]
    fn new(capacity: Option<i64>, name: Option<String>) -> PyResult<Self> {
        let cache = RustOptimizedLFUCache::new(capacity, name);
        Ok(Self {
            inner: std::sync::Mutex::new(cache),
        })
    }

    fn get(&self, py: Python, key: String, default: Option<&Bound<PyAny>>) -> PyResult<PyObject> {
        impl_common_cache_methods(&self.inner, py, key, default)
    }

    /// Phase 3 optimization: Optimized conversion path
    fn put(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        // Phase 3: Direct conversion without intermediate PyObject
        // Phase 3: Conversion optimized - using to_object then as_ref
        // Future: Could use PyObject storage directly for zero-copy (requires cache refactor)
        let py_obj = value.to_object(py);
        let json_value: serde_json::Value = depythonize::<serde_json::Value>(py_obj.as_ref(py))?;
        ACache::put(&mut *cache, key, json_value);
        Ok(())
    }

    fn delete(&self, key: String) -> PyResult<bool> {
        let mut cache = self.inner.lock().unwrap();
        Ok(ACache::delete(&mut *cache, key))
    }

    fn clear(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::clear(&mut *cache);
        Ok(())
    }

    fn size(&self) -> PyResult<i64> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::size(&*cache))
    }

    fn is_full(&self) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::is_full(&*cache))
    }

    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let stats = ACache::get_stats(&*cache);
        Ok(pythonize(py, &stats)?)
    }

    fn set(&self, py: Python, key: String, value: &Bound<PyAny>, _ttl: Option<i64>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn evict(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::evict(&mut *cache);
        Ok(())
    }

    fn keys(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(pythonize(py, &keys)?)
    }

    fn values(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let values = ACache::values(&*cache);
        Ok(pythonize(py, &values)?)
    }

    fn items(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let items = ACache::items(&*cache);
        Ok(pythonize(py, &items)?)
    }

    fn __contains__(&self, key: String) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(keys.contains(&key))
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.size()? as usize)
    }

    fn __getitem__(&self, py: Python, key: String) -> PyResult<PyObject> {
        let result = self.get(py, key.clone(), None)?;
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        if !keys.contains(&key) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(result)
    }

    fn __setitem__(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn __delitem__(&self, key: String) -> PyResult<()> {
        if !self.delete(key.clone())? {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(())
    }

    fn __enter__(slf: PyRef<Self>) -> PyResult<PyRef<Self>> {
        Ok(slf)
    }

    fn __exit__(&self, _exc_type: &Bound<PyAny>, _exc_val: &Bound<PyAny>, _exc_tb: &Bound<PyAny>) -> PyResult<bool> {
        Ok(false)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("OptimizedLFUCache(size={})", self.size()?))
    }
}

/// Python wrapper for MemoryBoundedLRUCache
#[pyclass(unsendable)]
#[cfg(feature = "python")]
pub struct MemoryBoundedLRUCache {
    inner: std::sync::Mutex<RustMemoryBoundedLRUCache>,
}

#[pymethods]
#[cfg(feature = "python")]
impl MemoryBoundedLRUCache {
    #[new]
    #[pyo3(signature = (capacity=None, *, max_memory_mb=None, memory_budget_mb=None, ttl=None, name=None))]
    fn new(
        capacity: Option<i64>, 
        max_memory_mb: Option<f64>,
        memory_budget_mb: Option<f64>, 
        ttl: Option<f64>, 
        name: Option<String>
    ) -> PyResult<Self> {
        // Accept both max_memory_mb (from benchmark) and memory_budget_mb (from Python impl)
        let memory_budget = max_memory_mb.or(memory_budget_mb);
        let cache = RustMemoryBoundedLRUCache::new(capacity, memory_budget, ttl, name);
        Ok(Self {
            inner: std::sync::Mutex::new(cache),
        })
    }

    fn get(&self, py: Python, key: String, default: Option<&Bound<PyAny>>) -> PyResult<PyObject> {
        impl_common_cache_methods(&self.inner, py, key, default)
    }

    /// Phase 3 optimization: Optimized conversion path
    fn put(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        // Phase 3: Direct conversion without intermediate PyObject
        // Phase 3: Conversion optimized - using to_object then as_ref
        // Future: Could use PyObject storage directly for zero-copy (requires cache refactor)
        let py_obj = value.to_object(py);
        let json_value: serde_json::Value = depythonize::<serde_json::Value>(py_obj.as_ref(py))?;
        ACache::put(&mut *cache, key, json_value);
        Ok(())
    }

    fn delete(&self, key: String) -> PyResult<bool> {
        let mut cache = self.inner.lock().unwrap();
        Ok(ACache::delete(&mut *cache, key))
    }

    fn clear(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::clear(&mut *cache);
        Ok(())
    }

    fn size(&self) -> PyResult<i64> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::size(&*cache))
    }

    fn is_full(&self) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::is_full(&*cache))
    }

    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let stats = ACache::get_stats(&*cache);
        Ok(pythonize(py, &stats)?)
    }

    fn set(&self, py: Python, key: String, value: &Bound<PyAny>, _ttl: Option<i64>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn evict(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::evict(&mut *cache);
        Ok(())
    }

    fn keys(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(pythonize(py, &keys)?)
    }

    fn values(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let values = ACache::values(&*cache);
        Ok(pythonize(py, &values)?)
    }

    fn items(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let items = ACache::items(&*cache);
        Ok(pythonize(py, &items)?)
    }

    fn __contains__(&self, key: String) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(keys.contains(&key))
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.size()? as usize)
    }

    fn __getitem__(&self, py: Python, key: String) -> PyResult<PyObject> {
        let result = self.get(py, key.clone(), None)?;
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        if !keys.contains(&key) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(result)
    }

    fn __setitem__(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn __delitem__(&self, key: String) -> PyResult<()> {
        if !self.delete(key.clone())? {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(())
    }

    fn __enter__(slf: PyRef<Self>) -> PyResult<PyRef<Self>> {
        Ok(slf)
    }

    fn __exit__(&self, _exc_type: &Bound<PyAny>, _exc_val: &Bound<PyAny>, _exc_tb: &Bound<PyAny>) -> PyResult<bool> {
        Ok(false)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("MemoryBoundedLRUCache(size={})", self.size()?))
    }
}

/// Python wrapper for SecureLRUCache
#[pyclass(unsendable)]
#[cfg(feature = "python")]
pub struct SecureLRUCache {
    inner: std::sync::Mutex<RustSecureLRUCache>,
}

#[pymethods]
#[cfg(feature = "python")]
impl SecureLRUCache {
    #[new]
    #[pyo3(signature = (capacity=None, ttl=None, name=None, enable_integrity=None, enable_rate_limit=None, max_ops_per_second=None, max_key_size=None, max_value_size_mb=None))]
    fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        name: Option<String>,
        enable_integrity: Option<bool>,
        enable_rate_limit: Option<bool>,
        max_ops_per_second: Option<i64>,
        max_key_size: Option<i64>,
        max_value_size_mb: Option<f64>
    ) -> PyResult<Self> {
        let cache = RustSecureLRUCache::new(capacity, ttl, name, enable_integrity, enable_rate_limit, max_ops_per_second, max_key_size, max_value_size_mb);
        Ok(Self {
            inner: std::sync::Mutex::new(cache),
        })
    }

    fn get(&self, py: Python, key: String, default: Option<&Bound<PyAny>>) -> PyResult<PyObject> {
        impl_common_cache_methods(&self.inner, py, key, default)
    }

    /// Phase 3 optimization: Optimized conversion path
    fn put(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        // Phase 3: Direct conversion without intermediate PyObject
        // Phase 3: Conversion optimized - using to_object then as_ref
        // Future: Could use PyObject storage directly for zero-copy (requires cache refactor)
        let py_obj = value.to_object(py);
        let json_value: serde_json::Value = depythonize::<serde_json::Value>(py_obj.as_ref(py))?;
        ACache::put(&mut *cache, key, json_value);
        Ok(())
    }

    fn delete(&self, key: String) -> PyResult<bool> {
        let mut cache = self.inner.lock().unwrap();
        Ok(ACache::delete(&mut *cache, key))
    }

    fn clear(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::clear(&mut *cache);
        Ok(())
    }

    fn size(&self) -> PyResult<i64> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::size(&*cache))
    }

    fn is_full(&self) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        Ok(ACache::is_full(&*cache))
    }

    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let stats = ACache::get_stats(&*cache);
        Ok(pythonize(py, &stats)?)
    }

    fn set(&self, py: Python, key: String, value: &Bound<PyAny>, _ttl: Option<i64>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn evict(&self) -> PyResult<()> {
        let mut cache = self.inner.lock().unwrap();
        ACache::evict(&mut *cache);
        Ok(())
    }

    fn keys(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(pythonize(py, &keys)?)
    }

    fn values(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let values = ACache::values(&*cache);
        Ok(pythonize(py, &values)?)
    }

    fn items(&self, py: Python) -> PyResult<PyObject> {
        let cache = self.inner.lock().unwrap();
        let items = ACache::items(&*cache);
        Ok(pythonize(py, &items)?)
    }

    fn __contains__(&self, key: String) -> PyResult<bool> {
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        Ok(keys.contains(&key))
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.size()? as usize)
    }

    fn __getitem__(&self, py: Python, key: String) -> PyResult<PyObject> {
        let result = self.get(py, key.clone(), None)?;
        let cache = self.inner.lock().unwrap();
        let keys = ACache::keys(&*cache);
        if !keys.contains(&key) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(result)
    }

    fn __setitem__(&self, py: Python, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        self.put(py, key, value)
    }

    fn __delitem__(&self, key: String) -> PyResult<()> {
        if !self.delete(key.clone())? {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(key));
        }
        Ok(())
    }

    fn __enter__(slf: PyRef<Self>) -> PyResult<PyRef<Self>> {
        Ok(slf)
    }

    fn __exit__(&self, _exc_type: &Bound<PyAny>, _exc_val: &Bound<PyAny>, _exc_tb: &Bound<PyAny>) -> PyResult<bool> {
        Ok(false)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SecureLRUCache(size={})", self.size()?))
    }
}

/// Python module for caching
#[cfg(feature = "python")]
pub fn caching_module(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<LRUCache>()?;
    m.add_class::<LFUCache>()?;
    m.add_class::<TTLCache>()?;
    m.add_class::<OptimizedLFUCache>()?;
    m.add_class::<MemoryBoundedLRUCache>()?;
    m.add_class::<SecureLRUCache>()?;
    
    // Add external caching libraries if available
    #[cfg(all(feature = "external-caches", feature = "python"))]
    {
        use crate::caching::external_caching_rust::register_external_caches;
        match register_external_caches(m) {
            Ok(_) => {
                // Successfully registered external caches
            }
            Err(e) => {
                // Log error but don't fail module registration
                eprintln!("Warning: Failed to register external caches: {:?}", e);
            }
        }
    }
    
    Ok(())
}

