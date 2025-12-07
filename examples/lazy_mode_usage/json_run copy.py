"""
Example: Lazy Mode Usage with BSON Serialization

Usage:
    python json_run.py

Behavior:
    - Optionally uninstalls xwlazy if it is installed (moves beyond xwlazy)
    - Uninstalls serialization dependencies first to test lazy installation
    - Enables lazy mode for xwsystem via exonware.conf with "smart" mode
    - Demonstrates that missing dependencies are handled lazily
"""

import subprocess
import sys
import importlib.metadata

def is_xwlazy_installed() -> bool:
    """Check if xwlazy is installed (optionally, skip if not)."""
    try:
        importlib.metadata.distribution("exonware-xwlazy")
        return True
    except importlib.metadata.PackageNotFoundError:
        try:
            importlib.metadata.distribution("xwlazy")
            return True
        except importlib.metadata.PackageNotFoundError:
            return False
    except Exception:
        try:
            import exonware.xwlazy  # noqa: F401
            return True
        except ImportError:
            return False

def uninstall_xwlazy_if_installed():
    """Optionally uninstall xwlazy if it is installed, skip if not."""
    if not is_xwlazy_installed():
        print("ℹ️  xwlazy is not installed, skipping uninstall")
        return
    
    print("🧹 Uninstalling xwlazy...")
    try:
        subprocess.run(
            [sys.executable, "-m", "pip", "uninstall", "-y", "exonware-xwlazy", "xwlazy"],
            capture_output=True,
            check=False
        )
        print("✅ Uninstalled xwlazy\n")
    except Exception as exc:
        print(f"❌ Failed to uninstall xwlazy: {exc}\n")

# Optionally uninstall xwlazy if installed
#uninstall_xwlazy_if_installed()

from exonware.xwsystem.io.serialization.formats.binary import BsonSerializer as serializer

data = {"name": "John", "age": 30}
serializer_obj = serializer()
result = serializer_obj.encode(data)
print(f"\n📄 Serialized: {result}")
#print("✅ Success! Lazy mode auto-installed dependencies if missing.")
