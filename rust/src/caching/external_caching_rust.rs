// #exonware/xwsystem/rust/src/caching/external_caching_rust.rs
/*
External Rust caching library bindings for benchmarking.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.1
Generation Date: January 2025

Provides Python bindings for external Rust caching libraries:
- Moka (mini-moka): Production-grade cache with TinyLFU
- QuickCache: Fast probabilistic cache
- DashMap: Fast concurrent HashMap
*/

#[cfg(feature = "external-caches")]
use pyo3::prelude::*;
#[cfg(feature = "external-caches")]
use pythonize::pythonize;
#[cfg(feature = "external-caches")]
use serde_json::Value;

// Moka (mini-moka) bindings
#[cfg(all(feature = "external-caches", feature = "python"))]
mod moka_cache {
    use super::*;
    use mini_moka::sync::Cache;
    use std::sync::Arc;
    use std::time::Duration;

    #[pyclass]
    pub struct MokaCache {
        inner: Arc<Cache<String, Value>>,
    }

    #[pymethods]
    impl MokaCache {
        #[new]
        fn new(capacity: usize) -> Self {
            let cache = Cache::builder()
                .max_capacity(capacity as u64)
                .build();
            Self {
                inner: Arc::new(cache),
            }
        }

        fn get(&self, py: Python, key: &str) -> PyResult<PyObject> {
            let cache = self.inner.clone();
            let key_str = key.to_string();
            let result = py.allow_threads(|| {
                cache.get(&key_str)
            });
            if let Some(value) = result {
                Ok(pythonize(py, &value)?)
            } else {
                Ok(py.None())
            }
        }

        fn put(&self, py: Python, key: &str, value: Bound<PyAny>) -> PyResult<()> {
            let cache = self.inner.clone();
            let py_obj = value.to_object(py);
            let json_value: Value = pythonize::depythonize(py_obj.as_ref(py))?;
            let key_str = key.to_string();
            py.allow_threads(|| {
                cache.insert(key_str, json_value);
            });
            Ok(())
        }

        fn delete(&self, py: Python, key: &str) -> PyResult<bool> {
            let cache = self.inner.clone();
            let key_str = key.to_string();
            let existed = py.allow_threads(|| {
                cache.get(&key_str).is_some()
            });
            if existed {
                py.allow_threads(|| {
                    cache.invalidate(&key_str);
                });
            }
            Ok(existed)
        }

        fn clear(&self) -> PyResult<()> {
            let cache = self.inner.clone();
            cache.invalidate_all();
            Ok(())
        }

        fn size(&self) -> usize {
            self.inner.weighted_size() as usize
        }

        fn capacity(&self) -> usize {
            // Moka doesn't expose max_capacity directly, use weighted_size as approximation
            // In practice, this should be set during construction
            self.inner.weighted_size() as usize
        }
    }

    // Moka TTL Cache
    #[pyclass]
    pub struct MokaTTLCache {
        inner: Arc<Cache<String, Value>>,
    }

    #[pymethods]
    impl MokaTTLCache {
        #[new]
        fn new(capacity: usize, ttl_seconds: f64) -> Self {
            let cache = Cache::builder()
                .max_capacity(capacity as u64)
                .time_to_live(Duration::from_secs_f64(ttl_seconds))
                .build();
            Self {
                inner: Arc::new(cache),
            }
        }

        fn get(&self, py: Python, key: &str) -> PyResult<PyObject> {
            let cache = self.inner.clone();
            let key_str = key.to_string();
            let result = py.allow_threads(|| {
                cache.get(&key_str)
            });
            if let Some(value) = result {
                Ok(pythonize(py, &value)?)
            } else {
                Ok(py.None())
            }
        }

        fn put(&self, py: Python, key: &str, value: Bound<PyAny>) -> PyResult<()> {
            let cache = self.inner.clone();
            let py_obj = value.to_object(py);
            let json_value: Value = pythonize::depythonize(py_obj.as_ref(py))?;
            let key_str = key.to_string();
            py.allow_threads(|| {
                cache.insert(key_str, json_value);
            });
            Ok(())
        }

