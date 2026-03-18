# Rust CI Tools

Batch scripts for building, installing, and testing the xwsystem_rust library.

## Available Scripts

### build.bat
Builds the Rust library in release mode.

```batch
build.bat
```

### install_dev.bat
Builds the library for development use and shows usage instructions.

```batch
install_dev.bat
```

### run.bat
Runs all Rust tests for the library.

```batch
run.bat
```

### build_python.bat
Builds and installs Python bindings for the Rust library.

```batch
build_python.bat
```

This will:
- Check for maturin installation (installs if needed)
- Build the Python extension module
- Install it as a wheel package
- Make it available for Python import

### run_python_tests.bat
Runs Python tests for the Rust bindings.

```batch
run_python_tests.bat
```

This will:
- Check if Python bindings are built (builds if needed)
- Set up PYTHONPATH
- Run all Python tests
- Show test results

## Quick Start

### Build and Test Rust Library

```batch
# Build
build.bat

# Run Rust tests
run.bat
```

### Build and Test Python Bindings

```batch
# Build Python bindings
build_python.bat

# Run Python tests
run_python_tests.bat
```

## Usage Examples

### Using from Python

After building with `build_python.bat`:

```python
from exonware.rust.xwsystem import version, dummy

# Version module
print(version.get_version())  # "0.1.0.1"
info = version.get_version_dict()

# Dummy function
result = dummy.dummy_complicated(10, 20, 30, "test", True, 5, 15, 25)
print(result.output1)  # 44100
print(result.output2)
```

## Requirements

- **Rust toolchain** - Install from https://rustup.rs/
- **Python 3.12+** - Make sure Python is in PATH
- **maturin** - Automatically installed by build_python.bat if needed

## Error Handling

All scripts check for required tools and provide helpful error messages if dependencies are missing.
