# exonware/xwsystem/src/xwsystem.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Top-level ``xwsystem`` import alias: same module object as ``exonware.xwsystem``.

Use ``import xwsystem`` / ``from xwsystem import ...`` interchangeably with the
namespace package path. Replaces the historical star-import shim, which could not
mirror ``exonware.xwsystem`` when ``__all__`` listed lazily or indirectly defined
names.
"""

from __future__ import annotations

import sys

import exonware.xwsystem as _pkg

sys.modules[__name__] = _pkg
