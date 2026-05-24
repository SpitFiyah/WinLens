import { useState, useEffect } from "react";
import {
  AlertCircle,
  Database,
  Download,
  FileText,
  FolderOpen,
  Globe,
  Monitor,
  Play,
  ScanLine,
  Settings2,
} from "lucide-react";
import {
  invokeIfAvailable,
  isTauriRuntime,
  listenIfAvailable,
} from "../tauriBridge";
import "../styles/Scanner.css";

interface ScannerProps {
  onScanStart: () => void;
  onScanComplete: (score: number, findings: number) => void;
}

interface ScanProgressEvent {
  files_scanned: number;
  current_file?: string;
  progress_percent: number;
}

interface ScanCompleteEvent {
  score: { total_score: number };
  findings: Array<{ id: string }>;
  total_findings: number;
  scan_duration_secs: number;
}

export default function Scanner({ onScanStart, onScanComplete }: ScannerProps) {
  const [isScanning, setIsScanning] = useState(false);
  const [progress, setProgress] = useState(0);
  const [currentFile, setCurrentFile] = useState("");
  const [filesScanned, setFilesScanned] = useState(0);
  const [errorMessage, setErrorMessage] = useState("");
  const [userHome, setUserHome] = useState<string>("");
  const [selectedPaths, setSelectedPaths] = useState({
    downloads: true,
    desktop: true,
    documents: true,
    appdata: true,
    browser: true,
    registry: false,
  });

  useEffect(() => {
    // Get user home directory on mount
    const getHome = async () => {
      if (!isTauriRuntime()) {
        setUserHome("C:\\Users\\User");
        return;
      }

      try {
        const home = await invokeIfAvailable<string>("get_user_home");
        setUserHome(home);
      } catch (error) {
        console.error("Failed to get user home:", error);
        setUserHome("C:\\Users\\User");
      }
    };

    getHome();

    // Set up event listeners for progress and completion
    const setupListeners = async () => {
      const unlistenProgress = await listenIfAvailable<ScanProgressEvent>("scan:progress", (event) => {
        setProgress(event.payload.progress_percent);
        setFilesScanned(event.payload.files_scanned);
        setCurrentFile(event.payload.current_file ?? "Scanning selected locations...");
      });

      const unlistenComplete = await listenIfAvailable<ScanCompleteEvent>("scan:complete", (event) => {
        setProgress(100);
        setIsScanning(false);
        
        // Extract score and findings count
        const score = Math.round(event.payload.score.total_score);
        const findings = event.payload.total_findings;
        
        console.log(
          `Scan complete: ${findings} findings in ${event.payload.scan_duration_secs.toFixed(2)}s`
        );
        
        onScanComplete(score, findings);
      });

      return () => {
        unlistenProgress();
        unlistenComplete();
      };
    };

    // IMPORTANT: Must await to ensure listeners are registered BEFORE scan starts
    setupListeners().catch(console.error);
  }, [onScanComplete]);

  const handleStartScan = async () => {
    setIsScanning(true);
    onScanStart();
    setProgress(0);
    setFilesScanned(0);
    setErrorMessage("");
    setCurrentFile("Initializing scan...");

    try {
      if (!isTauriRuntime()) {
        setIsScanning(false);
        setProgress(0);
        setErrorMessage(
          "Real system scanning is only available in the Tauri desktop app. Launch with npm run tauri dev.",
        );
        return;
      }

      // Build paths from selected options
      const paths: string[] = [];
      
      if (selectedPaths.downloads) paths.push(`${userHome}\\Downloads`);
      if (selectedPaths.desktop) paths.push(`${userHome}\\Desktop`);
      if (selectedPaths.documents) paths.push(`${userHome}\\Documents`);
      if (selectedPaths.appdata) paths.push(`${userHome}\\AppData\\Local`);
      if (selectedPaths.browser) paths.push(`${userHome}\\AppData\\Local\\Google\\Chrome`);

      console.log("Starting scan with paths:", paths);

      // Call Tauri backend
      const result = await invokeIfAvailable<ScanCompleteEvent>("scan_directories", {
        request: {
          paths,
          include_browser: selectedPaths.browser,
          include_registry: selectedPaths.registry,
        },
      });

      console.log("Scan result:", result);
    } catch (error) {
      console.error("Scan failed:", error);
      setIsScanning(false);
      setProgress(0);
      setErrorMessage(String(error));
    }
  };

  const togglePath = (key: keyof typeof selectedPaths) => {
    setSelectedPaths((prev) => ({
      ...prev,
      [key]: !prev[key],
    }));
  };

  return (
    <div className="scanner">
      <div className="scanner-card">
        <div className="scanner-heading">
          <span className="scanner-heading-icon">
            <ScanLine size={22} />
          </span>
          <div>
            <h2>System Privacy Scan</h2>
            <p className="description">
              Scan your system for exposed secrets, tracking cookies, metadata
              leaks, and privacy residue.
            </p>
          </div>
        </div>

        {!isScanning ? (
          <>
            {errorMessage && (
              <div className="scan-error" role="alert">
                <AlertCircle size={18} />
                {errorMessage}
              </div>
            )}
            <button className="scan-button" onClick={handleStartScan}>
              <Play size={18} />
              Start Scan
            </button>
          </>
        ) : (
          <div className="scan-progress">
            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{ width: `${progress}%` }}
              ></div>
            </div>
            <div className="progress-stats">
              <span>{Math.round(progress)}%</span>
              <span>{filesScanned} files</span>
            </div>
            <p className="current-file">{currentFile}</p>
          </div>
        )}
      </div>

      <div className="scan-options">
        <div className="scan-options-heading">
          <Settings2 size={18} />
          <h3>Scan Locations</h3>
        </div>
        <div className="checkbox-group">
          <label>
            <input
              type="checkbox"
              checked={selectedPaths.downloads}
              onChange={() => togglePath("downloads")}
            />
            <Download size={17} />
            Downloads
          </label>
          <label>
            <input
              type="checkbox"
              checked={selectedPaths.desktop}
              onChange={() => togglePath("desktop")}
            />
            <Monitor size={17} />
            Desktop
          </label>
          <label>
            <input
              type="checkbox"
              checked={selectedPaths.documents}
              onChange={() => togglePath("documents")}
            />
            <FileText size={17} />
            Documents
          </label>
          <label>
            <input
              type="checkbox"
              checked={selectedPaths.appdata}
              onChange={() => togglePath("appdata")}
            />
            <FolderOpen size={17} />
            App Data
          </label>
          <label>
            <input
              type="checkbox"
              checked={selectedPaths.browser}
              onChange={() => togglePath("browser")}
            />
            <Globe size={17} />
            Browser Data
          </label>
          <label>
            <input
              type="checkbox"
              checked={selectedPaths.registry}
              onChange={() => togglePath("registry")}
              disabled
            />
            <Database size={17} />
            Windows Registry (Coming Soon)
          </label>
        </div>
      </div>
    </div>
  );
}
