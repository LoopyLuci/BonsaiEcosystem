# 🧨 BEDF Implementation Plan - Complete Roadmap

**Status:** Ready for Development  
**Timeline:** Modular, can be implemented in parallel  
**Integration:** 3-tier (Static → Dynamic → AI-Fix)

---

## Quick Start: BEDF Integration with Existing Bug Hunter

Add this to `Cargo.toml` workspace:
```toml
[workspace]
members = [
    # ... existing ...
    "crates/bonsai-bedf",  # NEW
]

[dependencies]
# Fuzzing
libfuzzer-sys = "0.4"

# Concurrency testing
loom = "0.7"
shuttle = "0.7"

# Memory sanitizers (via Rust feature)
# Compile with: RUSTFLAGS="-Zsanitizer=address" cargo test

# Property testing
proptest = "1.0"

# Penetration testing
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }

# Crash analysis & deduplication
blake3 = "1"
gimli = "0.27"  # For parsing debug info from crashes
```

---

## File Structure

```
crates/bonsai-bedf/
├── src/
│   ├── lib.rs                           # Main orchestrator
│   ├── fuzzing_engine.rs                # libFuzzer + coverage-guided fuzzing
│   ├── concurrency_tester.rs            # loom + shuttle tests
│   ├── sanitizer_runner.rs              # ASAN/MSAN/TSAN/LSAN execution
│   ├── property_tester.rs               # proptest harness generator
│   ├── penetration_tester.rs            # OWASP ZAP + protocol fuzzing
│   ├── sandbox_orchestrator.rs          # Sanctum vault management
│   ├── triage_engine.rs                 # Crash dedup + fix generation
│   ├── mcp_tools.rs                     # MCP integration (6 new tools)
│   └── crash_analysis.rs                # Stack parsing, deduplication
├── Cargo.toml
├── fuzz_targets/                        # libfuzzer targets
│   ├── nexus_execution_fuzz.rs
│   ├── nexus_interop_fuzz.rs
│   ├── api_bridge_fuzz.rs
│   └── ...
├── tests/
│   └── integration_tests.rs
└── README.md                            # BEDF documentation
```

---

## Component Implementation Details

### 1. Fuzzing Engine (`fuzzing_engine.rs`)

**Key Functions:**
```rust
pub async fn fuzz_in_vault(target: &FuzzTarget, duration_secs: u64) -> Result<Vec<RawCrash>>
pub async fn auto_generate_harness(func_name: &str, module_path: &str) -> Result<String>
pub fn compute_stack_hash(stack_trace: &str) -> u64
pub async fn deduplicate_crashes(crashes: Vec<RawCrash>) -> Vec<RawCrash>
```

**Execution:**
1. Scan workspace for functions with `#[fuzz_target]` attribute
2. For each target, launch `cargo fuzz run <target>` inside Sanctum vault
3. libFuzzer instruments code with SanitizerCoverage
4. Fuzzer uses coverage feedback to guide mutations
5. Crashes are captured, deduplicated by stack hash
6. Return minimal reproducing inputs

**Time estimate:** 2 weeks

---

### 2. Concurrency Tester (`concurrency_tester.rs`)

**Key Functions:**
```rust
pub async fn loom_test(test_fn: fn(), limit: usize) -> Result<Vec<ConcurrencyBug>>
pub async fn shuttle_test(test_fn: fn(), iterations: usize) -> Result<Vec<ConcurrencyBug>>
pub async fn analyze_race_condition(bug: &ConcurrencyBug) -> RootCauseAnalysis
```

**Execution:**
1. Find all `#[tokio::test]` functions that spawn threads/tasks
2. For each test, enumerate interleavings using loom (deterministic) or shuttle (randomized)
3. If deadlock/panic/data race detected, capture minimal schedule
4. Report with AI-generated explanation

**Time estimate:** 1.5 weeks

---

### 3. Sanitizer Runner (`sanitizer_runner.rs`)

**Key Functions:**
```rust
pub async fn run_with_asan(test_cmd: &str) -> Result<Vec<MemoryError>>
pub async fn run_with_msan(test_cmd: &str) -> Result<Vec<MemoryError>>
pub async fn run_with_tsan(test_cmd: &str) -> Result<Vec<MemoryError>>
pub async fn run_with_lsan(test_cmd: &str) -> Result<Vec<MemoryError>>
pub fn parse_sanitizer_output(output: &str) -> Vec<MemoryError>
```

**Execution:**
1. Compile test suite with `RUSTFLAGS="-Zsanitizer=address"` (or other sanitizer)
2. Run inside Sanctum vault with memory limits
3. Parse sanitizer output (ASAN reports buffer overflow, use-after-free, etc.)
4. Extract location, error type, stack trace
5. Return deduplicated findings

**Time estimate:** 1 week

---

### 4. Property Tester Generator (`property_tester.rs`)

**Key Functions:**
```rust
pub async fn generate_for_function(func_name: &str, sig: &ItemFn) -> Result<String>
pub async fn run_properties(module: &str, iterations: u32) -> Result<Vec<PropertyViolation>>
pub fn extract_property_from_fn_signature(sig: &ItemFn) -> Option<PropertySpec>
```

