// #exonware/xwsystem/rust/tests/0.core/test_core_base.rs
//! Core tests for base module.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! These tests verify that all base classes match Python 100%.

use xwsystem_rust::base::*;
use xwsystem_rust::defs::{CoreMode, CoreState};
use xwsystem_rust::contracts::{ISystemComponent, IConfigurable};
use std::collections::HashMap;
use serde_json::Value;

#[test]
fn test_axwsystem_base() {
    let mut base = BaseXWSystem::new(CoreMode::Development);
    
    // Test initial state
    assert_eq!(base.mode(), CoreMode::Development);
    assert_eq!(base.state(), CoreState::Uninitialized);
    assert!(!base.is_initialized());
    assert!(!base.is_shutdown());

    // Test initialization
    base.initialize().unwrap();
    assert!(base.is_initialized());
    assert_eq!(base.state(), CoreState::Initialized);

    // Test shutdown
    base.shutdown().unwrap();
    assert!(base.is_shutdown());
    assert_eq!(base.state(), CoreState::Shutdown);
}

#[test]
fn test_asystem_component() {
    let mut component = BaseSystemComponent::new("test_component".to_string(), CoreMode::Development);
    
    // Test properties
    assert_eq!(component.get_name(), "test_component");
    assert_eq!(component.get_version(), "0.1.0.1");
    assert!(component.get_config().is_empty());

    // Test configuration
    let mut config = HashMap::new();
    config.insert("key1".to_string(), Value::String("value1".to_string()));
    component.set_config(config.clone());
    let retrieved = component.get_config();
    assert_eq!(retrieved.get("key1"), Some(&Value::String("value1".to_string())));

    // Test initialization
    component.initialize().unwrap();
    assert!(component.is_initialized());

    // Test shutdown
    component.shutdown().unwrap();
    assert!(component.is_shutdown());
}

#[test]
fn test_aconfigurable_component() {
    let mut component = BaseConfigurableComponent::new("config_component".to_string(), CoreMode::Development);
    
    // Test initial state
    assert_eq!(component.get_name(), "config_component");
    assert!(component.get_config().is_empty());

    // Test configure
    let mut options = HashMap::new();
    options.insert("option1".to_string(), Value::String("value1".to_string()));
    options.insert("option2".to_string(), Value::Number(42.into()));
    component.configure(options);
    
    let config = component.get_config();
    assert_eq!(config.get("option1"), Some(&Value::String("value1".to_string())));
    assert_eq!(config.get("option2"), Some(&Value::Number(42.into())));

    // Test update_config
    component.update_config("option1", Value::String("updated".to_string()));
    let config = component.get_config();
    assert_eq!(config.get("option1"), Some(&Value::String("updated".to_string())));

    // Test has_config
    assert!(component.has_config("option1"));
    assert!(!component.has_config("nonexistent"));

    // Test remove_config
    assert!(component.remove_config("option1"));
    assert!(!component.has_config("option1"));

    // Test reset_config
    component.reset_config();
    assert!(component.get_config().is_empty());

    // Test merge_config
    let mut new_config = HashMap::new();
    new_config.insert("new_key".to_string(), Value::String("new_value".to_string()));
    component.merge_config(new_config, None);
    assert!(component.has_config("new_key"));
}

#[test]
fn test_base_core() {
    use xwsystem_rust::base::ACoreBase;
    
    let mut core = BaseCore::new(CoreMode::Development);
    
    // Test initial state
    assert_eq!(ACoreBase::get_state(&core), CoreState::Uninitialized);
    assert!(!ACoreBase::is_initialized(&core));
    assert!(!ACoreBase::is_shutdown(&core));

    // Test initialization
    ACoreBase::initialize(&mut core).unwrap();
    assert!(ACoreBase::is_initialized(&core));
    assert_eq!(ACoreBase::get_state(&core), CoreState::Initialized);

    // Test dependencies
    ACoreBase::add_dependency(&mut core, "dep1".to_string());
    ACoreBase::add_dependency(&mut core, "dep2".to_string());
    let deps = ACoreBase::get_dependencies(&core);
    assert_eq!(deps.len(), 2);
    assert!(deps.contains(&"dep1".to_string()));
    assert!(deps.contains(&"dep2".to_string()));

    ACoreBase::remove_dependency(&mut core, "dep1");
    let deps = ACoreBase::get_dependencies(&core);
    assert_eq!(deps.len(), 1);
    assert!(!deps.contains(&"dep1".to_string()));

    // Test shutdown
    ACoreBase::shutdown(&mut core).unwrap();
    assert!(ACoreBase::is_shutdown(&core));
}

#[test]
fn test_xw_object() {
    let mut obj = XWObject::new(None);
    
    // Test ID generation
    assert!(!obj.id().is_empty());
    assert!(!obj.uid().is_empty());
    assert_ne!(obj.id(), obj.uid());

    // Test with custom ID
    let obj2 = XWObject::new(Some("custom_id".to_string()));
    assert_eq!(obj2.id(), "custom_id");

    // Test title and description
    obj.set_title(Some("Test Title".to_string()));
    obj.set_description(Some("Test Description".to_string()));
    assert_eq!(obj.title(), Some("Test Title"));
    assert_eq!(obj.description(), Some("Test Description"));

    // Test to_dict
    let dict = obj.to_dict();
    assert!(dict.get("id").is_some());
    assert!(dict.get("uid").is_some());
    assert_eq!(dict.get("title").and_then(|v| v.as_str()), Some("Test Title"));
    assert_eq!(dict.get("description").and_then(|v| v.as_str()), Some("Test Description"));

    // Test from_native
    let data = serde_json::json!({
        "id": "new_id",
        "uid": "new_uid",
        "title": "New Title"
    });
    obj.from_native(&data).unwrap();
    assert_eq!(obj.id(), "new_id");
    assert_eq!(obj.title(), Some("New Title"));
}

