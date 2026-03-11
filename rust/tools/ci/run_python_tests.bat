@echo off
REM Run Python tests for Rust bindings
REM Company: eXonware.com
REM Author: eXonware Backend Team

setlocal
set SCRIPT_DIR=%~dp0
REM Navigate from tools/ci to rust root (go up 2 levels: ci -> tools -> rust)
cd /d "%SCRIPT_DIR%..\.."
set RUST_DIR=%CD%

echo ============================================================
echo Python Bindings Test Runner
echo ============================================================
echo.

REM Check if Python is available
where python >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Python not found. Please install Python or add it to PATH.
    exit /b 1
)

REM Check if maturin is available
python -m maturin --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Installing maturin...
    python -m pip install maturin --quiet
    if %ERRORLEVEL% NEQ 0 (
        echo ERROR: Failed to install maturin.
        exit /b 1
    )
)

REM Check if Python bindings are built
echo Checking if Python bindings are built...
python -c "import _xwsystem_rust" >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Python bindings not found. Building...
    echo.
    
    REM Build the wheel
    python -m maturin build --release
    if %ERRORLEVEL% NEQ 0 (
        echo ERROR: Failed to build Python bindings.
        exit /b 1
    )
    
    REM Find the wheel file
    for %%f in ("%RUST_DIR%\target\wheels\exonware_rust_xwsystem-*.whl") do (
        echo Installing wheel: %%f
        python -m pip install "%%f" --force-reinstall --quiet
        if %ERRORLEVEL% NEQ 0 (
            echo ERROR: Failed to install Python bindings.
            exit /b 1
        )
        goto :wheel_installed
    )
    
    echo ERROR: Could not find built wheel file.
    exit /b 1
    
    :wheel_installed
    echo Python bindings installed successfully.
    echo.
) else (
    echo Python bindings already installed.
    echo.
)

REM Set PYTHONPATH to include the python source directory
set PYTHONPATH=%RUST_DIR%\python;%PYTHONPATH%

echo Running Python tests...
echo.

REM Run the Python test
python "%RUST_DIR%\tests\test_python_bindings.py"
set TEST_EXIT_CODE=%ERRORLEVEL%

if %TEST_EXIT_CODE% EQU 0 (
    echo.
    echo ============================================================
    echo All tests passed successfully!
    echo ============================================================
) else (
    echo.
    echo ============================================================
    echo Tests failed with exit code %TEST_EXIT_CODE%
    echo ============================================================
)

endlocal
exit /b %TEST_EXIT_CODE%
