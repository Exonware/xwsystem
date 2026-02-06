"""
#exonware/xwsystem/examples/lazy_mode_quick_check.py

Simple verification script for the xwsystem lazy installation mode.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1.387
Generation Date: 08-Nov-2025

WHY:
- Quickly confirm that the lazy installation facade is wired correctly.
- Demonstrate how to enable lazy mode without installing the import hook.
- Provide a safe example that returns status data and restores the original configuration.
"""

from __future__ import annotations

import json
from typing import Any

import sys

from xwlazy.lazy import (
    LazyInstallConfig,
    config_package_lazy_install_enabled,
    get_lazy_install_stats,
)

if sys.platform == "win32":
    try:
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stderr.reconfigure(encoding="utf-8")
    except (AttributeError, ValueError):
        pass

PACKAGE_NAME = "xwsystem"


def run_lazy_mode_demo(package_name: str = PACKAGE_NAME) -> dict[str, Any]:
    """
    Enable lazy installation temporarily and report status.

    WHY:
    - Validates that `config_package_lazy_install_enabled` updates runtime state.
    - Ensures we can read lazy mode statistics without leaving side effects.
    - Keeps the original configuration intact for downstream tests.

    Args:
        package_name: Target library to configure. Defaults to ``"xwsystem"``.

    Returns:
        Dictionary containing the initial configuration, the demo configuration,
        and the collected lazy installation statistics.
    """
    initial_enabled = LazyInstallConfig.is_enabled(package_name)
    initial_mode = LazyInstallConfig.get_mode(package_name)

    config_package_lazy_install_enabled(
        package_name,
        True,
        mode="interactive",
        install_hook=False,
    )

    demo_enabled = LazyInstallConfig.is_enabled(package_name)
    demo_mode = LazyInstallConfig.get_mode(package_name)
    stats = get_lazy_install_stats(package_name)

    # Restore original configuration to avoid cross-test interference.
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
        "stats": stats,
    }


def main() -> None:
    """
    CLI entrypoint that prints the lazy mode demo results as JSON.

    WHY:
    - Provides a developer-friendly command for manual verification.
    - Keeps output deterministic for quick regression checks.
    """
    results = run_lazy_mode_demo()
    print("🚀 xwsystem lazy mode quick check")
    print(json.dumps(results, indent=2, sort_keys=True, default=str))


if __name__ == "__main__":
    main()