**Execution:**
1. Inspect function signature (parameters, return type)
2. Generate property test template using `proptest`
3. For each parameter type, use `Arbitrary` impl to generate values
4. Run test with high iteration count (1000-10000)
5. If property violated, shrink to minimal counterexample
6. Report with values that trigger violation

**Time estimate:** 1 week

---

### 5. Penetration Tester (`penetration_tester.rs`)

**Key Functions:**
```rust
pub async fn scan_api(service_url: &str) -> Result<Vec<Vulnerability>>
pub async fn protocol_fuzz(protocol: &str, host: &str, port: u16) -> Result<Vec<ProtocolBug>>
pub async fn fuzz_capability_tokens() -> Result<Vec<AuthBug>>
pub async fn fuzz_injection_attacks(endpoint: &str, param: &str) -> Result<Vec<Vulnerability>>
```

**Execution:**
1. Start service under test in Sanctum vault
2. Launch OWASP ZAP in headless mode
3. Configure ZAP to scan at `http://localhost:<port>`
4. Run active scanner (fuzzes all endpoints)
5. Parse ZAP report for vulnerabilities
6. Also perform manual protocol fuzzing for custom protocols
7. Return findings with endpoint, parameter, attack payload

**Time estimate:** 2 weeks

---

### 6. Sandbox Orchestrator (`sandbox_orchestrator.rs`)

**Key Functions:**
```rust
pub async fn run_in_vault<T, F>(name: &str, config: &SandboxConfig, fn: F) -> Result<T>
pub async fn run_parallel(tests: Vec<(String, Config, Box<Fn()>)>) -> Result<Vec<Result<()>>>
pub async fn capture_core_dump(vault_id: &str) -> Result<Vec<u8>>
pub async fn rollback_to_snapshot(vault_id: &str) -> Result<()>
```

**Execution:**
1. On each test invocation, create a fresh Sanctum vault
2. Configure with resource limits (CPU, memory, time, network)
3. Run test inside vault
4. If crash, capture core dump (register state, memory)
5. Rollback vault for next test
6. Support parallel execution (N independent vaults)

**Time estimate:** 2 weeks (depends on Sanctum stability)

---

### 7. Triage Engine (`triage_engine.rs`)

**Key Functions:**
```rust
pub async fn triage_crash(raw_crash: RawCrash) -> Result<TriagedBug>
pub fn compute_stack_hash(stack: &str) -> u64
pub async fn classify_severity(crash: &RawCrash) -> Severity
pub async fn explain_with_ai(crash: &RawCrash) -> Result<String>
pub async fn suggest_fix_with_ai(explanation: &str) -> Result<String>
pub async fn validate_fix(fix: &str, input: &[u8]) -> Result<bool>
pub async fn record_to_survival_system(entry: &SurvivalEntry) -> Result<()>
pub async fn publish_to_kdb(rule: &KdbRule) -> Result<()>
```

**Execution:**
1. Compute stack trace hash (deterministic)
2. Check if already in crash database
3. Classify severity (SEGV → Critical, panic → High, etc.)
4. Ask BonsAI V2 to explain the crash
5. Ask BonsAI V2 to suggest a fix
6. Apply fix to temporary branch, re-run test
7. If no crash: record to Survival System + KDB with high confidence
8. If still crashes: record with lower confidence, flag for human review

**Time estimate:** 2 weeks

---

### 8. MCP Tools (`mcp_tools.rs`)

**New tools:**
1. `bonsai_fuzz_function` – Trigger fuzzing on a specific function
2. `bonsai_run_concurrency_tests` – Run concurrency stress tests
3. `bonsai_run_sanitizers` – Compile and run with ASAN/MSAN/TSAN
4. `bonsai_penetration_test` – Run OWASP ZAP scan
5. `bonsai_replay_crash` – Re-run a known crash to verify fix
6. `bonsai_analyze_component` – Run complete analysis suite

**Execution:**
1. Parse JSON arguments from MCP request
2. Invoke corresponding engine function
3. Stream progress/results back to MCP client
4. Return findings in JSON format

**Time estimate:** 1 week

---

## Integration with Survival System

When a bug is discovered:

```
1. Crash detected by fuzzer/sanitizer/pen-test
   ↓
2. Triage engine:
   - Computes stack hash
   - Checks if already known
   - If new: classify severity, explain, suggest fix
   ↓
3. Survival System records:
   - Component: where the bug is
   - Symptom: what happened (crash, panic, etc.)
   - Cause: why (explanation from AI)
   - Fix: suggested patch
   - Confidence: 0.5 (new) → 0.9 (fix verified)
   - Test case: input that triggers the bug
   ↓
4. Knowledge Database records:
   - Pattern: regex/AST pattern for the bug
   - Fix template: code snippet to apply
   - Affected components: where else this could occur
   - References: link to survival entry
   ↓
5. Next time similar code appears in any project:
   - Static analysis catches the pattern
   - Suggests the recorded fix
   - Blocks the merge if severity is high
```

