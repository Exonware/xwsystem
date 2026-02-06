// #exonware/xwsystem/rust/src/security/defs.rs
//exonware/xwsystem/security/types.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Security types and enums for XWSystem.

/// Security levels.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Encryption algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    #[serde(rename = "aes_128")]
    Aes128,
    #[serde(rename = "aes_256")]
    Aes256,
    #[serde(rename = "rsa_2048")]
    Rsa2048,
    #[serde(rename = "rsa_4096")]
    Rsa4096,
    Chacha20,
    Blake2b,
    Sha256,
    Sha512,
}

/// Hash algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Blake2b,
    Blake2s,
    Pbkdf2,
    Scrypt,
}

/// Authentication methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    Password,
    Token,
    Certificate,
    Biometric,
    #[serde(rename = "multi_factor")]
    MultiFactor,
    Oauth,
    Saml,
    Ldap,
}

/// Authorization levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorizationLevel {
    None,
    Read,
    Write,
    Admin,
    #[serde(rename = "super_admin")]
    SuperAdmin,
}

/// Audit event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEvent {
    Login,
    Logout,
    Access,
    Modify,
    Delete,
    Create,
    Execute,
    #[serde(rename = "failed_access")]
    FailedAccess,
}

/// OAuth2 grant types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OAuth2GrantType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "client_credentials")]
    ClientCredentials,
    #[serde(rename = "password")]
    ResourceOwner,
    #[serde(rename = "refresh_token")]
    RefreshToken,
}
