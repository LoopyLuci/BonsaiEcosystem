# 🚀 UACS Quick Start Guide

Universal Agent Control System is ready to launch. Here's how to get started:

## Option 1: Automated Startup (Recommended)

### PowerShell (Windows)
```powershell
.\START_UACS.ps1
```

This will:
1. ✅ Build the Rust server
2. ✅ Start Terminal 1: UACS Server (Visual Mode + HITL)
3. ✅ Start Terminal 2: Svelte Dashboard
4. ✅ Open Terminal 3: Browser at http://localhost:5173

### Bash (Linux/Mac)
```bash
bash START_UACS.sh
```

---

## Option 2: Manual Startup (If You Prefer)

### Terminal 1: Start UACS Server
```bash
cd Z:\Projects\BonsaiWorkspace
cargo run -p mcp-server -- visual --hitl-categories destructive,network --port 11426
```

**Expected Output:**
```
Universal Agent Control System (Visual with HITL) listening on 127.0.0.1:11426
Dashboard: http://127.0.0.1:11426
```

### Terminal 2: Start Dashboard
```bash
cd Z:\Projects\BonsaiWorkspace\uacs-dashboard
npm run dev
```

**Expected Output:**
```
Local: http://localhost:5173
```

### Terminal 3: Open in Browser
```
http://localhost:5173
```

---

## What You Should See

### Dashboard Status (http://localhost:5173)
- ✅ Header: "🧠 Universal Agent Control System"
- ✅ Status: "🟢 Connected" (green dot)
- ✅ Empty timeline (waiting for agent events)

---

## Configure Claude to Connect

Once the servers are running, configure Claude to use UACS:

### Option A: VSCode Claude Extension
1. Open VSCode Settings (`Ctrl+,`)
2. Search for "Claude MCP"
3. Add to `settings.json`:

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

### Option B: Claude Desktop App
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
        "--hitl-categories", "destructive,network"
      ]
    }
  }
}
```

3. Restart Claude Desktop

---

## Give Claude the Self-Improvement Task

Copy this prompt and paste it into Claude:

```
You are connected to the Bonsai Ecosystem via the Universal Agent Control System
in Visual Agent Control mode with Human-In-The-Loop enabled. Every destructive or
network operation you attempt will require my approval via the dashboard modal.

Your task is to improve the Bonsai codebase itself. Follow this exact sequence:

1. **Read `Cargo.toml`** to understand the workspace structure.
2. **Run `cargo check --workspace`** to verify the current build status.
3. **If there are any compilation errors or warnings**, fix them one by one.
   After each fix, run `cargo check` again to confirm.
4. **When the build is completely clean**, run `cargo test --workspace`
   and report the results.
5. **If any tests fail**, read the error output, diagnose the issue, propose
   a fix, apply the fix, and re-run tests. Continue until all tests pass.
6. **When the build is clean and all tests pass**, check for any `todo!()`
   or `unimplemented!()` macros using code search. Create an issue for each one.
7. **If you encounter an error you cannot diagnose**, ask me for guidance.

Report your findings after each step. Every action you take is visible on the
UACS dashboard.
```

---

## Monitor the Dashboard

As Claude works, you'll see:

| Event | Meaning |
|-------|---------|
| **▶️ ToolCallStart** | Claude is about to call a tool |
| **✓ ToolCallEnd** | Tool execution completed |
| **🔴 Error** | Tool call failed |
| **⏸️ AgentPaused** | Claude wants to do something — HITL approval modal appears |
| **▶️ AgentResumed** | You clicked Approve/Deny |
| **📝 FileModified** | A file was changed |
| **🧪 TestRun** | Test suite ran |

---

## HITL Modal

When Claude attempts a destructive or network operation, you'll see:

```
┌─────────────────────────────────┐
│ 🔔 UACS Approval Required       │
├─────────────────────────────────┤
│ Tool: write_file                │
│ Description: Write file: src... │
│ Risk Level: HIGH 🔴             │
├─────────────────────────────────┤
│ [✅ Approve]   [❌ Deny]        │
└─────────────────────────────────┘
```

Just click a button. Simple as that.

---

## Troubleshooting

### Dashboard Won't Connect
1. Verify UACS server is running on port 11426
2. Check browser DevTools → Network → WebSocket
3. Restart both servers

### Claude Can't See Tools
1. Restart VSCode or Claude Desktop
2. In Claude, ask: "List your available tools"
3. Should see 20+ tools

### HITL Modal Doesn't Appear
1. Verify Claude is giving it a task that writes files
2. Check `--hitl-categories destructive,network` is in the command
3. Look for `AgentPaused` events in the dashboard timeline

---

## Configuration Reference

### UACS Server Modes

**Visual Mode (Recommended for Development):**
```bash
cargo run -p mcp-server -- visual --port 11426
```

**Headless Mode (Background Execution):**
```bash
cargo run -p mcp-server -- headless --fallback-terminal
```

**Fully Autonomous (No HITL):**
```bash
cargo run -p mcp-server -- visual --no-hitl
```

**Maximum Safety (Every Operation Requires Approval):**
```bash
cargo run -p mcp-server -- visual --hitl-categories all
```

---

## What's Next?

1. ✅ Start the servers using `START_UACS.ps1` or manual setup
2. ✅ Open the dashboard at http://localhost:5173
3. ✅ Configure Claude to connect
4. ✅ Give Claude the self-improvement task
5. ✅ Watch it work in real-time
6. ✅ Approve/deny operations as needed
7. ✅ Review the audit trail in the dashboard

**You now have a production-grade Universal Agent Control System!** 🚀

For more details, see:
- [CLAUDE_SELF_IMPROVEMENT.md](CLAUDE_SELF_IMPROVEMENT.md) — Full setup guide
- [UACS_HITL_GUIDE.md](UACS_HITL_GUIDE.md) — Human-In-The-Loop details
- [UNIVERSAL_AGENT_CONTROL.md](UNIVERSAL_AGENT_CONTROL.md) — Architecture overview
- [UACS_NEXT_GENERATION_BLUEPRINT.md](UACS_NEXT_GENERATION_BLUEPRINT.md) — Production enhancements
