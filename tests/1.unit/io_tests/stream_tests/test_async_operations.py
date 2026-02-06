#!/usr/bin/env python3
#exonware/xwsystem/tests/1.unit/io_tests/stream_tests/test_async_operations.py
# -*- coding: utf-8 -*-
"""
Unit tests for AsyncAtomicFileWriter.

Following GUIDE_TEST.md standards.
"""

import sys
from pathlib import Path

import pytest

from exonware.xwsystem.io.stream import AsyncAtomicFileWriter


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_io
class TestAsyncAtomicFileWriter:
    """Test AsyncAtomicFileWriter class."""
    
    @pytest.mark.asyncio
    async def test_async_atomic_file_writer_basic(self, tmp_path):
        """Test basic async atomic file writing."""
        test_file = tmp_path / "async_test.txt"
        test_content = "Async test content"
        
        async_writer = AsyncAtomicFileWriter(test_file)
        await async_writer.start()
        # In text mode (default), write() expects str, not bytes
        await async_writer.write(test_content)
        await async_writer.commit()
        
        assert test_file.exists()
        assert test_file.read_text() == test_content
    
    @pytest.mark.asyncio
    async def test_async_atomic_file_writer_context_manager(self, tmp_path):
        """Test async writer as context manager."""
        test_file = tmp_path / "async_context.txt"
        test_content = "Context test"
        
        # In text mode (default), write() expects str, not bytes
        async with AsyncAtomicFileWriter(test_file) as writer:
            await writer.write(test_content)
        
        assert test_file.exists()
        assert test_file.read_text() == test_content
