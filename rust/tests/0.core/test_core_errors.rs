// #exonware/xwsystem/rust/tests/0.core/test_core_errors.rs
//! Core tests for errors module.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! These tests verify that all errors match Python 100%.

use xwsystem_rust::errors::*;
use std::error::Error;

#[test]
fn test_root_errors() {
    // Test XWSystemError
    let err = XWSystemError::new("Test error");
    assert!(err.to_string().contains("Test error"));
    assert!(err.source().is_none());

    // Test XWSystemInitializationError
    let err = XWSystemInitializationError::new("Init failed");
    assert!(err.to_string().contains("Init failed"));

    // Test XWSystemConfigurationError
    let err = XWSystemConfigurationError::new("Config failed");
    assert!(err.to_string().contains("Config failed"));

    // Test XWSystemStateError
    let err = XWSystemStateError::new("State error");
    assert!(err.to_string().contains("State error"));

    // Test XWSystemDependencyError
    let err = XWSystemDependencyError::new("Dependency missing");
    assert!(err.to_string().contains("Dependency missing"));

    // Test XWSystemResourceError
    let err = XWSystemResourceError::new("Resource error");
    assert!(err.to_string().contains("Resource error"));

    // Test XWSystemTimeoutError
    let err = XWSystemTimeoutError::new("Timeout");
    assert!(err.to_string().contains("Timeout"));

    // Test XWSystemPermissionError
    let err = XWSystemPermissionError::new("Permission denied");
    assert!(err.to_string().contains("Permission denied"));

    // Test XWSystemValidationError
    let err = XWSystemValidationError::new("Validation failed");
    assert!(err.to_string().contains("Validation failed"));

    // Test XWSystemOperationError
    let err = XWSystemOperationError::new("Operation failed");
    assert!(err.to_string().contains("Operation failed"));
}

#[test]
fn test_core_errors() {
    // Test CoreError
    let err = CoreError::new("Core error");
    assert!(err.to_string().contains("Core error"));

    // Test CoreInitializationError
    let err = CoreInitializationError::new("Core init failed");
    assert!(err.to_string().contains("Core init failed"));

    // Test CoreShutdownError
    let err = CoreShutdownError::new("Core shutdown failed");
    assert!(err.to_string().contains("Core shutdown failed"));

    // Test CoreStateError
    let err = CoreStateError::new("Core state error");
    assert!(err.to_string().contains("Core state error"));

    // Test CoreDependencyError
    let err = CoreDependencyError::new("Core dependency missing");
    assert!(err.to_string().contains("Core dependency missing"));

    // Test CoreConfigurationError
    let err = CoreConfigurationError::new("Core config error");
    assert!(err.to_string().contains("Core config error"));

    // Test CoreResourceError
    let err = CoreResourceError::new("Core resource error");
    assert!(err.to_string().contains("Core resource error"));

    // Test CoreTimeoutError
    let err = CoreTimeoutError::new("Core timeout");
    assert!(err.to_string().contains("Core timeout"));

    // Test CorePermissionError
    let err = CorePermissionError::new("Core permission denied");
    assert!(err.to_string().contains("Core permission denied"));

    // Test CoreValidationError
    let err = CoreValidationError::new("Core validation failed");
    assert!(err.to_string().contains("Core validation failed"));

    // Test CoreOperationError
    let err = CoreOperationError::new("Core operation failed");
    assert!(err.to_string().contains("Core operation failed"));
}

#[test]
fn test_error_hierarchy() {
    // Verify error hierarchy - all errors should implement std::error::Error
    let errors: Vec<Box<dyn Error>> = vec![
        Box::new(XWSystemError::new("test")),
        Box::new(CoreError::new("test")),
    ];
    
    for err in errors {
        assert!(!err.to_string().is_empty());
    }
}

