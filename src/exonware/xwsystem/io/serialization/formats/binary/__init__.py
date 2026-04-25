#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/binary/__init__.py
"""Binary serialization formats (core lightweight formats)."""
# Core binary formats

from .pickle import PickleSerializer
from .marshal import MarshalSerializer
from .plistlib import PlistSerializer

try:
    from .msgpack import MsgPackSerializer
except (ImportError, ModuleNotFoundError):
    MsgPackSerializer = None

try:
    from .bson import BsonSerializer
except (ImportError, ModuleNotFoundError):
    BsonSerializer = None

try:
    from .cbor import CborSerializer
except (ImportError, ModuleNotFoundError):
    CborSerializer = None

__all__ = [
    "MsgPackSerializer",
    "PickleSerializer",
    "BsonSerializer",
    "MarshalSerializer",
    "CborSerializer",
    "PlistSerializer",
]
# NOTE: Enterprise binary formats moved to xwformats:
# - UBJSON (py-ubjson library, ~100 KB)
# 
# Install with: pip install exonware-xwformats
