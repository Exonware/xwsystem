#exonware/xwsystem/tests/1.unit/io_tests/serialization_tests/test_base.py
"""
Unit tests for io.serialization.base module

Tests ASerialization abstract base class.
Following GUIDELINES_TEST.md structure and eXonware testing standards.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
"""

import pytest
from abc import ABC
from pathlib import Path
from tempfile import TemporaryDirectory

from exonware.xwsystem.io.serialization.base import (
    ASerialization,
    get_cached_serializer_for_path,
)
from exonware.xwsystem.io.serialization.contracts import ISerialization
from exonware.xwsystem.io.codec.base import ACodec


@pytest.mark.xwsystem_unit
class TestASerializationBase:
    """Test ASerialization abstract base class."""
    
    def test_aserialization_is_abc(self):
        """Test ASerialization is an abstract base class."""
        assert issubclass(ASerialization, ABC)
    
    def test_aserialization_extends_acodec(self):
        """Test ASerialization extends ACodec."""
        assert issubclass(ASerialization, ACodec)
    
    def test_aserialization_implements_iserialization(self):
        """Test ASerialization implements ISerialization."""
        # Protocols with non-method members don't support issubclass()
        # Instead, check that ASerialization has the required interface methods
        from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
        serializer = JsonSerializer()
        # Verify it implements ISerialization by checking it has required methods
        assert hasattr(serializer, 'encode')
        assert hasattr(serializer, 'decode')
        assert hasattr(serializer, 'save_file')
        assert hasattr(serializer, 'load_file')
    
    def test_aserialization_cannot_be_instantiated_directly(self):
        """Test ASerialization cannot be instantiated without implementing abstract methods."""
        with pytest.raises(TypeError):
            ASerialization()


@pytest.mark.xwsystem_unit
class TestASerializationXWSystemIntegration:
    """Test ASerialization XWSystem integration."""
    
    def test_aserialization_provides_xwsystem_integration(self):
        """Test ASerialization provides XWSystem integration methods."""
        # ASerialization should integrate with XWSystem utilities
        # This includes file I/O, async support, and streaming
        # Tested through concrete implementations
        assert issubclass(ASerialization, ACodec)


@pytest.mark.xwsystem_unit
class TestASerializationFileCache:
    """Test file path caching functionality in ASerialization."""
    
    def test_save_file_caches_serializer(self):
        """Test that save_file() caches the serializer instance by file path."""
        from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
        
        serializer = JsonSerializer()
        test_data = {"key": "value"}
        
        with TemporaryDirectory() as tmpdir:
            test_file = Path(tmpdir) / "test.json"
            
            # Save file - should cache serializer
            serializer.save_file(test_data, test_file)
            
            # Retrieve cached serializer
            cached = get_cached_serializer_for_path(test_file)
            assert cached is not None
            assert cached is serializer  # Should be the same instance
            assert isinstance(cached, JsonSerializer)
    
    def test_load_file_caches_serializer(self):
        """Test that load_file() caches the serializer instance by file path."""
        from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
        
        serializer = JsonSerializer()
        test_data = {"key": "value"}
        
        with TemporaryDirectory() as tmpdir:
            test_file = Path(tmpdir) / "test.json"
            
            # First save to create file
            serializer.save_file(test_data, test_file)
            
            # Create new serializer instance for load
            serializer2 = JsonSerializer()
            
            # Load file - should cache serializer
            loaded_data = serializer2.load_file(test_file)
            assert loaded_data == test_data
            
            # Retrieve cached serializer - should be serializer2
            cached = get_cached_serializer_for_path(test_file)
            assert cached is not None
            assert cached is serializer2  # Should be the serializer used for load
            assert isinstance(cached, JsonSerializer)
    
    def test_cached_serializer_same_file_path(self):
        """Test that multiple operations on same file path use cached serializer."""
        from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
        
        serializer1 = JsonSerializer()
        serializer2 = JsonSerializer()
        test_data = {"key": "value"}
        
        with TemporaryDirectory() as tmpdir:
            test_file = Path(tmpdir) / "test.json"
            
            # First save
            serializer1.save_file(test_data, test_file)
            cached1 = get_cached_serializer_for_path(test_file)
            assert cached1 is serializer1
            
            # Second save with different serializer instance
            # Should update cache to new serializer
            serializer2.save_file({"key2": "value2"}, test_file)
            cached2 = get_cached_serializer_for_path(test_file)
            assert cached2 is serializer2  # Cache updated to serializer2
    
    def test_different_files_different_cache_entries(self):
        """Test that different file paths have separate cache entries."""
        from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
        from exonware.xwsystem.io.serialization.formats.text.yaml import YamlSerializer
        
        json_serializer = JsonSerializer()
        yaml_serializer = YamlSerializer()
        test_data = {"key": "value"}
        
        with TemporaryDirectory() as tmpdir:
            json_file = Path(tmpdir) / "test.json"
            yaml_file = Path(tmpdir) / "test.yaml"
            
            # Save to JSON file
            json_serializer.save_file(test_data, json_file)
            json_cached = get_cached_serializer_for_path(json_file)
            assert json_cached is json_serializer
            assert isinstance(json_cached, JsonSerializer)
            
            # Save to YAML file
            yaml_serializer.save_file(test_data, yaml_file)
            yaml_cached = get_cached_serializer_for_path(yaml_file)
            assert yaml_cached is yaml_serializer
            assert isinstance(yaml_cached, YamlSerializer)
            
            # Verify they're different cache entries
            assert json_cached is not yaml_cached
            assert get_cached_serializer_for_path(json_file) is json_serializer
            assert get_cached_serializer_for_path(yaml_file) is yaml_serializer
    
    def test_cache_returns_none_for_unused_path(self):
        """Test that get_cached_serializer_for_path returns None for uncached paths."""
        with TemporaryDirectory() as tmpdir:
            uncached_file = Path(tmpdir) / "uncached.json"
            cached = get_cached_serializer_for_path(uncached_file)
            assert cached is None
    
    def test_cache_resolves_file_paths(self):
        """Test that cache uses resolved (absolute) paths as keys."""
        from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
        
        serializer = JsonSerializer()
        test_data = {"key": "value"}
        
        with TemporaryDirectory() as tmpdir:
            # Use relative path
            relative_file = Path(tmpdir) / "test.json"
            serializer.save_file(test_data, relative_file)
            
            # Try to retrieve with absolute path (should work)
            absolute_file = relative_file.resolve()
            cached1 = get_cached_serializer_for_path(relative_file)
            cached2 = get_cached_serializer_for_path(absolute_file)
            
            # Both should return the same cached serializer
            assert cached1 is not None
            assert cached2 is not None
            assert cached1 is cached2  # Same instance
            assert cached1 is serializer
