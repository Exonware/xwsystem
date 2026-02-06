"""
#exonware/xwsystem/examples/lazy_mode_usage/lazy_usage_demo.py

Execute a turnkey verification of the xwsystem lazy installation mechanics.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1.387
Generation Date: 08-Nov-2025
"""

from __future__ import annotations

import json
import sys
import traceback
from typing import Any, Callable

from xwlazy.lazy import (
    LazyInstallConfig,
    config_package_lazy_install_enabled,
    get_lazy_install_stats,
    lazy_import_with_install,
)

if sys.platform == "win32":
    try:
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stderr.reconfigure(encoding="utf-8")
    except (AttributeError, ValueError):
        pass

PACKAGE_NAME = "xwsystem"
OPTIONAL_MODULE = "msgpack"


def verify_lazy_installation(
    package_name: str = PACKAGE_NAME,
    module_name: str = OPTIONAL_MODULE,
) -> dict[str, Any]:
    """
    Temporarily enable lazy mode and exercise a lazy import.

    WHY:
    - Demonstrates that enabling lazy mode via `config_package_lazy_install_enabled`
      updates runtime state.
    - Calls `lazy_import_with_install` to prove optional dependencies are handled on demand.
    - Restores the original configuration to avoid polluting subsequent sessions.
    """
    initial_enabled = LazyInstallConfig.is_enabled(package_name)
    initial_mode = LazyInstallConfig.get_mode(package_name)

    demo_enabled = initial_enabled
    demo_mode = initial_mode
    import_attempt: dict[str, Any] = {
        "module": module_name,
        "available": False,
        "skipped": True,
        "reason": "lazy configuration not applied",
    }
    stats: dict[str, Any] = {}

    try:
        config_package_lazy_install_enabled(
            package_name,
            True,
            mode="interactive",
            install_hook=False,
        )

        demo_enabled = LazyInstallConfig.is_enabled(package_name)
        demo_mode = LazyInstallConfig.get_mode(package_name)

        try:
            module, available = lazy_import_with_install(module_name)
            import_attempt = {
                "module": module_name,
                "available": available,
                "module_file": getattr(module, "__file__", "built-in") if module else None,
            }
        except Exception as exc:  # noqa: BLE001 (we need the complete context for diagnostics)
            import_attempt = {
                "module": module_name,
                "available": False,
                "error": str(exc),
                "traceback": traceback.format_exc(),
            }

        stats = get_lazy_install_stats(package_name)
    finally:
        config_package_lazy_install_enabled(
            package_name,
            initial_enabled,
            initial_mode,
            install_hook=False,
        )

    return {
        "initial_enabled": initial_enabled,
        "initial_mode": initial_mode,
        "demo_enabled": demo_enabled,
        "demo_mode": demo_mode,
        "import_attempt": import_attempt,
        "stats": stats,
    }


def main() -> None:
    """Run the verification and print a JSON summary."""
    print("🚀 Validating xwsystem lazy installation mode")
    results = verify_lazy_installation()
    print(json.dumps(results, indent=2, sort_keys=True, default=str))


if __name__ == "__main__":
    main()
