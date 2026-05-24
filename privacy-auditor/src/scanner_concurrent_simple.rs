//! Simplified scanner - actually working version

use crate::error::Result;
use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct ConcurrentScanConfig {
    pub max_concurrent_tasks: usize,
    pub batch_size: usize,
    pub max_file_size: Option<u64>,
    pub exclude_patterns: Vec<String>,
}

impl Default for ConcurrentScanConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 4,
            batch_size: 1000,
            max_file_size: Some(100 * 1024 * 1024),
            exclude_patterns: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "target".to_string(),
            ],
        }
    }
}

pub struct ConcurrentFileSystemScanner {
    pub config: ConcurrentScanConfig,
}

impl ConcurrentFileSystemScanner {
    pub fn new(config: ConcurrentScanConfig) -> Self {
        Self { config }
    }

    pub fn scan_directory(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                let path_str = entry.path().to_string_lossy().to_lowercase();
                
                // Skip excluded patterns
                if self.config.exclude_patterns.iter().any(|p| path_str.contains(p)) {
                    continue;
                }

                // Check size limit
                if let Ok(metadata) = entry.metadata() {
                    if let Some(max_size) = self.config.max_file_size {
                        if metadata.len() > max_size {
                            continue;
                        }
                    }
                }

                files.push(entry.path().to_path_buf());
            }
        }

        Ok(files)
    }

    pub fn scan_multiple(&self, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut all_files = Vec::new();
        for path in paths {
            let mut files = self.scan_directory(path)?;
            all_files.append(&mut files);
        }
        Ok(all_files)
    }

    pub fn hash_file(&self, path: &Path) -> Result<String> {
        let content = std::fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        Ok(format!("{:x}", hasher.finalize()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let c = ConcurrentScanConfig::default();
        assert!(c.max_concurrent_tasks > 0);
    }
}
