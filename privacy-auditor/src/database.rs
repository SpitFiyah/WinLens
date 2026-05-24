//! Database module for Privacy Debt Auditor

use crate::error::Result;
use crate::models::*;
use rusqlite::{params, Connection};
use std::path::Path;
use tracing::{debug, info};

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new database connection
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path).map_err(|e| {
            crate::error::AuditError::Database(format!("Failed to open database: {}", e))
        })?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = 10000;",
        )
        .map_err(|e| crate::error::AuditError::Database(e.to_string()))?;

        let db = Self { conn };
        db.initialize_schema()?;
        Ok(db)
    }

    /// Initialize database schema
    fn initialize_schema(&self) -> Result<()> {
        self.conn
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS findings (
                    id TEXT PRIMARY KEY,
                    category TEXT NOT NULL,
                    severity TEXT NOT NULL,
                    title TEXT NOT NULL,
                    description TEXT NOT NULL,
                    location TEXT NOT NULL,
                    value_hash TEXT NOT NULL,
                    remediation TEXT NOT NULL,
                    source_application TEXT,
                    file_size INTEGER,
                    last_modified TEXT,
                    discovered_at TEXT NOT NULL,
                    additional_info TEXT
                );
                
                CREATE TABLE IF NOT EXISTS reports (
                    id TEXT PRIMARY KEY,
                    created_at TEXT NOT NULL,
                    os_version TEXT NOT NULL,
                    username TEXT NOT NULL,
                    computer_name TEXT NOT NULL,
                    scan_paths TEXT NOT NULL,
                    privacy_score INTEGER NOT NULL,
                    scan_duration REAL NOT NULL,
                    findings_count INTEGER NOT NULL
                );
                
                CREATE TABLE IF NOT EXISTS score_factors (
                    report_id TEXT PRIMARY KEY,
                    exposed_secrets INTEGER,
                    tracking_cookies INTEGER,
                    cached_identifiers INTEGER,
                    browser_persistence INTEGER,
                    metadata_leakage INTEGER,
                    deleted_artifacts INTEGER,
                    stale_sessions INTEGER,
                    risky_storage INTEGER,
                    FOREIGN KEY (report_id) REFERENCES reports(id)
                );
                
                CREATE INDEX IF NOT EXISTS idx_findings_severity ON findings(severity);
                CREATE INDEX IF NOT EXISTS idx_findings_category ON findings(category);
                CREATE INDEX IF NOT EXISTS idx_findings_location ON findings(location);
                CREATE INDEX IF NOT EXISTS idx_reports_created ON reports(created_at);",
            )
            .map_err(|e| {
                crate::error::AuditError::Database(format!("Failed to initialize schema: {}", e))
            })?;

        info!("Database schema initialized");
        Ok(())
    }

    /// Store a finding in the database
    pub fn insert_finding(&self, finding: &Finding) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO findings (id, category, severity, title, description, location, 
                value_hash, remediation, source_application, file_size, last_modified, 
                discovered_at, additional_info)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![
                    &finding.id,
                    finding.category.to_string(),
                    finding.severity.to_string(),
                    &finding.title,
                    &finding.description,
                    &finding.location,
                    &finding.value_hash,
                    &finding.remediation,
                    &finding.metadata.source_application,
                    finding.metadata.file_size,
                    finding
                        .metadata
                        .last_modified
                        .map(|dt| dt.to_rfc3339()),
                    finding.discovered_at.to_rfc3339(),
                    serde_json::to_string(&finding.metadata.additional_info)
                        .unwrap_or_default(),
                ],
            )
            .map_err(|e| {
                crate::error::AuditError::Database(format!("Failed to insert finding: {}", e))
            })?;

        debug!("Finding inserted: {}", finding.id);
        Ok(())
    }

    /// Get all findings
    pub fn get_findings(&self) -> Result<Vec<Finding>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM findings ORDER BY discovered_at DESC")
            .map_err(|e| crate::error::AuditError::Database(e.to_string()))?;

        let findings: Vec<Finding> = stmt
            .query_map([], |row| {
                let category: String = row.get(1)?;
                let severity: String = row.get(2)?;
                let additional_info: Option<String> = row.get(12)?;

                Ok(Finding {
                    id: row.get(0)?,
                    category: parse_category(&category),
                    severity: parse_severity(&severity),
                    title: row.get(3)?,
                    description: row.get(4)?,
                    location: row.get(5)?,
                    value_hash: row.get(6)?,
                    metadata: FindingMetadata {
                        source_application: row.get(8)?,
                        file_size: row.get(9)?,
                        last_modified: parse_datetime(row.get::<_, Option<String>>(10)?),
                        additional_info: additional_info
                            .and_then(|value| serde_json::from_str(&value).ok())
                            .unwrap_or_default(),
                    },
                    remediation: row.get(7)?,
                    discovered_at: parse_datetime(row.get::<_, Option<String>>(11)?)
                        .unwrap_or_else(chrono::Utc::now),
                })
            })
            .map_err(|e| crate::error::AuditError::Database(e.to_string()))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| crate::error::AuditError::Database(e.to_string()))?;

        Ok(findings)
    }

    /// Get findings by severity
    pub fn get_findings_by_severity(&self, severity: Severity) -> Result<Vec<Finding>> {
        let severity_str = severity.to_string();
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM findings WHERE severity = ?1")
            .map_err(|e| crate::error::AuditError::Database(e.to_string()))?;

        let findings: Vec<Finding> = stmt
            .query_map(params![severity_str], |row| {
                let category: String = row.get(1)?;
                let severity: String = row.get(2)?;
                let additional_info: Option<String> = row.get(12)?;

                Ok(Finding {
                    id: row.get(0)?,
                    category: parse_category(&category),
                    severity: parse_severity(&severity),
                    title: row.get(3)?,
                    description: row.get(4)?,
                    location: row.get(5)?,
                    value_hash: row.get(6)?,
                    metadata: FindingMetadata {
                        source_application: row.get(8)?,
                        file_size: row.get(9)?,
                        last_modified: parse_datetime(row.get::<_, Option<String>>(10)?),
                        additional_info: additional_info
                            .and_then(|value| serde_json::from_str(&value).ok())
                            .unwrap_or_default(),
                    },
                    remediation: row.get(7)?,
                    discovered_at: parse_datetime(row.get::<_, Option<String>>(11)?)
                        .unwrap_or_else(chrono::Utc::now),
                })
            })
            .map_err(|e| crate::error::AuditError::Database(e.to_string()))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| crate::error::AuditError::Database(e.to_string()))?;

        Ok(findings)
    }

    /// Clear all findings
    pub fn clear_findings(&self) -> Result<()> {
        self.conn
            .execute("DELETE FROM findings", [])
            .map_err(|e| {
                crate::error::AuditError::Database(format!("Failed to clear findings: {}", e))
            })?;

        info!("All findings cleared");
        Ok(())
    }
}

fn parse_severity(value: &str) -> Severity {
    match value.trim().to_ascii_lowercase().as_str() {
        "critical" => Severity::Critical,
        "high" => Severity::High,
        "medium" => Severity::Medium,
        "low" => Severity::Low,
        _ => Severity::Medium,
    }
}

fn parse_category(value: &str) -> FindingCategory {
    match value.trim().to_ascii_lowercase().replace('_', " ").as_str() {
        "secret" => FindingCategory::Secret,
        "browser privacy" => FindingCategory::BrowserPrivacy,
        "metadata" => FindingCategory::Metadata,
        "cached identifier" => FindingCategory::CachedIdentifier,
        "windows artifact" => FindingCategory::WindowsArtifact,
        "deleted artifact" => FindingCategory::DeletedArtifact,
        "session token" => FindingCategory::SessionToken,
        "tracking cookie" => FindingCategory::TrackingCookie,
        other => FindingCategory::Other(other.to_string()),
    }
}

fn parse_datetime(value: Option<String>) -> Option<chrono::DateTime<chrono::Utc>> {
    value
        .and_then(|value| chrono::DateTime::parse_from_rfc3339(&value).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
}
