#!/usr/bin/env python3
#exonware/xwsystem/tests/0.core/utils/test_auto_install.py
"""
Test auto-installation of missing packages using xwimport
"""

import sys
import pytest
from pathlib import Path
# Add the src directory to the path
sys.path.insert(0, str(Path(__file__).parent / 'src'))
@pytest.mark.skip(reason="xwlazy has been removed from the codebase")


def test_auto_install():
    """Test if xwimport automatically installs missing packages."""
    print("🧪 Testing Auto-Installation Feature")
    print("=" * 50)
    print("1. Testing direct xwimport...")
    try:
        # from xwlazy.lazy import xwimport
        raise ImportError("xwlazy has been removed")
        print("   Attempting to import fastavro using xwimport...")
        fastavro = xwimport("fastavro")
        print(f"   ✅ SUCCESS: fastavro imported successfully!")
        print(f"   Module type: {type(fastavro)}")
        print(f"   Module location: {fastavro.__file__ if hasattr(fastavro, '__file__') else 'built-in'}")
        return
    except ImportError as e:
        print(f"   ❌ FAILED: Could not import fastavro: {e}")
        return
    except Exception as e:
        print(f"   ❌ ERROR: Unexpected error: {e}")
        return

def test_avro_serializer():
    """Test if AvroSerializer automatically installs fastavro."""
    print("\n2. Testing AvroSerializer...")
    try:
        from exonware.xwsystem.serialization.avro import AvroSerializer
        print("   Creating AvroSerializer instance...")
        serializer = AvroSerializer()
        print("   ✅ SUCCESS: AvroSerializer created successfully!")
        # Test basic functionality
        test_data = {"name": "test", "value": 123}
        schema = {
            "type": "record",
            "name": "TestRecord",
            "fields": [
                {"name": "name", "type": "string"},
                {"name": "value", "type": "int"}
            ]
        }
        serializer.schema = schema
        print("   Testing serialization...")
        serialized = serializer.dumps_binary(test_data)
        print(f"   ✅ SUCCESS: Data serialized to {len(serialized)} bytes")
        assert len(serialized) > 0, "Serialized data should not be empty"
        print("   Testing deserialization...")
        deserialized = serializer.loads_bytes(serialized)
        print(f"   ✅ SUCCESS: Data deserialized: {deserialized}")
        assert deserialized == test_data, "Deserialized data should match original"
    except ImportError as e:
        pytest.skip(f"AvroSerializer not available: {e}")
    except Exception as e:
        pytest.fail(f"Unexpected error: {e}")

def main():
    """Run the auto-installation tests."""
    print("🚀 Testing xwimport Auto-Installation")
    print("=" * 50)
    # Test 1: Direct xwimport
    test1_success = test_auto_install()
    # Test 2: AvroSerializer usage
    test2_success = test_avro_serializer()
    print("\n" + "=" * 50)
    print("📊 TEST RESULTS:")
    print(f"   Direct xwimport test: {'✅ PASSED' if test1_success else '❌ FAILED'}")
    print(f"   AvroSerializer test: {'✅ PASSED' if test2_success else '❌ FAILED'}")
    if test1_success and test2_success:
        print("\n🎉 ALL TESTS PASSED! Auto-installation is working!")
    else:
        print("\n⚠️  Some tests failed. Check the output above for details.")
    # Verify fastavro is now installed
    print("\n3. Verifying installation...")
    try:
        import fastavro
        print("   ✅ fastavro is now installed and available")
    except ImportError:
        print("   ❌ fastavro is still not available")
if __name__ == "__main__":
    main()
