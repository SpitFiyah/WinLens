# Privacy Debt Auditor - Implementation Roadmap

**Created:** May 24, 2026  
**Status:** Ready for Execution  
**Target Release:** MVP in 2-3 weeks, Production-ready in 6-8 weeks

---

## EXECUTIVE SUMMARY

This roadmap delivers a working Privacy Debt Auditor MVP in **2-3 weeks** with core scanning capabilities, then iteratively adds visualization and advanced features. The build order prioritizes unblocking parallel work and getting early wins with basic scanning functionality.

**MVP Definition:**
- Filesystem scanning + secret detection
- Browser artifact analysis (Chrome/Edge basics)
- Privacy score calculation
- Functional UI (no sophisticated visualization)
- Exportable findings

**Key Principle:** Build backend APIs first → validate with simple frontend → iterate on visualization.

---

## PART 1: DEPENDENCY MAP & BUILD ORDER

### Dependency Graph

```
PROJECT FOUNDATION (PHASE 1)
├─ Rust workspace + Cargo setup
├─ Tauri + React scaffolding
├─ SQLite schema
└─ Rust-React IPC bridge

├─ BLOCKING: All cores require this

CORE BACKEND ENGINES (PHASE 2)
├─ Filesystem Scanner (foundation for others)
│  └─ USED BY: Secret Detection, Metadata Analysis
├─ Secret Detection Engine
│  └─ USED BY: Dashboard findings, Privacy Score
├─ Privacy Score System
│  └─ USED BY: Dashboard, Export
├─ Windows Artifact Analysis (parallel with above)
├─ Browser Analysis (parallel)
└─ Metadata Detection (parallel, lower priority)

├─ BLOCKING FOR: UI rendering, visualization

FRONTEND UI LAYER (PHASE 3)
├─ Dashboard layout + API integration
├─ Progress tracking UI
├─ Findings detail view
├─ Settings/config panel

├─ BLOCKING FOR: Visualization

VISUALIZATION & POLISH (PHASE 4)
├─ Heatmap rendering (D3.js)
├─ Relationship graphs (Cytoscape.js)
├─ Timeline views
├─ Export + remediation UX

ADVANCED FEATURES (PHASE 5)
├─ Real-time monitoring
├─ AI-powered insights
├─ Behavioral timeline reconstruction
└─ Custom rules engine
```

### Critical Path Analysis

**Minimum for working MVP:**
1. ✅ Project foundation + IPC bridge
2. ✅ Filesystem scanner (basic)
3. ✅ Secret detection (regex-based)
4. ✅ Privacy score (simple weighted calculation)
5. ✅ Browser cookie/history parsing
6. ✅ Basic React dashboard
7. ✅ Findings table/list view

**Unblocks advanced features:**
- Once 1-7 complete, visualization, Windows artifacts, and metadata analysis can proceed in parallel

---

## PART 2: DETAILED BUILD ORDER & EFFORT ESTIMATES

### PHASE 1: FOUNDATION (Days 1-2)
**Goal:** Working Rust ↔ React IPC pipeline with database schema

#### 1.1 Rust Project Setup
**Effort:** 4 hours  
**Tasks:**
- [ ] Create `/src-tauri/` workspace
- [ ] Initialize Cargo with proper dependencies:
  - `tokio` (async runtime)
  - `walkdir` (filesystem recursion)
  - `regex` (pattern matching)
  - `windows-rs` (Windows API access)
  - `rusqlite` + `sqlx` (database)
  - `serde`/`serde_json` (serialization)
  - `serde-wasm-bindgen` (Rust→JS)
- [ ] Configure release optimizations (LTO, codegen)
- [ ] Set up workspace structure

**Dependencies:** None  
**Unblocks:** All backend work

---

#### 1.2 Tauri + React Frontend Setup
**Effort:** 3 hours  
**Tasks:**
- [ ] Bootstrap Tauri project with `tauri init`
- [ ] Remove Tauri default frontend, install React + TypeScript
- [ ] Install dependencies:
  - `react`, `react-dom`, `typescript`
  - `axios` or `@tauri-apps/api` for IPC
  - `tailwindcss` (styling foundation)
  - `recharts` (basic charts for MVP)
- [ ] Create `/src/components/` directory structure
- [ ] Set up ESLint + Prettier

**Dependencies:** 1.1  
**Unblocks:** All frontend work

---

#### 1.3 Rust-React IPC Bridge
**Effort:** 5 hours  
**Tasks:**
- [ ] Define command API contract (JSON schema):
  ```
  Commands:
  - start_scan(scan_config: ScanConfig) → ScanStatus
  - get_scan_progress() → Progress
  - get_findings(filter: FindingsFilter) → [Finding]
  - compute_privacy_score() → Score
  - export_report(format: "json"|"pdf") → Path
  ```
- [ ] Implement Rust handlers for each command
- [ ] Create React hooks for IPC (`useScan`, `useFindings`)
- [ ] Set up error handling & type safety (TypeScript interfaces ↔ Rust types)
- [ ] Test IPC round-trip with dummy data

**Dependencies:** 1.1, 1.2  
**Unblocks:** All feature development

**Testing Strategy:** Create simple test UI that fires each command and displays responses

---

