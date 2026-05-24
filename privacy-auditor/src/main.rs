use privacy_auditor::{
    database::Database, error::Result, models::*, scanner::FileSystemScanner,
    secret_detection::SecretDetector, privacy_score::ScoreCalculator,
};
use std::path::PathBuf;
use tracing::{info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Privacy Debt Auditor v{}", privacy_auditor::VERSION);

    // Example: Run a basic scan
    let config = ScanConfig::default();
    let scanner = FileSystemScanner::new(&config);
    let detector = SecretDetector::new()?;

    // Initialize database
    let db = Database::new("audit.db")?;

    // Get home directory
    let home = std::env::var("USERPROFILE")
        .unwrap_or_else(|_| "C:\\Users".to_string());
    let downloads = PathBuf::from(&home).join("Downloads");

    if downloads.exists() {
        // Scan Downloads folder
        info!("Scanning: {:?}", downloads);
        let files = scanner.scan_directory(&downloads).await?;
        info!("Found {} files", files.len());

        let mut all_findings = Vec::new();

        // Detect secrets in each file
        for file in files.iter().take(100) {
            // Limit to first 100 files for demo
            match detector.detect_in_file(file) {
                Ok(findings) => {
                    for finding in findings {
                        db.insert_finding(&finding)?;
                        all_findings.push(finding);
                    }
                }
                Err(e) => {
                    tracing::debug!("Error scanning {:?}: {}", file, e);
                }
            }
        }

        // Calculate privacy score
        let score = ScoreCalculator::calculate(&all_findings);
        info!("Privacy Debt Score: {}/100", score.total_score);
        info!("Found {} findings", all_findings.len());
    }

    info!("Audit complete!");
    Ok(())
}
