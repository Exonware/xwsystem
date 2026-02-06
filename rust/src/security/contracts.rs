// #exonware/xwsystem/rust/src/security/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Security protocol interfaces for XWSystem.


use std::collections::HashMap;

use crate::defs::{AuditEvent, AuthenticationMethod, AuthorizationLevel, EncryptionAlgorithm, HashAlgorithm, SecurityLevel};

// ============================================================================

// AUTHENTICATION INTERFACES

// ============================================================================

// ============================================================================

// AUDIT INTERFACES

// ============================================================================

// ============================================================================

// AUTHORIZATION INTERFACES

// ============================================================================

// ============================================================================

// SECURITY VALIDATION INTERFACES

// ============================================================================

// ============================================================================

// SECURITY MONITORING INTERFACES

// ============================================================================

// ============================================================================

// SECURITY POLICY INTERFACES

// ============================================================================

// ============================================================================

// SECURITY TOKEN INTERFACES

// ============================================================================

// ============================================================================

// SECURITY PROTOCOLS

// ============================================================================

/// Interface for secure objects.
///
/// Enforces consistent security behavior across XWSystem.
pub trait ISecure {
    /// Encrypt data.
    /// Args:
    /// data: Data to encrypt
    /// algorithm: Encryption algorithm
    /// Returns:
    /// Encrypted data
    fn encrypt(&self, data: String, algorithm: EncryptionAlgorithm) -> String;

    /// Decrypt data.
    /// Args:
    /// encrypted_data: Encrypted data
    /// algorithm: Encryption algorithm
    /// Returns:
    /// Decrypted data
    fn decrypt(&self, encrypted_data: String, algorithm: EncryptionAlgorithm) -> String;

    /// Hash data.
    /// Args:
    /// data: Data to hash
    /// algorithm: Hash algorithm
    /// Returns:
    /// Hash string
    fn hash(&self, data: String, algorithm: HashAlgorithm) -> String;

    /// Verify data against hash.
    /// Args:
    /// data: Data to verify
    /// hash_value: Hash to verify against
    /// algorithm: Hash algorithm
    /// Returns:
    /// True if hash matches
    fn verify_hash(&self, data: String, hash_value: String, algorithm: HashAlgorithm) -> bool;

    /// Generate encryption key.
    /// Args:
    /// algorithm: Encryption algorithm
    /// Returns:
    /// Generated key
    fn generate_key(&self, algorithm: EncryptionAlgorithm) -> Vec<u8>;

    /// Generate random salt.
    /// Args:
    /// length: Salt length in bytes
    /// Returns:
    /// Generated salt
    fn generate_salt(&self, length: i64) -> Vec<u8>;

    /// Generate secure random bytes.
    /// Args:
    /// length: Number of bytes to generate
    /// Returns:
    /// Random bytes
    fn secure_random(&self, length: i64) -> Vec<u8>;

}

/// Interface for authentication.
///
/// Enforces consistent authentication behavior across XWSystem.
pub trait IAuthenticatable {
    /// Authenticate user with credentials.
    /// Args:
    /// credentials: Authentication credentials
    /// Returns:
    /// True if authenticated
    fn authenticate(&self, credentials: HashMap<String, serde_json::Value>) -> bool;

    /// Authorize user for resource action.
    /// Args:
    /// user: User identifier
    /// resource: Resource identifier
    /// action: Action to authorize
    /// Returns:
    /// True if authorized
    fn authorize(&self, user: String, resource: String, action: String) -> bool;

    /// Logout user.
    /// Args:
    /// user: User identifier
    /// Returns:
    /// True if logged out
    fn logout(&self, user: String) -> bool;

    /// Check if user is authenticated.
    /// Args:
    /// user: User identifier
    /// Returns:
    /// True if authenticated
    fn is_authenticated(&self, user: String) -> bool;

    /// Get user permissions.
    /// Args:
    /// user: User identifier
    /// Returns:
    /// List of permissions
    fn get_user_permissions(&self, user: String) -> Vec<String>;

    /// Set user permissions.
    /// Args:
    /// user: User identifier
    /// permissions: List of permissions
    fn set_user_permissions(&self, user: String, permissions: Vec<String>) -> ();

    /// Validate credential format.
    /// Args:
    /// credentials: Credentials to validate
    /// Returns:
    /// True if valid format
    fn validate_credentials(&self, credentials: HashMap<String, serde_json::Value>) -> bool;

    /// Get authentication method.
    /// Returns:
    /// Authentication method
    fn get_authentication_method(&self) -> AuthenticationMethod;

}

/// Interface for audit trails.
///
/// Enforces consistent audit behavior across XWSystem.
pub trait IAuditable {
    /// Log audit action.
    /// Args:
    /// action: Audit event type
    /// user: User identifier
    /// resource: Resource identifier
    /// details: Additional details
    fn log_action(&self, action: AuditEvent, user: String, resource: String, details: HashMap<String, serde_json::Value>) -> ();

