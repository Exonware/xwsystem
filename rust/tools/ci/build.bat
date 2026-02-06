@echo off
REM Build script for xwsystem_rust library
REM Company: eXonware.com
REM Author: Eng. Muhammad AlShehri

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

echo Building xwsystem_rust library...
echo.

REM Build the library
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Build completed successfully!
    echo Library location: %RUST_DIR%\target\release\
) else (
    echo.
    echo Build failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

endlocal

