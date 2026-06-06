# Polyglot Pong – 750-Language Validation Framework

Polyglot Pong is a deterministic test framework that validates 750+ programming languages against a canonical fixed-point specification. It serves as both a stress-test suite and a demonstration of truly deterministic, AI-optional architecture.

---

## Purpose

### Why Polyglot Pong?

1. **Determinism Validation**: Proves that identical input produces identical output across all languages, using only fixed-point arithmetic (no floats, no randomness).

2. **Language Support Verification**: Automatically tests code generation for 750+ languages, catching bugs in BPLIS/LAIR translators.

3. **Fidelity Measurement**: Scores how closely each language implementation matches the canonical spec (0.0–1.0 fidelity).

4. **Performance Benchmarking**: Measures execution time, energy consumption, and compilation time per language.

5. **Bug Discovery**: Differential fuzzing detects edge cases where languages diverge from spec.

6. **AI-Optional Proof**: The entire framework works without any AI/ML. All intelligence is deterministic heuristics and formal specs.

---

## The Canonical Specification

### Pong Physics

Polyglot Pong implements the classic arcade game in a deterministic, language-agnostic way:

**Game State** (fixed-point 16.16):
```
struct GameState {
  ball_x:      i32,    // Ball X position (0–16384)
  ball_y:      i32,    // Ball Y position (0–12288)
  ball_dx:     i32,    // Ball X velocity per frame
  ball_dy:     i32,    // Ball Y velocity per frame
  paddle_left: i32,    // Left paddle Y (0–12288)
  paddle_right: i32,   // Right paddle Y (0–12288)
  score_left:  u32,    // Left player score
  score_right: u32,    // Right player score
}
```

**Physics Rules** (deterministic):
1. Ball moves by `(dx, dy)` each frame
2. Paddle moves ±256 units per frame (max 20 frames to traverse full height)
3. Collision with paddle: reverse `dx`, increment score
4. Collision with wall: reverse `dy`
5. Out-of-bounds ball: reset to center, increment opponent score

**Why Fixed-Point?**
- No floating-point rounding errors
- Identical results across CPU architectures
- Languages without floats (Java, Go) still work
- Bit-identical traces across all platforms

---

## Core Components

### 1. Orchestrator (`orchestrator/`)

Coordinates the test matrix:
- Loads 750 languages from manifest
- Creates 750×750 job matrix (source language → target language conversions)
- Schedules jobs via `JobScheduler` (round-robin, family-based, difficulty-based)
- Implements `SovereignService` trait with 4 execution tiers (AI → Heuristic → Deterministic → Safe Stub)
- Uses `Arbiter` for safety-clamped execution

**Files:**
- `src/main.rs` – CLI entry point
- `src/lib.rs` – Orchestrator struct, SovereignService impl
- `src/scheduler.rs` – Job scheduling algorithms
- `src/comparison.rs` – Trace comparison and fidelity scoring

### 2. Sandbox (`sandbox/`)

