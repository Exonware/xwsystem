#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/1.unit/operations_tests/runner.py
Test runner for operations module.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.387
Generation Date: October 27, 2025
Usage:
 python tests/1.unit/operations_tests/runner.py
"""

import sys
import pytest
from pathlib import Path
from exonware.xwsystem.console.cli import ensure_utf8_console
ensure_utf8_console()


def main():
    """Run operations module tests. Layer runners stream only, no file writes."""
    module_name = Path(__file__).parent.name.replace("_tests", "")
    print(f"🧪 Testing: {module_name}")
    exit_code = pytest.main([
        "-v",
        "--tb=short",
        "-x",
        str(Path(__file__).parent),
        "-m", "xwsystem_unit",
    ])
    status = "✅ PASSED" if exit_code == 0 else "❌ FAILED"
    print(f"\n{status}")
    sys.exit(exit_code)
if __name__ == "__main__":
    main()
