//! Simplified scanning pipeline that actually works

use crate::error::Result;
use crate::models::Finding;
use crate::secret_detection::SecretDetector;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct PipelineConfig {
    pub batch_insert_size: usize,
    pub enable_hashing: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            batch_insert_size: 500,
            enable_hashing: true,
        }
    }
}

pub struct ScanningPipeline {
    config: PipelineConfig,
}

impl ScanningPipeline {
    pub fn new(_config: PipelineConfig) -> Self {
        Self {
            config: PipelineConfig::default(),
        }
    }

    pub fn scan_and_detect(&self, paths: &[PathBuf]) -> Result<Vec<Finding>> {
        let mut all_findings = Vec::new();
        let detector = SecretDetector::new()?;
        let start = Instant::now();

        for path in paths {
            // Scan files in directory
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                let file_path = entry.path();
                
                // Skip large files
                if let Ok(metadata) = entry.metadata() {
                    if metadata.len() > 100 * 1024 * 1024 {
                        continue;
                    }
                }

                // Try to read and detect secrets
                if let Ok(content) = std::fs::read_to_string(file_path) {
                    match detector.detect_in_content(&content, file_path) {
                        Ok(findings) => all_findings.extend(findings),
                        Err(_) => {} // Silently skip files that can't be processed
                    }
                }
            }
        }

        let elapsed = start.elapsed();
        println!("✅ Scan complete in {:.2}s - Found {} secrets", elapsed.as_secs_f64(), all_findings.len());

        Ok(all_findings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let _pipeline = ScanningPipeline::new(PipelineConfig::default());
    }
}
