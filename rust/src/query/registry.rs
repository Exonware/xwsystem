// #exonware/xwsystem/rust/src/query/registry.rs
//! #exonware/xwsystem/src/exonware/xwsystem/query/registry.py
//! 
//! Global query provider registry (foundation layer).
//! 
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 28-Dec-2025

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use super::contracts::IQueryProvider;
use super::errors::QueryProviderNotRegisteredError;

/// Thread-safe registry for query providers.
///
/// Design notes:
/// - Multiple providers may be registered (future-proofing).
/// - A default provider may be selected; consumers typically use the default.
pub struct QueryProviderRegistry {
    _lock: Mutex<()>,
    providers: Mutex<HashMap<String, Arc<dyn IQueryProvider>>>,
    default_provider_id: Mutex<Option<String>>,
}

impl QueryProviderRegistry {
    /// Constructor
    pub fn new() -> Self {
        Self {
            _lock: Mutex::new(()),
            providers: Mutex::new(HashMap::new()),
            default_provider_id: Mutex::new(None),
        }
    }

    pub fn register(&self, provider: Arc<dyn IQueryProvider>, overwrite: bool, make_default: bool) -> Result<(), String> {
        let provider_id = provider.provider_id().trim().to_lowercase();
        if provider_id.is_empty() {
            return Err("provider.provider_id must be a non-empty string".to_string());
        }

        let _guard = self._lock.lock().unwrap();
        let mut providers = self.providers.lock().unwrap();
        let mut default_id = self.default_provider_id.lock().unwrap();

        if providers.contains_key(&provider_id) && !overwrite {
            return Ok(());
        }

        providers.insert(provider_id.clone(), provider);
        if make_default || default_id.is_none() {
            *default_id = Some(provider_id);
        }

        Ok(())
    }

    pub fn unregister(&self, provider_id: &str) -> bool {
        let pid = provider_id.trim().to_lowercase();
        let _guard = self._lock.lock().unwrap();
        let mut providers = self.providers.lock().unwrap();
        let mut default_id = self.default_provider_id.lock().unwrap();

        if !providers.contains_key(&pid) {
            return false;
        }

        providers.remove(&pid);
        if *default_id == Some(pid.clone()) {
            *default_id = providers.keys().next().cloned();
        }
        true
    }

    pub fn get(&self, provider_id: &str) -> Option<Arc<dyn IQueryProvider>> {
        let pid = provider_id.trim().to_lowercase();
        let _guard = self._lock.lock().unwrap();
        let providers = self.providers.lock().unwrap();
        providers.get(&pid).cloned()
    }

    pub fn get_default(&self) -> Option<Arc<dyn IQueryProvider>> {
        let _guard = self._lock.lock().unwrap();
        let providers = self.providers.lock().unwrap();
        let default_id = self.default_provider_id.lock().unwrap();
        
        if let Some(ref id) = *default_id {
            providers.get(id).cloned()
        } else {
            None
        }
    }

    pub fn require_default(&self) -> Result<Arc<dyn IQueryProvider>, QueryProviderNotRegisteredError> {
        self.get_default().ok_or_else(|| {
            QueryProviderNotRegisteredError::new(
                "No query provider registered. Install and import exonware-xwquery to register a provider."
            )
        })
    }

    pub fn list_provider_ids(&self) -> Vec<String> {
        let _guard = self._lock.lock().unwrap();
        let providers = self.providers.lock().unwrap();
        providers.keys().cloned().collect()
    }

    pub fn clear(&self) {
        let _guard = self._lock.lock().unwrap();
        let mut providers = self.providers.lock().unwrap();
        let mut default_id = self.default_provider_id.lock().unwrap();
        providers.clear();
        *default_id = None;
    }
}

static GLOBAL_REGISTRY: OnceLock<Arc<QueryProviderRegistry>> = OnceLock::new();

/// Get the global query provider registry (thread-safe singleton).
pub fn get_query_provider_registry() -> Arc<QueryProviderRegistry> {
    GLOBAL_REGISTRY.get_or_init(|| Arc::new(QueryProviderRegistry::new())).clone()
}

/// Reset the global registry (primarily for testing).
pub fn reset_query_provider_registry() {
    // Note: OnceLock doesn't support reset, so this is a no-op in Rust
    // For testing, you'd need to use a different approach like a Mutex<Option<Arc<...>>>
}

/// Register a query provider in the global registry.
pub fn register_query_provider(
    provider: Arc<dyn IQueryProvider>,
    overwrite: bool,
    make_default: bool,
) -> Result<(), String> {
    get_query_provider_registry().register(provider, overwrite, make_default)
}

/// Get a provider by id, or the default provider if provider_id is None.
pub fn get_query_provider(provider_id: Option<&str>) -> Option<Arc<dyn IQueryProvider>> {
    let registry = get_query_provider_registry();
    if let Some(id) = provider_id {
        registry.get(id)
    } else {
        registry.get_default()
    }
}
