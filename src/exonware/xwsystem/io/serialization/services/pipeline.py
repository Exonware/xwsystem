#exonware/xwsystem/src/exonware/xwsystem/io/serialization/services/pipeline.py
"""
Serialization pipeline: apply encryption, archive, binary services to bytes.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Save: encode -> binary (optional) -> encrypt (optional) -> archive (optional) -> write
Load: read -> decompress (optional) -> decrypt (optional) -> binary unwrap (optional) -> decode
"""

from __future__ import annotations
from typing import Any
from .encryption import EncryptionService
from .archive import ArchiveService
from .binary import BinaryService


def apply_pipeline_save(
    repr_data: bytes,
    options: dict[str, Any] | None = None,
) -> bytes:
    """
    Apply pipeline for save: binary_framing -> encryption -> archive.
    Options can contain: encryption (dict with algorithm, password/key), archive (str format), binary_framing (bool).
    """
    options = options or {}
    data = repr_data
    # 1. Binary framing (optional)
    binary_framing = options.get("binary_framing", False)
    if binary_framing:
        svc = BinaryService()
        data = svc.wrap(data, use_length_prefix=True)
    # 2. Encryption (optional)
    encryption = options.get("encryption")
    if encryption is not None and isinstance(encryption, dict):
        enc_svc = EncryptionService()
        key = encryption.get("key")
        password = encryption.get("password")
        algorithm = encryption.get("algorithm", "aes256-gcm")
        if key is not None or password is not None:
            data = enc_svc.encrypt(data, key=key, password=password, algorithm=algorithm)
    # 3. Archive (optional)
    archive = options.get("archive")
    if archive is not None and isinstance(archive, str):
        arch_svc = ArchiveService()
        data = arch_svc.compress(data, format=archive)
    return data


def apply_pipeline_load(
    raw_bytes: bytes,
    options: dict[str, Any] | None = None,
) -> bytes:
    """
    Apply pipeline for load: decompress -> decrypt -> binary unwrap.
    Options can contain: encryption (dict with password/key), archive (str or True for auto), binary_framing (bool).
    """
    options = options or {}
    data = raw_bytes
    # 1. Decompress (optional) - try auto-detect or use options["archive"]
    archive = options.get("archive")
    if archive is not None:
        arch_svc = ArchiveService()
        if archive is True:
            # Auto-detect from magic bytes
            try:
                data = arch_svc.decompress(data, format=None)
            except ValueError:
                pass  # Not compressed
        elif isinstance(archive, str):
            data = arch_svc.decompress(data, format=archive)
    # 2. Decrypt (optional) - if XWJE magic and encryption credentials provided
    encryption = options.get("encryption")
    if encryption is not None and isinstance(encryption, dict):
        enc_svc = EncryptionService()
        if enc_svc.is_encrypted(data):
            key = encryption.get("key")
            password = encryption.get("password")
            if key is not None or password is not None:
                data = enc_svc.decrypt(data, key=key, password=password)
    # 3. Binary unwrap (optional)
    binary_framing = options.get("binary_framing", False)
    if binary_framing:
        svc = BinaryService()
        data = svc.unwrap(data, use_length_prefix=True)
    return data
