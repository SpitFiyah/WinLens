# Privacy Debt Auditor - Technical Risks & Architecture Decisions

**Purpose:** Deep dive on high-risk areas and how to mitigate them  
**Audience:** Developers making implementation decisions  
**Read Time:** 10 minutes

---

## CRITICAL ARCHITECTURE DECISIONS

### Decision 1: Regex-based Secret Detection vs Machine Learning

**Context:**  
Application needs to identify API keys, passwords, tokens, and private keys in files.

**Options Evaluated:**

| Approach | Pros | Cons | Recommended |
|----------|------|------|-------------|
| **Regex + Entropy** | Fast, deterministic, works offline, <5MB | 5-10% false negatives, needs tuning | ✅ YES |
| **ML (local model)** | Higher accuracy, learns from data | 100MB+ model, requires GPU, overkill for MVP | ❌ Later |
| **Hybrid (regex + entropy)** | Best accuracy, still offline | More complexity, harder to tune | ⚠️ Phase 5 |

**Chosen:** Regex + Entropy scoring  
**Confidence Level:** 85%  
**Rationale:**
- MVP doesn't need 99.9% accuracy (90% is fine)
- Users expect offline operation (no cloud ML)
- Regex patterns are well-researched (OWASP, Trufflehogg)
- Entropy analysis adds confidence without model bloat
- Easy to tune based on user feedback post-MVP

**Implementation:**
```rust
// High-confidence patterns
let critical_patterns = vec![
  ("AWS_KEY", r"AKIA[0-9A-Z]{16}", 0.99),  // 99% confidence
  ("PRIVATE_KEY", r"-----BEGIN.*PRIVATE KEY", 1.0),  // 100% (literal match)
  ("JWT", r"eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.", 0.95),  // 95%
];

// Medium-confidence patterns (require entropy check)
let medium_patterns = vec![
  ("API_KEY_GENERIC", r"[a-z_]+_key\s*[=:]\s*['\"]?[A-Za-z0-9]{20,}['\"]?", 0.70),
];

// Entropy check: if string entropy > 4.0 and matches pattern → likely secret
fn is_likely_secret(matched_string: &str, pattern_confidence: f32) -> bool {
  let entropy = calculate_entropy(matched_string);
  entropy > 4.0 || pattern_confidence > 0.90
}
```

**Risk:** 5-10% false positives (acceptable for MVP)  
**Mitigation:** User can whitelist patterns by directory

---

### Decision 2: SQLite vs Binary/RocksDB vs In-Memory

**Context:**  
Need to store 10k-100k findings persistently, with queryable access.

**Options Evaluated:**

| Approach | Query Speed | Offline | Portable | Debuggable |
|----------|-------------|---------|----------|-----------|
| **SQLite** | <100ms | ✅ | ✅ | ✅✅✅ |
| **RocksDB** | <10ms | ✅ | ⚠️ | ⚠️ |
| **Binary format** | <1ms | ✅ | ⚠️ | ❌ |
| **In-Memory** | <1ms | ❌ | — | ✅ |

**Chosen:** SQLite  
**Confidence:** 95%  
**Rationale:**
- Can query findings (filter by severity, type, date)
- Portable (single file, can email, backup easily)
- Debuggable (open in any SQLite viewer)
- ACID guarantees (data safety)
- Tauri has built-in SQLite support
- Minimal dependencies (already in most systems)

**Schema Highlights:**
```sql
CREATE TABLE findings (
  id INTEGER PRIMARY KEY,
  scan_id TEXT NOT NULL,
  finding_type TEXT NOT NULL,  -- 'secret', 'browser', 'metadata'
  severity TEXT NOT NULL,      -- 'critical', 'high', 'medium', 'low'
  title TEXT NOT NULL,
  file_path TEXT,
  value_hash TEXT,             -- NOT actual secret (hashed)
  confidence REAL,             -- 0.0-1.0
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_findings_severity ON findings(severity);
CREATE INDEX idx_findings_type ON findings(finding_type);
CREATE INDEX idx_scans_created ON scans(created_at);
```

**Risk:** Query performance with 100k+ findings  
**Mitigation:** Indexes + pagination (load 100 at a time)

---

### Decision 3: Tauri vs Electron vs Native Windows App

**Context:**  
Need desktop app that runs on Windows 10/11, with native performance and small binary size.

**Options:**

| Framework | Binary Size | Memory | Performance | Learning Curve |
|-----------|-------------|--------|-------------|-----------------|
| **Tauri** | 15-30MB | 50MB | Excellent (native) | Steep (Rust) |
| **Electron** | 150-200MB | 200MB+ | Good (Chromium) | Easy (JS) |
| **Native (Win32)** | 5-10MB | 20MB | Excellent | Very steep |

