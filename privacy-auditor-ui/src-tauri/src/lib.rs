// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Emitter, State};
use std::sync::{Arc, Mutex as StdMutex};
use std::path::PathBuf;
use privacy_auditor::{
    browser_analysis::BrowserAnalyzer,
    metadata_analyzer::MetadataAnalyzer,
    windows_artifacts::WindowsArtifactAnalyzer,
    Database, PrivacyDebtScore,
    ScanningPipeline, PipelineConfig,
    Finding, ScoreCalculator,
};
use serde::{Serialize, Deserialize};

/// Response types for frontend
#[derive(Serialize, Clone)]
pub struct ScanProgressEvent {
    pub files_scanned: u64,
    pub secrets_detected: u64,
    pub progress_percent: u32,
    pub current_file: String,
}

#[derive(Serialize, Clone)]
pub struct ScanCompleteEvent {
    pub score: PrivacyDebtScore,
    pub findings: Vec<Finding>,
    pub total_findings: usize,
    pub scan_duration_secs: f64,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub paths: Vec<String>,
    pub include_browser: bool,
    pub include_registry: bool,
}

/// Application state
pub struct AppState {
    pub db: Arc<StdMutex<Database>>,
}

/// Scan directories for privacy findings
#[tauri::command]
async fn scan_directories(
    request: ScanRequest,
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<ScanCompleteEvent, String> {
    let start = std::time::Instant::now();
    
    let paths: Vec<PathBuf> = request.paths.iter().map(PathBuf::from).collect();
    if paths.is_empty() {
        return Err("No paths provided".to_string());
    }

    let _ = app_handle.emit(
        "scan:progress",
        ScanProgressEvent {
            files_scanned: 0,
            secrets_detected: 0,
            progress_percent: 5,
            current_file: "Preparing selected scan locations...".to_string(),
        },
    );

    // Run filesystem secret detection.
    let pipeline = ScanningPipeline::new(PipelineConfig::default());
    let mut findings = pipeline.scan_and_detect(&paths).map_err(|e| e.to_string())?;

    // Run metadata detection against the selected filesystem roots.
    for path in &paths {
        if let Ok(metadata_findings) = MetadataAnalyzer::scan_image_metadata(path).await {
            findings.extend(metadata_findings);
        }

        if let Ok(metadata_findings) = MetadataAnalyzer::scan_pdf_metadata(path).await {
            findings.extend(metadata_findings);
        }

        if let Ok(metadata_findings) = MetadataAnalyzer::scan_office_metadata(path).await {
            findings.extend(metadata_findings);
        }
    }

    if request.include_browser {
        if let Ok(browser_findings) = BrowserAnalyzer::scan_all_browsers().await {
            findings.extend(browser_findings);
        }
    }

    if request.include_registry {
        if let Ok(windows_findings) = WindowsArtifactAnalyzer::scan_all_artifacts().await {
            findings.extend(windows_findings);
        }
    }

    // Emit progress
    let _ = app_handle.emit(
        "scan:progress",
        ScanProgressEvent {
            files_scanned: findings.len() as u64,
            secrets_detected: findings.len() as u64,
            progress_percent: 50,
            current_file: "Analyzing findings and calculating privacy score...".to_string(),
        },
    );

    // Get database
    let db = state.db.lock().map_err(|e| e.to_string())?;

    // Store findings in database
    for finding in &findings {
        let _ = db.insert_finding(finding);
    }

    // Calculate score
    let score = ScoreCalculator::calculate(&findings);
    let duration = start.elapsed().as_secs_f64();

    // Emit completion
    let _ = app_handle.emit(
        "scan:complete",
        ScanCompleteEvent {
            score: score.clone(),
            findings: findings.clone(),
            total_findings: findings.len(),
            scan_duration_secs: duration,
        },
    );

    let total_findings = findings.len();

    Ok(ScanCompleteEvent {
        score,
        findings,
        total_findings,
        scan_duration_secs: duration,
    })
}

/// Get all findings
#[tauri::command]
fn get_findings(state: State<'_, AppState>) -> Result<Vec<Finding>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_findings().map_err(|e| e.to_string())
}

/// Clear all findings
#[tauri::command]
fn clear_findings(state: State<'_, AppState>) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.clear_findings().map_err(|e| e.to_string())?;
    Ok(true)
}

/// Get privacy score
#[tauri::command]
fn get_privacy_score(state: State<'_, AppState>) -> Result<PrivacyDebtScore, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let findings = db.get_findings().map_err(|e| e.to_string())?;
    Ok(ScoreCalculator::calculate(&findings))
}

/// Get user home directory
#[tauri::command]
fn get_user_home() -> Result<String, String> {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map_err(|e| format!("Could not get home directory: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = std::env::var("USERPROFILE")
        .unwrap_or_else(|_| ".".to_string()) + "\\AppData\\Local\\PrivacyAuditor\\findings.db";

    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }
    
    let db = Database::new(&db_path).expect("Failed to initialize database");
    
    tauri::Builder::default()
        .manage(AppState {
            db: Arc::new(StdMutex::new(db)),
        })
        .invoke_handler(tauri::generate_handler![
            scan_directories,
            get_findings,
            clear_findings,
            get_privacy_score,
            get_user_home,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
