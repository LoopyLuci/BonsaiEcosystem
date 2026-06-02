# Bug Hunter Quick Start Guide

**Get the Bug Hunter running in 5 minutes** ⚡

---

## Prerequisites

1. **Rust installed:** `rustc --version` should show a version
2. **PowerShell 7+** (or Windows PowerShell for most scripts)
3. **~5 minutes** and patience

---

## Step-by-Step Quick Start

### 1️⃣ Fix Build Issues (2 minutes)

The workspace has dependency conflicts. We need to temporarily exclude problematic crates.

**Edit the root `Cargo.toml`:**

```bash
# Open in your favorite editor
notepad Z:\Projects\BonsaiWorkspace\Cargo.toml
```

Find the `[workspace]` section and remove these two lines:

```toml
# REMOVE these lines:
  "bonsai-bot",
  # (bonsai-bot is somewhere in the members list)
```

Also comment out crates that depend on rusqlite 0.32 if they cause issues:
```toml
# "crates/bonsai-credits",
# "crates/bonsai-tui",
```

Save and close.

### 2️⃣ Run Setup Script (2 minutes)

```powershell
cd Z:\Projects\BonsaiWorkspace
.\scripts\setup-bug-hunter.ps1 -BuildOnly
```

This will:
- ✅ Check Rust installation
- ✅ Build bonsai-mcp-server
- ✅ Create binary at `target/release/bonsai-mcp-server.exe`

**Expected output:**
```
✓ Binary created: target/release/bonsai-mcp-server.exe
Setup complete!
```

### 3️⃣ Start MCP Server (30 seconds)

```powershell
# Start the server
.\target\release\bonsai-mcp-server.exe
```

**Expected output:**
```
Starting MCP server on http://127.0.0.1:3000
Registering 15 tools...
✓ bonsai_scan_repo
✓ bonsai_list_findings
... (more tools)
MCP server ready for connections
```

Leave this window open.

### 4️⃣ Open New PowerShell Window & Verify

```powershell
# In a NEW terminal/PowerShell window:
curl http://127.0.0.1:3000/tools | ConvertFrom-Json | ForEach-Object { $_.tools } | Select-Object -First 5
```

You should see the first 5 tools listed. ✅

### 5️⃣ Run First Scan

```powershell
# In the same NEW PowerShell window:
.\scripts\run-bug-hunter.ps1 -Path "Z:\Projects\BonsaiWorkspace" -Mode quick
```

**What happens:**
- 🔍 Scans repository for bugs (should take 1-3 seconds in quick mode)
- 📊 Shows findings
- 🔧 Applies auto-fixable issues
- 📝 Saves fixes to database
- 📈 Generates report

**Expected output:**
```
[HH:MM:SS] ╔════════════════════════════════════════╗
[HH:MM:SS] ║   Bug Hunter Automated Scan & Fix     ║
[HH:MM:SS] ╚════════════════════════════════════════╝

[HH:MM:SS] ✓ MCP server running with 15 tools
[HH:MM:SS] ✓ Scan complete: scan-2024-060201
[HH:MM:SS]   Total issues found: 47
[HH:MM:SS]   Critical: 3
[HH:MM:SS]   High: 12
...
[HH:MM:SS] Scan complete!
```

✅ **You're done!**

---

## Command Reference

### Most Common Commands

```powershell
# Build the MCP server
.\scripts\setup-bug-hunter.ps1 -BuildOnly

# Build AND start the server
.\scripts\setup-bug-hunter.ps1 -StartServer

# Run a quick scan
.\scripts\run-bug-hunter.ps1 -Mode quick

# Run a full scan with all fixes applied
.\scripts\run-bug-hunter.ps1 -Mode full -AutoFix

# Run scan and save to both systems
.\scripts\run-bug-hunter.ps1 -Mode full -SaveToSurvival -SaveToKDB

# Show live dashboard
.\scripts\bug-hunter-dashboard.ps1 -Realtime

# View dashboard once without refresh
.\scripts\bug-hunter-dashboard.ps1 -Realtime:$false
```

### Manual HTTP Calls (if not using scripts)

```powershell
# List available tools
curl http://127.0.0.1:3000/tools | ConvertFrom-Json | Select-Object -ExpandProperty tools | Select-Object name, description

# Scan repository
$body = @{
    path = "Z:\Projects\BonsaiWorkspace"
    mode = "quick"
    ai_review = $true
} | ConvertTo-Json

Invoke-WebRequest -Uri "http://127.0.0.1:3000/tools/call" `
    -Method POST `
    -ContentType "application/json" `
    -Body $body | ConvertFrom-Json
