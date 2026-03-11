#!/usr/bin/env python3
#exonware/xwsystem/examples/serialization_example/x2_path_access/benchmark.py
"""
Path-Based Access Benchmark
Tests path-based partial access without full deserialization:
- JSON Pointer (get_at, set_at)
- XPath for XML
- Dot notation for YAML
- Nested path access
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
from x0_common.data_generator import generate_nested_data
from x0_common.test_helpers import print_section, save_to_file
from exonware.xwsystem.serialization import (
    JsonSerializer, YamlSerializer, XmlSerializer
)


def test_json_pointer():
    """Test JSON Pointer path access"""
    print("\n### JSON Pointer (RFC 6901) ###")
    serializer = JsonSerializer()
    data = generate_nested_data()
    # Serialize
    json_data = serializer.dumps(data)
    data_dir = Path(__file__).parent / "data"
    data_dir.mkdir(exist_ok=True)
    save_to_file(json_data, data_dir / "nested.json")
    # Test get_at with different paths
    paths = [
        "/company/name",
        "/company/departments/engineering/manager",
        "/company/departments/engineering/employees/0/name",
        "/company/metadata/founded"
    ]
    print("\nget_at() tests:")
    for path in paths:
        try:
            start = time.perf_counter()
            value = serializer.get_at(json_data, path)
            elapsed = (time.perf_counter() - start) * 1000
            print(f"  ✓ {path}: {value} ({elapsed:.3f}ms)")
        except Exception as e:
            print(f"  ✗ {path}: {e}")
    # Test set_at
    print("\nset_at() tests:")
    try:
        start = time.perf_counter()
        updated = serializer.set_at(json_data, "/company/departments/engineering/budget", 150000)
        elapsed = (time.perf_counter() - start) * 1000
        # Verify
        value = serializer.get_at(updated, "/company/departments/engineering/budget")
        print(f"  ✓ Updated budget: {value} ({elapsed:.3f}ms)")
        save_to_file(updated, data_dir / "nested_updated.json")
    except Exception as e:
        print(f"  ✗ Error: {e}")
    # Test iter_path
    print("\niter_path() test:")
    try:
        start = time.perf_counter()
        count = 0
        for item in serializer.iter_path(json_data, "company.departments.*"):
            count += 1
        elapsed = (time.perf_counter() - start) * 1000
        print(f"  ✓ Found {count} departments ({elapsed:.3f}ms)")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def test_xpath():
    """Test XML XPath access"""
    print("\n### XML XPath ###")
    serializer = XmlSerializer()
    data = generate_nested_data()
    # Serialize
    xml_data = serializer.dumps(data)
    data_dir = Path(__file__).parent / "data"
    save_to_file(xml_data, data_dir / "nested.xml")
    # Test get_at with XPath
    paths = [
        "//company/name",
        "//engineering/manager"
    ]
    print("\nget_at() with XPath:")
    for path in paths:
        try:
            start = time.perf_counter()
            value = serializer.get_at(xml_data, path)
            elapsed = (time.perf_counter() - start) * 1000
            print(f"  ✓ {path}: {value} ({elapsed:.3f}ms)")
        except Exception as e:
            print(f"  ✗ {path}: {e}")


def test_yaml_dot_notation():
    """Test YAML dot notation access"""
    print("\n### YAML Dot Notation ###")
    serializer = YamlSerializer()
    data = generate_nested_data()
    # Serialize
    yaml_data = serializer.dumps(data)
    data_dir = Path(__file__).parent / "data"
    save_to_file(yaml_data, data_dir / "nested.yaml")
    # Test get_at with dot notation
    paths = [
        "company.name",
        "company.departments.engineering.manager",
        "company.departments.engineering.employees.0.name"
    ]
    print("\nget_at() with dot notation:")
    for path in paths:
        try:
            start = time.perf_counter()
            value = serializer.get_at(yaml_data, path)
            elapsed = (time.perf_counter() - start) * 1000
            print(f"  ✓ {path}: {value} ({elapsed:.3f}ms)")
        except Exception as e:
            print(f"  ✗ {path}: {e}")
    # Test set_at
    print("\nset_at() test:")
    try:
        start = time.perf_counter()
        updated = serializer.set_at(yaml_data, "company.departments.sales.budget", 75000)
        elapsed = (time.perf_counter() - start) * 1000
        # Verify
        value = serializer.get_at(updated, "company.departments.sales.budget")
        print(f"  ✓ Updated budget: {value} ({elapsed:.3f}ms)")
        save_to_file(updated, data_dir / "nested_updated.yaml")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def main():
    """Main benchmark runner"""
    print_section("PATH-BASED ACCESS BENCHMARK")
    print("\nPath-based access allows reading/writing specific nodes")
    print("WITHOUT loading the entire file into memory!")
    test_json_pointer()
    test_xpath()
    test_yaml_dot_notation()
    print_section("SUMMARY")
    print("\n✓ Path access enables:")
    print("  - Random access to nested data")
    print("  - Partial updates without full deserialization")
    print("  - Memory-efficient operations on large files")
    print("  - Database-like operations on serialized data")
    data_dir = Path(__file__).parent / "data"
    print(f"\n✓ Output saved to: {data_dir}")
if __name__ == "__main__":
    main()
