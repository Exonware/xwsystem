@echo off
REM Build Python bindings for xwsystem_rust library
REM Company: eXonware.com
REM Author: eXonware Backend Team

setlocal
set SCRIPT_DIR=%~dp0
REM Navigate from tools/ci to rust root (go up 2 levels: ci -> tools -> rust)
cd /d "%SCRIPT_DIR%..\.."
set RUST_DIR=%CD%

REM Check if maturin is available
python -m maturin --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Maturin not found. Installing...
    python -m pip install maturin --quiet
    if %ERRORLEVEL% NEQ 0 (
        echo ERROR: Failed to install maturin. Please install manually:
        echo   pip install maturin
        exit /b 1
    )
)

echo Building Python bindings for xwsystem_rust...
echo.

REM Build the wheel
python -m maturin build --release
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Build failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

REM Find and install the wheel file
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

echo.
echo Python bindings built and installed successfully!
echo.
echo You can now use it in Python:
echo   from exonware.rust.xwsystem import version
echo   print(version.get_version())
echo.
echo To test, run:
echo   %SCRIPT_DIR%run_python_tests.bat
echo   or
echo   python examples\python_example.py

endlocal