#### 1.4 SQLite Schema & Initialization
**Effort:** 4 hours  
**Tasks:**
- [ ] Design normalized schema:
  ```sql
  -- Core findings table
  CREATE TABLE findings (
    id INTEGER PRIMARY KEY,
    scan_id TEXT,
    finding_type TEXT (secret|metadata|browser|artifact),
    severity TEXT (critical|high|medium|low),
    title TEXT,
    description TEXT,
    file_path TEXT,
    value_preview TEXT (encrypted/hashed for secrets),
    created_at TIMESTAMP,
    remediation_id INTEGER
  );
  
  -- Scan metadata
  CREATE TABLE scans (
    id TEXT PRIMARY KEY,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    status TEXT (running|completed|error),
    total_files_scanned INTEGER,
    files_with_findings INTEGER,
    total_findings INTEGER,
    privacy_score REAL
  );
  
  -- Browser artifacts
  CREATE TABLE browser_artifacts (
    id INTEGER PRIMARY KEY,
    browser TEXT (chrome|edge|firefox),
    artifact_type TEXT (cookies|history|tracker),
    domain TEXT,
    count INTEGER,
    last_seen TIMESTAMP
  );
  
  -- Windows artifacts
  CREATE TABLE windows_artifacts (
    id INTEGER PRIMARY KEY,
    artifact_type TEXT (registry|jumplist|recent),
    artifact_name TEXT,
    artifact_path TEXT,
    metadata TEXT (JSON)
  );
  
  -- Remediation guidance
  CREATE TABLE remediations (
    id INTEGER PRIMARY KEY,
    finding_type TEXT,
    pattern TEXT,
    guidance TEXT,
    auto_fix_available BOOLEAN,
    priority INTEGER
  );
  ```
- [ ] Create database initialization function (run once on startup)
- [ ] Add migration system (for future schema updates)
- [ ] Create query helpers (insert_finding, get_findings_by_type, etc.)

**Dependencies:** 1.1  
**Unblocks:** Backend data persistence

---

#### 1.5 Build Pipeline & CI Setup
**Effort:** 3 hours  
**Tasks:**
- [ ] Create `Taskfile.yml` or Makefile:
  ```bash
  make dev      # Run in dev mode with hot reload
  make build    # Create release binary
  make test     # Run all tests
  make bench    # Run performance benchmarks
  ```
- [ ] Configure Tauri build settings (`tauri.conf.json`)
- [ ] Set up GitHub Actions (or equivalent) for CI
- [ ] Create cross-platform build targets (Windows native, WSL)

**Dependencies:** 1.1, 1.2  
**Unblocks:** All development

---

**PHASE 1 COMPLETION GATES:**
- [ ] `cargo build` succeeds
- [ ] `npm run tauri dev` launches app
- [ ] IPC test UI shows command round-trips working
- [ ] SQLite database created and queryable
- [ ] Both developers can run full dev environment

---

### PHASE 2: CORE BACKEND ENGINES (Days 3-7)

#### 2.1 Filesystem Scanner (CRITICAL PATH)
**Effort:** 8 hours  
**Priority:** HIGHEST (enables 2.2, 2.3, 2.6)  

**Tasks:**
- [ ] **2.1.1** Implement concurrent directory traversal using `walkdir`:
  ```rust
  async fn scan_filesystem(
    root_paths: Vec<PathBuf>,
    exclude_patterns: Vec<String>,
    progress_callback: F,
  ) -> Result<Vec<FileMetadata>>
  ```
  - Filter out system directories (System32, Windows, AppData\Roaming\Microsoft)
  - Skip symbolic links to prevent infinite loops
  - Implement pause/resume capability
  
- [ ] **2.1.2** Build file prioritization system:
  ```
  Priority 1: Documents, Downloads, Desktop (~often contain secrets)
  Priority 2: User home directory (~sensitive data)
  Priority 3: ProgramData, Application directories
  Priority 4: Everything else
  ```
  
- [ ] **2.1.3** Collect metadata per file:
  ```rust
  pub struct FileMetadata {
    path: PathBuf,
    size: u64,
    modified_time: SystemTime,
    mime_type: String,
    entropy: f32,  // For preliminary secret scoring
  }
  ```
  
- [ ] **2.1.4** Implement progress tracking:
  - Emit progress events: `(files_scanned, files_remaining, current_path)`
  - Update React UI in real-time via IPC
  
- [ ] **2.1.5** Performance optimization:
  - Use Tokio tasks for parallel directory traversal
  - Cache MIME type detection
  - Implement file size limits (skip files > 100MB)
  
- [ ] **2.1.6** Testing:
  - Unit tests for path filtering
  - Integration tests with sample directory trees
  - Benchmark for 100k file scan

**Dependency Issues to Solve:**
- How to make progress updates from Tauri → React? (Answer: emit events via IPC channel)
- How to handle permission errors gracefully? (Answer: Log and skip, continue scan)

**Dependencies:** 1.1, 1.3, 1.4  
**Unblocks:** 2.2, 2.3, 2.6, all secret detection

---

#### 2.2 Secret Detection Engine
**Effort:** 8 hours  
**Parallel with:** 2.1 (but starts after 2.1 begins)  

