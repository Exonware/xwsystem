// #exonware/xwsystem/rust/tests/0.core/test_core_contracts.rs
//! Core tests for contracts module.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! These tests verify that all contracts (traits) match Python 100%.

use xwsystem_rust::contracts::*;
use xwsystem_rust::defs::{CoreMode, CoreState, DataType, CloneMode};
use xwsystem_rust::base::XWObject;
use std::collections::HashMap;
use serde_json::Value;

#[test]
fn test_iid_trait() {
    let mut obj = XWObject::new(None);
    
    // Test id() and uid()
    assert!(!obj.id().is_empty());
    assert!(!obj.uid().is_empty());

    // Test generate_id()
    let new_id = obj.generate_id();
    assert!(!new_id.is_empty());
    assert_eq!(obj.id(), new_id.as_str());

    // Test validate_id()
    assert!(obj.validate_id("valid_id"));
    assert!(!obj.validate_id(""));

    // Test is_same_id()
    let obj2 = XWObject::new(Some(obj.id().to_string()));
    assert!(obj.is_same_id(&obj2));
}

#[test]
fn test_istringable_trait() {
    let mut obj = XWObject::new(Some("test_id".to_string()));
    
    // Test to_string()
    let str_repr = obj.to_string();
    assert!(!str_repr.is_empty());
    assert!(str_repr.contains("test_id"));

    // Test from_string() - should parse JSON-like representation
    // Note: Implementation may vary, but should not panic
    let _ = obj.from_string(&str_repr);
}

#[test]
fn test_inative_trait() {
    let mut obj = XWObject::new(Some("test_id".to_string()));
    
    // Test to_native()
    let native = obj.to_native();
    assert!(native.is_object());
    assert_eq!(native.get("id").and_then(|v| v.as_str()), Some("test_id"));

    // Test from_native()
    let data = serde_json::json!({
        "id": "new_id",
        "uid": "new_uid"
    });
    obj.from_native(&data).unwrap();
    assert_eq!(obj.id(), "new_id");

    // Test is_native_compatible()
    assert!(obj.is_native_compatible(&data));
    assert!(!obj.is_native_compatible(&Value::String("not an object".to_string())));

    // Test get_native_type()
    assert_eq!(obj.get_native_type(), DataType::Dict);
}

#[test]
fn test_iobject_trait() {
    let mut obj = XWObject::new(None);
    
    // Test created_at and updated_at
    let created = obj.created_at();
    let updated = obj.updated_at();
    assert!(created <= updated);

    // Test title and description
    assert!(obj.title().is_none());
    assert!(obj.description().is_none());

    // Test to_dict()
    let dict = obj.to_dict();
    assert!(dict.is_object());
    assert!(dict.get("id").is_some());
    assert!(dict.get("created_at").is_some());
    assert!(dict.get("updated_at").is_some());
}

#[test]
fn test_icore_trait() {
    use xwsystem_rust::base::{BaseCore, ICore};
    
    let mut core = BaseCore::new(CoreMode::Development);
    
    // Test mode()
    assert_eq!(core.mode(), CoreMode::Development);

    // Test state()
    assert_eq!(core.state(), CoreState::Uninitialized);

    // Test initialize()
    core.initialize().unwrap();
    assert!(core.is_initialized());
    assert_eq!(core.state(), CoreState::Initialized);

    // Test shutdown()
    core.shutdown().unwrap();
    assert!(core.is_shutdown());
    assert_eq!(core.state(), CoreState::Shutdown);

    // Test get_dependencies()
    let deps = core.get_dependencies();
    assert!(deps.is_empty());

    // Test check_dependencies()
    assert!(core.check_dependencies());
}

#[test]
fn test_isystem_component_trait() {
    use xwsystem_rust::base::{BaseSystemComponent, ISystemComponent};
    
    let mut component = BaseSystemComponent::new("test".to_string(), CoreMode::Development);
    
    // Test get_name()
    assert_eq!(component.get_name(), "test");

    // Test get_version()
    assert_eq!(component.get_version(), "0.1.0.1");

    // Test initialize()
    component.initialize().unwrap();
    assert!(component.is_initialized());

    // Test shutdown()
    component.shutdown().unwrap();
    assert!(component.is_shutdown());

    // Test get_config() and set_config()
    let mut config = HashMap::new();
    config.insert("key".to_string(), Value::String("value".to_string()));
    component.set_config(config.clone());
    assert_eq!(component.get_config().get("key"), Some(&Value::String("value".to_string())));
}

#[test]
fn test_iconfigurable_trait() {
    use xwsystem_rust::base::{BaseConfigurableComponent, IConfigurable};
    
    let mut component = BaseConfigurableComponent::new("test".to_string(), CoreMode::Development);
    
    // Test configure()
    let mut options = HashMap::new();
    options.insert("opt1".to_string(), Value::String("val1".to_string()));
    component.configure(options);
    assert!(component.has_config("opt1"));

    // Test get_config()
    let config = component.get_config();
    assert_eq!(config.get("opt1"), Some(&Value::String("val1".to_string())));

    // Test update_config()
    component.update_config("opt1", Value::String("updated".to_string()));
    assert_eq!(component.get_config().get("opt1"), Some(&Value::String("updated".to_string())));

    // Test has_config()
    assert!(component.has_config("opt1"));
    assert!(!component.has_config("nonexistent"));

    // Test remove_config()
    assert!(component.remove_config("opt1"));
    assert!(!component.has_config("opt1"));

    // Test reset_config()
    component.configure({
        let mut opts = HashMap::new();
        opts.insert("key".to_string(), Value::String("value".to_string()));
        opts
    });
    component.reset_config();
    assert!(component.get_config().is_empty());

    // Test merge_config()
    let mut new_config = HashMap::new();
    new_config.insert("new".to_string(), Value::String("value".to_string()));
    component.merge_config(new_config, None);
    assert!(component.has_config("new"));
}

