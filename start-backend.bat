@echo off
echo ======================================
echo   Starting StaxmedOS Backend
echo ======================================
echo.

cd backend

echo Current directory: %CD%
echo.

if not exist Cargo.toml (
    echo ERROR: Cargo.toml not found!
    echo Please run this script from the staxmed-os root directory
    pause
    exit /b 1
)

echo Building and starting Rust backend...
echo.

cargo run

pause
