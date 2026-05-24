use privacy_auditor::{models::ScanConfig, scanner::FileSystemScanner};
use std::time::Instant;

#[tokio::main]
async fn main() {
    println!("=== Filesystem Scanner Benchmark ===\n");

    let config = ScanConfig {
        scan_downloads: true,
        scan_desktop: true,
        scan_documents: true,
        scan_appdata: false, // Skip app data for faster benchmark
        ..Default::default()
    };

    let scanner = FileSystemScanner::new(&config);
    let home = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users".to_string());
    let paths = vec![
        std::path::PathBuf::from(&home).join("Downloads"),
        std::path::PathBuf::from(&home).join("Desktop"),
        std::path::PathBuf::from(&home).join("Documents"),
    ];

    let start = Instant::now();
    match scanner.scan_directories(&paths).await {
        Ok(files) => {
            let elapsed = start.elapsed();
            println!("✓ Files scanned: {}", files.len());
            println!("✓ Time elapsed: {:.2}s", elapsed.as_secs_f64());
            println!(
                "✓ Files/sec: {:.0}",
                files.len() as f64 / elapsed.as_secs_f64()
            );

            // Target: 50k files in <20 seconds
            if files.len() >= 50000 && elapsed.as_secs() < 20 {
                println!("\n✓ PERFORMANCE TARGET MET!");
            } else if files.len() < 50000 {
                println!("\n⚠ Benchmark: Need more test data (target 50k files)");
            } else {
                println!("\n✗ PERFORMANCE TARGET NOT MET - optimize parallelization");
            }
        }
        Err(e) => println!("✗ Scan failed: {}", e),
    }
}
