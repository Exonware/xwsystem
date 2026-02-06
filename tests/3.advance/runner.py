#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/3.advance/runner.py

Advance test runner for xwsystem - Excellence Validation
Tests against eXonware's 5 core priorities.

OPTIONAL until v1.0.0, MANDATORY for production releases.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1.388
Generation Date: 01-Nov-2025

Usage:
    python tests/3.advance/runner.py  # Run all advance tests
    python tests/3.advance/runner.py --security  # Priority #1
    python tests/3.advance/runner.py --performance  # Priority #4
    python tests/3.advance/runner.py --extensibility  # Priority #5
"""

import sys
from pathlib import Path

from exonware.xwsystem.console.cli import ensure_utf8_console
from exonware.xwsystem.utils.test_runner import (
    format_path,
    print_header,
    print_status,
    run_pytest,
)

ensure_utf8_console()


def main():
    """Advance test runner - Excellence validation. Uses shared utilities, no file writes."""
    advance_dir = Path(__file__).parent
    args = sys.argv[1:]

    if "--security" in args:
        print_header("Advance Tests - Priority #1: Security Excellence", None)
        markers = ["xwsystem_security"]
        extra = ["-k", "caching"]
    elif "--performance" in args:
        print_header("Advance Tests - Priority #4: Performance Excellence", None)
        markers = ["xwsystem_performance"]
        extra = ["-k", "caching"]
    elif "--extensibility" in args:
        print_header("Advance Tests - Priority #5: Extensibility Excellence", None)
        markers = ["xwsystem_extensibility"]
        extra = ["-k", "caching"]
    elif "--usability" in args or "--maintainability" in args:
        print_header("Advance Tests - Excellence Validation", None)
        m = "xwsystem_usability" if "--usability" in args else "xwsystem_maintainability"
        markers = [m]
        extra = ["-k", "caching"]
    else:
        print_header("Advance Tests - All Priorities (Caching Module)", None)
        markers = ["xwsystem_advance"]
        extra = ["-k", "caching"]

    print(f"📂 Directory: {format_path(advance_dir)}\n")
    result = run_pytest(test_dir=advance_dir, markers=markers, extra_args=extra)
    ok = result.exit_code == 0
    print_status(ok, "Advance tests PASSED" if ok else "Advance tests FAILED", None)
    sys.exit(result.exit_code)


if __name__ == "__main__":
    main()
