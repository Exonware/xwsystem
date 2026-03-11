#!/usr/bin/env python3
#exonware/xwsystem/tests/1.unit/shared_tests/runner.py
"""
Unit test runner for shared module.
Following GUIDE_TEST.md standards.
"""

import sys
import pytest
from pathlib import Path


def main():
    """Run shared module unit tests."""
    test_dir = Path(__file__).parent
    # Run unit tests
    exit_code = pytest.main([
        "-v",
        "--tb=short",
        "-x",  # Stop on first failure
        str(test_dir),
        "-m", "xwsystem_unit",
    ])
    return exit_code
if __name__ == "__main__":
    sys.exit(main())