**Chosen:** Tauri  
**Confidence:** 90%  
**Rationale:**
- Already decided in project brief
- Small binary (15-30MB vs 150MB for Electron)
- Native performance (uses OS rendering)
- Rust backend is performant
- React frontend for fast iteration

**Risk:** Learning curve (Tauri + Rust)  
**Mitigation:** Use existing examples, pair with experienced Rust dev

---

### Decision 4: Frontend Framework - React vs Vue vs Svelte

**Context:**  
Need to build dashboard with live updates, charts, and responsive design.

**Chosen:** React  
**Confidence:** 85%  
**Rationale:**
- Large ecosystem (Recharts, React-Window for performance)
- Most developers know React
- Good integration with Tauri via `@tauri-apps/api`
- TypeScript support

**Risk:** React might re-render unnecessarily during scan  
**Mitigation:**
```tsx
// Use React.memo and useMemo to prevent re-renders
const FindingsList = React.memo(({ findings, onSelect }) => {
  const memoized = useMemo(() => findings, [findings]);
  return <VirtualScroll items={memoized} />;
});
```

---

### Decision 5: D3.js vs Cytoscape.js vs Sigma.js for Graphs

**Context:**  
MVP doesn't need graph visualization, but Phase 4 will.

**Status:** DEFERRED (not in MVP)

**Options for Phase 4:**

| Library | Use Case | Learning Curve | Performance |
|---------|----------|-----------------|-------------|
| **D3.js** | Custom visualizations | Very steep | Excellent |
| **Cytoscape.js** | Network graphs | Steep | Good |
| **Sigma.js** | Large graphs (10k+ nodes) | Medium | Excellent |

**Recommendation for Phase 4:** Cytoscape.js  
**Rationale:**
- Better API than D3 for network graphs
- Good performance for 1000-5000 nodes
- Cleaner documentation
- More maintained than Sigma

**Risk:** Neither chosen yet (Phase 4 decision)

---

## MAJOR TECHNICAL RISKS

### Risk 1: Filesystem Scan Performance ⚠️ HIGH RISK

**What can go wrong:**
- Scanning 500k files takes 2+ minutes
- User thinks app is hung
- Parallel traversal causes memory spike (1GB+)

**Probability:** 40% (depends on Windows I/O patterns)  
**Impact:** MVP unusable without optimization

**Prevention Strategy (Do this on Day 4):**

1. **Benchmark immediately:**
   ```bash
   # Create test directory
   mkdir /test_data
   # Generate 50k files
   for i in {1..50000}; do touch /test_data/file_$i.txt; done
   
   # Benchmark scan
   time cargo run --release --example benchmark_scan
   # Target: <20 seconds
   ```

2. **If scan > 30 seconds:**
   ```rust
   // Add parallelism with Tokio
   let tasks: Vec<_> = subdirs
     .into_iter()
     .map(|subdir| tokio::spawn(async move {
       scan_directory_recursive(&subdir).await
     }))
     .collect();
   
   let results = futures::future::join_all(tasks).await;
   ```

3. **If still > 30 seconds:**
   - Check CPU profile (flamegraph): `cargo flamegraph --bin scanner`
   - Likely hotspots: MIME type detection, entropy calculation
   - Solution: Cache MIME types, sample larger files only

**Success Metric:** 50k files scanned in <20 seconds

---

### Risk 2: Browser Artifact Encryption ⚠️ HIGH RISK

**What can go wrong:**
- Chrome cookies encrypted (Edge may also be)
- Tool can't decrypt without user's Windows key
- Zero browser findings despite user having many

**Probability:** 60% (depends on Windows version)  
**Impact:** Browser analysis doesn't work

**Prevention Strategy (Do this on Day 2):**

1. **Test immediately on real Chrome profile:**
   ```rust
   // Attempt to read Chrome cookies
   let chrome_path = "C:\\Users\\{USER}\\AppData\\Local\\Google\\Chrome\\User Data\\Default";
   let cookies_db = format!("{}\\Cookies", chrome_path);
   
   // Try to open with rusqlite
   let conn = sqlite::Connection::open(&cookies_db)?;
   // If encrypted, error message will show
   ```

2. **If encrypted, three fallback options:**
   - Option A: Use `windows-rs` DPAPI to decrypt:
     ```rust
     use windows::Win32::Security::Cryptography::*;
     
     fn decrypt_chrome_cookie(encrypted_value: &[u8]) -> Result<String> {
       // Use CryptUnprotectData with local machine key
     }
     ```
   - Option B: Ask user to export cookies manually via browser
   - Option C: Skip browser analysis for now (graceful degradation)