**Tasks:**
- [ ] **2.2.1** Implement pattern-based detection:
  ```rust
  pub struct SecretPattern {
    name: String,
    regex: Regex,
    confidence: f32,  // 0.7-1.0
    severity: Severity,  // Critical/High/Medium
  }
  
  static PATTERNS: &[SecretPattern] = &[
    // API Keys
    SecretPattern {
      name: "AWS_ACCESS_KEY",
      regex: Regex::new(r"AKIA[0-9A-Z]{16}"),
      confidence: 0.99,
      severity: Critical,
    },
    // JWT tokens
    SecretPattern {
      name: "JWT_TOKEN",
      regex: Regex::new(r"eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]*"),
      confidence: 0.95,
      severity: High,
    },
    // Database passwords
    SecretPattern {
      name: "MYSQL_PASSWORD",
      regex: Regex::new(r"mysql://\w+:([^\s@]+)@"),
      confidence: 0.85,
      severity: Critical,
    },
    // Private keys (simplified)
    SecretPattern {
      name: "RSA_PRIVATE_KEY",
      regex: Regex::new(r"-----BEGIN RSA PRIVATE KEY-----"),
      confidence: 1.0,
      severity: Critical,
    },
    // ... 15-20 more patterns
  ];
  ```
  
- [ ] **2.2.2** Implement entropy analysis for additional confidence:
  ```rust
  fn shannon_entropy(s: &str) -> f32 {
    // Calculate Shannon entropy to detect high-randomness strings
    // Strings with entropy > 4.0 are likely secrets
  }
  ```
  
- [ ] **2.2.3** Build false-positive filtering:
  - Whitelist common false positives (test data, documentation)
  - Check against common non-sensitive patterns (regex examples, test keys)
  
- [ ] **2.2.4** Severity classification:
  ```rust
  pub enum Severity {
    Critical,  // API keys, private keys, passwords
    High,      // OAuth tokens, DB connection strings
    Medium,    // Email addresses, IP addresses
    Low,       // Sensitive file paths, internal URLs
  }
  ```
  
- [ ] **2.2.5** Integration with filesystem scanner:
  - For each file, sample first 1MB (to avoid scanning huge files)
  - Run secret detection in parallel with filesystem scan
  - Store matches in database as they're found

**Dependencies:** 2.1, 1.4  
**Unblocks:** Privacy score calculation

---

#### 2.3 Privacy Score System
**Effort:** 6 hours  
**Parallel with:** 2.1, 2.2  

**Tasks:**
- [ ] **2.3.1** Design scoring algorithm:
  ```
  PRIVACY SCORE (0-100, where 100 = maximum exposure)
  
  Factor 1: Secrets Found (Max 40 points)
    Critical secret: +15 per finding
    High secret: +10 per finding
    Medium secret: +5 per finding
    (Capped at 40)
  
  Factor 2: Browser Artifacts (Max 30 points)
    Tracking cookies: 2 points per domain
    History entries > 1 month old: +10 (indicates retention)
    Third-party tracking pixels: +5 per 10 found
    (Capped at 30)
  
  Factor 3: Sensitive File Patterns (Max 15 points)
    .env files: +3
    config files with passwords: +5 per file
    SSH keys: +7 per key
    (Capped at 15)
  
  Factor 4: Metadata Leaks (Max 15 points)
    EXIF data with geolocation: +8
    PDF with author/comments: +4
    Office docs with revision history: +3
    (Capped at 15)
  
  FINAL_SCORE = SUM(all factors)
  
  Display Tiers:
  0-20: Green "Minimal Exposure"
  21-40: Yellow "Moderate Exposure"
  41-60: Orange "Significant Exposure"
  61-80: Red "Severe Exposure"
  81-100: Dark Red "Critical Exposure"
  ```
  
- [ ] **2.3.2** Implement weighted calculation system:
  ```rust
  pub struct PrivacyScoreCalculator {
    secret_weight: f32,
    browser_weight: f32,
    metadata_weight: f32,
    artifacts_weight: f32,
  }
  
  impl PrivacyScoreCalculator {
    fn calculate(&self, findings: &[Finding]) -> PrivacyScore {
      // Aggregate findings, apply weights, return score
    }
  }
  ```
  
- [ ] **2.3.3** Create real-time score updates:
  - As each phase of scan completes, recalculate and emit updated score
  - React dashboard shows live score changes
  
- [ ] **2.3.4** Persist to database:
  - Store calculation breakdown per scan for transparency

**Dependencies:** 2.1, 2.2, 1.4  
**Unblocks:** Dashboard display

---

#### 2.4 Browser Privacy Analysis
**Effort:** 10 hours  
**Parallel with:** 2.1, 2.2, 2.3  

**Tasks:**
- [ ] **2.4.1** Chrome/Edge cookie parser:
  ```rust
  // Typical path: %APPDATA%\Google\Chrome\User Data\Default\Cookies (SQLite DB)
  async fn parse_chrome_cookies(profile_path: &Path) -> Result<Vec<BrowserCookie>> {
    // Open SQLite database
    // Extract: domain, name, value (encrypted), secure_flag, http_only, expiry
    // Categorize by tracking domain vs legitimate service
  }
  
  struct BrowserCookie {
    domain: String,
    name: String,
    is_tracking: bool,  // Detected via domain list
    is_secure: bool,
    is_http_only: bool,
    expires: Option<DateTime>,
    risk_level: RiskLevel,
  }
  ```
  
- [ ] **2.4.2** Browser history analyzer:
  ```rust
  async fn parse_browser_history(profile_path: &Path) -> Result<Vec<HistoryEntry>> {
    // Extract visit history with timestamps
    // Identify sensitive searches (medical, financial, etc.)
    // Detect retention age
  }
  ```
  
