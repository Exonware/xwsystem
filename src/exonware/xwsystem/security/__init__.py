#exonware/xwsystem/src/exonware/xwsystem/security/__init__.py
"""
XSystem Security Package
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
]
