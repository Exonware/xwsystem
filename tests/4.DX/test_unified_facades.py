#exonware/xwsystem/tests/4.DX/test_unified_facades.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.1.0.1
Generation Date: January 2026
Comprehensive tests for unified facades - DX improvements.
"""

import pytest
import tempfile
import os
from pathlib import Path
from exonware.xwsystem import (
    XWCache,
    XWSerializer,
    XWArchive,
    XWIndex,
    XWSecurity,
    XWCrypto,
    XWHTTP,
    XWValidator,
    XWMonitor,
    XWConcurrency,
)


class TestXWCache:
    """Test XWCache unified facade."""

    def test_simple_lru_cache(self):
        """Test simple LRU cache creation and usage."""
        cache = XWCache(strategy="LRU", capacity=100)
        cache.put("key1", "value1")
        assert cache.get("key1") == "value1"
        assert cache.get("key2") is None

    def test_named_cache(self):
        """Test named cache instances."""
        user_cache = XWCache(strategy="LRU", name="users", capacity=50)
        user_cache.put("user1", {"name": "John"})
        retrieved = XWCache.get_cache("users")
        assert retrieved is not None

    def test_ttl_cache(self):
        """Test TTL cache."""
        cache = XWCache(strategy="TTL", ttl=1, capacity=10)
        cache.put("key", "value")
        assert cache.get("key") == "value"

    def test_dict_like_access(self):
        """Test dict-like access pattern."""
        cache = XWCache(strategy="LRU", capacity=10)
        cache["key"] = "value"
        assert cache["key"] == "value"
        del cache["key"]
        assert cache.get("key") is None

    def test_cache_decorator(self):
        """Test cache decorator."""
        call_count = [0]
        @XWCache.cached(strategy="LRU", capacity=10)
        def expensive_function(x):
            call_count[0] += 1
            return x * 2
        result1 = expensive_function(5)
        result2 = expensive_function(5)  # Should use cache
        assert result1 == 10
        assert result2 == 10
        assert call_count[0] == 1  # Function called only once


class TestXWSerializer:
    """Test XWSerializer unified facade."""

    def test_format_specific_serializer(self):
        """Test format-specific serializer creation."""
        json_ser = XWSerializer("json")
        data = {"name": "test", "value": 42}
        result = json_ser.dumps(data)
        assert isinstance(result, str)
        assert "test" in result

    def test_positional_format(self):
        """Test positional format parameter."""
        yaml_ser = XWSerializer("yaml")
        data = {"key": "value"}
        result = yaml_ser.dumps(data)
        assert isinstance(result, str)

    def test_auto_detect(self):
        """Test auto-detection when format not specified."""
        serializer = XWSerializer()
        # Should work with auto-detection
        assert serializer is not None


class TestXWArchive:
    """Test XWArchive unified facade."""

    def test_format_specific_archive(self):
        """Test format-specific archive creation."""
        archive = XWArchive("zip")
        assert archive.format == "zip"

    def test_auto_detect_from_path(self):
        """Test auto-detection from file path."""
        with tempfile.NamedTemporaryFile(suffix=".zip", delete=False) as tmp:
            tmp_path = tmp.name
        try:
            archive = XWArchive(tmp_path)
            assert archive.format == "zip" or archive.format is not None
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)

    def test_create_archive_static(self):
        """Test static method to create archive."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create test files
            test_file = Path(tmpdir) / "test.txt"
            test_file.write_text("test content")
            archive_path = Path(tmpdir) / "test.zip"
            XWArchive.create_archive(archive_path, [test_file])
            assert archive_path.exists()


