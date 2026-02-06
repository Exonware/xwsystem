#exonware/xwsystem/src/exonware/xwsystem/security/facade.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.5
Generation Date: January 2026

XWSecurity & XWCrypto - Unified Security Facades

Simplified API for security operations:
- Hashing (SHA-256, SHA-512, BLAKE2b, HMAC)
- Password hashing and verification
- Encryption (Symmetric, Asymmetric)
- Secure storage
- Path validation
"""

from typing import Any, Optional, Tuple

from .crypto import (
    SecureHash,
    SecureRandom,
    SymmetricEncryption,
    AsymmetricEncryption,
    SecureStorage,
    AsyncSecureStorage,
    AsyncSymmetricEncryption,
    AsyncAsymmetricEncryption,
    hash_password,
    verify_password,
    generate_api_key,
    generate_session_token,
)
from .path_validator import PathValidator, PathSecurityError
from ..config.logging_setup import get_logger

logger = get_logger(__name__)


class XWSecurity:
    """
    Unified security facade - simple API for security operations.
    
    Examples:
        >>> # Hashing
        >>> hash_value = XWSecurity.hash("password", algorithm="sha256")
        >>> is_valid = XWSecurity.verify_hash("password", hash_value)
        
        >>> # Password hashing
        >>> hashed = XWSecurity.hash_password("mypassword")
        >>> XWSecurity.verify_password("mypassword", hashed)
        
        >>> # Path validation
        >>> XWSecurity.validate_path("/safe/dir/file.txt", base_path="/safe")
        
        >>> # Secure storage
        >>> storage = XWSecurity.Storage()
        >>> storage.set("api_key", "sk_...")
        >>> value = storage.get("api_key")
    """
    
    @staticmethod
    def hash(data: str | bytes, algorithm: str = "sha256") -> str:
        """
        Hash data with specified algorithm.
        
        Args:
            data: Data to hash
            algorithm: Hash algorithm ("sha256", "sha512", "blake2b", "hmac_sha256")
            
        Returns:
            Hex digest string
        """
        if algorithm == "sha256":
            return SecureHash.sha256(data)
        elif algorithm == "sha512":
            return SecureHash.sha512(data)
        elif algorithm == "blake2b":
            return SecureHash.blake2b(data)
        else:
            raise ValueError(f"Unknown algorithm: {algorithm}")
    
    @staticmethod
    def verify_hash(data: str | bytes, hash_value: str, algorithm: str = "sha256") -> bool:
        """
        Verify hash matches data.
        
        Args:
            data: Original data
            hash_value: Hash to verify against
            algorithm: Hash algorithm used
            
        Returns:
            True if hash matches
        """
        computed = XWSecurity.hash(data, algorithm)
        return computed == hash_value
    
    @staticmethod
    def hash_password(password: str, rounds: int = 12) -> str:
        """
        Hash password securely.
        
        Args:
            password: Password to hash
            rounds: Bcrypt rounds (default: 12)
            
        Returns:
            Hashed password string
        """
        return hash_password(password, rounds)
    
    @staticmethod
    def verify_password(password: str, hashed_password: str) -> bool:
        """
        Verify password against hash.
        
        Args:
            password: Plain password
            hashed_password: Hashed password
            
        Returns:
            True if password matches
        """
        return verify_password(password, hashed_password)
    
    @staticmethod
    def validate_path(path: str, base_path: Optional[str] = None) -> str:
        """
        Validate and sanitize file path.
        
        Args:
            path: Path to validate
            base_path: Base directory (prevents directory traversal)
            
        Returns:
            Validated path
            
        Raises:
            PathSecurityError: If path is invalid
        """
        validator = PathValidator(base_path) if base_path else PathValidator()
        return validator.validate_path(path)
    
    @staticmethod
    def generate_api_key(length: int = 32) -> str:
        """Generate cryptographically secure API key."""
        return generate_api_key(length)
    
    @staticmethod
    def generate_session_token(length: int = 32) -> str:
        """Generate cryptographically secure session token."""
        return generate_session_token(length)
    
    class Storage:
        """Secure encrypted key-value storage."""
        
        def __init__(self, password: Optional[str] = None):
            """
            Initialize secure storage.
            
            Args:
                password: Optional password for encryption (auto-generated if None)
            """
            self._storage = SecureStorage() if password is None else SecureStorage.from_password(password)
        
        def set(self, key: str, value: Any) -> None:
            """Store encrypted value."""
            self._storage.store(key, value)
        
        def get(self, key: str) -> Any:
            """Retrieve and decrypt value."""
            return self._storage.retrieve(key)
        
        def delete(self, key: str) -> bool:
            """Delete stored value."""
            return self._storage.delete(key)


class XWCrypto:
    """
    Unified cryptography facade - simple API for encryption operations.
    
    Examples:
        >>> # Symmetric encryption
        >>> crypto = XWCrypto()  # Auto-generates key
        >>> encrypted = crypto.encrypt("secret data")
        >>> decrypted = crypto.decrypt(encrypted)
        
        >>> # Password-based encryption
        >>> crypto = XWCrypto.from_password("mypassword")
        >>> encrypted = crypto.encrypt("data")
        
        >>> # Asymmetric encryption
        >>> public, private = XWCrypto.generate_key_pair()
        >>> encrypted = XWCrypto.encrypt("data", public_key=public)
        >>> decrypted = XWCrypto.decrypt(encrypted, private_key=private)
    """
    
    def __init__(self, key: Optional[bytes] = None):
        """
        Initialize crypto with key.
        
        Args:
            key: Optional encryption key (auto-generated if None)
        """
        if key is None:
            self._crypto = SymmetricEncryption()
            self._key = self._crypto.key
        else:
            self._crypto = SymmetricEncryption(key)
            self._key = key
    
    @classmethod
    def from_password(cls, password: str) -> "XWCrypto":
        """
        Create crypto instance from password.
        
        Args:
            password: Password for key derivation
            
        Returns:
            XWCrypto instance
        """
        instance = cls.__new__(cls)
        # Derive key from password
        key, _ = SymmetricEncryption.derive_key_from_password(password)
        instance._crypto = SymmetricEncryption(key)
        instance._key = key
        return instance
    
    def encrypt(self, data: str | bytes) -> bytes:
        """Encrypt data using instance's crypto."""
        return self._crypto.encrypt(data)
    
    def decrypt(self, encrypted_data: bytes) -> str | bytes:
        """Decrypt data using instance's crypto."""
        return self._crypto.decrypt(encrypted_data)
    
    @property
    def key(self) -> bytes:
        """Get encryption key."""
        return self._key
    
    @staticmethod
    def encrypt_static(data: str | bytes, password: Optional[str] = None, public_key: Optional[bytes] = None, key: Optional[bytes] = None) -> bytes:
        """
        Static method to encrypt data.
        
        Args:
            data: Data to encrypt
            password: Password for symmetric encryption (key derivation)
            public_key: Public key for asymmetric encryption
            key: Direct encryption key (for symmetric)
            
        Returns:
            Encrypted data
        """
        if public_key:
            crypto = AsymmetricEncryption(public_key=public_key)
            return crypto.encrypt(data)
        elif password:
            # Use deterministic salt based on password for consistent encryption/decryption
            # Note: This is less secure than random salt but allows same password to work
            import hashlib
            salt = hashlib.sha256(password.encode()).digest()[:16]  # Deterministic salt
            key, _ = SymmetricEncryption.derive_key_from_password(password, salt=salt)
            crypto = SymmetricEncryption(key)
            return crypto.encrypt(data)
        elif key:
            crypto = SymmetricEncryption(key)
            return crypto.encrypt(data)
        else:
            # Auto-generate key for one-time encryption
            crypto = SymmetricEncryption()
            return crypto.encrypt(data)
    
    @staticmethod  
    def decrypt_static(encrypted_data: bytes, password: Optional[str] = None, private_key: Optional[bytes] = None, key: Optional[bytes] = None) -> bytes:
        """
        Static method to decrypt data.
        
        Args:
            encrypted_data: Encrypted data
            password: Password for symmetric decryption (key derivation)
            private_key: Private key for asymmetric decryption
            key: Direct decryption key (for symmetric)
            
        Returns:
            Decrypted data
        """
        if private_key:
            crypto = AsymmetricEncryption(private_key=private_key)
            return crypto.decrypt(encrypted_data)
        elif password:
            # Use same deterministic salt for decryption
            import hashlib
            salt = hashlib.sha256(password.encode()).digest()[:16]  # Deterministic salt
            key, _ = SymmetricEncryption.derive_key_from_password(password, salt=salt)
            crypto = SymmetricEncryption(key)
            return crypto.decrypt(encrypted_data)
        elif key:
            crypto = SymmetricEncryption(key)
            return crypto.decrypt(encrypted_data)
        else:
            raise ValueError("Either password, key, or private_key must be provided")
    
    @staticmethod
    def generate_key_pair(key_size: int = 2048) -> Tuple[bytes, bytes]:
        """
        Generate RSA key pair.
        
        Args:
            key_size: Key size in bits (default: 2048)
            
        Returns:
            Tuple of (public_key, private_key)
        """
        crypto = AsymmetricEncryption.generate_key_pair(key_size)
        return crypto.public_key, crypto.private_key
