# Universal Agent Control System (UACS)

The **Universal Agent Control System** enables any AI agent—Claude, Copilot, GPT, or custom—to safely control the Bonsai Ecosystem with complete observability, auditability, and human oversight.

## Quick Start

### 1. Start the MCP Server (Visual Mode)

```bash
cd Z:\Projects\BonsaiWorkspace
cargo run -p mcp-server -- visual --hitl-categories destructive,network --port 11426
```

**Expected Output:**
```
Universal Agent Control System (Visual with HITL) listening on 127.0.0.1:11426
Dashboard: http://127.0.0.1:11426
```

### 2. Start the Dashboard

```bash
cd uacs-dashboard
npm run dev
```

**Expected Output:**
```
Local: http://localhost:5173
```

### 3. Open in Browser

Navigate to **http://localhost:5173**

You should see:
- Header: "🧠 Universal Agent Control System"
- Status: "🟢 Connected"
- Empty timeline waiting for events

### 4. Connect an Agent

Configure Claude, Copilot, or your custom agent to connect via MCP.

#### Claude Desktop App

Edit `%APPDATA%\Claude\claude_desktop_config.json`:

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

Restart Claude Desktop.

#### VSCode Extension

1. Open Settings: `Ctrl+,`
2. Search: "Claude MCP"
3. Add configuration to `settings.json`
4. Restart VSCode

### 5. Give Claude a Task

Paste this prompt into Claude:

```
You are connected to the Bonsai Ecosystem via the Universal Agent Control System
in Visual Agent Control mode with Human-In-The-Loop enabled.

Your task is to improve the Bonsai codebase:

1. Read Cargo.toml to understand the workspace structure.
2. Run cargo check --workspace to verify the current build status.
3. If there are compilation errors, fix them one by one.
4. When the build is clean, run cargo test --workspace.
5. If any tests fail, diagnose and fix them.
6. Report your findings after each step.

Every action you take is visible on the UACS dashboard at http://localhost:5173
```

### 6. Watch It Work

Monitor the dashboard as Claude executes:

| Event | Meaning |
|-------|---------|
| **▶️ ToolCallStart** | Claude is calling a tool (read_file, run_cargo_check, etc.) |
| **✓ ToolCallEnd** | Tool execution completed |
| **🔴 Error** | Tool call failed |
| **⏸️ AgentPaused** | HITL approval needed (modal appears) |
| **▶️ AgentResumed** | You approved or denied the action |
| **📝 FileModified** | A file was changed |
| **🧪 TestRun** | Test suite ran |

## Visual Mode

### Features

- **Real-time dashboard** at http://localhost:5173
- **Live action timeline** showing every tool call
- **HITL approval modal** for destructive operations
- **One-click approve/deny** decisions
- **"Take the wheel"** to pause and edit files yourself
- **Time-travel replay** to debug agent behavior
- **Audit trail** of all approvals

### HITL Approval Modal

When Claude attempts a **destructive** or **network** operation:

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

Just click a button. The agent waits for your response.

### Risk Levels

| Level | Color | Examples |
|-------|-------|----------|
| **HIGH** 🔴 | Red | write_file, delete_file, deploy |
| **MEDIUM** 🟠 | Orange | web_search, http_request |
| **LOW** 🟢 | Green | read_file, search_codebase |

## Headless Mode

Run agents silently with approval policies:

```bash
cargo run -p mcp-server -- headless \
  --fallback-terminal \
  --verbose \
  --notify-on-error
```

### Features

- **No GUI required** (runs on servers, CI/CD)
- **Desktop notifications** with 30-second timeout
- **Terminal fallback prompts** (automatic if notifications fail)
- **Structured JSON logs** for integration
- **Configurable policies** (YAML approval rules)

### Approval Policies

Create `~/.config/bonsai/approval-policies.yaml`:

```yaml
policies:
  - name: "block_deploy"
    tool: "deploy_model"
    action: "deny"
    reason: "Deployments require manual review"

  - name: "auto_approve_reads"
    tool: "read_file"
    action: "approve"

  - name: "require_approval_for_external"
    regex: "http_request|web_search"
    action: "notify"
    timeout_seconds: 60
```

## Advanced Usage

### Custom Approval Categories

Require approval only for specific operations:

```bash
# Only destructive operations need approval
cargo run -p mcp-server -- visual --hitl-categories destructive

# Destructive and network operations
cargo run -p mcp-server -- visual --hitl-categories destructive,network

# All operations require approval (maximum safety)
cargo run -p mcp-server -- visual --hitl-categories all

# No HITL (fully autonomous, requires trust)
cargo run -p mcp-server -- visual --no-hitl
```

### Python Agent Example

```python
import requests
import json

URL = "http://127.0.0.1:11425/mcp"
TOKEN = "your-capability-token"

def call_tool(tool, args):
    payload = {
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {"name": tool, "arguments": args},
        "id": 1
    }
    headers = {"Authorization": f"Bearer {TOKEN}"}
    resp = requests.post(URL, json=payload, headers=headers)
    return resp.json()

# Read a file
result = call_tool("read_file", {"path": "README.md"})
print("File:", result)

# Run cargo check
result = call_tool("run_cargo_check", {})
print("Check:", result)
```

### TypeScript Agent Example

