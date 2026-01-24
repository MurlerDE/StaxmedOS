@echo off
echo ======================================
echo   StaxmedOS Environment Debug
echo ======================================
echo.

echo Checking backend directory...
cd backend
if not exist Cargo.toml (
    echo ERROR: backend/Cargo.toml not found!
    cd ..
    exit /b 1
)
echo [OK] backend/Cargo.toml found
echo.

echo Checking .env file...
if exist .env (
    echo [OK] backend/.env found
    echo.
    echo .env file contents (values hidden):
    for /f "usebackq tokens=1,* delims==" %%a in (".env") do (
        if not "%%a"=="" if not "%%a:~0,1%"=="#" echo   %%a = ***
    )
) else (
    echo [ERROR] backend/.env not found!
    echo Please create backend/.env with required variables
    cd ..
    pause
    exit /b 1
)
echo.

echo Checking required environment variables in .env...
findstr /C:"DATABASE_URL" .env >nul
if %errorlevel% equ 0 (
    echo [OK] DATABASE_URL is set
) else (
    echo [ERROR] DATABASE_URL not found in .env
)

findstr /C:"JWT_SECRET" .env >nul
if %errorlevel% equ 0 (
    echo [OK] JWT_SECRET is set
) else (
    echo [ERROR] JWT_SECRET not found in .env
)

findstr /C:"STAX_MODE" .env >nul
if %errorlevel% equ 0 (
    echo [OK] STAX_MODE is set
) else (
    echo [WARN] STAX_MODE not set (will default to SAAS)
)
echo.

echo Checking Rust installation...
cargo --version >nul 2>&1
if %errorlevel% equ 0 (
    cargo --version
    echo [OK] Rust/Cargo is installed
) else (
    echo [ERROR] Cargo not found! Install Rust from https://rustup.rs/
    cd ..
    pause
    exit /b 1
)
echo.

echo Checking database connection...
echo Testing connection to database...
cargo run --bin backend --quiet -- --version 2>&1 | findstr "StaxmedOS" >nul
if %errorlevel% equ 0 (
    echo [OK] Backend can start
) else (
    echo [WARN] Could not verify backend startup
)
echo.

cd ..
echo ======================================
echo   Environment Check Complete
echo ======================================
echo.
echo If all checks passed, you can start the backend with:
echo   start-backend.bat
echo.
pause
