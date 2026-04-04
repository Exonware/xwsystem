#!/usr/bin/env python3
"""
Multi-tenant / B2B request context helpers.

Resolution order for HTTP APIs (when enforcing alignment with path-scoped org routes):
1) Immutable org id from access token claims (org_id) — authoritative when present
2) Path segment under /.../organizations/{org_id}/...
3) Trusted gateway headers (only when explicitly enabled via environment)

Patterns follow common B2B IdP practice: org-bound sessions reject cross-org URL access unless
a break-glass instance operator is recognized (roles/scopes).
"""

from __future__ import annotations

import os
from dataclasses import dataclass
from typing import Any, Mapping


def _truthy_env(name: str) -> bool:
    return os.environ.get(name, "").strip().lower() in {"1", "true", "yes", "on"}


@dataclass(frozen=True)
class TenancyContext:
    """Resolved tenancy slice for a single HTTP request (observability + guard rails)."""

    tenant_id: str | None
    org_id_from_token: str | None
    project_id_from_token: str | None
    path_org_id: str | None
    trusted_header_org_id: str | None
    trusted_header_tenant_id: str | None


def extract_path_org_id(path: str, organizations_prefix: str) -> str | None:
    """
    Return org id from /{prefix}/{org_id}/... or /{prefix}/{org_id} (no trailing slash required).
    Does not match GET /{prefix} list endpoint.
    """
    p = (path or "").split("?", 1)[0].rstrip("/")
    prefix = (organizations_prefix or "").rstrip("/")
    if not prefix or not p.startswith(prefix + "/"):
        return None
    rest = p[len(prefix) + 1 :]
    if not rest:
        return None
    segment = rest.split("/", 1)[0]
    return segment if segment else None


def org_id_from_claims_mapping(claims: Mapping[str, Any] | None) -> str | None:
    if not isinstance(claims, Mapping):
        return None
    for key in ("org_id", "organization_id", "org"):
        v = claims.get(key)
        if v is not None and str(v).strip():
            return str(v).strip()
    return None


def project_id_from_claims_mapping(claims: Mapping[str, Any] | None) -> str | None:
    if not isinstance(claims, Mapping):
        return None
    for key in ("project_id", "application_id", "app_id"):
        v = claims.get(key)
        if v is not None and str(v).strip():
            return str(v).strip()
    return None


def introspection_claims_org_project(introspection: Mapping[str, Any] | None) -> tuple[str | None, str | None]:
    """Read org/project from RFC 7662-style introspection dict or JWT payload-like map."""
    if not isinstance(introspection, Mapping):
        return None, None
    org = org_id_from_claims_mapping(introspection)
    proj = project_id_from_claims_mapping(introspection)
    return org, proj


def is_instance_operator_introspection(introspection: Mapping[str, Any] | None) -> bool:
    """
    Recognize platform break-glass principals (narrow; extend per product policy).

    Treats instance_admin role, admin role, or admin scope as operators that may bypass
    org claim vs path org mismatch (must still be audited at call sites).
    """
    if not isinstance(introspection, Mapping):
        return False
    roles = introspection.get("roles") or []
    if not isinstance(roles, list):
        roles = []
    normalized_roles = {str(r).strip() for r in roles if r is not None and str(r).strip()}
    if "instance_admin" in normalized_roles or "system_admin" in normalized_roles:
        return True
    scopes = introspection.get("scopes") or []
    if isinstance(scopes, str):
        scopes = [s for s in scopes.split() if s]
    if not isinstance(scopes, list):
        scopes = []
    if "admin" in {str(s).strip() for s in scopes if s}:
        return True
    scope_str = introspection.get("scope")
    if isinstance(scope_str, str) and "admin" in scope_str.split():
        return True
    if "admin" in normalized_roles:
        return True
    return False


def tenancy_violation_for_path_org(
    introspection: Mapping[str, Any] | None,
    path_org_id: str | None,
) -> str | None:
    """
    If the token carries org_id, it must equal path org for org-scoped routes unless operator.

    Returns None when allowed, or a stable machine-readable error code.
    """
    if not path_org_id or not str(path_org_id).strip():
        return None
    if not isinstance(introspection, Mapping) or not introspection.get("active", False):
        return None
    token_org, _ = introspection_claims_org_project(introspection)
    if not token_org:
        return None
    if token_org == path_org_id:
        return None
    if is_instance_operator_introspection(introspection):
        return None
    return "tenant_context_mismatch"


def effective_isolation_key(
    *,
    path_org_id: str | None,
    header_tenant_id: str | None,
    header_org_id: str | None,
    token_org_id: str | None,
    token_tenant_id: str | None,
) -> str:
    """
    Key for rate limits / noisy-neighbor controls: prefer immutable org id from path or token.
    """
    for candidate in (path_org_id, token_org_id, header_org_id, token_tenant_id, header_tenant_id):
        if candidate is not None and str(candidate).strip():
            return str(candidate).strip()
    return "public"


def build_tenancy_context(
    *,
    path: str,
    organizations_prefix: str,
    header_tenant_id: str | None,
    header_org_id: str | None,
    introspection: Mapping[str, Any] | None = None,
) -> TenancyContext:
    path_org = extract_path_org_id(path, organizations_prefix)
    token_org: str | None = None
    token_proj: str | None = None
    token_tid: str | None = None
    if isinstance(introspection, Mapping) and introspection.get("active", False):
        token_org, token_proj = introspection_claims_org_project(introspection)
        token_tid = introspection.get("tenant_id") or introspection.get("tid")
        if token_tid is not None:
            token_tid = str(token_tid).strip() or None

    trusted_org: str | None = None
    trusted_tid: str | None = None
    if _truthy_env("XWAUTH_TRUST_GATEWAY_TENANT_HEADERS"):
        if header_org_id and str(header_org_id).strip():
            trusted_org = str(header_org_id).strip()
        if header_tenant_id and str(header_tenant_id).strip():
            trusted_tid = str(header_tenant_id).strip()

    layered_tid = token_tid or trusted_tid or trusted_org

    return TenancyContext(
        tenant_id=layered_tid,
        org_id_from_token=token_org,
        project_id_from_token=token_proj,
        path_org_id=path_org,
        trusted_header_org_id=trusted_org,
        trusted_header_tenant_id=trusted_tid,
    )
