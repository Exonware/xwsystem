#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/0.core/caching/runner.py
Core test runner for caching module - Fast, High-Value Checks.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.388
Generation Date: 01-Nov-2025
Usage:
    python tests/0.core/caching/runner.py
"""

import sys
import pytest
from pathlib import Path
from exonware.xwsystem.console.cli import ensure_utf8_console
ensure_utf8_console()


def main():
    """Core test runner - 20% tests for 80% value."""
    print("🎯 Core Caching Tests - Fast, High-Value Checks")
    print(f"📂 Test Directory: {Path(__file__).parent.absolute()}")
    print()
    # Run core tests
    exit_code = pytest.main([
        "-v",
        "--tb=short",
        "-x",  # Stop on first failure
        str(Path(__file__).parent),
        "-m", "xwsystem_core and xwsystem_caching"
    ])
    # Status output
    if exit_code == 0:
        status = "✅ Core caching tests PASSED"
    else:
        status = "❌ Core caching tests FAILED"
    print()
    print(status)
    sys.exit(exit_code)
if __name__ == "__main__":
    main()
