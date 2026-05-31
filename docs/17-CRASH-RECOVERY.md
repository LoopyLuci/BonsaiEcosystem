# Crash Recovery & Rollback

Bonsai uses a multi-layer recovery system that combines WAL-based consistency, Universe time-travel snapshots, and autonomous survival rules.

## Recovery Layers

### Layer 1: WAL Replay (immediate)
On every startup, Bonsai checks for a `crash.flag` file. If present (unclean shutdown), it replays the write-ahead log to restore consistent database state before any user interaction.

### Layer 2: Universe Snapshot Rollback (time-travel)
After WAL replay, the crash recovery module queries the Universe store for the last snapshot taken before the crash timestamp. The frontend receives a `rollback_proposal` in the `recovery-state` event:

```json
{
  "crashed": true,
  "wal_replayed": true,
  "rollback_proposal": {
    "snapshot_id": "abc123...",
    "label": "Pre-swarm: bug-fixer for issue-42",
    "timestamp_ns": 1748650000000000000,
    "event_count_at_creation": 1247
  }
}
```

The user can click **"Restore to safe state"** to revert all files and configuration to the snapshot state.

### Layer 3: BotRuleEngine Auto-Rollback
The `CrashDetected` event (published on crash flag detection) triggers the `CrashDetected → Rollback` bot rule, which calls `UpgradeDispatcher::rollback` for the crashed component. For `Critical` severity crashes, this happens without user intervention.

### Layer 4: Survival Knowledge Base
The existing `SurvivalEngine` logs crash patterns, applies known fixes from the knowledge base, and records new error-fix pairs as training data via the `EternalTrainingLoop`.

## Severity Levels

| Severity | Description | Auto-Action |
|----------|-------------|------------|
| `Low` | Non-critical component, self-healed | Log only |
| `Medium` | Degraded functionality | Notify user, propose rollback |
| `High` | Core service down | Propose rollback, spawn BugFixer |
| `Critical` | System unusable | Auto-rollback, restart daemon |

## Configuration

Auto-rollback behavior is controlled by the `survival` section in `config/features.yaml`:

```yaml
survival:
  auto_rollback_on_critical: true
  rollback_grace_seconds: 10
```

With `auto_rollback_on_critical: true`, the system reverts to the last snapshot automatically 10 seconds after a `Critical` crash (giving the user time to cancel via the notification).

## Audit Trail

Every crash, recovery action, and rollback is recorded as a `SurvivalEvent` in the Universe timeline. You can review the full history in the Time Travel panel, filtered by category "SurvivalEvent".
