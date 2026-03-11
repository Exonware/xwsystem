#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/3.advance/test_performance.py
Comprehensive performance benchmarks for xwsystem.
Priority #4: Performance Excellence
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 07-Jan-2025
"""

import pytest
import time
import sys
from pathlib import Path
from exonware.xwsystem.caching.lru_cache import LRUCache
from exonware.xwsystem.io.serialization.serializer import XWSerializer
from exonware.xwsystem.security.crypto import SecureHash, SymmetricEncryption
from exonware.xwsystem.io.file import XWFile
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_performance

class TestSerializationPerformance:
    """Performance tests for serialization operations."""

    def test_json_serialization_performance(self):
        """Test JSON serialization performance."""
        serializer = XWSerializer()
        data = {"key": "value" * 1000, "numbers": list(range(10000))}
        start = time.time()
        serialized = serializer.serialize(data, format="json")
        serialize_time = time.time() - start
        start = time.time()
        deserialized = serializer.deserialize(serialized, format="json")
        deserialize_time = time.time() - start
        # Should complete in reasonable time (< 1 second)
        assert serialize_time < 1.0, f"Serialization too slow: {serialize_time:.3f}s"
        assert deserialize_time < 1.0, f"Deserialization too slow: {deserialize_time:.3f}s"
        assert deserialized == data

    def test_large_data_serialization(self):
        """Test serialization performance with large data."""
        serializer = XWSerializer()
        large_data = {"items": [{"id": i, "data": "x" * 100} for i in range(10000)]}
        start = time.time()
        serialized = serializer.serialize(large_data, format="json")
        serialize_time = time.time() - start
        # Large data should still serialize in reasonable time
        assert serialize_time < 5.0, f"Large data serialization too slow: {serialize_time:.3f}s"
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_performance

class TestCachingPerformance:
    """Performance tests for caching operations."""

    def test_cache_get_performance(self):
        """Test cache get operation performance."""
        cache = LRUCache(capacity=10000)
        # Pre-populate cache
        for i in range(1000):
            cache.put(f"key_{i}", f"value_{i}")
        # Measure get performance
        start = time.time()
        for i in range(1000):
            _ = cache.get(f"key_{i}")
        elapsed = time.time() - start
        # 1000 gets should complete in < 0.1 seconds
        assert elapsed < 0.1, f"Cache get too slow: {elapsed:.3f}s for 1000 operations"

    def test_cache_put_performance(self):
        """Test cache put operation performance."""
        cache = LRUCache(capacity=10000)
        start = time.time()
        for i in range(1000):
            cache.put(f"key_{i}", f"value_{i}")
        elapsed = time.time() - start
        # 1000 puts should complete in < 0.1 seconds
        assert elapsed < 0.1, f"Cache put too slow: {elapsed:.3f}s for 1000 operations"

    def test_cache_eviction_performance(self):
        """Test cache eviction performance."""
        cache = LRUCache(capacity=100)
        # Fill cache beyond capacity
        start = time.time()
        for i in range(200):
            cache.put(f"key_{i}", f"value_{i}")
        elapsed = time.time() - start
        # Eviction should not significantly slow down operations
        assert elapsed < 0.2, f"Cache eviction too slow: {elapsed:.3f}s"
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_performance

class TestCryptographicPerformance:
    """Performance tests for cryptographic operations."""

    def test_encryption_performance(self):
        """Test encryption performance."""
        key = SymmetricEncryption.generate_key()
        crypto = SymmetricEncryption(key)
        data = b"x" * 1024 * 1024  # 1MB
        start = time.time()
        encrypted = crypto.encrypt(data)
        encrypt_time = time.time() - start
        start = time.time()
        decrypted = crypto.decrypt(encrypted)
        decrypt_time = time.time() - start
        # 1MB encryption/decryption should complete in < 1 second
        assert encrypt_time < 1.0, f"Encryption too slow: {encrypt_time:.3f}s"
        assert decrypt_time < 1.0, f"Decryption too slow: {decrypt_time:.3f}s"
        assert decrypted == data

    def test_hashing_performance(self):
        """Test hashing performance."""
        data = b"x" * 1024 * 1024  # 1MB
        start = time.time()
        hash_value = SecureHash.sha256(data)
        elapsed = time.time() - start
        # 1MB hashing should complete in < 0.5 seconds
        assert elapsed < 0.5, f"Hashing too slow: {elapsed:.3f}s"
        assert len(hash_value) == 64  # SHA-256 produces 64-char hex
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_performance

class TestFileOperationsPerformance:
    """Performance tests for file operations."""

    def test_file_read_performance(self, tmp_path):
        """Test file read performance."""
        test_file = tmp_path / "test.txt"
        test_file.write_text("x" * 1024 * 100)  # 100KB
        file_ops = XWFile(test_file)
        start = time.time()
        content = file_ops.load()
        elapsed = time.time() - start
        # 100KB read should complete in < 0.1 seconds
        assert elapsed < 0.1, f"File read too slow: {elapsed:.3f}s"
        assert len(content) == 1024 * 100

    def test_file_write_performance(self, tmp_path):
        """Test file write performance."""
        test_file = tmp_path / "test.txt"
        data = "x" * 1024 * 100  # 100KB
        file_ops = XWFile(test_file)
        start = time.time()
        result = file_ops.save(data)
        elapsed = time.time() - start
        # 100KB write should complete in < 0.1 seconds
        assert elapsed < 0.1, f"File write too slow: {elapsed:.3f}s"
        assert result is True
        assert test_file.exists()
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_performance

class TestMemoryPerformance:
    """Performance tests for memory usage."""

    def test_cache_memory_efficiency(self):
        """Test that cache uses memory efficiently."""
        cache = LRUCache(capacity=1000)
        # Fill cache
        for i in range(1000):
            cache.put(f"key_{i}", f"value_{i}" * 10)
        # Check memory usage (approximate)
        import sys
        cache_size = sys.getsizeof(cache)
        # Cache should not use excessive memory
        # This is a rough check - actual memory depends on implementation
        assert cache_size < 10 * 1024 * 1024, f"Cache uses too much memory: {cache_size} bytes"
@pytest.mark.xwsystem_advance
@pytest.mark.xwsystem_performance

class TestConcurrentPerformance:
    """Performance tests for concurrent operations."""

    def test_concurrent_cache_operations(self):
        """Test concurrent cache operations performance."""
        import threading
        cache = LRUCache(capacity=10000)
        errors = []
        def worker(worker_id):
            try:
                for i in range(100):
                    key = f"w{worker_id}_k{i}"
                    cache.put(key, f"value_{i}")
                    value = cache.get(key)
                    assert value is not None
            except Exception as e:
                errors.append(str(e))
        start = time.time()
        threads = [threading.Thread(target=worker, args=(i,)) for i in range(10)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()
        elapsed = time.time() - start
        # 10 threads, 100 operations each = 1000 operations
        # Should complete in < 1 second
        assert elapsed < 1.0, f"Concurrent operations too slow: {elapsed:.3f}s"
        assert len(errors) == 0, f"Errors in concurrent operations: {errors[:5]}"
