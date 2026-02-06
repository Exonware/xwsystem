#!/usr/bin/env python3
#exonware/xwsystem/examples/serialization_example/x1_basic_formats/benchmark.py
"""
Basic Serialization Formats Benchmark

Tests basic serialization/deserialization with multiple formats:
- JSON, YAML, XML, MessagePack, Pickle, CBOR, BSON, CSV, TOML

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
from exonware.xwsystem.serialization import (
    JsonSerializer, YamlSerializer, XmlSerializer,
    MsgPackSerializer, PickleSerializer, CborSerializer,
    BsonSerializer, CsvSerializer, TomlSerializer
)

def test_format(name: str, serializer, data, data_dir: Path):
    """Test a single serialization format"""
    print(f"\n### Testing {name} ###")
    
    try:
        # Serialize
        start = time.perf_counter()
        serialized = serializer.dumps(data)
        serialize_time = (time.perf_counter() - start) * 1000
        
        # Deserialize
        start = time.perf_counter()
        deserialized = serializer.loads(serialized)
        deserialize_time = (time.perf_counter() - start) * 1000
        
        # File operations
        ext = serializer.file_extensions[0] if hasattr(serializer, 'file_extensions') else f'.{name.lower()}'
        file_path = data_dir / f"sample{ext}"
        
        start = time.perf_counter()
        serializer.save_file(str(file_path), data)
        save_time = (time.perf_counter() - start) * 1000
        
        start = time.perf_counter()
        loaded = serializer.load_file(str(file_path))
        load_time = (time.perf_counter() - start) * 1000
        
        # Get size
        size_bytes = len(serialized) if isinstance(serialized, bytes) else len(serialized.encode())
        size_kb = size_bytes / 1024
        
        print(f"  ✓ Serialize: {serialize_time:.2f}ms")
        print(f"  ✓ Deserialize: {deserialize_time:.2f}ms")
        print(f"  ✓ Save file: {save_time:.2f}ms")
        print(f"  ✓ Load file: {load_time:.2f}ms")
        print(f"  ✓ Size: {size_kb:.2f} KB")
        print(f"  ✓ Data integrity: {loaded == data}")
        
        return {
            'name': name,
            'serialize_ms': serialize_time,
            'deserialize_ms': deserialize_time,
            'save_ms': save_time,
            'load_ms': load_time,
            'size_kb': size_kb,
            'success': True
        }
    
    except Exception as e:
        print(f"  ✗ Error: {e}")
        return {'name': name, 'success': False, 'error': str(e)}


def main():
    """Main benchmark runner"""
    print_section("BASIC SERIALIZATION FORMATS BENCHMARK")
    
    # Generate test data
    data = generate_sample_data()
    data_dir = Path(__file__).parent / "data"
    data_dir.mkdir(exist_ok=True)
    
    # Test all formats
    results = []
    
    formats = [
        ('JSON', JsonSerializer()),
        ('YAML', YamlSerializer()),
        ('XML', XmlSerializer()),
        ('MessagePack', MsgPackSerializer()),
        ('Pickle', PickleSerializer()),
        ('CBOR', CborSerializer()),
        ('BSON', BsonSerializer()),
        ('TOML', TomlSerializer()),
    ]
    
    for name, serializer in formats:
        result = test_format(name, serializer, data, data_dir)
        results.append(result)
    
    # Summary
    print_section("SUMMARY")
    
    successful = [r for r in results if r.get('success')]
    
    if successful:
        # Fastest serialization
        fastest_serialize = min(successful, key=lambda x: x['serialize_ms'])
        print(f"Fastest Serialize: {fastest_serialize['name']} ({fastest_serialize['serialize_ms']:.2f}ms)")
        
        # Fastest deserialization
        fastest_deserialize = min(successful, key=lambda x: x['deserialize_ms'])
        print(f"Fastest Deserialize: {fastest_deserialize['name']} ({fastest_deserialize['deserialize_ms']:.2f}ms)")
        
        # Smallest size
        smallest = min(successful, key=lambda x: x['size_kb'])
        print(f"Smallest Size: {smallest['name']} ({smallest['size_kb']:.2f} KB)")
        
        # Largest size
        largest = max(successful, key=lambda x: x['size_kb'])
        print(f"Largest Size: {largest['name']} ({largest['size_kb']:.2f} KB)")
    
    print(f"\nTotal formats tested: {len(formats)}")
    print(f"Successful: {len(successful)}")
    print(f"Failed: {len(results) - len(successful)}")
    
    print(f"\n✓ Output saved to: {data_dir}")


if __name__ == "__main__":
    main()