- [ ] **2.4.3** Tracker detection:
  - Implement public tracker domain list (from Disconnect, DuckDuckGo)
  - Classify domains as: ads, analytics, social, fingerprinting, etc.
  
- [ ] **2.4.4** localStorage/IndexedDB analysis:
  ```rust
  // Path: %APPDATA%\Google\Chrome\User Data\Default\Local Storage\
  async fn parse_local_storage(storage_path: &Path) -> Result<Vec<StorageEntry>> {
    // Extract stored data, identify PII patterns
  }
  ```
  
- [ ] **2.4.5** Firefox support:
  ```rust
  // paths: %APPDATA%\Mozilla\Firefox\Profiles\<random>.default\
  // Similar to Chrome but different formats (JSON+SQLite hybrid)
  ```
  
- [ ] **2.4.6** Testing:
  - Create test Chrome profile with sample cookies
  - Test tracker domain detection
  - Verify sensitive search detection

**Complexity Risks:**
- Chrome/Edge cookies encrypted? (Answer: Use Tauri access to Chrome API, or ask user to copy unencrypted data)
- Firefox has multiple profile formats? (Answer: Start with default profile, document workarounds)

**Dependencies:** 1.1, 1.4  
**Unblocks:** Privacy score, visualization data

---

#### 2.5 Windows Artifact Analysis (OPTIONAL for MVP)
**Effort:** 12 hours (skip for MVP, do in Phase 5)  
**Parallel with:** Others  

**Why skip for MVP:**
- Not visible to average user
- Require complex Windows API understanding (windows-rs learning curve)
- Can be added later without breaking existing features

**2.6 Metadata Leak Detection (OPTIONAL for MVP)
**Effort:** 8 hours (skip for MVP)  

**Why skip:**
- Requires image/PDF parsing libraries (adds complexity)
- Lower impact than filesystem secrets
- Can iterate on later

---

**PHASE 2 COMPLETION GATES (MVP):**
- [ ] Filesystem scan completes in < 30 seconds for 50k files
- [ ] At least 20 secret patterns implemented and tested
- [ ] Browser cookie/history parsing works for Chrome + Edge
- [ ] Privacy score calculation tested with sample findings
- [ ] All Rust functions have unit tests (>80% coverage)
- [ ] IPC commands fully functional from React test UI

---

### PHASE 3: FRONTEND UI & BASIC VISUALIZATION (Days 8-10)

#### 3.1 Dashboard Layout
**Effort:** 6 hours  
**Dependencies:** 2.1, 2.2, 2.3, 1.3  

**Tasks:**
- [ ] **3.1.1** Create main dashboard with:
  - [ ] Privacy Score widget (large, prominent)
  - [ ] Quick stats (files scanned, findings count, critical issues)
  - [ ] Scan progress bar (visible during scanning)
  - [ ] Navigation tabs (Summary, Secrets, Browser, Settings)

- [ ] **3.1.2** Implement scan progress UI:
  - [ ] Real-time file count updates
  - [ ] Current directory being scanned
  - [ ] ETA based on scan rate
  - [ ] Pause/Resume buttons

- [ ] **3.1.3** Create findings list view:
  ```jsx
  <FindingsList 
    findings={findings}
    filterBy={['type', 'severity']}
    sortBy={['date', 'severity']}
    onSelectFinding={(finding) => showDetail(finding)}
  />
  ```
  - Searchable/filterable table
  - Color-coded severity
  - Quick actions (copy, remediate, ignore)

- [ ] **3.1.4** Settings panel:
  - Scan location selection
  - Exclusion patterns
  - Browser selections
  - Privacy policy clarity

**Dependencies:** Phase 2 core components  
**Unblocks:** Visualization, export

---

#### 3.2 Basic Visualization (Charts, not graphs)
**Effort:** 4 hours  
**Dependencies:** 3.1  

**Tasks:**
- [ ] **3.2.1** Privacy score breakdown (pie chart):
  ```jsx
  <ResponsivePie
    data={[
      { id: 'Secrets', value: 40 },
      { id: 'Browser', value: 30 },
      { id: 'Metadata', value: 15 },
      { id: 'Artifacts', value: 15 },
    ]}
  />
  ```
  Uses Recharts

- [ ] **3.2.2** Findings by severity (bar chart):
  - Critical, High, Medium, Low counts

- [ ] **3.2.3** Top sensitive file types (horizontal bar):
  - .env, .config, .key, etc.

**Why skip sophisticated D3/Cytoscape for MVP:**
- Takes 2+ weeks to learn
- Basic charts get MVP out faster
- Can upgrade to sophisticated graphs in Phase 4

---

#### 3.3 Findings Detail View
**Effort:** 4 hours  
**Dependencies:** 3.1  

**Tasks:**
- [ ] **3.3.1** Detail card for each finding:
  - Finding type + severity
  - File path
  - Value preview (hashed/encrypted for secrets)
  - Risk explanation
  - Remediation guidance
  - Action buttons (copy, delete, ignore)

- [ ] **3.3.2** Remediation UI:
  - Display suggested fixes
  - One-click remediation (if safe)
  - Manual remediation steps

---

**PHASE 3 COMPLETION GATES (MVP):**
- [ ] Dashboard loads and displays real scanning data
- [ ] All findings from backend appear in UI
- [ ] Filtering, sorting, searching works
- [ ] Privacy score updates in real-time during scan
- [ ] UI responsive on 1920x1080 and 1366x768 resolutions
- [ ] No performance lag with 1000+ findings

