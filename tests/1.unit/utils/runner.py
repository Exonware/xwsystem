#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/1.unit/utils/runner.py
Runner for xwsystem utility unit tests.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.387
Generation Date: 08-Nov-2025
"""

from __future__ import annotations

import os
import sys
from pathlib import Path


def _package_root() -> Path:
    """Folder with pyproject.toml + src/ (any tests/**/runner.py depth)."""
    p = Path(__file__).resolve().parent
    while p != p.parent:
        if (p / "pyproject.toml").is_file() and (p / "src").is_dir():
            return p
        p = p.parent
    raise RuntimeError("Could not locate package root from " + str(Path(__file__)))


_PKG_ROOT = _package_root()

from exonware.xwsystem.utils.test_runner import TestRunner


def main() -> int:
    """Execute utils-focused unit tests using the shared runner."""
    os.chdir(_PKG_ROOT)
    test_dir = Path(__file__).parent
    runner = TestRunner(
        library_name="xwsystem",
        layer_name="1.unit",
        description="Utils lazy mode regression tests",
        test_dir=test_dir,
        pytest_cwd=_PKG_ROOT,
        markers=["xwsystem_unit"],
    )
    return runner.run()


if __name__ == "__main__":
    raise SystemExit(main())
