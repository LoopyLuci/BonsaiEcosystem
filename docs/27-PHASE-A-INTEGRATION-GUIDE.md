# Phase A: Integration Guide – Complete System Wiring

**Status:** Complete ✓  
**Date:** 2026-06-01  
**Scope:** IDE → MCP → ETL → RuleRegistry → Universe  

---

## System Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                         IDE LAYER (Frontend)                      │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │ LintPanel.svelte (bonsai-workspace)                        │  │
│  │  - Display diagnostics with severity badges               │  │
│  │  - New buttons: Apply Fix | False Positive | Dismiss      │  │
│  │  - Send feedback via API calls to /api/tools/bonsai_*     │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
                               ↓ (HTTP POST)
┌──────────────────────────────────────────────────────────────────┐
│                    MCP SERVER LAYER (Backend)                     │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │ bonsai-mcp-server/src/tools.rs (Tool Registry)            │  │
│  │  - bonsai_report_false_positive                           │  │
│  │  - bonsai_dismiss_diagnostic                              │  │
│  │  - bonsai_apply_fix                                        │  │
│  └────────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │ bonsai-mcp-server/src/lint_commands.rs (Handlers)         │  │
│  │  - handle_report_false_positive()                         │  │
│  │  - handle_dismiss_diagnostic()                            │  │
│  │  - handle_apply_fix()                                      │  │
│  └────────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │ bonsai-mcp-server/src/lint_integration.rs (Processors)    │  │
│  │  - process_report_false_positive()                        │  │
│  │  - process_dismiss_diagnostic()                           │  │
│  │  - process_apply_fix()                                     │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
                               ↓ (Async call)
┌──────────────────────────────────────────────────────────────────┐
│                   ETL LAYER (Learning System)                     │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │ bonsai-etl/src/feedback.rs (FeedbackCollector)            │  │
│  │  - on_fix_applied()                                        │  │
│  │  - on_false_positive_report()                             │  │
│  │  - on_diagnostic_dismissed()                              │  │
│  │  → Stores FeedbackEvent in ETLStorage                     │  │
│  └────────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │ bonsai-etl/src/storage.rs | storage_sqlx.rs              │  │
│  │  - In-memory (dev) or SQLx SQLite (prod)                 │  │
│  │  - Persists FeedbackEvent and RuleMetrics                │  │
│  └────────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │ EternalTrainingLoop (lib.rs) - Nightly Cycle             │  │
│  │  Stage 1: Collect feedback (last 24h)                     │  │
│  │  Stage 2: Aggregate metrics by rule_id                    │  │
│  │  Stage 3: Calculate confidence (algorithm)                │  │
│  │  Stage 4: Apply updates (via adjuster)                    │  │
│  │  Stage 5: Refine patterns (mutations)                     │  │
│  │  Stage 6: Store metrics                                   │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
        ↓ (update_confidence)        ↓ (publish_event)
┌──────────────────────┐  ┌──────────────────────────────────┐
│  RULE REGISTRY       │  │  UNIVERSE EVENT BUS               │
│  (bonsai-lint)       │  │  (system_event_bus.rs)           │
│  ┌────────────────┐  │  │  ┌──────────────────────────┐    │
│  │ RuleMetadata   │  │  │  │ SystemEvent::            │    │
│  │ - confidence   │  │  │  │ RuleConfidenceUpdated    │    │
│  │ - severity     │  │  │  │ RuleMutationProposed     │    │
│  │ - enabled      │  │  │  │ EtlCycleCompleted        │    │
│  │ - last_updated │  │  │  │ DiagnosticFeedbackRcvd   │    │
│  └────────────────┘  │  │  └──────────────────────────┘    │
└──────────────────────┘  └──────────────────────────────────┘
                                       ↓
                        ┌──────────────────────────────┐
                        │  DASHBOARDS & MONITORING     │
                        │  - Lint Analytics            │
                        │  - Rule Performance          │
                        │  - ETL Pipeline Status       │
                        └──────────────────────────────┘
