# Privacy Debt Auditor - Project Plan & Execution Strategy

**Project Start Date:** May 24, 2026
**Status:** Planning & Setup Phase

---

## PROJECT OVERVIEW

A Windows-first, local-only privacy forensic auditor that identifies and visualizes personal data exposure.

**Tech Stack:**
- Backend: Rust (performance, safety, concurrency)
- Frontend: Tauri + React (native desktop)
- Visualization: D3.js/Cytoscape.js
- Database: SQLite
- Async: Tokio
- APIs: windows-rs for WinAPI access

---

## PHASE 1: PROJECT SETUP & ARCHITECTURE

### Phase 1 Subtasks
- [ ] **1.1** Initialize project directory structure
- [ ] **1.2** Set up Rust workspace & Cargo configuration
- [ ] **1.3** Set up Tauri + React frontend scaffolding
- [ ] **1.4** Configure SQLite database schema
- [ ] **1.5** Set up build pipeline & dependencies
- [ ] **1.6** Document API contracts between Rust backend & React frontend
- [ ] **1.7** Create development environment configuration

---

## PHASE 2: CORE BACKEND ENGINE

### 2.1 Privacy Debt Score™ System
- [ ] **2.1.1** Design scoring algorithm
- [ ] **2.1.2** Implement weighted factor calculation
- [ ] **2.1.3** Create real-time score updates
- [ ] **2.1.4** Build score persistence to database

### 2.2 Filesystem Scanner
- [ ] **2.2.1** Implement concurrent filesystem recursion
- [ ] **2.2.2** Add path exclusion logic
- [ ] **2.2.3** Build entropy analysis for secret detection
- [ ] **2.2.4** Implement MIME type detection
- [ ] **2.2.5** Add file filtering & prioritization

### 2.3 Secret Detection Engine
- [ ] **2.3.1** Build regex-based pattern matchers (JWT, API keys, etc.)
- [ ] **2.3.2** Implement entropy analysis for secrets
- [ ] **2.3.3** Add severity classification
- [ ] **2.3.4** Create remediation guidance database
- [ ] **2.3.5** Build false-positive filtering

### 2.4 Windows Artifact Analysis
- [ ] **2.4.1** Implement Registry scanner (windows-rs)
- [ ] **2.4.2** Add Jump Lists analyzer
- [ ] **2.4.3** Build Recent Files extractor
- [ ] **2.4.4** Create Thumbnail Cache analyzer
- [ ] **2.4.5** Implement ShellBags parser
- [ ] **2.4.6** Add temp file analysis

### 2.5 Browser Privacy Analysis
- [ ] **2.5.1** Implement Chrome/Edge cookie parser
- [ ] **2.5.2** Build browser history analyzer
- [ ] **2.5.3** Create tracker detection logic
- [ ] **2.5.4** Implement localStorage/IndexedDB analyzer
- [ ] **2.5.5** Add Firefox support

### 2.6 Metadata Leak Detection
- [ ] **2.6.1** Build image metadata extractor (EXIF)
- [ ] **2.6.2** Implement PDF metadata analyzer
- [ ] **2.6.3** Add Office document metadata extraction
- [ ] **2.6.4** Create severity mapping for metadata leaks

### 2.7 Database & Application Scanning
- [ ] **2.7.1** SQLite database parser
- [ ] **2.7.2** Implement Discord database analyzer
- [ ] **2.7.3** Add Telegram database analyzer
- [ ] **2.7.4** Create generic Electron app database scanner

---

## PHASE 3: FRONTEND UI & VISUALIZATION

### 3.1 React Component Architecture
- [ ] **3.1.1** Create main dashboard layout
- [ ] **3.1.2** Build scan progress UI
- [ ] **3.1.3** Implement findings detail views
- [ ] **3.1.4** Create settings & configuration panels

### 3.2 Privacy Heatmap Visualization
- [ ] **3.2.1** Design glowing heat layer rendering
- [ ] **3.2.2** Implement drill-down interactions
- [ ] **3.2.3** Add animated transitions
- [ ] **3.2.4** Create responsive layout

### 3.3 Relationship Graph Visualization
- [ ] **3.3.1** Integrate Cytoscape.js or D3.js
- [ ] **3.3.2** Build graph node/edge generation
- [ ] **3.3.3** Implement interactive exploration
- [ ] **3.3.4** Add filtering & search

### 3.4 Timeline Reconstruction View
- [ ] **3.4.1** Design timeline UI component
- [ ] **3.4.2** Implement chronological event rendering
- [ ] **3.4.3** Add event detail popups
- [ ] **3.4.4** Create timeline filtering

### 3.5 Severity & Classification UI
- [ ] **3.5.1** Build severity indicator components
- [ ] **3.5.2** Create color-coded risk visualization
- [ ] **3.5.3** Implement sorting by severity
- [ ] **3.5.4** Add filtering by risk level

---

## PHASE 4: ADVANCED FEATURES

### 4.1 Offline AI Assistant (Optional)
- [ ] **4.1.1** Evaluate ONNX model options
- [ ] **4.1.2** Integrate llama.cpp or similar
- [ ] **4.1.3** Build summary generation
- [ ] **4.1.4** Create risk explanation generator

### 4.2 Privacy Cleanup Actions
- [ ] **4.2.1** Implement cookie clearing
- [ ] **4.2.2** Build metadata stripping
- [ ] **4.2.3** Create cache purging
- [ ] **4.2.4** Add secure file deletion
- [ ] **4.2.5** Build confirmation & transparency UI

### 4.3 Real-Time Monitoring Mode (Optional)
- [ ] **4.3.1** Implement file system watcher
- [ ] **4.3.2** Build credential detection alerts
- [ ] **4.3.3** Create tracker detection notifications
- [ ] **4.3.4** Add alert persistence

---

## PHASE 5: TESTING & OPTIMIZATION

- [ ] **5.1** Unit tests for backend modules
- [ ] **5.2** Integration tests for scanning pipeline
- [ ] **5.3** Performance profiling & optimization
- [ ] **5.4** UI/UX testing
- [ ] **5.5** Windows compatibility testing (10/11)

---

## PHASE 6: DOCUMENTATION & RELEASE

- [ ] **6.1** API documentation
- [ ] **6.2** User guide & feature explanations
- [ ] **6.3** Developer setup guide
- [ ] **6.4** Privacy policy (local-only assurance)
- [ ] **6.5** Build release artifacts

---

## PRIORITY MATRIX

**CRITICAL (Must Have):**
- Privacy Debt Score
- Filesystem Scanner
- Secret Detection
- Windows Artifacts
- Browser Analysis
- Core UI Dashboard

**HIGH (Should Have):**
- Metadata Leak Detection
- Privacy Heatmap Visualization
- Relationship Graph
- Privacy Cleanup Actions
- Timeline Reconstruction

**MEDIUM (Nice to Have):**
- Offline AI Assistant
- Real-Time Monitoring
- SQLite/App Database Analysis
- Advanced Filtering

---

## RESOURCE ALLOCATION

- **Backend Engineer:** Filesystem scanning, artifact analysis, secret detection, database
- **Frontend Engineer:** React components, visualizations, UI/UX
- **Full-Stack:** API contracts, Tauri integration, testing

---

## SUCCESS METRICS

✓ Full offline capability (zero external calls)
✓ Sub-5-second scan of typical Windows system
✓ Accurate secret detection (>95% precision)
✓ Visually compelling UI (demo-ready)
✓ Clear remediation guidance for each finding
✓ Privacy Debt Score correlates with actual exposure
