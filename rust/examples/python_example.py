#!/usr/bin/env python3
#exonware/xwsystem/rust/examples/python_example.py
"""
Example usage of exonware.rust.xwsystem from Python.
This demonstrates how to use the Rust implementation of XWSystem
from Python code.
"""

try:
    from exonware.rust.xwsystem import version
    print("=" * 60)
    print("XWSystem Rust - Python Bindings Example")
    print("=" * 60)
    print()
    # Access version constants
    print("Version Constants:")
    print(f"  VERSION: {version.VERSION}")
    print(f"  VERSION_MAJOR: {version.VERSION_MAJOR}")
    print(f"  VERSION_MINOR: {version.VERSION_MINOR}")
    print(f"  VERSION_PATCH: {version.VERSION_PATCH}")
    print(f"  VERSION_BUILD: {version.VERSION_BUILD}")
    print(f"  VERSION_SUFFIX: {version.VERSION_SUFFIX}")
    print(f"  VERSION_STRING: {version.VERSION_STRING}")
    print()
    # Use version functions
    print("Version Functions:")
    print(f"  get_version(): {version.get_version()}")
    print(f"  get_version_info(): {version.get_version_info()}")
    print(f"  get_version_dict(): {version.get_version_dict()}")
    print(f"  is_dev_version(): {version.is_dev_version()}")
    print(f"  is_release_version(): {version.is_release_version()}")
    print()
    # Use VersionInfo class
    print("VersionInfo Class:")
    info = version.VersionInfo()
    print(f"  info = {info}")
    print(f"  info.version: {info.version}")
    print(f"  info.major: {info.major}")
    print(f"  info.minor: {info.minor}")
    print(f"  info.patch: {info.patch}")
    print(f"  info.build: {info.build}")
    print(f"  info.suffix: {info.suffix}")
    print(f"  info.to_dict(): {info.to_dict()}")
    print()
    print("=" * 60)
    print("Example completed successfully!")
    print("=" * 60)
except ImportError as e:
    print("ERROR: Failed to import Rust bindings")
    print(f"  {e}")
    print()
    print("To build the Python bindings, run:")
    print("  cd xwsystem/rust")
    print("  pip install maturin")
    print("  maturin develop")
    print()
    print("Or use the build script:")
    print("  xwsystem\\rust\\tools\\ci\\build_python.bat")
