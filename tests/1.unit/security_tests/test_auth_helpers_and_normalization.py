#!/usr/bin/env python3
"""Unit tests for shared auth helper and normalization utilities."""

from __future__ import annotations

import pytest

from exonware.xwsystem.security.auth_helpers import (
    parse_authorization_bearer,
    resolve_bearer_or_cookie_token,
    resolve_principal_from_auth_provider,
)
from exonware.xwsystem.security.contracts import PolicyContext
from exonware.xwsystem.security.normalization import (
    default_owner_row_visible,
    default_tenant_row_visible,
    infer_scope_set_from_roles,
    policy_context_to_dict,
    resolve_tenant_id_layered,
    tenant_id_from_claims_mapping,
)


class _Provider:
    async def resolve_auth_context(self, token: str):
        if token == "resolve-token":
            return {"sub": "u1"}
        return None

    async def verify_api_token(self, token: str):
        if token == "verify-token":
            return {"sub": "u2"}
        return None

    async def validate_token(self, token: str):
        if token == "legacy-token":
            return {"sub": "u3"}
        return None


@pytest.mark.xwsystem_unit
def test_parse_authorization_bearer_and_cookie_resolution() -> None:
    assert parse_authorization_bearer("Bearer abc") == "abc"
    assert parse_authorization_bearer("Basic abc") is None
    assert resolve_bearer_or_cookie_token(
        authorization=None,
        cookies={"session_token": "s-token"},
    ) == "s-token"


@pytest.mark.xwsystem_unit
@pytest.mark.asyncio
async def test_resolve_principal_from_auth_provider_fallback_order() -> None:
    provider = _Provider()
    resolved = await resolve_principal_from_auth_provider(provider, "resolve-token")
    verified = await resolve_principal_from_auth_provider(provider, "verify-token")
    legacy = await resolve_principal_from_auth_provider(provider, "legacy-token")
    assert resolved == {"sub": "u1"}
    assert verified == {"sub": "u2"}
    assert legacy == {"sub": "u3"}


@pytest.mark.xwsystem_unit
def test_policy_context_mapping_scope_and_row_visibility_helpers() -> None:
    ctx = PolicyContext(tenant_id="t1", user_id="u1", roles=["storage:read", "admin"])
    assert policy_context_to_dict(ctx) == {
        "tenant_id": "t1",
        "org_id": None,
        "project_id": None,
        "user_id": "u1",
        "roles": ["storage:read", "admin"],
    }
    assert infer_scope_set_from_roles(ctx.roles) == {"storage:read", "*"}
    assert default_tenant_row_visible({"tenant_id": "t1"}, ctx) is True
    assert default_owner_row_visible({"owner_id": "u1"}, ctx) is True


@pytest.mark.xwsystem_unit
def test_layered_tenant_resolution_precedence() -> None:
    class _Principal:
        tenant_id = "principal-tenant"

    assert tenant_id_from_claims_mapping({"tenant_id": "claims-tenant", "tid": "claims-tid"}) == "claims-tenant"
    assert tenant_id_from_claims_mapping({"tid": "claims-tid"}) == "claims-tid"

    resolved = resolve_tenant_id_layered(
        principal=_Principal(),
        request_claims={"tenant_id": "claims-tenant"},
        header_tenant_id="header-tenant",
        query_tenant_id="query-tenant",
    )
    assert resolved == "principal-tenant"

    resolved_no_principal = resolve_tenant_id_layered(
        principal=None,
        request_claims={"tid": "claims-tid"},
        header_tenant_id="header-tenant",
        query_tenant_id="query-tenant",
    )
    assert resolved_no_principal == "claims-tid"
