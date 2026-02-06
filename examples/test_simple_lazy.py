#exonware/xwsystem/examples/test_simple_lazy.py
"""
Simple test to verify xwlazy basic functionality.

This test just verifies that:
1. xwlazy can be activated
2. The integration code runs without errors

Usage:
    python examples/test_simple_lazy.py
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

def test_xwlazy_activation():
    """Test that xwlazy can be activated."""
    print("=" * 70)
    print("Testing xwlazy activation")
    print("=" * 70)
    
    # Step 1: Check if xwlazy is installed
    print("\nStep 1: Checking if xwlazy is installed...")
    try:
        from exonware.xwlazy import auto_enable_lazy, is_global_import_hook_installed
        print("   OK: xwlazy is installed")
    except ImportError as e:
        print(f"   ERROR: xwlazy is NOT installed: {e}")
        print("   Install with: pip install exonware-xwsystem[lazy]")
        return False
    
    # Step 2: Try to activate xwlazy manually
    print("\nStep 2: Testing manual xwlazy activation...")
    try:
        result = auto_enable_lazy("xwsystem", mode="smart")
        if result:
            print("   OK: xwlazy activated successfully")
            print(f"   Result: {result}")
        else:
            print("   WARNING: xwlazy activation returned None")
    except Exception as e:
        print(f"   ERROR: Failed to activate xwlazy: {e}")
        import traceback
        traceback.print_exc()
        return False
    
    # Step 3: Check if the hook is installed
    print("\nStep 3: Checking if import hook is installed...")
    try:
        hook_installed = is_global_import_hook_installed()
        if hook_installed:
            print("   OK: Global import hook is installed")
        else:
            print("   INFO: Global import hook is NOT installed (this may be normal)")
    except Exception as e:
        print(f"   WARNING: Could not check hook status: {e}")
    
    print("\n" + "=" * 70)
    print("Test completed!")
    print("=" * 70)
    print("\nNote: This test only verifies xwlazy activation.")
    print("To test actual auto-installation, you need:")
    print("  1. A missing package (e.g., try importing a package that's not installed)")
    print("  2. The package must be in xwlazy's mapping")
    print()
    
    return True

if __name__ == "__main__":
    success = test_xwlazy_activation()
    sys.exit(0 if success else 1)