    /// Get audit trail.
    /// Args:
    /// user: Filter by user
    /// resource: Filter by resource
    /// start_time: Filter by start time
    /// end_time: Filter by end time
    /// Returns:
    /// List of audit entries
    fn get_audit_trail(&self, user: Option<String>, resource: Option<String>, start_time: Option<f64>, end_time: Option<f64>) -> Vec<HashMap<String, serde_json::Value>>;

    /// Clear audit trail.
    /// Args:
    /// older_than: Clear entries older than timestamp
    /// Returns:
    /// Number of entries cleared
    fn clear_audit_trail(&self, older_than: Option<f64>) -> i64;

    /// Export audit trail to file.
    /// Args:
    /// file_path: Export file path
    /// format: Export format
    /// Returns:
    /// True if exported successfully
    fn export_audit_trail(&self, file_path: String, format: String) -> bool;

    /// Get audit statistics.
    /// Returns:
    /// Audit statistics dictionary
    fn get_audit_stats(&self) -> HashMap<String, serde_json::Value>;

    /// Check if auditing is enabled.
    /// Returns:
    /// True if enabled
    fn is_audit_enabled(&self) -> bool;

    /// Enable auditing.
    fn enable_audit(&self) -> ();

    /// Disable auditing.
    fn disable_audit(&self) -> ();

}

/// Interface for authorization.
///
/// Enforces consistent authorization behavior across XWSystem.
pub trait IAuthorization {
    /// Check user permission for resource action.
    /// Args:
    /// user: User identifier
    /// resource: Resource identifier
    /// action: Action to check
    /// Returns:
    /// True if permitted
    fn check_permission(&self, user: String, resource: String, action: String) -> bool;

    /// Grant permission to user.
    /// Args:
    /// user: User identifier
    /// resource: Resource identifier
    /// action: Action to grant
    /// Returns:
    /// True if granted
    fn grant_permission(&self, user: String, resource: String, action: String) -> bool;

    /// Revoke permission from user.
    /// Args:
    /// user: User identifier
    /// resource: Resource identifier
    /// action: Action to revoke
    /// Returns:
    /// True if revoked
    fn revoke_permission(&self, user: String, resource: String, action: String) -> bool;

    /// Get user roles.
    /// Args:
    /// user: User identifier
    /// Returns:
    /// List of role names
    fn get_user_roles(&self, user: String) -> Vec<String>;

    /// Assign role to user.
    /// Args:
    /// user: User identifier
    /// role: Role name
    /// Returns:
    /// True if assigned
    fn assign_role(&self, user: String, role: String) -> bool;

    /// Remove role from user.
    /// Args:
    /// user: User identifier
    /// role: Role name
    /// Returns:
    /// True if removed
    fn remove_role(&self, user: String, role: String) -> bool;

    /// Get role permissions.
    /// Args:
    /// role: Role name
    /// Returns:
    /// List of permissions
    fn get_role_permissions(&self, role: String) -> Vec<String>;

    /// Set role permissions.
    /// Args:
    /// role: Role name
    /// permissions: List of permissions
    fn set_role_permissions(&self, role: String, permissions: Vec<String>) -> ();

}

/// Interface for security validation.
///
/// Enforces consistent security validation across XWSystem.
pub trait ISecurityValidator {
    /// Validate password strength.
    /// Args:
    /// ...word: Password to validate
    /// Returns:
    /// Tuple of (is_valid, error_messages)
    fn validate_password(&self, password: String) -> (bool, Vec<String>);

    /// Validate input data.
    /// Args:
    /// input_data: Input data to validate
    /// input_type: Type of input (email, url, etc.)
    /// Returns:
    /// Tuple of (is_valid, error_messages)
    fn validate_input(&self, input_data: String, input_type: String) -> (bool, Vec<String>);

    /// Sanitize input data.
    /// Args:
    /// input_data: Input data to sanitize
    /// Returns:
    /// Sanitized data
    fn sanitize_input(&self, input_data: String) -> String;

    /// Detect SQL injection attempts.
    /// Args:
    /// input_data: Input data to check
    /// Returns:
    /// True if SQL injection detected
    fn detect_sql_injection(&self, input_data: String) -> bool;

    /// Detect XSS attempts.
    /// Args:
    /// input_data: Input data to check
    /// Returns:
    /// True if XSS detected
    fn detect_xss(&self, input_data: String) -> bool;

    /// Validate certificate.
    /// Args:
    /// certificate: Certificate data
    /// Returns:
    /// Tuple of (is_valid, error_message)
    fn validate_certificate(&self, certificate: Vec<u8>) -> (bool, String);

    /// Check security headers.
    /// Args:
    /// headers: HTTP headers
    /// Returns:
    /// Dictionary of header validation results
    fn check_security_headers(&self, headers: HashMap<String, String>) -> HashMap<String, bool>;

}

/// Interface for security monitoring.
///
/// Enforces consistent security monitoring across XWSystem.
pub trait ISecurityMonitor {
    /// Detect intrusion attempts.
    /// Args:
    /// event_data: Event data to analyze
    /// Returns:
    /// True if intrusion detected
    fn detect_intrusion(&self, event_data: HashMap<String, serde_json::Value>) -> bool;

