import { useEffect, useMemo, useState } from "react";
import type { CSSProperties } from "react";
import {
  Area,
  AreaChart,
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  Legend,
  Line,
  LineChart,
  Pie,
  PieChart,
  PolarAngleAxis,
  PolarGrid,
  PolarRadiusAxis,
  Radar,
  RadarChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import {
  AlertTriangle,
  BarChart3,
  Cookie,
  Database,
  FileCode2,
  Fingerprint,
  Gauge,
  HardDrive,
  KeyRound,
  LayoutDashboard,
  LineChart as LineChartIcon,
  ListChecks,
  Radar as RadarIcon,
  ShieldAlert,
  ShieldCheck,
  Sparkles,
} from "lucide-react";
import { invokeIfAvailable, isTauriRuntime } from "../tauriBridge";
import "../styles/Dashboard.css";

interface DashboardProps {
  score: number;
  findingsCount: number;
}

type ViewMode = "overview" | "detailed" | "trends";
type Severity = "Critical" | "High" | "Medium" | "Low";

interface FindingMetadata {
  source_application?: string | null;
  file_size?: number | null;
  last_modified?: string | null;
  additional_info?: Record<string, string>;
}

interface Finding {
  id: string;
  category: string | { other: string };
  severity: string;
  title: string;
  description: string;
  location: string;
  remediation: string;
  metadata: FindingMetadata;
  discovered_at: string;
}

interface ScoreFactors {
  exposed_secrets: number;
  tracking_cookies: number;
  cached_identifiers: number;
  browser_persistence: number;
  metadata_leakage: number;
  deleted_artifacts: number;
  stale_sessions: number;
  risky_storage: number;
}

interface PrivacyDebtScore {
  total_score: number;
  factors: ScoreFactors;
  findings_count: number;
}

const severityColors: Record<Severity, string> = {
  Critical: "#ff5d73",
  High: "#ff9b54",
  Medium: "#f2c94c",
  Low: "#4fd18b",
};

const severityWeights: Record<Severity, number> = {
  Critical: 10,
  High: 6,
  Medium: 3,
  Low: 1,
};

const viewModes: Array<{
  id: ViewMode;
  label: string;
  icon: typeof LayoutDashboard;
}> = [
  { id: "overview", label: "Overview", icon: LayoutDashboard },
  { id: "detailed", label: "Detailed", icon: ListChecks },
  { id: "trends", label: "Trends", icon: LineChartIcon },
];

const factorLabels: Array<{
  key: keyof ScoreFactors;
  label: string;
  icon: typeof KeyRound;
}> = [
  { key: "exposed_secrets", label: "Exposed Secrets", icon: KeyRound },
  { key: "tracking_cookies", label: "Tracking Cookies", icon: Cookie },
  { key: "cached_identifiers", label: "Cached IDs", icon: Fingerprint },
  { key: "browser_persistence", label: "Browser Persistence", icon: Database },
  { key: "metadata_leakage", label: "Metadata Leaks", icon: FileCode2 },
  { key: "deleted_artifacts", label: "Deleted Artifacts", icon: HardDrive },
  { key: "stale_sessions", label: "Stale Sessions", icon: ShieldAlert },
  { key: "risky_storage", label: "Risky Storage", icon: HardDrive },
];

function getScoreMeta(score: number) {
  if (score >= 80) {
    return { color: severityColors.Critical, label: "Critical Risk", Icon: ShieldAlert };
  }

  if (score >= 60) {
    return { color: severityColors.High, label: "High Risk", Icon: AlertTriangle };
  }

  if (score >= 40) {
    return { color: severityColors.Medium, label: "Medium Risk", Icon: Gauge };
  }

  return { color: severityColors.Low, label: "Low Risk", Icon: ShieldCheck };
}

function normalizeSeverity(value: string): Severity {
  const normalized = value.toLowerCase();
  if (normalized === "critical") return "Critical";
  if (normalized === "high") return "High";
  if (normalized === "low") return "Low";
  return "Medium";
}

function categoryLabel(category: Finding["category"]) {
  const raw = typeof category === "string" ? category : category.other;
  return raw
    .replace(/_/g, " ")
    .replace(/\b\w/g, (letter) => letter.toUpperCase());
}

function inferApplication(finding: Finding) {
  if (finding.metadata.source_application) {
    return finding.metadata.source_application;
  }

  const location = finding.location.toLowerCase();
  if (location.includes("chrome")) return "Chrome";
  if (location.includes("edge")) return "Edge";
  if (location.includes("firefox")) return "Firefox";
  if (location.includes("appdata")) return "AppData";
  if (location.includes("windows") || location.includes("registry")) return "Windows";
  if (location.includes("download")) return "Downloads";
  if (location.includes("document")) return "Documents";
  if (location.includes("desktop")) return "Desktop";
  return "Filesystem";
}

function recommendationFor(finding: Finding) {
  if (finding.remediation) return finding.remediation;

  const category = categoryLabel(finding.category).toLowerCase();
  if (category.includes("secret")) return "Rotate the exposed credential, then remove it from this location.";
  if (category.includes("cookie")) return "Clear browser cookies and review site session persistence.";
  if (category.includes("metadata")) return "Strip metadata before sharing this file externally.";
  if (category.includes("deleted")) return "Purge stale cache remnants if they are no longer needed.";
  return "Review this artifact and remove or relocate it if it is no longer required.";
}

function ChartTitle({
  icon: Icon,
  title,
  detail,
}: {
  icon: typeof BarChart3;
  title: string;
  detail?: string;
}) {
  return (
    <div className="chart-title">
      <span className="chart-title-icon">
        <Icon size={18} />
      </span>
      <div>
        <h3>{title}</h3>
        {detail && <p>{detail}</p>}
      </div>
    </div>
  );
}

function EmptyState({ message }: { message: string }) {
  return (
    <div className="empty-state">
      <Database size={22} />
      <span>{message}</span>
    </div>
  );
}

export default function Dashboard({ score, findingsCount }: DashboardProps) {
  const [viewMode, setViewMode] = useState<ViewMode>("overview");
  const [findings, setFindings] = useState<Finding[]>([]);
  const [privacyScore, setPrivacyScore] = useState<PrivacyDebtScore | null>(null);
  const [loadError, setLoadError] = useState("");
  const [hasLoaded, setHasLoaded] = useState(false);

  useEffect(() => {
    let cancelled = false;

    async function loadAuditData() {
      if (!isTauriRuntime()) {
        setHasLoaded(true);
        return;
      }

      try {
        const [loadedFindings, loadedScore] = await Promise.all([
          invokeIfAvailable<Finding[]>("get_findings"),
          invokeIfAvailable<PrivacyDebtScore>("get_privacy_score"),
        ]);

        if (!cancelled) {
          setFindings(loadedFindings);
          setPrivacyScore(loadedScore);
          setLoadError("");
          setHasLoaded(true);
        }
      } catch (error) {
        if (!cancelled) {
          setLoadError(String(error));
          setHasLoaded(true);
        }
      }
    }

    loadAuditData();

    return () => {
      cancelled = true;
    };
  }, [score, findingsCount]);

  const effectiveScore = privacyScore?.total_score ?? score;
  const effectiveCount = privacyScore?.findings_count ?? findings.length;
  const factors = privacyScore?.factors;
  const scoreMeta = getScoreMeta(effectiveScore);
  const ScoreIcon = scoreMeta.Icon;

  const severityData = useMemo(() => {
    const counts: Record<Severity, number> = {
      Critical: 0,
      High: 0,
      Medium: 0,
      Low: 0,
    };

    findings.forEach((finding) => {
      counts[normalizeSeverity(finding.severity)] += 1;
    });

    return (Object.keys(counts) as Severity[])
      .map((name) => ({ name, value: counts[name], color: severityColors[name] }))
      .filter((item) => item.value > 0);
  }, [findings]);

  const categoryData = useMemo(() => {
    const counts = new Map<string, number>();
    findings.forEach((finding) => {
      const category = categoryLabel(finding.category);
      counts.set(category, (counts.get(category) ?? 0) + 1);
    });

    return Array.from(counts, ([name, count]) => ({ name, count })).sort(
      (a, b) => b.count - a.count,
    );
  }, [findings]);

  const radarData = useMemo(() => {
    const riskByCategory = new Map<string, number>();
    findings.forEach((finding) => {
      const category = categoryLabel(finding.category);
      const severity = normalizeSeverity(finding.severity);
      riskByCategory.set(category, (riskByCategory.get(category) ?? 0) + severityWeights[severity]);
    });

    const maxRisk = Math.max(1, ...riskByCategory.values());
    return Array.from(riskByCategory, ([category, risk]) => ({
      category,
      value: Math.round((risk / maxRisk) * 100),
    })).slice(0, 8);
  }, [findings]);

  const applicationRiskData = useMemo(() => {
    const byApp = new Map<string, { app: string; risk: number; findings: number }>();
    findings.forEach((finding) => {
      const app = inferApplication(finding);
      const severity = normalizeSeverity(finding.severity);
      const current = byApp.get(app) ?? { app, risk: 0, findings: 0 };
      current.risk += severityWeights[severity];
      current.findings += 1;
      byApp.set(app, current);
    });

    return Array.from(byApp.values())
      .sort((a, b) => b.risk - a.risk)
      .slice(0, 8);
  }, [findings]);

  const timelineData = useMemo(() => {
    const byDate = new Map<string, { date: string; findings: number; risk: number }>();
    findings.forEach((finding) => {
      const discovered = new Date(finding.discovered_at);
      const date = Number.isNaN(discovered.getTime())
        ? "Unknown"
        : discovered.toLocaleDateString(undefined, { month: "short", day: "numeric" });
      const severity = normalizeSeverity(finding.severity);
      const current = byDate.get(date) ?? { date, findings: 0, risk: 0 };
      current.findings += 1;
      current.risk += severityWeights[severity];
      byDate.set(date, current);
    });

    return Array.from(byDate.values()).slice(-10);
  }, [findings]);

  const scoreFactors = useMemo(() => {
    if (!factors) return [];
    const maxValue = Math.max(1, ...factorLabels.map((factor) => factors[factor.key]));

    return factorLabels
      .map((factor) => ({
        ...factor,
        value: factors[factor.key],
        percent: Math.round((factors[factor.key] / maxValue) * 100),
      }))
      .filter((factor) => factor.value > 0);
  }, [factors]);

  const topRecommendations = useMemo(() => {
    return [...findings]
      .sort(
        (a, b) =>
          severityWeights[normalizeSeverity(b.severity)] -
          severityWeights[normalizeSeverity(a.severity)],
      )
      .slice(0, 3)
      .map((finding) => ({
        id: finding.id,
        severity: normalizeSeverity(finding.severity).toLowerCase(),
        title: finding.title,
        body: recommendationFor(finding),
        Icon: categoryLabel(finding.category).toLowerCase().includes("cookie")
          ? Cookie
          : categoryLabel(finding.category).toLowerCase().includes("metadata")
            ? FileCode2
            : KeyRound,
      }));
  }, [findings]);

  const recentFindings = findings.slice(0, 8);
  const hasFindings = findings.length > 0;

  return (
    <div className="dashboard">
      <div className="view-mode-selector" aria-label="Dashboard view">
        {viewModes.map((mode) => {
          const Icon = mode.icon;
          return (
            <button
              key={mode.id}
              className={viewMode === mode.id ? "active" : ""}
              onClick={() => setViewMode(mode.id)}
              type="button"
            >
              <Icon size={17} />
              {mode.label}
            </button>
          );
        })}
      </div>

      {loadError && (
        <div className="dashboard-warning" role="alert">
          <AlertTriangle size={18} />
          {loadError}
        </div>
      )}

      <div className="score-section">
        <div className="score-card">
          <div className="score-card-header">
            <span className="card-icon">
              <Gauge size={20} />
            </span>
            <h2>Privacy Debt Score</h2>
          </div>
          <div
            className="score-circle"
            style={
              {
                "--score-color": scoreMeta.color,
                "--score": effectiveScore,
              } as CSSProperties
            }
          >
            <span className="score-value">{effectiveScore}</span>
            <span className="score-max">/100</span>
          </div>
          <p className="score-label" style={{ color: scoreMeta.color }}>
            <ScoreIcon size={18} />
            {scoreMeta.label}
          </p>
        </div>

        <div className="factors-card">
          <ChartTitle
            icon={ShieldAlert}
            title="Risk Factors"
            detail="Calculated from actual findings returned by the scanner"
          />
          {scoreFactors.length > 0 ? (
            <div className="factors-grid">
              {scoreFactors.map((factor) => {
                const Icon = factor.icon;
                return (
                  <div key={factor.key} className="factor-item">
                    <div className="factor-label">
                      <Icon size={16} />
                      <span>{factor.label}</span>
                    </div>
                    <div className="factor-meter">
                      <div
                        className="factor-fill"
                        style={{ width: `${factor.percent}%` }}
                      />
                    </div>
                    <div className="factor-value">{factor.value} risk points</div>
                  </div>
                );
              })}
            </div>
          ) : (
            <EmptyState message={hasLoaded ? "No score factors available yet. Run a scan to populate this panel." : "Loading audit data..."} />
          )}
        </div>
      </div>

      {viewMode === "overview" && (
        <div className="charts-section">
          <div className="chart-card">
            <ChartTitle
              icon={RadarIcon}
              title="Severity Distribution"
              detail="Actual findings grouped by urgency"
            />
            {severityData.length > 0 ? (
              <ResponsiveContainer width="100%" height={300}>
                <PieChart>
                  <Pie
                    data={severityData}
                    cx="50%"
                    cy="50%"
                    innerRadius={58}
                    outerRadius={98}
                    paddingAngle={3}
                    dataKey="value"
                  >
                    {severityData.map((entry) => (
                      <Cell key={entry.name} fill={entry.color} />
                    ))}
                  </Pie>
                  <Tooltip />
                  <Legend iconType="circle" />
                </PieChart>
              </ResponsiveContainer>
            ) : (
              <EmptyState message="No findings have been recorded." />
            )}
          </div>

          <div className="chart-card">
            <ChartTitle
              icon={BarChart3}
              title="Findings by Category"
              detail="Categories reported by the backend detectors"
            />
            {categoryData.length > 0 ? (
              <ResponsiveContainer width="100%" height={300}>
                <BarChart
                  data={categoryData}
                  margin={{ top: 20, right: 20, left: -15, bottom: 0 }}
                >
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="name" />
                  <YAxis />
                  <Tooltip />
                  <Bar dataKey="count" fill="#6aa8ff" radius={[6, 6, 0, 0]} />
                </BarChart>
              </ResponsiveContainer>
            ) : (
              <EmptyState message="No categories to display yet." />
            )}
          </div>

          <div className="chart-card">
            <ChartTitle
              icon={Sparkles}
              title="Risk Profile"
              detail="Weighted severity by finding category"
            />
            {radarData.length > 0 ? (
              <ResponsiveContainer width="100%" height={300}>
                <RadarChart data={radarData}>
                  <PolarGrid stroke="rgba(255,255,255,0.16)" />
                  <PolarAngleAxis dataKey="category" />
                  <PolarRadiusAxis />
                  <Radar
                    name="Risk Level"
                    dataKey="value"
                    stroke="#ff9b54"
                    fill="#ff9b54"
                    fillOpacity={0.45}
                  />
                  <Tooltip />
                </RadarChart>
              </ResponsiveContainer>
            ) : (
              <EmptyState message="Risk profile will appear after findings are detected." />
            )}
          </div>

          <div className="chart-card">
            <ChartTitle
              icon={HardDrive}
              title="Application Risk"
              detail="Inferred from source application and finding paths"
            />
            {applicationRiskData.length > 0 ? (
              <ResponsiveContainer width="100%" height={300}>
                <BarChart
                  data={applicationRiskData}
                  margin={{ top: 20, right: 20, left: -15, bottom: 0 }}
                >
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="app" />
                  <YAxis />
                  <Tooltip />
                  <Legend />
                  <Bar dataKey="risk" fill="#ff9b54" name="Risk Points" radius={[6, 6, 0, 0]} />
                  <Bar dataKey="findings" fill="#7c6dff" name="Findings" radius={[6, 6, 0, 0]} />
                </BarChart>
              </ResponsiveContainer>
            ) : (
              <EmptyState message="Application risk needs real scan findings." />
            )}
          </div>
        </div>
      )}

      {viewMode === "detailed" && (
        <div className="charts-section">
          <div className="chart-card wide">
            <ChartTitle
              icon={BarChart3}
              title="Severity Distribution"
              detail="Total records by severity tier"
            />
            {severityData.length > 0 ? (
              <ResponsiveContainer width="100%" height={350}>
                <BarChart data={severityData}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="name" />
                  <YAxis />
                  <Tooltip />
                  <Bar dataKey="value" radius={[6, 6, 0, 0]}>
                    {severityData.map((entry) => (
                      <Cell key={entry.name} fill={entry.color} />
                    ))}
                  </Bar>
                </BarChart>
              </ResponsiveContainer>
            ) : (
              <EmptyState message="Run a scan to see severity totals." />
            )}
          </div>

          <div className="chart-card wide">
            <ChartTitle
              icon={Database}
              title="Category Details"
              detail="Finding volume by source category"
            />
            {categoryData.length > 0 ? (
              <ResponsiveContainer width="100%" height={350}>
                <BarChart
                  data={categoryData}
                  layout="vertical"
                  margin={{ top: 5, right: 30, left: 120, bottom: 5 }}
                >
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis type="number" />
                  <YAxis dataKey="name" type="category" width={110} />
                  <Tooltip />
                  <Bar dataKey="count" fill="#4fd18b" radius={[0, 6, 6, 0]} />
                </BarChart>
              </ResponsiveContainer>
            ) : (
              <EmptyState message="No category details are available yet." />
            )}
          </div>
        </div>
      )}

      {viewMode === "trends" && (
        <div className="charts-section">
          <div className="chart-card wide">
            <ChartTitle
              icon={LineChartIcon}
              title="Risk Timeline"
              detail="Findings grouped by discovery date"
            />
            {timelineData.length > 0 ? (
              <ResponsiveContainer width="100%" height={300}>
                <AreaChart
                  data={timelineData}
                  margin={{ top: 10, right: 30, left: 0, bottom: 0 }}
                >
                  <defs>
                    <linearGradient id="colorRisk" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="5%" stopColor="#ff9b54" stopOpacity={0.75} />
                      <stop offset="95%" stopColor="#ff9b54" stopOpacity={0.02} />
                    </linearGradient>
                  </defs>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="date" />
                  <YAxis />
                  <Tooltip />
                  <Area
                    type="monotone"
                    dataKey="risk"
                    stroke="#ff9b54"
                    fill="url(#colorRisk)"
                    strokeWidth={2}
                    name="Risk Points"
                  />
                </AreaChart>
              </ResponsiveContainer>
            ) : (
              <EmptyState message="Timeline will populate from real discovery timestamps." />
            )}
          </div>

          <div className="chart-card wide">
            <ChartTitle
              icon={BarChart3}
              title="Findings Over Time"
              detail="Actual findings by discovery date"
            />
            {timelineData.length > 0 ? (
              <ResponsiveContainer width="100%" height={300}>
                <LineChart
                  data={timelineData}
                  margin={{ top: 10, right: 30, left: 0, bottom: 0 }}
                >
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="date" />
                  <YAxis />
                  <Tooltip />
                  <Legend />
                  <Line
                    type="monotone"
                    dataKey="findings"
                    stroke="#7c6dff"
                    name="Findings"
                    strokeWidth={3}
                    dot={{ r: 4 }}
                  />
                </LineChart>
              </ResponsiveContainer>
            ) : (
              <EmptyState message="No dated findings are available yet." />
            )}
          </div>
        </div>
      )}

      <div className="findings-section">
        <ChartTitle
          icon={ShieldAlert}
          title={`Recent Findings (${effectiveCount})`}
          detail="Latest privacy exposures detected on this device"
        />
        {hasFindings ? (
          <div className="findings-list">
            {recentFindings.map((finding) => {
              const severity = normalizeSeverity(finding.severity);
              return (
                <div
                  key={finding.id}
                  className={`finding-item severity-${severity.toLowerCase()}`}
                >
                  <div className="finding-left">
                    <span className="finding-icon">
                      <ShieldAlert size={18} />
                    </span>
                    <div>
                      <div className="finding-title">{finding.title}</div>
                      <div className="finding-location">{finding.location}</div>
                    </div>
                  </div>
                  <div className={`finding-badge severity-${severity.toLowerCase()}`}>
                    {severity}
                  </div>
                </div>
              );
            })}
          </div>
        ) : (
          <EmptyState message="No findings are stored yet. Run a desktop scan to populate this list." />
        )}
      </div>

      <div className="recommendations-section">
        <ChartTitle
          icon={ShieldCheck}
          title="Top Recommendations"
          detail="Generated from the highest-severity actual findings"
        />
        {topRecommendations.length > 0 ? (
          <div className="recommendations-list">
            {topRecommendations.map((item) => {
              const Icon = item.Icon;
              return (
                <div key={item.id} className={`recommendation ${item.severity}`}>
                  <span className="priority">
                    <Icon size={20} />
                  </span>
                  <div>
                    <strong>{item.title}</strong>
                    <p>{item.body}</p>
                  </div>
                </div>
              );
            })}
          </div>
        ) : (
          <EmptyState message="Recommendations will be generated from real findings after a scan." />
        )}
      </div>
    </div>
  );
}