---

### PHASE 4: SOPHISTICATED VISUALIZATION (Days 11-14)

#### 4.1 Privacy Heatmap (D3.js)
**Effort:** 12 hours  
**Dependencies:** 3.1, all backend phases  

**Deferred, NOT in MVP:**
- Too complex for early release
- Can be added once core functionality proven
- Requires D3.js expertise + user research

#### 4.2 Relationship Graph (Cytoscape.js)
**Effort:** 14 hours  
**Deferred, NOT in MVP**

#### 4.3 Timeline View
**Effort:** 8 hours  
**Deferred, NOT in MVP**

---

### PHASE 5: ADVANCED FEATURES (Days 15+)

All of these are POST-MVP:
- Real-time file monitoring
- Custom rule engine
- AI-powered insights (local ONNX model)
- Behavioral timeline reconstruction
- Windows Registry analysis
- Automatic remediation workflows

---

## PART 3: CRITICAL TECHNICAL RISKS & MITIGATION

### Risk 1: Rust/Tauri Learning Curve
**Risk Level:** HIGH (if team is new to Rust)  
**Impact:** 2-3 week delay  

**Mitigation:**
- [ ] Pair Rust developer (or hire contractor) with team for first week
- [ ] Use Tauri examples as boilerplate, not learning material
- [ ] Pre-build common patterns (IPC, async tasks, database access)
- [ ] Set up code review process early (catches mistakes before they compound)
- [ ] Allocate Day 0 for "Rust fundamentals" video course (4-6 hours)

**Success Metric:** IPC bridge working by end of Day 1

---

### Risk 2: Performance - Filesystem Scanning Too Slow
**Risk Level:** MEDIUM  
**Impact:** Scan takes 2+ minutes for 100k files (unacceptable UX)  

**Mitigation:**
- [ ] Benchmark on Day 2: scan 100k file test directory, measure time
- [ ] If >30 seconds: implement optimizations in priority order:
  1. Parallel directory traversal (Tokio tasks)
  2. Skip files > 50MB
  3. Batch database inserts (insert 1000 at a time)
  4. Cache MIME type detection
  5. Use memory-mapped files for secret detection

- [ ] Set performance targets early:
  - 50k files: < 20 seconds
  - 100k files: < 40 seconds
  - 500k files: < 3 minutes

**Testing:** Create benchmark test with `/test_data/` directory of 100k sample files

---

### Risk 3: Browser Artifact Parsing Complexity
**Risk Level:** MEDIUM  
**Impact:** Chrome/Firefox parsing fails, no browser findings in MVP  

