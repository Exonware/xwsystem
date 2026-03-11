#exonware/xwsystem/src/exonware/xwsystem/security/at_rest.py
"""
At-rest encryption: unified interface and implementations for serialization/XWJSON.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
All strategies (AES256-GCM, XChaCha20-Poly1305, Fernet) implement IAtRestEncryption
with a common envelope format so they are swappable and benchmarkable.
"""

from __future__ import annotations
import os
from abc import ABC, abstractmethod
from typing import Any, Optional
from .contracts import IAtRestEncryption
from .errors import CryptographicError
# Envelope format: magic(4) + version(1) + algo(1) + nonce(12) + salt_len(1) + salt(0..16) + ciphertext
XWJE_MAGIC = b"XWJE"
ENVELOPE_VERSION = 0x01
ALGO_AES256_GCM = 0x00
ALGO_XCHACHA20_POLY1305 = 0x01
ALGO_FERNET = 0x02
NONCE_SIZE = 12
SALT_SIZE = 16
ENVELOPE_HEADER_SIZE = 4 + 1 + 1 + 12 + 1  # magic + version + algo + nonce + salt_len
MAX_SALT_LEN = 32


def _derive_key_pbkdf2(password: str, salt: bytes, iterations: int = 100_000) -> bytes:
    """Derive 32-byte key from password using PBKDF2-HMAC-SHA256."""
    from .kdf import derive_key_pbkdf2
    return derive_key_pbkdf2(password, salt, iterations=iterations)


def build_envelope(
    ciphertext: bytes,
    algo_byte: int,
    nonce: bytes,
    salt: Optional[bytes] = None,
) -> bytes:
    """Build envelope: magic + version + algo + nonce + salt_len + salt + ciphertext."""
    if len(nonce) != NONCE_SIZE:
        raise CryptographicError(f"Nonce must be {NONCE_SIZE} bytes")
    salt = salt or b""
    if len(salt) > MAX_SALT_LEN:
        raise CryptographicError(f"Salt at most {MAX_SALT_LEN} bytes")
    return (
        XWJE_MAGIC
        + bytes([ENVELOPE_VERSION, algo_byte])
        + nonce
        + bytes([len(salt)])
        + salt
        + ciphertext
    )


def parse_envelope(payload: bytes) -> tuple[int, bytes, bytes, bytes]:
    """
    Parse envelope. Returns (algo_byte, nonce, salt, ciphertext).
    Raises CryptographicError if magic/version invalid or payload too short.
    """
    min_len = ENVELOPE_HEADER_SIZE
    if len(payload) < min_len:
        raise CryptographicError("Payload too short for envelope")
    if payload[:4] != XWJE_MAGIC:
        raise CryptographicError("Invalid envelope magic")
    if payload[4] != ENVELOPE_VERSION:
        raise CryptographicError("Unsupported envelope version")
    algo_byte = payload[5]
    nonce = payload[6 : 6 + NONCE_SIZE]
    salt_len = payload[6 + NONCE_SIZE]
    if salt_len > MAX_SALT_LEN:
        raise CryptographicError("Invalid salt length in envelope")
    start = 6 + NONCE_SIZE + 1
    salt = payload[start : start + salt_len]
    ciphertext = payload[start + salt_len :]
    return algo_byte, nonce, salt, ciphertext


def is_envelope(payload: bytes) -> bool:
    """Return True if payload looks like an XWJE envelope."""
    return len(payload) >= 4 and payload[:4] == XWJE_MAGIC


class AAtRestEncryption(ABC):
    """
    Abstract base for at-rest encryption with envelope build/parse.
    Subclasses implement _encrypt_core and _decrypt_core (raw cipher, no envelope).
    """

    def __init__(self, key: Optional[bytes] = None) -> None:
        self._key = key
    @abstractmethod

    def algorithm_id(self) -> str:
        """Return algorithm identifier."""
        ...
    @abstractmethod

    def supports_password(self) -> bool:
        """Return True if password-based KDF is supported."""
        ...
    @abstractmethod

    def _algo_byte(self) -> int:
        """Return envelope algo byte for this implementation."""
        ...
    @abstractmethod

    def _encrypt_core(self, data: bytes, key: bytes) -> tuple[bytes, bytes]:
        """Encrypt with key; return (nonce, ciphertext). Nonce may be empty for Fernet."""
        ...
    @abstractmethod

    def _decrypt_core(self, nonce: bytes, ciphertext: bytes, key: bytes) -> bytes:
        """Decrypt with key; return plaintext."""
        ...

    def _resolve_key(
        self,
        key: Optional[bytes] = None,
        password: Optional[str] = None,
        salt: Optional[bytes] = None,
    ) -> tuple[bytes, Optional[bytes]]:
        """Resolve key from instance key, or key arg, or password (with KDF). Returns (key, salt_for_envelope)."""
        if key is not None:
            return key, None
        if self._key is not None:
            return self._key, None
        if password is not None and self.supports_password():
            salt = salt or os.urandom(SALT_SIZE)
            return _derive_key_pbkdf2(password, salt), salt
        raise CryptographicError("Either key or password must be provided")

    def encrypt(
        self,
        data: bytes,
        *,
        key: Optional[bytes] = None,
        password: Optional[str] = None,
        salt: Optional[bytes] = None,
    ) -> bytes:
        """Encrypt data and return envelope bytes."""
        key_bytes, salt_for_envelope = self._resolve_key(key=key, password=password, salt=salt)
        nonce, ciphertext = self._encrypt_core(data, key_bytes)
        return build_envelope(
            ciphertext,
            self._algo_byte(),
            nonce,
            salt=salt_for_envelope,
        )

    def decrypt(
        self,
        payload: bytes,
        *,
        key: Optional[bytes] = None,
        password: Optional[str] = None,
    ) -> bytes:
        """Decrypt envelope and return plaintext bytes."""
        algo_byte, nonce, salt, ciphertext = parse_envelope(payload)
        if algo_byte != self._algo_byte():
            raise CryptographicError(f"Envelope algo {algo_byte} does not match {self.algorithm_id()}")
        if key is not None:
            key_bytes = key
        elif self._key is not None:
            key_bytes = self._key
        elif password is not None and salt:
            key_bytes = _derive_key_pbkdf2(password, salt)
        else:
            raise CryptographicError("Either key or password must be provided for decryption")
        return self._decrypt_core(nonce, ciphertext, key_bytes)


