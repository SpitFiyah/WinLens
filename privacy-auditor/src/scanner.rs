//! Filesystem scanner module

use crate::error::Result;
use crate::models::*;
use ignore::WalkBuilder;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

pub struct FileSystemScanner {
    exclude_patterns: Vec<String>,
    max_file_size: Option<u64>,
}

impl FileSystemScanner {
    pub fn new(config: &ScanConfig) -> Self {
        Self {
            exclude_patterns: config.exclude_patterns.clone(),
            max_file_size: config.max_file_size,
        }
    }

    /// Scan a directory recursively
    pub async fn scan_directory<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Vec<PathBuf>> {
        let path = path.as_ref();
        if !path.exists() {
            warn!("Path does not exist: {:?}", path);
            return Ok(Vec::new());
        }

        let mut files = Vec::new();
        let walker = WalkBuilder::new(path)
            .hidden(false)
            .ignore(true)
            .build();

        for entry in walker {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() {
                        // Check file size
                        if let Ok(metadata) = path.metadata() {
                            if let Some(max_size) = self.max_file_size {
                                if metadata.len() > max_size {
                                    debug!("Skipping file exceeding max size: {:?}", path);
                                    continue;
                                }
                            }
                        }
                        files.push(path.to_path_buf());
                    }
                }
                Err(e) => {
                    warn!("Error walking directory: {}", e);
                }
            }
        }

        info!(
            "Scanned directory {:?}, found {} files",
            path,
            files.len()
        );
        Ok(files)
    }

    /// Scan multiple directories
    pub async fn scan_directories(
        &self,
        paths: &[PathBuf],
    ) -> Result<Vec<PathBuf>> {
        let mut all_files = Vec::new();

        for path in paths {
            match self.scan_directory(path).await {
                Ok(files) => all_files.extend(files),
                Err(e) => warn!("Failed to scan {:?}: {}", path, e),
            }
        }

        info!("Total files scanned: {}", all_files.len());
        Ok(all_files)
    }

    /// Get file hash (SHA256)
    pub fn hash_file<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let path = path.as_ref();
        let content = std::fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// Check if file matches exclude patterns
    pub fn should_skip(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.exclude_patterns
            .iter()
            .any(|pattern| path_str.contains(pattern))
    }
}

/// Standard scan paths for Windows
pub fn get_default_scan_paths() -> Result<Vec<PathBuf>> {
    let user_home = std::env::var("USERPROFILE")
        .map_err(|_| crate::error::AuditError::Other("USERPROFILE not set".to_string()))?;
    let base = PathBuf::from(&user_home);

    Ok(vec![
        base.join("Downloads"),
        base.join("Desktop"),
        base.join("Documents"),
        base.join("AppData\\Local"),
        base.join("AppData\\Roaming"),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scanner_creation() {
        let config = ScanConfig::default();
        let scanner = FileSystemScanner::new(&config);
        assert_eq!(scanner.max_file_size, Some(100 * 1024 * 1024));
    }

    #[test]
    fn test_exclude_patterns() {
        let config = ScanConfig {
            exclude_patterns: vec!["node_modules".to_string(), ".git".to_string()],
            ..Default::default()
        };
        let scanner = FileSystemScanner::new(&config);

        let should_skip = scanner.should_skip(&PathBuf::from("/path/node_modules/file.js"));
        assert!(should_skip);

        let should_not_skip = scanner.should_skip(&PathBuf::from("/path/src/file.js"));
        assert!(!should_not_skip);
    }
}
