//! Metadata leak detection - EXIF, PDF, document properties

use crate::error::Result;
use crate::models::*;
use std::path::PathBuf;
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use std::fs;

pub struct MetadataAnalyzer;

impl MetadataAnalyzer {
    /// Scan for image files with EXIF data
    pub async fn scan_image_metadata(directory: &PathBuf) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        if !directory.exists() {
            return Ok(findings);
        }

        let image_extensions = vec!["jpg", "jpeg", "png", "gif", "webp", "bmp"];

        // Walk through directory
        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if image_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            // Detect potential EXIF data
                            if let Ok(metadata) = fs::metadata(&path) {
                                let mut meta = HashMap::new();
                                meta.insert("file_size".to_string(), metadata.len().to_string());
                                
                                let last_modified = if let Ok(t) = metadata.modified() {
                                    if let Ok(duration) = t.duration_since(std::time::SystemTime::UNIX_EPOCH) {
                                        Some(chrono::DateTime::<Utc>::from(std::time::SystemTime::UNIX_EPOCH + duration))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                };
                                
                                let finding = Finding {
                                    id: Uuid::new_v4().to_string(),
                                    category: FindingCategory::Metadata,
                                    title: format!("Image File with Potential EXIF Data: {}", path.file_name().unwrap_or_default().to_string_lossy()),
                                    description: "Image files may contain EXIF metadata including GPS coordinates, camera model, creation date, and GPS location history. This can expose sensitive location information.".to_string(),
                                    severity: Severity::High,
                                    location: path.to_string_lossy().to_string(),
                                    value_hash: String::new(),
                                    metadata: FindingMetadata {
                                        source_application: Some("Image Analysis".to_string()),
                                        file_size: Some(metadata.len()),
                                        last_modified,
                                        additional_info: meta,
                                    },
                                    remediation: "Use image metadata removal tools or view images in viewer that strips EXIF data".to_string(),
                                    discovered_at: Utc::now(),
                                };
                                findings.push(finding);
                            }
                        }
                    }
                }
            }
        }

        Ok(findings)
    }

    /// Scan for PDF files with embedded metadata
    pub async fn scan_pdf_metadata(directory: &PathBuf) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        if !directory.exists() {
            return Ok(findings);
        }

        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if let Some(ext) = path.extension() {
                    if ext == "pdf" {
                        if let Ok(metadata) = fs::metadata(&path) {
                            let mut meta = HashMap::new();
                            meta.insert("file_type".to_string(), "PDF".to_string());
                            meta.insert("privacy_risk".to_string(), "May contain author, creator, timestamps, revision history".to_string());
                            
                            let last_modified = if let Ok(t) = metadata.modified() {
                                if let Ok(duration) = t.duration_since(std::time::SystemTime::UNIX_EPOCH) {
                                    Some(chrono::DateTime::<Utc>::from(std::time::SystemTime::UNIX_EPOCH + duration))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            
                            let finding = Finding {
                                id: Uuid::new_v4().to_string(),
                                category: FindingCategory::Metadata,
                                title: format!("PDF File with Embedded Metadata: {}", path.file_name().unwrap_or_default().to_string_lossy()),
                                description: "PDF files often contain embedded metadata including author name, creator application, creation/modification timestamps, and revision history. This can expose identity information and document history.".to_string(),
                                severity: Severity::Critical,
                                location: path.to_string_lossy().to_string(),
                                value_hash: String::new(),
                                metadata: FindingMetadata {
                                    source_application: Some("PDF Analysis".to_string()),
                                    file_size: Some(metadata.len()),
                                    last_modified,
                                    additional_info: meta,
                                },
                                remediation: "Remove PDF metadata using tools like ExifTool or re-export PDFs without metadata".to_string(),
                                discovered_at: Utc::now(),
                            };
                            findings.push(finding);
                        }
                    }
                }
            }
        }

        Ok(findings)
    }

    /// Scan for Office documents with embedded properties
    pub async fn scan_office_metadata(directory: &PathBuf) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        if !directory.exists() {
            return Ok(findings);
        }

        let office_extensions = vec!["docx", "xlsx", "pptx", "doc", "xls", "ppt"];

        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if office_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            if let Ok(metadata) = fs::metadata(&path) {
                                let mut meta = HashMap::new();
                                meta.insert("file_type".to_string(), "Office Document".to_string());
                                meta.insert("privacy_risk".to_string(), "Author, company, revision history, timestamps".to_string());
                                
                                let last_modified = if let Ok(t) = metadata.modified() {
                                    if let Ok(duration) = t.duration_since(std::time::SystemTime::UNIX_EPOCH) {
                                        Some(chrono::DateTime::<Utc>::from(std::time::SystemTime::UNIX_EPOCH + duration))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                };
                                
                                let finding = Finding {
                                    id: Uuid::new_v4().to_string(),
                                    category: FindingCategory::Metadata,
                                    title: format!("Office Document with Metadata: {}", path.file_name().unwrap_or_default().to_string_lossy()),
                                    description: "Microsoft Office documents (.docx, .xlsx, .pptx) embed metadata including author name, company, creation date, modification history, and tracked changes. This reveals identity and document version history.".to_string(),
                                    severity: Severity::Critical,
                                    location: path.to_string_lossy().to_string(),
                                    value_hash: String::new(),
                                    metadata: FindingMetadata {
                                        source_application: Some("Office Analysis".to_string()),
                                        file_size: Some(metadata.len()),
                                        last_modified,
                                        additional_info: meta,
                                    },
                                    remediation: "Use Office 'Inspect Document' feature or ExifTool to remove all metadata before sharing".to_string(),
                                    discovered_at: Utc::now(),
                                };
                                findings.push(finding);
                            }
                        }
                    }
                }
            }
        }

        Ok(findings)
    }

    /// Scan file system properties for metadata leaks
    pub async fn scan_file_properties(directory: &PathBuf) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        if !directory.exists() {
            return Ok(findings);
        }

        // Check for file properties that might reveal information
        if let Ok(metadata) = fs::metadata(directory) {
            if let Ok(modified) = metadata.modified() {
                let last_modified = if let Ok(duration) = modified.duration_since(std::time::SystemTime::UNIX_EPOCH) {
                    Some(chrono::DateTime::<Utc>::from(std::time::SystemTime::UNIX_EPOCH + duration))
                } else {
                    None
                };
                
                let finding = Finding {
                    id: Uuid::new_v4().to_string(),
                    category: FindingCategory::Metadata,
                    title: format!("Directory with Accessible File Properties: {}", directory.display()),
                    description: "File system metadata (creation/modification times, file sizes, permissions) can reveal document creation patterns and activity timelines.".to_string(),
                    severity: Severity::Medium,
                    location: directory.to_string_lossy().to_string(),
                    value_hash: String::new(),
                    metadata: FindingMetadata {
                        source_application: Some("File System Analysis".to_string()),
                        file_size: None,
                        last_modified,
                        additional_info: {
                            let mut map = HashMap::new();
                            map.insert("exposure".to_string(), "File timestamps reveal activity".to_string());
                            map
                        },
                    },
                    remediation: "Use file shredding tools that securely delete files with timestamp clearing".to_string(),
                    discovered_at: Utc::now(),
                };
                findings.push(finding);
            }
        }

        Ok(findings)
    }

    /// Comprehensive metadata scan
    pub async fn scan_all_metadata(base_directory: &PathBuf) -> Result<Vec<Finding>> {
        let mut all_findings = Vec::new();

        // Scan common user directories
        let scan_dirs = vec![
            base_directory.join("Pictures"),
            base_directory.join("Documents"),
            base_directory.join("Downloads"),
            base_directory.join("Desktop"),
        ];

        for dir in scan_dirs {
            if dir.exists() {
                // Image metadata
                if let Ok(image_findings) = Self::scan_image_metadata(&dir).await {
                    all_findings.extend(image_findings);
                }

                // PDF metadata
                if let Ok(pdf_findings) = Self::scan_pdf_metadata(&dir).await {
                    all_findings.extend(pdf_findings);
                }

                // Office metadata
                if let Ok(office_findings) = Self::scan_office_metadata(&dir).await {
                    all_findings.extend(office_findings);
                }

                // File properties
                if let Ok(props_findings) = Self::scan_file_properties(&dir).await {
                    all_findings.extend(props_findings);
                }
            }
        }

        Ok(all_findings)
    }
}
