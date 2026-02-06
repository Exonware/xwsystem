# Lazy Mode Usage - Super Simple Test

Simple test to verify that xwlazy auto-installation works when xwsystem is installed with the `[lazy]` extra.

## Quick Test

```bash
cd examples/lazy_mode_usage
python simple_test.py
```

## What It Does

1. **Uninstalls** `msgpack` (the test package)
2. **Activates xwlazy** (simulating what `xwsystem.__init__.py` does automatically)
3. **Imports msgpack** - xwlazy should auto-install it
4. **Verifies** it works

## Expected Output

```
======================================================================
Super Simple Test: xwlazy Auto-Installation
======================================================================
Testing package: msgpack
======================================================================

[1/4] Uninstalling msgpack...
   OK: msgpack uninstalled (or wasn't installed)

[2/4] Verifying msgpack is uninstalled...
   OK: msgpack is not installed (as expected)

[3/4] Activating xwlazy (simulating xwsystem auto-activation)...
[OK] [xwlazy] Loaded 633 mappings from xwlazy_external_libs.toml
   OK: xwlazy activated successfully
   OK: xwlazy global import hook is installed

[4/4] Testing auto-installation of msgpack...
   INFO: Trying to import msgpack...
   INFO: xwlazy should auto-install it if missing

   Importing msgpack...
[INSTALL] [xwlazy] Blocking Install: msgpack (strategy: pip)...
[OK] [xwlazy] Installed: msgpack via pip (1.33s)
   OK: msgpack imported successfully!
   INFO: Location: .../msgpack/__init__.py
   INFO: Version: 1.1.2
   OK: msgpack is now installed in pip

======================================================================
SUCCESS: Auto-installation test completed!
======================================================================
```

## What This Proves

✅ **xwlazy integration works!** When xwsystem is imported with `[lazy]` extra:
- xwlazy is automatically activated in `xwsystem.__init__.py`
- Missing packages are automatically installed when imported
- Installation happens silently (unless `XWLAZY_VERBOSE=1` is set)
- Packages are installed via pip automatically

## How It Works

1. **Install xwsystem with lazy extra:**
   ```bash
   pip install exonware-xwsystem[lazy]
   ```

2. **Import xwsystem:**
   ```python
   import exonware.xwsystem  # This auto-activates xwlazy
   ```

3. **Import a missing package:**
   ```python
   import msgpack  # xwlazy automatically installs it!
   ```

## Verbose Mode

To see detailed installation messages:
```bash
export XWLAZY_VERBOSE=1  # Linux/Mac
set XWLAZY_VERBOSE=1     # Windows
python simple_test.py
```

## Full Installation Workflow

If you want to test in a clean virtual environment:

```bash
# 1. Create virtual environment
python -m venv .venv

# 2. Activate it
.venv\Scripts\Activate.ps1  # Windows PowerShell
source .venv/bin/activate    # Linux/Mac

# 3. Install xwsystem with lazy extra
pip install "exonware-xwsystem[lazy]"

# 4. Run the test
python simple_test.py

# 5. Cleanup (optional)
deactivate
Remove-Item -Recurse -Force .venv  # Windows
rm -rf .venv  # Linux/Mac
```

## Other Test Files

- `simple_test.py` - **Simple test** (recommended) - Uninstalls and tests auto-installation
- `json_run.py` - Tests BSON serialization with lazy installation
- `lazy_usage_demo.py` - More complex test with configuration checks

## Notes

- The test uses `msgpack` as an example, but any package in xwlazy's mapping will work
- Auto-installation is **silent by default** - set `XWLAZY_VERBOSE=1` to see installation messages
- Installation happens **on-demand** when a package is first imported
- Packages are installed via pip with smart strategy by default
