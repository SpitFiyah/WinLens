//! Privacy Debt Auditor - Rust Backend Library
//!
//! A high-performance, Windows-first, local-only privacy forensic auditor.
//! Identifies, maps, visualizes, and explains sensitive personal data exposure.

pub mod browser_analysis;
pub mod database;
pub mod error;
pub mod metadata_analyzer;
pub mod models;
pub mod privacy_score;
pub mod scanner;
pub mod scanner_concurrent_simple;
pub mod scanning_pipeline_simple;
pub mod secret_detection;
pub mod windows_artifacts;

// Re-export for convenience
pub use scanning_pipeline_simple::{ScanningPipeline, PipelineConfig};
pub use scanner_concurrent_simple::ConcurrentFileSystemScanner;
pub use models::{Finding, PrivacyDebtScore};
pub use database::Database;
pub use privacy_score::ScoreCalculator;

pub use error::{Result, AuditError};
pub use models::*;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
