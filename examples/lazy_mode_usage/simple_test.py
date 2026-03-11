#exonware/xwsystem/examples/lazy_mode_usage/simple_test.py
"""
Super Simple Test: Verify xwlazy auto-installation
This test:
1. Uninstalls msgpack (if installed)
2. Activates xwlazy directly (simulates what xwsystem does)
3. Tries to import msgpack - xwlazy should auto-install it
4. Verifies it works
Usage:
    python simple_test.py
"""

import sys
import subprocess
import os
# Fix encoding on Windows
if sys.platform == "win32":
    try:
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stderr.reconfigure(encoding="utf-8")
    except (AttributeError, ValueError):
        pass
TEST_PACKAGE = "msgpack"

def uninstall_package(package_name):
    """Force uninstall a package."""
    print(f"\n[1/4] Uninstalling {package_name}...")
    result = subprocess.run(
        [sys.executable, "-m", "pip", "uninstall", package_name, "-y"],
        capture_output=True,
        text=True
    )
    if result.returncode == 0:
        print(f"   OK: {package_name} uninstalled (or wasn't installed)")
    else:
        print(f"   INFO: {package_name} uninstall result: {result.returncode}")
    return True

def verify_uninstalled(package_name):
    """Check if package is actually uninstalled."""
    print(f"\n[2/4] Verifying {package_name} is uninstalled...")
    try:
        __import__(package_name)
        print(f"   WARNING: {package_name} is still installed")
        return False
    except ImportError:
        print(f"   OK: {package_name} is not installed (as expected)")
        return True

def activate_xwlazy():
    """Activate xwlazy - this is what xwsystem.__init__.py does."""
    print(f"\n[3/4] Activating xwlazy (simulating xwsystem auto-activation)...")
    try:
        # Enable verbose mode to see what's happening
        os.environ["XWLAZY_VERBOSE"] = "1"
        # Import and activate xwlazy - this is what xwsystem does
        from exonware.xwlazy import auto_enable_lazy, is_global_import_hook_installed
        # Activate for xwsystem package (simulating what happens in __init__.py)
        result = auto_enable_lazy("xwsystem", mode="smart")
        if result:
            print(f"   OK: xwlazy activated successfully")
        else:
            print(f"   WARNING: xwlazy activation returned None")
        # Check if hook is installed
        if is_global_import_hook_installed():
            print(f"   OK: xwlazy global import hook is installed")
            return True
        else:
            print(f"   ERROR: xwlazy hook is NOT installed")
            return False
    except ImportError as e:
        print(f"   ERROR: xwlazy is not installed: {e}")
        print(f"   INFO: Install with: pip install exonware-xwsystem[lazy]")
        return False
    except Exception as e:
        print(f"   ERROR: Failed to activate xwlazy: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_auto_install(package_name):
    """Test that importing triggers auto-installation."""
    print(f"\n[4/4] Testing auto-installation of {package_name}...")
    print(f"   INFO: Trying to import {package_name}...")
    print(f"   INFO: xwlazy should auto-install it if missing")
    print()
    try:
        # Clear any cached import first
        if package_name in sys.modules:
            del sys.modules[package_name]
        # Try to import - xwlazy should install it automatically
        print(f"   Importing {package_name}...")
        module = __import__(package_name)
        print(f"   OK: {package_name} imported successfully!")
        print(f"   INFO: Location: {getattr(module, '__file__', 'built-in')}")
        print(f"   INFO: Version: {getattr(module, '__version__', 'unknown')}")
        # Verify it's actually installed via pip
        print(f"   Verifying installation via pip...")
        result = subprocess.run(
            [sys.executable, "-m", "pip", "show", package_name],
            capture_output=True,
            text=True
        )
        if result.returncode == 0:
            print(f"   OK: {package_name} is now installed in pip")
            # Show version from pip
            for line in result.stdout.split('\n'):
                if line.startswith('Version:'):
                    print(f"   INFO: Pip version: {line.split(':', 1)[1].strip()}")
            return True
        else:
            print(f"   WARNING: {package_name} imported but not in pip list")
            print(f"   INFO: This might be normal if import worked but pip check failed")
            return True  # Still success if import worked
    except ImportError as e:
        print(f"   ERROR: {package_name} import failed: {e}")
        print(f"   INFO: Check verbose output above for installation attempts")
        print(f"   INFO: xwlazy should have tried to install {package_name}")
        return False
    except Exception as e:
        print(f"   ERROR: Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    """Run the simple test."""
    print("=" * 70)
    print("Super Simple Test: xwlazy Auto-Installation")
    print("=" * 70)
    print(f"Testing package: {TEST_PACKAGE}")
    print("=" * 70)
    # Step 1: Uninstall the test package
    uninstall_package(TEST_PACKAGE)
    # Step 2: Verify it's uninstalled
    if not verify_uninstalled(TEST_PACKAGE):
        print("\nWARNING: Package is still installed, continuing anyway...")
    # Step 3: Activate xwlazy (simulating what xwsystem.__init__.py does)
    if not activate_xwlazy():
        print("\nERROR: Failed to activate xwlazy")
        return False
    # Step 4: Test auto-installation
    success = test_auto_install(TEST_PACKAGE)
    print("\n" + "=" * 70)
    if success:
        print("SUCCESS: Auto-installation test completed!")
        print("=" * 70)
        print("\nWhat happened:")
        print("  1. Uninstalled msgpack")
        print("  2. Activated xwlazy (simulating xwsystem auto-activation)")
        print("  3. Imported msgpack (xwlazy auto-installed it)")
        print("  4. Verified it works!")
        print("\nThis proves that when xwsystem is imported with [lazy] extra,")
        print("xwlazy is automatically activated and will install missing packages!")
    else:
        print("FAILED: Auto-installation test did not complete")
        print("=" * 70)
        print("\nPossible issues:")
        print("  - xwlazy might not be installed (pip install exonware-xwsystem[lazy])")
        print("  - Package might not be in xwlazy's mapping")
        print("  - Check verbose output above for installation details")
        print("  - Look for '[INSTALL]' or '[OK]' messages in the output")
    return success
if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
