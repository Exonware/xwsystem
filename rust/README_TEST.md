# Testing Python Bindings

## Quick Test

After building the Python bindings, run:

```bash
cd xwsystem/rust
python tests/test_python_bindings.py
```

Or use the test script:
```bash
xwsystem\rust\tools\ci\run_python_tests.bat
```

## What the Test Does

The test file `tests/test_python_bindings.py` tests:

1. **Version Module** - Tests all version constants, functions, and the VersionInfo class
2. **Dummy Complicated Function** - Tests the `dummy_complicated` function with:
   - 5 required inputs (3 integers, 1 string, 1 boolean)
   - Variable number of additional integer arguments
   - Returns 2 outputs (output1: i64, output2: String)

## Test Cases

### Dummy Complicated Function

The function signature:
```python
dummy_complicated(input1: int, input2: int, input3: int, input4: str, input5: bool, *args: int) -> DummyResult
```

Test cases:
- With 3 additional args: `(10, 20, 30, "test", True, 5, 15, 25)`
- With 2 additional args: `(1, 2, 3, "hello", False, 10, 20)`
- With no additional args: `(5, 10, 15, "no_args", True)`

Expected behavior:
- Sums all integers (input1 + input2 + input3 + all args)
- Multiplies by 2 if input5 is True, else by 1
- Squares the result → output1
- Creates a formatted string with all inputs → output2

## Building Before Testing

Make sure to build the Python bindings first:

```bash
cd xwsystem/rust
pip install maturin
maturin develop --release
```

Or use:
```bash
xwsystem\rust\tools\ci\build_python.bat
```

