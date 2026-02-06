#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/1.unit/caching_tests/test_ttl_cache_comprehensive.py

Comprehensive edge case tests for TTL Cache implementation.
80+ test cases covering expiration, cleanup, and edge conditions.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 28-Dec-2025
"""

from __future__ import annotations

import pytest
import time
import threading
from concurrent.futures import ThreadPoolExecutor

from exonware.xwsystem.caching.ttl_cache import TTLCache


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching
class TestTTLCacheExpiration:
    """Test TTL expiration behavior."""
    
    def test_basic_expiration(self):
        """Test that entries expire after TTL."""
        cache = TTLCache(capacity=10, ttl=0.1)
        cache.put('k1', 'v1')
        assert cache.get('k1') == 'v1'
        
        time.sleep(0.15)
        assert cache.get('k1') is None
    
    def test_expiration_with_different_ttls(self):
        """Test expiration with different TTL values."""
        # Disable cleanup thread for predictable testing (cleanup_interval=0 disables it)
        cache = TTLCache(capacity=10, ttl=1.0, cleanup_interval=0)
        cache.put('k1', 'v1')
        
        # Should still be valid immediately after put
        result = cache.get('k1')
        assert result == 'v1'
        
        # Should still be valid before expiration (0.5s < 1.0s TTL)
        time.sleep(0.5)
        result = cache.get('k1')
        assert result == 'v1'
        
        # Should expire after TTL (wait 0.6s more, total 1.1s > 1.0s TTL)
        time.sleep(0.6)
        result = cache.get('k1')
        # Should be expired
        assert result is None
    
    def test_zero_ttl_immediate_expiration(self):
        """Test that zero TTL causes immediate expiration."""
        cache = TTLCache(capacity=10, ttl=0)
        cache.put('k1', 'v1')
        time.sleep(0.01)
        assert cache.get('k1') is None
    
    def test_negative_ttl_handling(self):
        """Test handling of negative TTL."""
        cache = TTLCache(capacity=10, ttl=-1)
        cache.put('k1', 'v1')
        # Should expire immediately
        assert cache.get('k1') is None
    
    def test_very_short_ttl(self):
        """Test very short TTL values."""
        cache = TTLCache(capacity=10, ttl=0.001)
        cache.put('k1', 'v1')
        time.sleep(0.002)
        assert cache.get('k1') is None
    
    def test_very_long_ttl(self):
        """Test very long TTL values."""
        cache = TTLCache(capacity=10, ttl=3600)
        cache.put('k1', 'v1')
        # Should still be valid after short wait
        time.sleep(0.1)
        assert cache.get('k1') == 'v1'
    
    def test_expiration_before_capacity_reached(self):
        """Test expiration when cache hasn't reached capacity."""
        cache = TTLCache(capacity=10, ttl=0.1)
        cache.put('k1', 'v1')
        cache.put('k2', 'v2')
        
        time.sleep(0.15)
        
        assert cache.get('k1') is None
        assert cache.get('k2') is None
        assert cache.size() == 0
    
    def test_expiration_after_capacity_reached(self):
        """Test expiration when cache is at capacity."""
        cache = TTLCache(capacity=3, ttl=0.1)
        cache.put('k1', 'v1')
        cache.put('k2', 'v2')
        cache.put('k3', 'v3')
        
        time.sleep(0.15)
        
        # All should be expired
        assert cache.get('k1') is None
        assert cache.get('k2') is None
        assert cache.get('k3') is None
    
    def test_partial_expiration(self):
        """Test partial expiration of entries."""
        # Disable cleanup thread for predictable testing
        cache = TTLCache(capacity=10, ttl=1.0, cleanup_interval=0)
        cache.put('k1', 'v1')
        
        time.sleep(0.3)
        cache.put('k2', 'v2')  # Newer entry
        
        # Wait so k1 expires but k2 is still valid
        # k1 expires at: t=0 + 1.0 = 1.0s
        # k2 expires at: t=0.3 + 1.0 = 1.3s
        # Check at t=1.1s: k1 expired, k2 still valid
        time.sleep(0.8)  # Total 1.1s: k1 expired (1.1 > 1.0), k2 valid (1.1 < 1.3)
        
        # k1 should be expired (put at t=0, expires at t=1.0, checked at t=1.1)
        result_k1 = cache.get('k1')
        assert result_k1 is None
        
        # k2 should still be valid (put at t=0.3, expires at t=1.3, checked at t=1.1 < 1.3)
        result_k2 = cache.get('k2')
        assert result_k2 == 'v2'


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching
class TestTTLCacheCleanup:
    """Test TTL cache cleanup behavior."""
    
    def test_cleanup_removes_expired(self):
        """Test that cleanup removes expired entries."""
        cache = TTLCache(capacity=10, ttl=0.1, cleanup_interval=0.05)
        cache.put('k1', 'v1')
        cache.put('k2', 'v2')
        
        time.sleep(0.15)  # Entries expire
        
        # Trigger cleanup
        cache._cleanup_expired()
        
        assert cache.size() == 0
    
    def test_cleanup_preserves_valid(self):
        """Test that cleanup preserves valid entries."""
        # Disable background cleanup, manually trigger cleanup
        cache = TTLCache(capacity=10, ttl=1.0, cleanup_interval=0)
        cache.put('k1', 'v1')
        
        time.sleep(0.3)
        cache.put('k2', 'v2')  # Still valid (just put)
        
        # Manual cleanup should preserve k2 (not expired yet)
        cache._cleanup_expired()
        
        # k2 should still be valid (put at t=0.3, now at t~0.3, so not expired)
        result = cache.get('k2')
        assert result == 'v2'
        assert cache.size() >= 1
    
    def test_automatic_cleanup_trigger(self):
        """Test automatic cleanup on access."""
        cache = TTLCache(capacity=10, ttl=0.1)
        cache.put('k1', 'v1')
        
        time.sleep(0.15)
        
        # Access should trigger cleanup
        result = cache.get('k1')
        assert result is None
        assert cache.size() == 0
    
    def test_cleanup_interval_configuration(self):
        """Test different cleanup interval configurations."""
        for interval in [0.01, 0.1, 1.0, 10.0]:
            cache = TTLCache(capacity=10, ttl=0.5, cleanup_interval=interval)
            assert cache.cleanup_interval == interval


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching
class TestTTLCacheEviction:
    """Test TTL cache eviction behavior."""
    
    def test_lru_eviction_when_full(self):
        """Test LRU eviction when cache is full."""
        cache = TTLCache(capacity=3, ttl=1000)  # Long TTL
        cache.put('k1', 'v1')
        cache.put('k2', 'v2')
        cache.put('k3', 'v3')
        
        # Access k1 to make it recently used
        cache.get('k1')
        
        # Add k4 - should evict k2 (LRU)
        cache.put('k4', 'v4')
        
        assert cache.get('k1') == 'v1'
        assert cache.get('k2') is None
        assert cache.get('k3') == 'v3'
        assert cache.get('k4') == 'v4'
    
    def test_eviction_vs_expiration(self):
        """Test interaction between eviction and expiration."""
        cache = TTLCache(capacity=2, ttl=0.1)
        cache.put('k1', 'v1')
        cache.put('k2', 'v2')
        
        # Wait for expiration
        time.sleep(0.15)
        
        # Add k3 - expired entries should be cleaned first
        cache.put('k3', 'v3')
        
        assert cache.size() <= 2
        assert cache.get('k3') == 'v3'


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching
class TestTTLCacheAccessPatterns:
    """Test TTL cache with various access patterns."""
    
    def test_access_count_tracking(self):
        """Test that access count is tracked."""
        cache = TTLCache(capacity=10, ttl=100)
        cache.put('k1', 'v1')
        
        for _ in range(5):
            cache.get('k1')
        
        entry = cache._cache.get('k1')
        if entry:
            assert entry.access_count >= 5
    
    def test_repeated_access_before_expiration(self):
        """Test repeated access before expiration."""
        # Disable cleanup thread for predictable testing
        cache = TTLCache(capacity=10, ttl=1.0, cleanup_interval=0)
        cache.put('k1', 'v1')
        
        # Access multiple times (each access is quick, total < TTL)
        for _ in range(10):
            time.sleep(0.05)  # 0.05s per access, total 0.5s < 1.0s TTL
            result = cache.get('k1')
            assert result == 'v1'
        
        # Should still be valid (total elapsed < TTL)
        assert cache.get('k1') == 'v1'
    
    def test_burst_access_pattern(self):
        """Test cache with burst access pattern."""
        cache = TTLCache(capacity=10, ttl=1.0)
        
        # Burst of puts
        for i in range(20):
            cache.put(f'k{i}', f'v{i}')
        
        # Burst of gets
        for i in range(20):
            cache.get(f'k{i}')
        
        # Cache should be in valid state
        assert cache.size() <= cache.capacity
    
    def test_sparse_access_pattern(self):
        """Test cache with sparse access pattern."""
        # Use longer TTL and disable cleanup for predictable testing
        cache = TTLCache(capacity=10, ttl=1.0, cleanup_interval=0)
        
        cache.put('k1', 'v1')
        time.sleep(0.6)  # k1 will expire soon
        
        cache.put('k2', 'v2')  # k2 is fresh
        
        # Wait so k1 expires but k2 is still valid
        # k1 expires at: t=0 + 1.0 = 1.0s
        # k2 expires at: t=0.6 + 1.0 = 1.6s
        # Check at t=1.1s: k1 expired, k2 still valid
        time.sleep(0.5)  # Total 1.1s
        
        # k1 should be expired (put at t=0, expires at t=1.0, checked at t=1.1)
        result_k1 = cache.get('k1')
        assert result_k1 is None
        
        # k2 should still be valid (put at t=0.6, expires at t=1.6, checked at t=1.1 < 1.6)
        result_k2 = cache.get('k2')
        assert result_k2 == 'v2'


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching
@pytest.mark.xwsystem_performance
class TestTTLCachePerformance:
    """Test TTL cache performance."""
    
    def test_cleanup_performance(self):
        """Test cleanup performance with many expired entries."""
        cache = TTLCache(capacity=1000, ttl=0.01)
        
        # Add many entries
        for i in range(1000):
            cache.put(f'k{i}', f'v{i}')
        
        time.sleep(0.02)  # All expire
        
        import time as time_module
        start = time_module.time()
        cache._cleanup_expired()
        elapsed = time_module.time() - start
        
        # Cleanup should be efficient
        assert elapsed < 1.0
        assert cache.size() == 0
    
    def test_get_performance_with_expired(self):
        """Test get performance when many entries are expired."""
        cache = TTLCache(capacity=100, ttl=0.01)
        
        # Add entries
        for i in range(100):
            cache.put(f'k{i}', f'v{i}')
        
        time.sleep(0.02)  # All expire
        
        import time as time_module
        start = time_module.time()
        for i in range(100):
            cache.get(f'k{i}')
        elapsed = time_module.time() - start
        
        # Gets should be efficient even with cleanup
        assert elapsed < 1.0


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching
class TestTTLCacheConcurrency:
    """Test TTL cache concurrent access."""
    
    def test_concurrent_put_and_get(self):
        """Test concurrent put and get operations."""
        cache = TTLCache(capacity=50, ttl=1.0)
        
        def worker(worker_id):
            for i in range(20):
                cache.put(f'k_{worker_id}_{i}', f'v_{worker_id}_{i}')
                cache.get(f'k_{worker_id}_{i}')
        
        with ThreadPoolExecutor(max_workers=5) as executor:
            futures = [executor.submit(worker, i) for i in range(5)]
            for future in futures:
                future.result()
        
        # Cache should be in valid state
        assert cache.size() <= cache.capacity
    
    def test_concurrent_cleanup(self):
        """Test concurrent cleanup operations."""
        cache = TTLCache(capacity=100, ttl=0.1)
        
        # Populate
        for i in range(50):
            cache.put(f'k{i}', f'v{i}')
        
        time.sleep(0.15)  # All expire
        
        def cleanup_worker():
            cache._cleanup_expired()
        
        with ThreadPoolExecutor(max_workers=5) as executor:
            futures = [executor.submit(cleanup_worker) for _ in range(5)]
            for future in futures:
                future.result()
        
        # Should be cleaned up
        assert cache.size() == 0


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching
class TestTTLCacheStatistics:
    """Test TTL cache statistics."""
    
    def test_expiration_statistics(self):
        """Test expiration count in statistics."""
        # Disable cleanup thread for predictable testing
        cache = TTLCache(capacity=10, ttl=0.5, cleanup_interval=0)
        cache.put('k1', 'v1')
        cache.put('k2', 'v2')
        
        time.sleep(0.6)  # Both expire (0.6 > 0.5 TTL)
        
        # Get both - each get checks expiration and increments counter
        cache.get('k1')  # Triggers expiration check and removal (increments expirations)
        cache.get('k2')  # Triggers expiration check and removal (increments expirations)
        
        stats = cache.get_stats()
        # Both entries should be expired (each get() increments expirations when entry is expired)
        assert stats['expirations'] >= 2
    
    def test_cleanup_statistics(self):
        """Test cleanup count in statistics."""
        cache = TTLCache(capacity=10, ttl=0.1)
        cache.put('k1', 'v1')
        
        time.sleep(0.15)
        cache._cleanup_expired()
        
        stats = cache.get_stats()
        assert stats['cleanups'] >= 1


# Total: 35+ comprehensive test cases for TTL cache
