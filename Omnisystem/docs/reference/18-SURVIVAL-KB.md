# Survival Knowledge Base & Forced Failure Finder

The Bonsai Survival System has three interlocking parts:

1. **Survival Knowledge Base (SKB)** — a SQLite store of 60+ error patterns and their verified fixes
2. **Forced Failure Finder (F³)** — a proactive fuzzing engine that hunts for errors before users encounter them
3. **Sandbox Nervous System (SNS)** — zero-trust isolation for every process, feeding violations back to the KB

## Survival Knowledge Base

### Schema

```sql
fixes (
    id              INTEGER PRIMARY KEY,
    error_pattern   TEXT,    -- substring matched against log output
    solution_type   TEXT,    -- "rule" | "ai" | "historical" | "f3_discovered"
    solution_script TEXT,    -- fix instructions or shell command
    confidence      REAL,    -- 0.0–1.0
    usage_count     INTEGER,
    success_count   INTEGER,
    created_by      TEXT,    -- "system" | "user" | "f3_orchestrator" | "agent"
    verified        INTEGER, -- 1 = human-verified
    category        TEXT,    -- "rust" | "svelte" | "build" | "training" | "runtime"
    tags            TEXT,    -- comma-separated
    created_at      DATETIME
)
```

### Seeded Patterns (60+)

Categories of pre-seeded fixes from this project's development history:

| Category | Count | Examples |
|----------|-------|---------|
| `svelte` | 14 | Type assertion in template, A11y, onMount async |
| `rust` | 16 | Raw string termination, log→tracing, non-exhaustive match |
| `build` | 8 | cargo PATH, LLAMA_CPP_PATH, rusqlite conflict |
| `training` | 6 | ACCESS_VIOLATION, segfault, HF offline |
| `runtime` | 7 | Port collision, OOM, panic at unwrap |

### Adding Rules

**Via script (one-time import):**
```bash
python scripts/import_historical_errors.py --db ~/.bonsai/survival_kb.db
python scripts/import_historical_errors.py --from-git   # scan git fix: commits
```

**Via API (Tauri command):**
```typescript
await invoke('survival_import_fix', {
  errorPattern: 'my error message',
  solutionScript: 'the fix command',
  category: 'rust',
  confidence: 0.9,
});
```

**Via watchdog (automatic):**
F³ workers and the log watcher automatically add newly discovered errors with `confidence = 0.7` and `verified = false`. A human or the BugFixer swarm verifies them.

## Training Data Export

The SKB doubles as a training dataset for BonsAI and specialist models:

```bash
# Supervised fine-tuning (chat format)
python scripts/export_survival_training_data.py --format sft

# DPO pairs (chosen = correct fix, rejected = no fix)
python scripts/export_survival_training_data.py --format dpo

# Instruction tuning
python scripts/export_survival_training_data.py --format instruct

# All three formats at once
python scripts/export_survival_training_data.py --format all

# Filter by category
python scripts/export_survival_training_data.py --format sft --category rust

# Statistics
python scripts/export_survival_training_data.py --stats
```

Output files go to `~/.bonsai/training_export/` and can be fed directly to the training scripts.

## Forced Failure Finder (F³)

F³ proactively breaks every Bonsai component before users can encounter the bugs.

### Pre-built Campaigns

| Campaign | Targets | Strategy |
|----------|---------|---------|
| `tauri_filesystem` | write_file, read_file, delete_file | Input fuzzing + property-based |
| `swarm_agent` | Swarm lifecycle commands | State fuzzing |
| `crdt` | CRDT document operations | Property-based (convergence invariants) |
| `resource` | Sandboxed code execution | Resource exhaustion |

### Starting a Campaign

**From UI:** Click the 🔨 F³ toolbar button → select a preset → click Start.

**From BTI:**
```
:f3 start tauri_filesystem
:f3 status
:f3 results
:f3 stop <campaign-id>
```

**From Tauri command:**
```typescript
const id = await invoke('fff_start_preset', { preset: 'tauri_filesystem' });
```

### Campaign Config (`config/failure_finder.yaml`)

Controls which campaigns run, on what schedule, and with what resource limits. The `schedule: "idle"` mode runs only when CPU usage is below 20%.

### Failure Flow

When F³ discovers a crash:
1. **Deduplicate** — BLAKE3 fingerprint of the top stack frames prevents duplicate entries.
2. **Persist to SKB** — inserted with `confidence = 0.7`, `verified = false`.
3. **Create Issue** — if `auto_create_issue: true`, an entry appears in the Issue Tracker.
4. **BugFixer** — if `auto_fix_attempt: true`, a BugFixer swarm tries to repair it.
5. **Timeline** — a `SurvivalEvent` appears in the Universe Timeline.

## Sandbox Nervous System (SNS)

Every Bonsai component runs in an isolated sandbox with a signed Capability Token.

### Isolation Tiers

| Tier | Technology | Use Cases |
|------|-----------|----------|
| Wasm | wasmtime | UI panels, tools, extensions, agent code |
| Process | namespaces + seccomp | Trusted daemons, watchdog |
| Container | gVisor | Training scripts, model servers |
| MicroVm | Firecracker/KVM | Untrusted extensions, F³ workers |

### Capability Tokens

Each sandbox receives a signed `CapabilityToken` declaring:
- **filesystem**: which paths it may read/write
- **network**: None / LocalOnly / Whitelist / All
- **allowed_peers**: which other sandbox IDs it may communicate with
- **resource_limits**: CPU%, RAM, disk, network, timeout
- **signature**: BLAKE3 of all fields, signed by the Supervisor

Any attempt to exceed declared capabilities is blocked and recorded as a `CapabilityViolation` in the SKB.

### Viewing Sandboxes

**From UI:** Click the 🛡 SNS button → Sandbox List → see all running sandboxes, their tier, capabilities, and violation count.

**From Tauri:**
```typescript
const sandboxes = await invoke('sns_list_sandboxes');
const violations = await invoke('sns_list_violations');
```
