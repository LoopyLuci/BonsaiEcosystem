# Claude Code Adaptation – Design Summary

**Full Design:** See [CLAUDE_CODE_ADAPTATION.md](CLAUDE_CODE_ADAPTATION.md)

---

## Quick Overview

The Claude Code to Bonsai Integration Layer replaces Anthropic's CLI-based protocol with a **Bonsai-native MCP HTTP API**, enabling:

- **Local Inference**: Run code edits on-device (Qwen-7B) instead of cloud roundtrips
- **Enhanced @-mentions**: `@knowledge:rust-patterns`, `@agent:code-reviewer`, `@module:auth`
- **Time-Travel Debugging**: Universe snapshots instead of simple checkpoints
- **Cross-Device Sync**: Sessions sync between desktop and mobile (Bonsai Buddy)
- **Full Extensibility**: Custom tools, models, and approval policies

---

## Architecture at a Glance

```
┌─────────────────────────────────┐
│  VS Code Extension (TypeScript) │
│  Claude Code Panel (Svelte)     │
└────────────┬────────────────────┘
             │ HTTP with Bearer token
             ▼
┌─────────────────────────────────────────┐
│  Bonsai MCP Server (HTTP)               │
│  POST /sessions/{id}/edit               │
│  GET /sessions/{id}/checkpoint/list     │
│  POST /sessions/{id}/mention-resolve    │
└────────────┬────────────────────────────┘
             │ Tauri IPC
             ▼
┌─────────────────────────────────────────┐
│  Bonsai Workspace Backend (Rust)        │
│  Model Orchestrator, KDB, Universe      │
└─────────────────────────────────────────┘
```

---

## Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **HTTP over stdio** | Enables mobile clients, better error handling, easier versioning |
| **MCP-based protocol** | Integrates with Bonsai's existing tool ecosystem |
| **Universe as source of truth** | Provides time-travel debugging, audit logging, crash recovery |
| **Per-operation approval** | Security: user explicitly approves file writes, git commits, etc. |
| **KDB for @knowledge** | Semantic search over traditional full-text; scales to large codebases |
| **Incremental diffs** | Only show what changed (fast); user accepts/rejects per hunk |
| **SQLite for sessions** | Local persistence + Echo sync for cross-device support |

---

## Protocol Endpoints (MVP)

| Method | Endpoint | Purpose |
|--------|----------|---------|
| POST | `/sessions` | Create new Claude Code session |
| POST | `/sessions/{id}/edit` | Request code changes (main endpoint) |
| POST | `/sessions/{id}/approve` | Approve pending diffs before applying |
| POST | `/sessions/{id}/checkpoint/create` | Create a checkpoint (mapped to Universe snapshot) |
| GET | `/sessions/{id}/checkpoint/list` | List all checkpoints with timeline |
| POST | `/sessions/{id}/checkpoint/restore` | Revert to checkpoint (triggers Universe rollback) |
| POST | `/sessions/{id}/mention-resolve` | Resolve @file, @knowledge, @agent mentions |
| POST | `/sessions/{id}/exec` | Run terminal commands (sandbox via MCP) |

---

## Feature Parity Checklist

- ✅ **Edit Requests** – Prompt → Unified Diff (identical to original)
- ✅ **Diffs** – Inline diff editor with accept/reject per hunk
- ✅ **Checkpoints** – Manual + automatic creation, stored in Universe
- ✅ **Timeline** – Visual scrubber showing all checkpoints + file changes
- ✅ **@-mentions** – Original `@file`, `@symbol` + new `@knowledge`, `@agent`
- ✅ **Session Persistence** – JSON stored locally + Echo sync for cross-device
- ✅ **Terminal Integration** – Subprocess execution via `sandboxed_shell_exec`
- ✅ **Approval** – Risk-scored operations with quick/detailed approval modals
- ✅ **Error Recovery** – Atomic writes, crash detection, rollback support
- ✅ **Extensibility** – Custom tools, models, approval policies

---

## Data Format

**Session** (locally stored in `~/.bonsai/claude-code/sessions/`):
```json
{
  "version": "2.0",
  "session_id": "sess_abc123",
  "workspace_path": "/path/to/project",
  "model_config": { "model": "qwen-7b", "temperature": 0.7 },
  "message_history": [ ... ],
  "diffs": [ ... ],
  "checkpoints": [ ... ],
  "universe_sync": { "last_sync_event_id": "evt_xyz" }
}
```

**Session on Bonsai Side** (stored in SQLite w/ Echo sync):
- Replicated to `~/.bonsai/claude-code/sessions.db`
- Synced to other devices via Echo/CRDT
- Immutable copy backed by Universe events

**Checkpoint** (maps to Universe Snapshot):
- BLAKE3-verified file snapshots
- Created automatically before swarms/edits or manually by user
- Restorable with one click; triggers Universe rollback

---

## Migration from Claude Code CLI

**Auto-detection:**
```
User opens Bonsai workspace
→ "Migrate existing Claude Code sessions?" banner
→ Bonsai reads ~/.claude/sessions/*.json
→ Transforms to v2.0 format + creates Universe snapshots
→ Sessions appear in new UI immediately
```

**Backward Compatibility:**
- Old `~/.claude/sessions/` still works if kept
- User can import selectively or all at once
- No data loss (checkpoints preserved as Universe snapshots)

---

## Performance Targets

| Operation | Latency | Notes |
|-----------|---------|-------|
| Code completion | < 100ms | Local Qwen-7B inference |
| Diff generation | < 500ms | Incremental diff algorithm |
| Checkpoint restore | < 1s | Universe snapshot query + file apply |
| Timeline render | < 500ms | SQLite query of < 50 events |
| @mention resolve | < 2s | File instant, KDB ~500ms, Agent timeout 5s |

---

## Extension Points (v2.1+)

**Custom Tools:**
```json
POST /tools/register
{
  "name": "my_linter",
  "description": "Run my custom linter",
  "handler": { "type": "subprocess", "command": "my-linter" }
}
```

**Model Switching:**
```json
POST /sessions/{id}/model
{
  "model": "llama2-7b"  // Switch mid-session
}
```

**Approval Policies:**
```json
POST /sessions/{id}/approval-policy
{
  "rules": [
    {
      "condition": "path.startsWith('tests/')",
      "action": "auto_approve"
    }
  ]
}
```

---

## Open Questions

1. **Mobile**: Should Bonsai Buddy be able to continue Claude Code sessions?
2. **Collaboration**: Multi-user editing on same session (CRDT-based)?
3. **Offline**: Full offline queueing of requests + eventual sync?
4. **Custom Mentions**: Let users define new mention types dynamically?

---

## Success Metrics

- **Feature Parity**: 100% (all original features + Bonsai enhancements)
- **Migration**: 0% data loss, < 5 minutes per workspace
- **Performance**: 20x faster than cloud (local inference)
- **Reliability**: 99.9% session continuity
- **UX**: Approval workflows < 2 seconds
- **Adoption**: 80% of Claude Code users migrate within 6 months

---

**Document Status:** Design complete. Ready for implementation planning.

See the full design at [CLAUDE_CODE_ADAPTATION.md](CLAUDE_CODE_ADAPTATION.md).
