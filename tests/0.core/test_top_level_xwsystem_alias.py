"""Top-level ``xwsystem`` must alias ``exonware.xwsystem`` (same module object)."""

from __future__ import annotations

import sys
from pathlib import Path


def _ensure_pkg_src_on_path() -> None:
    root = Path(__file__).resolve().parents[2]
    src = root / "src"
    if src.is_dir() and str(src) not in sys.path:
        sys.path.insert(0, str(src))


def test_xwsystem_top_level_module_is_exonware_xwsystem() -> None:
    _ensure_pkg_src_on_path()
    import xwsystem as tl
    import exonware.xwsystem as ns

    assert tl is ns
