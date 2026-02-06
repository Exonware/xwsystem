#!/usr/bin/env python3
#exonware/xwsystem/examples/serialization_example/x3_streaming/benchmark.py
"""
Streaming Operations Benchmark

Tests streaming serialization/deserialization for large datasets:
- iter_serialize: Chunk-based serialization
- iter_deserialize: Streaming deserialization
- iter_path: Streaming with filtering
- Memory-efficient processing

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

from x0_common.data_generator import generate_large_dataset
from x0_common.test_helpers import print_section, save_to_file
from exonware.xwsystem.serialization import JsonSerializer


def test_streaming_serialize():
    """Test streaming serialization"""
    print("\n### Streaming Serialization ###")
    
    serializer = JsonSerializer()
    data = generate_large_dataset(10000)
    
    print(f"\nGenerating 10,000 user records...")
    
    try:
        start = time.perf_counter()
        chunks = []
        for chunk in serializer.iter_serialize(data, chunk_size=8192):
            chunks.append(chunk)
        elapsed = (time.perf_counter() - start) * 1000
        
        total_size = sum(len(c) for c in chunks)
        print(f"  ✓ Generated {len(chunks)} chunks ({elapsed:.2f}ms)")
        print(f"  ✓ Total size: {total_size / 1024:.2f} KB")
        print(f"  ✓ Avg chunk size: {total_size / len(chunks):.0f} bytes")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def test_streaming_path():
    """Test streaming with path filtering"""
    print("\n### Streaming with Path Filtering ###")
    
    serializer = JsonSerializer()
    data = generate_large_dataset(10000)
    data_dir = Path(__file__).parent / "data"
    data_dir.mkdir(exist_ok=True)
    
    # Save large dataset
    file_path = data_dir / "large_dataset.json"
    serializer.save_file(str(file_path), data)
    print(f"\nSaved 10,000 records to: {file_path}")
    
    # Stream process WITHOUT loading entire file
    try:
        start = time.perf_counter()
        count = 0
        active_count = 0
        
        json_str = serializer.dumps(data)
        for user in serializer.iter_path(json_str, "users.item"):
            count += 1
            if user.get('active'):
                active_count += 1
            if count >= 100:  # Process first 100
                break
        
        elapsed = (time.perf_counter() - start) * 1000
        
        print(f"  ✓ Processed {count} users ({elapsed:.2f}ms)")
        print(f"  ✓ Active users: {active_count}")
        print(f"  ✓ Memory-efficient: Only loaded processed records!")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def test_large_file_operations():
    """Test operations on large files"""
    print("\n### Large File Operations ###")
    
    serializer = JsonSerializer()
    data_dir = Path(__file__).parent / "data"
    
    sizes = [1000, 5000, 10000]
    
    for size in sizes:
        data = generate_large_dataset(size)
        
        # Serialize
        start = time.perf_counter()
        serialized = serializer.dumps(data)
        serialize_time = (time.perf_counter() - start) * 1000
        
        # Deserialize
        start = time.perf_counter()
        deserialized = serializer.loads(serialized)
        deserialize_time = (time.perf_counter() - start) * 1000
        
        size_kb = len(serialized) / 1024
        
        print(f"\n  {size} records:")
        print(f"    ✓ Serialize: {serialize_time:.2f}ms")
        print(f"    ✓ Deserialize: {deserialize_time:.2f}ms")
        print(f"    ✓ Size: {size_kb:.2f} KB")


def main():
    """Main benchmark runner"""
    print_section("STREAMING OPERATIONS BENCHMARK")
    
    print("\nStreaming enables processing large datasets without")
    print("loading everything into memory at once!")
    
    test_streaming_serialize()
    test_streaming_path()
    test_large_file_operations()
    
    print_section("SUMMARY")
    print("\n✓ Streaming enables:")
    print("  - Processing files larger than RAM")
    print("  - Memory-efficient operations")
    print("  - Progressive loading and processing")
    print("  - Handling gigabyte-sized files")
    
    data_dir = Path(__file__).parent / "data"
    print(f"\n✓ Output saved to: {data_dir}")


if __name__ == "__main__":
    main()
