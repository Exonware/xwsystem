#!/usr/bin/env python3
"""Unit tests for B2B / multi-tenant tenancy helpers."""

from __future__ import annotations

import pytest

from exonware.xwsystem.security.tenancy import (
    build_tenancy_context,
    effective_isolation_key,
    extract_path_org_id,
    is_instance_operator_introspection,
    tenancy_violation_for_path_org,
)

_ORG_PREFIX = "/v1/organizations"


@pytest.mark.xwsystem_unit
def test_extract_path_org_id_segments() -> None:
    pfx = _ORG_PREFIX
    assert extract_path_org_id(f"{pfx}/org-1", pfx) == "org-1"
    assert extract_path_org_id(f"{pfx}/org-1/members", pfx) == "org-1"
    assert extract_path_org_id(f"{pfx}/org-1/members/u1/role", pfx) == "org-1"
    assert extract_path_org_id(f"{pfx}", pfx) is None
    assert extract_path_org_id(f"{pfx}/", pfx) is None
    assert extract_path_org_id("/other/v1/orgs/x", pfx) is None


@pytest.mark.xwsystem_unit
def test_tenancy_violation_org_bound_token() -> None:
    ir = {"active": True, "org_id": "a", "sub": "u1"}
    assert tenancy_violation_for_path_org(ir, "a") is None
    assert tenancy_violation_for_path_org(ir, "b") == "tenant_context_mismatch"
    assert tenancy_violation_for_path_org(ir, None) is None
    assert tenancy_violation_for_path_org({"active": True, "sub": "u1"}, "a") is None


@pytest.mark.xwsystem_unit
def test_instance_operator_bypass_mismatch() -> None:
    ir = {"active": True, "org_id": "a", "roles": ["instance_admin"], "sub": "op"}
    assert tenancy_violation_for_path_org(ir, "b") is None


@pytest.mark.xwsystem_unit
def test_is_instance_operator_introspection() -> None:
    assert is_instance_operator_introspection({"roles": ["instance_admin"]}) is True
    assert is_instance_operator_introspection({"scope": "admin read"}) is True
    assert is_instance_operator_introspection({"scopes": ["admin"]}) is True
    assert is_instance_operator_introspection({"roles": ["member"]}) is False


@pytest.mark.xwsystem_unit
def test_effective_isolation_key_precedence() -> None:
    assert (
        effective_isolation_key(
            path_org_id="p1",
            header_tenant_id="t1",
            header_org_id="o1",
            token_org_id="to1",
            token_tenant_id="tt1",
        )
        == "p1"
    )
    assert (
        effective_isolation_key(
            path_org_id=None,
            header_tenant_id="t1",
            header_org_id=None,
            token_org_id="to1",
            token_tenant_id=None,
        )
        == "to1"
    )
    assert effective_isolation_key(
        path_org_id=None,
        header_tenant_id=None,
        header_org_id=None,
        token_org_id=None,
        token_tenant_id=None,
    ) == "public"


@pytest.mark.xwsystem_unit
def test_build_tenancy_context_from_path_and_introspection() -> None:
    path = f"{_ORG_PREFIX}/acme/members"
    ir = {"active": True, "tenant_id": "tid", "org_id": "acme", "project_id": "proj1"}
    ctx = build_tenancy_context(
        path=path,
        organizations_prefix=_ORG_PREFIX,
        header_tenant_id=None,
        header_org_id=None,
        introspection=ir,
    )
    assert ctx.path_org_id == "acme"
    assert ctx.org_id_from_token == "acme"
    assert ctx.project_id_from_token == "proj1"
    assert ctx.tenant_id == "tid"
