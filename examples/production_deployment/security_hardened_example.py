#!/usr/bin/env python3
#exonware/xwsystem/examples/production_deployment/security_hardened_example.py
"""
Security-Hardened Application Example with xwsystem

This example demonstrates a security-hardened application using xwsystem
for path validation, secure storage, encryption, and security monitoring.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
"""

import logging
import os
from typing import Dict, Any, Optional
from pathlib import Path

from exonware.xwsystem import (
    PathValidator,
    SecureStorage,
    SecureHash,
    SymmetricEncryption,
    AsymmetricEncryption,
    AtomicFileWriter,
    SafeTypeValidator,
    ResourceLimits,
)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class SecurityHardenedApp:
    """Security-hardened application with xwsystem."""
    
    def __init__(self, base_path: str = "/secure/app/data"):
        # Path validation
        self.path_validator = PathValidator(base_path=base_path)
        
        # Secure storage
        self.secure_storage = SecureStorage()
        
        # Encryption
        self.symmetric_encryption = SymmetricEncryption()
        self.asymmetric_encryption, self.private_key, self.public_key = \
            AsymmetricEncryption.generate_key_pair(4096)
        
        # Validation
        self.type_validator = SafeTypeValidator()
        self.resource_limits = ResourceLimits(
            max_file_size=10 * 1024 * 1024,  # 10MB
            max_memory=100 * 1024 * 1024,     # 100MB
            max_depth=100
        )
        
        # Security configuration
        self.security_config = {
            "allowed_extensions": [".json", ".yaml", ".txt"],
            "max_file_size": 10 * 1024 * 1024,
            "require_encryption": True,
        }
    
    def validate_path(self, file_path: str) -> Path:
        """Validate and sanitize file path."""
        try:
            safe_path = self.path_validator.validate_path(file_path)
            logger.info(f"Path validated: {file_path} -> {safe_path}")
            return safe_path
        except Exception as e:
            logger.error(f"Path validation failed: {e}")
            raise ValueError(f"Invalid path: {file_path}")
    
    def validate_file_extension(self, file_path: str) -> bool:
        """Validate file extension is allowed."""
        ext = Path(file_path).suffix.lower()
        if ext not in self.security_config["allowed_extensions"]:
            raise ValueError(f"File extension not allowed: {ext}")
        return True
    
    def validate_file_size(self, file_size: int) -> bool:
        """Validate file size is within limits."""
        if file_size > self.security_config["max_file_size"]:
            raise ValueError(f"File size exceeds limit: {file_size}")
        return True
    
    def store_secure_data(self, key: str, data: Dict[str, Any], encrypt: bool = True):
        """Store data securely."""
        try:
            # Validate data structure
            if not self.type_validator.validate_untrusted_data(data, max_depth=10):
                raise ValueError("Data validation failed")
            
            # Encrypt if required
            if encrypt and self.security_config["require_encryption"]:
                data_str = str(data)
                encrypted_data = self.symmetric_encryption.encrypt(data_str)
                self.secure_storage.store(key, {"encrypted": True, "data": encrypted_data})
            else:
                self.secure_storage.store(key, data)
            
            logger.info(f"Data stored securely: {key}")
        except Exception as e:
            logger.error(f"Error storing data: {e}")
            raise
    
    def retrieve_secure_data(self, key: str) -> Dict[str, Any]:
        """Retrieve securely stored data."""
        try:
            stored = self.secure_storage.retrieve(key)
            
            # Decrypt if encrypted
            if stored.get("encrypted"):
                encrypted_data = stored["data"]
                decrypted_data = self.symmetric_encryption.decrypt(encrypted_data)
                return eval(decrypted_data)  # In production, use proper deserialization
            else:
                return stored
        except Exception as e:
            logger.error(f"Error retrieving data: {e}")
            raise
    
    def hash_sensitive_data(self, data: str) -> str:
        """Create secure hash of sensitive data."""
        return SecureHash.sha256(data)
    
    def sign_data(self, data: str) -> bytes:
        """Sign data with private key."""
        return self.asymmetric_encryption.sign(data, self.private_key)
    
    def verify_signature(self, data: str, signature: bytes) -> bool:
        """Verify data signature."""
        return self.asymmetric_encryption.verify(data, signature, self.public_key)
    
    def write_secure_file(self, file_path: str, content: str, encrypt: bool = True):
        """Write file securely with atomic operations."""
        # Validate path
        safe_path = self.validate_path(file_path)
        
        # Validate extension
        self.validate_file_extension(str(safe_path))
        
        # Validate size
        content_size = len(content.encode('utf-8'))
        self.validate_file_size(content_size)
        
        # Encrypt if required
        if encrypt:
            encrypted_content = self.symmetric_encryption.encrypt(content)
            content = encrypted_content
        
        # Atomic write
        with AtomicFileWriter(safe_path) as writer:
            writer.write(content)
        
        logger.info(f"File written securely: {safe_path}")
    
    def read_secure_file(self, file_path: str, decrypt: bool = True) -> str:
        """Read file securely."""
        safe_path = self.validate_path(file_path)
        
        with open(safe_path, 'rb' if decrypt else 'r') as f:
            content = f.read()
        
        if decrypt:
            content = self.symmetric_encryption.decrypt(content)
        
        return content.decode('utf-8') if isinstance(content, bytes) else content


# Example usage
def main():
    """Example security-hardened application."""
    app = SecurityHardenedApp(base_path="/tmp/secure_app")
    
    # Store sensitive data
    sensitive_data = {
        "api_key": "sk_live_1234567890",
        "user_id": "user_123",
        "permissions": ["read", "write"]
    }
    
    app.store_secure_data("api_keys", sensitive_data, encrypt=True)
    
    # Retrieve data
    retrieved = app.retrieve_secure_data("api_keys")
    print(f"Retrieved data: {retrieved}")
    
    # Hash sensitive data
    password = "my_secret_password"
    password_hash = app.hash_sensitive_data(password)
    print(f"Password hash: {password_hash}")
    
    # Sign data
    important_data = "This is important data"
    signature = app.sign_data(important_data)
    print(f"Signature created: {signature[:20]}...")
    
    # Verify signature
    is_valid = app.verify_signature(important_data, signature)
    print(f"Signature valid: {is_valid}")
    
    # Write secure file
    app.write_secure_file("config/secure_config.json", '{"key": "value"}', encrypt=True)
    
    # Read secure file
    content = app.read_secure_file("config/secure_config.json", decrypt=True)
    print(f"Read content: {content}")


if __name__ == "__main__":
    main()
