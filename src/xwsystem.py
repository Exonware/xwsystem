# src/xwsystem.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Top-level ``xwsystem`` import alias: same public names as ``exonware.xwsystem``.

Use ``import xwsystem`` / ``from xwsystem import ...`` interchangeably with the
namespace package path. Implemented with :pep:`562` module ``__getattr__`` so the
interpreter's ``sys.modules`` entry for ``xwsystem`` remains this module object
(rather than replacing it with another module).
"""

from __future__ import annotations

import importlib
from typing import Any

_pkg = importlib.import_module("exonware.xwsystem")
_default_all = [n for n in dir(_pkg) if not str(n).startswith("_")]
__all__ = list(getattr(_pkg, "__all__", _default_all))


def __getattr__(name: str) -> Any:
    return getattr(_pkg, name)


def __dir__() -> list[str]:
    return sorted(set(dir(_pkg)) | {"__getattr__", "__dir__", "_pkg", "__all__"})
