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
from pathlib import Path
from exonware.xwsystem.console.cli import ensure_utf8_console
from exonware.xwsystem.utils.test_runner import TestRunner
ensure_utf8_console()


def main() -> int:
    """Execute all integration tests using TestRunner."""
    test_dir = Path(__file__).parent
    runner = TestRunner(
        library_name="xwsystem",
        layer_name="2.integration",
        description="Integration Tests - Cross-module Scenarios",
        test_dir=test_dir,
        markers=["xwsystem_integration"],
        output_file=None,  # Layer runners don't write files
    )
    return runner.run()
if __name__ == "__main__":
    sys.exit(main())
