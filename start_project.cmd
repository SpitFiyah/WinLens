@echo off
setlocal

set "ROOT_DIR=%~dp0"
set "UI_DIR=%ROOT_DIR%privacy-auditor-ui"
set "RUSTUP_HOME=D:\Rust\rustup"
set "CARGO_HOME=D:\Rust\cargo"
set "PATH=D:\Rust\cargo\bin;D:\clion\CLion 2025.2.2\bin\mingw\bin;%PATH%"

where npm >nul 2>nul
if errorlevel 1 (
  echo Missing npm. Install Node.js LTS, reopen your terminal, then run install_dependencies.cmd.
  exit /b 1
)

where cargo >nul 2>nul
if errorlevel 1 (
  echo Missing Rust/cargo. Install Rust from https://rustup.rs/, reopen your terminal, then run install_dependencies.cmd.
  exit /b 1
)

cd /d "%UI_DIR%" || exit /b 1

if not exist node_modules (
  echo.
  echo [start] node_modules not found; installing frontend dependencies first
  if exist package-lock.json (
    call npm ci || exit /b 1
  ) else (
    call npm install || exit /b 1
  )
)

echo.
echo [start] Starting Privacy Debt Auditor desktop app
echo [start] This opens a Tauri desktop window. Vite runs internally at http://localhost:1420/.
call npm run tauri dev
