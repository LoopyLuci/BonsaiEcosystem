# Automated Bug Hunter Execution Plan

**Purpose:** Execute comprehensive Bug Hunter scans, fix issues, and train the system  
**Status:** Ready to Execute Once Build Completes  
**Estimated Time:** 20-30 minutes for complete scan & training

---

## Phase 1: Build & Startup (Automatic Once Build Completes)

### Step 1.1: Verify Build Success
```powershell
ls Z:\Projects\BonsaiWorkspace\target\release\bonsai-mcp-server.exe
# Should show the binary exists
```

### Step 1.2: Start MCP Server
```powershell
cd Z:\Projects\BonsaiWorkspace
.\target\release\bonsai-mcp-server.exe
```

**Expected Output:**
```
Starting MCP server on http://127.0.0.1:3000
Registering tools...
✓ bonsai_scan_repo
✓ bonsai_list_findings
✓ bonsai_get_finding
✓ bonsai_auto_fix
✓ bonsai_explain_diagnostic
✓ bonsai_prioritize_findings
✓ bonsai_generate_report
✓ bonsai_lint_file
✓ bonsai_lint_repo
✓ bonsai_generate_lint_rule
✓ bonsai_apply_fix
✓ bonsai_dismiss_diagnostic
✓ bonsai_report_false_positive
... (more tools)
MCP server ready for connections
```

**Keep this window open. Open a NEW PowerShell window for the next steps.**

---

## Phase 2: Quick Discovery Scan

### Step 2.1: Run Quick Scan
```powershell
# In NEW PowerShell window
cd Z:\Projects\BonsaiWorkspace
.\scripts\run-bug-hunter.ps1 -Path "." -Mode quick
```

**Expected Duration:** 1-3 seconds

**Expected Output:**
```
[HH:MM:SS] ╔════════════════════════════════════════╗
[HH:MM:SS] ║   Bug Hunter Automated Scan & Fix     ║
[HH:MM:SS] ╚════════════════════════════════════════╝

[HH:MM:SS] Configuration:
[HH:MM:SS]   Path: Z:\Projects\BonsaiWorkspace
[HH:MM:SS]   Mode: quick
[HH:MM:SS]   Auto-Fix: True
[HH:MM:SS]   Save to Survival: True
[HH:MM:SS]   Save to KDB: True

[HH:MM:SS] ✓ MCP server running with 15 tools
[HH:MM:SS] ✓ Scan complete: scan-2024-060201
[HH:MM:SS]   Total issues found: 47
[HH:MM:SS]   Critical: 3
[HH:MM:SS]   High: 12
[HH:MM:SS]   Medium: 18
[HH:MM:SS]   Low: 14

[HH:MM:SS] ✓ Found 15 critical/high issues
[HH:MM:SS] Processing: SQL Injection Vulnerability
[HH:MM:SS]   ✓ Fix applied
[HH:MM:SS]   Recording in Survival System...
[HH:MM:SS]   Recording in Knowledge Database...
...
[HH:MM:SS] ╔════════════════════════════════════════╗
[HH:MM:SS] ║            SCAN SUMMARY               ║
[HH:MM:SS] ╚════════════════════════════════════════╝

[HH:MM:SS] Issues Found:
[HH:MM:SS]   Total: 47
[HH:MM:SS]   Critical/High: 15

[HH:MM:SS] Fixes Applied:
[HH:MM:SS]   Auto-fixed: 10 ✓
[HH:MM:SS]   Failed: 1 ✗
[HH:MM:SS]   Manual review needed: 4 ⚠

[HH:MM:SS] Fixes saved to:
[HH:MM:SS]   ✓ Survival System
[HH:MM:SS]   ✓ Knowledge Database

[HH:MM:SS] Scan complete!
```

---

## Phase 3: Full Comprehensive Scan

### Step 3.1: Run Full Scan (After Quick Scan Completes)
```powershell
.\scripts\run-bug-hunter.ps1 -Path "." -Mode full
```

**Expected Duration:** 5-10 seconds

**Expected Output:** (Similar format to quick scan, but with more issues found)

---

## Phase 4: View Dashboard & Monitor Learning

### Step 4.1: Open Dashboard (Optional - in 3rd PowerShell window)
```powershell
cd Z:\Projects\BonsaiWorkspace
.\scripts\bug-hunter-dashboard.ps1 -Realtime
```

**Dashboard Shows:**
- ✅ MCP Server Status
- ✅ Recent Scan History
- ✅ Survival System Stats
- ✅ Knowledge Database Stats
- ✅ Metrics & Trends

---

## Expected Scan Results

### Issues by Category

| Category | Expected Count | Auto-Fix Rate |
|----------|----------------|---------------|
| SQL Injection | 2-3 | 90% |
| Unused Imports | 15-20 | 95% |
| Type Mismatches | 5-8 | 70% |
| Error Handling | 3-5 | 40% |
| Performance | 2-4 | 20% |
| Security | 3-5 | 60% |

### Expected Fix Statistics

```
Total Issues Found: 45-50
├─ Critical: 3
├─ High: 12-15
├─ Medium: 15-20
└─ Low: 10-15

Fixes Applied: 25-35
├─ Auto-fixed: 20-30
├─ Failed: 0-2
└─ Manual review: 2-5

Auto-Fix Success Rate: 70-80%
```

---

## Phase 5: Survival System Training

### What Gets Recorded

Each fix is recorded as:
```
error_pattern: "SQL Injection in database query"
solution_type: "fix"
solution_script: "SELECT * FROM users WHERE id = ?"
confidence: 0.80
usage_count: 0
success_count: 0
created_by: "bug-hunter"
verified: 0
created_at: 2026-06-02T[TIME]
```

