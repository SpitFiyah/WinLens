//! Optimized scanning pipeline orchestrating filesystem, secret detection, and storage
//! 
//! This module combines concurrent scanning, secret detection, and database operations
//! into a high-performance pipeline targeting 50k files in <20 seconds.

use crate::database::AuditDatabase;
use crate::error::Result;
use crate::models::Finding;
use crate::scanner_concurrent::{ConcurrentFileSystemScanner, ConcurrentScanConfig, ScanBenchmark};
use crate::secret_detection::SecretDetector;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{info, warn};

/// Configuration for the optimized pipeline
#[derive(Clone, Debug)]
pub struct PipelineConfig {
    pub scan_config: ConcurrentScanConfig,
    pub batch_insert_size: usize,
    pub enable_hashing: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            scan_config: ConcurrentScanConfig::default(),
            batch_insert_size: 500,
            enable_hashing: true,
        }
    }
}

/// Progress tracking during pipeline execution
#[derive(Clone, Debug)]
pub struct PipelineProgress {
    pub files_scanned: u64,
    pub secrets_detected: u64,
    pub elapsed_ms: u64,
}

/// Callback for real-time progress updates
pub type ProgressCallback = Box<dyn Fn(PipelineProgress) + Send + Sync>;

/// Optimized scanning pipeline
pub struct ScanningPipeline {
    config: PipelineConfig,
    detector: Arc<SecretDetector>,
    secrets_detected: Arc<AtomicU64>,
}

