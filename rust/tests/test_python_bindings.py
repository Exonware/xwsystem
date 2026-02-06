#!/usr/bin/env python3
#exonware/xwsystem/rust/tests/test_python_bindings.py
"""
Python tests for Rust bindings.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
"""

import sys
import pytest


def test_version_module():
    """Test version module bindings."""
    try:
        from exonware.rust.xwsystem import version
        
        # Test constants
        assert version.VERSION == "0.1.0.1"
        assert version.VERSION_MAJOR == 0
        assert version.VERSION_MINOR == 1
        assert version.VERSION_PATCH == 0
        
        # Test functions
        assert version.get_version() == "0.1.0.1"
        major, minor, patch, build = version.get_version_info()
        assert major == 0
        assert minor == 1
        assert patch == 0
        
        # Test dict
        info = version.get_version_dict()
        assert isinstance(info, dict)
        assert info["version"] == "0.1.0.1"
        
        # Test VersionInfo class
        info_obj = version.VersionInfo()
        assert info_obj.version == "0.1.0.1"
        assert info_obj.major == 0
        
        print("[OK] Version module tests passed")
        return True
    except ImportError as e:
        print(f"[FAIL] Failed to import version module: {e}")
        print("  Make sure to build Python bindings: maturin develop")
        return False


def test_dummy_complicated():
    """Test dummy_complicated function."""
    try:
        from exonware.rust.xwsystem import dummy
        
        # Test with 5 inputs + 3 args
        result = dummy.dummy_complicated(
            10,  # input1
            20,  # input2
            30,  # input3
            "test_string",  # input4
            True,  # input5
            5, 15, 25  # variable args
        )
        
        # Verify result type
        assert hasattr(result, 'output1')
        assert hasattr(result, 'output2')
        
        # Verify output1: (10 + 20 + 30 + 5 + 15 + 25) * 2 = 210, squared = 44100
        assert result.output1 == 44100
        
        # Verify output2 contains expected strings
        assert isinstance(result.output2, str)
        assert "input1=10" in result.output2
        assert "input2=20" in result.output2
        assert "input3=30" in result.output2
        assert "input4=test_string" in result.output2
        assert "input5=true" in result.output2.lower()  # Rust uses lowercase
        assert "args_count=3" in result.output2
        
        # Test with different inputs
        result2 = dummy.dummy_complicated(
            1,  # input1
            2,  # input2
            3,  # input3
            "hello",  # input4
            False,  # input5
            10, 20  # 2 args
        )
        
        # (1 + 2 + 3 + 10 + 20) * 1 = 36, squared = 1296
        assert result2.output1 == 1296
        assert "args_count=2" in result2.output2
        assert "input5=false" in result2.output2.lower()  # Rust uses lowercase
        
        # Test with no args
        result3 = dummy.dummy_complicated(
            5,  # input1
            10,  # input2
            15,  # input3
            "no_args",  # input4
            True  # input5
        )
        
        # (5 + 10 + 15) * 2 = 60, squared = 3600
        assert result3.output1 == 3600
        assert "args_count=0" in result3.output2
        assert "input5=true" in result3.output2.lower()  # Rust uses lowercase
        
        # Test to_dict method
        result_dict = result.to_dict()
        assert isinstance(result_dict, dict)
        assert result_dict["output1"] == 44100
        assert isinstance(result_dict["output2"], str)
        
        print("[OK] Dummy complicated function tests passed")
        return True
    except ImportError as e:
        print(f"[FAIL] Failed to import dummy module: {e}")
        print("  Make sure to build Python bindings: maturin develop")
        return False
    except Exception as e:
        print(f"[FAIL] Test failed with error: {e}")
        import traceback
        traceback.print_exc()
        return False


def main():
    """Run all tests."""
    print("=" * 60)
    print("Python Bindings Test Suite")
    print("=" * 60)
    print()
    
    results = []
    
    print("Testing version module...")
    results.append(("Version Module", test_version_module()))
    print()
    
    print("Testing dummy_complicated function...")
    results.append(("Dummy Complicated", test_dummy_complicated()))
    print()
    
    print("=" * 60)
    print("Test Results:")
    print("=" * 60)
    
    all_passed = True
    for name, passed in results:
        status = "PASS" if passed else "FAIL"
        print(f"  {name}: {status}")
        if not passed:
            all_passed = False
    
    print("=" * 60)
    
    if all_passed:
        print("All tests passed! [SUCCESS]")
        return 0
    else:
        print("Some tests failed! [FAILED]")
        return 1


if __name__ == "__main__":
    sys.exit(main())
