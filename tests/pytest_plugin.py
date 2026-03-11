#exonware/xwsystem/tests/pytest_plugin.py
"""
Pytest plugin to fix import issues with test directories.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
"""

from __future__ import annotations
import sys
from pathlib import Path
# Lib installed editable; no manual src path setup.
_tests_dir = Path(__file__).resolve().parent
# Add all test layer directories
_test_layer_dirs = [
    _tests_dir / "0.core",
    _tests_dir / "1.unit",
    _tests_dir / "2.integration",
    _tests_dir / "3.advance",
]
# Add all subdirectories in unit tests that might be imported as modules
_unit_tests_dir = _tests_dir / "1.unit"
if _unit_tests_dir.exists():
    for item in _unit_tests_dir.iterdir():
        if item.is_dir() and not item.name.startswith('__'):
            _test_layer_dirs.append(item)
# Add all subdirectories in integration tests
_integration_tests_dir = _tests_dir / "2.integration"
if _integration_tests_dir.exists():
    for item in _integration_tests_dir.iterdir():
        if item.is_dir() and not item.name.startswith('__'):
            _test_layer_dirs.append(item)
# Add all directories to sys.path for test discovery
for test_dir in _test_layer_dirs:
    if test_dir.exists() and str(test_dir) not in sys.path:
        sys.path.insert(0, str(test_dir))
# Also add the tests directory itself to handle nested imports
if str(_tests_dir) not in sys.path:
    sys.path.insert(0, str(_tests_dir))


def pytest_configure(config):
    """Configure pytest before test collection starts."""
    pass  # Lib installed editable; path setup handled at module level if needed


def pytest_collect_file(file_path: Path, parent):
    """
    Custom collection hook; returns None (default collection).
    """
    return None
