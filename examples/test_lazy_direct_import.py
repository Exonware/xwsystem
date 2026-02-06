#exonware/xwsystem/examples/test_lazy_direct_import.py
"""
Test that xwlazy can auto-install a missing package on direct import.

This test:
1. Imports xwsystem (which activates xwlazy)
2. Tries to import a package that's not installed but is in xwlazy's mapping
3. Verifies that xwlazy installs it automatically

Usage:
    python examples/test_lazy_direct_import.py
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

def test_direct_import():
    """Test that xwlazy can install a missing package on direct import."""
    print("=" * 70)
    print("Testing xwlazy direct import auto-installation")
    print("=" * 70)
    
    # Step 1: Import xwsystem (this should activate xwlazy)
    print("\nStep 1: Importing xwsystem to activate xwlazy...")
    try:
        # Only import version to avoid triggering all the imports
        import sys as _sys
        _sys.path.insert(0, str(Path(__file__).parent.parent / "src"))
        from exonware.xwsystem.version import __version__
        print(f"   OK: xwsystem version {__version__} loaded")
        print("   INFO: xwlazy should be activated by xwsystem.__init__.py")
    except Exception as e:
        print(f"   WARNING: Could not import xwsystem.version: {e}")
    
    # Step 2: Check if xwlazy is activated
    print("\nStep 2: Checking xwlazy activation...")
    try:
        from exonware.xwlazy import is_global_import_hook_installed
        hook_installed = is_global_import_hook_installed()
        if hook_installed:
            print("   OK: xwlazy global import hook is installed")
        else:
            print("   INFO: xwlazy global import hook is not installed")
            print("   Attempting to activate manually...")
            from exonware.xwlazy import auto_enable_lazy
            auto_enable_lazy("xwsystem", mode="smart")
            hook_installed = is_global_import_hook_installed()
            if hook_installed:
                print("   OK: xwlazy activated manually")
            else:
                print("   ERROR: Failed to activate xwlazy")
                return False
    except Exception as e:
        print(f"   ERROR: Could not check xwlazy status: {e}")
        import traceback
        traceback.print_exc()
        return False
    
    # Step 3: Test importing a package that might not be installed
    print("\nStep 3: Testing direct import of optional dependency...")
    print("   INFO: This will attempt to import 'msgpack' which may not be installed")
    print("   INFO: If xwlazy is working, it should install msgpack automatically")
    print("   INFO: Set XWLAZY_VERBOSE=1 to see installation messages")
    print()
    
    try:
        # Try to import msgpack - xwlazy should install it if missing
        import msgpack
        print(f"   OK: msgpack imported successfully")
        print(f"   INFO: msgpack version: {getattr(msgpack, '__version__', 'unknown')}")
        print(f"   INFO: msgpack location: {getattr(msgpack, '__file__', 'built-in')}")
        return True
    except ImportError as e:
        print(f"   INFO: msgpack import failed (may not be installed yet): {e}")
        print("   INFO: xwlazy should handle this - check verbose output if enabled")
        return False
    except Exception as e:
        print(f"   ERROR: Unexpected error importing msgpack: {e}")
        import traceback
        traceback.print_exc()
        return False
    
    print("\n" + "=" * 70)
    print("Test completed!")
    print("=" * 70)
    return True

if __name__ == "__main__":
    # Enable verbose mode for this test
    os.environ["XWLAZY_VERBOSE"] = "1"
    success = test_direct_import()
    sys.exit(0 if success else 1)