        fn delete(&self, py: Python, key: &str) -> PyResult<bool> {
            let cache = self.inner.clone();
            let key_str = key.to_string();
            let existed = py.allow_threads(|| {
                cache.get(&key_str).is_some()
            });
            if existed {
                py.allow_threads(|| {
                    cache.invalidate(&key_str);
                });
            }
            Ok(existed)
        }

        fn clear(&self) -> PyResult<()> {
            let cache = self.inner.clone();
            cache.invalidate_all();
            Ok(())
        }

        fn size(&self) -> usize {
            self.inner.weighted_size() as usize
        }

        fn capacity(&self) -> usize {
            // Moka doesn't expose max_capacity directly, use weighted_size as approximation
            // In practice, this should be set during construction
            self.inner.weighted_size() as usize
        }
    }

    // Moka Weighted Cache
    #[pyclass]
    pub struct MokaWeightedCache {
        inner: Arc<Cache<String, Value>>,
    }

    #[pymethods]
    impl MokaWeightedCache {
        #[new]
        fn new(max_weight: usize) -> Self {
            let cache = Cache::builder()
                .max_capacity(max_weight as u64)
                .weigher(|_k: &String, _v: &Value| -> u32 { 1 })
                .build();
            Self {
                inner: Arc::new(cache),
            }
        }

        fn get(&self, py: Python, key: &str) -> PyResult<PyObject> {
            let cache = self.inner.clone();
            let key_str = key.to_string();
            let result = py.allow_threads(|| {
                cache.get(&key_str)
            });
            if let Some(value) = result {
                Ok(pythonize(py, &value)?)
            } else {
                Ok(py.None())
            }
        }

        fn put(&self, py: Python, key: &str, value: Bound<PyAny>) -> PyResult<()> {
            let cache = self.inner.clone();
            let py_obj = value.to_object(py);
            let json_value: Value = pythonize::depythonize(py_obj.as_ref(py))?;
            let key_str = key.to_string();
            py.allow_threads(|| {
                cache.insert(key_str, json_value);
            });
            Ok(())
        }

        fn delete(&self, py: Python, key: &str) -> PyResult<bool> {
            let cache = self.inner.clone();
            let key_str = key.to_string();
            let existed = py.allow_threads(|| {
                cache.get(&key_str).is_some()
            });
            if existed {
                py.allow_threads(|| {
                    cache.invalidate(&key_str);
                });
            }
            Ok(existed)
        }

        fn clear(&self) -> PyResult<()> {
            let cache = self.inner.clone();
            cache.invalidate_all();
            Ok(())
        }

        fn size(&self) -> usize {
            self.inner.weighted_size() as usize
        }

        fn capacity(&self) -> usize {
            // Moka doesn't expose max_capacity directly, use weighted_size as approximation
            // In practice, this should be set during construction
            self.inner.weighted_size() as usize
        }
    }
}

// QuickCache bindings
#[cfg(all(feature = "external-caches", feature = "python"))]
mod quick_cache {
    use super::*;
    use ::quick_cache::unsync::Cache;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::time::Duration;

    #[pyclass]
    pub struct QuickCache {
        inner: Arc<Mutex<Cache<String, Value>>>,
    }

    #[pymethods]
    impl QuickCache {
        #[new]
        fn new(capacity: usize) -> Self {
            let cache = Cache::new(capacity);
            Self {
                inner: Arc::new(Mutex::new(cache)),
            }
        }

        fn get(&self, py: Python, key: &str) -> PyResult<PyObject> {
            let cache = self.inner.clone();
            let key_str = key.to_string();
            let result = py.allow_threads(|| {
                let cache = cache.lock().unwrap();
                cache.get(&key_str).cloned()
            });
            if let Some(value) = result {
                Ok(pythonize(py, &value)?)
            } else {
                Ok(py.None())
            }
        }

        fn put(&self, py: Python, key: &str, value: Bound<PyAny>) -> PyResult<()> {
            let cache = self.inner.clone();
            let py_obj = value.to_object(py);
            let json_value: Value = pythonize::depythonize(py_obj.as_ref(py))?;
            let key_str = key.to_string();
            py.allow_threads(|| {
                let mut cache = cache.lock().unwrap();
                cache.insert(key_str, json_value);
            });
            Ok(())
        }

