# Lazy Installation - Integration Summary for AI & Developers

**Date:** October 7, 2025  
**Purpose:** Quick reference for integrating lazy installation into ANY package  
**Audience:** AI assistants, developers, library authors

---

## ?? Quick Facts

**What:** Automatic dependency installation via Python import hooks  
**Status:** Production ready, enterprise-grade, fully documented  
**Dependencies:** exonware-xwsystem only (no external deps)  
**Setup Time:** 5 minutes  
**Code Required:** 1 line in `__init__.py`  

---

## ?? **MANDATORY: 3 Installation Modes for ALL Packages**

Every eXonware package (and any package using this system) **MUST** support:

### **1. LITE (Default)**
```bash
pip install package-name
```
- Core dependencies only
- Minimal footprint
- ImportError if optional deps used

### **2. LAZY (Development)**
```bash
pip install package-name[lazy]
```
- Auto-installs dependencies on demand
- Zero configuration
- Seamless development experience

### **3. FULL (Production)**
```bash
pip install package-name[full]
```
- All dependencies pre-installed
- No auto-installation
- Production-ready

---

## ? 4-Step Integration

### **Step 1: pyproject.toml**

```toml
[project]
name = "exonware-yourpackage"
version = "0.0.1"
dependencies = [
    "exonware-xwsystem>=0.0.1",  # REQUIRED
]

[project.optional-dependencies]
lazy = [
    "exonware-xwsystem[lazy]>=0.0.1",  # REQUIRED
]

full = [
    # List ALL optional dependencies here
    "pandas>=2.0.0",
    "numpy>=1.24.0",
    "scikit-learn>=1.3.0",
]
```

### **Step 2: __init__.py (Line ~84)**

```python
# LAZY INSTALLATION - Simple One-Line Configuration
from exonware.xwsystem.utils.lazy_discovery import config_package_lazy_install_enabled
config_package_lazy_install_enabled("yourpackage")  # Auto-detect
```

### **Step 3: Standard Imports (NO try/except!)**

```python
# Just import normally
import pandas
import numpy
import scikit-learn

# NOT this:
# try:
#     import pandas
# except ImportError:
#     pandas = None
```

### **Step 4: Document in README**

```markdown
## Installation

### Lite (Default) - Core Only
\`\`\`bash
pip install exonware-yourpackage
\`\`\`

### Lazy (Recommended for Development)
\`\`\`bash
pip install exonware-yourpackage[lazy]
\`\`\`
Auto-installs missing dependencies on demand.

### Full (Recommended for Production)
\`\`\`bash
pip install exonware-yourpackage[full]
\`\`\`
All dependencies pre-installed.
```

---

## ? Complete Example: xwdata

### **File: pyproject.toml**
```toml
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "exonware-xwdata"
version = "0.0.1"
description = "Data processing with lazy installation"
dependencies = [
    "exonware-xwsystem>=0.0.1",
]

[project.optional-dependencies]
lazy = [
    "exonware-xwsystem[lazy]>=0.0.1",
]

full = [
    "pandas>=2.0.0",
    "numpy>=1.24.0",
    "pyarrow>=13.0.0",
    "polars>=0.19.0",
]
```

### **File: src/exonware/xwdata/__init__.py**
```python
"""
#exonware/xwdata/src/exonware/xwdata/__init__.py

Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
"""

# LAZY INSTALLATION - Simple One-Line Configuration
from exonware.xwsystem.utils.lazy_discovery import config_package_lazy_install_enabled
config_package_lazy_install_enabled("xwdata")

# IMPORTS - Standard Python Imports
import pandas as pd
import numpy as np
import pyarrow as pa
import polars as pl

# PACKAGE CODE
from .processor import DataProcessor

__all__ = ["DataProcessor", "pd", "np", "pa", "pl"]
```

### **File: src/exonware/xwdata/processor.py**
```python
"""
#exonware/xwdata/src/exonware/xwdata/processor.py

Data processor with auto-dependency installation.
"""

# Standard imports - NO defensive code!
import pandas as pd
import numpy as np

class DataProcessor:
    def process(self, data):
        df = pd.DataFrame(data)
        return df.to_numpy()
```

---

## ?? **What NOT to Do (Common Mistakes)**

### ? **DON'T: Use try/except for imports**
```python
# WRONG!
try:
    import pandas
except ImportError:
    pandas = None
```

### ? **DON'T: Create HAS_* flags**
```python
# WRONG!
HAS_PANDAS = False
try:
    import pandas
    HAS_PANDAS = True
except ImportError:
    pass
```

### ? **DON'T: Conditional import logic**
```python
# WRONG!
if HAS_PANDAS:
    df = pandas.DataFrame(data)
else:
    raise ImportError("Please install pandas")
```

### ? **DON'T: Forget pyproject.toml extras**
```python
# WRONG - Missing [lazy] and [full] extras!
[project.optional-dependencies]
# Nothing here
```

### ? **DON'T: Forget lazy config line**
```python
# WRONG - Missing lazy configuration!
# Just imports without:
# config_package_lazy_install_enabled("package")
```

---

## ? **Verification Checklist**

Before considering lazy integration complete:

- [ ] pyproject.toml has `exonware-xwsystem>=0.0.1` in dependencies
- [ ] pyproject.toml has `lazy` extra with `exonware-xwsystem[lazy]>=0.0.1`
- [ ] pyproject.toml has `full` extra with ALL optional dependencies
- [ ] __init__.py has `config_package_lazy_install_enabled("package")` around line 84
- [ ] All imports are standard (NO try/except ImportError)
- [ ] No `HAS_*` flags anywhere in codebase
- [ ] No conditional import logic
- [ ] README documents all 3 installation modes
- [ ] Tested: `pip install package` (lite)
- [ ] Tested: `pip install package[lazy]` (auto-install works)
- [ ] Tested: `pip install package[full]` (all deps present)
- [ ] Import hook is installed when [lazy] is used
- [ ] Standard imports work in all modules

---

## ?? For AI Assistants

When you see a request to create a new eXonware package or add lazy support:

**ALWAYS include these 4 elements:**

1. **pyproject.toml with 3 modes**
2. **One-line lazy config in __init__.py**
3. **Standard imports (no defensive code)**
4. **README with 3 installation modes**

**NEVER:**
- Skip the lazy configuration
- Use try/except for imports
- Create HAS_* flags
- Forget the [lazy] and [full] extras

**Documentation Reference:**
- Full guide: `LAZY_INSTALLATION_COMPLETE.md`
- This quick ref: `LAZY_INTEGRATION_SUMMARY.md`
- Standards: `DEV_GUIDELINES.md` (Import & Dependency Management section)

---

## ?? **Features Summary**

| Feature | Included | Status |
|---------|----------|---------|
| Import Hook | ? | Auto-installs on ImportError |
| Zero Overhead | ? | Passive for installed packages |
| Auto-Detection | ? | Detects [lazy] extra automatically |
| PEP 668 Protection | ? | Blocks externally-managed envs |
| Allow/Deny Lists | ? | Package security control |
| Index URL Control | ? | Private PyPI support |
| Vulnerability Audit | ? | pip-audit integration (optional) |
| Lockfile Generation | ? | Version tracking |
| SBOM Export | ? | Compliance ready |
| Policy Modes | ? | AUTO/INTERACTIVE/WARN/DRY_RUN/DISABLED |

---

## ?? **Result**

**One line of configuration**  
**+ Standard Python imports**  
**= Automatic dependency management with enterprise security**

**No defensive code. No boilerplate. Just clean Python.** ?

---

**For complete details, see LAZY_INSTALLATION_COMPLETE.md**


