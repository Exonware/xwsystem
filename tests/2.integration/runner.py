#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/2.integration/runner.py
Integration Test Runner - Cross-module scenario tests.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 2026-01-24
"""

from __future__ import annotations
import sys
import os
from pathlib import Path
from exonware.xwsystem.console.cli import ensure_utf8_console

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
ensure_utf8_console()


def main() -> int:
    """Execute all integration tests using TestRunner."""
    os.chdir(_PKG_ROOT)
    test_dir = Path(__file__).parent
    runner = TestRunner(
        library_name="xwsystem",
        layer_name="2.integration",
        description="Integration Tests - Cross-module Scenarios",
        test_dir=test_dir,
        pytest_cwd=_PKG_ROOT,
        markers=["xwsystem_integration"],
        output_file=None,  # Layer runners don't write files
    )
    return runner.run()
if __name__ == "__main__":
    sys.exit(main())
