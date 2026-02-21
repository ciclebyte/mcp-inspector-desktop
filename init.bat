@echo off
REM MCP Inspector Desktop - Development Environment Setup (Windows)
REM This script sets up and starts the development environment

echo ========================================
echo MCP Inspector Desktop - Dev Setup
echo ========================================
echo.

REM Check if we're in the project root
if not exist "package.json" (
    echo [ERROR] Must be run from project root directory
    exit /b 1
)

REM Check Node.js
echo [1/5] Checking Node.js...
where node >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Node.js is not installed
    echo Please install Node.js from https://nodejs.org/
    exit /b 1
)
for /f "tokens=*" %%i in ('node --version') do set NODE_VERSION=%%i
echo [OK] Node.js version: %NODE_VERSION%

REM Check Rust
echo.
echo [2/5] Checking Rust...
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [WARNING] Rust toolchain not found
    echo Please install Rust from https://rustup.rs/
    echo Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    exit /b 1
)
for /f "tokens=*" %%i in ('rustc --version') do set RUST_VERSION=%%i
echo [OK] Rust version: %RUST_VERSION%

REM Install npm dependencies
echo.
echo [3/5] Installing npm dependencies...
if not exist "node_modules\" (
    call npm install
    echo [OK] Dependencies installed
) else (
    echo [SKIP] node_modules already exists
    echo Run 'npm install' manually if you need to update dependencies
)

REM Check Tauri CLI
echo.
echo [4/5] Checking Tauri CLI...
npm list tauri >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [INFO] Tauri CLI not found, installing...
    call npm install -D @tauri-apps/cli
)
echo [OK] Tauri CLI ready

REM Check project files
echo.
echo [5/5] Checking project files...
if exist "feature_list.json" (
    echo [OK] feature_list.json found
) else (
    echo [WARNING] feature_list.json not found
)

if exist "claude-progress.txt" (
    echo [OK] claude-progress.txt found
) else (
    echo [WARNING] claude-progress.txt not found
)

REM Start development server
echo.
echo [START] Starting development server...
echo Press Ctrl+C to stop the server
echo.

call npm run tauri dev
