# Python Bindings for XWSystem Rust

This directory contains Python bindings for the Rust implementation of XWSystem, allowing you to use Rust code from Python.

## Package Structure

The Python package is named `exonware.rust.xwsystem`:

```
exonware/
  rust/
    xwsystem/
      __init__.py          # Main package init
      version.py           # Python wrapper for version module
      _xwsystem_rust.pyd   # Compiled Rust extension (Windows)
      # or _xwsystem_rust.so (Linux) or _xwsystem_rust.dylib (macOS)
```

## Building Python Bindings

### Prerequisites

1. **Rust toolchain** - Install from https://rustup.rs/
2. **Python 3.8+** - Make sure Python is in your PATH
3. **maturin** - Python package builder for Rust extensions

### Quick Build

```bash
cd xwsystem/rust
pip install maturin
maturin develop
```

Or use the build script:
```bash
xwsystem\rust\tools\ci\build_python.bat
```

### Release Build

```bash
maturin build --release
```

This creates a wheel file that can be installed with `pip install`.

## Installation

### Development Installation

```bash
cd xwsystem/rust
maturin develop
```

This builds and installs the package in development mode (editable install).

### Production Installation

```bash
cd xwsystem/rust
maturin build --release
pip install target/wheels/exonware_rust_xwsystem-*.whl
```

## Usage Example

```python
from exonware.rust.xwsystem import version

# Access version constants
print(f"Version: {version.VERSION}")
print(f"Major: {version.VERSION_MAJOR}")

# Use version functions
version_str = version.get_version()
version_info = version.get_version_info()  # Returns tuple
version_dict = version.get_version_dict()  # Returns dict

# Check version type
if version.is_dev_version():
    print("This is a development version")

# Use VersionInfo class
info = version.VersionInfo()
print(f"Version: {info.version}")
print(f"Build: {info.build}")
print(f"Dict: {info.to_dict()}")
```

## Running the Example

```bash
cd xwsystem/rust
python examples/python_example.py
```

## Project Structure

```
rust/
├── Cargo.toml              # Rust project config (with PyO3)
├── pyproject.toml          # Python package config (maturin)
├── src/
│   ├── lib.rs              # Main Rust library
│   ├── version.rs          # Version module (Rust)
│   └── python_bindings.rs  # PyO3 bindings
├── python/                 # Python package source
│   └── exonware/
│       └── rust/
│           └── xwsystem/
│               ├── __init__.py
│               └── version.py
└── examples/
    └── python_example.py   # Usage example
```

## Features

- ✅ Full Python API for Rust version module
- ✅ Type-safe bindings using PyO3
- ✅ Pythonic interface with proper error handling
- ✅ Cross-platform support (Windows, Linux, macOS)
- ✅ Development and production builds

## Troubleshooting

### Import Error

If you get `ImportError: cannot import name 'version'`, make sure:
1. The Rust extension is built: `maturin develop`
2. You're using the correct Python environment
3. The package is installed: `pip list | grep exonware`

### Build Errors

If maturin fails to build:
1. Ensure Rust is installed: `rustc --version`
2. Ensure Python development headers are available
3. On Windows, you may need Visual Studio Build Tools
4. On Linux, install: `python3-dev` or `python3-devel`

## Next Steps

- Add more modules (errors, defs, etc.)
- Add async support
- Add comprehensive tests
- Publish to PyPI

