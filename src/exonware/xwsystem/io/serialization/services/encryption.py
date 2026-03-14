#exonware/xwsystem/src/exonware/xwsystem/io/serialization/services/encryption.py
"""
Encryption service for serialization pipeline: bytes -> encrypted bytes (envelope).
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
"""

from __future__ import annotations
from typing import Any
from exonware.xwsystem.security import get_at_rest_encryption, is_envelope
from exonware.xwsystem.security.errors import CryptographicError


class EncryptionService:
    """
    Bytes-in, bytes-out encryption using IAtRestEncryption (envelope format).
    Used by the serialization pipeline when encryption=... is passed to save_file/load_file.
    """

    def encrypt(
        self,
        data: bytes,
        *,
        key: bytes | None = None,
        password: str | None = None,
        algorithm: str = "aes256-gcm",
        **kwargs: Any,
    ) -> bytes:
        """
        Encrypt data and return envelope bytes (XWJE magic + nonce + salt + ciphertext).
        Either key or password must be provided.
        """
        enc = get_at_rest_encryption(algorithm, key=key)
        return enc.encrypt(data, key=key, password=password, **kwargs)

    def decrypt(
        self,
        payload: bytes,
        *,
        key: bytes | None = None,
        password: str | None = None,
        algorithm: str | None = None,
        **kwargs: Any,
    ) -> bytes:
        """
        Decrypt envelope payload and return plaintext bytes.
        If algorithm is None, it is read from the envelope.
        """
        if not is_envelope(payload):
            raise CryptographicError("Payload is not an encrypted envelope (missing XWJE magic)")
        # Algorithm is in the envelope; we need key or password. get_at_rest_encryption
        # needs algorithm for construction - we parse envelope to get algo byte and map to id
        from exonware.xwsystem.security.at_rest import parse_envelope, ALGO_AES256_GCM, ALGO_XCHACHA20_POLY1305, ALGO_FERNET
        algo_byte, _nonce, salt, _ciphertext = parse_envelope(payload)
        algo_map = {ALGO_AES256_GCM: "aes256-gcm", ALGO_XCHACHA20_POLY1305: "xchacha20-poly1305", ALGO_FERNET: "fernet"}
        algo_id = algo_map.get(algo_byte, "aes256-gcm")
        enc = get_at_rest_encryption(algo_id, key=key)
        return enc.decrypt(payload, key=key, password=password, **kwargs)
    @staticmethod

    def is_encrypted(payload: bytes) -> bool:
        """Return True if payload looks like an XWJE encrypted envelope."""
        return is_envelope(payload)
