#exonware/xwsystem/src/exonware/__init__.py
"""
exonware package - Enterprise-grade Python framework ecosystem
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.38
Generation Date: September 04, 2025
This is a namespace package allowing multiple exonware subpackages
to coexist (xwsystem, xwnode, xwdata, etc.)
"""
# Make this a namespace package FIRST

__path__ = __import__('pkgutil').extend_path(__path__, __name__)
import importlib.metadata
from pathlib import Path


def _load_version() -> str:
    """Resolve package version; fall back to ``version.py`` when metadata is unusable (GUIDE_53).

    ``importlib.metadata.version`` can raise ``OSError`` (e.g. Errno 22) on some Windows/OneDrive
    or damaged ``.dist-info`` installs when opening RECORD/METADATA files — not only
    ``PackageNotFoundError``.
    """
    version_path = Path(__file__).resolve().parent / "xwsystem" / "version.py"
    try:
        return importlib.metadata.version("exonware-xwsystem")
    except (importlib.metadata.PackageNotFoundError, OSError, ValueError):
        ns: dict = {}
        try:
            exec(version_path.read_text(encoding="utf-8"), ns)  # noqa: S102
        except FileNotFoundError as exc:  # pragma: no cover
            raise ImportError(
                f"Version metadata unavailable at {version_path}."
            ) from exc
        return str(ns["__version__"])
__version__ = _load_version()
__author__ = 'eXonware Backend Team'
__email__ = 'connect@exonware.com'
__company__ = 'eXonware.com'
