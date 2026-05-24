//! Concurrent filesystem scanner using Tokio for parallelization
//! 
//! This module provides high-performance parallel file scanning
//! targeting 50k files in <20 seconds.

use crate::error::Result;
use crate::models::*;
use ignore::WalkBuilder;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use tracing::{debug, info};

/// Configuration for concurrent scanning
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
            max_concurrent_tasks: num_cpus::get() * 2,
            batch_size: 1000,
            max_file_size: Some(100 * 1024 * 1024), // 100 MB
            exclude_patterns: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "target".to_string(),
                ".cache".to_string(),
                "$Recycle.Bin".to_string(),
                "System Volume Information".to_string(),
            ],
        }
    }
}

/// Progress callback for real-time updates
pub type ProgressCallback = Box<dyn Fn(u64, &str) + Send + Sync>;

/// Concurrent filesystem scanner
pub struct ConcurrentFileSystemScanner {
    config: ConcurrentScanConfig,
    files_processed: Arc<AtomicU64>,
}

impl ConcurrentFileSystemScanner {
    pub fn new(config: ConcurrentScanConfig) -> Self {
        Self {
            config,
            files_processed: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Get files processed counter
    pub fn files_processed(&self) -> u64 {
        self.files_processed.load(Ordering::Relaxed)
    }

    /// Scan directory with concurrent file processing
    /// Returns iterator of file paths
    pub async fn scan_concurrent<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Vec<PathBuf>> {
        let path = path.as_ref();
        if !path.exists() {
            debug!("Path does not exist: {:?}", path);
            return Ok(Vec::new());
        }

        // First pass: collect all files
        let mut files = Vec::new();
        let walker = WalkBuilder::new(path)
            .hidden(false)
            .ignore(true)
            .build();

        for entry in walker {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    // Check exclusions
                    let path_str = entry_path.to_string_lossy();
                    if self.config.exclude_patterns.iter().any(|p| path_str.contains(p)) {
                        continue;
                    }

                    // Check file size
                    if let Ok(metadata) = entry_path.metadata() {
                        if let Some(max_size) = self.config.max_file_size {
                            if metadata.len() > max_size {
                                debug!("Skipping file exceeding max size: {:?}", entry_path);
                                continue;
                            }
                        }
                        files.push(entry_path.to_path_buf());
                    }
                }
            }
        }

        info!(
            "Found {} files in {:?}, starting concurrent processing",
            files.len(),
            path
        );

        Ok(files)
    }

    /// Scan multiple directories concurrently
    pub async fn scan_directories_concurrent(
        &self,
        paths: &[PathBuf],
    ) -> Result<Vec<PathBuf>> {
        let mut tasks: Vec<JoinHandle<Result<Vec<PathBuf>>>> = Vec::new();

        for path in paths {
            let scanner = self.clone_for_task();
            let path_clone = path.clone();
            
            let task = tokio::spawn(async move {
                scanner.scan_concurrent(&path_clone).await
            });

            tasks.push(task);
        }

        // Collect results from all tasks
        let mut all_files = Vec::new();
        for task in tasks {
            match task.await {
                Ok(Ok(files)) => all_files.extend(files),
                Ok(Err(e)) => debug!("Task error: {}", e),
                Err(e) => debug!("Task join error: {}", e),
            }
        }

        info!("Total files collected: {}", all_files.len());
        Ok(all_files)
    }

