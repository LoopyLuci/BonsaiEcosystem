# Claude Agent Control — Verification Checklist

Use this checklist to verify that the Claude Agent Control system is properly set up and ready for use.

## ✅ File Creation Checklist

### Rust Source Files

- [x] `crates/bonsai-mcp-server/src/visualiser.rs` (NEW)
  - Contains: VisualMode, HeadlessConfig, VisualiserState, VisualEvent
  - Size: ~280 lines
  - Status: ✅ Created

- [x] `crates/bonsai-mcp-server/src/main.rs` (NEW)
  - Contains: CLI argument parsing with clap
  - Size: ~60 lines
  - Status: ✅ Created

- [x] `crates/bonsai-mcp-server/src/server.rs` (REFACTORED)
  - Contains: run_server, run_visual_server, run_headless_server, MCP handlers
  - Size: ~160 lines (cleaned up)
  - Status: ✅ Updated

- [x] `crates/bonsai-mcp-server/src/lib.rs` (UPDATED)
  - Added: `pub mod visualiser;`
  - Status: ✅ Updated

- [x] `crates/bonsai-mcp-server/Cargo.toml` (UPDATED)
  - Added: clap, notify-rust, chrono, headers
  - Status: ✅ Updated

### Svelte Dashboard Files

- [x] `visualiser-ui/package.json`
  - Contains: Dependencies (svelte, vite, typescript)
  - Status: ✅ Created

- [x] `visualiser-ui/vite.config.js`
  - Contains: Vite + Svelte plugin config
  - Status: ✅ Created

- [x] `visualiser-ui/tsconfig.json`
  - Contains: TypeScript configuration
  - Status: ✅ Created

- [x] `visualiser-ui/index.html`
  - Contains: HTML entry point for Svelte app
  - Status: ✅ Created

- [x] `visualiser-ui/src/main.ts`
  - Contains: Bootstrap code for Svelte app
  - Status: ✅ Created

- [x] `visualiser-ui/src/App.svelte`
  - Contains: Main dashboard component
  - Size: ~400 lines (with styles)
  - Features: Timeline, filtering, statistics, connection status
  - Status: ✅ Created

- [x] `visualiser-ui/src/components/EventCard.svelte`
  - Contains: Reusable event display component
  - Size: ~200 lines
  - Features: Expandable details, syntax highlighting, icons
  - Status: ✅ Created

- [x] `visualiser-ui/.gitignore`
  - Contains: Standard Node.js/Vite ignores
  - Status: ✅ Created

### Documentation Files

- [x] `CLAUDE_AGENT_CONTROL.md`
  - Comprehensive 300+ line specification
  - Includes: Architecture, setup, modes, troubleshooting
  - Status: ✅ Created

- [x] `QUICKSTART.md`
  - 2-minute getting started guide
  - Status: ✅ Created

- [x] `IMPLEMENTATION_SUMMARY.md`
  - Executive summary of everything built
  - Status: ✅ Created

- [x] `VERIFICATION_CHECKLIST.md` (THIS FILE)
  - Status: ✅ Created

---

## 🏗️ Build Checklist

### Rust Compilation

```bash
# Run this to verify the Rust code compiles:
cd Z:\Projects\BonsaiWorkspace
cargo check -p bonsai-mcp-server
```

- [ ] `cargo check` completes without errors
- [ ] No compiler warnings (or acceptable ones only)
- [ ] All dependencies resolve

```bash
# Build release binary (takes ~2-5 minutes)
cargo build -p bonsai-mcp-server --release
```

- [ ] Build succeeds
- [ ] Binary created at `target/release/bonsai-mcp-server.exe` (Windows) or `target/release/bonsai-mcp-server` (Unix)
- [ ] Binary is executable

### JavaScript/Node Dependencies

```bash
cd visualiser-ui
npm install
```

- [ ] `npm install` completes without critical errors
- [ ] `node_modules/` directory created
- [ ] `package-lock.json` generated

```bash
# Build the dashboard
npm run build
```

- [ ] Build succeeds
- [ ] `dist/` directory created with compiled assets
- [ ] `dist/index.html`, `dist/index.js`, etc. exist

---

## 🚀 Runtime Checklist

### Start Visual Mode Server

```powershell
cd Z:\Projects\BonsaiWorkspace
cargo run -p bonsai-mcp-server -- visual --host 127.0.0.1 --port 11426
```