```

---

## Integration Points

### 1. IDE Plugin ↔ MCP Server

**File:** `bonsai-workspace/src/lib/components/LintPanel.svelte`

```typescript
// New feedback buttons added
<button on:click={() => acceptAndApplyFix(diag)} class="btn-small btn-success">
  ✓ Apply Fix
</button>
<button on:click={() => reportFalsePositive(diag, 'Not applicable')} class="btn-small btn-warning">
  ✕ False Positive
</button>
<button on:click={() => dismissDiagnostic(diag)} class="btn-small btn-dismiss">
  → Dismiss
</button>
```

**New methods:**
- `acceptAndApplyFix(diag)` → POST `/api/tools/bonsai_apply_fix`
- `reportFalsePositive(diag)` → POST `/api/tools/bonsai_report_false_positive`
- `dismissDiagnostic(diag)` → POST `/api/tools/bonsai_dismiss_diagnostic`

### 2. MCP Server ↔ ETL

**Files:**
- `crates/bonsai-mcp-server/src/tools.rs` - Tool registry with 3 new tools
- `crates/bonsai-mcp-server/src/lint_commands.rs` - Request types & handlers
- `crates/bonsai-mcp-server/src/lint_integration.rs` - Process methods

**New Tools:**
```rust
McpTool {
    name: "bonsai_report_false_positive",
    input_schema: { rule_id, file, line, explanation }
}
McpTool {
    name: "bonsai_dismiss_diagnostic",
    input_schema: { rule_id, file, line }
}
McpTool {
    name: "bonsai_apply_fix",
    input_schema: { rule_id, file, line, fix }
}
```

**Handler Flow:**
```
POST /api/tools/bonsai_report_false_positive
  → handle_report_false_positive(req)
  → LintingState.process_report_false_positive()
  → FeedbackCollector.on_false_positive_report()
  → ETLStorage.store_feedback_event()
```

### 3. ETL ↔ Rule Registry

**Files:**
- `crates/bonsai-lint/src/rules/mod.rs` - RuleRegistry with confidence tracking
- `crates/bonsai-etl/src/adjuster.rs` - Confidence adjuster with registry client
- `crates/bonsai-etl/src/lint_integration.rs` - EtlRegistryClient trait

**RuleRegistry Methods (New):**
```rust
registry.update_confidence(rule_id, confidence: f32)
registry.set_severity(rule_id, severity: Severity)
registry.set_enabled(rule_id, enabled: bool)
registry.get_metadata(rule_id) -> RuleMetadata
```

**RuleMetadata Structure:**
```rust
pub struct RuleMetadata {
    pub rule_id: String,
    pub confidence: f32,      // 0.0-1.0
    pub severity: Severity,   // Error, Warning, Hint, Note
    pub enabled: bool,
    pub last_updated: DateTime<Utc>,
}
```

**ETL → Registry Update Flow:**
```
Stage 4: Apply Confidence Updates
  ├─ RuleConfidenceUpdate { confidence, action }
  ├─ RuleConfidenceAdjuster.apply_update()
  ├─ EtlRegistryClient.apply_update()
  ├─ registry.update_confidence(new_confidence)
  ├─ registry.set_severity(new_severity)
  └─ registry.set_enabled(enabled)
```

### 4. ETL ↔ Universe Event Bus

**Files:**
- `bonsai-workspace/src-tauri/src/system_event_bus.rs` - SystemEvent enum (extended)
- `crates/bonsai-etl/src/universe_bridge.rs` - UniverseBridge
- `crates/bonsai-etl/src/events.rs` - UniverseEventEmitter (updated)

**New SystemEvent Types:**
```rust
pub enum SystemEvent {
    RuleConfidenceUpdated {
        rule_id: String,
        old_confidence: f64,
        new_confidence: f64,
        action: String,
    },
    RuleMutationProposed {
        rule_id: String,
        expected_improvement: f64,
        proposal_id: String,
    },
    EtlCycleStarted { cycle_id: String },
    EtlCycleCompleted {
        cycle_id: String,
        feedback_events: usize,
        rules_updated: usize,
        duration_ms: u64,
    },
    EtlCycleFailed { cycle_id: String, error: String },
    DiagnosticFeedbackReceived {
        rule_id: String,
        file: String,
        line: u32,
        feedback_type: String,
    },
}
```

**Event Emission Flow:**
```
UniverseEventEmitter
  → UniverseBridge (in crates/bonsai-etl)
  → SystemEventBus (in src-tauri)
  → Dashboard Listeners
