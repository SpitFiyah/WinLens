# Privacy Debt Auditor - Development Environment Setup Checklist

**Purpose:** Verify all dependencies are installed before starting Day 1  
**Time Required:** 1-2 hours  
**Status:** Ready for verification

---

## QUICK CHECK (5 minutes)

Run these commands to verify your environment is ready:

```powershell
# Rust
rustc --version  # Should output: rustc 1.75+ 
cargo --version  # Should output: cargo 1.75+

# Node
node --version   # Should output: v18+ 
npm --version    # Should output: v9+

# Visual C++
# Look for: Visual Studio Build Tools or MSVC compiler
# Can verify by: cargo build (should work without errors)

# SQLite
sqlite3 --version  # Should output: SQLite 3.40+

# Git
git --version    # Should output: git 2.40+
```

**If all green:** Skip to "Final Setup" section  
**If any red:** Follow installation guide below

---

## DETAILED SETUP GUIDE

### 1. Rust + Cargo (Required)

**Check if installed:**
```powershell
rustc --version
cargo --version
```

**If not installed:**

1. Go to https://rustup.rs/
2. Download and run `rustup-init.exe`
3. Choose default installation (option 1)
4. When prompted "Proceed with installation?", type `y`
5. Close and reopen PowerShell
6. Verify: `rustc --version`

**Expected output:**
```
rustc 1.75.0 (2024-01-01 UTC)
cargo 1.75.0
```

**Install recommended tools:**
```powershell
# Code formatter
cargo install rustfmt

# Clippy (linter)
rustup component add clippy

# Flamegraph (for profiling)
cargo install flamegraph
```

**VS Code Extensions:**
- Install "rust-analyzer" (search in Extensions)
- Install "CodeLLDB" (debugging Rust)
- Install "Even Better TOML" (Cargo.toml editing)

**Time Required:** 10-15 minutes  
**Disk Space:** 2-3 GB

---

### 2. Node.js + npm (Required)

**Check if installed:**
```powershell
node --version
npm --version
```

**If not installed:**

1. Go to https://nodejs.org/ 
2. Download LTS version (18.x or higher)
3. Run installer, accept defaults
4. **During install, check:** ✅ Add to PATH
5. Restart PowerShell
6. Verify: `node --version`

**Expected output:**
```
v18.17.0
9.8.1
```

**Install recommended global tools:**
```powershell
npm install -g typescript  # TypeScript compiler
npm install -g @tauri-cli/cli  # Tauri CLI
```

**VS Code Extensions:**
- Install "ES7+ React/Redux/React-Native snippets"
- Install "Prettier - Code formatter"
- Install "ESLint"

**Time Required:** 5-10 minutes  
**Disk Space:** 500 MB

---

### 3. Visual C++ Build Tools (Required for Rust)

**Check if installed:**
```powershell
# Try building a Rust project
cargo build
# If it works without errors, you have build tools
```

**If not installed:**

**Option A: Quick Install (Recommended)**
1. Go to https://visualstudio.microsoft.com/downloads/
2. Look for "Visual Studio Build Tools"
3. Download and run
4. During installation, select:
   - ✅ "Desktop development with C++"
   - ✅ "C++ build tools core features"
5. Install (~5 GB)

**Option B: Full Visual Studio (if you prefer IDE)**
1. Download full "Visual Studio Community"
2. During installation, select C++ workload
3. Install (~30 GB)

**Verify installation:**
```powershell
cargo new test_project
cd test_project
cargo build --release
# Should complete without errors
cd ..
rmdir test_project -Recurse
```

**Time Required:** 10-20 minutes (plus 5-10 min download)  
**Disk Space:** 5-10 GB

---

### 4. SQLite (Required)

**Check if installed:**
```powershell
sqlite3 --version
```

**If not installed:**

**Option A: Scoop (Easiest)**
```powershell
# Install Scoop first if needed
iex (New-Object Net.WebClient).DownloadString('https://get.scoop.sh')

# Then install SQLite
scoop install sqlite
```

**Option B: Download Binary**
1. Go to https://www.sqlite.org/download.html
2. Download `sqlite-tools-win32-x86-3440200.zip`
3. Extract to `C:\SQLite`
4. Add to PATH:
   - Open Environment Variables (Win+X, "Environment")
   - Add `C:\SQLite` to PATH
5. Restart PowerShell
6. Verify: `sqlite3 --version`

**Optional: SQLite Browser**
- Download "DB Browser for SQLite" from https://sqlitebrowser.org/
- Useful for debugging database during development

**Time Required:** 5 minutes  
**Disk Space:** 5 MB

---

### 5. Git (Required)

**Check if installed:**
```powershell
git --version
```

**If not installed:**

1. Go to https://git-scm.com/download/win
2. Download Git for Windows
3. Run installer, accept defaults
4. Restart PowerShell
5. Verify: `git --version`

