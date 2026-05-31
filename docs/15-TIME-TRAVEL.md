# Time Travel Debugging

Bonsai records every state change in an append-only, cryptographically verifiable **Universe Ledger**. You can revert any aspect of your workspace — files, configuration, models, agent decisions — to any previous point in time.

## How It Works

Every mutation produces a `UniverseEvent` stored in a local SQLite database (`universe.db`). Events are captured from:

| Source | Event Category | Examples |
|--------|---------------|---------|
| File writes/deletes | `FileChange` | Editor saves, agent file writes |
| Settings changes | `ConfigChange` | Feature toggles, model selection |
| Training phases | `ModelChange` | Phase start/complete, adapter promotion |
| Swarm activity | `SwarmEvent` | Agent spawned, task completed, swarm failed |
| CI pipeline | `AgentAction` | Build passed/failed, tests run |
| Extensions | `ExtensionEvent` | Install, update, uninstall |
| Credits | `CreditTransaction` | Earned, spent |
| Crashes | `SurvivalEvent` | Crash detected, auto-resolved |
| Snapshots | `Checkpoint` | Periodic, pre-swarm, manual |

Every event carries:
- `event_id` — BLAKE3 hash of event contents (tamper-evident)
- `timestamp_ns` — nanosecond precision
- `before_hash` / `after_hash` — BLAKE3 hashes of content before and after the change
- `source` — who triggered it (user, agent, system, automation)
- `parent_event_ids` — causal chain

## Timeline Panel

Open with the **⏱ Time Travel** toolbar button.

**Timeline tab:** Scrollable list of all events, filtered by category or file path. Click any event to see before/after hashes and a revert preview.

**Snapshots tab:** Full-state checkpoints. Create manually with "📸 Create Snapshot" or let the engine create them automatically every 5 minutes and before every swarm task.

## Reverting

1. Select an event or snapshot in the panel.
2. Click **"⏪ Preview revert"** to see exactly what files and configs would change.
3. Click **"Revert"** to confirm. A `Reversion` event is recorded in the timeline.

## Retention & Storage

Configured in `config/time_travel.yaml`. Defaults:

| Category | Retention |
|----------|-----------|
| File changes | 30 days |
| Agent actions | 7 days |
| Survival events | 90 days |
| Collaboration events | 24 hours |
| Credit transactions | Forever |
| Model changes | Forever |
| Snapshots | Last 1,000 |

The pruning scheduler runs every hour and removes events older than their retention window, keeping storage within `max_storage_gb` (default: 100 GB).

## Crash Recovery

When Bonsai detects an unclean shutdown via the crash flag, it:
1. Replays the WAL to restore consistent database state.
2. Queries the Universe store for the last snapshot taken before the crash.
3. Emits a `recovery-state` frontend event with a `rollback_proposal` containing the snapshot ID and label.

The frontend shows a "Restore to safe state?" modal. Clicking **Restore** calls `revert_confirm` and creates a post-revert checkpoint.