- [ ] Server starts without errors
- [ ] Terminal shows: "Bonsai MCP visualiser (visual mode) listening on 127.0.0.1:11426"
- [ ] Terminal shows: "Dashboard: http://127.0.0.1:11426"

### Health Check

```bash
curl http://127.0.0.1:11426/health
```

- [ ] Returns: `OK`
- [ ] HTTP Status: 200

### List Tools

```bash
curl -X POST http://127.0.0.1:11426/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"tools/list","id":1}'
```

- [ ] Returns JSON with `result.tools` array
- [ ] Tools include expected entries
- [ ] HTTP Status: 200

### Start Svelte Dashboard

```powershell
cd visualiser-ui
npm run dev
```

- [ ] Dev server starts
- [ ] Shows: "Local: http://localhost:5173"
- [ ] Browser opens automatically

### Dashboard UI Verification

Open `http://localhost:5173` in browser:

- [ ] Dashboard loads without JavaScript errors
- [ ] Shows "Bonsai Agent Control" title
- [ ] Shows connection status indicator (initially red/disconnected)
- [ ] Shows filter dropdown (All, Tool Calls, Errors, System)
- [ ] Shows statistics section (Total Events, Tool Calls, Errors)
- [ ] Shows "Waiting for events..." message
- [ ] No console errors in browser DevTools

### WebSocket Connection

Open browser DevTools console and run:

```javascript
const ws = new WebSocket('ws://127.0.0.1:11426/ws/events');
ws.onopen = () => console.log('✅ Connected');
ws.onerror = (e) => console.error('❌ Error:', e);
ws.onmessage = (e) => console.log('📨 Event:', JSON.parse(e.data));
```

- [ ] Shows "✅ Connected"
- [ ] No error messages
- [ ] Ready to receive events

### Make a Test Tool Call

```bash
curl -X POST http://127.0.0.1:11426/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-token" \
  -d '{
    "jsonrpc":"2.0",
    "method":"tools/call",
    "params":{"name":"read_file","arguments":{"path":"Cargo.toml"}},
    "id":1
  }'
```

- [ ] Returns JSON response (success or error is OK)
- [ ] Dashboard shows the tool call in real-time
- [ ] Timeline shows event card with "read_file" tool name
- [ ] Event card shows duration (milliseconds)

---

## 🔧 Configuration Checklist

### Claude Desktop Configuration

Edit `%APPDATA%\Claude\claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "bonsai": {
      "command": "cargo",
      "args": [
        "run", "-p", "bonsai-mcp-server", "--",
        "headless", "--host", "127.0.0.1", "--port", "11425",
        "--verbose", "--notify-on-error"
      ]
    }
  }
}
```

- [ ] JSON is syntactically valid (use a JSON validator)
- [ ] File exists at the correct path
- [ ] Restart Claude desktop app after editing

### Claude Tool Discovery

After restarting Claude:

- [ ] Claude recognizes the MCP server connection
- [ ] "Bonsai" tools appear in Claude's tool menu
- [ ] Tools include expected commands (read_file, write_file, run_cargo_check, etc.)

### Headless Mode Test

```powershell
cargo run -p bonsai-mcp-server -- headless \
  --host 127.0.0.1 --port 11425 \
  --verbose --notify-on-error --log-path bonsai-agent.log
```

- [ ] Server starts successfully
- [ ] Shows: "Bonsai MCP server (headless) listening on 127.0.0.1:11425"
- [ ] `bonsai-agent.log` file created
- [ ] Log file contains timestamps and events

---

## 📋 Feature Verification

### Visual Mode Features

- [ ] Real-time event streaming via WebSocket
- [ ] Timeline displays all events in chronological order
- [ ] Event filtering works (switch between All/Calls/Errors/System)
- [ ] Click event card to expand details
- [ ] Expanded view shows args, results, and errors
- [ ] Duration is displayed (e.g., "45ms")
- [ ] Tool name is clearly visible
- [ ] Error events have red styling
- [ ] Auto-scroll toggle works
- [ ] Clear button removes all events
- [ ] Statistics update in real-time

### Headless Mode Features

- [ ] Server runs without UI
- [ ] Events logged to file (default: bonsai-agent.log)
- [ ] `--verbose` flag prints actions to console
- [ ] `--quiet` flag suppresses console output
- [ ] Log format includes timestamps (ISO 8601)
- [ ] Log entries are parseable JSON
- [ ] `--log-path` flag changes log file location

### Approval Workflow

- [ ] `--popup-on-approval` flag is recognized
- [ ] Destructive tools (write_file, delete_file) are detected
- [ ] Approval pause mechanism works
- [ ] Desktop notification appears (if enabled)

