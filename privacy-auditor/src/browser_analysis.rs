//! Browser privacy analysis (Chrome, Firefox, Edge)

use crate::error::Result;
use crate::models::*;
use rusqlite::Connection;
use std::path::PathBuf;
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;

pub struct BrowserAnalyzer;

impl BrowserAnalyzer {
    /// Analyze Chrome/Chromium browser data
    pub async fn analyze_chrome() -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        let appdata = match std::env::var("LOCALAPPDATA") {
            Ok(path) => path,
            Err(_) => return Ok(findings), // APPDATA not set, skip analysis
        };
        
        let chrome_path = PathBuf::from(&appdata)
            .join("Google")
            .join("Chrome")
            .join("User Data")
            .join("Default");

        if chrome_path.exists() {
            // Analyze cookies
            let cookie_findings = Self::parse_cookies(&chrome_path, "Chrome").await?;
            findings.extend(cookie_findings);

            // Analyze browser history
            let history_findings = Self::analyze_history(&chrome_path, "Chrome").await?;
            findings.extend(history_findings);

            // Check for autofill data
            let autofill_findings = Self::check_autofill_data(&chrome_path, "Chrome").await?;
            findings.extend(autofill_findings);
        }

        Ok(findings)
    }

    /// Analyze Edge browser data
    pub async fn analyze_edge() -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        let appdata = match std::env::var("LOCALAPPDATA") {
            Ok(path) => path,
            Err(_) => return Ok(findings),
        };
        
        let edge_path = PathBuf::from(&appdata)
            .join("Microsoft")
            .join("Edge")
            .join("User Data")
            .join("Default");

        if edge_path.exists() {
            let cookie_findings = Self::parse_cookies(&edge_path, "Edge").await?;
            findings.extend(cookie_findings);

            let history_findings = Self::analyze_history(&edge_path, "Edge").await?;
            findings.extend(history_findings);
        }

        Ok(findings)
    }

    /// Analyze Firefox browser data
    pub async fn analyze_firefox() -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        let appdata = match std::env::var("APPDATA") {
            Ok(path) => path,
            Err(_) => return Ok(findings),
        };
        
        let firefox_path = PathBuf::from(&appdata)
            .join("Mozilla")
            .join("Firefox")
            .join("Profiles");

        if firefox_path.exists() {
            // Firefox stores cookies in a SQLite database, similar analysis
            if let Ok(entries) = std::fs::read_dir(&firefox_path) {
                for entry in entries.flatten() {
                    let profile_path = entry.path();
                    let cookie_findings = Self::parse_cookies(&profile_path, "Firefox").await?;
                    findings.extend(cookie_findings);

                    let history_findings = Self::analyze_history(&profile_path, "Firefox").await?;
                    findings.extend(history_findings);
                }
            }
        }

        Ok(findings)
    }

    /// Parse cookies from browser database
    pub async fn parse_cookies(browser_path: &PathBuf, browser_name: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let cookies_db = browser_path.join("Cookies");

        if !cookies_db.exists() {
            return Ok(findings);
        }

        let hosts = Self::query_string_column(&cookies_db, "SELECT DISTINCT host_key FROM cookies")?;

        // Check actual cookie hosts for known tracking domains.
        for tracker in Self::get_tracker_domains().into_iter().filter(|tracker| {
            hosts.iter().any(|host| {
                let host = host.trim_start_matches('.').to_ascii_lowercase();
                host == *tracker || host.ends_with(&format!(".{}", tracker))
            })
        }) {
            let mut metadata = HashMap::new();
            metadata.insert("tracker_domain".to_string(), tracker.to_string());
            
            let tracking_finding = Finding {
                id: Uuid::new_v4().to_string(),
                category: FindingCategory::TrackingCookie,
                title: format!("Tracking Cookie: {}", tracker),
                description: format!(
                    "Browser has cookies from tracking domain '{}'. This domain may be collecting browsing behavior data.",
                    tracker
                ),
                severity: Severity::Medium,
                location: cookies_db.to_string_lossy().to_string(),
                value_hash: String::new(),
                metadata: FindingMetadata {
                    source_application: Some(browser_name.to_string()),
                    file_size: None,
                    last_modified: None,
                    additional_info: metadata,
                },
                remediation: format!("Clear browser cookies or configure privacy settings to block tracking cookies from {}", tracker),
                discovered_at: Utc::now(),
            };
            findings.push(tracking_finding);
        }

        Ok(findings)
    }

    /// Analyze browser history for sensitive patterns
    pub async fn analyze_history(browser_path: &PathBuf, browser_name: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let history_db = browser_path.join("History");

        if !history_db.exists() {
            return Ok(findings);
        }

        let sensitive_patterns = vec![
            ("banking", "Bank/Financial websites", Severity::Critical),
            ("health", "Health/Medical websites", Severity::High),
            ("adult", "Adult content", Severity::High),
            ("political", "Political content", Severity::Medium),
        ];

        let urls = Self::query_string_column(&history_db, "SELECT url FROM urls")?;

        for (pattern_type, label, severity) in sensitive_patterns {
            let matching_count = urls
                .iter()
                .filter(|url| url.to_ascii_lowercase().contains(pattern_type))
                .count();

            if matching_count == 0 {
                continue;
            }

            let mut metadata = HashMap::new();
            metadata.insert("pattern_type".to_string(), pattern_type.to_string());
            metadata.insert("matching_urls".to_string(), matching_count.to_string());
            
            let finding = Finding {
                id: Uuid::new_v4().to_string(),
                category: FindingCategory::BrowserPrivacy,
                title: format!("Browser History - {}", label),
                description: format!(
                    "Browser history contains {} entries matching {} patterns. This could expose personal information if accessed by malware or unauthorized users.",
                    matching_count,
                    label.to_lowercase()
                ),
                severity,
                location: history_db.to_string_lossy().to_string(),
                value_hash: String::new(),
                metadata: FindingMetadata {
                    source_application: Some(browser_name.to_string()),
                    file_size: None,
                    last_modified: None,
                    additional_info: metadata,
                },
                remediation: "Regularly clear browser history or use private browsing mode".to_string(),
                discovered_at: Utc::now(),
            };
            findings.push(finding);
        }

        Ok(findings)
    }

    /// Check for autofill data containing sensitive information
    pub async fn check_autofill_data(browser_path: &PathBuf, browser_name: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let web_data = browser_path.join("Web Data");

        if !web_data.exists() {
            return Ok(findings);
        }

        let finding = Finding {
            id: Uuid::new_v4().to_string(),
            category: FindingCategory::BrowserPrivacy,
            title: "Browser Autofill Data".to_string(),
            description: "Browser stores autofill data (names, addresses, phone numbers, email). If accessible to malware, this could leak personal information.".to_string(),
            severity: Severity::High,
            location: web_data.to_string_lossy().to_string(),
            value_hash: String::new(),
                metadata: FindingMetadata {
                source_application: Some(browser_name.to_string()),
                file_size: None,
                last_modified: None,
                additional_info: HashMap::new(),
            },
            remediation: "Disable autofill feature or manage autofill data in browser settings".to_string(),
            discovered_at: Utc::now(),
        };
        findings.push(finding);

        Ok(findings)
    }

    /// Scan all major browsers
    pub async fn scan_all_browsers() -> Result<Vec<Finding>> {
        let mut all_findings = Vec::new();

        // Chrome
        if let Ok(chrome_findings) = Self::analyze_chrome().await {
            all_findings.extend(chrome_findings);
        }

        // Edge
        if let Ok(edge_findings) = Self::analyze_edge().await {
            all_findings.extend(edge_findings);
        }

        // Firefox
        if let Ok(firefox_findings) = Self::analyze_firefox().await {
            all_findings.extend(firefox_findings);
        }

        Ok(all_findings)
    }

    /// Get list of tracking domains
    fn get_tracker_domains() -> Vec<&'static str> {
        vec![
            "google.com",
            "facebook.com",
            "amazon.com",
            "doubleclick.net",
            "googleadservices.com",
            "googlesyndication.com",
            "facebook.net",
            "segment.com",
            "analytics.google.com",
            "cdn.segment.com",
            "twitter.com",
            "scorecardresearch.com",
            "quantserve.com",
            "taboola.com",
            "outbrain.com",
            "teads.tv",
            "exponential.com",
            "gumgum.com",
            "rubicon.com",
            "criteo.com",
        ]
    }

    fn query_string_column(db_path: &PathBuf, query: &str) -> Result<Vec<String>> {
        let temp_path = std::env::temp_dir().join(format!(
            "privacy_auditor_{}_{}",
            Uuid::new_v4(),
            db_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("browser.db")
        ));

        let query_path = if std::fs::copy(db_path, &temp_path).is_ok() {
            temp_path.as_path()
        } else {
            db_path.as_path()
        };

        let result = (|| {
            let conn = Connection::open(query_path)?;
            let mut stmt = conn.prepare(query)?;
            let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
        })();

        let _ = std::fs::remove_file(&temp_path);

        result.map_err(|e| crate::error::AuditError::Database(e.to_string()))
    }
}
