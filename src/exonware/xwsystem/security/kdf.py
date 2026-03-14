#exonware/xwsystem/src/exonware/xwsystem/security/kdf.py
"""
Key derivation helpers: PBKDF2 (default) and optional Argon2id.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
PBKDF2 is always available (cryptography). Argon2id is used when argon2-cffi is installed.
"""

from __future__ import annotations
from .errors import CryptographicError
# Default salt size for password KDF
DEFAULT_SALT_SIZE = 16
# Argon2id defaults (time_cost=2, memory_cost=65536)
ARGON2_TIME_COST = 2
ARGON2_MEMORY_COST_KB = 65536  # 64 MB
ARGON2_HASH_LEN = 32


def derive_key_pbkdf2(
    password: str,
    salt: bytes,
    *,
    iterations: int = 100_000,
    key_len: int = 32,
) -> bytes:
    """
    Derive key from password using PBKDF2-HMAC-SHA256.
    Always available (uses cryptography).
    """
    from cryptography.hazmat.primitives import hashes
    from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
    kdf = PBKDF2HMAC(
        algorithm=hashes.SHA256(),
        length=key_len,
        salt=salt,
        iterations=iterations,
    )
    return kdf.derive(password.encode("utf-8"))


def derive_key_argon2id(
    password: str,
    salt: bytes,
    *,
    time_cost: int = ARGON2_TIME_COST,
    memory_cost_kb: int = ARGON2_MEMORY_COST_KB,
    hash_len: int = ARGON2_HASH_LEN,
) -> bytes | None:
    """
    Derive key from password using Argon2id.
    Returns None if argon2-cffi is not installed; otherwise returns 32-byte key.
    """
    try:
        from argon2 import PasswordHasher
        from argon2.low_level import Type as Argon2Type, hash_secret_raw
    except ImportError:
        return None
    try:
        raw = hash_secret_raw(
            secret=password.encode("utf-8"),
            salt=salt,
            time_cost=time_cost,
            memory_cost=memory_cost_kb,
            parallelism=1,
            hash_len=hash_len,
            type=Argon2Type.ID,
        )
        return raw
    except Exception as e:
        raise CryptographicError(f"Argon2id KDF failed: {e}") from e


def derive_key_from_password(
    password: str,
    salt: bytes,
    *,
    use_argon2: bool = False,
) -> bytes:
    """
    Derive 32-byte key from password. Uses Argon2id when use_argon2=True and
    argon2-cffi is available; otherwise PBKDF2-HMAC-SHA256.
    """
    if use_argon2:
        key = derive_key_argon2id(password, salt)
        if key is not None:
            return key
    return derive_key_pbkdf2(password, salt)
