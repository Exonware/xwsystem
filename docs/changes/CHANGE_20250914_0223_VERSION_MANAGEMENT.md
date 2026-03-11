# Version Management in XWSystem

**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Email:** connect@exonware.com  
**Version:** 0.0.1.3  
**Generation Date:** January 27, 2025

## ?? Overview

XWSystem now uses **centralized version management** to ensure consistency across all files and eliminate version drift. This system provides a single source of truth for version information.

## ?? Centralized Version File

The version is centrally managed in:
```
src/exonware/xwsystem/version.py
```

This file contains:
- `__version__`: Main version string
- Version components (major, minor, patch, build)
- Version utilities and helper functions
- Version metadata

## ?? How It Works

### 1. **Central Version Definition**
```python
# src/exonware/xwsystem/version.py
__version__ = "0.0.1.3"
VERSION_MAJOR = 0
VERSION_MINOR = 0
VERSION_PATCH = 1
VERSION_BUILD = 3
```

### 2. **Automatic Import in Python Files**
```python
# In any Python file that needs version
from .version import __version__  # or from exonware.xwsystem.version import __version__
```

### 3. **Build System Integration**
```toml
# pyproject.toml
[project]
dynamic = ["version"]

[tool.hatch.version]
path = "src/exonware/xwsystem/version.py"
```

## ?? Files That Use Centralized Version

### ? **Automatically Updated (via imports)**
- `src/exonware/xwsystem/__init__.py`
- `src/xwsystem.py`
- `src/exonware/xwsystem/cli/base.py`
- `src/exonware/xwsystem/enterprise/distributed_tracing.py`

### ?? **Manually Updated (config files)**
- `pyproject.xwsystem.toml`
- `requirements.txt`
- `README.md`
- `docs/DEV_GUIDELINES.md`

## ?? Updating Versions

### **Method 1: Using the Update Script (Recommended)**
```bash
# Update to new version
python update_version.py 0.0.1.3

# This will update:
# - Centralized version file
# - Configuration files
# - Documentation files
```

### **Method 2: Manual Update**
1. **Update central version file:**
   ```python
   # src/exonware/xwsystem/version.py
   __version__ = "0.0.1.3"
   VERSION_MAJOR = 0
   VERSION_MINOR = 0
   VERSION_PATCH = 1
   ```

2. **Update config files manually:**
   - `pyproject.xwsystem.toml`
   - `requirements.txt`
   - `README.md`
   - `docs/DEV_GUIDELINES.md`

## ?? Version Utilities

The centralized version file provides utility functions:

```python
from exonware.xwsystem.version import (
    get_version,           # Get version string
    get_version_info,      # Get version tuple
    get_version_dict,      # Get version dictionary
    is_dev_version,        # Check if dev version
    is_release_version     # Check if release version
)

# Examples
version = get_version()                    # "0.0.1.3"
info = get_version_info()                  # (0, 0, 1, 3)
details = get_version_dict()               # {"version": "0.0.1.3", "major": 0, ...}
is_dev = is_dev_version()                  # False
is_release = is_release_version()          # True
```

## ?? Version Format

### **Standard Format**
- `X.Y.Z` - Release versions (e.g., `0.0.1`)
- `X.Y.Z.W` - Build versions (e.g., `0.0.1.3`)

### **Development Versions**
- `X.Y.Z.dev` - Development version
- `X.Y.Z.alpha` - Alpha version
- `X.Y.Z.beta` - Beta version
- `X.Y.Z.rc1` - Release candidate

## ?? Benefits

### ? **Consistency**
- Single source of truth for version
- No more version drift between files
- Automatic synchronization

### ? **Maintainability**
- Easy to update versions
- Reduced human error
- Clear version history

### ? **Automation**
- Build system integration
- Automated version detection
- Utility functions for version handling

## ?? Integration Points

### **Build System**
- Hatchling automatically reads version from central file
- No need to manually update `pyproject.toml` version

### **Python Imports**
- All Python files import from central location
- Consistent version across all modules

### **Documentation**
- Version utilities available for dynamic documentation
- Consistent version display across all docs

## ?? Best Practices

### ? **Do**
- Always use the centralized version file
- Import version from central location in Python files
- Use the update script for version changes
- Keep version format consistent

### ? **Don't**
- Hardcode versions in Python files
- Manually update versions in multiple places
- Use inconsistent version formats
- Forget to update config files

## ?? Troubleshooting

### **Version Not Updating**
1. Check if you're importing from the right location
2. Verify the central version file exists
3. Ensure build system is configured correctly

### **Import Errors**
1. Check import path: `from exonware.xwsystem.version import __version__`
2. Verify the version file is in the correct location
3. Ensure the package is properly installed

### **Build Issues**
1. Check `pyproject.toml` configuration
2. Verify Hatchling can read the version file
3. Ensure version file syntax is correct

## ?? Examples

### **Using Version in Code**
```python
from exonware.xwsystem.version import __version__, get_version_dict

# Simple version display
print(f"XWSystem version: {__version__}")

# Detailed version info
version_info = get_version_dict()
print(f"Version: {version_info['version']}")
print(f"Major: {version_info['major']}")
print(f"Minor: {version_info['minor']}")
print(f"Patch: {version_info['patch']}")
```

### **CLI Version Display**
```python
from exonware.xwsystem.version import __version__

class MyCLI:
    def __init__(self):
        self.version = __version__
    
    def show_version(self):
        print(f"MyCLI version: {self.version}")
```

## ?? Summary

The centralized version management system in XWSystem provides:

- **Single source of truth** for version information
- **Automatic synchronization** across all files
- **Easy maintenance** with utility scripts
- **Build system integration** for seamless packaging
- **Consistent versioning** across the entire project

This system eliminates version drift and makes version management much more reliable and maintainable.

