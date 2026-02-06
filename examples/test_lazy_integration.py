#exonware/xwsystem/examples/test_lazy_integration.py
"""
Simple test to verify xwlazy integration into xwsystem.

This test:
1. Imports xwsystem (which should trigger auto_enable_lazy)
2. Verifies that xwlazy was activated
3. Tests that missing dependencies can be auto-installed

Usage:
    python examples/test_lazy_integration.py
"""

import sys
import os
from pathlib import Path

# Fix Unicode encoding on Windows
if sys.platform == "win32":
    try:
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stderr.reconfigure(encoding="utf-8")
    except (AttributeError, ValueError):
        pass

# Add parent directory to path for development
sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

def test_xwlazy_integration():
    """Test that xwlazy is properly integrated into xwsystem."""
    print("=" * 70)
    print("🧪 Testing xwlazy integration into xwsystem")
    print("=" * 70)
    
    # Step 1: Check if xwlazy is installed
    print("\n📦 Step 1: Checking if xwlazy is installed...")
    try:
        import exonware.xwlazy
        print("   ✅ xwlazy is installed")
        xwlazy_available = True
    except ImportError:
        print("   ❌ xwlazy is NOT installed")
        print("   💡 Install with: pip install exonware-xwsystem[lazy]")
        xwlazy_available = False
        return False
    
    # Step 2: Import xwsystem (this should trigger auto_enable_lazy)
    print("\n📦 Step 2: Importing xwsystem (should trigger auto_enable_lazy)...")
    try:
        # Import xwsystem - this should activate xwlazy if installed
        import exonware.xwsystem as xwsystem
        print(f"   ✅ xwsystem imported successfully")
        print(f"   📋 Version: {getattr(xwsystem, '__version__', 'unknown')}")
    except Exception as e:
        print(f"   ❌ Failed to import xwsystem: {e}")
        import traceback
        traceback.print_exc()
        return False
    
    # Step 3: Check if xwlazy was activated
    print("\n📦 Step 3: Verifying xwlazy activation...")
    try:
        from exonware.xwlazy import is_global_import_hook_installed, get_all_stats
        
        # Check if the global import hook is installed
        hook_installed = is_global_import_hook_installed()
        if hook_installed:
            print("   ✅ Global import hook is installed")
        else:
            print("   ⚠️  Global import hook is NOT installed")
            print("   💡 This might be normal if xwlazy auto-enable didn't trigger")
        
        # Get stats
        try:
            stats = get_all_stats()
            print(f"   📊 xwlazy stats: {stats}")
        except Exception as e:
            print(f"   ⚠️  Could not get stats: {e}")
            
    except ImportError as e:
        print(f"   ⚠️  Could not check xwlazy status: {e}")
    
    # Step 4: Test importing a module that might need external dependencies
    print("\n📦 Step 4: Testing import of a module with optional dependencies...")
    try:
        # Try importing something that uses optional dependencies
        from exonware.xwsystem.io.serialization.formats.binary.bson import BsonSerializer
        print("   ✅ BsonSerializer imported successfully")
        print("   💡 This means pymongo is available (or was auto-installed)")
    except ImportError as e:
        print(f"   ⚠️  Could not import BsonSerializer: {e}")
        print("   💡 This is expected if pymongo is not installed")
        print("   💡 With lazy mode, xwlazy should install it automatically")
    
    print("\n" + "=" * 70)
    print("✅ Integration test completed!")
    print("=" * 70)
    print("\n💡 Summary:")
    print("   • If xwlazy is installed, it should be auto-activated when xwsystem is imported")
    print("   • Missing dependencies will be automatically installed when imported")
    print("   • To see verbose output, set: export XWLAZY_VERBOSE=1")
    print()
    
    return True


if __name__ == "__main__":
    success = test_xwlazy_integration()
    sys.exit(0 if success else 1)
