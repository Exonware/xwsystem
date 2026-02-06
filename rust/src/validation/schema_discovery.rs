// #exonware/xwsystem/rust/src/validation/schema_discovery.rs
//! Schema validator discovery (plugin entry points).
//! 
//! This module provides a lightweight "plugin" mechanism for discovering
//! implementations of `xwsystem.validation.contracts.ISchemaValidator`.
//! 
//! In Rust, we use a registry pattern instead of Python's entry points.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use super::contracts::ISchemaValidator;

/// Constant: DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP
pub const DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP: &str = "xwsystem.schema_validators";

/// Structured discovery information for observability/debugging.
#[derive(Debug, Clone)]
pub struct SchemaValidatorDiscoveryResult {
    pub group: String,
    pub loaded: HashMap<String, String>,  // name -> provider repr
    pub errors: HashMap<String, String>,  // name -> error string
}

impl SchemaValidatorDiscoveryResult {
    pub fn new(group: String) -> Self {
        Self {
            group,
            loaded: HashMap::new(),
            errors: HashMap::new(),
        }
    }
}

// Global registry for schema validators
static SCHEMA_VALIDATOR_REGISTRY: OnceLock<Mutex<SchemaValidatorRegistry>> = OnceLock::new();

fn get_registry() -> &'static Mutex<SchemaValidatorRegistry> {
    SCHEMA_VALIDATOR_REGISTRY.get_or_init(|| Mutex::new(SchemaValidatorRegistry::new()))
}

struct SchemaValidatorRegistry {
    providers: HashMap<String, HashMap<String, Arc<dyn ISchemaValidator + Send + Sync>>>,
    default_provider: HashMap<String, String>,
    attempted_groups: std::collections::HashSet<String>,
}

impl SchemaValidatorRegistry {
    fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_provider: HashMap::new(),
            attempted_groups: std::collections::HashSet::new(),
        }
    }
}

/// Discover schema validator providers.
pub fn discover_schema_validators(
    group: Option<&str>,
    force: bool,
) -> (HashMap<String, Arc<dyn ISchemaValidator + Send + Sync>>, SchemaValidatorDiscoveryResult) {
    let group_name = group.unwrap_or(DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP);
    let mut registry = get_registry().lock().unwrap();
    
    let already_attempted = registry.attempted_groups.contains(group_name);
    if already_attempted && !force {
        let providers = registry.providers.get(group_name)
            .cloned()
            .unwrap_or_default();
        let loaded_repr: HashMap<String, String> = providers.iter()
            .map(|(k, v)| (k.clone(), format!("{:?}", v)))
            .collect();
        return (
            providers,
            SchemaValidatorDiscoveryResult {
                group: group_name.to_string(),
                loaded: loaded_repr,
                errors: HashMap::new(),
            },
        );
    }

    // In Rust, we can't dynamically discover entry points like Python
    // Providers must be manually registered via set_schema_validator
    let providers = registry.providers.get(group_name)
        .cloned()
        .unwrap_or_default();
    let loaded_repr: HashMap<String, String> = providers.iter()
        .map(|(k, v)| (k.clone(), format!("{:?}", v)))
        .collect();

    registry.attempted_groups.insert(group_name.to_string());
    
    // Preserve existing default selection if still valid; else choose first
    if !registry.default_provider.contains_key(group_name) || 
       !providers.contains_key(registry.default_provider.get(group_name).unwrap()) {
        if let Some(first_key) = providers.keys().min() {
            registry.default_provider.insert(group_name.to_string(), first_key.clone());
        }
    }

    (
        providers,
        SchemaValidatorDiscoveryResult {
            group: group_name.to_string(),
            loaded: loaded_repr,
            errors: HashMap::new(),
        },
    )
}

/// Manually register a schema validator provider (useful for tests / custom wiring).
pub fn set_schema_validator(
    validator: Arc<dyn ISchemaValidator + Send + Sync>,
    name: Option<&str>,
    group: Option<&str>,
    make_default: bool,
) {
    let name = name.unwrap_or("manual");
    let group_name = group.unwrap_or(DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP);
    
    let mut registry = get_registry().lock().unwrap();
    registry.attempted_groups.insert(group_name.to_string());
    registry.providers
        .entry(group_name.to_string())
        .or_insert_with(HashMap::new)
        .insert(name.to_string(), validator);
    
    if make_default {
        registry.default_provider.insert(group_name.to_string(), name.to_string());
    }
}

/// Get a schema validator provider.
pub fn get_schema_validator(
    name: Option<&str>,
    group: Option<&str>,
) -> Option<Arc<dyn ISchemaValidator + Send + Sync>> {
    let group_name = group.unwrap_or(DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP);
    let (providers, _) = discover_schema_validators(Some(group_name), false);
    
    if providers.is_empty() {
        return None;
    }

    if let Some(n) = name {
        return providers.get(n).cloned();
    }

    // Get default provider
    let registry = get_registry().lock().unwrap();
    if let Some(default_name) = registry.default_provider.get(group_name) {
        if let Some(provider) = providers.get(default_name) {
            return Some(provider.clone());
        }
    }

    // Fall back to deterministic choice
    if let Some(first_key) = providers.keys().min() {
        return providers.get(first_key).cloned();
    }

    None
}

/// List discovered provider names.
pub fn available_schema_validators(group: Option<&str>) -> Vec<String> {
    let group_name = group.unwrap_or(DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP);
    let (providers, _) = discover_schema_validators(Some(group_name), false);
    let mut names: Vec<String> = providers.keys().cloned().collect();
    names.sort();
    names
}
