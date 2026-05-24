//! Privacy Debt Score calculation

use crate::models::*;
use crate::error::Result;
use tracing::info;

pub struct ScoreCalculator;

impl ScoreCalculator {
    /// Calculate privacy debt score from findings
    pub fn calculate(findings: &[Finding]) -> PrivacyDebtScore {
        let mut factors = ScoreFactors {
            exposed_secrets: 0,
            tracking_cookies: 0,
            cached_identifiers: 0,
            browser_persistence: 0,
            metadata_leakage: 0,
            deleted_artifacts: 0,
            stale_sessions: 0,
            risky_storage: 0,
        };

        // Categorize findings and calculate factor scores
        for finding in findings {
            match finding.category {
                FindingCategory::Secret => {
                    factors.exposed_secrets += match finding.severity {
                        Severity::Critical => 30,
                        Severity::High => 20,
                        Severity::Medium => 10,
                        Severity::Low => 5,
                    };
                }
                FindingCategory::TrackingCookie => {
                    factors.tracking_cookies += match finding.severity {
                        Severity::Critical => 8,
                        Severity::High => 5,
                        Severity::Medium => 3,
                        Severity::Low => 1,
                    };
                }
                FindingCategory::CachedIdentifier => {
                    factors.cached_identifiers += match finding.severity {
                        Severity::Critical => 15,
                        Severity::High => 10,
                        Severity::Medium => 5,
                        Severity::Low => 2,
                    };
                }
                FindingCategory::Metadata => {
                    factors.metadata_leakage += match finding.severity {
                        Severity::Critical => 10,
                        Severity::High => 7,
                        Severity::Medium => 4,
                        Severity::Low => 1,
                    };
                }
                FindingCategory::DeletedArtifact => {
                    factors.deleted_artifacts += match finding.severity {
                        Severity::Critical => 12,
                        Severity::High => 8,
                        Severity::Medium => 4,
                        Severity::Low => 1,
                    };
                }
                FindingCategory::SessionToken => {
                    factors.stale_sessions += match finding.severity {
                        Severity::Critical => 25,
                        Severity::High => 15,
                        Severity::Medium => 8,
                        Severity::Low => 3,
                    };
                }
                _ => {}
            }
        }

        // Calculate total score (0-100 scale)
        let total = (factors.exposed_secrets
            + factors.tracking_cookies
            + factors.cached_identifiers
            + factors.browser_persistence
            + factors.metadata_leakage
            + factors.deleted_artifacts
            + factors.stale_sessions
            + factors.risky_storage) as u32;

        // Normalize to 0-100 scale with diminishing returns
        let normalized_score = std::cmp::min(
            100,
            (total as f64 * 0.8 + (findings.len() as f64 * 0.2).sqrt() as f64) as u32,
        );

        info!(
            "Privacy Debt Score calculated: {}/100 from {} findings",
            normalized_score,
            findings.len()
        );

        PrivacyDebtScore {
            total_score: normalized_score,
            score_timestamp: chrono::Utc::now(),
            factors,
            findings_count: findings.len(),
        }
    }
}
