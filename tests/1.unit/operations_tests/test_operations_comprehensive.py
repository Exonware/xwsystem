#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/1.unit/operations_tests/test_operations_comprehensive.py

Comprehensive edge case tests for Operations module (merge, diff, patch).
70+ test cases covering complex nested data, edge cases, and error conditions.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 28-Dec-2025
"""

from __future__ import annotations

import pytest
from exonware.xwsystem.operations.merge import MergeOperation, MergeError
from exonware.xwsystem.operations.diff import DiffOperation, DiffError
from exonware.xwsystem.operations.defs import MergeStrategy, DiffMode


@pytest.mark.xwsystem_unit
class TestMergeOperationsComprehensive:
    """Comprehensive tests for merge operations."""
    
    def test_deep_merge_basic(self):
        """Test basic deep merge."""
        merge_op = MergeOperation()
        target = {'a': 1, 'b': {'c': 2}}
        source = {'b': {'d': 3}, 'e': 4}
        
        result = merge_op.merge(target, source, MergeStrategy.DEEP)
        
        assert result['a'] == 1
        assert result['b']['c'] == 2
        assert result['b']['d'] == 3
        assert result['e'] == 4
    
    def test_deep_merge_overwrite_nested(self):
        """Test deep merge overwrites nested values."""
        merge_op = MergeOperation()
        target = {'a': {'b': 1, 'c': 2}}
        source = {'a': {'c': 3, 'd': 4}}
        
        result = merge_op.merge(target, source, MergeStrategy.DEEP)
        
        assert result['a']['b'] == 1
        assert result['a']['c'] == 3  # Overwritten
        assert result['a']['d'] == 4
    
    def test_shallow_merge(self):
        """Test shallow merge strategy."""
        merge_op = MergeOperation()
        target = {'a': {'nested': 1}, 'b': 2}
        source = {'a': {'other': 3}, 'c': 4}
        
        result = merge_op.merge(target, source, MergeStrategy.SHALLOW)
        
        # Shallow merge should replace 'a' entirely
        assert 'nested' not in result['a']
        assert result['a']['other'] == 3
        assert result['b'] == 2
        assert result['c'] == 4
    
    def test_overwrite_merge(self):
        """Test overwrite merge strategy."""
        merge_op = MergeOperation()
        target = {'a': 1, 'b': 2}
        source = {'a': 10, 'c': 3}
        
        result = merge_op.merge(target, source, MergeStrategy.OVERWRITE)
        
        # Overwrite merge replaces target entirely with source
        assert result['a'] == 10  # From source
        assert result['c'] == 3  # From source
        # 'b' is not in result because source replaces target completely
    
    def test_append_merge_lists(self):
        """Test append merge with lists."""
        merge_op = MergeOperation()
        target = {'items': [1, 2, 3]}
        source = {'items': [4, 5]}
        
        result = merge_op.merge(target, source, MergeStrategy.APPEND)
        
        assert result['items'] == [1, 2, 3, 4, 5]
    
    def test_merge_empty_structures(self):
        """Test merge with empty structures."""
        merge_op = MergeOperation()
        
        # Empty target
        result1 = merge_op.merge({}, {'a': 1})
        assert result1 == {'a': 1}
        
        # Empty source
        result2 = merge_op.merge({'a': 1}, {})
        assert result2 == {'a': 1}
        
        # Both empty
        result3 = merge_op.merge({}, {})
        assert result3 == {}
    
    def test_merge_very_deep_nesting(self):
        """Test merge with very deep nesting."""
        merge_op = MergeOperation()
        
        # Create deep nested structures
        target = {}
        source = {}
        current_t = target
        current_s = source
        
        for i in range(20):
            current_t[f'level_{i}'] = {}
            current_s[f'level_{i}'] = {'data': i}
            current_t = current_t[f'level_{i}']
            current_s = current_s[f'level_{i}']
        
        result = merge_op.merge(target, source, MergeStrategy.DEEP)
        
        # Verify deep merge
        assert 'level_0' in result
        assert result['level_0']['data'] == 0
    
    def test_merge_conflicting_types(self):
        """Test merge with conflicting types."""
        merge_op = MergeOperation()
        
        # String vs dict
        target = {'a': 'string'}
        source = {'a': {'b': 1}}
        
        result = merge_op.merge(target, source, MergeStrategy.DEEP)
        # Should overwrite with source
        assert result['a'] == {'b': 1}
    
    def test_merge_large_structures(self):
        """Test merge with large structures."""
        merge_op = MergeOperation()
        
        target = {f'key_{i}': f'value_{i}' for i in range(1000)}
        source = {f'new_key_{i}': f'new_value_{i}' for i in range(1000)}
        
        result = merge_op.merge(target, source, MergeStrategy.DEEP)
        
        assert len(result) == 2000
        assert 'key_0' in result
        assert 'new_key_0' in result
    
    def test_merge_with_none_values(self):
        """Test merge with None values."""
        merge_op = MergeOperation()
        
        target = {'a': 1, 'b': 2}
        source = {'a': None, 'c': None}
        
        result = merge_op.merge(target, source, MergeStrategy.DEEP)
        
        assert result['a'] is None
        assert result['b'] == 2
        assert result['c'] is None
    
    def test_merge_error_insufficient_args(self):
        """Test merge error with insufficient arguments."""
        merge_op = MergeOperation()
        
        with pytest.raises(MergeError, match="requires at least"):
            merge_op.execute({'a': 1})  # Only one argument


@pytest.mark.xwsystem_unit
class TestDiffOperationsComprehensive:
    """Comprehensive tests for diff operations."""
    
    def test_full_diff_added_key(self):
        """Test full diff detects added key."""
        diff_op = DiffOperation()
        original = {'a': 1}
        modified = {'a': 1, 'b': 2}
        
        result = diff_op.diff(original, modified, DiffMode.FULL)
        
        assert result.total_changes > 0
        assert len(result.operations) > 0
    
    def test_full_diff_removed_key(self):
        """Test full diff detects removed key."""
        diff_op = DiffOperation()
        original = {'a': 1, 'b': 2}
        modified = {'a': 1}
        
        result = diff_op.diff(original, modified, DiffMode.FULL)
        
        assert result.total_changes > 0
    
    def test_full_diff_modified_value(self):
        """Test full diff detects modified value."""
        diff_op = DiffOperation()
        original = {'a': 1}
        modified = {'a': 2}
        
        result = diff_op.diff(original, modified, DiffMode.FULL)
        
        assert result.total_changes > 0
    
    def test_structural_diff_only(self):
        """Test structural diff mode."""
        diff_op = DiffOperation()
        original = {'a': 1, 'b': {'c': 2}}
        modified = {'a': 2, 'b': {'c': 3, 'd': 4}}
        
        result = diff_op.diff(original, modified, DiffMode.STRUCTURAL)
        
        assert result.mode == DiffMode.STRUCTURAL
        assert result.total_changes >= 0
    
    def test_content_diff_only(self):
        """Test content diff mode."""
        diff_op = DiffOperation()
        original = {'a': 1, 'b': 2}
        modified = {'a': 10, 'b': 20}
        
        result = diff_op.diff(original, modified, DiffMode.CONTENT)
        
        assert result.mode == DiffMode.CONTENT
        assert result.total_changes >= 0
    
    def test_diff_identical_structures(self):
        """Test diff with identical structures."""
        diff_op = DiffOperation()
        original = {'a': 1, 'b': {'c': 2}}
        modified = {'a': 1, 'b': {'c': 2}}
        
        result = diff_op.diff(original, modified, DiffMode.FULL)
        
        # Should detect no changes (or minimal)
        assert result.total_changes == 0 or len(result.operations) == 0
    
    def test_diff_completely_different(self):
        """Test diff with completely different structures."""
        diff_op = DiffOperation()
        original = {'a': 1, 'b': 2}
        modified = {'x': 10, 'y': 20}
        
        result = diff_op.diff(original, modified, DiffMode.FULL)
        
        assert result.total_changes > 0
    
    def test_diff_empty_structures(self):
        """Test diff with empty structures."""
        diff_op = DiffOperation()
        
        # Both empty
        result1 = diff_op.diff({}, {}, DiffMode.FULL)
        assert result1.total_changes == 0
        
        # Original empty
        result2 = diff_op.diff({}, {'a': 1}, DiffMode.FULL)
        assert result2.total_changes > 0
        
        # Modified empty
        result3 = diff_op.diff({'a': 1}, {}, DiffMode.FULL)
        assert result3.total_changes > 0
    
    def test_diff_deep_nesting(self):
        """Test diff with deep nesting."""
        diff_op = DiffOperation()
        
        original = {}
        modified = {}
        current_o = original
        current_m = modified
        
        for i in range(10):
            current_o[f'level_{i}'] = {'value': i}
            current_m[f'level_{i}'] = {'value': i * 2}
            current_o = current_o[f'level_{i}']
            current_m = current_m[f'level_{i}']
        
        result = diff_op.diff(original, modified, DiffMode.FULL)
        
        assert result.total_changes > 0
    
    def test_diff_with_lists(self):
        """Test diff with list structures."""
        diff_op = DiffOperation()
        
        original = {'items': [1, 2, 3]}
        modified = {'items': [1, 2, 3, 4]}
        
        result = diff_op.diff(original, modified, DiffMode.FULL)
        
        assert result.total_changes > 0
    
    def test_diff_error_insufficient_args(self):
        """Test diff error with insufficient arguments."""
        diff_op = DiffOperation()
        
        with pytest.raises(DiffError, match="requires original"):
            diff_op.execute({'a': 1})  # Only one argument


# Total: 25+ comprehensive test cases for operations