```

---

## File Locations

**All scripts are in:** `Z:\Projects\BonsaiWorkspace\scripts\`

| File | Purpose |
|------|---------|
| `setup-bug-hunter.ps1` | Build & setup MCP server |
| `run-bug-hunter.ps1` | Run Bug Hunter scans |
| `bug-hunter-dashboard.ps1` | Monitor live stats |

**Output files:**

| File | Created By |
|------|------------|
| `bug-hunter-report-YYYYMMDD-HHmmss.md` | `run-bug-hunter.ps1` |
| `target/release/bonsai-mcp-server.exe` | Build process |

---

## Troubleshooting

### "Build failed: failed to select a version for libsqlite3-sys"

**Solution:** Edit Cargo.toml and comment out `bonsai-bot` and `bonsai-credits` as shown in Step 1.

### "MCP server not responding"

**Troubleshoot:**
```powershell
# Check if port is in use
netstat -ano | findstr :3000

# If something is listening, kill it
taskkill /PID <PID> /F

# Try starting the server again
.\target\release\bonsai-mcp-server.exe
```

### "Script execution disabled"

**Fix:**
```powershell
# Allow scripts for current user
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Then try again
.\scripts\run-bug-hunter.ps1 -Mode quick
```

### "Binary not found after build"

**Check:**
```powershell
# Verify it was created
ls target/release/ | findstr mcp

# If not there, check build output for errors
cargo build --package bonsai-mcp-server --release 2>&1 | tail -20
```

---

## What Each Script Does

### setup-bug-hunter.ps1

```
1. Check Rust is installed
2. Build bonsai-mcp-server in release mode
3. (Optional) Start the MCP server
4. (Optional) Register scheduled tasks for automation
```

**Usage:**
```powershell
# Just build
.\scripts\setup-bug-hunter.ps1 -BuildOnly

# Build and start
.\scripts\setup-bug-hunter.ps1 -StartServer

# Build, start, and schedule tasks
.\scripts\setup-bug-hunter.ps1 -StartServer -ScheduleTasks
```

### run-bug-hunter.ps1

```
1. Verify MCP server is running
2. Scan repository for bugs
3. List critical/high severity findings
4. For each fixable issue:
   a. Get fix details
   b. Apply fix
   c. Save to Survival System
   d. Save to Knowledge Database
5. Generate report
```

**Parameters:**
```powershell
-Path               # Repository to scan (default: current dir)
-Mode               # quick|full|ai (default: full)
-AutoFix           # Apply auto-fixable issues (default: true)
-SaveToSurvival    # Record in Survival System (default: true)
-SaveToKDB         # Record in Knowledge Database (default: true)
-GenerateReport    # Create markdown report (default: true)
```

### bug-hunter-dashboard.ps1

```
1. Connect to MCP server
2. Fetch statistics from:
   - Scan history
   - Survival System
   - Knowledge Database
3. Display live dashboard
4. Auto-refresh every N seconds
```

**Parameters:**
```powershell
-RefreshSeconds    # How often to refresh (default: 5)
-Realtime         # Auto-refresh or show once (default: true)
```

---

## Next Steps After Quick Start

### Option A: Manual Scans (Ad-hoc)

```powershell
# Whenever you want to scan
.\scripts\run-bug-hunter.ps1 -Mode full
```

### Option B: Automated Scans (Scheduled)

```powershell
# Schedule automated scans (requires admin)
.\scripts\setup-bug-hunter.ps1 -ScheduleTasks

# Check scheduled tasks
Get-ScheduledTask | Where-Object TaskName -like "*BugHunter*"
```

### Option C: Continuous Monitoring

```powershell
# Watch the dashboard in real-time
.\scripts\bug-hunter-dashboard.ps1 -Realtime
```

---

## Expected Results

### After First Quick Scan

```
Issues Found:
  Total: 47
  Critical/High: 15

Fixes Applied:
  Auto-fixed: 12
  Failed: 1
  Manual review needed: 2

Fixes saved to:
  ✓ Survival System
  ✓ Knowledge Database
```

### After One Week of Automated Scans

```
Survival System:
  Total Rules: 127
  Bug Hunter Rules: 47
  Average Confidence: 0.81
  Success Rate: 88%

Knowledge Database:
  Total Rules: 342
  Aggregated Confidence: 0.84
  Weekly Growth: +23 rules
```

---

## Performance

| Task | Time |
|------|------|
| Build MCP server | 2-5 min (first time) |
| Start MCP server | <1 sec |
| Quick scan | 1-3 sec |
| Full scan | 5-10 sec |
| Apply fix | <1 sec |
| Dashboard load | <1 sec |

---

## Questions?

See detailed documentation:
- **Architecture Details:** `MCP_TOOLS_ARCHITECTURE.md`
- **Build Help:** `BUILD_AND_RUN_GUIDE.md`
- **Integration Guide:** `BUG_HUNTER_SURVIVAL_INTEGRATION.md`
- **Tool Reference:** `BUG_HUNTER_MCP_USAGE.md`
- **Test Scenarios:** `BUG_HUNTER_TEST_SCENARIO.md`

---

## Success Checklist

✅ Rust installed
✅ Workspace build issues fixed (removed problematic crates from Cargo.toml)
✅ MCP server built successfully
✅ MCP server running on port 3000
✅ Bug Hunter scan completes without errors
✅ Issues are found and fixes applied
✅ Results saved to Survival System & KDB
✅ Dashboard shows statistics

**All set! Your Bug Hunter is ready to continuously find and fix issues!** 🚀
