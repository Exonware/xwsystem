#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/_tools/compliance_check.py

Compliance checker for xwsystem against the eXonware guide suite.

Goal: fail fast if `xwsystem/` drifts from required structure/docs/evidence.
"""

from __future__ import annotations

import sys
from dataclasses import dataclass
from pathlib import Path


ALLOWED_ROOT_ENTRIES = {
    ".github",
    ".gitignore",
    "LICENSE",
    "MANIFEST.in",
    "pyproject.toml",
    "pyproject.xwsystem.toml",
    "pytest.ini",
    "README.md",
    "requirements.txt",
    "docs",
    "examples",
    "src",
    "tests",
    "rust",
}

REQUIRED_DOCS = [
    "docs/REF_IDEA.md",
    "docs/REF_PROJECT.md",
    "docs/REF_PLAN.md",
    "docs/REF_ARCH.md",
    "docs/REF_DX.md",
    "docs/REF_API.md",
    "docs/REF_QA.md",
    "docs/REF_BENCH.md",
    "docs/GUIDE_USAGE.md",
    "docs/REPORT_TEST.md",
]

REQUIRED_LOGS = [
    "docs/logs/tests/INDEX.md",
    "docs/logs/benchmarks/INDEX.md",
    "docs/logs/releases/INDEX.md",
    "docs/logs/feedback/INDEX.md",
    "docs/logs/project/INDEX.md",
    "docs/logs/SUMMARY_CHANGE.md",
    "docs/logs/SUMMARY_PROJECT.md",
]

REQUIRED_COMPLIANCE = [
    "docs/compliance/traceability/TRACE_MATRIX.md",
    "docs/compliance/traceability/TRACE_MATRIX.json",
]


@dataclass(frozen=True)
class Failure:
    code: str
    message: str


def _repo_root() -> Path:
    # this file: xwsystem/tests/_tools/compliance_check.py
    return Path(__file__).resolve().parents[2]


def _check_root_structure(root: Path) -> list[Failure]:
    failures: list[Failure] = []
    for entry in root.iterdir():
        name = entry.name
        if name == ".git":
            # Git metadata (may exist if the package is a standalone repo or submodule).
            continue
        if name.startswith(".") and name in {".pytest_cache", ".mypy_cache"}:
            failures.append(Failure("root.forbidden_transient", f"Forbidden transient folder at root: {name}"))
            continue
        if name.startswith(".") and name in {".vscode", ".idea"}:
            # explicitly allowed (typically gitignored)
            continue
        if name not in ALLOWED_ROOT_ENTRIES:
            failures.append(Failure("root.forbidden_entry", f"Forbidden root entry: {name}"))
    return failures


def _check_required_files(root: Path, rel_paths: list[str], code: str) -> list[Failure]:
    failures: list[Failure] = []
    for rel in rel_paths:
        if not (root / rel).exists():
            failures.append(Failure(code, f"Missing required file: {rel}"))
    return failures


def _check_traceability_regenerates_clean(root: Path) -> list[Failure]:
    """
    Enforce that the traceability artefacts are not stale:
    regenerate them and fail if the contents differ.
    """
    failures: list[Failure] = []
    tools_root = root.parent  # monorepo root
    generator = tools_root / "tools" / "ci" / "python_scripts" / "generate_trace_matrix.py"
    if not generator.exists():
        return [Failure("traceability.missing_generator", f"Missing generator script: {generator.as_posix()}")]

    # Regenerate in-place (expected to be deterministic).
    import subprocess

    result = subprocess.run(
        [sys.executable, str(generator), "--package", "xwsystem"],
        cwd=str(tools_root),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    if result.returncode != 0:
        failures.append(Failure("traceability.generate_failed", f"Traceability generation failed:\n{result.stderr}"))
        return failures

    # Check that regeneration did not introduce unstaged changes in the artefacts.
    diff = subprocess.run(
        ["git", "diff", "--name-only", "xwsystem/docs/compliance/traceability/TRACE_MATRIX.md", "xwsystem/docs/compliance/traceability/TRACE_MATRIX.json"],
        cwd=str(tools_root),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    if diff.stdout.strip():
        failures.append(Failure("traceability.stale", "Traceability matrix is stale. Run the generator and commit the updated TRACE_MATRIX.* files."))
    return failures


def _check_file_headers(root: Path) -> list[Failure]:
    """
    Enforce: every source file starts with a `#exonware/...` (or `// #exonware/...`) path marker.
    """
    failures: list[Failure] = []
    targets = [
        root / "src",
        root / "tests",
        root / "examples",
        root / "rust",
    ]
    for base in targets:
        if not base.exists():
            continue
        for path in base.rglob("*"):
            if not path.is_file():
                continue
            if path.suffix not in {".py", ".rs"}:
                continue
            try:
                head = path.read_text(encoding="utf-8", errors="replace").splitlines()[:5]
            except OSError as exc:
                failures.append(Failure("headers.read_failed", f"Failed to read {path.as_posix()}: {exc}"))
                continue
            if path.suffix == ".py":
                if not any("#exonware/" in ln for ln in head):
                    failures.append(Failure("headers.missing", f"Missing #exonware path header in: {path.as_posix()}"))
            else:
                if not any("// #exonware/" in ln for ln in head):
                    failures.append(Failure("headers.missing", f"Missing // #exonware path header in: {path.as_posix()}"))
    return failures


def main() -> int:
    root = _repo_root()
    failures: list[Failure] = []

    failures.extend(_check_root_structure(root))
    failures.extend(_check_required_files(root, REQUIRED_DOCS, "docs.missing"))
    failures.extend(_check_required_files(root, REQUIRED_LOGS, "logs.missing"))
    failures.extend(_check_required_files(root, REQUIRED_COMPLIANCE, "compliance.missing"))
    failures.extend(_check_file_headers(root))
    failures.extend(_check_traceability_regenerates_clean(root))

    if failures:
        print("xwsystem compliance check FAILED")
        for f in failures:
            print(f"- [{f.code}] {f.message}")
        return 1

    print("xwsystem compliance check PASSED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

