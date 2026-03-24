#!/usr/bin/env python3
#exonware/xwsystem/rust/tests/test_python_bindings.py
"""
Python tests for Rust bindings.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
"""

from __future__ import annotations

import sys

import pytest


def _rust_bindings_available() -> bool:
    try:
        from exonware.rust.xwsystem import version as _version  # noqa: F401

        return True
    except ImportError:
        return False


_BINDINGS = _rust_bindings_available()


@pytest.mark.skipif(not _BINDINGS, reason="exonware.rust not built (run: maturin develop in xwsystem/rust)")
def test_version_module():
    """Test version module bindings."""
    from exonware.rust.xwsystem import version

    assert version.VERSION == "0.1.0.1"
    assert version.VERSION_MAJOR == 0
    assert version.VERSION_MINOR == 1
    assert version.VERSION_PATCH == 0
    assert version.get_version() == "0.1.0.1"
    major, minor, patch, build = version.get_version_info()
    assert major == 0
    assert minor == 1
    assert patch == 0
    info = version.get_version_dict()
    assert isinstance(info, dict)
    assert info["version"] == "0.1.0.1"
    info_obj = version.VersionInfo()
    assert info_obj.version == "0.1.0.1"
    assert info_obj.major == 0
    print("[OK] Version module tests passed")


@pytest.mark.skipif(not _BINDINGS, reason="exonware.rust not built (run: maturin develop in xwsystem/rust)")
def test_dummy_complicated():
    """Test dummy_complicated function."""
    from exonware.rust.xwsystem import dummy

    result = dummy.dummy_complicated(
        10,
        20,
        30,
        "test_string",
        True,
        5,
        15,
        25,
    )
    assert hasattr(result, "output1")
    assert hasattr(result, "output2")
    assert result.output1 == 44100
    assert isinstance(result.output2, str)
    assert "input1=10" in result.output2
    assert "input2=20" in result.output2
    assert "input3=30" in result.output2
    assert "input4=test_string" in result.output2
    assert "input5=true" in result.output2.lower()
    assert "args_count=3" in result.output2

    result2 = dummy.dummy_complicated(1, 2, 3, "hello", False, 10, 20)
    assert result2.output1 == 1296
    assert "args_count=2" in result2.output2
    assert "input5=false" in result2.output2.lower()

    result3 = dummy.dummy_complicated(5, 10, 15, "no_args", True)
    assert result3.output1 == 3600
    assert "args_count=0" in result3.output2
    assert "input5=true" in result3.output2.lower()

    result_dict = result.to_dict()
    assert isinstance(result_dict, dict)
    assert result_dict["output1"] == 44100
    assert isinstance(result_dict["output2"], str)
    print("[OK] Dummy complicated function tests passed")


def main() -> int:
    """Run all tests (plain script mode; pytest can run this file too)."""
    print("=" * 60)
    print("Python Bindings Test Suite")
    print("=" * 60)
    print()
    if not _BINDINGS:
        print("[SKIP] exonware.rust is not installed. Build with: maturin develop")
        print("=" * 60)
        return 0

    print("Testing version module...")
    try:
        test_version_module()
        v_ok = True
    except Exception as e:
        print(f"[FAIL] Version module: {e}")
        v_ok = False

    print()
    print("Testing dummy_complicated function...")
    try:
        test_dummy_complicated()
        d_ok = True
    except Exception as e:
        print(f"[FAIL] Dummy complicated: {e}")
        import traceback

        traceback.print_exc()
        d_ok = False

    print()
    print("=" * 60)
    print("Test Results:")
    print("=" * 60)
    print(f"  Version Module: {'PASS' if v_ok else 'FAIL'}")
    print(f"  Dummy Complicated: {'PASS' if d_ok else 'FAIL'}")
    print("=" * 60)
    if v_ok and d_ok:
        print("All tests passed! [SUCCESS]")
        return 0
    print("Some tests failed! [FAILED]")
    return 1


if __name__ == "__main__":
    sys.exit(main())
