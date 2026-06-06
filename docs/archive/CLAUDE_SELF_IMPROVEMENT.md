# Claude Self‑Improvement — Visual Agent Control with HITL

## Overview

Claude can now work autonomously on the **Bonsai Ecosystem to improve itself**, with complete transparency and safety:

- **Visual Dashboard** — Every action visible in real-time
- **Human-In-The-Loop (HITL)** — Approve/deny destructive operations instantly
- **Full Audit Trail** — Every decision logged
- **VSCode Integration** — Claude runs inside your editor

---

## Quick Setup (5 minutes)

### Terminal 1: Start UACS Visual Server

```powershell
cd Z:\Projects\BonsaiWorkspace
cargo run -p mcp-server -- visual --hitl-categories destructive,network --port 11426
```

**Expected output:**
```
Universal Agent Control System (Visual) listening on 127.0.0.1:11426
Dashboard: http://127.0.0.1:11426
```

**Keep this running.**

### Terminal 2: Start Dashboard

```powershell
cd Z:\Projects\BonsaiWorkspace\uacs-dashboard
npm install
npm run dev
```

**Expected output:**
```
Local: http://localhost:5173
```

**Keep this running.**

### Browser: Open Dashboard

Navigate to: **http://localhost:5173**

You should see the UACS dashboard with:
- Header: "🧠 Universal Agent Control System"
- Status: "🟢 Connected"
- Empty timeline (waiting for events)

---

## Configure Claude in VSCode

Claude running inside VSCode needs access to the UACS MCP server.

### Option A: VSCode Claude Extension (Native MCP Support)

1. Open VSCode Settings: `Ctrl+,`
2. Search: `Claude MCP`
3. Look for a config file field or "Edit in settings.json"
4. Add this configuration:

```json
{
  "claude.mcp.servers": {
    "bonsai": {
      "command": "cargo",
      "args": [
        "run", "-p", "mcp-server", "--",
        "visual",
        "--host", "127.0.0.1",
        "--port", "11426",
        "--hitl-categories", "destructive,network"
      ]
    }
  }
}
```

5. Restart VSCode or reload Claude extension
6. Verify Claude can see the tools (ask: "List your available tools")

### Option B: Claude Desktop App (Simpler Fallback)

If the VSCode extension doesn't support MCP, use the standalone Claude Desktop app with MCP configuration:

1. Edit: `%APPDATA%\Claude\claude_desktop_config.json`
2. Add:

```json
{
  "mcpServers": {
    "bonsai": {
      "command": "cargo",
      "args": [
        "run", "-p", "mcp-server", "--",
        "visual",
        "--host", "127.0.0.1",
        "--port", "11426"
      ]
    }
  }
}
```

3. Restart Claude Desktop
4. Claude now has access to Bonsai tools and will work on Bonsai files

---

## Give Claude the Self‑Improvement Task

Copy and paste this exact prompt into Claude:

> **You are connected to the Bonsai Ecosystem via the Universal Agent Control System in Visual Agent Control mode with Human‑In‑The‑Loop enabled. Every destructive or network operation you attempt will require my approval via the dashboard modal.**
>
> **Your task is to improve the Bonsai codebase itself. Follow this exact sequence:**
>
> 1. **Read `Cargo.toml`** to understand the workspace structure.  
> 2. **Run `cargo check --workspace`** to verify the current build status.  
> 3. **If there are any compilation errors or warnings**, fix them one by one. After each fix, run `cargo check` again to confirm.  
> 4. **When the build is completely clean**, run `cargo test --workspace` and report the results.  
> 5. **If any tests fail**, read the error output, diagnose the issue, propose a fix, apply the fix, and re‑run tests. Continue until all tests pass.  
> 6. **When the build is clean and all tests pass**, check for any `todo!()` or `unimplemented!()` macros using code search. Create an issue for each one.  
> 7. **If you encounter an error you cannot diagnose**, ask me for guidance.  
>
> **Report your findings after each step. Every action you take is visible on the UACS dashboard.**

---

## Watch Claude Work

Keep the **UACS Dashboard** visible at http://localhost:5173. You will see real-time events:

### Event Timeline

| Event | Meaning |
|-------|---------|
| **▶️ ToolCallStart** | Claude is about to call a tool (read_file, run_cargo_check, etc.) |
| **✓ ToolCallEnd** | Tool execution completed (with duration in milliseconds) |
| **🔴 Error** | Tool call failed (red card with error message) |
| **⏸️ AgentPaused** | Claude wants to write a file — HITL approval modal appears |
| **▶️ AgentResumed** | You clicked Approve/Deny in the modal |
| **📝 FileModified** | A file was changed (path and size shown) |
| **🧪 TestRun** | Test suite ran (pass/fail and output) |

