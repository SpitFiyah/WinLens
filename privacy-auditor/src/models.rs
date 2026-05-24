//! Data models for Privacy Debt Auditor

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Severity level for a privacy finding
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Low => write!(f, "Low"),
            Severity::Medium => write!(f, "Medium"),
            Severity::High => write!(f, "High"),
            Severity::Critical => write!(f, "Critical"),
        }
    }
}

/// Finding category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FindingCategory {
    Secret,
    BrowserPrivacy,
    Metadata,
    CachedIdentifier,
    WindowsArtifact,
    DeletedArtifact,
    SessionToken,
    TrackingCookie,
    Other(String),
}

impl std::fmt::Display for FindingCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindingCategory::Secret => write!(f, "Secret"),
            FindingCategory::BrowserPrivacy => write!(f, "Browser Privacy"),
            FindingCategory::Metadata => write!(f, "Metadata"),
            FindingCategory::CachedIdentifier => write!(f, "Cached Identifier"),
            FindingCategory::WindowsArtifact => write!(f, "Windows Artifact"),
            FindingCategory::DeletedArtifact => write!(f, "Deleted Artifact"),
            FindingCategory::SessionToken => write!(f, "Session Token"),
            FindingCategory::TrackingCookie => write!(f, "Tracking Cookie"),
            FindingCategory::Other(s) => write!(f, "{}", s),
        }
    }
}

/// A single privacy finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub category: FindingCategory,
    pub severity: Severity,
    pub title: String,
    pub description: String,
    pub location: String,  // File path or registry key
    pub value_hash: String, // Hash of the actual sensitive value
    pub metadata: FindingMetadata,
    pub remediation: String,
    pub discovered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingMetadata {
    pub source_application: Option<String>,
    pub file_size: Option<u64>,
    pub last_modified: Option<DateTime<Utc>>,
    pub additional_info: std::collections::HashMap<String, String>,
}

/// Privacy Debt Score calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyDebtScore {
    pub total_score: u32,  // 0-100
    pub score_timestamp: DateTime<Utc>,
    pub factors: ScoreFactors,
    pub findings_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Scan configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub scan_downloads: bool,
    pub scan_desktop: bool,
    pub scan_documents: bool,
    pub scan_appdata: bool,
    pub scan_browser: bool,
    pub scan_registry: bool,
    pub exclude_patterns: Vec<String>,
    pub max_file_size: Option<u64>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            scan_downloads: true,
            scan_desktop: true,
            scan_documents: true,
            scan_appdata: true,
            scan_browser: true,
            scan_registry: true,
            exclude_patterns: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "target".to_string(),
            ],
            max_file_size: Some(100 * 1024 * 1024), // 100MB
        }
    }
}

/// Scan progress event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub status: ScanStatus,
    pub current_path: String,
    pub files_scanned: usize,
    pub files_total: Option<usize>,
    pub findings_count: usize,
    pub elapsed_seconds: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanStatus {
    Starting,
    Scanning,
    Analyzing,
    Calculating,
    Complete,
    Error,
}

/// Audit report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub system_info: SystemInfo,
    pub findings: Vec<Finding>,
    pub privacy_score: PrivacyDebtScore,
    pub scan_duration_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_version: String,
    pub username: String,
    pub computer_name: String,
    pub scan_paths: Vec<String>,
}

impl Finding {
    pub fn new(
        category: FindingCategory,
        severity: Severity,
        title: String,
        description: String,
        location: String,
        value_hash: String,
        remediation: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            category,
            severity,
            title,
            description,
            location,
            value_hash,
            metadata: FindingMetadata {
                source_application: None,
                file_size: None,
                last_modified: None,
                additional_info: Default::default(),
            },
            remediation,
            discovered_at: Utc::now(),
        }
    }
}
