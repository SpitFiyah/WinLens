@echo off
setlocal

set "ROOT_DIR=%~dp0"
set "UI_DIR=%ROOT_DIR%privacy-auditor-ui"
set "CORE_DIR=%ROOT_DIR%privacy-auditor"
set "TAURI_DIR=%UI_DIR%\src-tauri"
set "RUSTUP_HOME=D:\Rust\rustup"
set "CARGO_HOME=D:\Rust\cargo"
set "PATH=D:\Rust\cargo\bin;D:\clion\CLion 2025.2.2\bin\mingw\bin;%PATH%"

where node >nul 2>nul
if errorlevel 1 (
  echo Missing Node.js. Install Node.js LTS from https://nodejs.org/ and reopen your terminal.
  exit /b 1
)

where npm >nul 2>nul
if errorlevel 1 (
  echo Missing npm. Install Node.js LTS from https://nodejs.org/ and reopen your terminal.
  exit /b 1
)

where cargo >nul 2>nul
if errorlevel 1 (
  echo Missing Rust/cargo. Install Rust from https://rustup.rs/ and reopen your terminal.
  exit /b 1
)

echo.
echo [install] Installing frontend dependencies
cd /d "%UI_DIR%" || exit /b 1
if exist package-lock.json (
  call npm ci || exit /b 1
) else (
  call npm install || exit /b 1
)

echo.
echo [install] Fetching Rust dependencies for core scanner
cd /d "%CORE_DIR%" || exit /b 1
call cargo fetch || exit /b 1

echo.
echo [install] Fetching Rust dependencies for Tauri desktop app
cd /d "%TAURI_DIR%" || exit /b 1
call cargo fetch || exit /b 1

echo.
echo [install] Building frontend to verify dependency install
cd /d "%UI_DIR%" || exit /b 1
call npm run build || exit /b 1

echo.
echo [install] Done. Start the full desktop app with start_project.cmd
