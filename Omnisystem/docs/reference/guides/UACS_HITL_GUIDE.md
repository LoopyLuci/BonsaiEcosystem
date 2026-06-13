# Universal Agent Control System — Human-In-The-Loop (HITL) Guide

The **Human-In-The-Loop (HITL)** system is a critical safety feature that pauses Claude's execution when it attempts operations that require human approval. You stay in control while the agent handles routine tasks autonomously.

---

## Overview

### What is HITL?

HITL adds **approval gates** to the agent's tool calls. When Claude attempts a high-risk operation (writing files, deploying models, accessing networks), a clear pop-up appears. You can approve or deny with one click, and the agent waits for your decision.

### Two Modes

| Mode | HITL Behavior |
|------|---|
| **Visual** | Pop-up modal in the dashboard at http://localhost:5173 |
| **Headless** | Desktop notification or terminal prompt in the console |

### Default Categories (Require Approval)

1. **Destructive** — write_file, delete_file, run_cargo_publish, git_force_push, deploy_model
2. **Network** — web_search, create_collaboration_session, get_peers, http_request

---

## Visual Mode — Dashboard Modal

### How It Works

1. Claude attempts a destructive or network operation
2. UACS emits an `AgentPaused` event
3. Dashboard shows a modal:
   ```
   🔔 UACS Approval Required
   ───────────────────────────
   Tool: write_file
   Description: Write file: src/main.rs
   Risk Level: HIGH 🔴
   ───────────────────────────
   [✅ Approve] [❌ Deny]
   ```
4. You click **Approve** or **Deny** in < 1 second
5. Agent continues or stops based on your decision

### Starting Visual Mode with HITL

```bash
# Enable HITL for destructive and network operations (default)
cargo run -p mcp-server -- visual

# Custom categories: only require approval for all operations
cargo run -p mcp-server -- visual --hitl-categories all

# Only destructive (no network approval needed)
cargo run -p mcp-server -- visual --hitl-categories destructive

# Disable HITL (fully autonomous)
cargo run -p mcp-server -- visual --no-hitl
```

### Dashboard Modal Features

- **Risk Level**: Color-coded (high=red, medium=orange, low=green)
- **Tool Name**: Which tool is being called
- **Description**: Human-readable summary of the operation
  - `Write file: src/main.rs`
  - `Delete file: target/debug`
  - `Deploy model: my-model`
  - `Search web for: Rust documentation`
- **Operation Details**: Click "View Operation Details" to see the full arguments
- **Instant Feedback**: Approve/Deny buttons respond immediately

---

## Headless Mode — Terminal or Desktop Notification

### How It Works

When Claude attempts an operation requiring approval:

1. **Try Desktop Notification First** (if supported)
   - Notification appears: `🔔 UACS Approval — write_file`
   - 30-second timeout
   - Auto-denies if no response

2. **Fallback: Terminal Prompt** (if `--fallback-terminal` is set)
   ```
   ════════════════════════════════════════════════════════
   🔔 UACS Approval Required
   ════════════════════════════════════════════════════════
   Tool:        write_file
   Description: Write file: src/main.rs
   Risk Level:  HIGH 🔴
   ────────────────────────────────────────────────────────
   Approve this operation? (y/N):
   ```
   - Type `y` + Enter to approve
   - Type anything else or just press Enter to deny

### Starting Headless Mode with HITL

```bash
# Full HITL with desktop notifications and terminal fallback
cargo run -p mcp-server -- headless --fallback-terminal

# HITL with verbose logging
cargo run -p mcp-server -- headless --verbose --fallback-terminal

# Custom categories
cargo run -p mcp-server -- headless --hitl-categories destructive,model

# Disable HITL
cargo run -p mcp-server -- headless --no-hitl

# Quiet mode (no console output, only notifications)
cargo run -p mcp-server -- headless --quiet --fallback-terminal
```

### Log File Records

All HITL decisions are logged to `uacs-agent.log`:

```
[2026-05-31T14:32:15.000Z] AgentPaused: write_file (request_id: abc123, risk: high)
[2026-05-31T14:32:18.456Z] AgentResumed: write_file (request_id: abc123, approved: true)
[2026-05-31T14:32:20.100Z] ToolCallEnd: write_file (45ms, success)
```

---

## Approval Categories

You can configure which operations require approval:

### Destructive
- `write_file` — Create/modify files
- `delete_file` — Remove files
- `deploy_model` — Deploy ML models
- `run_cargo_publish` — Publish crates
- `git_force_push` — Force-push to Git

### Network
- `web_search` — Search the web
- `create_collaboration_session` — Start remote sessions
- `get_peers` — List network peers
- `http_request` — Make HTTP calls

### Model Mutation
- `train` — Train models
- `deploy_model` — Deploy models
- `create_model` — Create new models
- `fine_tune` — Fine-tune models

### System Modification
- `network_set_firewall_profile` — Change firewall
- `network_toggle_adapter` — Enable/disable network
- `run_cargo_publish` — Publish crates

### All
- Requires approval for **every single tool call**, routine or not

---

## Example Workflows

### Development: Visual Mode with Default HITL

Best for: Testing Claude's work while maintaining control.

```bash
# Terminal 1: Start UACS Visual
cargo run -p mcp-server -- visual

# Terminal 2: Start Dashboard
cd uacs-dashboard && npm run dev

# Terminal 3: Configure Claude and give it a task
# When Claude attempts to write files, approve them in the dashboard
```

**Flow:**
1. Claude: "I'll fix the bug by updating src/lib.rs"
2. Dashboard Modal: "Write file: src/lib.rs — Risk: HIGH"
3. You: Click ✅ Approve
4. Claude: Proceeds, writes the file, logs success