```typescript
import axios from 'axios';

const URL = 'http://127.0.0.1:11425/mcp';
const TOKEN = 'your-capability-token';

async function callTool(tool: string, args: any) {
  const response = await axios.post(URL, {
    jsonrpc: '2.0',
    method: 'tools/call',
    params: { name: tool, arguments: args },
    id: 1,
  }, { headers: { Authorization: `Bearer ${TOKEN}` } });
  return response.data;
}

(async () => {
  const content = await callTool('read_file', { path: 'README.md' });
  console.log(content);
})();
```

## CLI Commands (BTI Integration)

In Bonsai TUI, these commands manage UACS:

```
:agent list                    # List all connected agents
:agent pause <id>              # Pause an agent
:agent resume <id>             # Resume a paused agent
:agent kill <id>               # Terminate an agent session
:agent spawn <role>            # Launch a new agent

:approval list                 # Show pending approvals
:approval approve <id>         # Approve an operation
:approval deny <id>            # Deny an operation

:uacs mode visual              # Switch to visual mode
:uacs mode headless            # Switch to headless mode
:uacs config show              # Show current configuration
```

## Monitoring & Logging

### Real-Time Events

The dashboard shows all events in real time. Events are also logged to:

```bash
tail -f ~/.cache/bonsai/uacs-events.log
```

### Approval History

View approval/denial history:

```bash
grep "AgentPaused\|AgentResumed" ~/.cache/bonsai/uacs-events.log
```

Count approvals vs. denials:

```bash
grep "approved: true" ~/.cache/bonsai/uacs-events.log | wc -l   # Approved
grep "approved: false" ~/.cache/bonsai/uacs-events.log | wc -l  # Denied
```

### Export Audit Trail

Export a session as JSON for compliance:

```bash
curl http://127.0.0.1:11426/api/session/<id>/events > session-audit.json
```

## Troubleshooting

### Dashboard Won't Connect

1. Verify UACS server is running on port 11426:
   ```bash
   netstat -an | grep 11426
   ```
2. Check browser DevTools → Network → WebSocket
3. Restart both servers

### Agent Can't See Tools

1. Restart Claude or VSCode
2. Ask Claude: "List your available tools"
3. Should see 20+ tools. If not, MCP configuration is wrong.

### HITL Modal Never Appears

1. Verify Claude is doing something destructive (write_file, delete_file)
2. Check `--hitl-categories` includes the operation:
   ```bash
   cargo run -p mcp-server -- visual --hitl-categories destructive,network
   ```
3. Look for `AgentPaused` events in dashboard

### Build Fails After Changes

Claude will see the error and attempt to fix it. If it can't:
1. Look at the error message in the dashboard
2. Tell Claude: "Here's the error... [paste]. What do you think?"
3. Claude will propose and implement a fix

## Production Deployment

### Security Checklist

- ✅ Use HTTPS in production (not HTTP)
- ✅ Require authentication tokens
- ✅ Restrict network access (firewall rules)
- ✅ Enable HITL for all destructive operations
- ✅ Log all approvals and events
- ✅ Regular audit trail exports
- ✅ Backup capability tokens securely

### High-Availability Setup

```bash
# Server 1: Primary UACS
cargo run -p mcp-server -- visual --port 11426

# Server 2: Replica (for failover)
cargo run -p mcp-server -- visual --port 11427 --sync-from 127.0.0.1:11426

# Load balancer: nginx
# Forward to active server, failover on health check failure
```

### Monitoring

Expose Prometheus metrics:

```bash
curl http://127.0.0.1:11426/metrics
```

Track:
- `uacs_tool_calls_total` — Total tool calls by type
- `uacs_approvals_total` — Approvals vs. denials
- `uacs_agent_sessions_active` — Active agents
- `uacs_latency_ms` — Approval response time

## Integration with CI/CD

Use UACS in your CI pipeline:

```yaml
# .github/workflows/bonsai-improve.yml
name: Self-Improving Bonsai

on:
  schedule:
    - cron: '0 2 * * *'  # Daily at 2 AM UTC

jobs:
  improve:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: rust-lang/rust-toolchain@v1

      - name: Start UACS
        run: cargo run -p mcp-server -- headless --no-hitl &

      - name: Configure Claude
        env:
          CLAUDE_API_KEY: ${{ secrets.CLAUDE_API_KEY }}
        run: |
          # Configure Claude to connect to UACS
          # (Claude automatically discovers available tools)

      - name: Self-Improvement Task
        run: |
          # Claude connects via MCP and runs self-improvement
          # All actions logged to stdout
```

## Architecture

```
┌──────────────┐
│ Claude       │ (via MCP)
│ Copilot      │
│ Custom Agent │
└──────┬───────┘
       │ MCP JSON-RPC
       ▼
┌──────────────────────────────────┐
│ UACS Server (port 11426)         │
│  ├─ Tool Execution               │
│  ├─ HITL Approval Gates          │
│  ├─ Event Broadcast (WebSocket)  │
│  └─ Audit Logging                │
└──────┬──────────────────┬────────┘
       │ WebSocket        │ REST API
       ▼                  ▼
┌──────────────┐  ┌──────────────────┐
│ Dashboard    │  │ Approval Queue   │
│ (Svelte)     │  │ Logging          │
│              │  │ Policy Enforcer  │
└──────────────┘  └──────────────────┘
```

---

**Ready to start?** Run `.\START_UACS.ps1` (Windows) or `bash START_UACS.sh` (Linux/Mac). 🚀