---

## Integration with CI/CD

**.github/workflows/bedf-analysis.yml:**

```yaml
name: BEDF Dynamic Analysis

on:
  push:
    branches: [main, develop]
  schedule:
    - cron: '0 2 * * *'  # Nightly

jobs:
  dynamic-analysis:
    runs-on: ubuntu-latest-8core  # 8-core machine for parallel fuzzing
    
    steps:
      - uses: actions/checkout@v3
      - uses: rust-lang/rust-toolchain@v1
        with:
          toolchain: nightly
          
      - name: Setup fuzzing tools
        run: |
          cargo install cargo-fuzz
          sudo apt-get install -y llvm clang lld
          
      - name: Run fuzzing
        env:
          FUZZ_DURATION: ${{ github.event_name == 'schedule' && '28800' || '300' }}
        run: cargo fuzz run --release -- -max_len=2048 -timeout=10 -artifact_prefix=crashes/
        
      - name: Run sanitizers
        run: cargo +nightly test --release --features "sanitizer-all"
        
      - name: Run concurrency tests
        run: cargo test --release --features "loom,shuttle" -- --test-threads=1
        
      - name: Run penetration tests
        if: contains(github.changed_files, 'api-bridge|mcp-server')
        run: cargo run -p bonsai-bedf -- penetration-test http://localhost:11429
        
      - name: Upload findings
        if: always()
        run: |
          cargo run -p bonsai-bedf -- triage-and-report \
            --survival-system http://localhost:8080 \
            --kdb http://localhost:9000
```

---

## Success Criteria

After BEDF is fully integrated:

✅ Every commit is fuzzed before merge  
✅ Every runtime crash is automatically explained and fixed  
✅ Every memory error is caught before production  
✅ Every concurrency bug is discovered in testing  
✅ Every security vulnerability is found before deployment  
✅ Survival System has 100+ learned crash patterns  
✅ Knowledge Database has 50+ reusable rules  
✅ Mean time to discover and fix bugs: <2 hours  
✅ Regression rate: 0% (same bug never happens twice)  

---

## Parallel Development Strategy

Teams can work in parallel on:

| Component | Owner | Blocker | Est. Time |
|-----------|-------|---------|-----------|
| Fuzzing Engine | Team A | Sandbox API | 2 weeks |
| Concurrency Tester | Team B | None | 1.5 weeks |
| Sanitizer Runner | Team C | Rust nightly | 1 week |
| Property Tester | Team D | None | 1 week |
| Penetration Tester | Team E | OWASP ZAP | 2 weeks |
| Sandbox Orchestrator | Team F | Sanctum | 2 weeks |
| Triage Engine | Team G | BonsAI V2 API | 2 weeks |
| MCP Tools | Team H | All engines | 1 week |

**Critical path:** Sandbox → Triage → All engines in parallel → MCP Tools → CI/CD integration

**Total duration (critical path):** 4-5 weeks with full team

---

## Deployment Phases

### Phase 1: Foundation (Week 1-2)
- Sandbox orchestrator
- Basic fuzzing harness generator
- Crash database setup

### Phase 2: Analysis Tools (Week 3-4)
- Fuzzing engine
- Concurrency tester
- Sanitizer integration

### Phase 3: Advanced Analysis (Week 5-6)
- Property testing
- Penetration testing
- Crash analysis

### Phase 4: Integration (Week 7-8)
- Triage engine
- Survival System integration
- Knowledge Database integration

### Phase 5: Automation (Week 9)
- MCP tools
- CI/CD pipeline
- Auto-fix validation

### Phase 6: Self-Improvement (Week 10+)
- EternalTrainingLoop integration
- Fuzzing strategy optimization
- Continuous learning

---

## Success Metrics Dashboard

Monitor these KPIs weekly:

```
Crash Discovery Rate:      >50 unique crashes per 1000 fuzzer-hours
False Positive Rate:       <5% (not real bugs)
Mean Time to Fix:          <2 hours (with AI assistance)
Regression Prevention:     100% of historical bugs blocked at CI
Code Coverage Increase:    +30% per nightly fuzz session
Pen-Test Coverage:        100% of endpoints tested
Fix Validation Success:    >85% of AI-suggested fixes pass tests
```

---

## Questions & Troubleshooting

**Q: How do we handle flaky tests?**
A: BEDF runs each test multiple times; transient failures are not reported as bugs.

**Q: What if the fuzzer finds a known-good behavior, not a bug?**
A: The triage engine's human-in-the-loop (HITL) review flags questionable findings; developers can mark as false positives.

**Q: Can BEDF be used locally (not just in CI)?**
A: Yes! Developers can run `cargo bedf analyze <component>` locally before pushing.

**Q: What's the performance impact?**
A: Nightly fuzzing takes 8 hours on a 4-core machine. PR fuzzing (5 min) adds ~5 min to CI.

---

This is a complete, ready-to-implement specification. Teams can start immediately.
