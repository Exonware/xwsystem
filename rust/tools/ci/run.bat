@echo off
REM Run tests for xwsystem_rust library
REM Similar to Python's pytest runner
REM Company: eXonware.com
REM Author: eXonware Backend Team

setlocal
set SCRIPT_DIR=%~dp0
REM Navigate from tools/ci to rust root (go up 2 levels: ci -> tools -> rust)
cd /d "%SCRIPT_DIR%..\.."
set RUST_DIR=%CD%

REM Check if cargo is available
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo not found. Please install Rust from https://rustup.rs/
    exit /b 1
)

echo Running tests for xwsystem_rust library...
echo.

REM Run all tests with verbose output
REM Similar to pytest -v --tb=short
cargo test --verbose

if %ERRORLEVEL% EQU 0 (
    echo.
    echo All tests passed!
) else (
    echo.
    echo Tests failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

endlocal

