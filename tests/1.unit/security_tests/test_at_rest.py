#!/usr/bin/env python3
"""Unit tests for at-rest encryption (IAtRestEncryption implementations)."""

import pytest
from exonware.xwsystem.security.at_rest import (
    AES256GCMAtRest,
    XChaCha20Poly1305AtRest,
    FernetAtRest,
    get_at_rest_encryption,
    is_envelope,
    parse_envelope,
    build_envelope,
    XWJE_MAGIC,
)
from exonware.xwsystem.security.errors import CryptographicError
@pytest.mark.xwsystem_unit


class TestAtRestEncryption:
    """Round-trip and interface tests for at-rest encryption."""
    @pytest.fixture

    def key(self):
        return b"0" * 32
    @pytest.mark.parametrize("cls", [AES256GCMAtRest, XChaCha20Poly1305AtRest, FernetAtRest])

    def test_round_trip_with_key(self, cls, key):
        enc = cls(key=key)
        plain = b"hello world"
        payload = enc.encrypt(plain, key=key)
        assert is_envelope(payload)
        assert payload[:4] == XWJE_MAGIC
        dec = enc.decrypt(payload, key=key)
        assert dec == plain
    @pytest.mark.parametrize("cls", [AES256GCMAtRest, XChaCha20Poly1305AtRest, FernetAtRest])

    def test_round_trip_with_password(self, cls, key):
        enc = cls()  # no key
        plain = b"secret data"
        payload = enc.encrypt(plain, password="test-pass")
        assert is_envelope(payload)
        dec = enc.decrypt(payload, password="test-pass")
        assert dec == plain

    def test_algorithm_ids(self):
        key = b"1" * 32
        assert AES256GCMAtRest(key=key).algorithm_id() == "aes256-gcm"
        assert XChaCha20Poly1305AtRest(key=key).algorithm_id() == "xchacha20-poly1305"
        assert FernetAtRest(key=key).algorithm_id() == "fernet"

    def test_supports_password(self):
        assert AES256GCMAtRest().supports_password() is True
        assert FernetAtRest().supports_password() is True

    def test_get_at_rest_encryption(self, key):
        e = get_at_rest_encryption("aes256-gcm", key=key)
        assert e.algorithm_id() == "aes256-gcm"
        assert e.decrypt(e.encrypt(b"x", key=key), key=key) == b"x"
        e2 = get_at_rest_encryption("fernet", key=key)
        assert e2.algorithm_id() == "fernet"
        with pytest.raises(CryptographicError, match="Unknown"):
            get_at_rest_encryption("unknown", key=key)

    def test_wrong_key_fails(self, key):
        enc = AES256GCMAtRest(key=key)
        payload = enc.encrypt(b"data", key=key)
        other_key = b"1" * 32
        with pytest.raises((CryptographicError, Exception)):
            enc.decrypt(payload, key=other_key)

    def test_wrong_password_fails(self):
        enc = AES256GCMAtRest()
        payload = enc.encrypt(b"data", password="right")
        with pytest.raises((CryptographicError, Exception)):
            enc.decrypt(payload, password="wrong")
