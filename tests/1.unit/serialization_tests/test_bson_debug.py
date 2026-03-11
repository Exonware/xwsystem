#!/usr/bin/env python3
#exonware/xwsystem/tests/1.unit/serialization_tests/test_bson_debug.py
"""
Debug BSON serializer issue
"""

import sys
import os
from pathlib import Path
# Add xwsystem to path - adjusted for new location
current_dir = Path(__file__).parent
src_dir = current_dir.parent.parent.parent.parent / 'src'
sys.path.insert(0, str(src_dir))

def debug_bson():
    """Debug BSON serializer issue"""
    print("🔍 DEBUGGING BSON SERIALIZER")
    print("=" * 40)
    try:
        from exonware.xwsystem.io.serialization import BsonSerializer
        # Create serializer (ASerialization accepts max_depth and max_size_mb only)
        serializer = BsonSerializer(max_depth=10)
        test_data = {"test": "bson", "number": 42}
        print(f"📋 Format: {serializer.format_name}")
        print(f"🔧 Binary: {serializer.is_binary_format}")
        print(f"📄 Extensions: {serializer.file_extensions}")
        # Test 1: dumps/loads
        print(f"\n🔄 Testing dumps/loads...")
        try:
            serialized = serializer.dumps(test_data)
            print(f"  ✅ dumps() returned: {type(serialized)}, length: {len(serialized)}")
            deserialized = serializer.loads(serialized)
            print(f"  ✅ loads() returned: {deserialized}")
        except Exception as e:
            print(f"  ❌ dumps/loads failed: {e}")
            import traceback
            traceback.print_exc()
            return False
        # Test 2: File operations
        print(f"\n🔄 Testing file operations...")
        test_file = Path("test_bson_debug.bson")
        try:
            if test_file.exists():
                test_file.unlink()
            print(f"  🔄 Testing save_file...")
            serializer.save_file(test_data, test_file)
            print(f"  ✅ save_file completed")
            if not test_file.exists():
                print(f"  ❌ File was not created!")
                return False
            print(f"  📁 File created: {test_file.stat().st_size} bytes")
            print(f"  🔄 Testing load_file...")
            loaded = serializer.load_file(test_file)
            print(f"  ✅ load_file returned: {loaded}")
            return True
        except Exception as e:
            print(f"  ❌ File operations failed: {e}")
            import traceback
            traceback.print_exc()
            return False
        finally:
            if test_file.exists():
                test_file.unlink()
    except ImportError as e:
        print(f"❌ BSON import failed: {e}")
        return False
    except Exception as e:
        print(f"❌ BSON test failed: {e}")
        import traceback
        traceback.print_exc()
        return False
if __name__ == "__main__":
    success = debug_bson()
    print(f"\n{'✅ BSON WORKING' if success else '❌ BSON FAILED'}")