```

### 5. ETL Storage Backend

**Files:**
- `crates/bonsai-etl/src/storage.rs` - In-memory (dev)
- `crates/bonsai-etl/src/storage_sqlx.rs` - SQLx SQLite (prod)
- `crates/bonsai-etl/migrations/20260601000000_init.sql` - Database schema

**Database Tables:**
- `feedback_events` - All user feedback (accepts, rejects, dismissals)
- `rule_metrics` - Aggregated metrics per rule
- `etl_cycles` - ETL cycle history and status

**Feature Flag:**
```toml
[features]
default = ["integration"]
sqlx = ["dep:sqlx"]  # Enable SQLx backend
```

---

## Data Flow Examples

### Example 1: User Applies a Fix

```
1. IDE (LintPanel.svelte)
   └─ User clicks "✓ Apply Fix" button
   └─ Call: acceptAndApplyFix(diagnostic)
   └─ POST /api/tools/bonsai_apply_fix
      {
        rule_id: "unused-import",
        file: "src/main.rs",
        line: 42,
        fix: "Remove import statement"
      }

2. MCP Server (lint_commands.rs)
   └─ handle_apply_fix(request)
   └─ Log: "Fix applied: rule=unused-import, file=src/main.rs:42"

3. MCP Integration (lint_integration.rs)
   └─ process_apply_fix(...)
   └─ Emit: DiagnosticGenerated event

4. ETL (FeedbackCollector) - Async call
   └─ on_fix_applied(
        rule_id="unused-import",
        file="src/main.rs",
        line=42,
        outcome="success"
      )
   └─ Create FeedbackEvent
   └─ Store in ETLStorage

5. Storage
   └─ INSERT feedback_events (
        event_id, DiagnosticAccepted, unused-import, src/main.rs, 42, ...
      )

6. ETL Nightly Cycle (Stage 2-4)
   └─ aggregate_metrics() → true_positives += 1
   └─ calculate_confidence() → 0.92 (high)
   └─ apply_update() → registry.update_confidence("unused-import", 0.92)

7. Rule Registry
   └─ RuleMetadata { confidence: 0.92, enabled: true, ... }

8. Universe
   └─ Emit: RuleConfidenceUpdated { confidence: 0.92, action: "promote_to_error" }
```

### Example 2: User Reports False Positive

```
1. IDE (LintPanel.svelte)
   └─ User clicks "✕ False Positive" button
   └─ POST /api/tools/bonsai_report_false_positive
      {
        rule_id: "clippy-pedantic",
        file: "src/utils.rs",
        line: 128,
        explanation: "This pattern is intentional for readability"
      }

2. MCP Server → ETL → Storage
   └─ FeedbackEvent { FalsePositiveReported, clippy-pedantic, ... }
   └─ INSERT feedback_events

3. ETL Nightly Cycle (Stage 2-4)
   └─ aggregate_metrics() → false_positives += 1
   └─ calculate_confidence() → 0.58 (medium)
   └─ recommend_action() → "demote_to_hint"
   └─ apply_update() → registry.set_severity("clippy-pedantic", Hint)

4. Rule Registry
   └─ RuleMetadata { confidence: 0.58, severity: Hint, ... }

5. Universe
   └─ Emit: RuleConfidenceUpdated { confidence: 0.58, action: "demote_to_hint" }
   └─ Emit: RuleMutationProposed { rule_id: "clippy-pedantic", improvement: 0.15 }
