#!/usr/bin/env python3
#exonware/xwsystem/tests/1.unit/io_tests/runner.py
"""
Test runner for io_tests module

Runs all unit tests for the io module following GUIDELINES_TEST.md.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
"""

import sys
import pytest
from pathlib import Path

from exonware.xwsystem.console.cli import ensure_utf8_console

ensure_utf8_console()


def main():
    """Run io module unit tests. Layer runners stream only, no file writes."""
    print("🧩 Unit Tests - IO Module")
    print(f"📂 Test Directory: {Path(__file__).parent}")
    print("=" * 80)

    exit_code = pytest.main([
        "-v",
        "--tb=short",
        "-x",
        str(Path(__file__).parent),
        "-m", "xwsystem_unit",
    ])
    status = "✅ IO unit tests PASSED" if exit_code == 0 else "❌ IO unit tests FAILED"
    print(f"\n{status}")
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
