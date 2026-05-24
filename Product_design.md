# Project Prompt: Windows-First Local Privacy Debt Auditor

You are designing and engineering a next-generation cybersecurity and privacy analysis platform called:

# “Privacy Debt Auditor”

A high-performance, Windows-first, local-only forensic privacy auditor that identifies, maps, visualizes, and explains sensitive personal data exposure and tracking residue stored across a user’s machine.

The application must operate entirely offline.

No cloud processing.
No telemetry.
No analytics.
No external APIs.
No remote logging.
No account systems.

The audit itself must never become a privacy leak.

The project should feel like:

* a forensic observability platform
* a privacy archaeology engine
* an OSINT investigation against your own computer
* a behavioral residue mapper

The product philosophy should communicate:

> “Your machine remembers more about you than you realize.”

---

# PRIMARY GOAL

Build a visually sophisticated desktop application capable of:

* scanning Windows filesystems
* analyzing browser artifacts
* inspecting application databases
* detecting secrets and privacy leaks
* reconstructing behavioral timelines
* visualizing privacy exposure
* generating actionable remediation guidance

The project should be hackathon-grade:

* visually memorable
* emotionally impactful
* technically impressive
* demo-friendly
* realistically useful

---

# TARGET PLATFORM

Primary Platform:

* Windows 10/11

Development Environment:

* WSL acceptable
* Native Windows support required

Future Support:

* Linux
* macOS

---

# CORE PRODUCT PRINCIPLES

# 1. Local-First Architecture

All analysis must happen locally.

No internet communication allowed.

---

# 2. Transparency

Users must understand:

* what data exists
* where it exists
* why it exists
* which application created it
* why it matters

---

# 3. Human-Centered Cybersecurity

Translate invisible privacy exposure into understandable visual insights.

The experience should feel investigative, cinematic, and eye-opening.

---

# 4. Forensic Authenticity

Use real system artifacts and realistic privacy findings.

Avoid fake “Hollywood hacker” aesthetics.

---

# RECOMMENDED TECH STACK

## Backend

Rust

Reason:

* performance
* memory safety
* concurrency
* filesystem efficiency
* WinAPI compatibility

---

## Desktop Application

Tauri + React

---

## Visualization

D3.js or Cytoscape.js

---

## Database Layer

SQLite

---

## Async Runtime

Tokio

---

## OCR

Tesseract OCR

---

## Windows APIs

Rust windows-rs crate

Use for:

* registry access
* filesystem metadata
* Windows-specific artifacts

---

# CORE FEATURES

# 1. Privacy Debt Score™

Generate a global privacy exposure score.

Example:
Privacy Debt Score: 78/100

Factors include:

* exposed secrets
* tracking cookies
* cached identifiers
* browser persistence
* metadata leakage
* retained deleted artifacts
* stale sessions
* risky storage locations

The score should update dynamically during scans.

---

# 2. Filesystem Scanner

Recursive filesystem analysis engine.

Scan:

* Downloads
* Desktop
* Documents
* browser storage
* temp directories
* app data folders

Detect:

* API keys
* passwords
* .env files
* private keys
* wallet seed phrases
* sensitive PDFs
* identity documents
* screenshots containing text

Capabilities:

* concurrent scanning
* path exclusions
* entropy analysis
* file type detection
* MIME analysis

---

# 3. Browser Privacy Analysis

Support:

* Chrome
* Edge
* Firefox later

Analyze:

* cookies
* browser history
* autofill traces
* localStorage
* IndexedDB
* trackers
* ad identifiers
* session remnants

Visualize:

* tracking companies
* tracker counts
* persistence levels
* tracking severity

Example:
Google → 42 trackers
Meta → 31 trackers

---

# 4. Windows Artifact Analysis

Critical feature category.

Analyze:

* Registry
* Jump Lists
* Recent Files
* Thumbnail Cache
* ShellBags
* temp files
* OneDrive remnants

Use findings to reconstruct behavioral traces and privacy exposure.

---

# 5. Metadata Leak Detection

Scan:

* images
* PDFs
* Office documents
* videos

Detect:

* GPS coordinates
* device models
* author names
* software metadata
* timestamps
* usernames

Provide:

* explanations
* metadata stripping options
* severity scoring

---

# 6. Secret Detection Engine

Detect:

* JWTs
* AWS keys
* OAuth tokens
* SSH keys
* bearer tokens
* mnemonic phrases
* crypto wallet traces

Methods:

* regex
* entropy analysis
* heuristics

Each finding must include:

* severity
* explanation
* source path
* remediation guidance

---

# 7. Interactive Privacy Heatmap

Visualize filesystem risk intensity.

Example:
/Downloads → Critical
/Desktop → High
/Documents → Medium

Use:

* glowing heat layers
* drill-down interactions
* animated transitions

The visualization should feel cinematic and investigative.

---

# 8. Relationship Graph Visualization

Interactive graph mapping relationships between:

* applications
* trackers
* sessions
* cached identities
* stored artifacts

Example:
Chrome
├── Cookies
├── Sessions
├── Trackers
└── Autofill identities

The graph should feel like a digital forensic investigation board.

---

# 9. Timeline Reconstruction

Reconstruct behavioral activity from system artifacts.

Example:
8:13 PM → Banking PDF opened
8:14 PM → Screenshot created
8:16 PM → Discord upload

Sources:

* filesystem timestamps
* browser history
* Jump Lists
* thumbnail cache
* app databases