impl ScanningPipeline {
    pub fn new(config: PipelineConfig) -> Self {
        Self {
            config,
            detector: Arc::new(SecretDetector::new()),
            secrets_detected: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Execute full scanning pipeline
    pub async fn execute(
        &self,
        db: &AuditDatabase,
        paths: &[PathBuf],
        progress_callback: Option<ProgressCallback>,
    ) -> Result<PipelineStats> {
        let start_time = std::time::Instant::now();
        let scanner = ConcurrentFileSystemScanner::new(self.config.scan_config.clone());

        // Phase 1: Scan directories
        info!("Phase 1: Scanning directories...");
        let files = scanner.scan_directories_concurrent(paths).await?;
        let files_count = files.len() as u64;

        if files_count == 0 {
            warn!("No files found in scan paths");
            return Ok(PipelineStats {
                files_scanned: 0,
                findings_count: 0,
                total_time_ms: start_time.elapsed().as_millis() as u64,
                throughput_files_per_sec: 0,
            });
        }

        info!("Phase 1 complete: Found {} files", files_count);

        // Emit progress
        if let Some(ref callback) = progress_callback {
            callback(PipelineProgress {
                files_scanned: files_count,
                secrets_detected: 0,
                elapsed_ms: start_time.elapsed().as_millis() as u64,
            });
        }

        // Phase 2: Process files and detect secrets
        info!("Phase 2: Processing files for secrets...");
        let findings = self
            .process_files_parallel(files.clone(), progress_callback.as_ref())
            .await?;

        info!(
            "Phase 2 complete: Detected {} findings",
            findings.len()
        );

        // Phase 3: Batch insert into database
        info!("Phase 3: Persisting findings to database...");
        self.batch_insert_findings(db, findings.clone()).await?;
        info!("Phase 3 complete: {} findings stored", findings.len());

        let total_time = start_time.elapsed();
        let throughput = if total_time.as_secs_f64() > 0.0 {
            (files_count as f64 / total_time.as_secs_f64()) as u64
        } else {
            0
        };

        Ok(PipelineStats {
            files_scanned: files_count,
            findings_count: findings.len() as u64,
            total_time_ms: total_time.as_millis() as u64,
            throughput_files_per_sec: throughput,
        })
    }

    /// Process files in parallel with secret detection
    async fn process_files_parallel(
        &self,
        files: Vec<PathBuf>,
        progress_callback: Option<&ProgressCallback>,
    ) -> Result<Vec<Finding>> {
        let semaphore = Arc::new(Semaphore::new(self.config.scan_config.max_concurrent_tasks));
        let detector = Arc::clone(&self.detector);
        let mut tasks = Vec::new();
        let start_time = std::time::Instant::now();

        for file_path in files.clone() {
            let permit = semaphore.acquire().await.unwrap();
            let detector = Arc::clone(&detector);
            let secrets_detected = Arc::clone(&self.secrets_detected);
            let progress_callback = progress_callback.cloned();
            let start_time = start_time;
            let total_files = files.len() as u64;

            let task = tokio::spawn(async move {
                let mut file_findings = Vec::new();

                // Try to read and detect secrets
                match tokio::fs::read_to_string(&file_path).await {
                    Ok(content) => {
                        match detector.detect_in_content(&content, &file_path.to_string_lossy()) {
                            Ok(findings) => {
                                file_findings = findings;
                                let count = file_findings.len() as u64;
                                secrets_detected.fetch_add(count, Ordering::Relaxed);

                                // Emit progress callback
                                if let Some(ref callback) = progress_callback {
                                    let elapsed = start_time.elapsed().as_millis() as u64;
                                    callback(PipelineProgress {
                                        files_scanned: total_files,
                                        secrets_detected: secrets_detected.load(Ordering::Relaxed),
                                        elapsed_ms: elapsed,
                                    });
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }

                drop(permit);
                file_findings
            });

            tasks.push(task);
        }

        // Collect results from all tasks
        let mut all_findings = Vec::new();
        for task in tasks {
            match task.await {
                Ok(findings) => all_findings.extend(findings),
                Err(e) => warn!("Task error: {}", e),
            }
        }

        Ok(all_findings)
    }

    /// Batch insert findings into database for efficiency
    async fn batch_insert_findings(
        &self,
        db: &AuditDatabase,
        findings: Vec<Finding>,
    ) -> Result<()> {
        for batch in findings.chunks(self.config.batch_insert_size) {
            for finding in batch {
                db.insert_finding(finding)?;
            }
        }
        Ok(())
    }
}

/// Statistics from pipeline execution
#[derive(Clone, Debug)]
pub struct PipelineStats {
    pub files_scanned: u64,
    pub findings_count: u64,
    pub total_time_ms: u64,
    pub throughput_files_per_sec: u64,
}

impl PipelineStats {
    /// Check if performance targets are met
    pub fn meets_targets(&self) -> bool {
        // Target: 50k files in 20 seconds
        let target_ms = 20_000;
        let target_files = 50_000;

        if self.files_scanned >= target_files {
            self.total_time_ms <= target_ms
        } else {
            // Scale target proportionally
            let scaled_target = (target_ms as f64 * self.files_scanned as f64 / target_files as f64)
                as u64;
            self.total_time_ms <= scaled_target
        }
    }

    /// Get performance summary
    pub fn summary(&self) -> String {
        format!(
            "Scanned {} files in {}ms ({} files/sec)",
            self.files_scanned, self.total_time_ms, self.throughput_files_per_sec
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let config = PipelineConfig::default();
        let _pipeline = ScanningPipeline::new(config);
    }

    #[test]
    fn test_stats_meets_targets() {
        let stats = PipelineStats {
            files_scanned: 50_000,
            findings_count: 100,
            total_time_ms: 15_000,
            throughput_files_per_sec: 3_333,
        };
        assert!(stats.meets_targets());
    }

    #[test]
    fn test_stats_misses_targets() {
        let stats = PipelineStats {
            files_scanned: 50_000,
            findings_count: 100,
            total_time_ms: 30_000,
            throughput_files_per_sec: 1_666,
        };
        assert!(!stats.meets_targets());
    }

    #[test]
    fn test_stats_summary() {
        let stats = PipelineStats {
            files_scanned: 1000,
            findings_count: 10,
            total_time_ms: 5000,
            throughput_files_per_sec: 200,
        };
        let summary = stats.summary();
        assert!(summary.contains("1000 files"));
        assert!(summary.contains("5000ms"));
    }
}
