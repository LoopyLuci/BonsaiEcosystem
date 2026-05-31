# Self-Building Loop

Bonsai can autonomously write code, fix bugs, run CI, deploy upgrades, and train itself — all triggered by events in the running system.

## Architecture

```
External AI / User request
        │ natural language or TestFailed event
        ▼
SystemEventBus (41 typed events)
        │ broadcast to all subscribers
        ▼
BotRuleEngine (6 default rules)
        │ matches event type → selects template
        ▼
TemplateRegistry (7 pre-built templates)
        │ instantiates SwarmSpec
        ▼
SwarmRegistry::spawn_from_template
        │ creates Universe checkpoint first
        ▼
SwarmOrchestrator (bonsai-swarm)
        │ FeatureDeveloper / BugFixer agent runs
        ▼
bonsai-ci (cargo check → test → build → sign)
        │ BuildPassed → artifact in CAS
        ▼
UpgradeDispatcher
        │ blue-green swap, 60s health check
        ▼
EternalTrainingLoop
        │ fix attempt → DPO pair → adapter update
        ▼
Better model next iteration
```

## Default Rules

| Trigger | Action |
|---------|--------|
| `TestFailed` | Spawn `bug-fixer` swarm |
| `PRMerged` | Run CI pipeline |
| `CrashDetected` | Rollback component |
| `TrainingComplete` | Hot-reload model adapter |
| `BuildFailed` | Create issue in tracker |
| `ComponentHealthDegraded` | Notify user |

Rules are evaluated with a per-rule cooldown to prevent thundering-herd loops.

## Swarm Templates

| Template | Trigger | Purpose |
|----------|---------|---------|
| `ci-validation` | `PRMerged`, `ExternalPush` | Build, test, frontend check |
| `feature-developer` | `IssueCreated` | Implement feature from description |
| `bug-fixer` | `TestFailed`, `CrashDetected` | Diagnose, patch, verify |
| `security-audit` | `ExtensionInstalled` | Review for vulnerabilities |
| `deployment` | `UpgradeReady` | Monitor health, rollback on failure |
| `training-pipeline` | `TrainingScheduled` | Full DPO/SFT pipeline |
| `dream-cycle` | Nightly cron | Consolidate memory nodes |

## Personas

**FeatureDeveloper** — reads the codebase, generates complete Rust/Svelte files, commits, runs CI. Never leaves placeholders or TODOs. Follows `cargo clippy` and `svelte-check` clean standards.

**BugFixer** — diagnoses test failures via `git bisect`, applies minimal surgical patches, adds a regression test, verifies. After 3 failed attempts, escalates with a detailed root-cause analysis.

## Pre-Task Checkpoints

Before every swarm dispatch, `spawn_swarm_from_template` emits a `CheckpointRequested` event. The Universe engine creates a full snapshot, so any autonomous changes can be reverted with one click if they go wrong.

## Training Feedback Loop

When a BugFixer successfully repairs a test failure:
1. CI passes → `BuildPassed` event emitted.
2. `UpgradeDispatcher` hot-swaps the new binary.
3. `EternalTrainingLoop` receives the cycle completion.
4. Fix attempt + success = DPO training pair added to the dataset.
5. Next training cycle produces a model better at fixing similar bugs.
