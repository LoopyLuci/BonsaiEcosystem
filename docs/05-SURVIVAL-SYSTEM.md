# Survival System & Self-Healing

Bonsai is designed to run forever without human intervention. When something goes wrong, the Survival System detects the problem, applies a known fix, and restarts. If no known fix exists, it asks BonsAI to generate one.

---

## Philosophy

Software crashes. Config files get corrupted. Ports get stolen by other processes. Dependencies vanish. The Survival System treats all of these as *expected events* and handles them automatically:

1. **Detect** – the Watchdog monitors the daemon's health in real time.
2. **Diagnose** – match the error signature against the Survival Knowledge Base.
3. **Fix** – apply the best-matching repair script.
4. **Restart** – bring the component back online.
5. **Learn** – record whether the fix worked; update success rates.

The goal is **zero-downtime** operation. In practice, most recoveries take under 10 seconds.

---

## Watchdog Process (`bonsai-watchdog`)

The Watchdog is a tiny, dependency-free Rust binary that runs as a separate OS process. It launches the main Bonsai daemon and monitors it.

### What the Watchdog monitors
- **Process liveness** – is the daemon process still running?
- **IPC heartbeat** – does the daemon respond to a ping every 5 seconds?
- **Memory usage** – is RSS exceeding the configured limit?
- **Log patterns** – are panic messages, port conflicts, or fatal errors appearing in stderr?

### Restart policy
| Attempt | Wait before retry |
|---|---|
| 1st | 1 second |
| 2nd | 2 seconds |
| 3rd | 4 seconds |
| 4th–10th | 8 seconds |
| After 10 failures | Show SurvivalOverlay; wait for user |

Each restart attempt applies the best-available fix first, so each attempt has a higher chance of succeeding than the last.

### Starting the Watchdog
The Watchdog is embedded in the Tauri app and starts automatically. On Windows it is also registered as a startup entry so it survives reboots. You can also run it manually:

```powershell
./bonsai-watchdog --daemon-path ./bonsai-workspace.exe
```

---

## Survival Engine

The Survival Engine (`SurvivalEngine`) runs inside the main daemon. It handles errors that the daemon catches at runtime — not just crashes.

### Error Detection
The engine monitors:
- Tauri command errors (IPC failures)
- Inference errors (model load failures, OOM)
- Training job failures
- File system errors (permission denied, disk full)
- Network errors (port conflicts, connection refused)

### Rule-Based Fixes
When an error is detected, the engine queries the **Survival Knowledge Base** for matching rules. A rule looks like:

```json
{
  "symptom_pattern": "port .* already in use",
  "fix_script": "find_and_kill_port_conflict",
  "success_rate": 0.94,
  "applies_to": ["windows", "linux", "macos"]
}
```

The engine scores all matching rules by similarity and success rate, then applies the highest-scoring one.

### AI-Assisted Repairs
When no matching rule exists (or all known fixes have failed), the engine calls `ai_repair_error`:

1. The error message, stack trace, and recent logs are sent to BonsAI.
2. BonsAI generates a repair script.
3. The engine presents the script to the user for approval (can be automated with the `survival_ai_repair` feature flag).
4. If the fix succeeds, it is added to the Knowledge Base with an initial success rate of 0.5.

---

## Survival Knowledge Base

The Survival KB is a SQLite database at `~/.bonsai/survival_kb.db`. It stores all known error patterns and their fixes.

### Pre-seeded Rules (35 built-in)
| Category | Examples |
|---|---|
| Port conflicts | `port 11369 already in use` → kill zombie process |
| Config corruption | JSON parse error → restore from last backup |
| GPU OOM | CUDA out of memory → reduce GPU layers |
| Training crashes | ACCESS_VIOLATION → reduce batch size / max-length |
| Missing binaries | `llama-server` not found → re-download |
| File permissions | Permission denied → fix ownership |
| IPC timeouts | WebSocket timeout → restart IPC server |

### Adding a Rule Manually
If you fix an issue yourself, teach Bonsai about it:

```
# In Bonsai terminal:
bonsai survival learn \
  --symptom "database locked" \
  --fix "close all sqlite connections and retry" \
  --applies-to windows
```

Or click **"I fixed it manually"** in the SurvivalOverlay and describe what you did. BonsAI will formalise the rule.

### Browsing the KB
Run `bonsai survival list` to see all rules with their success rates. High-success rules are shown first. You can export the KB to JSON and share it with other Bonsai users.

---

## SurvivalOverlay

After 5 failed auto-repair attempts, Bonsai shows the **SurvivalOverlay** — a full-screen diagnostic view:

```
┌─────────────────────────────────────────────────┐
│  ⚠️  Bonsai needs your help                     │
│                                                 │
│  Error: database is locked                      │
│  5 auto-repair attempts failed                  │
│                                                 │
│  BonsAI suggests:                               │
│  "Close any other apps using SQLite and         │
│   click 'Try Again'."                           │
│                                                 │
│  [🤖 Ask BonsAI to Fix]  [✅ I Fixed It]       │
│  [📋 View Logs]          [↩ Rollback]           │
└─────────────────────────────────────────────────┘
```

**Ask BonsAI to Fix** – opens a chat with the error context pre-loaded. BonsAI proposes and optionally executes a fix.  
**I Fixed It** – dismisses the overlay and teaches Bonsai what you did.  
**Rollback** – restores the last known-good configuration from CAS checkpoint.

---

## System Health Panel

The **System Health Panel** (toggle with `Ctrl+Shift+H`) shows real-time system status:

```
┌──────────────────────────────────────────┐
│  System Health                           │
│                                          │
│  Memory:  IPC  ████████░░ 72%           │
│  CPU:     8%   ██░░░░░░░░               │
│  GPU:     34%  ████░░░░░░               │
│  VRAM:    4.2/8 GB                      │
│                                          │
│  Sidecars:  llama-server ● active       │
│             whisper      ● active       │
│             watchdog     ● active       │
│                                          │
│  IPC Health: ● connected  latency: 2ms  │
│  Last event: "epoch 3 completed"        │
└──────────────────────────────────────────┘
```

Green ● = healthy. Yellow ● = degraded. Red ● = failed.

---

## Crash Recovery

If Bonsai crashes hard (segfault, OOM kill), the next launch detects the crash flag and enters recovery mode:

1. **WAL replay** – any uncommitted Write-Ahead Log entries are replayed to recover in-progress work.
2. **State restore** – the last CAS-checkpointed `AppState` is loaded.
3. **Chat continuity** – the last chat session is restored so you can continue the conversation.
4. **Report** – a recovery report is added to the Activity Log.

To test crash recovery:
```powershell
bonsai survival simulate-crash
```

---

*← [Model Trainer](04-MODEL-TRAINER.md) · [Knowledge Database →](06-KNOWLEDGE-DATABASE.md)*
