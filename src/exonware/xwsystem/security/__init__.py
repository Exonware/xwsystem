#exonware/xwsystem/src/exonware/xwsystem/security/__init__.py
"""
xwsystem Security Package
Provides security utilities including:
- Path validation and resource limits
- Security contracts and interfaces (IAuthenticatable, IAuthorization, ISecurityToken)
- Abstract base classes for authentication (AAuthProvider, ATokenInfo, AUserInfo)
- Cryptography and encryption
- Security errors and definitions
"""

from .path_validator import PathValidator, PathSecurityError
from .resource_limits import (
    GenericLimitError,
    ResourceLimits,
    get_resource_limits,
    reset_resource_limits,
)
from .file_security import (
    FileSecurity,
    FileSecurityError,
    FileSizeLimitError,
    FileIOError,
    get_file_security,
    set_file_security,
)
from .audit import SecurityAuditor, SecurityLevel, SecurityIssue, audit_security
# Security implementations
from .validator import SecurityValidator
from .monitor import SecurityMonitor
from .policy import SecurityPolicy
# Base classes and contracts (kept in xwsystem - foundation layer)
from .base import (
    AAuthProvider,
    ATokenInfo,
    AUserInfo,
    ASecurityValidatorBase,
    ASecurityMonitorBase,
    ASecurityPolicyBase,
)
# Unified Facades
from .facade import XWSecurity, XWCrypto
from .errors import (
    AuthenticationError,
    AuthorizationError,
    TokenExpiredError,
    OAuth2Error,
    JWTError,
    SAMLError,
)
from .defs import OAuth2GrantType
# Contracts/interfaces (kept in xwsystem - foundation layer)
from .contracts import (
    AuthContext,
    IAuthContextResolver,
    PolicyContext,
    IAuthenticatable,
    IAuthorization,
    ISecurityToken,
    ISecure,
    IAuditable,
    ISecurityValidator,
    ISecurityMonitor,
    ISecurityPolicy,
    IAtRestEncryption,
)
# Key derivation (PBKDF2 + optional Argon2id)
from .kdf import (
    derive_key_pbkdf2,
    derive_key_argon2id,
    derive_key_from_password,
)
# At-rest encryption (unified interface for serialization/XWJSON)
from .at_rest import (
    AAtRestEncryption,
    AES256GCMAtRest,
    XChaCha20Poly1305AtRest,
    FernetAtRest,
    get_at_rest_encryption,
    build_envelope,
    parse_envelope,
    is_envelope,
    XWJE_MAGIC,
)
from .secret_key import SecretKeyStore, secret_key
from .normalization import (
    claims_from_principal,
    subject_id_from_principal,
    tenant_id_from_principal,
    tenant_id_from_claims_mapping,
    org_id_from_principal,
    project_id_from_principal,
    roles_from_principal,
    policy_context_from_principal,
    policy_context_to_dict,
    infer_scope_set_from_roles,
    scopes_from_policy_context,
    default_tenant_row_visible,
    default_owner_row_visible,
    resolve_tenant_id_layered,
    auth_context_compat_from_policy_context,
)
from .tenancy import (
    TenancyContext,
    build_tenancy_context,
    effective_isolation_key,
    extract_path_org_id,
    introspection_claims_org_project,
    is_instance_operator_introspection,
    org_id_from_claims_mapping,
    project_id_from_claims_mapping,
    tenancy_violation_for_path_org,
)
from .auth_helpers import (
    parse_authorization_bearer,
    resolve_bearer_or_cookie_token,
    resolve_principal_from_auth_provider,
)
from .oauth_errors import (
    oauth_error_body,
    oauth_error_status,
    oauth_error_response,
    oauth_error_to_http_parts,
)
__all__ = [
    # Unified Facades
    "XWSecurity",
    "XWCrypto",
    # Path & Resources
    "PathValidator",
    "PathSecurityError",
    "ResourceLimits",
    "GenericLimitError",
    "get_resource_limits",
    "reset_resource_limits",
    # File Security
    "FileSecurity",
    "FileSecurityError",
    "FileSizeLimitError",
    "FileIOError",
    "get_file_security",
    "set_file_security",
    # Security Implementations
    "SecurityValidator",
    "SecurityMonitor",
    "SecurityPolicy",
    # Authentication Base Classes (foundation - kept in xwsystem)
    "AAuthProvider",
    "ATokenInfo",
    "AUserInfo",
    "ASecurityValidatorBase",
    "ASecurityMonitorBase",
    "ASecurityPolicyBase",
    # Security Errors
    "AuthenticationError",
    "AuthorizationError",
    "TokenExpiredError",
    "OAuth2Error",
    "JWTError",
    "SAMLError",
    # Security Definitions
    "OAuth2GrantType",
    # Security Audit
    "SecurityAuditor",
    "SecurityLevel",
    "SecurityIssue",
    "audit_security",
    # Security Contracts/Interfaces (foundation - kept in xwsystem)
    "AuthContext",
    "IAuthContextResolver",
    "PolicyContext",
    "IAuthenticatable",
    "IAuthorization",
    "ISecurityToken",
    "ISecure",
    "IAuditable",
    "ISecurityValidator",
    "ISecurityMonitor",
    "ISecurityPolicy",
    "IAtRestEncryption",
    # Key derivation
    "derive_key_pbkdf2",
    "derive_key_argon2id",
    "derive_key_from_password",
    # At-rest encryption
    "AAtRestEncryption",
    "AES256GCMAtRest",
    "XChaCha20Poly1305AtRest",
    "FernetAtRest",
    "get_at_rest_encryption",
    "build_envelope",
    "parse_envelope",
    "is_envelope",
    "XWJE_MAGIC",
    # Secret key helper
    "SecretKeyStore",
    "secret_key",
    # Principal normalization helpers
    "claims_from_principal",
    "subject_id_from_principal",
    "tenant_id_from_principal",
    "tenant_id_from_claims_mapping",
    "roles_from_principal",
    "policy_context_from_principal",
    "policy_context_to_dict",
    "infer_scope_set_from_roles",
    "scopes_from_policy_context",
    "default_tenant_row_visible",
    "default_owner_row_visible",
    "resolve_tenant_id_layered",
    "auth_context_compat_from_policy_context",
    # B2B / multi-tenant tenancy
    "TenancyContext",
    "build_tenancy_context",
    "effective_isolation_key",
    "extract_path_org_id",
    "introspection_claims_org_project",
    "is_instance_operator_introspection",
    "org_id_from_claims_mapping",
    "project_id_from_claims_mapping",
    "tenancy_violation_for_path_org",
    # Shared auth helpers
    "parse_authorization_bearer",
    "resolve_bearer_or_cookie_token",
    "resolve_principal_from_auth_provider",
    # OAuth error helper primitives
    "oauth_error_body",
    "oauth_error_status",
    "oauth_error_response",
    "oauth_error_to_http_parts",
]
