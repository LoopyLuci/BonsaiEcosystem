# Claude Agent Control — Quick Start

Get Claude working on Bonsai autonomously in 2 minutes.

## Setup

### 1. Build the Server

```powershell
cd Z:\Projects\BonsaiWorkspace
cargo build -p mcp-server --release
```

### 2. Run in Visual Mode (Watch Everything)

**Terminal 1:**
```powershell
cargo run -p mcp-server -- visual --host 127.0.0.1 --port 11426
```

**Terminal 2 (optional, for a better UI):**
```powershell
cd visualiser-ui
npm install
npm run dev
# Open http://localhost:5173 in your browser
```

## Connect Claude

Edit `%APPDATA%\Claude\claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "bonsai": {
      "command": "cargo",
      "args": [
        "run", "-p", "mcp-server", "--",
        "headless",
        "--host", "127.0.0.1",
        "--port", "11425",
        "--verbose",
        "--notify-on-error"
      ]
    }
  }
}
```

Restart Claude.

## Give Claude a Task

> "You are now connected to the Bonsai Ecosystem via MCP. Start by reading `Cargo.toml`, then run `cargo check --workspace`. If there are errors, propose and implement fixes. Commit successful changes."

## Watch in Real-Time

All Claude's actions appear in the dashboard instantly:
- ✅ Tool calls with arguments
- ✅ Results and errors
- ✅ Execution time
- ✅ Full audit trail

---

## Modes

### Visual Mode (Recommended for Development)

```powershell
cargo run -p mcp-server -- visual --port 11426
# Dashboard at http://127.0.0.1:11426
```

### Headless Mode (Production)

```powershell
cargo run -p mcp-server -- headless --verbose --notify-on-error
# Logs to bonsai-agent.log
```

---

## Files Created

- `crates/mcp-server/src/visualiser.rs` — Event proxy and modes
- `crates/mcp-server/src/main.rs` — CLI with argument parsing
- `crates/mcp-server/src/server.rs` — Refactored for visualiser
- `visualiser-ui/` — Svelte dashboard (standalone web app)

See `CLAUDE_AGENT_CONTROL.md` for the full specification.

---

**Ready to go!** Claude will now work autonomously with full transparency. 🚀
