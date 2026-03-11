#exonware/xwsystem/tests/0.core/runner.py
#!/usr/bin/env python3
"""
Core Test Runner for xwsystem.
Runs core tests (0.core) using shared TestRunner utilities.
GUIDE_TEST: layer runners use TestRunner, stream only, no file writes.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.2
Generation Date: 2026-01-24
"""

from __future__ import annotations
import sys
from pathlib import Path
from exonware.xwsystem.console.cli import ensure_utf8_console
from exonware.xwsystem.utils.test_runner import TestRunner
ensure_utf8_console()


def main() -> int:
    """Run core tests via TestRunner."""
    test_dir = Path(__file__).parent
    runner = TestRunner(
        library_name="xwsystem",
        layer_name="0.core",
        description="Core Tests - Fast, High-Value Checks",
        test_dir=test_dir,
        markers=["xwsystem_core"],
        output_file=None,
    )
    return runner.run()
if __name__ == "__main__":
    sys.exit(main())
