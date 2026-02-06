"""
#exonware/xwsystem/tests/compliance/test_traceability_matrix.py

Compliance: traceability artefacts must exist and be well-formed.

WHY: GUIDE_11_COMP.md requires traceability from requirements → plans → verification evidence.
"""

from __future__ import annotations

from pathlib import Path

import pytest


@pytest.mark.xwsystem_compliance
def test_traceability_matrix_files_exist() -> None:
    repo_root = Path(__file__).resolve().parents[2]
    trace_dir = repo_root / "docs" / "compliance" / "traceability"

    md = trace_dir / "TRACE_MATRIX.md"
    js = trace_dir / "TRACE_MATRIX.json"

    assert md.exists(), "Missing TRACE_MATRIX.md (generate via tools/ci/python_scripts/generate_trace_matrix.py)"
    assert js.exists(), "Missing TRACE_MATRIX.json (generate via tools/ci/python_scripts/generate_trace_matrix.py)"


@pytest.mark.xwsystem_compliance
def test_traceability_matrix_markdown_has_table_header() -> None:
    repo_root = Path(__file__).resolve().parents[2]
    md = repo_root / "docs" / "compliance" / "traceability" / "TRACE_MATRIX.md"
    content = md.read_text(encoding="utf-8", errors="replace")

    assert "# Requirements Traceability Matrix" in content
    assert "| Requirement ID | Plan | Components | Verification Evidence | Notes |" in content