    /// Monitor failed login attempts.
    /// Args:
    /// user: User identifier
    /// max_attempts: Maximum allowed attempts
    /// Returns:
    /// True if threshold exceeded
    fn monitor_failed_logins(&self, user: String, max_attempts: i64) -> bool;

    /// Detect anomalous behavior.
    /// Args:
    /// behavior_data: Behavior data to analyze
    /// Returns:
    /// True if anomaly detected
    fn detect_anomaly(&self, behavior_data: HashMap<String, serde_json::Value>) -> bool;

    /// Get security alerts.
    /// Returns:
    /// List of security alerts
    fn get_security_alerts(&self) -> Vec<HashMap<String, serde_json::Value>>;

    /// Clear security alerts.
    fn clear_security_alerts(&self) -> ();

    /// Get current threat level.
    /// Returns:
    /// Current threat level
    fn get_threat_level(&self) -> SecurityLevel;

    /// Set threat level.
    /// Args:
    /// level: Threat level to set
    fn set_threat_level(&self, level: SecurityLevel) -> ();

    /// Get security metrics.
    /// Returns:
    /// Security metrics dictionary
    fn get_security_metrics(&self) -> HashMap<String, serde_json::Value>;

}

/// Interface for security policies.
///
/// Enforces consistent security policy behavior across XWSystem.
pub trait ISecurityPolicy {
    /// Get security policy.
    /// Args:
    /// policy_name: Policy name
    /// Returns:
    /// Policy dictionary
    fn get_policy(&self, policy_name: String) -> HashMap<String, serde_json::Value>;

    /// Set security policy.
    /// Args:
    /// policy_name: Policy name
    /// policy: Policy dictionary
    fn set_policy(&self, policy_name: String, policy: HashMap<String, serde_json::Value>) -> ();

    /// Validate security policy.
    /// Args:
    /// policy: Policy to validate
    /// Returns:
    /// Tuple of (is_valid, error_messages)
    fn validate_policy(&self, policy: HashMap<String, serde_json::Value>) -> (bool, Vec<String>);

    /// Apply security policy.
    /// Args:
    /// policy_name: Policy name
    /// context: Context data
    /// Returns:
    /// True if policy applied successfully
    fn apply_policy(&self, policy_name: String, context: HashMap<String, serde_json::Value>) -> bool;

    /// List all security policies.
    /// Returns:
    /// List of policy names
    fn list_policies(&self) -> Vec<String>;

    /// Remove security policy.
    /// Args:
    /// policy_name: Policy name to remove
    /// Returns:
    /// True if removed
    fn remove_policy(&self, policy_name: String) -> bool;

    /// Get policy violations.
    /// Returns:
    /// List of policy violations
    fn get_policy_violations(&self) -> Vec<HashMap<String, serde_json::Value>>;

    /// Clear policy violations.
    fn clear_policy_violations(&self) -> ();

}

/// Interface for security tokens.
///
/// Enforces consistent security token behavior across XWSystem.
pub trait ISecurityToken {
    /// Generate security token.
    /// Args:
    /// payload: Token payload
    /// expires_in: Expiration time in seconds
    /// Returns:
    /// Generated token
    fn generate_token(&self, payload: HashMap<String, serde_json::Value>, expires_in: i64) -> String;

    /// Validate security token.
    /// Args:
    /// token: Token to validate
    /// Returns:
    /// Tuple of (is_valid, payload)
    fn validate_token(&self, token: String) -> (bool, HashMap<String, serde_json::Value>);

    /// Refresh security token.
    /// Args:
    /// token: Token to refresh
    /// expires_in: New expiration time
    /// Returns:
    /// Refreshed token
    fn refresh_token(&self, token: String, expires_in: i64) -> String;

    /// Revoke security token.
    /// Args:
    /// token: Token to revoke
    /// Returns:
    /// True if revoked
    fn revoke_token(&self, token: String) -> bool;

    /// Check if token is expired.
    /// Args:
    /// token: Token to check
    /// Returns:
    /// True if expired
    fn is_token_expired(&self, token: String) -> bool;

    /// Get token information.
    /// Args:
    /// token: Token to get info for
    /// Returns:
    /// Token information dictionary
    fn get_token_info(&self, token: String) -> HashMap<String, serde_json::Value>;

    /// List active tokens.
    /// Args:
    /// user: Filter by user
    /// Returns:
    /// List of active tokens
    fn list_active_tokens(&self, user: Option<String>) -> Vec<String>;

    /// Cleanup expired tokens.
    /// Returns:
    /// Number of tokens cleaned up
    fn cleanup_expired_tokens(&self) -> i64;

}

/// Protocol for objects that can be hashed securely.
pub trait IHashable {
    /// Generate hash of data.
    fn hash(&self, data: String) -> String;

}

/// Protocol for objects that support encryption/decryption.
pub trait IEncryptable {
    /// Encrypt data.
    fn encrypt(&self, data: String) -> Vec<u8>;

    /// Decrypt data.
    fn decrypt(&self, data: Vec<u8>) -> String;

}
