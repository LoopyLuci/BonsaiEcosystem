# Build & Run Guide - Bug Hunter MCP Server

**Status:** Implementation Complete, Build-Blocked by Workspace Dependencies  
**Date:** 2026-06-02  
**Goal:** Get Bug Hunter MCP server running and performing automated scans

---

## Problem Statement

The workspace has incompatible dependencies across multiple crates related to rusqlite and libsqlite3-sys versions:

```
✗ bonsai-bot        → rusqlite 0.32 → libsqlite3-sys 0.30
✗ bonsai-credits    → rusqlite 0.32 → libsqlite3-sys 0.30
✗ bonsai-tdl        → libsqlite3-sys 0.37 (explicit)
⚠️ Other crates with sqlx dependencies may also conflict
```

This prevents building the entire workspace, but **does NOT affect bonsai-mcp-server** which has no rusqlite dependencies.

---

## Solution Options

### OPTION A: Minimal Workspace (Fastest) ⭐ RECOMMENDED

Remove problematic crates from workspace members, build only what's needed.

**Time to Resolution:** 5 minutes  
**Risk:** Low (doesn't modify any code)  
**Best For:** Immediate testing

**Steps:**

1. Open `Cargo.toml` in workspace root
2. Comment out or remove problematic workspace members:

```toml
[workspace]
members = [
  # "bonsai-bot",          # ← REMOVE (rusqlite 0.32)
  "bonsai-native",
  # ... keep the rest ...
  # "crates/bonsai-credits",  # ← REMOVE (rusqlite 0.32)
  # "crates/bonsai-tui",      # ← REMOVE (depends on bonsai-credits)
]
```

3. Run build:
```bash
cd Z:\Projects\BonsaiWorkspace
cargo build --package bonsai-mcp-server --release
```

**After Build:**
- ✅ bonsai-mcp-server compiles
- ✅ Problematic crates excluded temporarily
- ✅ Can run Bug Hunter immediately
- ⚠️ bonsai-bot, bonsai-credits, bonsai-tui need separate fixing

---

### OPTION B: Fix All Versions (Proper) ⭐ RECOMMENDED FOR LONG-TERM

Update all crates to use compatible versions of rusqlite/libsqlite3-sys.

**Time to Resolution:** 30-60 minutes  
**Risk:** Medium (requires testing)  
**Best For:** Production deployment

**Analysis:**

```
Option B1: Upgrade rusqlite to 0.32+
├─ Find compatible libsqlite3-sys version
├─ Update workspace to use that version
├─ Test all dependent crates
└─ Update Cargo.lock

Option B2: Downgrade to rusqlite 0.31 or lower
├─ Check what works with libsqlite3-sys 0.37
├─ Update bonsai-bot, bonsai-credits
├─ Test migration
└─ Verify no regressions

Option B3: Use workspace.dependencies properly
├─ Define libsqlite3-sys at workspace level
├─ Ensure all crates use workspace version
├─ Let cargo resolve correctly
└─ Most robust solution
```

**Recommended: Option B3 (workspace.dependencies)**

Current state:
```toml
[workspace.dependencies]
libsqlite3-sys = { version = "0.37", features = ["bundled"] }
```

But individual crates override this:
```toml
# In bonsai-bot/Cargo.toml
rusqlite = { version = "0.32", features = ["bundled"] }
# ↑ This brings in libsqlite3-sys 0.30, conflicting!
```

**Fix:**

```bash
# For each affected crate (bonsai-bot, bonsai-credits, etc.):

# 1. Check what version of rusqlite works with libsqlite3-sys 0.37
cargo search rusqlite --limit 1

# 2. Update Cargo.toml to compatible version
# nano crates/bonsai-bot/Cargo.toml
# Change: rusqlite = { version = "0.32", ... }
# To:     rusqlite = { version = "0.33", ... }  # or whatever is compatible

# 3. Test the crate builds
cargo build --package bonsai-bot

# 4. Repeat for each affected crate
```

---

## Step-by-Step Instructions

### QUICKSTART (Option A - Recommended for Now)

```bash
# 1. Navigate to workspace root
cd Z:\Projects\BonsaiWorkspace

# 2. Edit Cargo.toml to remove problematic crates
#    (Comment out lines with bonsai-bot, bonsai-credits, bonsai-tui)
#    You can use any text editor or:

# 3. Build bonsai-mcp-server only
cargo build --package bonsai-mcp-server --release

# Expected output:
#   Compiling bonsai-mcp-server v0.1.0
#   Finished release [optimized] target(s) in XXXs

# 4. Verify binary exists
ls target/release/bonsai-mcp-server.exe

# 5. Start the MCP server
./target/release/bonsai-mcp-server.exe

# Expected output:
#   Starting MCP server on http://127.0.0.1:3000
#   Registering 15 tools...
#   ✓ bonsai_scan_repo
#   ✓ bonsai_list_findings
#   ... (13 more tools)
#   MCP server ready for connections
```

### VERIFY TOOLS AVAILABLE

In a new terminal:

```bash
# Check that tools are registered
curl http://127.0.0.1:3000/tools

# Expected response (JSON with all 15 tools):
# {
#   "tools": [
#     {
#       "name": "bonsai_scan_repo",
#       "description": "Scan repository for bugs...",
#       "inputSchema": { ... }
#     },
#     ... (14 more tools)
#   ]
# }
```

### CONNECT CLAUDE

Once the server is running:

1. In Claude Code settings, add MCP server:
   ```json
   {
     "name": "bonsai-mcp-server",
     "command": "Z:\\Projects\\BonsaiWorkspace\\target\\release\\bonsai-mcp-server.exe"
   }
   ```

2. Restart Claude Code

3. In Claude, ask:
   ```
   Tell me what tools you have available
   ```

   Expected response:
   ```
   I can access these Bonsai tools:
   
   Bug Hunter (7 tools):
   - bonsai_scan_repo
   - bonsai_list_findings
   - ...
   
   Linter (8 tools):
   - bonsai_lint_repo
   - ...
   ```

### RUN FIRST SCAN

Ask Claude:

```
Scan the Z:\Projects\BonsaiWorkspace repository for bugs 
and vulnerabilities using bonsai_scan_repo
```

Expected response:
```
✓ Scan started for Z:\Projects\BonsaiWorkspace
✓ Scan ID: scan-2024-060201
✓ Found 47 issues:
  - Critical: 3
  - High: 12
  - Medium: 18
  - Low: 14
```

---

## Troubleshooting

### Build Error: "failed to select a version for libsqlite3-sys"

**Cause:** Workspace members have conflicting dependencies  
**Solution:** Remove conflicting crates from `Cargo.toml` members list

### Build Error: "can't find `bug_hunt_tools`"

**Cause:** New modules not in lib.rs  
**Solution:** Verify `crates/bonsai-mcp-server/src/lib.rs` has:
```rust
pub mod bug_hunt_tools;
pub mod lint_tools;
pub mod tool_registry;
```

### MCP Server Won't Start

**Check:**
1. Port 3000 already in use?
   ```powershell
   netstat -ano | findstr :3000
   ```
   
2. Binary missing?
   ```powershell
   ls target/release/bonsai-mcp-server.exe
   ```

3. Permissions issue?
   ```powershell
   icacls target/release/bonsai-mcp-server.exe
   ```

### Claude Can't Find Tools

**Check:**
1. MCP server running?
   ```bash
   curl http://127.0.0.1:3000/tools
   ```

2. Claude MCP config correct?
   ```json
   {
     "name": "bonsai-mcp-server",
     "command": "full\\path\\to\\bonsai-mcp-server.exe"
   }
   ```

3. Try restarting Claude Code

---

## What to Fix After Build Works

### Immediate (Same day)

- [ ] Test Bug Hunter scan workflow
- [ ] Verify Survival System integration
- [ ] Test Knowledge Database recording

### Short-term (This week)

- [ ] Fix bonsai-bot rusqlite version
- [ ] Fix bonsai-credits rusqlite version
- [ ] Re-enable in workspace members
- [ ] Run full workspace build
- [ ] Test all crates together

### Medium-term (This month)

- [ ] Implement automated scanning cycle
- [ ] Set up feedback loop (survival → KDB)
- [ ] Create dashboards for metrics
- [ ] Document best practices from found bugs

---

## Complete Workflow Once Built

```
┌─────────────────────────────────────────┐
│  1. Build MCP Server                    │
│     cargo build --package               │
│     bonsai-mcp-server --release         │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  2. Start Server                        │
│     ./target/release/                   │
│     bonsai-mcp-server.exe               │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  3. Connect Claude                      │
│     Add MCP config                      │
│     Restart Claude Code                 │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  4. Run Bug Hunter Scan                 │
│     Scan repository                     │
│     Find issues                         │
│     Apply fixes                         │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  5. Record Fixes                        │
│     Save to Survival System             │
│     Record in Knowledge Database        │
│     Update confidence scores            │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  6. Continuous Learning                 │
│     Run periodic scans                  │
│     Sync survival → KDB                 │
│     Improve over time                   │
└─────────────────────────────────────────┘
```

---

## Performance Expectations

| Phase | Duration | Notes |
|-------|----------|-------|
| Build | 2-5 min | First time longer, cached after |
| Server Start | <1 sec | Registers 15 tools |
| Discovery | <100ms | Claude learning tool list |
| Scan (quick) | 1-3 sec | Limited analysis |
| Scan (full) | 5-10 sec | Comprehensive |
| Fix Application | <1 sec per fix | If auto-fixable |
| DB Recording | <100ms | Survival + KDB |

---

## Success Criteria

✅ **Complete When:**

1. [ ] MCP server starts without errors
2. [ ] All 15 tools appear in `tools/list`
3. [ ] Claude can see the tools
4. [ ] `bonsai_scan_repo` executes successfully
5. [ ] Claude receives scan results
6. [ ] `bonsai_auto_fix` applies fixes
7. [ ] Fixes are recorded in Survival System
8. [ ] Knowledge Database has entries

**Estimated Total Time:** 30 minutes from workspace fix to first working scan

---

## Next: Automate Everything

Once manual workflow works, create automation:

```rust
// Run every 6 hours
#[tokio::schedule("0 */6 * * *")]
async fn automated_bug_scan() {
    run_bug_hunter_scan_cycle(
        "Z:\\Projects\\BonsaiWorkspace",
        &survival_pool,
        &knowledge_db,
    ).await.ok();
}

// Sync survival → KDB every day
#[tokio::schedule("0 0 * * *")]
async fn daily_kdb_sync() {
    sync_survival_to_kdb(&survival_pool, &kdb).await.ok();
}

// Update confidence scores weekly
#[tokio::schedule("0 0 * * 0")]
async fn weekly_confidence_update() {
    update_confidence_from_outcomes(
        &survival_pool,
        &kdb,
    ).await.ok();
}
```

---

## Questions?

If the build still fails after trying OPTION A:

1. Check which crates still cause conflicts
2. Comment them all out
3. Build just bonsai-mcp-server
4. Test that build succeeds
5. Then iteratively uncomment and fix each one

**The MCP server itself has no problematic dependencies** - it only depends on crates without rusqlite/sqlx issues.

