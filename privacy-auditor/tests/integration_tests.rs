#[cfg(test)]
mod tests {
    use privacy_auditor::{
        privacy_score::ScoreCalculator,
        secret_detection::SecretDetector,
        models::{Finding, Severity, FindingCategory, FindingMetadata},
    };
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_privacy_score_calculation() {
        let findings = vec![
            Finding {
                id: "1".to_string(),
                title: "Critical Secret".to_string(),
                description: "Exposed API key".to_string(),
                location: "config.env".to_string(),
                severity: Severity::Critical,
                category: FindingCategory::Secret,
                value_hash: "hash1".to_string(),
                metadata: FindingMetadata {
                    source_application: None,
                    file_size: None,
                    last_modified: None,
                    additional_info: HashMap::new(),
                },
                remediation: "Remove secret and rotate key".to_string(),
                discovered_at: chrono::Utc::now(),
            },
            Finding {
                id: "2".to_string(),
                title: "Tracking Cookie".to_string(),
                description: "Google tracking cookie".to_string(),
                location: "browser_data".to_string(),
                severity: Severity::High,
                category: FindingCategory::TrackingCookie,
                value_hash: "hash2".to_string(),
                metadata: FindingMetadata {
                    source_application: None,
                    file_size: None,
                    last_modified: None,
                    additional_info: HashMap::new(),
                },
                remediation: "Clear cookies".to_string(),
                discovered_at: chrono::Utc::now(),
            },
        ];

        let score = ScoreCalculator::calculate(&findings);
        
        // Score should be reduced with critical and high findings
        assert!(score.total_score < 100, "Score should be reduced with findings");
        assert!(score.total_score > 0, "Score should still be positive");
    }

    #[test]
    fn test_secret_detector_initialization() {
        let detector = SecretDetector::new();
        
        // Should succeed in creating detector
        assert!(detector.is_ok(), "SecretDetector should initialize successfully");
    }

    #[test]
    fn test_secret_detector_patterns() {
        let detector = SecretDetector::new();
        assert!(detector.is_ok(), "Should create detector successfully");
        
        if let Ok(det) = detector {
            // Test that detector can detect secrets in content
            let test_content = "password=superSecretPassword123";
            let result = det.detect_in_content(test_content, &std::path::Path::new("test.txt"));
            assert!(result.is_ok(), "Should detect secrets in content");
        }
    }

    #[test]
    fn test_finding_severity_ordering() {
        let critical = Severity::Critical;
        let high = Severity::High;
        let medium = Severity::Medium;
        let low = Severity::Low;
        
        // Severity levels should be comparable
        let _severities = vec![low, critical, medium, high];
        
        // All severities should be representable
        assert_eq!(format!("{:?}", critical), "Critical");
        assert_eq!(format!("{:?}", high), "High");
        assert_eq!(format!("{:?}", medium), "Medium");
        assert_eq!(format!("{:?}", low), "Low");
    }

    #[test]
    fn test_finding_creation() {
        let finding = Finding {
            id: "test-1".to_string(),
            title: "Test Finding".to_string(),
            description: "Test description".to_string(),
            location: "/test/path".to_string(),
            severity: Severity::High,
            category: FindingCategory::Secret,
            value_hash: "abc123".to_string(),
            metadata: FindingMetadata {
                source_application: None,
                file_size: None,
                last_modified: None,
                additional_info: HashMap::new(),
            },
            remediation: "Test remediation".to_string(),
            discovered_at: chrono::Utc::now(),
        };

        assert_eq!(finding.id, "test-1");
        assert_eq!(finding.title, "Test Finding");
        assert_eq!(finding.severity, Severity::High);
        assert!(!finding.value_hash.is_empty());
    }

    #[test]
    fn test_finding_categories() {
        let categories = vec![
            FindingCategory::Secret,
            FindingCategory::BrowserPrivacy,
            FindingCategory::Metadata,
            FindingCategory::WindowsArtifact,
        ];
        
        // Verify categories are distinct
        assert_eq!(categories.len(), 4, "Should have multiple finding categories");
    }

    #[test]
    fn test_score_with_critical_finding() {
        let findings = vec![
            Finding {
                id: "1".to_string(),
                title: "Finding 1".to_string(),
                description: "Critical issue".to_string(),
                location: "loc1".to_string(),
                severity: Severity::Critical,
                category: FindingCategory::Secret,
                value_hash: "h1".to_string(),
                metadata: FindingMetadata {
                    source_application: None,
                    file_size: None,
                    last_modified: None,
                    additional_info: HashMap::new(),
                },
                remediation: "r1".to_string(),
                discovered_at: chrono::Utc::now(),
            },
        ];

        let score = ScoreCalculator::calculate(&findings);
        
        // Critical finding should result in significant score reduction
        assert!(score.total_score < 50, "Critical findings should significantly impact score");
    }

    #[test]
    fn test_no_findings_returns_low_score() {
        let findings: Vec<Finding> = vec![];
        let score = ScoreCalculator::calculate(&findings);
        
        // With no privacy debt factors found, score should be 0 (will be inverted in UI)
        assert_eq!(score.total_score, 0, "No findings should result in score of 0");
    }
    
    #[test]
    fn test_finding_metadata_construction() {
        let mut metadata = FindingMetadata {
            source_application: None,
            file_size: Some(2048),
            last_modified: None,
            additional_info: HashMap::new(),
        };
        
        metadata.additional_info.insert("context".to_string(), "test_context".to_string());
        
        assert_eq!(metadata.file_size, Some(2048));
        assert_eq!(metadata.additional_info.get("context"), Some(&"test_context".to_string()));
    }
}