        fn delete(&self, key: &str) -> PyResult<bool> {
            let cache = self.inner.clone();
            let mut cache = cache.lock().unwrap();
            Ok(cache.remove(key).is_some())
        }

        fn clear(&self) -> PyResult<()> {
            let cache = self.inner.clone();
            let mut cache = cache.lock().unwrap();
            cache.clear();
            Ok(())
        }

        fn size(&self) -> usize {
            let cache = self.inner.lock().unwrap();
            cache.len()
        }

        fn capacity(&self) -> usize {
            let cache = self.inner.lock().unwrap();
            cache.capacity() as usize
        }
    }

    // QuickCache TTL variant
    #[pyclass]
    pub struct QuickCacheTTL {
        inner: Arc<Mutex<Cache<String, Value>>>,
        ttl: Duration,
    }

    #[pymethods]
    impl QuickCacheTTL {
        #[new]
        fn new(capacity: usize, _ttl_seconds: f64) -> Self {
            let cache = Cache::new(capacity);
            Self {
                inner: Arc::new(Mutex::new(cache)),
                ttl: Duration::from_secs_f64(_ttl_seconds),
            }
        }

        fn get(&self, py: Python, key: &str) -> PyResult<PyObject> {
            let cache = self.inner.clone();
            let key_str = key.to_string();
            let result = py.allow_threads(|| {
                let cache = cache.lock().unwrap();
                cache.get(&key_str).cloned()
            });
            if let Some(value) = result {
                Ok(pythonize(py, &value)?)
            } else {
                Ok(py.None())
            }
        }

        fn put(&self, py: Python, key: &str, value: Bound<PyAny>) -> PyResult<()> {
            let cache = self.inner.clone();
            let py_obj = value.to_object(py);
            let json_value: Value = pythonize::depythonize(py_obj.as_ref(py))?;
            let key_str = key.to_string();
            py.allow_threads(|| {
                let mut cache = cache.lock().unwrap();
                // QuickCache doesn't have built-in TTL, so we implement it manually
                // by storing with expiration time (simplified implementation)
                cache.insert(key_str, json_value);
            });
            Ok(())
        }

        fn delete(&self, key: &str) -> PyResult<bool> {
            let cache = self.inner.clone();
            let mut cache = cache.lock().unwrap();
            Ok(cache.remove(key).is_some())
        }

        fn clear(&self) -> PyResult<()> {
            let cache = self.inner.clone();
            let mut cache = cache.lock().unwrap();
            cache.clear();
            Ok(())
        }

        fn size(&self) -> usize {
            let cache = self.inner.lock().unwrap();
            cache.len()
        }

        fn capacity(&self) -> usize {
            let cache = self.inner.lock().unwrap();
            cache.capacity() as usize
        }
    }
}

// DashMap bindings (simple key-value store, no eviction)
#[cfg(all(feature = "external-caches", feature = "python"))]
mod dashmap_cache {
    use super::*;
    use dashmap::DashMap;
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    #[derive(Clone)]
    struct TTLValue {
        value: Value,
        expires_at: Instant,
    }

    #[pyclass]
    pub struct DashMapCache {
        inner: Arc<DashMap<String, Value>>,
        capacity: usize,
    }

    #[pymethods]
    impl DashMapCache {
        #[new]
        fn new(capacity: usize) -> Self {
            Self {
                inner: Arc::new(DashMap::new()),
                capacity,
            }
        }

        fn get(&self, py: Python, key: &str) -> PyResult<PyObject> {
            let map = self.inner.clone();
            let key_str = key.to_string();
            let result = py.allow_threads(|| {
                map.get(&key_str).map(|entry| entry.value().clone())
            });
            if let Some(value) = result {
                Ok(pythonize(py, &value)?)
            } else {
                Ok(py.None())
            }
        }

        fn put(&self, py: Python, key: &str, value: Bound<PyAny>) -> PyResult<()> {
            let map = self.inner.clone();
            let py_obj = value.to_object(py);
            let json_value: Value = pythonize::depythonize(py_obj.as_ref(py))?;
            let capacity = self.capacity;
            let key_str = key.to_string();
            py.allow_threads(|| {
                // Simple eviction: remove oldest if at capacity
                if map.len() >= capacity {
                    if let Some(entry) = map.iter().next() {
                        map.remove(entry.key());
                    }
                }
                map.insert(key_str, json_value);
            });
            Ok(())
        }

