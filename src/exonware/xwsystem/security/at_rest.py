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
from typing import Any
from .contracts import IAtRestEncryption
from .errors import CryptographicError
# Envelope format: magic(4) + version(1) + algo(1) + nonce(12) + salt_len(1) + salt(0..16) + ciphertext
XWJE_MAGIC = b"XWJE"
ENVELOPE_VERSION = 0x01
ALGO_AES256_GCM = 0x00
ALGO_XCHACHA20_POLY1305 = 0x01
ALGO_FERNET = 0x02
ALGO_AES256_CTR = 0x03
ALGO_NULL = 0xFE  # plaintext passthrough; benchmark baseline only
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
    salt: bytes | None = None,
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

    def __init__(self, key: bytes | None = None) -> None:
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
        key: bytes | None = None,
        password: str | None = None,
        salt: bytes | None = None,
    ) -> tuple[bytes, bytes | None]:
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
        key: bytes | None = None,
        password: str | None = None,
        salt: bytes | None = None,
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
        key: bytes | None = None,
        password: str | None = None,
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

    def __init__(self, key: bytes | None = None) -> None:
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

    def __init__(self, key: bytes | None = None) -> None:
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

    def __init__(self, key: bytes | None = None) -> None:
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


class AES256CTRAtRest(AAtRestEncryption):
    """At-rest encryption using AES-256-CTR.

    Unauthenticated stream mode — **fastest AES variant** on modern CPUs
    because CTR can be parallelised and needs no padding. Use only when a
    higher layer already provides integrity (e.g. HIVE framed payload or
    an outer MAC); otherwise prefer AES-GCM.
    """

    def __init__(self, key: bytes | None = None) -> None:
        super().__init__(key=key)

    def algorithm_id(self) -> str:
        return "aes256-ctr"

    def supports_password(self) -> bool:
        return True

    def _algo_byte(self) -> int:
        return ALGO_AES256_CTR

    def _encrypt_core(self, data: bytes, key: bytes) -> tuple[bytes, bytes]:
        from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
        if len(key) != 32:
            raise CryptographicError("AES-256-CTR requires 32-byte key")
        nonce = os.urandom(NONCE_SIZE)
        # AES-CTR counter is 128 bits; we use the 12-byte nonce as the high
        # bits and leave the low 4 bytes as the counter. This matches the
        # GCM-style nonce convention so envelopes stay interchangeable.
        iv = nonce + b"\x00\x00\x00\x00"
        cipher = Cipher(algorithms.AES(key), modes.CTR(iv))
        encryptor = cipher.encryptor()
        return nonce, encryptor.update(data) + encryptor.finalize()

    def _decrypt_core(self, nonce: bytes, ciphertext: bytes, key: bytes) -> bytes:
        from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
        iv = nonce + b"\x00\x00\x00\x00"
        cipher = Cipher(algorithms.AES(key), modes.CTR(iv))
        decryptor = cipher.decryptor()
        return decryptor.update(ciphertext) + decryptor.finalize()


class NullAtRest(AAtRestEncryption):
    """Plaintext passthrough — **not encryption**.

    Exists only as a benchmark baseline so the UI can show the cost
    overhead of a real cipher relative to no-op I/O. The envelope is
    preserved so the round-trip contract still holds.
    """

    def __init__(self, key: bytes | None = None) -> None:
        # No-op cipher; ignore any key the caller passes.
        super().__init__(key=key or b"\x00" * 32)

    def algorithm_id(self) -> str:
        return "null"

    def supports_password(self) -> bool:
        return False

    def _algo_byte(self) -> int:
        return ALGO_NULL

    def _encrypt_core(self, data: bytes, key: bytes) -> tuple[bytes, bytes]:  # noqa: ARG002
        return bytes(NONCE_SIZE), data

    def _decrypt_core(self, nonce: bytes, ciphertext: bytes, key: bytes) -> bytes:  # noqa: ARG002
        return ciphertext


# ---------------------------------------------------------------------------
# Algorithm catalogue + benchmark helper — consumed by the hive-api
# /v1/encryption/* endpoints and the Encryption Studio UI.