### Fully Autonomous: Headless with No HITL

Best for: Overnight batch jobs where you trust Claude completely.

```bash
cargo run -p mcp-server -- headless --no-hitl --quiet
```

Claude runs **without pausing**, all actions logged to `uacs-agent.log`.

### Safe Automation: Headless with HITL + Terminal

Best for: Running Claude in the background with easy approval.

```bash
cargo run -p mcp-server -- headless --verbose --fallback-terminal
```

When Claude needs approval:
- Desktop notification appears (if supported)
- Terminal prompts: `Approve this operation? (y/N):`
- You type `y` or `N` and press Enter
- Agent continues or stops

---

## Advanced Scenarios

### Require Approval for Only Destructive Operations

Network access is trusted; only file writes need approval.

**Visual:**
```bash
cargo run -p mcp-server -- visual --hitl-categories destructive
```

**Headless:**
```bash
cargo run -p mcp-server -- headless --hitl-categories destructive --fallback-terminal
```

### Require Approval for All Operations

Maximum safety: Claude must ask permission for **everything**.

**Visual:**
```bash
cargo run -p mcp-server -- visual --hitl-categories all
```

**Headless:**
```bash
cargo run -p mcp-server -- headless --hitl-categories all --fallback-terminal
```

### Custom Category Combinations

```bash
# Destructive, network, and model mutations require approval
cargo run -p mcp-server -- visual --hitl-categories destructive,network,model

# Only system modifications require approval
cargo run -p mcp-server -- headless --hitl-categories system
```

---

## Timeout & Auto-Denial

### In Headless Mode

- Desktop notification: **30-second timeout**
- Terminal prompt: **Waits indefinitely** (you must respond)
- Auto-denies if timeout expires without response

### In Visual Mode

- Modal stays open **indefinitely**
- You must explicitly click Approve or Deny
- No auto-denial (requires human decision)

---

## Monitoring Approvals

### View Approval Log

```bash
tail -f uacs-agent.log | grep -i approval
```

Output:
```
[2026-05-31T14:32:15.000Z] AgentPaused: write_file (risk: high)
[2026-05-31T14:32:18.456Z] AgentResumed: write_file (approved: true)
[2026-05-31T14:32:15.000Z] AgentPaused: delete_file (risk: high)
[2026-05-31T14:32:20.100Z] AgentResumed: delete_file (approved: false)
```

### Count Approvals vs. Denials

```bash
# Approved
grep "approved: true" uacs-agent.log | wc -l

# Denied
grep "approved: false" uacs-agent.log | wc -l
```

---

## Risk Assessment

UACS automatically assesses operation risk:

| Category | Risk | Color |
|----------|------|-------|
| Destructive | **HIGH** | 🔴 Red |
| Network | **MEDIUM** | 🟠 Orange |
| Model Mutation | **MEDIUM** | 🟠 Orange |
| System Modification | **MEDIUM** | 🟠 Orange |
| Other | **LOW** | 🟢 Green |

The risk level appears in the modal/notification to help you decide quickly.

---

## Denying Operations

When you click **❌ Deny** or respond `N` to the terminal:

1. Agent receives: `Error: User denied the operation: write_file`
2. Event logged: `AgentResumed (approved: false)`
3. Tool call **does not execute**
4. Claude can retry or modify its approach

### What Happens Next?

Claude will typically:
- Ask why you denied it
- Propose a different approach
- Ask for clarification
- Move on to the next task

---

## Best Practices

### ✅ Do This

- **Start with HITL enabled** — Safety first
- **Use Visual mode for development** — Clear, immediate feedback
- **Review denials** — If you deny many operations, Claude's approach may need adjustment
- **Approve quickly** — Most decisions take <1 second
- **Monitor logs** — Check `uacs-agent.log` weekly to understand patterns

### ❌ Avoid This

- **Disabling HITL without reason** — Removes important safety checks
- **Auto-approving everything** — Defeats the purpose of HITL
- **Ignoring denials** — If Claude keeps proposing dangerous operations, that's a signal
- **Leaving timeouts at default** — 30 seconds may be too short in headless mode

---

## Troubleshooting

### Modal Doesn't Appear (Visual Mode)

1. Verify the dashboard is open at http://localhost:5173
2. Check browser console for WebSocket errors
3. Ensure HITL is enabled: `cargo run -- visual` (not `--no-hitl`)
4. Restart the dashboard and UACS server

### Notification Not Showing (Headless)

1. Verify `--fallback-terminal` is enabled
2. Check if terminal is visible in the background
3. On some Linux/Mac systems, notifications require explicit permission
4. Look for the terminal prompt: `Approve this operation? (y/N):`

### Agent Keeps Getting Denied

If Claude's operations are repeatedly denied:

1. Review the descriptions — Is Claude's approach reasonable?
2. Give Claude feedback: "Why don't you try a different approach?"
3. Adjust approval categories — Maybe network access doesn't need approval
4. Consider disabling HITL for routine operations only

---

## Summary

The **HITL system** ensures you stay in control while Claude works autonomously:

- ✅ **Visual Mode** — One-click approval in the dashboard
- ✅ **Headless Mode** — Terminal prompts or notifications
- ✅ **Configurable Categories** — Choose what needs approval
- ✅ **Risk Assessment** — Color-coded indicators
- ✅ **Full Audit Trail** — Every decision logged
- ✅ **Timeout Protection** — Auto-denies if no response

**Default Setup:** HITL enabled for destructive and network operations, visual modal in the dashboard, 30-second timeout in headless mode.

**Next:** Start UACS, give Claude a task, and watch it work with your oversight. 🚀
