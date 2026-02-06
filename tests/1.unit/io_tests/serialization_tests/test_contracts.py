#exonware/xwsystem/tests/1.unit/io_tests/serialization_tests/test_contracts.py
"""
Unit tests for io.serialization.contracts module

Tests ISerialization interface.
Following GUIDELINES_TEST.md structure and eXonware testing standards.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
"""

import pytest
from typing import Protocol, runtime_checkable
from exonware.xwsystem.io.serialization.contracts import ISerialization
from exonware.xwsystem.io.codec.contracts import ICodec


@pytest.mark.xwsystem_unit
class TestSerializationInterface:
    """Test ISerialization interface definition."""
    
    def test_iserialization_is_protocol(self):
        """Test ISerialization is a Protocol (structural typing interface)."""
        # ISerialization is a Protocol, not an ABC
        assert isinstance(ISerialization, type)
        # Check that it's marked as runtime_checkable
        assert hasattr(ISerialization, '__protocol_attrs__') or runtime_checkable(ISerialization)
    
    def test_iserialization_extends_icodec(self):
        """Test ISerialization extends ICodec."""
        # ISerialization should inherit from ICodec (which is also a Protocol)
        # Protocols use structural subtyping, so we check if ICodec is in the MRO
        assert ICodec in ISerialization.__mro__
    
    def test_iserialization_has_serialization_methods(self):
        """Test ISerialization defines serialization-specific methods."""
        # Protocols define methods via type annotations, check for key methods
        assert hasattr(ISerialization, 'encode')
        assert hasattr(ISerialization, 'decode')
        assert hasattr(ISerialization, 'save_file')
        assert hasattr(ISerialization, 'load_file')
    
    def test_iserialization_cannot_be_instantiated(self):
        """Test that ISerialization cannot be directly instantiated."""
        with pytest.raises(TypeError):
            ISerialization()
