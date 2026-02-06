#!/usr/bin/env python3
#exonware/xwsystem/examples/serialization_example/x4_patching/benchmark.py
"""
JSON Patch Operations Benchmark

Tests atomic patch operations without full deserialization:
- RFC 6902 (JSON Patch)
- RFC 7386 (JSON Merge Patch)
- Batch operations
- Database-like updates

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Generation Date: October 12, 2025
"""

import sys
import time
from pathlib import Path

# Add common to path
parent_dir = Path(__file__).parent.parent
sys.path.insert(0, str(parent_dir))

# Add xwsystem to path
xwsystem_root = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(xwsystem_root))

from x0_common import generate_sample_data, print_section, save_to_file
from exonware.xwsystem.serialization import JsonSerializer


def test_rfc6902_patch():
    """Test RFC 6902 JSON Patch"""
    print("\n### RFC 6902 JSON Patch ###")
    
    serializer = JsonSerializer()
    data = generate_sample_data()
    json_data = serializer.dumps(data)
    
    data_dir = Path(__file__).parent / "data"
    data_dir.mkdir(exist_ok=True)
    save_to_file(json_data, data_dir / "original.json")
    
    # Define patch operations
    patch = [
        {'op': 'replace', 'path': '/users/1/age', 'value': 26},
        {'op': 'add', 'path': '/users/-', 'value': {'id': 4, 'name': 'David', 'email': 'david@example.com', 'age': 28, 'active': True, 'roles': ['user']}},
        {'op': 'remove', 'path': '/users/0/roles/0'},
        {'op': 'replace', 'path': '/metadata/version', 'value': '2.0'}
    ]
    
    print(f"\nApplying {len(patch)} patch operations:")
    for op in patch:
        print(f"  - {op['op']}: {op['path']}")
    
    try:
        start = time.perf_counter()
        patched = serializer.apply_patch(json_data, patch, rfc="6902")
        elapsed = (time.perf_counter() - start) * 1000
        
        print(f"\n  ✓ Patched in {elapsed:.2f}ms")
        
        # Verify changes
        patched_data = serializer.loads(patched)
        print(f"  ✓ Bob's age updated: {patched_data['users'][1]['age']}")
        print(f"  ✓ Total users now: {len(patched_data['users'])}")
        print(f"  ✓ Version updated: {patched_data['metadata']['version']}")
        
        save_to_file(patched, data_dir / "patched.json")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def test_rfc7386_merge():
    """Test RFC 7386 JSON Merge Patch"""
    print("\n### RFC 7386 JSON Merge Patch ###")
    
    serializer = JsonSerializer()
    data = generate_sample_data()
    json_data = serializer.dumps(data)
    
    # Merge patch
    merge_patch = {
        'users': [
            {'id': 1, 'age': 31}  # Update Alice's age
        ],
        'metadata': {
            'version': '2.0',
            'updated': '2025-10-12'
        }
    }
    
    print("\nApplying merge patch...")
    
    try:
        start = time.perf_counter()
        merged = serializer.apply_patch(json_data, merge_patch, rfc="7386")
        elapsed = (time.perf_counter() - start) * 1000
        
        print(f"  ✓ Merged in {elapsed:.2f}ms")
        
        merged_data = serializer.loads(merged)
        print(f"  ✓ Metadata updated: {merged_data['metadata']}")
        
        data_dir = Path(__file__).parent / "data"
        save_to_file(merged, data_dir / "merged.json")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def test_batch_patches():
    """Test batch patch operations"""
    print("\n### Batch Patch Operations ###")
    
    serializer = JsonSerializer()
    data = generate_sample_data()
    
    # Multiple patch batches
    patches = [
        [{'op': 'replace', 'path': '/users/0/active', 'value': False}],
        [{'op': 'replace', 'path': '/users/1/active', 'value': False}],
        [{'op': 'replace', 'path': '/users/2/active', 'value': True}],
    ]
    
    print(f"\nApplying {len(patches)} batches...")
    
    try:
        json_data = serializer.dumps(data)
        
        start = time.perf_counter()
        for i, patch in enumerate(patches):
            json_data = serializer.apply_patch(json_data, patch, rfc="6902")
        elapsed = (time.perf_counter() - start) * 1000
        
        result = serializer.loads(json_data)
        active_count = sum(1 for u in result['users'] if u.get('active'))
        
        print(f"  ✓ Applied {len(patches)} batches in {elapsed:.2f}ms")
        print(f"  ✓ Active users: {active_count}/{len(result['users'])}")
        
        data_dir = Path(__file__).parent / "data"
        save_to_file(json_data, data_dir / "batch_patched.json")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def main():
    """Main benchmark runner"""
    print_section("JSON PATCH OPERATIONS BENCHMARK")
    
    print("\nJSON Patch enables atomic updates without full deserialization!")
    print("Perfect for database-like operations on serialized files.")
    
    test_rfc6902_patch()
    test_rfc7386_merge()
    test_batch_patches()
    
    print_section("SUMMARY")
    print("\n✓ JSON Patch enables:")
    print("  - Atomic updates without full load/save")
    print("  - Database-like operations on files")
    print("  - Version control and diff operations")
    print("  - Efficient partial updates")
    
    data_dir = Path(__file__).parent / "data"
    print(f"\n✓ Output saved to: {data_dir}")


if __name__ == "__main__":
    main()
