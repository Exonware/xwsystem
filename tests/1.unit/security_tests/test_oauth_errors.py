#!/usr/bin/env python3
"""
Unit tests for shared OAuth error helpers.
"""

from __future__ import annotations

import pytest

from exonware.xwsystem.security.oauth_errors import (
    oauth_error_body,
    oauth_error_status,
    oauth_error_response,
    oauth_error_to_http_parts,
)


@pytest.mark.xwsystem_unit
def test_oauth_error_body_and_status_mapping() -> None:
    body = oauth_error_body("invalid_client", "bad client", state="s1")
    assert body["error"] == "invalid_client"
    assert body["error_description"] == "bad client"
    assert body["state"] == "s1"
    assert oauth_error_status("invalid_client") == 401
    assert oauth_error_status("access_denied") == 403
    assert oauth_error_status("invalid_request") == 400


@pytest.mark.xwsystem_unit
def test_oauth_error_response_with_status_override() -> None:
    body, status = oauth_error_response(
        "unauthorized",
        "auth required",
        status_code=401,
    )
    assert status == 401
    assert body["error"] == "unauthorized"
    assert body["error_description"] == "auth required"


class _ExcWithOAuthFields(Exception):
    def __init__(self) -> None:
        super().__init__("fallback")
        self.error_code = "invalid_grant"
        self.error_description = "Grant is invalid"
        self.state = "abc123"


@pytest.mark.xwsystem_unit
def test_oauth_error_to_http_parts_from_exception() -> None:
    body, status = oauth_error_to_http_parts(_ExcWithOAuthFields())
    assert status == 400
    assert body["error"] == "invalid_grant"
    assert body["error_description"] == "Grant is invalid"
    assert body["state"] == "abc123"
