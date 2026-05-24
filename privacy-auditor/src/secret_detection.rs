//! Secret detection engine

use crate::error::Result;
use crate::models::*;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::path::Path;
use tracing::{debug, info};

pub struct SecretDetector {
    patterns: Vec<SecretPattern>,
}

#[derive(Clone)]
pub struct SecretPattern {
    pub name: String,
    pub pattern: Regex,
    pub severity: Severity,
    pub description: String,
}

impl SecretDetector {
    pub fn new() -> Result<Self> {
        let patterns = Self::load_patterns()?;
        Ok(Self { patterns })
    }

    /// Load all secret detection patterns
    fn load_patterns() -> Result<Vec<SecretPattern>> {
        Ok(vec![
            SecretPattern {
                name: "AWS API Key".to_string(),
                pattern: Regex::new(r"AKIA[0-9A-Z]{16}")
                    .map_err(|e| crate::error::AuditError::RegexError(e.to_string()))?,
                severity: Severity::Critical,
                description: "AWS Access Key found in file. This grants access to AWS resources."
                    .to_string(),
            },
            SecretPattern {
                name: "Private SSH Key".to_string(),
                pattern: Regex::new(
                    r"-----BEGIN (RSA|EC|OPENSSH) PRIVATE KEY-----",
                )
                .map_err(|e| crate::error::AuditError::RegexError(e.to_string()))?,
                severity: Severity::Critical,
                description: "SSH private key found. This grants unauthorized access to systems."
                    .to_string(),
            },
            SecretPattern {
                name: "GitHub Token".to_string(),
                pattern: Regex::new(r"ghp_[A-Za-z0-9_]{36}")
                    .map_err(|e| crate::error::AuditError::RegexError(e.to_string()))?,
                severity: Severity::Critical,
                description: "GitHub personal access token found."
                    .to_string(),
            },
            SecretPattern {
                name: "Generic API Key".to_string(),
                pattern: Regex::new(r#"api[_-]?key\s*[=:]\s*['\"]?[a-zA-Z0-9]{32,}"#)
                    .map_err(|e| crate::error::AuditError::RegexError(e.to_string()))?,
                severity: Severity::High,
                description: "API key found in configuration file.".to_string(),
            },
            SecretPattern {
                name: "Database Password".to_string(),
                pattern: Regex::new(
                    r#"(?i)(password|passwd|pwd)\s*[=:]\s*['"]?[a-zA-Z0-9!@#$%^&*]{6,}['"]?"#
                )
                .map_err(|e| crate::error::AuditError::RegexError(e.to_string()))?,
                severity: Severity::High,
                description: "Database password found in configuration file.".to_string(),
            },
            SecretPattern {
                name: "JWT Token".to_string(),
                pattern: Regex::new(
                    r"eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+"
                )
                .map_err(|e| crate::error::AuditError::RegexError(e.to_string()))?,
                severity: Severity::High,
                description: "JWT authentication token found.".to_string(),
            },
            SecretPattern {
                name: "OAuth Token".to_string(),
                pattern: Regex::new(r#"(?i)(access_token|oauth_token)\s*[=:]\s*['"]?[a-zA-Z0-9._-]{20,}['"]?"#)
                    .map_err(|e| crate::error::AuditError::RegexError(e.to_string()))?,
                severity: Severity::High,
                description: "OAuth token found in file.".to_string(),
            },
            SecretPattern {
                name: "Crypto Seed Phrase".to_string(),
                pattern: Regex::new(
                    r"\b(abandon|ability|able|about|above|absent|absorb|abstract|abuse|access|accident|account|accuse|achieve|acid|across|act|action|actor|actress|acts|actual|acuity|acute|ad|ada|adam|add|addict|adding|addle|address|adds|adequate|adhere|adjacent|adjust|admin|admire|admit|admix|admittance|admitted|admits|adopter|adopts|adore|adorned|adorns|adorn|adornment|adornments|adornments|adornments|adornments|adornments|adornments|adornments|adornments)\b.*?\b(about|accept|access|accident|account|accuse|achieve|acid|across|act|action|actor|actress|acts|actual|acuity|acute|ad|ada|adam|add|addict|adding|addle|address|adds|adequate|adhere|adjacent|adjust|admin|admire|admit|admix|admittance|admitted|admits|adopter|adopts|adore|adorned|adorns|adorn|adornment)\b"
                )
                .map_err(|e| crate::error::AuditError::RegexError(e.to_string()))?,
                severity: Severity::Critical,
                description: "Cryptocurrency seed phrase or mnemonic found.".to_string(),
            },
            SecretPattern {
                name: "Private Key (Generic)".to_string(),
                pattern: Regex::new(r#"(?i)private[_-]?key\s*[=:]\s*['"]?[a-zA-Z0-9+/=]{32,}['"]?"#)
                    .map_err(|e| crate::error::AuditError::RegexError(e.to_string()))?,
                severity: Severity::Critical,
                description: "Private key found in file.".to_string(),
            },
        ]
        .into_iter()
        .collect::<Vec<_>>())
    }

    /// Detect secrets in file content
    pub fn detect_in_content(
        &self,
        content: &str,
        file_path: &Path,
    ) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        for pattern in &self.patterns {
            for mat in pattern.pattern.find_iter(content) {
                let secret_value = mat.as_str();

                // Skip if it's likely a placeholder or example
                if Self::is_placeholder(secret_value) {
                    continue;
                }

                // Hash the secret (don't store plaintext)
                let value_hash = Self::hash_secret(secret_value);

                let finding = Finding::new(
                    FindingCategory::Secret,
                    pattern.severity,
                    pattern.name.clone(),
                    pattern.description.clone(),
                    file_path.to_string_lossy().to_string(),
                    value_hash,
                    format!(
                        "Remove the {} from {}. Rotate any credentials with access to your systems.",
                        pattern.name.to_lowercase(),
                        file_path.display()
                    ),
                );

                findings.push(finding);
                debug!(
                    "Found {} in {:?}",
                    pattern.name,
                    file_path
                );
            }
        }

        Ok(findings)
    }

    /// Detect secrets in a file
    pub fn detect_in_file(&self, file_path: &Path) -> Result<Vec<Finding>> {
        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => return Ok(Vec::new()), // Skip binary files
        };

        self.detect_in_content(&content, file_path)
    }

    /// Check if value is a placeholder (skip false positives)
    fn is_placeholder(value: &str) -> bool {
        let lower = value.to_lowercase();
        lower.contains("xxx")
            || lower.contains("xxxxxxxx")
            || lower.contains("placeholder")
            || lower.contains("example")
            || lower.contains("demo")
            || lower.contains("test")
            || lower.contains("fake")
    }

    /// Hash a secret (SHA256) so we don't store plaintext
    fn hash_secret(secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl Default for SecretDetector {
    fn default() -> Self {
        Self::new().expect("Failed to initialize SecretDetector")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aws_key_detection() {
        let detector = SecretDetector::new().expect("Failed to create detector");
        let content = "aws_access_key_id = AKIAIOSFODNN7REALKEY";
        let findings = detector
            .detect_in_content(content, &Path::new("test.txt"))
            .expect("Detection failed");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_placeholder_filtering() {
        assert!(SecretDetector::is_placeholder("AKIA0000000000XXXX"));
        assert!(SecretDetector::is_placeholder("your-api-key-here-example"));
        assert!(!SecretDetector::is_placeholder("AKIA1234567890ABCD"));
    }
}
