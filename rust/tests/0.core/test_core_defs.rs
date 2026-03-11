// #exonware/xwsystem/rust/tests/0.core/test_core_defs.rs
//! Core tests for defs module.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! These tests verify that all defs match Python 100%.

use xwsystem_rust::defs::*;

#[test]
fn test_root_level_enums() {
    // Test SystemStatus enum
    assert_eq!(SystemStatus::Unknown.to_string(), "unknown");
    assert_eq!(SystemStatus::Initializing.to_string(), "initializing");
    assert_eq!(SystemStatus::Ready.to_string(), "ready");
    assert_eq!(SystemStatus::Running.to_string(), "running");
    assert_eq!(SystemStatus::Paused.to_string(), "paused");
    assert_eq!(SystemStatus::Stopping.to_string(), "stopping");
    assert_eq!(SystemStatus::Stopped.to_string(), "stopped");
    assert_eq!(SystemStatus::Error.to_string(), "error");
    assert_eq!(SystemStatus::Maintenance.to_string(), "maintenance");

    // Test ComponentType enum
    assert_eq!(ComponentType::Core.to_string(), "core");
    assert_eq!(ComponentType::Io.to_string(), "io");
    assert_eq!(ComponentType::Security.to_string(), "security");
    assert_eq!(ComponentType::Monitoring.to_string(), "monitoring");

    // Test ErrorSeverity enum
    assert_eq!(ErrorSeverity::Low.to_string(), "low");
    assert_eq!(ErrorSeverity::Medium.to_string(), "medium");
    assert_eq!(ErrorSeverity::High.to_string(), "high");
    assert_eq!(ErrorSeverity::Critical.to_string(), "critical");

    // Test LogCategory enum
    assert_eq!(LogCategory::System.to_string(), "system");
    assert_eq!(LogCategory::Security.to_string(), "security");
    assert_eq!(LogCategory::Performance.to_string(), "performance");
}

#[test]
fn test_shared_enums() {
    // Test ValidationLevel
    assert_eq!(ValidationLevel::None.to_string(), "none");
    assert_eq!(ValidationLevel::Basic.to_string(), "basic");
    assert_eq!(ValidationLevel::Strict.to_string(), "strict");
    assert_eq!(ValidationLevel::Paranoid.to_string(), "paranoid");
    assert_eq!(ValidationLevel::Comprehensive.to_string(), "comprehensive");
    assert_eq!(ValidationLevel::default(), ValidationLevel::Basic);

    // Test PerformanceLevel
    assert_eq!(PerformanceLevel::Low.to_string(), "low");
    assert_eq!(PerformanceLevel::Medium.to_string(), "medium");
    assert_eq!(PerformanceLevel::High.to_string(), "high");
    assert_eq!(PerformanceLevel::Critical.to_string(), "critical");
    assert_eq!(PerformanceLevel::default(), PerformanceLevel::Medium);

    // Test AuthType
    assert_eq!(AuthType::None.to_string(), "none");
    assert_eq!(AuthType::Basic.to_string(), "basic");
    assert_eq!(AuthType::Bearer.to_string(), "bearer");
    assert_eq!(AuthType::ApiKey.to_string(), "apikey");
    assert_eq!(AuthType::Oauth2.to_string(), "oauth2");
    assert_eq!(AuthType::Jwt.to_string(), "jwt");
    assert_eq!(AuthType::Saml.to_string(), "saml");
    assert_eq!(AuthType::default(), AuthType::None);
}

#[test]
fn test_constants() {
    // Version constants
    assert_eq!(DEFS_VERSION_MAJOR, 0);
    assert_eq!(DEFS_VERSION_MINOR, 1);
    assert_eq!(DEFS_VERSION_PATCH, 0);
    assert_eq!(DEFS_VERSION_BUILD, 1);
    assert_eq!(DEFS_VERSION_STRING, "0.1.0.1");

    // System constants
    assert_eq!(DEFAULT_TIMEOUT, 30);
    assert_eq!(DEFAULT_RETRY_COUNT, 3);
    assert_eq!(DEFAULT_RETRY_DELAY, 1.0);
    assert_eq!(MAX_RETRY_DELAY, 60.0);

    // Configuration constants
    assert_eq!(DEFAULT_CONFIG_FILE, "xwsystem.json");
    assert_eq!(DEFAULT_CONFIG_DIR, ".xwsystem");
    assert_eq!(DEFAULT_LOG_DIR, "logs");
    assert_eq!(DEFAULT_CACHE_DIR, ".cache");

    // Security constants
    assert_eq!(MIN_PASSWORD_LENGTH, 8);
    assert_eq!(MAX_PASSWORD_LENGTH, 128);
    assert_eq!(DEFAULT_HASH_ALGORITHM, "sha256");
    assert_eq!(DEFAULT_ENCRYPTION_ALGORITHM, "AES-256-GCM");

    // Performance constants
    assert_eq!(DEFAULT_CACHE_SIZE, 1000);
    assert_eq!(DEFAULT_CACHE_TTL, 3600);
    assert_eq!(DEFAULT_THREAD_POOL_SIZE, 10);
    assert_eq!(DEFAULT_MAX_WORKERS, 4);

    // Validation constants
    assert_eq!(MAX_DEPTH_DEFAULT, 100);
    assert_eq!(MAX_SIZE_DEFAULT, 10 * 1024 * 1024);
    assert_eq!(MAX_ITEMS_DEFAULT, 10000);
}

#[test]
fn test_type_aliases() {
    // Type aliases should compile and be usable
    let _config_dict: ConfigDict = serde_json::json!({});
    let _config_list: ConfigList = serde_json::json!([]);
    let _error_dict: ErrorDict = serde_json::json!({});
    let _metrics_dict: MetricsDict = serde_json::json!({});
    let _metadata_dict: MetadataDict = serde_json::json!({});
}

