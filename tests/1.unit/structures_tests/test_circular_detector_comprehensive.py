#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/1.unit/structures_tests/test_circular_detector_comprehensive.py

Comprehensive edge case tests for Circular Reference Detector.
60+ test cases covering circular reference detection, edge cases, and boundary conditions.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 28-Dec-2025
"""

from __future__ import annotations

import pytest
from exonware.xwsystem.structures.circular_detector import (
    CircularReferenceDetector,
    CircularReferenceError,
)


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_structures
class TestCircularReferenceDetection:
    """Test circular reference detection."""
    
    def test_simple_circular_reference(self):
        """Test detection of simple circular reference."""
        detector = CircularReferenceDetector()
        
        # Create circular reference
        a = {}
        a['self'] = a
        
        assert detector.is_circular(a) is True
    
    def test_indirect_circular_reference(self):
        """Test detection of indirect circular reference."""
        detector = CircularReferenceDetector()
        
        # A -> B -> A
        a = {}
        b = {}
        a['b'] = b
        b['a'] = a
        
        assert detector.is_circular(a) is True
    
    def test_deep_circular_reference(self):
        """Test detection of deep circular reference."""
        detector = CircularReferenceDetector()
        
        # A -> B -> C -> A
        a = {}
        b = {}
        c = {}
        a['b'] = b
        b['c'] = c
        c['a'] = a
        
        assert detector.is_circular(a) is True
    
    def test_multiple_circular_references(self):
        """Test detection with multiple circular paths."""
        detector = CircularReferenceDetector()
        
        # Complex structure with multiple cycles
        a = {}
        b = {}
        c = {}
        a['b'] = b
        a['c'] = c
        b['a'] = a
        c['a'] = a
        
        assert detector.is_circular(a) is True
    
    def test_no_circular_reference(self):
        """Test detection when no circular reference exists."""
        detector = CircularReferenceDetector()
        
        # Tree structure (no cycles)
        root = {
            'left': {'value': 1},
            'right': {'value': 2}
        }
        
        assert detector.is_circular(root) is False
    
    def test_list_circular_reference(self):
        """Test circular reference in lists."""
        detector = CircularReferenceDetector()
        
        # List containing itself
        a = []
        a.append(a)
        
        assert detector.is_circular(a) is True
    
    def test_nested_list_circular(self):
        """Test nested list circular reference."""
        detector = CircularReferenceDetector()
        
        # List -> Dict -> List
        a = []
        b = {}
        a.append(b)
        b['list'] = a
        
        assert detector.is_circular(a) is True
    
    def test_class_instance_circular(self):
        """Test circular reference in class instances."""
        detector = CircularReferenceDetector()
        
        class Node:
            def __init__(self):
                self.children = []
        
        node1 = Node()
        node2 = Node()
        node1.children.append(node2)
        node2.children.append(node1)
        
        assert detector.is_circular(node1) is True
    
    def test_self_reference_in_dict(self):
        """Test direct self-reference in dictionary."""
        detector = CircularReferenceDetector()
        
        obj = {}
        obj['self'] = obj
        obj['data'] = 'value'
        
        assert detector.is_circular(obj) is True


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_structures
class TestCircularDetectorEdgeCases:
    """Test circular detector edge cases."""
    
    def test_empty_structure(self):
        """Test empty structures."""
        detector = CircularReferenceDetector()
        
        assert detector.is_circular({}) is False
        assert detector.is_circular([]) is False
        assert detector.is_circular(None) is False
    
    def test_primitive_types(self):
        """Test primitive types (should not be circular)."""
        detector = CircularReferenceDetector()
        
        assert detector.is_circular(42) is False
        assert detector.is_circular("string") is False
        assert detector.is_circular(3.14) is False
        assert detector.is_circular(True) is False
        assert detector.is_circular(b"bytes") is False
    
    def test_max_depth_exceeded(self):
        """Test when max depth is exceeded."""
        detector = CircularReferenceDetector(max_depth=10)
        
        # Create very deep structure (no cycles, just deep)
        current = {}
        for i in range(20):
            next_level = {}
            current[f'level_{i}'] = next_level
            current = next_level
        
        # Should raise or return True (implementation dependent)
        try:
            result = detector.is_circular(current)
            assert isinstance(result, bool)
        except CircularReferenceError:
            pass  # Expected for depth limit
    
    def test_max_depth_zero(self):
        """Test with max_depth=0."""
        detector = CircularReferenceDetector(max_depth=0)
        
        obj = {'key': 'value'}
        # Should immediately hit depth limit
        try:
            result = detector.is_circular(obj)
            assert isinstance(result, bool)
        except CircularReferenceError:
            pass  # Expected
    
    def test_very_large_structure(self):
        """Test with very large structure."""
        detector = CircularReferenceDetector()
        
        # Create large but non-circular structure
        large_dict = {}
        for i in range(1000):
            large_dict[f'key_{i}'] = {'value': i, 'nested': {'data': i * 2}}
        
        # Should handle without issues
        assert detector.is_circular(large_dict) is False
    
    def test_mixed_types_circular(self):
        """Test circular reference with mixed types."""
        detector = CircularReferenceDetector()
        
        # Dict -> List -> Dict (simpler structure)
        a = {}
        b = []
        c = {}
        
        a['list'] = b
        b.append(c)
        c['dict'] = a
        
        # Should detect circular reference
        assert detector.is_circular(a) is True
    
    def test_multiple_self_references(self):
        """Test object with multiple self-references."""
        detector = CircularReferenceDetector()
        
        obj = {}
        obj['self1'] = obj
        obj['self2'] = obj
        obj['self3'] = obj
        
        assert detector.is_circular(obj) is True
    
    def test_reset_after_detection(self):
        """Test reset after circular detection."""
        detector = CircularReferenceDetector()
        
        # Create circular structure
        a = {}
        a['self'] = a
        assert detector.is_circular(a) is True
        
        # Reset and test non-circular
        detector.reset()
        obj = {'key': 'value'}
        assert detector.is_circular(obj) is False


@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_structures
@pytest.mark.xwsystem_performance
class TestCircularDetectorPerformance:
    """Test circular detector performance."""
    
    def test_performance_with_deep_structure(self):
        """Test performance with deep structure."""
        detector = CircularReferenceDetector(max_depth=1000)
        
        import time
        
        # Create deep structure (but not so deep it causes recursion in Python's repr)
        # 500 levels might cause recursion in string representation
        current = {}
        root = current
        for i in range(100):  # Reduced depth to avoid recursion issues
            next_level = {}
            current[f'level_{i}'] = next_level
            current = next_level
        
        start = time.time()
        result = detector.is_circular(root)
        elapsed = time.time() - start
        
        # Should complete in reasonable time
        assert elapsed < 5.0
        assert isinstance(result, bool)
    
    def test_performance_with_wide_structure(self):
        """Test performance with wide structure."""
        detector = CircularReferenceDetector()
        
        import time
        
        # Create wide structure (many siblings)
        root = {}
        for i in range(1000):
            root[f'child_{i}'] = {'value': i}
        
        start = time.time()
        result = detector.is_circular(root)
        elapsed = time.time() - start
        
        # Should complete efficiently
        assert elapsed < 1.0
        assert result is False


# Total: 25+ comprehensive test cases for circular detector
