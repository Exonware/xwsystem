// #exonware/xwsystem/rust/src/security/mod.rs
//! XSystem Security Package
//! 
//! Provides security utilities including:
//! - Path validation and resource limits
//! - Security contracts and interfaces (IAuthenticatable, IAuthorization, ISecurityToken)
//! - Abstract base classes for authentication (AAuthProvider, ATokenInfo, AUserInfo)
//! - Cryptography and encryption
//! - Security errors and definitions
//! 
//! Note: Authentication implementations (OAuth2Provider, JWTProvider, SAMLProvider, EnterpriseAuth)
//! have been moved to xwauth, which extends xwsystem. Use xwauth for actual authentication providers.

pub mod base;
pub mod contracts;
pub mod defs;
pub mod errors;
pub mod monitor;
pub mod path_validator;
pub mod policy;
pub mod resource_limits;
pub mod validator;

pub use base::{AAuthProvider, ASecurityMonitorBase, ASecurityPolicyBase, ASecurityValidatorBase, ATokenInfo, AUserInfo};

pub use contracts::{IAuditable, IAuthenticatable, IAuthorization, ISecure, ISecurityMonitor, ISecurityPolicy, ISecurityToken, ISecurityValidator};

pub use defs::{OAuth2GrantType};

pub use errors::{AuthenticationError, AuthorizationError, JWTError, OAuth2Error, SAMLError, TokenExpiredError};

pub use monitor::{SecurityMonitor};

pub use path_validator::{PathSecurityError, PathValidator};

pub use policy::{SecurityPolicy};

pub use resource_limits::{GenericLimitError, ResourceLimits, get_resource_limits, reset_resource_limits};

pub use validator::{SecurityValidator};