### CLI Argument Parsing

```bash
cargo run -p bonsai-mcp-server -- --help
```

- [ ] Shows help text
- [ ] Lists all available subcommands (visual, headless)
- [ ] Shows all flags for each mode

```bash
cargo run -p bonsai-mcp-server -- visual --help
cargo run -p bonsai-mcp-server -- headless --help
```

- [ ] Shows mode-specific help
- [ ] Lists all available flags with descriptions

---

## 🧪 Integration Checklist

### Claude Autonomous Operation

After configuring Claude and giving it a task:

- [ ] Claude recognizes Bonsai MCP tools
- [ ] Claude can read files (read_file tool)
- [ ] Claude can run cargo commands (run_cargo_check, etc.)
- [ ] All Claude actions appear in dashboard (visual mode)
- [ ] All Claude actions logged (headless mode)
- [ ] Errors are properly captured and displayed

### Event Timeline Quality

- [ ] Events appear in correct chronological order
- [ ] Timestamps are accurate and consistent
- [ ] Duration calculations are correct (end time - start time)
- [ ] Event details are complete (args, results, errors)
- [ ] No events are lost or duplicated
- [ ] WebSocket connection is stable over time

### Error Handling

- [ ] Invalid JSON in request → proper error response
- [ ] Missing auth header → 401 Unauthorized
- [ ] Unknown tool → proper error message
- [ ] Tool execution error → error logged and displayed
- [ ] Server gracefully handles disconnect/reconnect

---

## 📊 Performance Checklist

### Dashboard Performance

- [ ] Page loads in <2 seconds
- [ ] Timeline updates appear within 100ms of event
- [ ] No lag when filtering events
- [ ] No memory leaks (check DevTools > Memory)
- [ ] Handles 100+ events without slowdown
- [ ] Smooth scrolling and animations

### Server Performance

- [ ] Tool call latency <500ms (excluding tool execution)
- [ ] WebSocket event delivery <50ms latency
- [ ] No memory growth over time (check with `top` or Task Manager)
- [ ] CPU usage <10% at idle

---

## 🚨 Troubleshooting Checklist

If you encounter issues, verify:

### Port Issues

```powershell
# Check if port 11426 is in use
netstat -ano | findstr 11426
```

- [ ] Port is free or reassign to different port
- [ ] Use `--port` flag to change port number

### Module Not Found Errors

```bash
cargo build -p bonsai-mcp-server
```

- [ ] All dependencies resolve
- [ ] `visualiser.rs` is in `src/` directory
- [ ] `lib.rs` exports the visualiser module
- [ ] No circular dependencies

### Dashboard Won't Connect

- [ ] Server is actually running (check terminal)
- [ ] Port number matches between server and dashboard
- [ ] Check browser console for WebSocket errors
- [ ] Verify firewall isn't blocking connection

### Claude Can't See Tools

- [ ] MCP server is running on configured port
- [ ] Claude config JSON is syntactically valid
- [ ] Claude desktop app was restarted after config change
- [ ] Check Claude's Settings > Advanced for MCP errors

---

## ✅ Final Verification

When all checkboxes are complete, the system is fully operational:

1. **Rust code compiles** ✅
2. **JavaScript dashboard builds** ✅
3. **Server starts in both modes** ✅
4. **Dashboard connects and shows events** ✅
5. **Claude can invoke tools** ✅
6. **Events are properly logged/displayed** ✅
7. **Approval workflow works** ✅
8. **Performance is acceptable** ✅

---

## 📞 Next Steps

### If Everything Works

Congratulations! The Claude Agent Control system is fully operational.

**Next:**
1. Give Claude a real task
2. Watch it work in the dashboard
3. Monitor the event timeline
4. Check logs for audit trail

### If Something Fails

1. Re-read the error message carefully
2. Check the relevant section in `CLAUDE_AGENT_CONTROL.md`
3. Verify all files were created (see File Creation Checklist)
4. Ensure dependencies are installed (Rust + Node)
5. Try the troubleshooting steps above

---

## 📄 Related Documentation

- **QUICKSTART.md** — 2-minute setup guide
- **CLAUDE_AGENT_CONTROL.md** — Full specification
- **IMPLEMENTATION_SUMMARY.md** — What was built

---

**Status:** Ready for verification ✅

**Last Updated:** 2026-05-31

**System Version:** 1.0.0 Production

Good luck! 🚀