```

---

## Deployment Steps

### Step 1: Wire IDE Plugin

**File:** `bonsai-workspace/src/lib/components/LintPanel.svelte`

✅ **Status:** Complete
- New feedback buttons added
- API calls implemented
- Styling applied

### Step 2: Update MCP Server

**Files:**
- `crates/bonsai-mcp-server/src/tools.rs` ✅ Complete
- `crates/bonsai-mcp-server/src/lint_commands.rs` ✅ Complete
- `crates/bonsai-mcp-server/src/lint_integration.rs` ✅ Complete
- `crates/bonsai-mcp-server/src/bridge.rs` ✅ Complete

### Step 3: Extend Rule Registry

**File:** `crates/bonsai-lint/src/rules/mod.rs`

✅ **Status:** Complete

Changes:
- Added `RuleMetadata` struct with confidence tracking
- Added methods: `update_confidence()`, `set_severity()`, `set_enabled()`
- Added thread-safe RwLock for metadata storage
- Automatic metadata creation when rules are loaded

### Step 4: Integrate ETL

**Files:**
- `crates/bonsai-etl/src/adjuster.rs` ✅ Updated
- `crates/bonsai-etl/src/lint_integration.rs` ✅ Created
- `crates/bonsai-etl/src/universe_bridge.rs` ✅ Created
- `crates/bonsai-etl/src/storage_sqlx.rs` ✅ Created
- `crates/bonsai-etl/migrations/` ✅ Created

### Step 5: Wire Universe Event Bus

**File:** `bonsai-workspace/src-tauri/src/system_event_bus.rs`

✅ **Status:** Complete

Changes:
- Added 6 new SystemEvent variants for linting
- Updated type_name() match statement
- Events automatically routed to subscribers (dashboards, monitoring)

---

## Testing Workflow

### Manual Testing (After Deployment)

1. **Apply Fix Test:**
   - Open LintPanel
   - Click "▶ Lint Repository"
   - Select a diagnostic with a fix
   - Click "✓ Apply Fix"
   - Check MCP logs for handler invocation
   - Check ETLStorage for stored event

2. **False Positive Test:**
   - Select a diagnostic
   - Click "✕ False Positive"
   - Verify `on_false_positive_report()` called
   - Check event in ETLStorage

3. **Dismiss Test:**
   - Select a diagnostic
   - Click "→ Dismiss"
   - Verify `on_diagnostic_dismissed()` called

4. **ETL Cycle Test:**
   - Run ETL daemon manually: `cargo run --bin etl-daemon`
   - Verify 6 stages execute
   - Check rule metadata updated in registry
   - Verify Universe events emitted

### Integration Tests

**File:** `crates/bonsai-etl/tests/integration_tests.rs`

```bash
cargo test -p bonsai-etl

# Specific tests
cargo test -p bonsai-etl test_full_etl_cycle
cargo test -p bonsai-etl test_confidence_calculation_pipeline
cargo test -p bonsai-etl test_mutation_proposal_for_noisy_rules
```

---

## Configuration

### Environment Variables

```bash
# ETL
export BONSAI_ETL_STORAGE_URL="sqlite:///tmp/bonsai_etl.db"
export BONSAI_ETL_CYCLE_INTERVAL="86400"  # 24 hours in seconds
export BONSAI_ETL_CLEANUP_DAYS="90"       # Retain 90 days of feedback

# MCP Server
export BONSAI_MCP_PORT="8080"
export BONSAI_LINT_CONFIDENCE_THRESHOLD="0.7"
```

### Cargo Features

```bash
# Development (in-memory storage)
cargo build -p bonsai-etl

# Production (SQLx SQLite)
cargo build -p bonsai-etl --features sqlx
```

---

## Monitoring & Observability

### Key Metrics to Track

**Via Universe Event Bus:**
- `RuleConfidenceUpdated` events per day
- Average confidence change magnitude
- Rules promoted to error / demoted to hint
- ETL cycle duration and success rate

**Via ETLStorage:**
- Feedback events per day per rule
- True positive / false positive ratio
- Rule mutation proposal generation rate

**Via Rule Registry:**
- Confidence distribution across all rules
- Rules disabled due to low confidence
- Last update timestamp for each rule

### Dashboard Queries

```sql
-- Rules most changed in last 7 days
SELECT rule_id, COUNT(*) as feedback_count, AVG(confidence) as avg_confidence
FROM rule_metrics
WHERE last_updated >= NOW() - INTERVAL 7 DAY
GROUP BY rule_id
ORDER BY feedback_count DESC;

-- False positive rate by rule
SELECT rule_id, 
       false_positives,
       (false_positives / (true_positives + false_positives)) as fp_rate
