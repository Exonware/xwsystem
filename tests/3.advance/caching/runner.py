#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/3.advance/caching/runner.py
Advance test runner for caching module - Excellence Validation (v1.0.0+).
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.388
Generation Date: 01-Nov-2025
OPTIONAL until v1.0.0, MANDATORY for use releases.
Usage:
    python tests/3.advance/caching/runner.py              # All advance tests
    python tests/3.advance/caching/runner.py --security    # Security only
    python tests/3.advance/caching/runner.py --performance # Performance only
"""

import sys
import pytest
from pathlib import Path
from exonware.xwsystem.console.cli import ensure_utf8_console
ensure_utf8_console()


def main():
    """Advance test runner - Excellence validation."""
    args = sys.argv[1:]
    print("🎓 Advance Caching Tests - Excellence Validation")
    print("   Testing against eXonware's 5 Core Priorities")
    print(f"📂 Test Directory: {Path(__file__).parent.absolute()}")
    print()
    # Determine which tests to run
    if "--security" in args:
        print("   Priority #1: Security Excellence")
        marker = "xwsystem_security and xwsystem_caching"
        category = "Security"
    elif "--performance" in args:
        print("   Priority #4: Performance Excellence")
        marker = "xwsystem_performance and xwsystem_caching"
        category = "Performance"
    else:
        print("   All Priorities")
        marker = "xwsystem_advance and xwsystem_caching"
        category = "All"
    # Run tests
    exit_code = pytest.main([
        "-v",
        "--tb=short",
        "-x",
        str(Path(__file__).parent),
        "-m", marker
    ])
    # Status
    if exit_code == 0:
        status = "✅ Advance caching tests PASSED"
    else:
        status = "❌ Advance caching tests FAILED"
    print()
    print(status)
    sys.exit(exit_code)
if __name__ == "__main__":
    main()