class AES256GCMAtRest(AAtRestEncryption):
    """At-rest encryption using AES-256-GCM (AEAD)."""

    def __init__(self, key: Optional[bytes] = None) -> None:
        super().__init__(key=key)

    def algorithm_id(self) -> str:
        return "aes256-gcm"

    def supports_password(self) -> bool:
        return True

    def _algo_byte(self) -> int:
        return ALGO_AES256_GCM

    def _encrypt_core(self, data: bytes, key: bytes) -> tuple[bytes, bytes]:
        from .hazmat import AES_GCM, HazmatError
        if len(key) != 32:
            raise CryptographicError("AES-256-GCM requires 32-byte key")
        cipher = AES_GCM(key)
        nonce = os.urandom(NONCE_SIZE)
        ciphertext = cipher.encrypt(nonce, data, None)
        return nonce, ciphertext

    def _decrypt_core(self, nonce: bytes, ciphertext: bytes, key: bytes) -> bytes:
        from .hazmat import AES_GCM, HazmatError
        if len(key) != 32:
            raise CryptographicError("AES-256-GCM requires 32-byte key")
        cipher = AES_GCM(key)
        return cipher.decrypt(nonce, ciphertext, None)


class XChaCha20Poly1305AtRest(AAtRestEncryption):
    """At-rest encryption using XChaCha20-Poly1305 (AEAD)."""

    def __init__(self, key: Optional[bytes] = None) -> None:
        super().__init__(key=key)

    def algorithm_id(self) -> str:
        return "xchacha20-poly1305"

    def supports_password(self) -> bool:
        return True

    def _algo_byte(self) -> int:
        return ALGO_XCHACHA20_POLY1305

    def _encrypt_core(self, data: bytes, key: bytes) -> tuple[bytes, bytes]:
        from .hazmat import ChaCha20Poly1305_Cipher
        if len(key) != 32:
            raise CryptographicError("XChaCha20-Poly1305 requires 32-byte key")
        cipher = ChaCha20Poly1305_Cipher(key)
        nonce = os.urandom(NONCE_SIZE)
        ciphertext = cipher.encrypt(nonce, data, None)
        return nonce, ciphertext

    def _decrypt_core(self, nonce: bytes, ciphertext: bytes, key: bytes) -> bytes:
        from .hazmat import ChaCha20Poly1305_Cipher
        if len(key) != 32:
            raise CryptographicError("XChaCha20-Poly1305 requires 32-byte key")
        cipher = ChaCha20Poly1305_Cipher(key)
        return cipher.decrypt(nonce, ciphertext, None)


class FernetAtRest(AAtRestEncryption):
    """At-rest encryption using Fernet (AES-128-CBC + HMAC). Uses envelope for consistency."""

    def __init__(self, key: Optional[bytes] = None) -> None:
        super().__init__(key=key)

    def algorithm_id(self) -> str:
        return "fernet"

    def supports_password(self) -> bool:
        return True

    def _algo_byte(self) -> int:
        return ALGO_FERNET

    def _encrypt_core(self, data: bytes, key: bytes) -> tuple[bytes, bytes]:
        from base64 import urlsafe_b64encode
        from cryptography.fernet import Fernet
        if len(key) != 32:
            raise CryptographicError("Fernet requires 32-byte key (will be base64-encoded)")
        fernet_key = urlsafe_b64encode(key)
        f = Fernet(fernet_key)
        ciphertext = f.encrypt(data)
        return bytes(NONCE_SIZE), ciphertext

    def _decrypt_core(self, nonce: bytes, ciphertext: bytes, key: bytes) -> bytes:
        from base64 import urlsafe_b64encode
        from cryptography.fernet import Fernet
        if len(key) != 32:
            raise CryptographicError("Fernet requires 32-byte key")
        fernet_key = urlsafe_b64encode(key)
        f = Fernet(fernet_key)
        return f.decrypt(ciphertext)


def get_at_rest_encryption(algorithm_id: str, key: Optional[bytes] = None) -> IAtRestEncryption:
    """Factory: return IAtRestEncryption implementation by algorithm id."""
    aliases = {
        "aes256-gcm": AES256GCMAtRest,
        "aes256_gcm": AES256GCMAtRest,
        "xchacha20-poly1305": XChaCha20Poly1305AtRest,
        "xchacha20_poly1305": XChaCha20Poly1305AtRest,
        "fernet": FernetAtRest,
    }
    cls = aliases.get(algorithm_id.lower())
    if cls is None:
        raise CryptographicError(f"Unknown at-rest algorithm: {algorithm_id}")
    return cls(key=key)
