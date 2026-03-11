#!/usr/bin/env python3
#exonware/xwsystem/tests/2.integration/caching_integration/runner.py
"""
#exonware/xwsystem/tests/2.integration/caching/runner.py
Integration test runner for caching module - End-to-End Scenarios.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.388
Generation Date: 01-Nov-2025
Usage:
    python tests/2.integration/caching/runner.py
"""

import sys
import pytest
from pathlib import Path
from exonware.xwsystem.console.cli import ensure_utf8_console
ensure_utf8_console()


def main():
    """Integration test runner for caching module."""
    print("🔗 Integration Caching Tests - End-to-End Scenarios")
    print(f"📂 Test Directory: {Path(__file__).parent.absolute()}")
    print()
    # Run integration tests
    exit_code = pytest.main([
        "-v",
        "--tb=short",
        "-x",
        str(Path(__file__).parent),
        "-m", "xwsystem_integration and xwsystem_caching"
    ])
    # Status output
    if exit_code == 0:
        status = "✅ Integration caching tests PASSED"
    else:
        status = "❌ Integration caching tests FAILED"
    print()
    print(status)
    sys.exit(exit_code)
if __name__ == "__main__":
    main()
