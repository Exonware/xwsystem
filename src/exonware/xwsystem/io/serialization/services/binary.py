#exonware/xwsystem/src/exonware/xwsystem/io/serialization/services/binary.py
"""
Binary service for serialization pipeline: optional length-prefix framing.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Ensures binary-safe handling; optional length-prefixed framing for streaming.
"""

from __future__ import annotations
import struct
# 4-byte big-endian length prefix
LENGTH_PREFIX_FORMAT = ">I"
LENGTH_PREFIX_SIZE = 4


class BinaryService:
    """
    Bytes-in, bytes-out. Optional length-prefix wrap/unwrap for framing.
    """

    def wrap(self, data: bytes, *, use_length_prefix: bool = False) -> bytes:
        """
        Optionally wrap data with 4-byte big-endian length prefix.
        If use_length_prefix is False, return data as-is.
        """
        if not use_length_prefix:
            return data
        return struct.pack(LENGTH_PREFIX_FORMAT, len(data)) + data

    def unwrap(self, data: bytes, *, use_length_prefix: bool = False) -> bytes:
        """
        If use_length_prefix, strip 4-byte length prefix and return payload.
        Otherwise return data as-is.
        """
        if not use_length_prefix:
            return data
        if len(data) < LENGTH_PREFIX_SIZE:
            raise ValueError("Data shorter than length prefix")
        length, = struct.unpack(LENGTH_PREFIX_FORMAT, data[:LENGTH_PREFIX_SIZE])
        if len(data) < LENGTH_PREFIX_SIZE + length:
            raise ValueError("Truncated payload")
        return data[LENGTH_PREFIX_SIZE : LENGTH_PREFIX_SIZE + length]