Generates, compiles, and executes code:
- `PongRunner` handles code generation for each language
- Template implementations for Rust, Python, JavaScript, Go, C
- Compile step produces platform-specific binaries
- Execute step captures full game trace (every frame's state)
- Integration with energy measurement (RAPL)

**Files:**
- `src/lib.rs` – Sandbox struct, SovereignService impl
- `src/runner.rs` – PongRunner, templates, compilation, execution
- `src/main.rs` – Daemon mode (listens for jobs from orchestrator)

### 3. Fuzzer (`fuzzer/`)

Discovers edge cases via differential fuzzing:
- Compares two language implementations on same seed
- Tracks divergences: CompilationFailure, RuntimeCrash, BehavioralDifference, PerformanceAnomaly
- Binary search to minimize failing cases
- Traces compared frame-by-frame to find exact divergence point

**Files:**
- `src/lib.rs` – DifferentialFuzzer, LanguageExecutor trait, trace comparison

### 4. Energy Measurement (`energy/`)

Tracks power consumption:
- Reads Intel RAPL (Running Average Power Limit) from `/sys/class/powercap/`
- Fallback estimation for non-Linux
- EnergyLeaderboard ranks languages by efficiency
- Metrics logged to dashboard

**Files:**
- `src/lib.rs` – EnergyReader, EnergyBenchmark, EnergyLeaderboard

### 5. Bug Tracker (`bug-tracker/`)

Automatic issue filing:
- When divergences are found, automatically creates bug reports
- Categorizes by severity and language
- Files issues externally (GitHub, Jira, etc.)
- Tracks fix status

**Files:**
- `src/lib.rs` – BugReporter, issue formatting, status tracking

### 6. Graph Analyzer (`graph-analyzer/`)

Language fidelity analysis:
- Builds a graph of languages, edges weighted by fidelity
- Detects clusters of similar-fidelity languages
- Computes centrality (which languages are "bridge" points)
- Identifies which languages are most divergent

**Files:**
- `src/lib.rs` – LanguageGraph, fidelity matrix, cluster detection

### 7. Dashboard (`dashboard/`)

Real-time metrics visualization:
- WebSocket server on 0.0.0.0:8080
- Broadcasts metrics every 5 seconds
- Shows: jobs completed, success rate, fidelity, latency, energy
- Multi-client support, auto-reconnect on disconnect

**Files:**
- `src/main.rs` – Axum server, WebSocket handler
- `dashboard.html` – Responsive UI (HTML/CSS/JavaScript)

### 8. Common (`common/`)

Shared types:
- `GameState` (canonical Pong state)
- `TestResult` (result of one language pair test)
- `EnergyMetrics`, `RuntimeMetrics`, `ZkProof`, `TeeAttestation`
- `BugReport` with FailureType enum
- `AggregateMetrics` for test run summaries

**Files:**
- `src/lib.rs` – Type definitions
- `src/spec.rs` – CanonicalSpec with deterministic physics, unit tests

---

## Running Polyglot Pong

### Quick Test (Single Language Pair)

```bash
cd polyglot-pong
cargo run --release --bin polyglot-pong-orchestrator -- \
  --nodes 1 \
  --rounds 1 \
  --limit 1
```

Output:
```
[INFO] Orchestrator created for 2 languages
[INFO] Starting test matrix: 4 jobs
[INFO] Completed 1/4 jobs (0 failed)
[INFO] === Final Metrics ===
[INFO] Total tests: 1
[INFO] Successful: 1 (100.0%)
[INFO] Avg fidelity: 1.000
[INFO] Avg exec time: 1234µs
[INFO] Total energy: 0.12J
```

### Medium Test (100 Languages)

```bash
cargo run --release --bin polyglot-pong-orchestrator -- \
  --manifest languages.yaml \
  --nodes 4 \
  --rounds 10
```

This runs a 100×100 matrix (10,000 jobs) distributed across 4 CPU cores.

### Full Test (750 Languages)

```bash
cargo run --release --bin polyglot-pong-orchestrator -- \
  --manifest languages.yaml \
  --nodes 8 \
  --rounds 100
```

This runs the complete 750×750 matrix (562,500 jobs) across 8 machines. Expect 4–8 hours depending on hardware.

### Dashboard Monitoring

In another terminal, start the dashboard:

```bash
cargo run --release --bin polyglot-pong-dashboard
```

Then open: `http://localhost:8080`

You'll see:
- Progress bar (% complete)
- Metrics: success rate, avg fidelity, latency, energy
- Activity log with language status
- Real-time WebSocket updates

### Fuzzing

Run differential fuzzer on a specific language pair:

```bash
cargo run --release --bin polyglot-pong-fuzzer -- \
  --source rust \
  --target python \
  --seeds 100
```

Output shows divergences found and minimized test cases.

---

## Understanding the Output

### Fidelity Score

Fidelity (0.0–1.0) measures how closely a target language matches the canonical spec:

- **1.0**: Perfect match, identical trace
- **0.9–0.99**: Minor numeric differences (rounding)
- **0.8–0.89**: Occasional divergence (off-by-one in collision detection)
- **0.7–0.79**: Moderate divergence (different physics interpretation)
- **<0.7**: Major bugs (crashes, infinite loops, wrong output)

Fidelity is computed by comparing execution traces frame-by-frame using `compare_traces()` in the `comparison` module.

### Metrics

```
Total tests: 562,500         # 750×750 language pairs
Successful: 560,000 (99.6%)  # Tests completed without error
Avg fidelity: 0.987          # Average match to spec
Avg exec time: 45,320µs      # Average execution time per test
Total energy: 12,345.67J     # Total energy consumed
Highest energy: Rust (543.2J) # Languages ranked by efficiency
Lowest energy: C (2.1J)
```

### Bug Report

When divergence detected:

```
Bug ID: bonsai-pong-rust-to-python-001
Title: Python paddle collision offset
Description: Python implementation detects collision 1 pixel earlier than spec
Severity: Medium
Reproducer: [minimized test case]
Language Pair: Rust → Python
Status: Filed (GitHub #4521)
```

---

## Configuration

### languages.yaml Manifest

```yaml
languages:
  - name: "Rust"
    family: "systems"
    version: "1.70"
    compiler: "rustc"
    flags: ["--edition", "2021"]
    
  - name: "Python"
    family: "dynamic"
    version: "3.11"
    compiler: "python3"
    flags: []
    
  # ... 748 more languages
```

Each language entry:
- `name`: Language identifier
- `family`: Language category (systems, dynamic, functional, etc.)
- `version`: Compiler/interpreter version
- `compiler`: Executable name
- `flags`: Compilation/execution flags

### Job Scheduler Options

```bash
--nodes <N>              # Number of parallel workers
--rounds <R>             # Multiplier for job count (default 1 = full matrix)
--limit <L>              # Max jobs to run (default unlimited)
--strategy <STRATEGY>    # Scheduling: "deterministic" | "family" | "difficulty"
--ai <yes|no>            # Enable AI advisor (default no)
--fuzz <yes|no>          # Enable fuzzing (default yes)
--output <FILE>          # Results file (default stdout)
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Polyglot Pong Validation

on: [push, pull_request]

jobs:
  polyglot-pong:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run Quick Polyglot Pong Test
        run: |
          cd polyglot-pong
          cargo run --release --bin polyglot-pong-orchestrator -- \
            --nodes 4 --rounds 10 --limit 100
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Build fails with "cannot find crate" | Run `cargo update` and `cargo clean` |
| Dashboard doesn't connect | Check that port 8080 is not in use; firewall may block WebSocket |
| Some languages compile fail | Check `languages.yaml` for correct compiler path |
| Fuzzer runs forever | Use `--limit` to cap job count |
| Energy metrics always 0 | Running on non-Linux? Energy measurement fallback activated |

---

## Design Philosophy

### Determinism First
- No randomness, no floating-point rounding
- 16.16 fixed-point ensures bit-identical execution
- Traces are fully reproducible with same seed

### AI-Optional
- All scheduling, fuzzing, and metrics work without any ML
- Optional AI advisor in orchestrator (safety-clamped via Arbiter)
- System is fully functional with `--no-default-features`

### Modular
- Each crate can be used independently
- Job scheduler, fuzzer, graph analyzer all standalone
- Dashboard consumes metrics via WebSocket (decoupled)

### Scalable
- Horizontal scaling: add more nodes/workers
- Job distribution via message passing
- Dashboard handles 1000+ clients

---

## Advanced Topics

### Custom Language Support

To add a new language:

1. Create template in `sandbox/src/runner.rs`
2. Add language to `languages.yaml`
3. Run orchestrator with `--manifest languages.yaml`

### Formal Verification

The canonical spec is verified:
- TLA+ model of collision physics
- Axiom proofs of determinism
- See `verified/pong_spec.ax`

### Performance Optimization

BACE (Atomic Compilation Engine) optimizations:
- Macro caching reduces recompilation
- Incremental compilation for templates
- See [BUILD.md](BUILD.md)

---

## Related Documents

- [ARCHITECTURE.md](ARCHITECTURE.md) – Full system architecture
- [DETERMINISTIC_BACKBONE.md](DETERMINISTIC_BACKBONE.md) – SovereignService trait
- [LANGUAGE_SUPPORT.md](LANGUAGE_SUPPORT.md) – 750+ languages and BPLIS
- [FORMAL_VERIFICATION.md](FORMAL_VERIFICATION.md) – TLA+ and Axiom proofs

---

**Last Updated**: 2026-06-04  
**Status**: Production-ready (v0.1.0)  
**Maintainers**: Bonsai Project  
**Questions?**: Open an issue on GitHub or post in Discussions
