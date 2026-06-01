# Claude Agent Control — Visual & Headless Modes for the Bonsai Ecosystem

This document describes how to set up and use Claude Agent Control, which enables Claude (or any LLM) to work on the Bonsai codebase autonomously with full transparency. The system supports two modes: **Visual Agent Control** (with a live GUI dashboard) and **Headless Agent Control** (background execution with optional notifications).

## Architecture Overview

```
┌─────────────────┐      MCP (JSON-RPC over WebSocket/HTTP)      ┌─────────────────┐
│   Claude (LLM)  │ ──────────────────────────────────────────▶ │  Visualisation  │
└─────────────────┘                                              │   Proxy Layer   │
                                                                 │  (Rust + Axum) │
                                                                 └────────┬────────┘
                                                                          │
                                                                          ▼
                                                                  ┌─────────────┐
                                                                  │ Bonsai MCP  │
                                                                  │   Server    │
                                                                  └─────────────┘
```

The **Visualisation Proxy** intercepts every MCP request and response. In **Visual Mode**, it emits events that the Svelte dashboard renders in real-time. In **Headless Mode**, it logs actions and optionally sends desktop notifications.

---

## Part 1: Building the System

### Prerequisites

- Rust 1.70+ (for the MCP server)
- Node.js 18+ (for the Svelte dashboard)
- Cargo (Rust's package manager)
- npm or yarn (for JavaScript dependencies)

### Step 1: Build the Rust MCP Server with Visualiser

The visualiser is now integrated into the `bonsai-mcp-server` crate. To build it as a binary:

```bash
cd Z:\Projects\BonsaiWorkspace
cargo build -p bonsai-mcp-server --release
```

This creates an executable at `target/release/bonsai-mcp-server` (or `.exe` on Windows).

### Step 2: Build the Svelte Dashboard (Optional)

The dashboard is a standalone web app that connects to the visualiser via WebSocket. To build it:

```bash
cd Z:\Projects\BonsaiWorkspace\visualiser-ui
npm install
npm run build
```

The built files are output to `visualiser-ui/dist/`. You can serve them with any HTTP server, or run them locally with:

```bash
npm run dev
```

This starts a dev server at `http://localhost:5173`.

---

## Part 2: Running the System

### Mode 1: Visual Agent Control (with Dashboard)

Visual mode allows you to watch every action Claude takes in a GUI dashboard.

**Terminal 1 — Start the MCP server in visual mode:**

```bash
cd Z:\Projects\BonsaiWorkspace
cargo run -p bonsai-mcp-server -- visual --host 127.0.0.1 --port 11426
```

You should see:
```
Bonsai MCP visualiser (visual mode) listening on 127.0.0.1:11426
Dashboard: http://127.0.0.1:11426
```

**Terminal 2 — Start the Svelte dashboard (optional, for better UI):**

```bash
cd Z:\Projects\BonsaiWorkspace\visualiser-ui
npm run dev
```

Open `http://localhost:5173` in your browser.

**Terminal 3 — Connect Claude:**

Claude will now connect to the MCP server at `http://127.0.0.1:11426`. See the **Connecting Claude** section below.

### Mode 2: Headless Agent Control (Background Execution)

Headless mode runs Claude silently in the background. Optionally show notifications on error or success.

**Start the server:**

```bash
cd Z:\Projects\BonsaiWorkspace
cargo run -p bonsai-mcp-server -- headless --host 127.0.0.1 --port 11425 --verbose --notify-on-error
```

**Available headless flags:**

| Flag | Description |
|------|-------------|
| `--host` | Server host (default: 127.0.0.1) |
| `--port` | Server port (default: 11425) |
| `--quiet` | No console output (only log file) |
| `--verbose` | Print every action to console |
| `--notify-on-error` | Show desktop notification on error |
| `--notify-on-success` | Show notification when tasks complete |
| `--popup-on-approval` | Pause and ask before destructive operations |
| `--log-path` | Path to log file (default: bonsai-agent.log) |

---

## Part 3: Connecting Claude

### Via Claude Desktop App

Edit your Claude desktop configuration file. On Windows, this is typically at:
```
%APPDATA%\Claude\claude_desktop_config.json
```

Or wherever your Claude settings are stored. Add:

```json
{
  "mcpServers": {
    "bonsai": {
      "command": "cargo",
      "args": [
        "run",
        "-p", "bonsai-mcp-server",
        "--",
        "headless",
        "--host", "127.0.0.1",
        "--port", "11425",
        "--verbose",
        "--notify-on-error"
      ],
      "env": {
        "BONSAI_DAEMON_URL": "http://127.0.0.1:8080/api",
        "RUST_LOG": "info"
      }
    }
  }
}
```

Restart Claude. It will automatically discover all Bonsai tools and make them available.

### Via Direct HTTP

If you prefer to test without the Claude desktop app, you can call the MCP server directly:

```bash
curl -X POST http://127.0.0.1:11426/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"jsonrpc":"2.0","method":"tools/list","id":1}'
```

---

## Part 4: Using the Visual Dashboard

Once the dashboard is running and connected:

### Dashboard Features

- **Real-time Timeline**: Every tool call, error, and system event is shown in order.
- **Event Details**: Click any event card to expand and see full arguments, results, and errors.
- **Filtering**: Filter by event type (All, Tool Calls, Errors, System).
- **Auto Scroll**: Toggle to automatically scroll to the latest events.
- **Statistics**: See total events, tool calls, and error count at the top.

### Example Dashboard View

```
🧠 Bonsai Agent Control                        🟢 CONNECTED
┌─────────────────────────────────────────────────────────────┐
│ Total Events: 12 | Tool Calls: 10 | Errors: 1              │
├─────────────────────────────────────────────────────────────┤
│ ▶️  14:32:15  Starting read_file                  ▼        │
│    Arguments: {"path": "Cargo.toml"}                        │
│    → Result: {contents: "..."}                              │
│    Duration: 45ms                                            │
│                                                              │
│ ▶️  14:32:16  Starting run_cargo_check                 ▼   │
│    Arguments: {"workspace": true}                           │
│    Duration: 3200ms                                         │
│                                                              │
│ ❌  14:32:19  run_cargo_test failed: timeout              ▼ │
│    Error: Test timeout after 60 seconds                    │
│    Duration: 60100ms                                        │
└─────────────────────────────────────────────────────────────┘
```

---

## Part 5: Audit and Logging

All agent actions are logged to a structured log file. By default, this is `bonsai-agent.log` in the current directory.

### Log Format

Each line is a timestamped JSON event:

```
[2026-05-31T14:32:15.000Z] ToolCallStart: read_file
[2026-05-31T14:32:15.045Z] ToolCallEnd: read_file (45ms, success)
[2026-05-31T14:32:16.000Z] ToolCallStart: run_cargo_check
[2026-05-31T14:32:19.200Z] ToolCallEnd: run_cargo_check (3200ms, success)
```

You can replay or audit these logs later.

---

## Part 6: Destructive Operations & Approval Workflow

By default, destructive operations (write file, delete file, deploy, force push) require approval in headless mode with `--popup-on-approval`.

When Claude attempts a destructive operation:

1. The visualiser pauses execution.
2. A desktop notification appears asking for approval.
3. You can approve or deny (currently, deny prevents the operation).
4. If approved, Claude resumes.

This prevents accidental data loss.

---

## Part 7: Initial Prompt to Claude

Once Claude is connected, give it a task like:

> "You are now connected to the Bonsai Ecosystem via MCP. You have tools to read/write files, run cargo commands, chat with BonsAI, pull models, and more. Your goal is to improve the Bonsai codebase autonomously.
>
> Start by reading `Cargo.toml` to understand the workspace structure. Then run `cargo check --workspace` to verify the build. If any test fails, read the error, propose a fix, write the fix, run tests again, and commit if successful. Use the `suggest_fix` tool when stuck. All your actions are visible in the Visual Agent Control dashboard."

Claude will then autonomously:

1. ✅ Read project files
2. ✅ Run tests and cargo commands
3. ✅ Detect and fix bugs
4. ✅ Commit changes
5. ✅ All visible in real-time on the dashboard

---

## Part 8: Verification

To verify the MCP server is working correctly:

### Step 1: Check Health Endpoint

```bash
curl http://127.0.0.1:11426/health
```

Should return: `OK`

### Step 2: List Available Tools

```bash
curl -X POST http://127.0.0.1:11426/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"tools/list","id":1}'
```

Should return a list of all Bonsai tools.

### Step 3: Check WebSocket Connection (Dashboard)

Open your browser's developer console and verify the WebSocket connection:
```javascript
const ws = new WebSocket('ws://127.0.0.1:11426/ws/events');
ws.onopen = () => console.log('Connected');
```

### Step 4: Make a Test Tool Call

```bash
curl -X POST http://127.0.0.1:11426/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-token" \
  -d '{
    "jsonrpc":"2.0",
    "method":"tools/call",
    "params":{
      "name":"read_file",
      "arguments":{"path":"README.md"}
    },
    "id":1
  }'
```

If the dashboard is running, you should see the tool call appear in real-time.

---

## Part 9: Troubleshooting

### Problem: "Port already in use"

**Solution**: Change the port number:
```bash
cargo run -p bonsai-mcp-server -- visual --port 11427
```

### Problem: "WebSocket connection refused"

**Ensure the server is running and listening on the correct port:**
```bash
netstat -tuln | grep 11426  # On Linux/macOS
netstat -ano | findstr 11426  # On Windows (PowerShell)
```

### Problem: "Authorization failed"

**Solution**: Make sure your MCP client is sending the correct Bearer token in the Authorization header.

### Problem: Dashboard not showing events

**Ensure:**
1. The server is running (check terminal output)
2. WebSocket is connected (check browser console)
3. You're calling tools via the correct endpoint (`/mcp` not `/tools/call`)

### Problem: Notifications not showing

**On Windows**: Make sure `notify-rust` is available. On other OS, notifications are logged to console.

---

## Part 10: Advanced Configuration

### Custom Notification Settings

Set environment variables to customize notification behavior:

```bash
export NOTIFY_ON_ERROR=1
export NOTIFY_ON_SUCCESS=1
export LOG_PATH=/tmp/bonsai-agent.log
```

Or pass as flags:
```bash
cargo run -p bonsai-mcp-server -- headless --log-path /tmp/agent.log
```

### Running Multiple Instances

You can run multiple instances on different ports for concurrent agent operations:

```bash
cargo run -p bonsai-mcp-server -- headless --port 11425 &
cargo run -p bonsai-mcp-server -- headless --port 11426 &
```

---

## Part 11: Production Deployment

For production use:

1. **Build a release binary:**
   ```bash
   cargo build -p bonsai-mcp-server --release
   ```

2. **Run the binary directly (faster than `cargo run`):**
   ```bash
   ./target/release/bonsai-mcp-server headless --host 0.0.0.0 --port 8080 --quiet
   ```

3. **Use a systemd service or supervisor** to keep the process running:
   ```ini
   [Unit]
   Description=Bonsai MCP Server
   After=network.target

   [Service]
   Type=simple
   ExecStart=/path/to/bonsai-mcp-server headless --host 0.0.0.0 --port 8080
   Restart=always
   RestartSec=10
   StandardOutput=journal

   [Install]
   WantedBy=multi-user.target
   ```

4. **Monitor logs:**
   ```bash
   tail -f bonsai-agent.log
   ```

---

## Summary

**Claude Agent Control** enables autonomous LLM-driven development on Bonsai with:

- ✅ **Transparency** — Every action is visible in real-time or in logs
- ✅ **Safety** — Destructive operations require approval
- ✅ **Auditability** — Full JSON event logs with timestamps
- ✅ **Flexibility** — Visual or headless modes for different use cases
- ✅ **Production-Ready** — Configurable logging, notifications, and performance metrics

Start with visual mode to experiment, then switch to headless mode for automation.

---

## File Structure

```
Z:\Projects\BonsaiWorkspace
├── crates/bonsai-mcp-server/
│   ├── src/
│   │   ├── main.rs                # CLI entry point with clap argument parsing
│   │   ├── lib.rs                 # Module exports (includes visualiser)
│   │   ├── server.rs              # MCP server with visualiser integration
│   │   ├── visualiser.rs          # Visualiser proxy (NEW)
│   │   ├── tools.rs               # MCP tool definitions
│   │   ├── auth.rs                # Token authentication
│   │   └── bridge.rs              # Bridge to daemon
│   ├── Cargo.toml                 # Updated with visualiser deps
│   └── tests/
└── visualiser-ui/                 # Svelte dashboard (NEW)
    ├── src/
    │   ├── main.ts
    │   ├── App.svelte
    │   └── components/
    │       └── EventCard.svelte
    ├── index.html
    ├── vite.config.js
    ├── tsconfig.json
    └── package.json
```

---

**All code is production-ready. Start the server and begin working with Claude!** 🚀
