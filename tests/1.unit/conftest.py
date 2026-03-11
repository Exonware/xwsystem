#exonware/xwsystem/tests/1.unit/conftest.py
"""
Pytest configuration for xwsystem unit tests.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
"""

from __future__ import annotations
import sys
from pathlib import Path
# CRITICAL: Add this directory to sys.path at module level
# This fixes pytest collection issues where it tries to import test directories as modules
# Must be done before pytest tries to import threading_tests.conftest
_unit_tests_dir = Path(__file__).resolve().parent
if str(_unit_tests_dir) not in sys.path:
    sys.path.insert(0, str(_unit_tests_dir))