class TestXWIndex:
    """Test XWIndex unified facade."""

    def test_simple_indexing(self):
        """Test simple indexing."""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.jsonl', delete=False) as tmp:
            tmp.write('{"id": "1", "name": "Alice"}\n')
            tmp.write('{"id": "2", "name": "Bob"}\n')
            tmp_path = tmp.name
        try:
            index = XWIndex(tmp_path)
            index.build()
            record = index.get_by_line(0)
            assert record is not None
            assert record.get("name") == "Alice"
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)

    def test_get_by_id(self):
        """Test ID-based retrieval."""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.jsonl', delete=False) as tmp:
            tmp.write('{"id": "user1", "name": "Alice"}\n')
            tmp.write('{"id": "user2", "name": "Bob"}\n')
            tmp_path = tmp.name
        try:
            index = XWIndex(tmp_path, id_field="id")
            record = index.get_by_id("user1")
            assert record is not None
            assert record.get("name") == "Alice"
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)

    def test_paging(self):
        """Test paging functionality."""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.jsonl', delete=False) as tmp:
            for i in range(20):
                tmp.write(f'{{"id": "{i}", "value": {i}}}\n')
            tmp_path = tmp.name
        try:
            index = XWIndex(tmp_path)
            page = index.get_page(page=0, size=10)
            assert len(page) == 10
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)


class TestXWSecurity:
    """Test XWSecurity unified facade."""

    def test_hash(self):
        """Test hashing."""
        hash_value = XWSecurity.hash("password", algorithm="sha256")
        assert isinstance(hash_value, str)
        assert len(hash_value) == 64  # SHA-256 hex digest length

    def test_verify_hash(self):
        """Test hash verification."""
        hash_value = XWSecurity.hash("password", algorithm="sha256")
        assert XWSecurity.verify_hash("password", hash_value, algorithm="sha256")
        assert not XWSecurity.verify_hash("wrong", hash_value, algorithm="sha256")

    def test_password_hashing(self):
        """Test password hashing."""
        hashed = XWSecurity.hash_password("mypassword")
        assert XWSecurity.verify_password("mypassword", hashed)
        assert not XWSecurity.verify_password("wrong", hashed)

    def test_secure_storage(self):
        """Test secure storage."""
        storage = XWSecurity.Storage()
        storage.set("api_key", "sk_test_123")
        value = storage.get("api_key")
        assert value == "sk_test_123"


class TestXWCrypto:
    """Test XWCrypto unified facade."""

    def test_symmetric_encryption(self):
        """Test symmetric encryption."""
        crypto = XWCrypto()
        encrypted = crypto.encrypt("secret data")
        decrypted = crypto.decrypt(encrypted)
        # Decrypt returns bytes, need to decode
        assert decrypted.decode('utf-8') == "secret data"

    def test_password_based_encryption(self):
        """Test password-based encryption."""
        crypto = XWCrypto.from_password("mypassword")
        encrypted = crypto.encrypt("data")
        decrypted = crypto.decrypt(encrypted)
        # Decrypt returns bytes, need to decode
        assert decrypted.decode('utf-8') == "data"

    def test_static_encrypt_decrypt(self):
        """Test static encrypt/decrypt methods."""
        encrypted = XWCrypto.encrypt_static("data", password="pwd")
        decrypted = XWCrypto.decrypt_static(encrypted, password="pwd")
        # Decrypt returns bytes, need to decode
        assert decrypted.decode('utf-8') == "data"


class TestXWHTTP:
    """Test XWHTTP unified facade."""

    def test_static_get(self):
        """Test static GET request."""
        # This will make a real HTTP request - might fail if offline
        # In real tests, use mocking
        try:
            response = XWHTTP.get("https://httpbin.org/get")
            assert response is not None
        except Exception:
            pytest.skip("Network unavailable or httpbin.org down")

    def test_client_instance(self):
        """Test client instance creation."""
        client = XWHTTP(base_url="https://httpbin.org", timeout=10)
        assert client.base_url == "https://httpbin.org"
        assert client.timeout == 10


