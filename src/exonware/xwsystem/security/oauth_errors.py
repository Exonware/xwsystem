#!/usr/bin/env python3
"""
Shared OAuth-style error payload helpers.

These helpers are pure (no FastAPI/Starlette dependency) so they can be
reused across libraries while keeping transport concerns at the caller side.
"""

from __future__ import annotations

from typing import Any


def oauth_error_body(
    error: str,
    error_description: str,
    *,
    state: str | None = None,
    **extra: Any,
) -> dict[str, Any]:
    """Build OAuth-style error body payload."""
    body: dict[str, Any] = {
        "error": str(error or "invalid_request"),
        "error_description": str(error_description or "invalid request"),
    }
    if state is not None:
        body["state"] = state
    for key, value in extra.items():
        if value is not None:
            body[str(key)] = value
    return body


def oauth_error_status(error: str) -> int:
    """Map OAuth error code to HTTP status."""
    err = str(error or "invalid_request")
    if err in ("invalid_client", "unauthorized_client", "invalid_consumer"):
        return 401
    if err in ("invalid_grant", "invalid_token", "unsupported_grant_type"):
        return 400
    if err == "access_denied":
        return 403
    if err == "server_error":
        return 500
    if err in ("temporarily_unavailable", "service_unavailable"):
        return 503
    if err in ("invalid_request", "unsupported_response_type"):
        return 400
    return 400


def oauth_error_response(
    error: str,
    error_description: str,
    *,
    state: str | None = None,
    status_code: int | None = None,
    **extra: Any,
) -> tuple[dict[str, Any], int]:
    """Build full (body, status) response tuple."""
    body = oauth_error_body(error, error_description, state=state, **extra)
    status = int(status_code) if status_code is not None else oauth_error_status(error)
    return body, status


def oauth_error_to_http_parts(exc: Exception) -> tuple[dict[str, Any], int]:
    """
    Convert an OAuth-like exception object into (body, status) tuple.

    Supports duck-typed exception attributes:
    - error_code
    - error_description
    - state
    """
    error_code = getattr(exc, "error_code", None) or "invalid_request"
    error_description = getattr(exc, "error_description", None) or str(exc)
    state = getattr(exc, "state", None)
    return oauth_error_response(
        str(error_code),
        str(error_description),
        state=str(state) if state is not None else None,
    )