**Mitigation:**
- [ ] Start with Chrome only (safest, most common)
- [ ] Test on 3 real Chrome profiles (yours, colleague's, fake test profile)
- [ ] If encryption issues arise, document workaround:
  - User exports browser data via built-in tools
  - Tool imports pre-exported data
- [ ] Implement graceful degradation:
  - If Chrome parser fails → log error, continue with other scans
  - Show user: "Browser data unavailable (try manually exporting)"

**Testing:** Integration test with real Chrome profile

---

### Risk 4: Windows API (windows-rs) Complexity
**Risk Level:** MEDIUM-HIGH  
**Impact:** Registry/JumpList parsing delayed or buggy  

**Mitigation:**
- [ ] SKIP for MVP (not critical for MVP)
- [ ] Defer Windows artifact analysis to Phase 5
- [ ] Use existing Rust crates instead of raw Windows API:
  - `winreg` for Registry
  - `jumplist-parser` if available
- [ ] Test extensively on Windows 10 + 11 (different APIs)

---

### Risk 5: Visualization Performance - Rendering 1000+ Nodes
**Risk Level:** LOW (for MVP, which uses simple charts)  
**Impact:** Graph visualization lags or crashes  

**Mitigation:**
- [ ] Skip D3/Cytoscape for MVP (use Recharts)
- [ ] When building advanced visualization (Phase 4):
  - Implement virtual scrolling / dynamic rendering
  - Limit graph size (show top 100 connections, load on demand)
  - Use WebGL renderer (three.js) for 1000+ nodes
  - Test with worst-case data (50k findings)

---

### Risk 6: Database Bloat - SQLite Slow with Large Scans
**Risk Level:** MEDIUM  
**Impact:** Queries get slow as database grows  

**Mitigation:**
- [ ] Design indexes upfront:
  ```sql
  CREATE INDEX idx_findings_severity ON findings(severity);
  CREATE INDEX idx_findings_type ON findings(finding_type);
  CREATE INDEX idx_scans_created ON scans(created_at);
  ```
- [ ] Set retention policy (keep last 10 scans, archive others)
- [ ] Implement query pagination (limit 100 results at a time)
- [ ] Benchmark with 100k findings: ensure queries < 100ms

---

### Risk 7: Privacy Violations - Tool Becomes Privacy Leak
**Risk Level:** CRITICAL  
**Impact:** User data leaks during export or caching  

**Mitigation:**
- [ ] NEVER store unencrypted secret values in database
- [ ] Store only: finding_type, severity, file_path, hash(value)
- [ ] Implement access controls:
  - Value previews shown only on device
  - Export masked (hash only, no actual values)
  - Clear cache on exit
- [ ] Code review: anyone handling sensitive data gets 2nd eyes
- [ ] Testing:
  - Manual verification that .json export contains no actual secret values
  - Audit database for unencrypted PII

---

### Risk 8: Secret Pattern False Positives
**Risk Level:** MEDIUM  
**Impact:** User spam-reported for "false alarms"  

**Mitigation:**
- [ ] Test each pattern against:
  - 100 real false positives (documentation, test code, comments)
  - Verify <1% false positive rate
- [ ] Implement multi-stage confirmation:
  - Regex match (loose)
  - Entropy check (strict)
  - Context analysis (is it in a comment?)
- [ ] Allow user to "whitelist" patterns per directory
- [ ] Include pattern confidence scores in findings (user sees "95% confidence")

---

## PART 4: MVP SCOPE DEFINITION

### MUST HAVE (Core MVP)
- [x] Filesystem scanner (all user-accessible locations)
- [x] Secret detection (30+ patterns: API keys, passwords, tokens, private keys)
- [x] Privacy score calculation (basic weighted system)
- [x] Chrome/Edge cookie analysis
- [x] Chrome/Edge history analysis (timestamp + URL)
- [x] Browser tracker detection (basic domain list)
- [x] React dashboard with:
  - [ ] Privacy score display
  - [ ] Findings list (searchable, sorterable)
  - [ ] Real-time progress tracking
  - [ ] Basic charts (pie, bar)
- [x] SQLite database for persistence
- [x] JSON export of findings
- [x] Settings (scan locations, exclusions)

**MVP Effort:** ~35 hours of focused development = 5-7 calendar days with focused team

---

### SHOULD HAVE (Post-MVP Phase 4)
- [ ] D3.js heatmap visualization
- [ ] Cytoscape.js relationship graph
- [ ] Firefox support
- [ ] Sensitive metadata detection (EXIF, PDF metadata)
- [ ] PDF export reports
- [ ] Custom rule engine
- [ ] Auto-remediation (safe operations like deleting .env files)

---

### NICE TO HAVE (Phase 5+)
- [ ] Real-time file monitoring
- [ ] Windows Registry analysis
- [ ] Jump Lists parser
- [ ] Thumbnail cache analysis
- [ ] AI-powered insights (local LLM)
- [ ] Behavioral timeline reconstruction
- [ ] Decentralized threat intelligence (peer sharing)
- [ ] macOS/Linux support

---

## PART 5: DEVELOPMENT WORKFLOW

### How to Parallelize Work

**Scenario: 2-3 developer team**

**Team Assignment:**

**Developer A (Backend Lead):**
- Phase 1: Rust setup, IPC bridge (1-2 days)
- Phase 2: Filesystem scanner, secret detection (3-4 days)
- Phase 2: Privacy score (1 day)
- Phase 4: Advanced features (after MVP)

**Developer B (Frontend Lead):**
- Phase 1: React setup, test UI for IPC validation (1 day)
- Phase 2: Helper (test backend findings, validate IPC) (1 day)
- Phase 3: Dashboard UI, findings components (2-3 days)
- Phase 4: Visualization work

**Developer C (if available):**
- Phase 2: Browser analysis (Chrome/Edge parsing) (2 days, parallel)
- Phase 2: Integration testing (runs full backend tests) (1 day)
- Phase 3: Testing + QA (1 day)
- Phase 4: Visualization

---

### Testing Strategy for Offline-First Architecture

#### Unit Tests (Tier 1)
**Coverage:** 80%+  
**Tools:** `cargo test`, `jest`

```rust
#[test]
fn test_secret_pattern_matching() {
  let pattern = SecretPattern::aws_key();
  assert!(pattern.matches("AKIA1234567890ABCDEF"));
  assert!(!pattern.matches("AKIATEST123"));
}

#[test]
fn test_filesystem_traversal_respects_excludes() {
  let excludes = vec!["System32", "AppData"];
  let files = scan_filesystem(root, excludes);
  assert!(!files.iter().any(|f| f.contains("System32")));
}
```

#### Integration Tests (Tier 2)
**Coverage:** Critical paths  
**Tools:** Custom test framework + fixtures

```rust
#[test]
fn test_full_scan_pipeline() {
  // 1. Create test directory with sample files
  let test_dir = setup_test_fixtures();
  
  // 2. Run full scan
  let results = scan_full(test_dir);
  
  // 3. Verify findings
  assert!(results.secrets.len() > 0);
  assert!(results.privacy_score > 0.0);
  
  // 4. Verify database persistence
  assert_eq!(db_query("SELECT COUNT(*) FROM findings"), results.secrets.len());
}
```

#### System Tests (Tier 3)
**Coverage:** End-to-end scenarios  
**Manual QA checklist:**

```
Pre-MVP Checklist:
- [ ] Scan 50k test files, verify completes in <30s
- [ ] Identify 5+ known secrets in test data
- [ ] Privacy score updates in real-time during scan
- [ ] Chrome cookies parse without errors
- [ ] Export JSON contains no actual secret values
- [ ] UI responsive during long scans
- [ ] Pause/resume scan functionality works
- [ ] Database survives restart (persists findings)
```

#### Performance Benchmarks
**Run on Day 7, before Phase 3:**

```
Benchmark: scan_filesystem
  Input: /test_data/ (50k files, 2GB total)
  Target: < 20 seconds
  Actual: ___ seconds

Benchmark: secret_detection
  Input: 500 text files (1MB each)
  Target: < 5 seconds
  Actual: ___ seconds

Benchmark: query_findings
  Input: Database with 10k findings
  Query: SELECT * WHERE severity = "CRITICAL"
  Target: < 100ms
  Actual: ___ ms
```

---

### How to Test Backend Without Frontend

**Strategy:** Test UI in Rust binary

```rust
// src-tauri/src/bin/test_backend.rs
// Executable that runs all backend functions with sample data

#[tokio::main]
async fn main() {
  println!("Testing Filesystem Scanner...");
  let files = scan_filesystem(&test_dir);
  println!("  Found {} files", files.len());
  
  println!("Testing Secret Detection...");
  let secrets = detect_secrets(&files).await;
  println!("  Found {} secrets", secrets.len());
  
  println!("Testing Privacy Score...");
  let score = calculate_privacy_score(&secrets);
  println!("  Privacy Score: {}", score);
  
  println!("Testing Database...");
  store_findings(&secrets);
  let retrieved = query_findings();
  println!("  Stored and retrieved {} findings", retrieved.len());
}

// Run with: cargo run --bin test_backend
```

Then React test UI can call individual endpoints to validate.

---

### Continuous Integration (CI)

**GitHub Actions workflow:**

```yaml
name: CI

on: [push, pull_request]

jobs:
  backend:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --workspace
      - run: cargo build --release

  frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '18'
      - run: npm install
      - run: npm run test
      - run: npm run build
```

---

## PART 6: EXECUTION CHECKLISTS

### Development Environment Checklist

**Before Day 1, ensure:**

- [ ] **Rust Setup**
  - [ ] Rust 1.75+ installed (`rustc --version`)
  - [ ] Cargo configured
  - [ ] `rustup` updated
  - [ ] VS Code + rust-analyzer extension
  - [ ] Recommended extensions:
    - rust-analyzer (official)
    - CodeLLDB (debugging)
    - Even Better TOML
    - Error Lens

- [ ] **Node/Frontend Setup**
  - [ ] Node 18+ (`node --version`)
  - [ ] npm 9+ (`npm --version`)
  - [ ] VS Code + Prettier, ESLint extensions

- [ ] **Windows Setup**
  - [ ] Windows 10/11 with latest updates
  - [ ] Visual C++ build tools (required for Rust)
  - [ ] git configured
  - [ ] Admin access (some filesystem operations need it)

- [ ] **Database**
  - [ ] SQLite 3.40+ (usually included)
  - [ ] DB Browser for SQLite (optional, for debugging)

- [ ] **Build Tools**
  - [ ] make or Taskfile.yml for common commands
  - [ ] Docker (optional, for testing)

---

### Key Dependencies to Evaluate (Decision Gates)

**Before Phase 1.1, decide:**

| Dependency | Options | Recommendation | Reason |
|---|---|---|---|
| **Async runtime** | Tokio vs async-std | Tokio | Most mature, best Tauri integration |
| **Database** | SQLite vs RocksDB vs custom | SQLite | Simplicity, ACID, built-in Tauri support |
| **Secret patterns** | hand-coded vs library (detect-secrets) | Hand-coded | Full control, offline, no false positives |
| **Graph viz** | D3.js vs Cytoscape.js vs sigma.js | Cytoscape.js | Better for networks, cleaner API |
| **Charts** | Recharts vs Nivo vs Chart.js | Recharts | React-friendly, good defaults |
| **Styling** | Tailwind vs CSS-in-JS (styled-components) | Tailwind | Faster to prototype, consistent design |
| **Icons** | Feather vs Heroicons vs FontAwesome | Heroicons | Clean, modern, MIT licensed |
| **File encryption** | sodium (libsodium) vs RustCrypto | sodium | Audited, battle-tested |

---

### Security & Privacy Checkpoints

**Before each phase completion, verify:**

#### Phase 1 Checkpoints
- [ ] Database file encrypted at rest (SQLite extension)
- [ ] No secrets committed to git (check with gitleaks)
- [ ] Rust code scanned for unsafe blocks (review each one)

#### Phase 2 Checkpoints
- [ ] Test export: contains NO unencrypted secrets
- [ ] Test database: query findings returns hashed values only
- [ ] Code review: all secret handling paths audited
- [ ] Verify: no unencrypted values in memory dumps

#### Phase 3 Checkpoints
- [ ] UI shows value hashes only (no actual secret text)
- [ ] No console.log statements with sensitive data
- [ ] Cache cleared on app exit

#### Phase 4+ Checkpoints
- [ ] Security audit of entire codebase (external consultant)
- [ ] Penetration testing (attempt data extraction)
- [ ] Privacy review: verify tool itself doesn't leak

---

### Performance Benchmarks to Establish (Day 7)

**Create `BENCHMARKS.md` with these targets:**

```markdown
# Privacy Debt Auditor - Performance Benchmarks

## Baseline Metrics (Established Day 7)

### Filesystem Scanning
- [ ] 50,000 files / 2GB: _____ seconds (Target: <20s)
- [ ] 100,000 files / 5GB: _____ seconds (Target: <40s)
- [ ] Memory usage (max): _____ MB (Target: <500MB)
- [ ] False positive rate: ____% (Target: <1%)

### Secret Detection
- [ ] 500 files (1MB each): _____ seconds (Target: <5s)
- [ ] Entropy analysis speed: _____ files/sec (Target: >10k)
- [ ] Pattern matching accuracy: ____% (Target: >99%)

### Browser Analysis
- [ ] Chrome cookie parser: _____ seconds (Target: <2s)
- [ ] History parsing: _____ seconds (Target: <3s)
- [ ] Tracker detection: _____ seconds (Target: <1s)

### Database Operations
- [ ] Insert 10k findings: _____ seconds (Target: <5s)
- [ ] Query findings (filter): _____ ms (Target: <100ms)
- [ ] Full scan -> DB: _____ seconds (Target: <35s)

### React/UI
- [ ] Dashboard load: _____ ms (Target: <500ms)
- [ ] Findings list render (1k items): _____ ms (Target: <1000ms)
- [ ] Privacy score update: _____ ms (Target: <100ms)

### Binary Size
- [ ] Rust backend binary: _____ MB (Target: <50MB)
- [ ] Tauri app package: _____ MB (Target: <100MB)

---

## Regression Testing (Run before each release)

- [ ] All above benchmarks pass
- [ ] No memory leaks (check Valgrind/profiler)
- [ ] CPU usage during scan <50%
```

---

## PART 7: TIMELINE SUMMARY

### Critical Path (Minimum path to MVP)

```
Day 1-2:  Project Foundation (Phase 1)
          ✓ Rust + React setup
          ✓ IPC bridge working
          ✓ Database schema created

Day 3-4:  Core Scanners (Phase 2.1-2.2)
          ✓ Filesystem scanner functional
          ✓ Secret detection patterns implemented
          ✓ Test UI validates backend

Day 5:    Privacy Score + Browser Analysis (Phase 2.3-2.4)
          ✓ Score calculation working
          ✓ Chrome/Edge parsing complete

Day 6-7:  Dashboard UI (Phase 3)
          ✓ Findings list view
          ✓ Real-time progress
          ✓ Basic charts

Day 8-9:  Integration, Testing, Polish
          ✓ Full end-to-end scan works
          ✓ UI responsive
          ✓ Export functional

Day 10:   MVP Launch
          ✓ Executable built
          ✓ Tested on clean Windows 10/11
          ✓ Documentation complete

Day 11+:  Advanced Features (Phase 4-5)
```

### Recommended Full Timeline

```
Week 1: Foundation + Core Scanning
Week 2: Browser Analysis + Dashboard
Week 3: Testing, Performance, Polish, MVP Launch
Week 4-6: Visualization, Windows Artifacts, Advanced Features
Week 7-8: Security audit, final optimizations, Production Release
```

---

## PART 8: SUCCESS CRITERIA FOR MVP

When can you declare MVP "done"?

- [x] **Functional:**
  - Scans filesystem and detects secrets
  - Calculates privacy score
  - Parses browser data
  - All findings stored in database
  - Findings displayed in React UI
  - Export works (JSON format)

- [x] **Performance:**
  - Full scan of 50k files completes in <30 seconds
  - Privacy score updates live during scan
  - UI remains responsive during scan

- [x] **Security:**
  - No unencrypted secrets in database
  - No actual values in export
  - No telemetry or external calls

- [x] **UX:**
  - Dashboard visually clear and professional
  - Findings easy to understand
  - Scan progress visible
  - Help/remediation guidance provided

- [x] **Testing:**
  - >80% code coverage
  - All critical paths tested
  - Tested on Windows 10 + 11

- [x] **Documentation:**
  - README with setup instructions
  - Known limitations documented
  - Architecture diagram created

---

## APPENDIX A: Common Decisions

### Decision: Regex vs ML for Secret Detection?
**Chosen:** Regex (confidence: 85% accuracy)  
**Rationale:** Offline, deterministic, fast. ML would require model download.

### Decision: SQLite vs Custom Binary Format?
**Chosen:** SQLite (confidence: 95% accuracy)  
**Rationale:** Queryable, standard, easy debugging. Binary would be faster but unqueryable.

### Decision: Local LLM for Insights?
**Chosen:** Defer to Phase 5 (confidence: 70%)  
**Rationale:** Adds 100MB+ binary size, model download time. Basic MVP doesn't need it.

### Decision: Real-time Monitoring?
**Chosen:** Defer to Phase 5 (confidence: 80%)  
**Rationale:** Lower priority, requires file watcher complexity. MVP is one-shot scans.

### Decision: Cytoscape.js vs D3.js?
**Chosen:** Cytoscape.js (confidence: 80%)  
**Rationale:** Better for network graphs, cleaner API. D3 has steeper learning curve.

---

## APPENDIX B: Resources & Learning

### Rust Resources (for beginners)
- [Rust Book](https://doc.rust-lang.org/book/) - 6 hours, essential
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - 2 hours, for async
- [windows-rs docs](https://microsoft.github.io/windows-rs/) - reference

### Tauri Resources
- [Tauri Docs](https://tauri.app/v1/guides/getting-started/setup/) - 3 hours setup
- [IPC Examples](https://tauri.app/v1/guides/features/command/) - copy-paste from here

### React Visualization
- [Recharts Docs](https://recharts.org/) - 1 hour to get comfortable
- [Cytoscape.js Docs](https://js.cytoscape.org/) - 2 hours, steep learning curve

### Performance Profiling
- [Flamegraph (Rust)](https://www.brendangregg.com/flamegraphs.html)
- [Chrome DevTools (React)](https://developer.chrome.com/docs/devtools/)

---

**END OF ROADMAP**

Next steps: Print this document, assign team, start Day 1 tasks.
Good luck! 🚀