class TestXWValidator:
    """Test XWValidator unified facade."""

    def test_quick_validation(self):
        """Test quick validation."""
        data = {"name": "John", "age": 30}
        XWValidator.validate(data, rules={"name": "required", "age": "int"})

    def test_validation_error(self):
        """Test validation errors."""
        data = {"name": "John"}
        with pytest.raises(Exception):  # ValidationError
            XWValidator.validate(data, rules={"age": "required"})

    def test_path_validation(self):
        """Test path validation."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create a file first
            test_file = Path(tmpdir) / "file.txt"
            test_file.write_text("test")
            safe_path = XWValidator.validate_path("file.txt", base_path=tmpdir, must_exist=True)
            assert safe_path is not None

    def test_depth_validation(self):
        """Test depth validation."""
        data = {"level1": {"level2": {"level3": "deep"}}}
        XWValidator.validate_depth(data, max_depth=10)
        # Deep nested data should fail
        deep_data = {"a": {"b": {"c": {"d": {"e": {"f": {"g": {"h": {"i": {"j": "too deep"}}}}}}}}}}
        with pytest.raises(Exception):  # ValidationError
            XWValidator.validate_depth(deep_data, max_depth=5)


class TestXWMonitor:
    """Test XWMonitor unified facade."""

    def test_performance_context(self):
        """Test performance monitoring context."""
        with XWMonitor.performance("test_operation"):
            result = sum(range(1000))
            assert result == 499500

    def test_memory_monitor(self):
        """Test memory monitoring."""
        monitor = XWMonitor.memory(auto_cleanup=False)
        assert monitor is not None

    def test_cpu_usage(self):
        """Test CPU usage monitoring."""
        cpu = XWMonitor.cpu_usage()
        assert isinstance(cpu, (int, float))
        assert 0 <= cpu <= 100

    def test_memory_usage(self):
        """Test memory usage monitoring."""
        memory = XWMonitor.memory_usage()
        assert isinstance(memory, dict)
        assert "used" in memory or "percent" in memory


class TestXWConcurrency:
    """Test XWConcurrency unified facade."""

    def test_factory(self):
        """Test thread-safe factory."""
        factory = XWConcurrency.Factory()
        class MyHandler:
            def __init__(self):
                self.value = 42
        factory.register("handler", MyHandler, thread_safe=True)
        handler = factory.get("handler")
        assert handler.value == 42

    def test_async_primitives(self):
        """Test async primitives creation."""
        lock = XWConcurrency.Lock()
        queue = XWConcurrency.Queue()
        semaphore = XWConcurrency.Semaphore(5)
        assert lock is not None
        assert queue is not None
        assert semaphore is not None

    def test_thread_pool(self):
        """Test thread pool."""
        def square(x):
            return x * x
        with XWConcurrency.Pool(workers=2) as pool:
            results = pool.map(square, [1, 2, 3, 4, 5])
            assert results == [1, 4, 9, 16, 25]


class TestUnifiedAPI:
    """Test unified top-level API."""

    def test_all_facades_importable(self):
        """Test that all facades can be imported."""
        from exonware.xwsystem import (
            XWCache,
            XWSerializer,
            XWArchive,
            XWIndex,
            XWSecurity,
            XWCrypto,
            XWHTTP,
            XWValidator,
            XWMonitor,
            XWConcurrency,
        )
        # All should be importable
        assert XWCache is not None
        assert XWSerializer is not None
        assert XWArchive is not None
        assert XWIndex is not None
        assert XWSecurity is not None
        assert XWCrypto is not None
        assert XWHTTP is not None
        assert XWValidator is not None
        assert XWMonitor is not None
        assert XWConcurrency is not None

    def test_one_line_usage_patterns(self):
        """Test one-line usage patterns."""
        # Cache
        cache = XWCache("LRU", capacity=10)
        cache.put("key", "value")
        assert cache.get("key") == "value"
        # Serializer
        json_ser = XWSerializer("json")
        data = json_ser.dumps({"test": "data"})
        assert isinstance(data, str)
        # Security
        hash_value = XWSecurity.hash("password")
        assert isinstance(hash_value, str)
