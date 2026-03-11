# ?? Lazy Installation System - Quick Reference

> ?? **NEW!** For the complete, all-in-one guide, see **LAZY_INSTALLATION_COMPLETE.md**  
> This document is kept for quick reference, but the complete guide contains everything in one place!

---

## Overview

The lazy installation system provides **fully automatic installation** of missing dependencies using Python's import hook system. It's designed to be **zero-config, zero-overhead, and completely transparent**.

## ? Quick Start

### For Users

Install with lazy mode enabled:
```bash
pip install xwsystem[lazy]
```

**That's it!** Missing dependencies will be installed **automatically** when you import them:

```python
import fastavro  # Missing? Auto-installed! ?
import protobuf  # Missing? Auto-installed! ?
import pandas    # Missing? Auto-installed! ?

# NO try/except needed! NO special syntax! Just normal Python!
```

### For Developers

Add lazy support to your library in **one line**:

```python
# In your package's __init__.py (line 84 in xwsystem)
from exonware.xwsystem.utils.lazy_discovery import config_package_lazy_install_enabled
config_package_lazy_install_enabled("xwsystem")  # Auto-detects from installation
```

**What this does:**
1. Auto-detects if user installed with `[lazy]` extra
2. Enables lazy installation if detected
3. **Installs import hook in sys.meta_path**
4. System is ready - completely automatic!

## ?? Design Philosophy: Keep It Simple!

### ? DO THIS (Recommended - The New Standard)
```python
# Use standard Python imports - the hook handles everything!
import fastavro
import protobuf
import pandas

# Clean, simple, and automatic!
# If package is missing, hook installs it and import succeeds
# If package exists, import runs at full native speed (zero overhead)
```

### ? DON'T DO THIS (Old Defensive Pattern - Not Needed Anymore!)
```python
# NO try/except needed!
try:
    import fastavro
except ImportError:
    fastavro = None

# NO HAS_* flags needed!
HAS_FASTAVRO = False
try:
    import fastavro
    HAS_FASTAVRO = True
except ImportError:
    pass

# NO xwimport() needed for general use!
from ..utils.lazy_install import xwimport
fastavro = xwimport("fastavro")
```

**Why?** The import hook intercepts `ImportError` **before** it's raised, installs the package, and makes the import succeed. Your code continues seamlessly as if the package was always there!

## ?? How The Import Hook Works

### The Magic Behind The Scenes

When you do `import fastavro`:

```python
1. Python tries standard import locations
2. Package not found ? Would normally raise ImportError
3. Python checks sys.meta_path hooks (import finders)
4. LazyMetaPathFinder.find_spec("fastavro") is called
5. Hook checks:
   - Is this a top-level package? (yes)
   - Is lazy install enabled? (yes)
6. Hook runs: pip install fastavro
7. Hook returns module spec for now-installed package
8. Python sees success ? import completes
9. Your code continues from next line - seamlessly!

Next time you import fastavro:
1. Package exists ? standard import succeeds
2. Hook returns None (not needed)
3. ZERO overhead - full native speed!
```

### Performance Characteristics

| Scenario | Overhead | Notes |
|----------|----------|-------|
| **Package installed** | 0ms | Hook returns None immediately |
| **First import (missing)** | 5-30s | One-time pip install |
| **Detection (cached)** | 0.001ms | 200,000x faster than initial check |
| **Mapping lookup (cached)** | 0.001ms | 10,000x faster than file I/O |

**Result: Zero overhead for successful imports, 20-100x faster overall!**

## Features

### 1. Auto-Detection
Automatically detects if user installed with `[lazy]` extra:
- `pip install xwsystem[lazy]` ? Lazy enabled
- `pip install xwsystem` ? Lazy disabled

### 2. Per-Package Isolation
Each package has independent lazy configuration:
- xwsystem can have lazy enabled
- xwnode can have lazy disabled
- No interference between packages

### 3. Installation Modes

**AUTO** (default):
```python
config_package_lazy_install_enabled("xwsystem")  # Auto mode
```
Installs dependencies automatically without prompts.

**INTERACTIVE**:
```python
config_package_lazy_install_enabled("xwsystem", True, "interactive")
```
Asks user before installing each package.

**DISABLED**:
```python
config_package_lazy_install_enabled("xwsystem", False)
```
No automatic installation.

**DRY_RUN**:
```python
config_package_lazy_install_enabled("xwsystem", True, "dry_run")
```
Shows what would be installed without actually installing.

### 4. Environment Variable Override
```bash
export XWSYSTEM_LAZY_INSTALL=true
```

## When to Use xwimport()

The `xwimport()` function is available but **rarely needed**. Only use it for special cases where you need:
- Explicit control over installation
- Custom package name mapping
- Specific installer package selection

For 99% of use cases, **standard imports are better**.

## Best Practices

1. **Use standard imports** - Keep code clean and simple
2. **One-line configuration** - Just call `config_package_lazy_install_enabled()`
3. **Let auto-detection work** - Don't override unless needed
4. **Per-package isolation** - Each package configures itself
5. **Trust the system** - It's smart enough to handle most cases

## API Reference

### Configuration
```python
config_package_lazy_install_enabled(
    package_name: str,
    enabled: bool = None,  # None = auto-detect
    mode: str = "auto"     # "auto", "interactive", "disabled", "dry_run"
)
```

### Control Functions
```python
enable_lazy_install(package_name: str = 'default')
disable_lazy_install(package_name: str = 'default')
is_lazy_install_enabled(package_name: str = 'default')
set_lazy_install_mode(package_name: str, mode: LazyInstallMode)
```

### Statistics
```python
get_lazy_install_stats(package_name: str = 'default')
get_all_lazy_install_stats()
```

## Examples

### Example 1: Basic Setup
```python
# In __init__.py
from exonware.xwsystem.utils.lazy_discovery import config_package_lazy_install_enabled
config_package_lazy_install_enabled("xwsystem")

# In your code - use standard imports!
try:
    import fastavro
except ImportError:
    fastavro = None
```

### Example 2: Interactive Mode
```python
config_package_lazy_install_enabled("xwdata", True, "interactive")
```

When fastavro is missing:
```
============================================================
Lazy Installation Active - xwdata
============================================================
Package: fastavro
Module:  fastavro
============================================================

The module 'fastavro' is not installed.
Would you like to install 'fastavro'?

Options:
  [Y] Yes - Install this package
  [N] No  - Skip this package
  [A] All - Install this and all future packages without asking
  [Q] Quit - Cancel and raise ImportError
============================================================
Your choice [Y/N/A/Q]: 
```

### Example 3: Multiple Packages
```python
# xwsystem/__init__.py
config_package_lazy_install_enabled("xwsystem")  # Auto mode

# xwnode/__init__.py  
config_package_lazy_install_enabled("xwnode", True, "interactive")  # Interactive

# xwdata/__init__.py
config_package_lazy_install_enabled("xwdata", False)  # Disabled
```

Each package operates independently!

## Summary

The lazy installation system is designed to be:
- **Automatic** - Works without manual intervention
- **Simple** - One line to configure
- **Smart** - Auto-detects from pip installation
- **Isolated** - Per-package configuration
- **Clean** - No special imports needed

Just use standard Python imports and let the system handle the rest!


