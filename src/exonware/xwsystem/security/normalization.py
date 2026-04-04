#!/usr/bin/env python3
"""
Shared principal/context normalization helpers.

These helpers keep cross-library auth context handling consistent between
xwauth, xwapi, and xwstorage.
"""

from __future__ import annotations

from typing import Any

from collections.abc import Iterable, Mapping
from types import SimpleNamespace

from .contracts import PolicyContext
from .tenancy import org_id_from_claims_mapping, project_id_from_claims_mapping

def claims_from_principal(principal: Any) -> dict[str, Any]:
    """Return normalized claims map from dict/object principal."""
    if isinstance(principal, dict):
        return dict(principal.get("claims") or principal)
    claims = getattr(principal, "claims", None)
    if isinstance(claims, dict):
        return dict(claims)
    return {}

def subject_id_from_principal(principal: Any) -> str | None:
    """Extract normalized subject/user id from principal object."""
    direct = getattr(principal, "subject_id", None)
    if direct:
        return str(direct)
    claims = claims_from_principal(principal)
    subject = claims.get("subject_id") or claims.get("sub") or claims.get("user_id")
    return str(subject) if subject else None

def tenant_id_from_principal(principal: Any) -> str | None:
    """Extract normalized tenant id from principal object."""
    direct = getattr(principal, "tenant_id", None)
    if direct:
        return str(direct)
    claims = claims_from_principal(principal)
    tenant = claims.get("tenant_id") or claims.get("tid")
    return str(tenant) if tenant else None

def tenant_id_from_claims_mapping(claims: Mapping[str, Any] | None) -> str | None:
    """Extract tenant id from claims map using tenant_id then tid precedence."""
    if not isinstance(claims, Mapping):
        return None
    tenant = claims.get("tenant_id") or claims.get("tid")
    return str(tenant) if tenant else None

def roles_from_principal(principal: Any) -> list[str]:
    """Extract normalized role list from principal object."""
    direct = getattr(principal, "roles", None)
    if isinstance(direct, list):
        return [str(role) for role in direct if role]
    claims = claims_from_principal(principal)
    claim_roles = claims.get("roles") or []
    if isinstance(claim_roles, list):
        return [str(role) for role in claim_roles if role]
    return []

def org_id_from_principal(principal: Any) -> str | None:
    """Extract org id from principal claims (org_id, organization_id, org)."""
    direct = getattr(principal, "org_id", None)
    if direct:
        return str(direct)
    claims = claims_from_principal(principal)
    return org_id_from_claims_mapping(claims)


def project_id_from_principal(principal: Any) -> str | None:
    """Extract project / application id from principal claims."""
    direct = getattr(principal, "project_id", None)
    if direct:
        return str(direct)
    claims = claims_from_principal(principal)
    return project_id_from_claims_mapping(claims)


def policy_context_from_principal(principal: Any) -> PolicyContext:
    """Build shared PolicyContext from a resolved principal/auth context."""
    return PolicyContext(
        tenant_id=tenant_id_from_principal(principal),
        org_id=org_id_from_principal(principal),
        project_id=project_id_from_principal(principal),
        user_id=subject_id_from_principal(principal),
        roles=roles_from_principal(principal),
    )

def policy_context_to_dict(context: PolicyContext) -> dict[str, Any]:
    """Convert PolicyContext into stable JSON-serializable mapping."""
    return {
        "tenant_id": context.tenant_id,
        "org_id": context.org_id,
        "project_id": context.project_id,
        "user_id": context.user_id,
        "roles": list(context.roles or []),
    }

def infer_scope_set_from_roles(roles: Iterable[str] | None) -> set[str]:
    """Infer scope set from role values using shared legacy convention."""
    normalized = {str(role).strip() for role in (roles or []) if str(role).strip()}
    scopes = {role for role in normalized if ":" in role}
    if "admin" in normalized:
        scopes.add("*")
    return scopes

def scopes_from_policy_context(context: PolicyContext) -> set[str]:
    """Infer policy scopes from PolicyContext roles."""
    return infer_scope_set_from_roles(context.roles)

def default_tenant_row_visible(row: Any, context: PolicyContext) -> bool:
    """Default tenant-level row visibility predicate."""
    if context.tenant_id is None:
        return True
    if not isinstance(row, dict):
        return True
    row_tenant = row.get("tenant_id", row.get("tenantId", row.get("tid")))
    return row_tenant == context.tenant_id

def default_owner_row_visible(row: Any, context: PolicyContext) -> bool:
    """Default owner/role row visibility predicate."""
    if "admin" in context.roles:
        return True
    if context.user_id is None or not isinstance(row, dict):
        return False if context.user_id is None else True
    owner = row.get("owner_id", row.get("ownerId", row.get("user_id", row.get("userId"))))
    return owner == context.user_id

def resolve_tenant_id_layered(
    *,
    principal: Any | None = None,
    request_claims: Mapping[str, Any] | None = None,
    header_tenant_id: str | None = None,
    query_tenant_id: str | None = None,
) -> str | None:
    """
    Resolve tenant id from layered request sources.

    Precedence:
    1) principal tenant resolution
    2) request claims mapping
    3) X-Tenant-Id header
    4) tenant_id query parameter
    """
    tenant = tenant_id_from_principal(principal) if principal is not None else None
    if not tenant:
        tenant = tenant_id_from_claims_mapping(request_claims)
    if not tenant and header_tenant_id:
        tenant = str(header_tenant_id)
    if not tenant and query_tenant_id:
        tenant = str(query_tenant_id)
    return tenant or None

def auth_context_compat_from_policy_context(
    context: PolicyContext,
    *,
    scopes: list[str] | None = None,
) -> Any:
    """
    Build a minimal auth-context compatible object from PolicyContext.
    Useful for delegating policy checks without importing domain-layer DTOs.
    """
    return SimpleNamespace(
        subject_id=context.user_id or "",
        tenant_id=context.tenant_id,
        org_id=context.org_id,
        project_id=context.project_id,
        roles=list(context.roles or []),
        scopes=list(scopes or []),
        claims={},
    )
