# Privacy Debt Auditor (WinLens) 🔍

[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-blue.svg)](https://microsoft.com/windows)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-18.x-61dafb.svg)](https://reactjs.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-ffc131.svg)](https://tauri.app/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

> “Your machine remembers more about you than you realize.”

**Privacy Debt Auditor** is a high-performance, Windows-first, local-only forensic privacy auditor that identifies, maps, visualizes, and explains sensitive personal data exposure and tracking residue stored across a user’s machine.

It operates entirely offline with **no cloud processing, no telemetry, no analytics, no external APIs, and no remote logging**. The audit itself never becomes a privacy leak.

---

## ✨ Features

- **🔒 Local-First Architecture:** 100% offline analysis. Your data never leaves your machine.
- **📈 Privacy Debt Score™:** Generates a global privacy exposure score based on exposed secrets, tracking cookies, and metadata leakage.
- **📂 High-Performance Filesystem Scanner:** Fast, concurrent Rust-based scanning of local directories for exposed secrets, identity documents, and sensitive data.
- **🔑 Secret Detection Engine:** Detects JWTs, AWS keys, OAuth tokens, SSH keys, bearer tokens, and mnemonic phrases using regex and entropy analysis.
- **🌐 Browser Privacy Analysis:** Analyzes browser remnants (Chrome, Edge) including cookies, history, autofill traces, and ad trackers.
- **🗃️ Windows Artifact Analysis:** Examines Windows-specific artifacts to reconstruct behavioral timelines and privacy exposure.
- **📊 Interactive Dashboards:** A cinematic, forensic-grade React interface with dynamic heatmaps and relationship graphs.

---

## 🛠️ Tech Stack

### Core Engine (Backend)
- **Language:** [Rust](https://www.rust-lang.org/)
- **Async Runtime:** Tokio
- **Database Layer:** SQLite
- **Windows APIs:** `windows-rs` crate

### User Interface (Frontend)
- **Framework:** [Tauri](https://tauri.app/) + [React](https://react.dev/)
- **Language:** TypeScript
- **Styling:** Vanilla CSS (Matte black, charcoal gray, amber highlights)
- **Build Tool:** Vite

---

## 🚀 Installation & Setup Guide

**Privacy Debt Auditor (WinLens)** requires a modern development environment due to its integration of a fast Rust backend and a React/Tauri frontend. Please follow these detailed steps to ensure your system is properly configured.

### Phase 1: Prerequisites Setup (Windows)

#### 1. Visual Studio C++ Build Tools
The Rust compiler (`rustc`) relies on the C++ linker provided by Microsoft.
- Download the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
- Run the installer.
- Under **Workloads**, ensure that **"Desktop development with C++"** is checked.
- On the right side under "Installation details", make sure **"Windows 10 SDK"** (or Windows 11 SDK) is selected.
- Click **Install** and wait for it to complete.

#### 2. Install Rust
Rust is the core language for our high-performance backend.
- Download `rustup-init.exe` from [rustup.rs](https://rustup.rs/).
- Run the installer and proceed with the default installation (Press `1` when prompted).
- Once installed, open a **new** command prompt or PowerShell window and verify the installation:
  ```bash
  rustc --version
  cargo --version
  ```

#### 3. Install Node.js & npm
Node.js is required to run the React frontend build tools (Vite).
- Download the latest LTS version of [Node.js](https://nodejs.org/).
- Run the installer. Ensure that **"Add to PATH"** is selected.
- Verify the installation in a new terminal:
  ```bash
  node -v
  npm -v
  ```

---

### Phase 2: Project Setup

#### 1. Clone the Repository
Clone the WinLens project to your local machine:
```bash
git clone https://github.com/SpitFiyah/WinLens.git
cd WinLens
```

#### 2. Install Frontend Dependencies
Navigate into the Tauri user interface folder and install the NPM packages:
```bash
cd privacy-auditor-ui
npm install
```
*Note: Due to the current state of Vite plugins, if you encounter peer dependency errors, run `npm install --legacy-peer-deps`.*

---

### Phase 3: Running the Application

You can start the project using our automated scripts or manually.

#### Option A: Quick Start Scripts
From the root of the `WinLens` directory, run the appropriate script for your terminal:

**Windows Command Prompt / PowerShell:**
```cmd
.\start_project.cmd
```

**Git Bash / WSL:**
```bash
./start_project.sh
```

#### Option B: Manual Development Mode
If you want to view logs and work interactively:
1. Ensure you are inside the `privacy-auditor-ui` directory.
2. Run the Tauri development command:
   ```bash
   npm run tauri dev
   ```
This will compile the Rust backend, start the Vite development server, and launch the native desktop window. Note that the **first compilation will take a few minutes** as Rust fetches and compiles all dependencies.

---

### Phase 4: Building for Production

To create a standalone, optimized `.exe` installer for Windows:
```bash
npm run tauri build
```
Once the process finishes, your executable will be located in:
`privacy-auditor-ui/src-tauri/target/release/bundle/msi/` or `.../bundle/nsis/`.

---

## 📂 Project Structure

```text
WinLens/
├── privacy-auditor/              # Core Rust Library / CLI Engine
│   ├── src/                      # Rust modules (scanner, secrets, browser analysis)
│   └── Cargo.toml
│
├── privacy-auditor-ui/           # Tauri Desktop Application
│   ├── src/                      # React frontend components & hooks
│   ├── src-tauri/                # Tauri backend integration
│   └── package.json
│
├── PROJECT_PLAN.md               # Detailed execution phases
├── IMPLEMENTATION_ROADMAP.md     # Development roadmap and tracking
├── Product_design.md             # Core design philosophy and architecture
└── QUICK_START.md                # Development quickstart guide
```

---

## 🛡️ Security & Privacy Guarantees

This project was built from the ground up to respect user privacy:

1. **Zero Egress:** The application makes zero outbound network requests.
2. **Data Minimization:** We hash or redact secrets before displaying them in the UI or exporting them to reports.
3. **No Telemetry:** We do not track application usage, crashes, or feature adoption.
4. **Local Database:** All findings are stored in a local SQLite database that never syncs to the cloud.

---

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/amazing-feature`).
3. Ensure you follow the project's security guidelines (no secrets in test data).
4. Commit your changes (`git commit -m 'Add amazing feature'`).
5. Push to the branch (`git push origin feature/amazing-feature`).
6. Open a Pull Request.

Please review our `IMPLEMENTATION_ROADMAP.md` and `PROJECT_PLAN.md` to see where we currently need help.

---

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