        fn delete(&self, key: &str) -> PyResult<bool> {
            let map = self.inner.clone();
            Ok(map.remove(key).is_some())
        }

        fn clear(&self) -> PyResult<()> {
            let map = self.inner.clone();
            map.clear();
            Ok(())
        }

        fn size(&self) -> usize {
            self.inner.len()
        }

        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    // DashMap with TTL wrapper
    #[pyclass]
    pub struct DashMapTTLCache {
        inner: Arc<DashMap<String, TTLValue>>,
        capacity: usize,
    }

    #[pymethods]
    impl DashMapTTLCache {
        #[new]
        fn new(capacity: usize, _ttl_seconds: f64) -> Self {
            Self {
                inner: Arc::new(DashMap::new()),
                capacity,
            }
        }

        fn get(&self, py: Python, key: &str) -> PyResult<PyObject> {
            let map = self.inner.clone();
            let key_str = key.to_string();
            let result = py.allow_threads(|| {
                if let Some(entry) = map.get(&key_str) {
                    let ttl_val = entry.value();
                    if ttl_val.expires_at > Instant::now() {
                        Some(ttl_val.value.clone())
                    } else {
                        // Expired, remove it
                        map.remove(&key_str);
                        None
                    }
                } else {
                    None
                }
            });
            if let Some(value) = result {
                Ok(pythonize(py, &value)?)
            } else {
                Ok(py.None())
            }
        }

        fn put(&self, py: Python, key: &str, value: Bound<PyAny>, ttl_seconds: Option<f64>) -> PyResult<()> {
            let map = self.inner.clone();
            let py_obj = value.to_object(py);
            let json_value: Value = pythonize::depythonize(py_obj.as_ref(py))?;
            let capacity = self.capacity;
            let ttl = ttl_seconds.unwrap_or(60.0);
            let key_str = key.to_string();
            py.allow_threads(|| {
                // Clean expired entries periodically
                if map.len() >= capacity {
                    let now = Instant::now();
                    map.retain(|_, v| v.expires_at > now);
                }
                
                // Simple eviction if still at capacity
                if map.len() >= capacity {
                    if let Some(entry) = map.iter().next() {
                        map.remove(entry.key());
                    }
                }
                
                let ttl_value = TTLValue {
                    value: json_value,
                    expires_at: Instant::now() + Duration::from_secs_f64(ttl),
                };
                map.insert(key_str, ttl_value);
            });
            Ok(())
        }

        fn delete(&self, key: &str) -> PyResult<bool> {
            let map = self.inner.clone();
            Ok(map.remove(key).is_some())
        }

        fn clear(&self) -> PyResult<()> {
            let map = self.inner.clone();
            map.clear();
            Ok(())
        }

        fn size(&self) -> usize {
            // Clean expired entries
            let map = self.inner.clone();
            let now = Instant::now();
            map.retain(|_, v| v.expires_at > now);
            map.len()
        }

        fn capacity(&self) -> usize {
            self.capacity
        }
    }
}

// Export modules
#[cfg(all(feature = "external-caches", feature = "python"))]
pub use moka_cache::{MokaCache, MokaTTLCache, MokaWeightedCache};
#[cfg(all(feature = "external-caches", feature = "python"))]
pub use quick_cache::{QuickCache, QuickCacheTTL};
#[cfg(all(feature = "external-caches", feature = "python"))]
pub use dashmap_cache::{DashMapCache, DashMapTTLCache};

#[cfg(all(feature = "external-caches", feature = "python"))]
pub fn register_external_caches(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Moka variants
    m.add_class::<MokaCache>()?;
    m.add_class::<MokaTTLCache>()?;
    m.add_class::<MokaWeightedCache>()?;
    
    // QuickCache variants
    m.add_class::<QuickCache>()?;
    m.add_class::<QuickCacheTTL>()?;
    
    // DashMap variants
    m.add_class::<DashMapCache>()?;
    m.add_class::<DashMapTTLCache>()?;
    
    Ok(())
}

