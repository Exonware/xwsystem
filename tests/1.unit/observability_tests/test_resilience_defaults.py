"""Tests for observability resilience defaults."""

from __future__ import annotations

import pytest

from exonware.xwsystem.observability import outbound_http_timeout_tuple


def test_outbound_http_timeout_tuple_defaults(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.delenv("XWAUTH_OUTBOUND_HTTP_CONNECT_TIMEOUT", raising=False)
    monkeypatch.delenv("XWAUTH_OUTBOUND_HTTP_READ_TIMEOUT", raising=False)
    c, r = outbound_http_timeout_tuple()
    assert c == 5.0
    assert r == 30.0


def test_outbound_http_timeout_tuple_env_override(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("XWAUTH_OUTBOUND_HTTP_CONNECT_TIMEOUT", "2.5")
    monkeypatch.setenv("XWAUTH_OUTBOUND_HTTP_READ_TIMEOUT", "12")
    c, r = outbound_http_timeout_tuple()
    assert c == 2.5
    assert r == 12.0


def test_outbound_http_timeout_tuple_custom_defaults() -> None:
    c, r = outbound_http_timeout_tuple(default_connect=1.0, default_read=2.0)
    assert c == 1.0
    assert r == 2.0


def test_outbound_http_timeout_tuple_clamps_minimum() -> None:
    c, r = outbound_http_timeout_tuple(default_connect=0.0, default_read=0.0)
    assert c >= 0.1
    assert r >= 0.1