FROM rule_metrics
WHERE (true_positives + false_positives) >= 10
ORDER BY fp_rate DESC;

-- ETL cycle performance
SELECT DATE(completed_at), AVG(duration_ms), COUNT(*) as cycles
FROM etl_cycles
WHERE status = 'completed'
GROUP BY DATE(completed_at);
```

---

## Troubleshooting

### Issue: Feedback Events Not Stored

**Diagnosis:**
```bash
# Check ETL logs
tail -f /tmp/bonsai-etl.log | grep "Stored feedback event"

# Check storage backend
sqlite3 bonsai_etl.db "SELECT COUNT(*) FROM feedback_events;"
```

**Solution:**
1. Verify FeedbackCollector.on_*() methods are called
2. Check ETLStorage connection string
3. Verify database migrations ran successfully

### Issue: Rule Confidence Not Updating

**Diagnosis:**
1. Check ETL daemon is running: `ps aux | grep etl-daemon`
2. Check confidence calculation in logs
3. Verify RuleRegistry.update_confidence() was called

**Solution:**
```bash
# Manually run ETL cycle
cargo run --bin etl-daemon

# Force immediate cycle (modify daemon.rs interval)
```

### Issue: Universe Events Not Received

**Diagnosis:**
1. Check SystemEventBus has listeners
2. Verify event type name in match statement

**Solution:**
1. Verify dashboard is subscribed to SystemEventBus
2. Check event serialization format

---

## Next Steps

### Phase A+ : Human Approval Workflow

- [ ] Create MCP tool: `bonsai_review_rule_mutations`
- [ ] Create MCP tool: `bonsai_adjust_rule_confidence`
- [ ] Dashboard UI for mutation approval

### Phase B: Persistent Knowledge

- [ ] Complete SQLx migration to all deployments
- [ ] Implement KDB integration for cross-project learning
- [ ] Build historical trend analysis

### Phase C: Formal Verification

- [ ] Axiom integration for rule correctness proofs
- [ ] ML-powered predictive linting
- [ ] Natural language rule generation

---

## Files Modified/Created Summary

### Created (15 files)
- `crates/bonsai-etl/src/confidence.rs` - Confidence calculation
- `crates/bonsai-etl/src/adjuster.rs` - Confidence application
- `crates/bonsai-etl/src/refiner.rs` - Pattern refinement
- `crates/bonsai-etl/src/storage.rs` - In-memory storage
- `crates/bonsai-etl/src/events.rs` - Event emission
- `crates/bonsai-etl/src/feedback.rs` - Feedback collection
- `crates/bonsai-etl/src/lint_integration.rs` - Registry client
- `crates/bonsai-etl/src/universe_bridge.rs` - Universe bridge
- `crates/bonsai-etl/src/storage_sqlx.rs` - SQLx backend
- `crates/bonsai-etl/src/bin/daemon.rs` - ETL daemon
- `crates/bonsai-etl/tests/integration_tests.rs` - 25+ tests
- `crates/bonsai-etl/README.md` - Usage guide
- `crates/bonsai-etl/migrations/20260601000000_init.sql` - DB schema

### Modified (5 files)
- `bonsai-workspace/src/lib/components/LintPanel.svelte` - IDE feedback
- `crates/bonsai-mcp-server/src/tools.rs` - Tool registry
- `crates/bonsai-mcp-server/src/lint_commands.rs` - Feedback handlers
- `crates/bonsai-mcp-server/src/lint_integration.rs` - Processors
- `crates/bonsai-lint/src/rules/mod.rs` - Rule registry with metadata
- `bonsai-workspace/src-tauri/src/system_event_bus.rs` - Universe events

### Documentation
- `docs/26-PHASE-A-IMPLEMENTATION-SUMMARY.md` - Technical overview
- `docs/27-PHASE-A-INTEGRATION-GUIDE.md` - This file
- `docs/PHASE-A-DEPLOYMENT-READY.md` - Deployment checklist
- `docs/PHASE-ROADMAP.md` - Future phases

---

**Ready for Production Deployment:** 2026-06-01 ✓
