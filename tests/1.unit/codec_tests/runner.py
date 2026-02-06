#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/1.unit/codec_tests/runner.py

Test runner for codec module tests.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 04-Nov-2025
"""

import sys
import pytest
from pathlib import Path

from exonware.xwsystem.console.cli import ensure_utf8_console

ensure_utf8_console()


def main():
    """Run codec module tests."""
    module_name = "codec"
    print(f"🧩 Testing module: {module_name}")
    
    # Run module tests
    exit_code = pytest.main([
        "-v",
        "--tb=short",
        "-x",  # Stop on first failure (GUIDELINES_TEST.md)
        str(Path(__file__).parent),
        "-m", "xwsystem_unit"
    ])
    
    # Layer runners stream to stdout only - do NOT write files
    # Only main runner (tests/runner.py) writes to docs/tests/
    
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
