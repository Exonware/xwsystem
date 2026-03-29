#exonware/xwsystem/tests/1.unit/utils/test_lazy_mode_example.py
"""
Validate lazy-install wiring: exonware.xwlazy.config_package_lazy_install_enabled
(pip: exonware-xwlazy; alias: xwlazy).
Company: eXonware.com
Author: eXonware Backend Team
"""

from __future__ import annotations

from pathlib import Path

import pytest

pytest.importorskip("exonware.xwlazy")  # exonware-xwlazy installed ([lazy] / dev)

from exonware.xwlazy import config_package_lazy_install_enabled


def _reset_xwlazy_singleton() -> None:
    from exonware.xwlazy import auto_enable_lazy, uninstall_global_import_hook

    uninstall_global_import_hook()
    globals_dict = auto_enable_lazy.__globals__
    if "_instance" in globals_dict:
        globals_dict["_instance"] = None


@pytest.mark.xwsystem_unit
def test_config_package_lazy_importable() -> None:
    """Public lazy API must be importable from xwlazy.lazy (GUIDE_00_MASTER contract)."""
    assert callable(config_package_lazy_install_enabled)


@pytest.mark.xwsystem_unit
def test_config_package_lazy_install_enabled_smoke() -> None:
    """Calling the helper for exonware.xwsystem must not raise (matches __init__.py usage)."""
    config_package_lazy_install_enabled(
        "exonware.xwsystem",
        enabled=True,
        mode="smart",
    )


@pytest.mark.xwsystem_unit
def test_config_package_lazy_install_enabled_disabled_noop() -> None:
    """When disabled, the wrapper should not call auto_enable (no exception)."""
    config_package_lazy_install_enabled(
        "exonware.xwsystem",
        enabled=False,
        mode="smart",
    )


@pytest.mark.xwsystem_unit
def test_xwsystem_lazy_loading_policy_enabled_via_config(tmp_path: Path) -> None:
    """With both flags enabled, xwsystem policy should be configured in lazy mode."""
    _reset_xwlazy_singleton()
    pyproject = tmp_path / "pyproject.toml"
    pyproject.write_text(
        "\n".join(
            [
                "[project]",
                "name = 'demo'",
                "version = '0.0.0'",
                "",
                "[tool.xwlazy]",
                "enable_lazy_install = true",
                "enable_lazy_loading = true",
                "",
            ]
        ),
        encoding="utf-8",
    )

    config_package_lazy_install_enabled(
        "exonware.xwsystem",
        enabled=True,
        mode="smart",
        root=str(tmp_path),
    )

    # Access singleton via public function globals (read-only assertion in tests).
    from exonware.xwlazy import auto_enable_lazy

    instance = auto_enable_lazy.__globals__.get("_instance")
    assert instance is not None
    policy = instance.package_policies.get("exonware.xwsystem")
    assert policy is not None
    assert policy["enabled"] is True
    assert policy["mode"] == "lazy"


@pytest.mark.xwsystem_unit
def test_xwsystem_lazy_install_disabled_via_config(tmp_path: Path) -> None:
    """With lazy install disabled, wrapper should not configure xwsystem policy."""
    _reset_xwlazy_singleton()
    pyproject = tmp_path / "pyproject.toml"
    pyproject.write_text(
        "\n".join(
            [
                "[project]",
                "name = 'demo'",
                "version = '0.0.0'",
                "",
                "[tool.xwlazy]",
                "enable_lazy_install = false",
                "enable_lazy_loading = true",
                "",
            ]
        ),
        encoding="utf-8",
    )

    config_package_lazy_install_enabled(
        "exonware.xwsystem",
        enabled=True,
        mode="smart",
        root=str(tmp_path),
    )

    from exonware.xwlazy import auto_enable_lazy

    instance = auto_enable_lazy.__globals__.get("_instance")
    if instance is not None:
        assert "exonware.xwsystem" not in instance.package_policies