**Configure Git:**
```powershell
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

**Time Required:** 5 minutes  
**Disk Space:** 300 MB

---

### 6. Docker (Optional, for testing)

**Not required for MVP, but useful for**
- Testing isolated environments
- Cross-platform binary builds
- Performance benchmarking

**Install if interested:**
1. Download "Docker Desktop" from https://www.docker.com/products/docker-desktop/
2. Run installer
3. Enable WSL 2 integration when prompted
4. Restart Windows
5. Verify: `docker --version`

**Time Required:** 5 minutes + restart  
**Disk Space:** 1-2 GB

---

## DEVELOPMENT ENVIRONMENT STRUCTURE

Create this directory structure on your dev machine:

```
D:/
├── WinLens/                      # Main project directory
│   ├── src-tauri/                # Rust backend (after Day 1)
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── ...
│   ├── src/                      # React frontend (after Day 1)
│   ├── test_data/                # 50k sample files for benchmarking (create Day 2)
│   ├── IMPLEMENTATION_ROADMAP.md # (this file)
│   ├── QUICK_START.md
│   ├── TECHNICAL_RISKS.md
│   ├── PROJECT_PLAN.md
│   └── Product_design.md
```

---

## VERIFY ENVIRONMENT WORKS

Run this test to confirm everything is set up:

### Test 1: Create Rust Project
```powershell
cd D:\temp
cargo new test_rust_app
cd test_rust_app
cargo build --release
# Should complete without errors
cargo run --release
# Should print "Hello, world!"
cd ..
rm -Recurse test_rust_app
```

**Expected:** Completes in 30-60 seconds  
**If fails:** Check Rust/MSVC installation

---

### Test 2: Create Node Project
```powershell
cd D:\temp
npm create react-app test_react_app
cd test_react_app
npm run build
# Should complete without errors
cd ..
rm -Recurse test_react_app
```

**Expected:** Completes in 2-3 minutes  
**If fails:** Check Node/npm installation

---

### Test 3: SQLite Database
```powershell
cd D:\temp
sqlite3 test.db "CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT); INSERT INTO test VALUES (1, 'Hello'); SELECT * FROM test;"
# Should output: 1|Hello
del test.db
```

**Expected:** Shows "1|Hello"  
**If fails:** Check SQLite installation

---

## TROUBLESHOOTING

### Issue: `rustc` not found
**Solution:**
1. Verify Rust installed: `C:\Users\{USERNAME}\.cargo\bin\rustc.exe`
2. If missing, reinstall from https://rustup.rs/
3. Verify it was added to PATH:
   ```powershell
   $env:Path -split ';' | Select-String cargo
   ```

### Issue: Visual C++ build tools missing
**Solution:**
```powershell
# Try building a small Rust project
cargo new test
cd test
cargo build
```
If error mentions "Microsoft Visual C++", install Build Tools as described in section 3.

### Issue: npm extremely slow
**Solution:**
```powershell
# Use npm cache clean
npm cache clean --force

# Or use alternative package manager
npm install -g pnpm  # Faster alternative
pnpm install  # Use instead of npm install
```

### Issue: Port 3000 already in use
**Solution:**
```powershell
# Find process using port 3000
netstat -ano | findstr :3000

# Kill process (replace {PID} with process ID)
taskkill /PID {PID} /F
```

---

## RECOMMENDED VS CODE EXTENSIONS

**Essential:**
- rust-analyzer (rust)
- CodeLLDB (debugging)
- ES7+ React/Redux/React-Native snippets (react)
- Prettier (formatting)
- ESLint (linting)

**Recommended:**
- Thunder Client (API testing)
- SQLite (database browsing)
- Git Lens (git history)
- GitHub Copilot (coding assistance)

**Install all at once:**
```powershell
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension dsznajder.es7-react-js-snippets
code --install-extension esbenp.prettier-vscode
code --install-extension dbaeumer.vscode-eslint
```

---

## ENVIRONMENT VARIABLES (Optional)

Add these to make development easier:

```powershell
# Edit: System Properties → Environment Variables → New

RUST_BACKTRACE=1          # Better error messages in Rust
RUST_LOG=debug            # Logging for troubleshooting
NODE_ENV=development      # React development mode
```

Restart PowerShell after adding these.

---

## FINAL CHECKLIST (Day 1 Morning)

Before you write any code, verify:

- [ ] `rustc --version` works (outputs 1.75+)
- [ ] `cargo --version` works (outputs 1.75+)
- [ ] `node --version` works (outputs 18+)
- [ ] `npm --version` works (outputs 9+)
- [ ] `sqlite3 --version` works (outputs 3.40+)
- [ ] `git --version` works (outputs 2.40+)
- [ ] VS Code has rust-analyzer installed
- [ ] VS Code has Prettier installed
- [ ] Can create new Rust project: `cargo new test` → `cargo build`
- [ ] Can create new Node project: `npm create react-app test`
- [ ] Can open SQLite database: `sqlite3 test.db`

**All green?** You're ready for Day 1! ✅

---

## ADDITIONAL RESOURCES

### Learning Resources
- [Rust Book (Getting Started)](https://doc.rust-lang.org/book/ch01-01-installation.html)
- [npm Getting Started](https://docs.npmjs.com/getting-started)
- [Tauri Setup Guide](https://tauri.app/v1/guides/getting-started/setup/)

### Quick Help
- Rust docs: `rustup docs` (opens in browser)
- npm docs: `npm docs` (opens npm site)
- SQLite docs: `sqlite3` → `.help` (shows SQL help)

### Community
- Rust: https://users.rust-lang.org/
- Tauri: https://discord.gg/tauri
- React: https://stackoverflow.com/questions/tagged/reactjs

---

## NEXT STEPS

Once environment is verified:

1. Read [QUICK_START.md](QUICK_START.md) (5 min)
2. Read [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) (30 min)
3. Skim [TECHNICAL_RISKS.md](TECHNICAL_RISKS.md) (10 min)
4. Day 1: Start Phase 1 (project setup)

**Total prep time:** 2-3 hours (includes environment setup + reading)

---

**Questions?** Check [TECHNICAL_RISKS.md](TECHNICAL_RISKS.md) troubleshooting section.

Good luck! 🚀