3. **Workaround if decryption fails:**
   ```rust
   // Show error to user
   "Browser data requires manual export. Click here for instructions..."
   
   // Allow user to drag-drop exported CSV
   fn import_browser_data_csv(csv_path: &Path) -> Result<Vec<BrowserCookie>> {
     // Parse user-exported CSV
   }
   ```

**Success Metric:** Chrome cookies parsed successfully on both test machine and colleague's machine

---

### Risk 3: Rust Async/Await Complexity ⚠️ MEDIUM RISK

**What can go wrong:**
- Deadlocks in Tokio channels
- Memory leaks from background tasks
- Task cancellation doesn't work properly

**Probability:** 25% (with inexperienced Rust dev)  
**Impact:** Bugs that are hard to diagnose

**Prevention Strategy:**

1. **Use proven patterns (don't invent new ones):**
   ```rust
   // GOOD: standard Tokio pattern
   let (tx, rx) = tokio::sync::mpsc::channel(100);
   
   tokio::spawn(async move {
     for item in items {
       tx.send(item).await.ok();  // Ignore closed channel error
     }
   });
   
   // BAD: inventing custom concurrency
   let results = Arc::Mutex::new(Vec::new());  // Avoid this
   ```

2. **Code review rule:**
   - Every async function requires 2nd dev review
   - Look for: `.await` without error handling, shared state, task cancellation

3. **Test concurrency:**
   ```rust
   #[tokio::test]
   async fn test_concurrent_scans_no_deadlock() {
     let scan1 = tokio::spawn(scan_filesystem("C:\\Users\\Test1"));
     let scan2 = tokio::spawn(scan_filesystem("C:\\Users\\Test2"));
     
     let (r1, r2) = tokio::join!(scan1, scan2);
     assert!(r1.is_ok() && r2.is_ok());
   }
   ```

**Mitigation:** Pair with experienced Rust dev for first week

---

### Risk 4: Secret Detection False Positives/Negatives ⚠️ MEDIUM RISK

**What can go wrong:**
- Regex pattern matches test data ("test_api_key = 'abc123'")
- User reports 10 false positives, loses confidence in tool
- Real secrets missed because pattern too strict

**Probability:** 50% (regex patterns hard to tune)  
**Impact:** User trust, tool credibility

**Prevention Strategy (Do this on Day 4):**

1. **Test patterns against real data:**
   ```
   False positive test set:
   - README.md with code examples
   - Documentation with test credentials
   - Comments with deprecated keys
   - Configuration templates
   - Stack Overflow articles copied locally
   ```

2. **Build confidence matrix for each pattern:**
   ```
   Pattern: AWS_KEY
   Regex: AKIA[0-9A-Z]{16}
   
   Test Results:
   - Real AWS keys: 100 matches, 100 correct ✅ (100% precision)
   - False positives: 50 matches, 3 incorrect ❌ (94% precision)
   
   Recommendation: Keep pattern, mark as 94% confidence
   ```

3. **Add entropy filter:**
   ```rust
   // Don't report secrets with suspiciously low entropy
   fn filter_by_entropy(secret: &str) -> bool {
     let entropy = calculate_entropy(secret);
     entropy >= 3.5  // Real secrets have high entropy
   }
   ```

4. **Allow user to whitelist:**
   ```rust
   // User creates .privacy-debt-ignores file
   [patterns]
   skip_regex = ["test.*key", "example.*credential"]
   
   [paths]
   ignore_dirs = ["node_modules", ".git", "docs"]
   ```

**Success Metric:** <1% false positive rate on test dataset

---

### Risk 5: Database Query Performance Degrades ⚠️ MEDIUM RISK

**What can go wrong:**
- With 100k findings, filtering takes 2+ seconds
- UI lags when switching tabs
- Export takes 30+ seconds

**Probability:** 30% (depends on index strategy)  
**Impact:** Poor UX during analysis phase

**Prevention Strategy:**

1. **Index proactively:**
   ```sql
   CREATE INDEX idx_findings_severity ON findings(severity);
   CREATE INDEX idx_findings_type ON findings(finding_type);
   CREATE INDEX idx_findings_scan_id ON findings(scan_id);
   CREATE INDEX idx_findings_created ON findings(created_at DESC);
   ```

2. **Benchmark queries on Day 8:**
   ```rust
   #[bench]
   fn bench_query_findings_by_severity(b: &mut Bencher) {
     let db = setup_db_with_100k_findings();
     b.iter(|| {
       db.query_findings_by_severity("critical")
     });
   }
   // Target: <100ms per query
   ```

3. **Implement pagination:**
   ```rust
   async fn get_findings(
     filter: FindingsFilter,
     page: u32,
     per_page: u32,
   ) -> Result<Page<Finding>> {
     let offset = (page - 1) * per_page;
     let findings = db
       .query()
       .filter(filter)
       .offset(offset)
       .limit(per_page)
       .execute();
   }
   ```

4. **If still slow, profile first:**
   ```bash
   # Run with profiler
   cargo flamegraph --bin query_benchmark
   # Look for: slow I/O, missing indexes, N+1 queries
   ```

**Success Metric:** Query 100k findings by severity in <100ms

---

### Risk 6: Metadata Parsing Complexity (JPEG EXIF, PDF) ⚠️ MEDIUM RISK

**What can go wrong:**
- EXIF parsing crashes on malformed images
- PDF metadata extraction takes too long
- Dependencies bloat binary size

**Status:** DEFERRED to Phase 5 (not in MVP)  
**Why:** Lower priority, adds complexity

**Prevention Strategy (for Phase 5):**
- Use vetted libraries (kamadak-exif, pdf for Rust)
- Implement error recovery (skip malformed files)
- Benchmark: ensure doesn't slow down scan

---

## PERFORMANCE BASELINE (Establish Day 7)

Create a `BENCHMARKS.md` file tracking these metrics:

```markdown
# Privacy Debt Auditor - Performance Baselines

## Scanning Performance
- 50k files / 2GB: _____ seconds (Target: <20s)
- 100k files / 5GB: _____ seconds (Target: <40s)
- Memory peak: _____ MB (Target: <500MB)

## Detection Performance
- Regex pattern matching: _____ files/sec (Target: >5000/s)
- Entropy calculation: _____ strings/sec (Target: >10000/s)
- Browser parsing: _____ seconds (Target: <2s)

## Database Performance
- Insert 10k findings: _____ seconds (Target: <5s)
- Query by severity: _____ ms (Target: <100ms)
- Full export: _____ seconds (Target: <5s)

## UI Performance
- Dashboard load: _____ ms (Target: <500ms)
- Findings list render (1k items): _____ ms (Target: <1000ms)
- Filter/sort: _____ ms (Target: <300ms)
```

---

## SECURITY CONSIDERATIONS

### Secret Values Must Never Leak

**Rule 1: Never store actual secret values**
```rust
// ❌ WRONG
let finding = Finding {
  secret_type: "AWS_KEY",
  actual_value: "AKIA1234567890ABCDEF",  // ← NEVER STORE
};

// ✅ CORRECT
let finding = Finding {
  secret_type: "AWS_KEY",
  value_hash: hash_sha256("AKIA1234567890ABCDEF"),
  value_length: 20,
  snippet: "AKIA...BCDEF",  // First and last chars only
};
```

**Rule 2: Show secrets in UI only briefly**
```tsx
// ✅ CORRECT: Show hash by default, reveal on click
const SecretValue = ({ valueHash }) => {
  const [revealed, setRevealed] = useState(false);
  
  return (
    <div>
      {revealed ? <span>{valueHash}</span> : <span>••••••••</span>}
      <button onClick={() => setRevealed(true)}>Reveal</button>
    </div>
  );
};
```

**Rule 3: Export must contain hashes only**
```json
{
  "findings": [
    {
      "type": "api_key",
      "severity": "critical",
      "file": "C:\\Users\\John\\config.env",
      "value_hash": "sha256:a1b2c3d4...",
      "value_preview": "AKIA...BCDEF"
    }
  ]
}
```

### Remediation Checkpoints

- [ ] Day 2: Security review of secret handling design
- [ ] Day 5: Code review before first secret detection test
- [ ] Day 8: Verify export contains NO actual values (automated test)
- [ ] Day 9: Final security audit (another dev)

---

## SUMMARY: RISK vs MITIGATION

| Risk | Probability | Impact | Mitigation | Timeline |
|------|-------------|--------|-----------|----------|
| Scan too slow | 40% | High | Benchmark Day 4, parallelize | Day 4-5 |
| Browser auth fails | 60% | High | Test Day 2, plan fallbacks | Day 2-3 |
| Rust learning curve | 40% | Medium | Pair programming, examples | Day 1-2 |
| False positives spam | 50% | Medium | Test patterns thoroughly | Day 4-5 |
| DB performance | 30% | Medium | Index early, benchmark Day 8 | Day 8 |
| Metadata parsing | 20% | Low | Skip MVP, defer to Phase 5 | N/A |

**Red flags to watch:**
- Scan taking >30s on Day 4 → escalate immediately
- Browser parsing fails on real profile Day 2 → escalate immediately
- Any false positives >1% → iterate patterns

---

**End of Technical Risks Document**

Next: Start with QUICK_START.md for day-by-day execution plan.