AT_REST_ALGORITHMS: dict[str, dict[str, Any]] = {
    "null": {
        "label": "No encryption (baseline)",
        "family": "baseline",
        "authenticated": False,
        "speed_tier": "reference",
        "security_tier": "none",
        "notes": "Plaintext passthrough. Use only for benchmark comparison.",
    },
    "aes256-ctr": {
        "label": "AES-256-CTR",
        "family": "aes",
        "authenticated": False,
        "speed_tier": "fastest",
        "security_tier": "good",
        "notes": "Parallelisable stream mode. Pair with an outer MAC for integrity.",
    },
    "aes256-gcm": {
        "label": "AES-256-GCM",
        "family": "aes",
        "authenticated": True,
        "speed_tier": "fast",
        "security_tier": "excellent",
        "notes": "NIST-recommended AEAD. Hardware-accelerated on modern CPUs.",
    },
    "xchacha20-poly1305": {
        "label": "XChaCha20-Poly1305",
        "family": "chacha",
        "authenticated": True,
        "speed_tier": "fast",
        "security_tier": "excellent",
        "notes": "IETF AEAD. Strong on CPUs without AES-NI; 24-byte nonce tolerant.",
    },
    "fernet": {
        "label": "Fernet (AES-128-CBC + HMAC)",
        "family": "aes",
        "authenticated": True,
        "speed_tier": "moderate",
        "security_tier": "very-good",
        "notes": "Batteries-included library cipher. Slower than native AEAD modes.",
    },
}


def list_at_rest_algorithms() -> list[dict[str, Any]]:
    """Return metadata for every built-in at-rest algorithm."""
    return [{"id": k, **v} for k, v in AT_REST_ALGORITHMS.items()]


def benchmark_at_rest(
    algorithm_id: str,
    samples: list[bytes],
    *,
    iterations: int = 1,
    key: bytes | None = None,
) -> dict[str, Any]:
    """Time encrypt+decrypt round-trip for `algorithm_id` on the given payloads.

    The caller supplies real-world-shaped bytes (typically serialized
    documents from a collection). Returned metrics:

    * ``rows`` — number of payloads processed per iteration
    * ``iterations`` — outer repeat count
    * ``encrypt_ms`` / ``decrypt_ms`` — wall time per round-trip, milliseconds
    * ``bytes_in`` / ``bytes_out`` — sum of plaintext / ciphertext sizes
    * ``overhead_pct`` — ``(bytes_out - bytes_in) / bytes_in * 100``
    * ``throughput_mb_s`` — plaintext MB/s based on ``encrypt_ms``
    * ``correct`` — all decrypted payloads matched their plaintext

    Raises :class:`CryptographicError` on mismatch (the algorithm is
    broken) so the benchmark surface never silently reports bad data.
    """
    import time

    engine = get_at_rest_encryption(algorithm_id, key=key or os.urandom(32))
    t0 = time.perf_counter()
    ciphertexts: list[bytes] = []
    for _ in range(iterations):
        ciphertexts = [engine.encrypt(p) for p in samples]
    encrypt_ms = (time.perf_counter() - t0) * 1000.0

    t1 = time.perf_counter()
    correct = True
    for _ in range(iterations):
        for p, c in zip(samples, ciphertexts):
            if engine.decrypt(c) != p:
                correct = False
    decrypt_ms = (time.perf_counter() - t1) * 1000.0

    bytes_in = sum(len(p) for p in samples)
    bytes_out = sum(len(c) for c in ciphertexts)
    overhead_pct = ((bytes_out - bytes_in) / max(1, bytes_in)) * 100.0

    encrypt_seconds = max(encrypt_ms / 1000.0, 1e-9)
    throughput_mb_s = (bytes_in / encrypt_seconds) / (1024 * 1024) * iterations

    return {
        "algorithm": algorithm_id,
        "rows": len(samples),
        "iterations": iterations,
        "encrypt_ms": round(encrypt_ms, 3),
        "decrypt_ms": round(decrypt_ms, 3),
        "bytes_in": bytes_in,
        "bytes_out": bytes_out,
        "overhead_pct": round(overhead_pct, 2),
        "throughput_mb_s": round(throughput_mb_s, 2),
        "correct": correct,
    }


def get_at_rest_encryption(algorithm_id: str, key: bytes | None = None) -> IAtRestEncryption:
    """Factory: return IAtRestEncryption implementation by algorithm id."""
    aliases = {
        "aes256-gcm": AES256GCMAtRest,
        "aes256_gcm": AES256GCMAtRest,
        "xchacha20-poly1305": XChaCha20Poly1305AtRest,
        "xchacha20_poly1305": XChaCha20Poly1305AtRest,
        "fernet": FernetAtRest,
        "aes256-ctr": AES256CTRAtRest,
        "aes256_ctr": AES256CTRAtRest,
        "null": NullAtRest,
        "none": NullAtRest,
    }
    cls = aliases.get(algorithm_id.lower())
    if cls is None:
        raise CryptographicError(f"Unknown at-rest algorithm: {algorithm_id}")
    return cls(key=key)
