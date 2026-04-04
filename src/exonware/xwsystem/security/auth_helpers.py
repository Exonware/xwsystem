#!/usr/bin/env python3
"""
Shared auth helper utilities.

These helpers are framework-agnostic and are intended to reduce duplicated
token extraction and provider-resolution glue across xwapi/xwauth/xwstorage.
"""

from __future__ import annotations

from typing import Any

from collections.abc import Mapping, Sequence

def parse_authorization_bearer(authorization: str | None) -> str | None:
    """Extract bearer token from Authorization header value."""
    if not authorization:
        return None
    if not authorization.startswith("Bearer "):
        return None
    token = authorization.removeprefix("Bearer ").strip()
    return token or None

def resolve_bearer_or_cookie_token(
    *,
    authorization: str | None,
    cookies: Mapping[str, str] | None,
    cookie_keys: Sequence[str] = ("session_token", "access_token"),
) -> str | None:
    """Resolve request token from bearer header first, then cookie keys."""
    bearer = parse_authorization_bearer(authorization)
    if bearer:
        return bearer
    if not cookies:
        return None
    for key in cookie_keys:
        value = str(cookies.get(key) or "").strip()
        if value:
            return value
    return None

async def resolve_principal_from_auth_provider(
    auth_provider: Any,
    token: str,
    *,
    include_verify_api_token: bool = True,
    allow_legacy_validate_token: bool = True,
) -> Any:
    """
    Resolve principal from auth provider using backward-compatible method order.

    Resolution order:
    1) resolve_auth_context
    2) verify_api_token (optional)
    3) validate_token (optional legacy path)
    """
    resolver = getattr(auth_provider, "resolve_auth_context", None)
    if callable(resolver):
        principal = await resolver(token)
        if principal:
            return principal

    if include_verify_api_token:
        verifier = getattr(auth_provider, "verify_api_token", None)
        if callable(verifier):
            principal = await verifier(token)
            if principal:
                return principal

    if allow_legacy_validate_token:
        validator = getattr(auth_provider, "validate_token", None)
        if callable(validator):
            return await validator(token)

    return None