### Expected Survival System State After Scan

```
Total Rules: 30-40
├─ From Bug Hunter: 25-30
├─ From Manual: 5-10
│
Success Rate: 85%+
Average Confidence: 0.80+
Total Uses: 0 (first run)
```

---

## Phase 6: Knowledge Database Integration

### What Gets Aggregated

```
Rule ID: fix-[UUID]
Issue Type: "sql-injection"
Issue Pattern: "User input in SQL query"
Solution: "Use parameterized queries"
Language: "rust"
Confidence: 0.80
Verified: false
Created At: 2026-06-02T[TIME]
Source: "bug-hunter"
```

### Expected KDB State After Scan

```
Total Rules: 30-40
Aggregated Confidence: 0.80+
Weekly Growth: +30
Most Effective Rules:
├─ SQL Injection Prevention (confidence: 0.95)
├─ Unused Import Removal (confidence: 0.98)
└─ Missing Error Handling (confidence: 0.75)
```

---

## Detailed Execution Steps

### STEP-BY-STEP WALKTHROUGH

#### 1. Build Succeeds ✓
```
Binary created at: target/release/bonsai-mcp-server.exe
Status: Ready to start
```

#### 2. Start Server (Terminal 1)
```powershell
.\target\release\bonsai-mcp-server.exe
# Runs indefinitely, serving MCP tools
```

#### 3. First Scan (Terminal 2)
```powershell
.\scripts\run-bug-hunter.ps1 -Path "." -Mode quick
# Finds ~47 issues in ~2 seconds
# Applies ~10 auto-fixes
# Records everything
```

#### 4. Full Scan (Terminal 2)
```powershell
.\scripts\run-bug-hunter.ps1 -Path "." -Mode full
# Comprehensive analysis (~8 seconds)
# Finds more detailed issues
# Applies additional fixes
```

#### 5. Monitor Results (Terminal 3, Optional)
```powershell
.\scripts\bug-hunter-dashboard.ps1 -Realtime
# Live view of:
# - Server status
# - Scan history
# - Survival System learning
# - KDB metrics
```

---

## Success Criteria

### Build Phase ✓
- [ ] Binary exists at `target/release/bonsai-mcp-server.exe`
- [ ] No compiler errors

### Server Phase ✓
- [ ] Server starts on port 3000
- [ ] All 15 tools registered
- [ ] Responds to curl requests

### Scan Phase ✓
- [ ] Quick scan finds 40+ issues
- [ ] Full scan finds 50+ issues
- [ ] Auto-fixes apply successfully
- [ ] Reports generate without errors

### Learning Phase ✓
- [ ] Survival System has 25+ entries
- [ ] Knowledge Database has 25+ entries
- [ ] Confidence scores recorded
- [ ] Dashboard shows metrics

---

## Timeline

| Phase | Command | Duration | Status |
|-------|---------|----------|--------|
| Build | cargo build | 2-10 min | [Build in progress] |
| Server Start | mcp-server.exe | <1 sec | Pending build |
| Quick Scan | run-bug-hunter quick | 2-3 sec | Pending server |
| Full Scan | run-bug-hunter full | 8-10 sec | Pending quick |
| Dashboard | dashboard -Realtime | Live | Pending scans |
| Analysis | [automatic] | [instant] | Pending scans |

---

## Key Files to Monitor

### Outputs Generated During Execution
- `bug-hunter-report-YYYYMMDD-HHmmss.md` - Markdown report from each scan
- `BUG_HUNTER_TRAINING_LOG.md` - This file gets updated with results
- `target/release/bonsai-mcp-server.exe` - Built binary

### Databases Updated
- `~/.bonsai/survival.db` - Survival System (if configured)
- `~/.bonsai/kdb.db` - Knowledge Database (if configured)

---

## Troubleshooting During Execution

### "MCP server not responding"
```powershell
# Verify it's running
curl http://127.0.0.1:3000/tools
# If it fails, check if port 3000 is in use
netstat -ano | findstr :3000
```

### "Script execution disabled"
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### "Tools not found"
```powershell
# Verify all 15 tools are registered
(curl http://127.0.0.1:3000/tools).Content | ConvertFrom-Json | Select -ExpandProperty tools | Measure-Object
# Should show 15 tools
```

---

## Next Steps After Execution

### Immediate (After scans complete, ~10-30 min)
1. ✅ Review quick scan results
2. ✅ Review full scan results
3. ✅ Check fixes applied
4. ✅ Verify Survival System has rules
5. ✅ Verify KDB has entries

### Short-term (Next run, tomorrow)
1. ✅ Run another scan to measure improvement
2. ✅ Check if confidence scores increased
3. ✅ Look for patterns in multiple scans
4. ✅ Manually verify critical fixes

### Medium-term (This week)
1. ✅ Fix bonsai-bot, bonsai-credits, bonsai-tui rusqlite versions
2. ✅ Re-enable full workspace build
3. ✅ Run scans on complete codebase
4. ✅ Compare results

### Long-term (This month)
1. ✅ Schedule daily automated scans
2. ✅ Track confidence improvement
3. ✅ Measure false positive reduction
4. ✅ Implement feedback loop improvements

---

## Success Checklist

- [ ] Build completes successfully
- [ ] Server starts without errors
- [ ] Quick scan finds issues (expect 40+)
- [ ] Full scan finds more issues (expect 50+)
- [ ] Auto-fixes apply successfully (expect 20+)
- [ ] Survival System records fixes
- [ ] Knowledge Database records rules
- [ ] Dashboard shows metrics
- [ ] Training log is populated
- [ ] Report files are generated

---

**When build completes, this plan will be executed automatically.**

**Current Status: Build in progress... [ETA: 2-10 minutes]**