### Example Sequence

```
1. ▶️ read_file: Cargo.toml (starts)
   ✓ ToolCallEnd: read_file (45ms)

2. ▶️ run_cargo_check (starts)
   ✓ ToolCallEnd: run_cargo_check (3200ms)
   └─ Output: "error[E0433]: cannot find function `foo`"

3. ▶️ read_file: src/lib.rs (starts)
   ✓ ToolCallEnd: read_file (32ms)

4. ⏸️ AgentPaused: write_file
   └─ Tool: write_file
   └─ Description: Write file: src/lib.rs
   └─ Risk: HIGH
   └─ [Approve] [Deny] buttons

5. [You click: Approve]
   ▶️ AgentResumed: write_file (approved: true)

6. 📝 FileModified: src/lib.rs (+87 bytes)

7. ▶️ run_cargo_check (starts)
   ✓ ToolCallEnd: run_cargo_check (3100ms)
   └─ Output: "Build succeeded"
```

---

## HITL Approval Modal

When Claude attempts a **destructive operation** (write file, delete file, deploy, etc.), the dashboard shows:

```
╔═════════════════════════════════════════════╗
║  🔔 UACS Approval Required                 ║
╠═════════════════════════════════════════════╣
║  Tool: write_file                           ║
║  Description: Write file: src/main.rs       ║
║  Risk Level: HIGH 🔴                        ║
║                                             ║
║  [View Operation Details]                   ║
╠═════════════════════════════════════════════╣
║  [✅ Approve]              [❌ Deny]        ║
╚═════════════════════════════════════════════╝
```

### Your Decision

- **✅ Approve** — Claude's change is applied immediately, tool call proceeds
- **❌ Deny** — Operation is blocked, Claude receives error and can try an alternative

**Routine operations** (reading files, running checks, listing tools) proceed **without interruption**.

---

## Following Up: Extended Self‑Improvement

After Claude completes the initial cleanup, give this follow‑up task:

> **Now that the codebase is clean, begin improving the Bonsai Ecosystem for agent‑driven development. I would like you to:**
>
> 1. **Analyze** `crates/mcp-server/src/tools.rs` and identify 3 new tools that would be useful.
> 2. **Propose** the tools to me (names, descriptions, use cases).
> 3. **Implement** each tool with proper error handling.
> 4. **Write tests** for each new tool.
> 5. **Run the full test suite** and ensure all tests pass.
> 6. **Commit** with a descriptive message.
>
> Every write operation will require my approval via the HITL modal. Report your progress after each step.

Claude will:
1. Read the tools file
2. Propose new tools in the chat
3. Wait for your feedback
4. Request approval for each file write
5. Implement, test, and commit

**Everything is visible and auditable on the dashboard.**

---

## Key Commands Reference

### Starting Everything

| Terminal | Command |
|----------|---------|
| **T1** | `cargo run -p mcp-server -- visual --hitl-categories destructive,network --port 11426` |
| **T2** | `cd uacs-dashboard && npm run dev` |
| **Browser** | `http://localhost:5173` |

### Useful Claude Prompts

```
"List all the tools you have available."

"What files are in the Bonsai workspace?"

"Run cargo check and tell me if there are any errors."

"Fix any compilation errors you find, then run the full test suite."

"Show me all todo!() macros in the codebase and create an issue for each."
```

### Logs

View approval decisions:
```bash
grep "AgentPaused\|AgentResumed" uacs-agent.log
```

View all events:
```bash
tail -f uacs-agent.log
```

---

## Safety Guarantees

### ✅ What Requires HITL Approval

By default (`--hitl-categories destructive,network`):

**Destructive:**
- `write_file` — Create/modify files
- `delete_file` — Remove files
- `deploy_model` — Deploy models
- `run_cargo_publish` — Publish crates
- `git_force_push` — Force-push to Git

**Network:**
- `web_search` — Search the web
- `create_collaboration_session` — Remote access
- `http_request` — HTTP calls

### ✅ What Proceeds Without Interruption

Routine operations don't need approval:
- `read_file` — Reading files (no confirmation)
- `run_cargo_check` — Building (no confirmation)
- `run_cargo_test` — Testing (no confirmation)
- `search_codebase` — Searching (no confirmation)
- `chat` — Claude talking to Bonsai (no confirmation)

### ✅ Timeout Protection

In headless mode (not visual):
- Approval requests timeout after 30 seconds
- Auto-deny if no response
- Logged for audit

### ✅ Audit Trail

Every approval/denial is logged:
```
[2026-05-31T14:32:15Z] AgentPaused: write_file (risk: high)
[2026-05-31T14:32:18Z] AgentResumed: write_file (approved: true)
```

