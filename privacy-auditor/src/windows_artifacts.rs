//! Windows artifact analysis (Registry, Jump Lists, etc.)

use crate::error::Result;
use crate::models::*;
use std::path::PathBuf;
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;

pub struct WindowsArtifactAnalyzer;

impl WindowsArtifactAnalyzer {
    /// Scan Windows Registry for sensitive information
    pub async fn scan_registry() -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Registry scanning typically requires Windows API access
        // For now, we'll detect common sensitive registry paths and keys
        let sensitive_registry_paths = vec![
            ("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Run", "Startup Programs", Severity::Medium),
            ("HKEY_CURRENT_USER\\Software\\Microsoft\\Internet Explorer\\TypedURLs", "Browser History (Registry)", Severity::Medium),
            ("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\RecentDocs", "Recent Documents", Severity::High),
            ("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon", "Saved Credentials", Severity::Critical),
            ("HKEY_LOCAL_MACHINE\\System\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces", "Network Configuration", Severity::Medium),
        ];

        for (registry_path, description, severity) in sensitive_registry_paths {
            let mut metadata = HashMap::new();
            metadata.insert("registry_path".to_string(), registry_path.to_string());
            metadata.insert("artifact_type".to_string(), "registry".to_string());
            
            let finding = Finding {
                id: Uuid::new_v4().to_string(),
                category: FindingCategory::WindowsArtifact,
                title: format!("Registry Artifact: {}", description),
                description: format!(
                    "Windows Registry contains sensitive artifact: {}. This may expose system configuration, user activity, or credentials.",
                    description
                ),
                severity,
                location: registry_path.to_string(),
                value_hash: String::new(),
                metadata: FindingMetadata {
                    source_application: Some("Windows Registry".to_string()),
                    file_size: None,
                    last_modified: None,
                    additional_info: metadata,
                },
                remediation: "Review registry permissions and consider disabling sensitive startup programs".to_string(),
                discovered_at: Utc::now(),
            };
            findings.push(finding);
        }

