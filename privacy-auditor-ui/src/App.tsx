import { useEffect, useState } from "react";
import { Activity, LayoutDashboard, ScanLine, Shield } from "lucide-react";
import Dashboard from "./components/Dashboard";
import Scanner from "./components/Scanner";

export default function App() {
  const [currentView, setCurrentView] = useState<"dashboard" | "scanner">(
    "scanner"
  );
  const [privacyScore, setPrivacyScore] = useState<number>(0);
  const [findingsCount, setFindingsCount] = useState<number>(0);
  const [, setIsScanning] = useState<boolean>(false);

  useEffect(() => {
    // TODO: Initialize app state
  }, []);

  return (
    <div className="app">
      <header className="app-header">
        <div className="header-left">
          <div className="brand-mark">
            <Shield size={22} />
          </div>
          <div>
            <h1>Privacy Debt Auditor</h1>
            <p className="subtitle">
              Your machine remembers more about you than you realize.
            </p>
          </div>
        </div>
        <div className="header-stats">
          <div className="stat-box">
            <div className="stat-label">Privacy Score</div>
            <div className="stat-value">{privacyScore}/100</div>
          </div>
          <div className="stat-box">
            <div className="stat-label">Findings</div>
            <div className="stat-value">{findingsCount}</div>
          </div>
        </div>
      </header>

      <nav className="app-nav">
        <button
          className={`nav-btn ${currentView === "scanner" ? "active" : ""}`}
          onClick={() => setCurrentView("scanner")}
        >
          <ScanLine size={17} />
          Scanner
        </button>
        <button
          className={`nav-btn ${currentView === "dashboard" ? "active" : ""}`}
          onClick={() => setCurrentView("dashboard")}
        >
          <LayoutDashboard size={17} />
          Dashboard
        </button>
      </nav>

      <main className="app-main">
        {currentView === "scanner" ? (
          <Scanner
            onScanStart={() => setIsScanning(true)}
            onScanComplete={(score, findings) => {
              setIsScanning(false);
              setPrivacyScore(score);
              setFindingsCount(findings);
              setCurrentView("dashboard");
            }}
          />
        ) : (
          <Dashboard score={privacyScore} findingsCount={findingsCount} />
        )}
      </main>

      <footer className="app-footer">
        <Activity size={15} />
        <p>All analysis happens locally. Zero cloud. Everything stays on your machine.</p>
      </footer>
    </div>
  );
}
