#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UI_DIR="$ROOT_DIR/privacy-auditor-ui"

export RUSTUP_HOME="${RUSTUP_HOME:-/d/Rust/rustup}"
export CARGO_HOME="${CARGO_HOME:-/d/Rust/cargo}"
export PATH="$CARGO_HOME/bin:/d/clion/CLion 2025.2.2/bin/mingw/bin:$PATH"

info() {
  printf '\n[start] %s\n' "$1"
}

if ! command -v npm >/dev/null 2>&1; then
  printf 'Missing npm. Run bash install_dependencies.sh after installing Node.js LTS.\n' >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  printf 'Missing cargo. Install Rust from https://rustup.rs/, reopen your terminal, then run bash install_dependencies.sh.\n' >&2
  exit 1
fi

cd "$UI_DIR"

if [[ ! -d node_modules ]]; then
  info "node_modules not found; installing frontend dependencies first"
  if [[ -f package-lock.json ]]; then
    npm ci
  else
    npm install
  fi
fi

info "Starting Privacy Debt Auditor desktop app"
info "This launches the full Tauri app, including the Rust scanner backend."
npm run tauri dev