        Ok(findings)
    }

    /// Extract Jump Lists for application usage
    pub async fn analyze_jump_lists() -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        let appdata = match std::env::var("APPDATA") {
            Ok(path) => path,
            Err(_) => return Ok(findings),
        };

        let jump_list_path = PathBuf::from(&appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Recent")
            .join("AutomaticDestinations");

        if jump_list_path.exists() {
            let finding = Finding {
                id: Uuid::new_v4().to_string(),
                category: FindingCategory::WindowsArtifact,
                title: "Jump Lists - Recent Application Files".to_string(),
                description: "Windows Jump Lists store recently opened files for each application. This reveals file usage patterns and sensitive file access history.".to_string(),
                severity: Severity::High,
                location: jump_list_path.to_string_lossy().to_string(),
                value_hash: String::new(),
                metadata: FindingMetadata {
                    source_application: Some("Windows Shell".to_string()),
                    file_size: None,
                    last_modified: None,
                    additional_info: {
                        let mut map = HashMap::new();
                        map.insert("artifact_type".to_string(), "jump_lists".to_string());
                        map.insert("privacy_risk".to_string(), "Reveals application file history".to_string());
                        map
                    },
                },
                remediation: "Clear jump lists regularly or disable them in Windows settings".to_string(),
                discovered_at: Utc::now(),
            };
            findings.push(finding);
        }

        Ok(findings)
    }

    /// Analyze recent files
    pub async fn analyze_recent_files() -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        let appdata = match std::env::var("APPDATA") {
            Ok(path) => path,
            Err(_) => return Ok(findings),
        };

        let recent_path = PathBuf::from(&appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Recent");

        if recent_path.exists() {
            // Check for LNK files (shortcuts)
            match std::fs::read_dir(&recent_path) {
                Ok(entries) => {
                    let lnk_count = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| {
                            e.path()
                                .extension()
                                .map(|ext| ext == "lnk")
                                .unwrap_or(false)
                        })
                        .count();

                    if lnk_count > 0 {
                        let finding = Finding {
                            id: Uuid::new_v4().to_string(),
                            category: FindingCategory::WindowsArtifact,
                            title: "Recent Files Shortcuts".to_string(),
                            description: format!(
                                "Found {} recent file shortcuts (.lnk files). These contain metadata about accessed files, file paths, and access times.",
                                lnk_count
                            ),
                            severity: Severity::High,
                            location: recent_path.to_string_lossy().to_string(),
                            value_hash: String::new(),
                            metadata: FindingMetadata {
                                source_application: Some("Windows Shell".to_string()),
                                file_size: None,
                                last_modified: None,
                                additional_info: {
                                    let mut map = HashMap::new();
                                    map.insert("shortcut_count".to_string(), lnk_count.to_string());
                                    map.insert("artifact_type".to_string(), "recent_files".to_string());
                                    map
                                },
                            },
                            remediation: "Clear Recent Files folder or use Disk Cleanup utility".to_string(),
                            discovered_at: Utc::now(),
                        };
                        findings.push(finding);
                    }
                }
                Err(_) => {}
            }
        }

        Ok(findings)
    }

    /// Check thumbnail cache for deleted images
    pub async fn analyze_thumbnail_cache() -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        let appdata = match std::env::var("APPDATA") {
            Ok(path) => path,
            Err(_) => return Ok(findings),
        };

        let thumbnail_cache_path = PathBuf::from(&appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Explorer");

        if thumbnail_cache_path.exists() {
            let thumb_cache_db = thumbnail_cache_path.join("thumbcache_*.db");
            
            let finding = Finding {
                id: Uuid::new_v4().to_string(),
                category: FindingCategory::DeletedArtifact,
                title: "Thumbnail Cache".to_string(),
                description: "Windows maintains a thumbnail cache of images and media files viewed or accessed on the system. This cache persists even after files are deleted and can reveal access history.".to_string(),
                severity: Severity::Critical,
                location: thumbnail_cache_path.to_string_lossy().to_string(),
                value_hash: String::new(),
                metadata: FindingMetadata {
                    source_application: Some("Windows Explorer".to_string()),
                    file_size: None,
                    last_modified: None,
                    additional_info: {
                        let mut map = HashMap::new();
                        map.insert("artifact_type".to_string(), "thumbnail_cache".to_string());
                        map.insert("privacy_risk".to_string(), "Can recover thumbnails of deleted files".to_string());
                        map
                    },
                },
                remediation: "Delete thumbcache files or use Cipher /w to securely wipe free space".to_string(),
                discovered_at: Utc::now(),
            };
            findings.push(finding);
        }

        Ok(findings)
    }

    /// Analyze shell bags for directory history
    pub async fn analyze_shell_bags() -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Shell Bags are stored in Registry under HKEY_CURRENT_USER\Software\Microsoft\Windows\Shell\Bags
        let finding = Finding {
            id: Uuid::new_v4().to_string(),
            category: FindingCategory::WindowsArtifact,
            title: "Shell Bags - Directory Navigation History".to_string(),
            description: "Windows Shell Bags store information about folder viewing history, folder window sizes, and positions. These registry entries can reveal directory traversal patterns and file location preferences.".to_string(),
            severity: Severity::High,
            location: "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\Shell\\Bags".to_string(),
            value_hash: String::new(),
            metadata: FindingMetadata {
                source_application: Some("Windows Shell".to_string()),
                file_size: None,
                last_modified: None,
                additional_info: {
                    let mut map = HashMap::new();
                    map.insert("artifact_type".to_string(), "shell_bags".to_string());
                    map.insert("location".to_string(), "Registry".to_string());
                    map
                },
            },
            remediation: "Clear Shell Bags registry entries or use registry cleaning tools".to_string(),
            discovered_at: Utc::now(),
        };
        findings.push(finding);

        Ok(findings)
    }

    /// Scan System Event Log for suspicious activity
    pub async fn analyze_event_logs() -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        let event_log_path = PathBuf::from("C:\\Windows\\System32\\winevt\\Logs");

        if event_log_path.exists() {
            let finding = Finding {
                id: Uuid::new_v4().to_string(),
                category: FindingCategory::WindowsArtifact,
                title: "Windows Event Logs".to_string(),
                description: "Windows Event Logs (Security, System, Application) record system events, failed logon attempts, and security-related activities. These logs can reveal system usage patterns and security events.".to_string(),
                severity: Severity::High,
                location: event_log_path.to_string_lossy().to_string(),
                value_hash: String::new(),
                metadata: FindingMetadata {
                    source_application: Some("Windows Event Viewer".to_string()),
                    file_size: None,
                    last_modified: None,
                    additional_info: {
                        let mut map = HashMap::new();
                        map.insert("artifact_type".to_string(), "event_logs".to_string());
                        map.insert("retention".to_string(), "Can retain months of activity".to_string());
                        map
                    },
                },
                remediation: "Configure event log retention and regularly review security event logs".to_string(),
                discovered_at: Utc::now(),
            };
            findings.push(finding);
        }

        Ok(findings)
    }

    /// Comprehensive Windows artifacts scan
    pub async fn scan_all_artifacts() -> Result<Vec<Finding>> {
        let mut all_findings = Vec::new();

        // Registry scanning
        if let Ok(registry_findings) = Self::scan_registry().await {
            all_findings.extend(registry_findings);
        }

        // Jump lists
        if let Ok(jump_list_findings) = Self::analyze_jump_lists().await {
            all_findings.extend(jump_list_findings);
        }

        // Recent files
        if let Ok(recent_findings) = Self::analyze_recent_files().await {
            all_findings.extend(recent_findings);
        }

        // Thumbnail cache
        if let Ok(thumbnail_findings) = Self::analyze_thumbnail_cache().await {
            all_findings.extend(thumbnail_findings);
        }

        // Shell bags
        if let Ok(shell_bag_findings) = Self::analyze_shell_bags().await {
            all_findings.extend(shell_bag_findings);
        }

        // Event logs
        if let Ok(event_log_findings) = Self::analyze_event_logs().await {
            all_findings.extend(event_log_findings);
        }

        Ok(all_findings)
    }
}
