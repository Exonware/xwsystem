# Quick Start: Python Bindings

## Build and Install

```bash
cd xwsystem/rust

# Install maturin if not already installed
pip install maturin

# Build and install in development mode
maturin develop --release
```

Or use the build script:
```bash
xwsystem\rust\tools\ci\build_python.bat
```

## Usage

```python
from exonware.rust.xwsystem import version

# Get version
print(version.get_version())  # "0.1.0.1"

# Get version info as tuple
major, minor, patch, build = version.get_version_info()
print(f"{major}.{minor}.{patch}.{build}")

# Get version as dict
info = version.get_version_dict()
print(info)  # {'version': '0.1.0.1', 'major': 0, ...}

# Check version type
if version.is_dev_version():
    print("Development version")

# Use VersionInfo class
info = version.VersionInfo()
print(info.version)
print(info.to_dict())
```

## Run Example

```bash
python xwsystem/rust/examples/python_example.py
```

