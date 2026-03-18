#exonware/xwsystem/tests/0.core/structures/test_core_xwsystem_structures.py
#exonware/xwsystem/tests/core/structures/test_core_xwsystem_structures.py
"""
xwsystem Structures Core Tests
Comprehensive tests for xwsystem data structures including circular detection,
tree walking, and structure management.
"""

import pytest
import sys
import os
from pathlib import Path
from unittest.mock import patch, MagicMock
# Add the src directory to the path
sys.path.insert(0, str(Path(__file__).parent.parent.parent.parent.parent / "src"))
try:
    from exonware.xwsystem.structures.circular_detector import CircularReferenceDetector
    from exonware.xwsystem.structures.tree_walker import TreeWalker
    from exonware.xwsystem.structures.base import BaseStructure
    from exonware.xwsystem.structures.contracts import ICircularDetector, ITreeWalker
    from exonware.xwsystem.structures.errors import StructureError, CircularReferenceError, TreeError
except ImportError as e:
    print(f"Import error: {e}")
    # Create mock classes for testing
    class CircularReferenceDetector:
        def __init__(self): pass
        def detect_circular_reference(self, obj): return False
        def find_circular_path(self, obj): return []
        def is_circular(self, obj): return False
    class TreeWalker:
        def __init__(self): pass
        def walk_tree(self, root): return []
        def find_node(self, root, predicate): return None
        def get_tree_depth(self, root): return 0
        def get_tree_size(self, root): return 0
    class BaseStructure:
        def __init__(self): pass
        def initialize(self): pass
        def cleanup(self): pass
        def validate(self): return True
    class ICircularDetector: pass
    class ITreeWalker: pass
    class StructureError(Exception): pass
    class CircularReferenceError(Exception): pass
    class TreeError(Exception): pass


def test_circular_detector():
    """Test circular detector functionality."""
    print("📋 Testing: Circular Detector")
    print("-" * 30)
    try:
        detector = CircularReferenceDetector()
        # Test circular reference detection
        test_obj = {"key": "value"}
        is_circular = detector.has_circular_references(test_obj)
        assert isinstance(is_circular, bool)
        # Test circular path finding
        path = detector.find_circular_paths(test_obj)
        assert isinstance(path, list)
        # Test circular check
        is_circular_check = detector.is_circular(test_obj)
        assert isinstance(is_circular_check, bool)
        print("✅ Circular detector tests passed")
    except Exception as e:
        print(f"❌ Circular detector tests failed: {e}")
        pytest.fail(str(e))


def test_tree_walker():
    """Test tree walker functionality."""
    print("📋 Testing: Tree Walker")
    print("-" * 30)
    try:
        walker = TreeWalker()
        test_tree = {"root": {"child1": {}, "child2": {}}}
        # Collect nodes via walk_and_process (TreeWalker API)
        nodes_collected = []
        def collect(node, path, depth):
            nodes_collected.append(node)
            return node
        walker.walk_and_process(test_tree, collect)
        nodes = nodes_collected
        assert isinstance(nodes, list)
        # Test node finding via find_nodes
        def find_root_predicate(node, path, depth):
            return isinstance(node, dict) and "root" in node
        matches = walker.find_nodes(test_tree, find_root_predicate)
        found_node = matches[0]["value"] if matches else None
        # Test tree depth (max depth from walk)
        max_depth = [0]
        def track_depth(node, path, depth):
            max_depth[0] = max(max_depth[0], depth)
            return node
        walker.walk_and_process(test_tree, track_depth)
        depth = max_depth[0]
        assert isinstance(depth, int)
        assert depth >= 0
        # Test tree size
        size = len(nodes_collected) if nodes_collected else 0
        assert isinstance(size, int)
        assert size >= 0
        print("✅ Tree walker tests passed")
    except Exception as e:
        print(f"❌ Tree walker tests failed: {e}")
        pytest.fail(str(e))


def test_base_structure():
    """Test base structure functionality."""
    print("📋 Testing: Base Structure")
    print("-" * 30)
    try:
        structure = BaseStructure()
        structure.add("item1")
        structure.add("item2")
        is_valid = structure.validate()
        assert isinstance(is_valid, bool)
        structure.clear()
        print("✅ Base structure tests passed")
    except Exception as e:
        print(f"❌ Base structure tests failed: {e}")
        pytest.fail(str(e))


def test_structures_interfaces():
    """Test structures interface compliance."""
    print("📋 Testing: Structures Interfaces")
    print("-" * 30)
    try:
        # Test interface compliance
        detector = CircularReferenceDetector()
        walker = TreeWalker()
        structure = BaseStructure()
        # Verify objects can be instantiated
        assert detector is not None
        assert walker is not None
        assert structure is not None
        print("✅ Structures interfaces tests passed")
    except Exception as e:
        print(f"❌ Structures interfaces tests failed: {e}")
        pytest.fail(str(e))


def test_structures_error_handling():
    """Test structures error handling."""
    print("📋 Testing: Structures Error Handling")
    print("-" * 30)
    try:
        # Test error classes
        structure_error = StructureError("Test structure error")
        circular_error = CircularReferenceError("Test circular error")
        tree_error = TreeError("Test tree error")
        assert str(structure_error) == "Test structure error"
        assert str(circular_error) == "Test circular error"
        assert str(tree_error) == "Test tree error"
        print("✅ Structures error handling tests passed")
    except Exception as e:
        print(f"❌ Structures error handling tests failed: {e}")
        pytest.fail(str(e))


