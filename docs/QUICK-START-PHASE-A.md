# Phase A: Quick Start Guide

**Get the ETL System Running in 5 Minutes**

---

## What Is Phase A?

Phase A is the **EternalTrainingLoop (ETL)** – a self-improving linting system that learns from user feedback to continuously improve rule accuracy.

**Key Feature:** When users accept/reject/dismiss diagnostics, the system learns and automatically adjusts rule confidence and severity.

---

## Architecture in 30 Seconds

```
IDE (Apply Fix / False Positive / Dismiss)
  ↓
MCP Server (Feedback Tools)
  ↓
ETL (Collect → Analyze → Update)
  ↓
Rule Registry (Confidence Scores)
  ↓
Universe (Dashboard Observability)
```

---

## Quick Start

### 1. **Build the ETL Crate**

```bash
cd Z:\Projects\BonsaiWorkspace
cargo build -p bonsai-etl --release
```

### 2. **Run the ETL Daemon** (Nightly Cycle)

```bash
cargo run --bin etl-daemon --release
```

The daemon will:
- Run immediately on startup
- Then run every 24 hours automatically
- Process feedback from the last 24 hours
- Update rule confidence and severity
- Emit Universe events for monitoring

### 3. **Test the IDE Integration**

In the Workspace IDE:
1. Open LintPanel
2. Click "▶ Lint Repository"
3. Select a diagnostic
4. Try the new buttons:
   - **✓ Apply Fix** – Record successful fix
   - **✕ False Positive** – Report incorrect diagnostic
   - **→ Dismiss** – Ignore without action

### 4. **Check ETL Results**

```bash
# View ETL logs
tail -f /tmp/bonsai-etl.log

# Example output:
# Stage 1: Collected 542 feedback events
# Stage 2: Analyzed 87 rules
# Stage 3: Updated 23 rule confidences
# Stage 4: Applied 23 updates to registry
# Stage 5: Proposed 5 pattern refinements
# Stage 6: Stored metrics
```

---

## Core Concepts

### Confidence Score (0.0–1.0)

```
✓ >= 0.85  →  Promote to ERROR
✓ 0.70-0.85  →  Keep as WARNING
✓ 0.50-0.70  →  Demote to HINT
✓ 0.30-0.50  →  Mark EXPERIMENTAL
✗ < 0.30  →  DISABLE
```

### Feedback Loop

```
User Action (IDE)
  ↓
Stored as FeedbackEvent
  ↓
Aggregated into RuleMetrics
  ↓
Confidence Calculated
  ↓
Rule Severity Updated
  ↓
Universe Event Emitted
  ↓
Dashboard Updated
```

---

## Important Files

| File | Purpose |
|------|---------|
| `crates/bonsai-etl/src/lib.rs` | Main orchestrator (6-stage pipeline) |
| `crates/bonsai-etl/src/feedback.rs` | Capture user actions from IDE |
| `crates/bonsai-etl/src/confidence.rs` | Calculate dynamic confidence scores |
| `crates/bonsai-lint/src/rules/mod.rs` | Rule registry with metadata |
| `bonsai-workspace/src/lib/components/LintPanel.svelte` | IDE feedback UI |
| `crates/bonsai-mcp-server/src/lint_commands.rs` | Handle feedback requests |

---

## Testing

### Run Tests

```bash
# All tests
cargo test -p bonsai-etl

# Specific test
cargo test -p bonsai-etl test_confidence_calculation_pipeline

# Integration tests only
cargo test -p bonsai-etl --test integration_tests
```

### Test Coverage

- ✅ 25+ tests total (8 integration + 17 unit)
- ✅ Confidence calculation verified
- ✅ Low-confidence rule detection
- ✅ Dismissal penalty handling
- ✅ Mutation proposal generation
- ✅ Storage persistence
- ✅ Event emission

---

## Configuration

### Environment Variables

```bash
# Optional - defaults provided
export BONSAI_ETL_STORAGE_URL="sqlite:///tmp/bonsai_etl.db"
export BONSAI_ETL_CYCLE_INTERVAL="86400"  # 24 hours
export BONSAI_ETL_CLEANUP_DAYS="90"       # Retention
```

### Cargo Features

```bash
# Development (in-memory)
cargo build -p bonsai-etl

# Production (SQLite)
cargo build -p bonsai-etl --features sqlx
```

---

## Common Tasks

### View Feedback Events

```bash
# In your own code:
use bonsai_etl::ETLStorage;

let storage = ETLStorage::new();
let events = storage.get_feedback_events_since(
    chrono::Utc::now() - chrono::Duration::hours(24)
).await?;

println!("Feedback events: {}", events.len());
for event in events {
    println!("  - {}: {}", event.rule_id, event.event_type);
}
```

### Manually Run ETL Cycle

```bash
# In your code or test:
use bonsai_etl::*;
use std::sync::Arc;

let etl = EternalTrainingLoop::new(
    Arc::new(ETLStorage::new()),
    Arc::new(RuleConfidenceCalculator),
    Arc::new(RuleConfidenceAdjuster::new()),
    Arc::new(RuleRefiner::new()),
    Arc::new(UniverseEventEmitter::new()),
);

let result = etl.run_cycle().await?;
println!("Processed {} events, updated {} rules",
    result.feedback_events_processed,
    result.rules_analyzed);
```

### Check Rule Metadata

```bash
use bonsai_lint::rules::RuleRegistry;

let registry = RuleRegistry::load(Path::new("."))?;
if let Some(metadata) = registry.get_metadata("unused-import") {
    println!("Rule: {}", metadata.rule_id);
    println!("Confidence: {:.2}", metadata.confidence);
    println!("Severity: {:?}", metadata.severity);
    println!("Enabled: {}", metadata.enabled);
}
```

---

## Troubleshooting

### "ETL daemon not running"

```bash
# Start it in the background:
cargo run --bin etl-daemon --release &

# Or in a separate terminal:
cargo run --bin etl-daemon --release
```

### "Feedback events not being stored"

1. Check FeedbackCollector is called from IDE
2. Verify ETLStorage is initialized
3. Check database file exists (for SQLx backend)

### "Rule confidence not updating"

1. Run ETL cycle manually to test
2. Check confidence calculation in logs
3. Verify RuleRegistry::update_confidence() is called

---

## Next Steps

1. **Deploy to production:** See `PHASE-A-DEPLOYMENT-READY.md`
2. **Integrate with dashboards:** See `27-PHASE-A-INTEGRATION-GUIDE.md`
3. **Phase B planning:** See `PHASE-ROADMAP.md`

---

## Key Metrics to Watch

**Per ETL Cycle:**
- Feedback events processed (100–10,000+)
- Rules analyzed (10–1,000+)
- Confidence updates applied
- Refinement proposals generated
- Cycle duration (200–700ms)

**Per Rule:**
- True positives vs false positives
- Confidence trend over time
- Dismissal count
- Fix success rate

---

## References

- **Full Implementation:** `docs/26-PHASE-A-IMPLEMENTATION-SUMMARY.md`
- **Integration Details:** `docs/27-PHASE-A-INTEGRATION-GUIDE.md`
- **Deployment:** `docs/PHASE-A-DEPLOYMENT-READY.md`
- **Architecture:** `docs/22-UNIVERSAL-LINTER.md`
- **Blueprint:** `docs/25-SELF-IMPROVING-RULES-BLUEPRINT.md`

---

## Questions?

1. Check the documentation files above
2. Review test cases in `crates/bonsai-etl/tests/`
3. Read source code comments in `crates/bonsai-etl/src/`

---

**Ready to ship!** 🚀
