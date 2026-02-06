"""
#exonware/xwsystem/tests/_tools/normalize_file_headers.py

Normalize file header path comments across the xwsystem project.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Last Updated: 29-Jan-2026

This script enforces the rule from `docs/guides/GUIDE_41_DOCS.md`:
- Every source file MUST start with a file path comment like: `#exonware/...`

For Python files:
- If the file starts with a shebang, we insert the path comment on line 2.
- Otherwise, we insert the path comment on line 1.
- If the path comment already exists within the first 5 lines, we leave the file unchanged.

For Rust files:
- Similar rule, but uses `// #exonware/...`.

This script is designed to be safe:
- It only inserts comments (it does not rewrite existing docstrings or code).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


REPO_ROOT_MARKER = "xwsystem"
MAX_SCAN_LINES = 5


@dataclass(frozen=True)
class Stats:
    scanned: int = 0
    changed: int = 0
    skipped: int = 0


def _rel_exonware_path(file_path: Path) -> str:
    # We want a stable repo-relative marker line, not an absolute machine path.
    # Example: #exonware/xwsystem/src/exonware/xwsystem/module.py
    parts = list(file_path.as_posix().split("/"))
    if REPO_ROOT_MARKER not in parts:
        # Best-effort fallback
        return f"#exonware/{file_path.name}"
    idx = parts.index(REPO_ROOT_MARKER)
    rel = "/".join(parts[idx:])
    return f"#exonware/{rel}"


def _has_marker_near_top(lines: list[str], marker: str) -> bool:
    head = lines[:MAX_SCAN_LINES]
    return any(marker in ln for ln in head)


def _normalize_python(file_path: Path) -> bool:
    raw = file_path.read_text(encoding="utf-8", errors="replace")
    lines = raw.splitlines(keepends=True)

    marker = _rel_exonware_path(file_path)
    if _has_marker_near_top(lines, marker):
        return False

    insert_at = 0
    if lines and lines[0].startswith("#!"):
        insert_at = 1

    # Always include a trailing newline on the inserted comment.
    new_line = marker + "\n"
    lines.insert(insert_at, new_line)
    file_path.write_text("".join(lines), encoding="utf-8", newline="")
    return True


def _normalize_rust(file_path: Path) -> bool:
    raw = file_path.read_text(encoding="utf-8", errors="replace")
    lines = raw.splitlines(keepends=True)

    marker = _rel_exonware_path(file_path).replace("#exonware/", "// #exonware/")
    if _has_marker_near_top(lines, marker):
        return False

    insert_at = 0
    new_line = marker + "\n"
    lines.insert(insert_at, new_line)
    file_path.write_text("".join(lines), encoding="utf-8", newline="")
    return True


def main() -> int:
    root = Path(__file__).resolve()
    # find `.../xwsystem/` folder
    while root.name != REPO_ROOT_MARKER and root.parent != root:
        root = root.parent
    if root.name != REPO_ROOT_MARKER:
        raise RuntimeError("Could not locate xwsystem/ root directory from script location.")

    targets = [
        root / "src",
        root / "tests",
        root / "examples",
        root / "rust",
    ]

    stats = Stats()
    changed = 0
    scanned = 0

    for base in targets:
        if not base.exists():
            continue

        for file_path in base.rglob("*"):
            if not file_path.is_file():
                continue
            if file_path.suffix == ".py":
                scanned += 1
                if _normalize_python(file_path):
                    changed += 1
            elif file_path.suffix == ".rs":
                scanned += 1
                if _normalize_rust(file_path):
                    changed += 1

    stats = Stats(scanned=scanned, changed=changed, skipped=scanned - changed)
    print(f"scanned={stats.scanned} changed={stats.changed} skipped={stats.skipped}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

