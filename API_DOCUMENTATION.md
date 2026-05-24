# API Documentation - WinLens Privacy Auditor

**Version**: 0.1.0  
**Last Updated**: 2024-12-14

## Table of Contents

1. [Overview](#overview)
2. [Backend API](#backend-api)
3. [Data Models](#data-models)
4. [Privacy Score Calculation](#privacy-score-calculation)
5. [Error Handling](#error-handling)
6. [Examples](#examples)

---

## Overview

The WinLens Privacy Auditor is a Rust-based backend library with a React TypeScript frontend. It scans Windows systems for privacy leaks across:

- **Secrets Detection**: API keys, passwords, tokens in config files
- **Browser Privacy**: Tracking cookies, saved credentials, history
- **Metadata Leaks**: EXIF data in images, PDF metadata, Office doc properties
- **Windows Artifacts**: Registry entries, recent files, thumbnail cache, event logs
- **Cached Data**: Temporary files, browser cache, deleted artifacts

---

## Backend API

### Core Modules

#### 1. **SecretDetector**

Detects sensitive credentials in files using pattern matching.

```rust
pub struct SecretDetector {
    patterns: Vec<SecretPattern>,
}

impl SecretDetector {
    /// Initialize detector with all secret patterns
    pub fn new() -> Result<Self>
    
    /// Detect secrets in file content
    pub fn detect_in_content(
        &self,
        content: &str,
        path: &Path,
    ) -> Result<Vec<Finding>>
    
    /// Detect secrets in file system
    pub fn detect_in_file(&self, path: &Path) -> Result<Vec<Finding>>
}
```

**Supported Secret Types**:
- AWS Access Keys (AKIA*)
- Private SSH Keys (RSA, EC, OpenSSH)
- GitHub Personal Access Tokens (ghp_*)
- Database Passwords (mysql://, postgres://)
- JWT Tokens (Bearer tokens)
- OAuth Tokens
- API Keys (generic patterns)

**Example**:
```rust
let detector = SecretDetector::new()?;
let findings = detector.detect_in_file(Path::new("config.env"))?;
for finding in findings {
    println!("{}: {}", finding.severity, finding.title);
}
```

#### 2. **BrowserAnalyzer**

Analyzes browser privacy data (Chrome, Firefox, Edge).

```rust
pub struct BrowserAnalyzer;

impl BrowserAnalyzer {
    /// Analyze Chrome/Chromium browser
    pub async fn analyze_chrome() -> Result<Vec<Finding>>
    
    /// Analyze Firefox browser
    pub async fn analyze_firefox() -> Result<Vec<Finding>>
    
    /// Analyze Microsoft Edge
    pub async fn analyze_edge() -> Result<Vec<Finding>>
    
    /// Scan all installed browsers
    pub async fn scan_all_browsers() -> Result<Vec<Finding>>
}
```

**Findings Detected**:
- Third-party tracking cookies
- Autofill/saved credentials
- Browser history with sensitive URLs
- Persistent login tokens
- Advertising identifiers

**Example**:
```rust
let findings = BrowserAnalyzer::scan_all_browsers().await?;
println!("Found {} browser privacy issues", findings.len());
```

#### 3. **MetadataAnalyzer**

Detects metadata leaks in media and documents.

```rust
pub struct MetadataAnalyzer;

impl MetadataAnalyzer {
    /// Scan images for EXIF metadata
    pub async fn scan_image_metadata(directory: &PathBuf) -> Result<Vec<Finding>>
    
    /// Scan PDFs for metadata
    pub async fn scan_pdf_metadata(directory: &PathBuf) -> Result<Vec<Finding>>
    
    /// Scan Office documents (DOCX, XLSX, PPTX)
    pub async fn scan_office_metadata(directory: &PathBuf) -> Result<Vec<Finding>>
    
    /// Scan all metadata types
    pub async fn scan_all_metadata() -> Result<Vec<Finding>>
}
```

**Metadata Detected**:
- GPS coordinates in photos
- Camera/device information
- Author/creator names
- Document modification dates
- Office document properties

**Example**:
```rust
let findings = MetadataAnalyzer::scan_image_metadata(&Path::new("Pictures").to_path_buf()).await?;
```

#### 4. **WindowsArtifactScanner**

Scans Windows-specific privacy artifacts.

```rust
pub struct WindowsArtifactScanner;

impl WindowsArtifactScanner {
    /// Scan Windows Registry for sensitive keys
    pub async fn scan_registry() -> Result<Vec<Finding>>
    
    /// Scan recently accessed files
    pub async fn scan_recent_files() -> Result<Vec<Finding>>
    
    /// Analyze Windows Event Logs
    pub async fn scan_event_logs() -> Result<Vec<Finding>>
    
    /// Check thumbnail cache for deleted file info
    pub async fn scan_thumbnail_cache() -> Result<Vec<Finding>>
    
    /// Scan all Windows artifacts
    pub async fn scan_all_artifacts() -> Result<Vec<Finding>>
}
```

**Artifacts Detected**:
- MRU (Most Recently Used) entries
- Shell Bags (folder view settings)
- Prefetch files
- Recycle Bin entries
- Jump Lists (Windows 7+)
- Event Log entries

#### 5. **ScoreCalculator**

Calculates Privacy Debt Score from findings.

```rust
pub struct ScoreCalculator;

impl ScoreCalculator {
    /// Calculate privacy debt score (0-100)
    pub fn calculate(findings: &[Finding]) -> PrivacyDebtScore
}
```

**Scoring Factors**:
- Exposed secrets: 30 pts (Critical), 20 (High), 10 (Medium), 5 (Low)
- Tracking cookies: 8 pts (Critical), 5 (High), 3 (Medium), 1 (Low)
- Cached identifiers: 15 pts (Critical), 10 (High), 5 (Medium), 2 (Low)
- Metadata leaks: 5 pts per finding
- Deleted artifacts: 10 pts per finding
- Stale sessions: 8 pts per finding
- Risky storage: 12 pts per finding

**Formula**: `score = min(100, total_factor_points * 0.8 + sqrt(finding_count) * 0.2)`

---

## Data Models

### Finding

Represents a single privacy finding.

```rust
pub struct Finding {
    pub id: String,                           // UUID v4
    pub category: FindingCategory,            // Type of finding
    pub severity: Severity,                   // CRITICAL/HIGH/MEDIUM/LOW
    pub title: String,                        // Brief description
    pub description: String,                  // Detailed explanation
    pub location: String,                     // File path or registry key
    pub value_hash: String,                   // SHA256 hash of sensitive value
    pub metadata: FindingMetadata,            // Additional context
    pub remediation: String,                  // How to fix
    pub discovered_at: DateTime<Utc>,        // When found
}
```

### FindingCategory

```rust
pub enum FindingCategory {
    Secret,                 // Exposed credentials
    BrowserPrivacy,        // Browser-related
    Metadata,              // File metadata
    CachedIdentifier,      // Cached user IDs
    WindowsArtifact,       // Windows-specific
    DeletedArtifact,       // From deleted files
    SessionToken,          // Active sessions
    TrackingCookie,        // Tracking cookies
    Other(String),         // Custom category
}
```

### Severity

```rust
pub enum Severity {
    Low,       // Minor privacy concern
    Medium,    // Notable exposure
    High,      // Significant exposure
    Critical,  // Immediate remediation needed
}
```

### FindingMetadata

```rust
pub struct FindingMetadata {
    pub source_application: Option<String>,   // Source app (browser, etc.)
    pub file_size: Option<u64>,               // File size if applicable
    pub last_modified: Option<DateTime<Utc>>, // Last modified time
    pub additional_info: HashMap<String, String>, // Custom context
}
```

### PrivacyDebtScore

```rust
pub struct PrivacyDebtScore {
    pub total_score: u32,              // 0-100
    pub score_timestamp: DateTime<Utc>, // When calculated
    pub factors: ScoreFactors,         // Breakdown
    pub findings_count: usize,         // Total findings
}

pub struct ScoreFactors {
    pub exposed_secrets: u32,
    pub tracking_cookies: u32,
    pub cached_identifiers: u32,
    pub browser_persistence: u32,
    pub metadata_leakage: u32,
    pub deleted_artifacts: u32,
    pub stale_sessions: u32,
    pub risky_storage: u32,
}
```

---

## Privacy Score Calculation

### Algorithm

1. **Factor Calculation**: Each finding adds points based on category and severity
2. **Normalization**: Total points converted to 0-100 scale with diminishing returns
3. **Weighting**: 80% from factor points, 20% from finding count sqrt
4. **Capping**: Score capped at maximum 100

### Interpretation

- **90-100**: Minimal privacy concerns
- **70-89**: Moderate privacy issues, recommended cleanup
- **50-69**: Significant privacy exposure
- **30-49**: Severe privacy concerns
- **0-29**: Critical privacy situation

---

## Error Handling

### AuditError

```rust
pub enum AuditError {
    IoError(String),              // File system errors
    RegexError(String),           // Pattern matching errors
    EncodingError(String),        // UTF-8/encoding errors
    WindowsApiError(String),      // Windows API errors
    DatabaseError(String),        // SQLite errors
    SerializationError(String),   // JSON/TOML errors
    ConfigError(String),          // Configuration errors
}

impl std::error::Error for AuditError {}
```

### Result Type

```rust
pub type Result<T> = std::result::Result<T, AuditError>;
```

### Error Handling Examples

```rust
// Propagate errors
let findings = SecretDetector::new()?;

// Pattern matching
match detector.detect_in_file(path) {
    Ok(findings) => println!("Found {} findings", findings.len()),
    Err(AuditError::IoError(e)) => eprintln!("File error: {}", e),
    Err(e) => eprintln!("Error: {}", e),
}

// Graceful degradation
let findings = match detector.detect_in_file(path) {
    Ok(f) => f,
    Err(_) => Vec::new(), // Empty findings if error
};
```

---

## Examples

### Example 1: Complete System Scan

```rust
use privacy_auditor::{
    secret_detection::SecretDetector,
    browser_analysis::BrowserAnalyzer,
    metadata_analyzer::MetadataAnalyzer,
    windows_artifacts::WindowsArtifactScanner,
    privacy_score::ScoreCalculator,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut all_findings = Vec::new();

    // 1. Detect secrets in common locations
    let detector = SecretDetector::new()?;
    let secret_findings = detector.detect_in_file(Path::new("config.env"))?;
    all_findings.extend(secret_findings);

    // 2. Scan browsers
    let browser_findings = BrowserAnalyzer::scan_all_browsers().await?;
    all_findings.extend(browser_findings);

    // 3. Scan metadata
    let metadata_findings = MetadataAnalyzer::scan_all_metadata().await?;
    all_findings.extend(metadata_findings);

    // 4. Scan Windows artifacts
    let artifact_findings = WindowsArtifactScanner::scan_all_artifacts().await?;
    all_findings.extend(artifact_findings);

    // 5. Calculate score
    let score = ScoreCalculator::calculate(&all_findings);
    println!("Privacy Debt Score: {}/100", score.total_score);
    println!("Total Findings: {}", all_findings.len());

    Ok(())
}
```

### Example 2: Targeted Secret Detection

```rust
let detector = SecretDetector::new()?;

// Scan specific directories
for entry in WalkDir::new("src")
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
{
    if let Ok(findings) = detector.detect_in_file(entry.path()) {
        for finding in findings {
            println!("[{}] {}: {}", 
                finding.severity, 
                finding.title,
                finding.location
            );
        }
    }
}
```

### Example 3: Browser Privacy Analysis

```rust
let browser_findings = BrowserAnalyzer::scan_all_browsers().await?;

// Filter by severity
let critical_findings: Vec<_> = browser_findings
    .iter()
    .filter(|f| f.severity == Severity::Critical)
    .collect();

// Group by category
let mut by_category: HashMap<FindingCategory, Vec<_>> = HashMap::new();
for finding in browser_findings {
    by_category.entry(finding.category).or_insert_with(Vec::new).push(finding);
}

for (category, findings) in by_category {
    println!("{}: {} findings", category, findings.len());
}
```

---

## Frontend API

### React Components

#### Dashboard Component

```typescript
interface DashboardProps {
    score: number;
    findingsCount: number;
}

export function Dashboard({ score, findingsCount }: DashboardProps) {
    // Renders visualizations and findings
}
```

**Features**:
- Privacy Debt Score visualization (circular gauge)
- Risk factors breakdown
- Multiple view modes (Overview, Detailed, Trends)
- Interactive charts (Pie, Bar, Area, Line, Radar)
- Recent findings list with severity badges
- Top recommendations

#### Scanner Component

```typescript
interface ScannerProps {
    onScanStart?: () => void;
    onScanComplete?: (score: number, findings: number) => void;
}

export function Scanner(props: ScannerProps) {
    // Triggers system scan via Tauri
}
```

---

## CLI Usage

```bash
# Run full system scan
cargo run --release

# Run tests
cargo test

# Build documentation
cargo doc --open
```

---

## Performance Characteristics

- **Scan Time**: 30-120 seconds (first run), 5-10 seconds (cached)
- **Memory Usage**: ~50-100 MB per scan
- **CPU Usage**: Parallelized across available cores
- **Database**: SQLite, ~5-10 MB per month of scans

---

## Version History

- **v0.1.0** (2024-12-14): Initial MVP
  - Secret detection
  - Browser privacy analysis
  - Metadata leak detection
  - Windows artifact scanning
  - Privacy score calculation
  - React dashboard

---

## Support & Contributing

For issues, feature requests, or contributions, please refer to the project repository.

**Security Contact**: For security issues, please report responsibly to the project maintainers.