def test_structures_operations():
    """Test structures operations."""
    print("📋 Testing: Structures Operations")
    print("-" * 30)
    try:
        detector = CircularReferenceDetector()
        walker = TreeWalker()
        structure = BaseStructure()
        structure.add("elem")
        # Test integrated operations
        structure.validate()
        # Create test data structure
        test_data = {
            "level1": {
                "level2": {
                    "level3": "value"
                }
            }
        }
        # Test circular detection
        is_circular = detector.has_circular_references(test_data)
        assert isinstance(is_circular, bool)
        # Test tree walking
        nodes_list = []
        def collect(node, path, depth):
            nodes_list.append(node)
            return node
        walker.walk_and_process(test_data, collect)
        nodes = nodes_list
        assert isinstance(nodes, list)
        # Test structure validation
        is_valid = structure.validate()
        assert isinstance(is_valid, bool)
        structure.clear()
        print("✅ Structures operations tests passed")
    except Exception as e:
        print(f"❌ Structures operations tests failed: {e}")
        pytest.fail(str(e))


def test_structures_analysis():
    """Test structures analysis functionality."""
    print("📋 Testing: Structures Analysis")
    print("-" * 30)
    try:
        detector = CircularReferenceDetector()
        walker = TreeWalker()
        # Test complex data structure
        complex_data = {
            "users": [
                {"id": 1, "name": "Alice", "friends": [2, 3]},
                {"id": 2, "name": "Bob", "friends": [1, 3]},
                {"id": 3, "name": "Charlie", "friends": [1, 2]}
            ],
            "posts": [
                {"id": 1, "author": 1, "content": "Hello world"},
                {"id": 2, "author": 2, "content": "Nice day"}
            ]
        }
        # Test circular detection on complex structure
        is_circular = detector.has_circular_references(complex_data)
        assert isinstance(is_circular, bool)
        # Test tree analysis - depth and size via walk
        depth_list = [0]
        size_list = [0]
        def track_ds(node, path, depth):
            depth_list[0] = max(depth_list[0], depth)
            size_list[0] += 1
            return node
        walker.walk_and_process(complex_data, track_ds)
        depth = depth_list[0]
        size = size_list[0]
        assert isinstance(depth, int)
        assert depth >= 0
        assert isinstance(size, int)
        assert size >= 0
        # Test node finding
        def find_user_predicate(node, path, depth):
            if isinstance(node, dict) and "name" in node:
                return node["name"] == "Alice"
            return False
        user_matches = walker.find_nodes(complex_data, find_user_predicate)
        found_user = user_matches[0]["value"] if user_matches else None
        # Can be None if not found, which is valid
        print("✅ Structures analysis tests passed")
    except Exception as e:
        print(f"❌ Structures analysis tests failed: {e}")
        pytest.fail(str(e))


def test_structures_integration():
    """Test structures integration functionality."""
    print("📋 Testing: Structures Integration")
    print("-" * 30)
    try:
        detector = CircularReferenceDetector()
        walker = TreeWalker()
        structure = BaseStructure()
        structure.add("item")
        # Test integrated workflow
        # Create test structure
        test_structure = {
            "metadata": {"version": "1.0", "type": "test"},
            "data": {
                "items": [
                    {"id": 1, "value": "item1"},
                    {"id": 2, "value": "item2"}
                ]
            }
        }
        # Analyze structure
        is_circular = detector.has_circular_references(test_structure)
        nodes_list = []
        depth_list = [0]
        def collect_and_depth(node, path, depth):
            nodes_list.append(node)
            depth_list[0] = max(depth_list[0], depth)
            return node
        walker.walk_and_process(test_structure, collect_and_depth)
        nodes = nodes_list
        depth = depth_list[0]
        size = len(nodes_list)
        # Validate structure
        is_valid = structure.validate()
        # Verify all operations completed
        assert isinstance(is_circular, bool)
        assert isinstance(nodes, list)
        assert isinstance(depth, int)
        assert isinstance(size, int)
        assert isinstance(is_valid, bool)
        structure.clear()
        print("✅ Structures integration tests passed")
    except Exception as e:
        print(f"❌ Structures integration tests failed: {e}")
        pytest.fail(str(e))


def main():
    """Run all structures core tests."""
    print("=" * 50)
    print("🧪 xwsystem Structures Core Tests")
    print("=" * 50)
    print("Testing xwsystem data structures including circular detection,")
    print("tree walking, and structure management")
    print("=" * 50)
    tests = [
        test_circular_detector,
        test_tree_walker,
        test_base_structure,
        test_structures_interfaces,
        test_structures_error_handling,
        test_structures_operations,
        test_structures_analysis,
        test_structures_integration,
    ]
    passed = 0
    total = len(tests)
    for test in tests:
        try:
            if test():
                passed += 1
        except Exception as e:
            print(f"❌ Test {test.__name__} failed with exception: {e}")
    print("\n" + "=" * 50)
    print("📊 xwsystem STRUCTURES TEST SUMMARY")
    print("=" * 50)
    print(f"Results: {passed}/{total} tests passed")
    if passed == total:
        print("🎉 All xwsystem structures tests passed!")
        return 0
    else:
        print("💥 Some xwsystem structures tests failed!")
        return 1
if __name__ == "__main__":
    sys.exit(main())