    /// Process files concurrently with rate limiting
    /// Callback is invoked for each file with (count, path)
    pub async fn process_files_concurrent<F>(
        &self,
        files: Vec<PathBuf>,
        callback: F,
    ) -> Result<u64>
    where
        F: Fn(PathBuf) + Send + Sync + 'static,
    {
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_tasks));
        let callback = Arc::new(callback);
        let files_processed = Arc::clone(&self.files_processed);
        let mut tasks = Vec::new();

        for file_path in files {
            let permit = semaphore.acquire().await.unwrap();
            let callback = Arc::clone(&callback);
            let files_processed = Arc::clone(&files_processed);

            let task = tokio::spawn(async move {
                callback(file_path);
                files_processed.fetch_add(1, Ordering::Relaxed);
                drop(permit);
            });

            tasks.push(task);
        }

        // Wait for all tasks
        for task in tasks {
            let _ = task.await;
        }

        let total = self.files_processed();
        info!("Processed {} files concurrently", total);
        Ok(total)
    }

    /// Batch read files with concurrent I/O
    pub async fn batch_read_files(
        &self,
        files: Vec<PathBuf>,
    ) -> Result<Vec<(PathBuf, Vec<u8>)>> {
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_tasks));
        let mut tasks = Vec::new();

        for file_path in files {
            let permit = semaphore.acquire().await.unwrap();

            let task = tokio::spawn(async move {
                match tokio::fs::read(&file_path).await {
                    Ok(content) => Some((file_path, content)),
                    Err(_) => None,
                }
            });

            let task_handle = async move {
                let result = task.await.ok().flatten();
                drop(permit);
                result
            };

            tasks.push(task_handle);
        }

        // Collect results
        let mut results = Vec::new();
        for task in tasks {
            if let Some((path, content)) = task.await {
                results.push((path, content));
                self.files_processed.fetch_add(1, Ordering::Relaxed);
            }
        }

        info!("Batch read {} files", results.len());
        Ok(results)
    }

    /// Compute file hash concurrently
    pub async fn hash_files_concurrent(
        &self,
        files: Vec<PathBuf>,
    ) -> Result<Vec<(PathBuf, String)>> {
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_tasks));
        let mut tasks = Vec::new();

        for file_path in files {
            let permit = semaphore.acquire().await.unwrap();

            let task = tokio::spawn(async move {
                match tokio::fs::read(&file_path).await {
                    Ok(content) => {
                        let mut hasher = Sha256::new();
                        hasher.update(&content);
                        let hash = format!("{:x}", hasher.finalize());
                        Some((file_path, hash))
                    }
                    Err(_) => None,
                }
            });

            let task_handle = async move {
                let result = task.await.ok().flatten();
                drop(permit);
                result
            };

            tasks.push(task_handle);
        }

        // Collect results
        let mut results = Vec::new();
        for task in tasks {
            if let Some((path, hash)) = task.await {
                results.push((path, hash));
                self.files_processed.fetch_add(1, Ordering::Relaxed);
            }
        }

        info!("Hashed {} files", results.len());
        Ok(results)
    }

    fn clone_for_task(&self) -> Self {
        Self {
            config: self.config.clone(),
            files_processed: Arc::clone(&self.files_processed),
        }
    }
}

/// Benchmark utility for performance testing
pub async fn benchmark_scan(paths: &[PathBuf]) -> Result<ScanBenchmark> {
    let config = ConcurrentScanConfig::default();
    let scanner = ConcurrentFileSystemScanner::new(config);

    let start = std::time::Instant::now();

    // Scan directories
    let files = scanner.scan_directories_concurrent(paths).await?;
    let scan_duration = start.elapsed();

    // Hash files
    let hash_start = std::time::Instant::now();
    let _ = scanner.hash_files_concurrent(files.clone()).await?;
    let hash_duration = hash_start.elapsed();

    let total_duration = start.elapsed();
    let files_per_second = files.len() as f64 / total_duration.as_secs_f64();

    Ok(ScanBenchmark {
        files_found: files.len() as u64,
        scan_time_ms: scan_duration.as_millis() as u64,
        hash_time_ms: hash_duration.as_millis() as u64,
        total_time_ms: total_duration.as_millis() as u64,
        files_per_second: files_per_second as u64,
    })
}

/// Benchmark results
#[derive(Debug, Clone)]
pub struct ScanBenchmark {
    pub files_found: u64,
    pub scan_time_ms: u64,
    pub hash_time_ms: u64,
    pub total_time_ms: u64,
    pub files_per_second: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_scanner_creation() {
        let config = ConcurrentScanConfig::default();
        let scanner = ConcurrentFileSystemScanner::new(config);
        assert_eq!(scanner.files_processed(), 0);
    }

    #[test]
    fn test_config_defaults() {
        let config = ConcurrentScanConfig::default();
        assert!(config.max_concurrent_tasks > 0);
        assert!(config.batch_size > 0);
        assert!(!config.exclude_patterns.is_empty());
    }
}
