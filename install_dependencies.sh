#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UI_DIR="$ROOT_DIR/privacy-auditor-ui"
CORE_DIR="$ROOT_DIR/privacy-auditor"
TAURI_DIR="$UI_DIR/src-tauri"

export RUSTUP_HOME="${RUSTUP_HOME:-/d/Rust/rustup}"
export CARGO_HOME="${CARGO_HOME:-/d/Rust/cargo}"
export PATH="$CARGO_HOME/bin:/d/clion/CLion 2025.2.2/bin/mingw/bin:$PATH"

info() {
  printf '\n[install] %s\n' "$1"
}

require_command() {
  local command_name="$1"
  local install_hint="$2"

  if ! command -v "$command_name" >/dev/null 2>&1; then
    printf '\nMissing required command: %s\n' "$command_name" >&2
    printf '%s\n' "$install_hint" >&2
    exit 1
  fi
}

info "Checking required tools"
require_command node "Install Node.js LTS from https://nodejs.org/ and reopen your terminal."
require_command npm "Install Node.js LTS from https://nodejs.org/ and reopen your terminal."
require_command cargo "Install Rust from https://rustup.rs/ and reopen your terminal."

info "Node: $(node --version)"
info "npm: $(npm --version)"
info "Rust: $(cargo --version)"

info "Installing frontend dependencies"
cd "$UI_DIR"
if [[ -f package-lock.json ]]; then
  npm ci
else
  npm install
fi

info "Fetching Rust dependencies for core scanner"
cd "$CORE_DIR"
cargo fetch

info "Fetching Rust dependencies for Tauri desktop app"
cd "$TAURI_DIR"
cargo fetch

info "Building frontend to verify dependency install"
cd "$UI_DIR"
npm run build

info "Done. Start the full desktop app with: bash start_project.sh"
