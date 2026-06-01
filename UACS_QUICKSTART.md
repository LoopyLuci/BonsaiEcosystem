# Universal Agent Control System (UACS) — Quick Start

Get Claude working autonomously with UACS in 3 minutes.

## 1️⃣ Build UACS

```powershell
cd Z:\Projects\BonsaiWorkspace
cargo build -p bonsai-mcp-server --release
```

## 2️⃣ Start Visual Mode

**Terminal 1 — UACS Server:**

```powershell
cargo run -p bonsai-mcp-server -- visual --port 11426
```

**Terminal 2 — Dashboard:**

```powershell
cd uacs-dashboard
npm install
npm run dev
```

Open your browser → `http://localhost:5173`

## 3️⃣ Configure Claude

Edit `%APPDATA%\Claude\claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "bonsai": {
      "command": "cargo",
      "args": ["run", "-p", "bonsai-mcp-server", "--", "visual", "--port", "11426"]
    }
  }
}
```

Restart Claude.

## 4️⃣ Give Claude a Task

> "You are connected to the Universal Agent Control System in Visual mode. Every action you take is visible on the live dashboard. Start by reading `Cargo.toml`, then run `cargo check --workspace`. If there are errors, fix them. Report all findings."

## 5️⃣ Watch It Work

The dashboard shows every action in real-time:

```
ToolCallStart: read_file (Cargo.toml) ▶️
ToolCallEnd: 45ms ✓
ToolCallStart: run_cargo_check ▶️
ToolCallEnd: 3200ms ✓
FileModified: src/main.rs 📝
TestRun: cargo test (pass) ✅
```

---

## Modes

| Mode | Use Case | Command |
|------|----------|---------|
| **Visual** | Development | `uacs visual --port 11426` |
| **Headless** | Production | `uacs headless --verbose --notify-on-error` |

---

## Files Updated

- `crates/bonsai-mcp-server/src/uacs.rs` — Core system
- `crates/bonsai-mcp-server/src/main.rs` — CLI
- `crates/bonsai-mcp-server/src/server.rs` — MCP integration
- `uacs-dashboard/` — React dashboard
- `UNIVERSAL_AGENT_CONTROL.md` — Full docs

---

**Ready to go!** Claude is now working autonomously with full visibility. 🚀