---

## Example: Complete Self‑Improvement Session

### Setup (First Time)

```powershell
# Terminal 1
cd Z:\Projects\BonsaiWorkspace
cargo run -p mcp-server -- visual --port 11426

# Terminal 2
cd uacs-dashboard
npm run dev

# Browser
Open http://localhost:5173
```

### Claude Starts

You give Claude the self‑improvement prompt. It begins:

```
✓ Checking Cargo.toml... (read_file)
✓ Running cargo check... (run_cargo_check)

Found 3 compilation errors in src/lib.rs.

Attempting to fix:
1. Missing import `use crate::bridge;`
   [HITL Modal] Approve? ✅ Click Approve
   ✓ File written
   ✓ cargo check: Build passed

2. Unused variable in src/main.rs
   [HITL Modal] Approve? ✅ Click Approve
   ✓ File written
   ✓ cargo check: Build passed

3. Deprecation warning in Cargo.toml
   [HITL Modal] Approve? ✅ Click Approve
   ✓ File written
   ✓ cargo check: Build passed

All errors fixed. Running full test suite...
✓ 45 tests passed, 0 failed

Codebase is now clean. Creating issues for todo!() macros...
✓ Found 7 todo!() calls
✓ Created 7 GitHub issues

Self‑improvement complete! ✅
```

### Dashboard Shows

- Timeline of 50+ events
- 5 HITL approvals (you clicked Approve each time)
- All files modified are logged
- Test results visible
- Final status: Green ✅

### Result

The Bonsai codebase is now:
- ✅ Clean (no errors)
- ✅ Tested (all tests pass)
- ✅ Documented (issues created for todos)
- ✅ Improved (all under your supervision)

---

## Troubleshooting

### Dashboard Won't Connect

1. Verify UACS server is running (Terminal 1 should show "listening on 127.0.0.1:11426")
2. Check browser at http://localhost:5173
3. Open browser DevTools → Network → WebSocket (should see `/ws/events`)
4. If WebSocket fails, restart Terminal 1

### Claude Can't See Tools

1. Restart VSCode or Claude Desktop app
2. Check that UACS server is running on port 11426
3. In Claude, ask: "List your available tools" (should see 20+ tools)
4. If no tools: MCP configuration may be incorrect

### HITL Modal Never Appears

1. Verify you're giving Claude a task that writes files (e.g., "fix this error and write the fix")
2. Check that HITL is enabled: `--hitl-categories destructive,network` should be in the command
3. Look at the dashboard timeline for `AgentPaused` events

### Build Fails After Changes

Claude will see the error and attempt to fix it automatically. If it can't:
- Look at the error message in the dashboard
- Tell Claude: "Here's the error... [paste error]. What do you think?"
- Claude will propose and implement a fix

---

## What's Happening Behind the Scenes

### Architecture

```
Claude (in VSCode or Desktop)
    ↓ MCP JSON-RPC
Bonsai MCP Server (port 11426)
    ↓ UACS Proxy Layer
Tool Categorization & HITL Logic
    ↓ Event Emission (WebSocket)
Dashboard (http://localhost:5173)
    ↓ User Approval
Back to Agent via /api/respond endpoint
```

### Event Flow

1. Claude calls `write_file("src/lib.rs", "...")`
2. UACS checks: is `write_file` destructive? YES
3. UACS pauses, generates request_id: `abc123`
4. Emits `AgentPaused` event via WebSocket
5. Dashboard receives event, shows modal
6. You click ✅ Approve
7. Dashboard POSTs to `/api/respond` with `approved: true`
8. UACS wakes up, allows tool call to proceed
9. File is written, emits `ToolCallEnd` and `FileModified` events
10. Dashboard updates timeline

**Total time:** 1-5 seconds (your decision) + tool execution time

---

## Next Steps

1. **Start the servers** (Terminals 1 & 2)
2. **Open the dashboard** (Browser)
3. **Configure Claude** (VSCode or Desktop)
4. **Give Claude the self‑improvement task** (Paste prompt)
5. **Watch it work** (Monitor dashboard)
6. **Approve/deny** as needed (Click modal buttons)
7. **Follow up** with extended improvement tasks

---

## Summary

Claude is now ready to:

✅ **Self‑improve** the Bonsai Ecosystem
✅ **Fix compilation errors** autonomously
✅ **Run tests** and diagnose failures
✅ **Propose code changes** with your approval
✅ **Commit improvements** with full audit trail
✅ **Operate safely** with Human-In-The‑Loop gates

**You stay in complete control. Every action is visible. Every change requires your approval.**

The self‑building, self‑improving Bonsai Ecosystem is now live. 🚀
