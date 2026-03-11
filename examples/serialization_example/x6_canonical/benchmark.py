#!/usr/bin/env python3
#exonware/xwsystem/examples/serialization_example/x6_canonical/benchmark.py
"""
Canonical Serialization Benchmark
Tests deterministic canonical serialization and hashing:
- canonicalize(): Deterministic output
- hash_stable(): Content-based hashing
- checksum(): Data integrity
- Use cases: caching, deduplication, version control
Company: eXonware.com
Author: eXonware Backend Team
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
from exonware.xwsystem.serialization import JsonSerializer, YamlSerializer


def test_canonical_serialization():
    """Test canonical serialization"""
    print("\n### Canonical Serialization ###")
    serializer = JsonSerializer()
    # Same data, different key order
    data1 = {'c': 3, 'a': 1, 'b': 2}
    data2 = {'a': 1, 'b': 2, 'c': 3}
    print("\nOriginal data:")
    print(f"  data1: {data1}")
    print(f"  data2: {data2}")
    print("  (Same content, different order)")
    try:
        start = time.perf_counter()
        canonical1 = serializer.canonicalize(data1)
        canonical2 = serializer.canonicalize(data2)
        elapsed = (time.perf_counter() - start) * 1000
        print(f"\n  ✓ Canonical serialization: {elapsed:.2f}ms")
        print(f"  ✓ Canonical outputs match: {canonical1 == canonical2}")
        print(f"  ✓ Output: {canonical1[:50]}...")
        data_dir = Path(__file__).parent / "data"
        data_dir.mkdir(exist_ok=True)
        save_to_file(canonical1, data_dir / "canonical.json")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def test_stable_hashing():
    """Test stable hashing"""
    print("\n### Stable Hashing ###")
    serializer = JsonSerializer()
    data = generate_sample_data()
    print("\nGenerating stable hashes...")
    try:
        # Generate hash multiple times
        start = time.perf_counter()
        hash1 = serializer.hash_stable(data, algorithm="sha256")
        hash2 = serializer.hash_stable(data, algorithm="sha256")
        hash3 = serializer.hash_stable(data, algorithm="sha256")
        elapsed = (time.perf_counter() - start) * 1000
        print(f"  ✓ Generated 3 hashes in {elapsed:.2f}ms")
        print(f"  ✓ hash1: {hash1}")
        print(f"  ✓ hash2: {hash2}")
        print(f"  ✓ hash3: {hash3}")
        print(f"  ✓ All hashes match: {hash1 == hash2 == hash3}")
        # Test with modified data
        modified_data = data.copy()
        modified_data['metadata']['version'] = '2.0'
        hash_modified = serializer.hash_stable(modified_data, algorithm="sha256")
        print(f"\n  ✓ Modified data hash: {hash_modified}")
        print(f"  ✓ Hashes differ: {hash1 != hash_modified}")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def test_checksums():
    """Test checksum operations"""
    print("\n### Checksums ###")
    serializer = JsonSerializer()
    data = generate_sample_data()
    print("\nCalculating checksums...")
    try:
        # Serialize
        serialized = serializer.dumps(data)
        # Calculate checksums with different algorithms
        start = time.perf_counter()
        checksum_sha256 = serializer.checksum(data, algorithm="sha256")
        elapsed = (time.perf_counter() - start) * 1000
        print(f"  ✓ SHA256 checksum: {checksum_sha256} ({elapsed:.2f}ms)")
        # Verify checksum
        is_valid = serializer.verify_checksum(serialized, checksum_sha256, algorithm="sha256")
        print(f"  ✓ Checksum valid: {is_valid}")
        # Modify data and verify
        modified = serialized.replace(b'"Alice"', b'"Alicia"')
        is_valid_modified = serializer.verify_checksum(modified, checksum_sha256, algorithm="sha256")
        print(f"  ✓ Modified data checksum valid: {is_valid_modified}")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def test_use_cases():
    """Demonstrate use cases"""
    print("\n### Use Cases ###")
    serializer = JsonSerializer()
    # Use case 1: Content-based caching
    print("\n1. Content-Based Caching:")
    cache = {}
    data1 = {'user': 'Alice', 'action': 'login'}
    data2 = {'action': 'login', 'user': 'Alice'}  # Same content, different order
    try:
        hash1 = serializer.hash_stable(data1)
        hash2 = serializer.hash_stable(data2)
        cache[hash1] = "Result for Alice login"
        print(f"  ✓ Cached result under: {hash1[:16]}...")
        print(f"  ✓ Hash2 matches: {hash1 == hash2}")
        print(f"  ✓ Cache hit: {hash2 in cache}")
    except Exception as e:
        print(f"  ✗ Error: {e}")
    # Use case 2: Deduplication
    print("\n2. Deduplication:")
    seen_hashes = set()
    duplicates = 0
    items = [
        {'name': 'item1'},
        {'name': 'item2'},
        {'name': 'item1'},  # Duplicate
        {'name': 'item3'},
        {'name': 'item2'},  # Duplicate
    ]
    try:
        for item in items:
            hash_val = serializer.hash_stable(item)
            if hash_val in seen_hashes:
                duplicates += 1
            else:
                seen_hashes.add(hash_val)
        print(f"  ✓ Processed {len(items)} items")
        print(f"  ✓ Unique items: {len(seen_hashes)}")
        print(f"  ✓ Duplicates found: {duplicates}")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def main():
    """Main benchmark runner"""
    print_section("CANONICAL SERIALIZATION BENCHMARK")
    print("\nCanonical serialization ensures deterministic output")
    print("regardless of input order - perfect for hashing and caching!")
    test_canonical_serialization()
    test_stable_hashing()
    test_checksums()
    test_use_cases()
    print_section("SUMMARY")
    print("\n✓ Canonical serialization enables:")
    print("  - Content-based caching")
    print("  - Deduplication")
    print("  - Version control / diff operations")
    print("  - Integrity verification")
    print("  - Deterministic hashing")
    data_dir = Path(__file__).parent / "data"
    print(f"\n✓ Output saved to: {data_dir}")
if __name__ == "__main__":
    main()
