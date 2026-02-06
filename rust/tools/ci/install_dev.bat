@echo off
REM Install xwsystem_rust library in development mode
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

echo Building xwsystem_rust library for development use...
echo.

REM Build the library (both debug and release)
REM This makes it available for use as a path dependency
cargo build
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Build failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Release build failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

echo.
echo Library built successfully for development use!
echo.
echo To use this library in other Rust projects, add to Cargo.toml:
echo   [dependencies]
echo   xwsystem_rust = { path = "../xwsystem/rust" }
echo.
echo Or use the absolute path:
echo   xwsystem_rust = { path = "D:\OneDrive\DEV\exonware\xwsystem\rust" }
echo.

endlocal