The goal is to reveal behavioral residue.

---

# 10. Cache Archaeology

Identify remnants of deleted or forgotten artifacts.

Examples:

* deleted image previews
* stale thumbnails
* cached media remnants
* abandoned temp files

Focus on evidence visibility rather than full recovery.

---

# 11. Severity Classification System

Severity levels:

* Critical
* High
* Medium
* Low

Examples:
Critical:

* wallet seed phrase
* private SSH key

High:

* session token
* OAuth credential

Medium:

* GPS metadata

Low:

* stale cache

---

# 12. Offline AI Assistant (Optional)

Completely local AI assistant.

Functions:

* summarize findings
* explain risks
* generate remediation recommendations

Requirements:

* no cloud inference
* lightweight local models
* ONNX or llama.cpp support

---

# 13. Privacy Cleanup Actions

Allow users to:

* clear tracking cookies
* remove metadata
* purge caches
* securely delete files

Requirements:

* confirmation prompts
* operation transparency
* safety warnings

Never silently delete user data.

---

# 14. Real-Time Monitoring Mode (Optional)

Monitor:

* new trackers
* sensitive file creation
* leaked credentials
* risky downloads

Example:
“New OAuth token detected in Downloads.”

---

# 15. SQLite & Application Database Scanner

Analyze:

* browser SQLite databases
* Electron application storage
* IndexedDB
* LevelDB

Applications:

* Discord
* Telegram
* Slack
* VS Code
* Spotify
* Steam

Detect:

* trackers
* cached identifiers
* deleted remnants
* stored sessions

---

# 16. Recommendation Engine

Generate actionable recommendations.

Examples:
“Move SSH keys out of Downloads.”
“Clear stale browser sessions.”
“Strip GPS metadata from exported images.”

Recommendations should be:

* prioritized
* understandable
* security-aware

---

# 17. Exportable Audit Reports

Export:

* PDF
* JSON
* Markdown

Include:

* findings
* severity
* screenshots
* timelines
* remediation guidance

Reports should look forensic-grade and professional.

---

# 18. Offline Integrity Banner

Prominently display:
✓ No internet access
✓ No telemetry
✓ Fully local processing

This should establish trust immediately.

---

# USER EXPERIENCE DESIGN

# Design Language

The UI should feel:

* elegant
* cinematic
* investigative
* modern
* cyber-forensic

Avoid:

* cliché green hacker terminals
* excessive neon
* gimmicky effects

Preferred visual style:

* matte black
* charcoal gray
* amber highlights
* subtle warning reds
* soft animations
* glassmorphism accents

Inspirations:

* modern observability dashboards
* forensic analysis platforms
* high-end DFIR tooling

---

# MAIN INTERFACE LAYOUT

# Left Sidebar

Navigation:

* Dashboard
* Browser Artifacts
* Windows Artifacts
* Applications
* Timeline
* Heatmap
* Reports
* Recommendations

---

# Main Dashboard

Display:

* Privacy Debt Score
* live findings
* scan progress
* tracker summaries
* heatmaps
* relationship graphs

---

# Right Context Panel

Show:

* artifact details
* metadata
* severity
* remediation guidance
* related applications

---

# LIVE SCANNING EXPERIENCE

The scan should feel dynamic and alive.

Examples:
Scanning browser artifacts...
Inspecting thumbnail cache...
Analyzing registry traces...
Reconstructing behavioral timeline...

Live findings should appear progressively.

---

# DEMO EXPERIENCE

The demo should unfold like a forensic investigation.

# Demo Flow

1. Launch application
2. Display offline integrity banner
3. Start scan
4. Live findings appear
5. Reveal trackers and sessions
6. Open privacy heatmap
7. Show timeline reconstruction
8. Reveal deleted artifact remnants
9. End with:

“This audit never left your machine.”

The emotional impact should be:
“My computer remembers everything.”

---

# PERFORMANCE REQUIREMENTS

The system should:

* remain responsive during scans
* support concurrent scanning
* process large filesystems efficiently
* minimize RAM usage
* support incremental indexing

Use:

* async Rust
* worker pools
* modular parsers

---

# SECURITY REQUIREMENTS

The application itself must:

* never transmit user data
* avoid unnecessary privilege escalation
* securely store reports locally
* isolate risky parsers where possible
* clearly communicate all destructive actions

---

# RECOMMENDED PROJECT STRUCTURE

privacy-debt-auditor/
├── core/
│   ├── scanner/
│   ├── detectors/
│   ├── parsers/
│   ├── entropy/
│   ├── metadata/
│   └── risk_engine/
│
├── windows/
│   ├── registry/
│   ├── jump_lists/
│   ├── shellbags/
│   ├── browser/
│   ├── thumbnails/
│   └── recent_files/
│
├── frontend/
│   ├── dashboard/
│   ├── timeline/
│   ├── graph/
│   ├── heatmap/
│   └── reports/
│
└── exports/

---

# DELIVERABLES

Generate:

1. Full system architecture
2. Rust backend design
3. Tauri frontend design
4. Module breakdown
5. Windows artifact parsing plan
6. Data flow diagrams
7. Database schema
8. UI wireframes
9. Feature implementation roadmap
10. Risk scoring model
11. Visualization concepts
12. Demo script
13. Branding direction
14. MVP vs advanced roadmap
15. Threat model

The final result should feel like a believable next-generation forensic privacy platform suitable for a cybersecurity hackathon.
