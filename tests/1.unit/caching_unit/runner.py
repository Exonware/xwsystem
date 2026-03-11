#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/1.unit/caching_unit/runner.py
Unit test runner for caching module - Component-Level Tests.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.388
Generation Date: 01-Nov-2025
Usage:
    python tests/1.unit/caching_unit/runner.py
"""

import sys
import pytest
from pathlib import Path
from exonware.xwsystem.console.cli import ensure_utf8_console
ensure_utf8_console()


def main():
    """Unit test runner for caching module."""
    print("🧩 Unit Caching Tests - Component-Level Validation")
    print(f"📂 Test Directory: {Path(__file__).parent.absolute()}")
    print()
    # Run unit tests
    exit_code = pytest.main([
        "-v",
        "--tb=short",
        "-x",
        str(Path(__file__).parent),
        "-m", "xwsystem_unit and xwsystem_caching"
    ])
    # Status output
    if exit_code == 0:
        status = "✅ Unit caching tests PASSED"
    else:
        status = "❌ Unit caching tests FAILED"
    print()
    print(status)
    sys.exit(exit_code)
if __name__ == "__main__":
    main()
