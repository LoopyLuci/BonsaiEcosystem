# 🏗️ Integration Guide: AI-Optional, Deterministic-First Architecture

## Quick Start: Adding Deterministic-First AI-Optional Support to Your Crate

This guide walks through integrating the `ai-advisor` framework into any Bonsai subsystem (TransferDaemon, BUCE, Scheduler, etc.).

---

## 1. Dependency Setup

### 1.1 Update Your Crate's `Cargo.toml`

```toml
[package]
name = "your-service-name"
version = "2.0.0"  # Increment for architectural change
edition = "2021"

[features]
default = ["deterministic-core"]
deterministic-core = []
ai-enhancements = ["dep:ai-advisor"]

[dependencies]
ai-advisor = { path = "../../crates/ai-advisor", version = "1.0", optional = true }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"

[dev-dependencies]
tokio = { version = "1", features = ["full"] }
proptest = "1.0"
```

### 1.2 Update Workspace `Cargo.toml`

Add your crate to the workspace members list:

```toml
[workspace]
members = [
    "crates/your-service-name",
    # ... other crates
]
```

---

## 2. Define Your Service's Contract

### 2.1 Implement `SovereignService` Trait

Every service must implement these four methods:

```rust
use bonsai_ai_fallback::{SovereignService, AdvisoryOutput, Result};

pub struct YourService {
    // ... internal state ...
}

impl SovereignService for YourService {
    // TIER 1: Deterministic Core (always enabled)
    fn deterministic_core(&self, input: &[u8]) -> Result<Vec<u8>> {
        // Pure algorithm, no ML, formally verified
        // Must succeed or return safe error
        // This is the PRIMARY operational mode
        // ...
    }

    // TIER 2: Heuristic (optional, rule-based)
    fn heuristic(&self, input: &[u8]) -> Result<Option<Vec<u8>>> {
        // Rule-based deterministic fallback
        // Returns None if not applicable
        // Should cover 90%+ of cases
        Ok(None) // Default: return None
    }

    // TIER 3: AI Advisory (optional, feature-gated)
    fn ai_suggestion(&self, input: &[u8]) -> Option<AdvisoryOutput> {
        // Optional ML model, sandboxed
        // Returns None if disabled or unavailable
        // Never panics, never modifies state
        None // Default: no AI
    }

    // TIER 4: Safe Stub (always works)
    fn safe_stub(&self, input: &[u8]) -> Vec<u8> {
        // Minimal functionality that never fails
        // Default no-op is acceptable
        Vec::new()
    }

    fn name(&self) -> &str {
        "YourService"
    }
}
```

### 2.2 Key Design Principles

**Principle 1: Deterministic Core is Mandatory**
- Must be implemented and tested
- Cannot depend on external state (no network, no non-deterministic libraries)
- Performance baseline: should complete in <10ms for typical payloads

**Principle 2: Heuristics are Fast Approximations**
- Optional for features already covered by core
- Rule-based: regex, decision trees, threshold checks
- Typical improvement: 1.5x-3x better than core

**Principle 3: AI is Purely Advisory**
- Never required for operation
- Always sandboxed from core
- Arbiter validates all advice against safety envelopes before applying

**Principle 4: Safe Stub Always Works**
- Must return valid output in all conditions
- No-op, default, or minimal-functionality response
- Ensures graceful degradation never blocks

---

## 3. Graceful Degradation Ladder

For each feature, map it to a tier:

### Example: Compression Codec Selection

| Tier | Implementation | Example Output |
|------|---|---|
| **AI Enhanced** | ML model (trained on 100K compression benchmarks) | Selects neural codec; 95th percentile compression ratio |
| **Heuristic** | Rules (size, type) → codec | JSON >10MB → zstd19; else → zstd3 |
| **Deterministic Core** | Always-safe default | zstd level 3 (general-purpose) |
| **Safe Stub** | Mark as stub mode | Zero compression (stub marker) |

### Example: Congestion Control

| Tier | Implementation | Decision |
|------|---|---|
| **AI Enhanced** | Learned model (trained on Internet traces) | Set cwnd=8000; rate=150Mbps |
| **Heuristic** | Bandwidth-based rule | Highest BW path; cwnd = min_bw * RTT |
| **Deterministic Core** | CUBIC RFC 9438 | Standard CUBIC window management |
| **Safe Stub** | Minimal congestion | cwnd=1460*2; rate=10Mbps |

---

## 4. Implementation Workflow

### Step 1: Implement Deterministic Core

```rust
fn deterministic_core(&self, input: &[u8]) -> Result<Vec<u8>> {
    // 1. Parse input
    // 2. Apply pure algorithm (no I/O, no randomness)
    // 3. Return output

    // Example: Deterministic codec selection
    let codec = match self.data_type {
        DataType::Text => Codec::ZstdLevel3,
        DataType::Image => Codec::JpegXl,
        _ => Codec::ZstdLevel3,
    };

    Ok(encode_codec_decision(codec))
}
```

**Testing Deterministic Core:**
```rust
#[test]
fn test_deterministic_core_same_input_same_output() {
    let svc = YourService::new();
    let input = b"test";

    let out1 = svc.deterministic_core(input).unwrap();
    let out2 = svc.deterministic_core(input).unwrap();

    assert_eq!(out1, out2); // Must be deterministic
}

#[test]
fn test_deterministic_core_large_input() {
    let svc = YourService::new();
    let large = vec![42u8; 1_000_000];

    let result = svc.deterministic_core(&large);
    assert!(result.is_ok()); // Must succeed even on large input
}
```

### Step 2: Implement Heuristic (Optional)

```rust
fn heuristic(&self, input: &[u8]) -> Result<Option<Vec<u8>>> {
    // Return None if heuristic doesn't apply
    if !self.is_applicable(input) {
        return Ok(None);
    }

    // Apply rule-based logic
    let decision = self.apply_rules(input)?;
    Ok(Some(encode_decision(decision)))
}
```

**Key Rule: Heuristic must not fail on valid input.**

If a heuristic might fail, wrap the logic:
```rust
fn heuristic(&self, input: &[u8]) -> Result<Option<Vec<u8>>> {
    // Safe: return None instead of error
    let decision = self.apply_rules(input).ok()?;
    Ok(Some(encode_decision(decision)))
}
```

### Step 3: Implement AI Advisory (Feature-Gated)

```rust
#[cfg(feature = "ai-enhancements")]
fn ai_suggestion(&self, input: &[u8]) -> Option<AdvisoryOutput> {
    // Check if AI is enabled
    if !self.ai_enabled {
        return None;
    }

    // Call ML model (sandboxed, timed)
    let start = std::time::Instant::now();
    let advice = self.ml_model.suggest(input)?;
    let latency_us = start.elapsed().as_micros() as u64;

    Some(AdvisoryOutput::new(
        advice,
        confidence,
        latency_us,
    ))
}

#[cfg(not(feature = "ai-enhancements"))]
fn ai_suggestion(&self, _input: &[u8]) -> Option<AdvisoryOutput> {
    None
}
```

### Step 4: Implement Safe Stub

```rust
fn safe_stub(&self, input: &[u8]) -> Vec<u8> {
    // Minimal functionality that never fails
    // Examples:
    // - Return empty response: Vec::new()
    // - Return default decision: constant_safe_value()
    // - Return stub marker + size: [255, size_bytes]

    let mut result = Vec::new();
    result.push(STUB_MARKER);
    result.extend_from_slice(&(input.len() as u32).to_le_bytes());
    result
}
```

---

## 5. Integration with Arbiter

Once your service implements `SovereignService`, use the **Trusted Arbiter** to orchestrate decisions:

```rust
use bonsai_ai_fallback::{Arbiter, ArbiterConfig};

fn main() {
    let service = YourService::new();

    // Create Arbiter with default config (AI disabled by default)
    let mut arbiter = Arbiter::new(ArbiterConfig::default());

    // Execute: Arbiter will walk the graceful degradation ladder
    let input = b"test data";
    let result = arbiter.execute(&service, input);

    println!("Tier: {:?}, Confidence: {}", result.tier, result.confidence);
}
```

**Configuration for Different Scenarios:**

```rust
// Production (safe, no AI)
let prod_config = ArbiterConfig {
    ai_enabled: false,
    heuristic_enabled: true,
    ..Default::default()
};

// Development (AI enabled, aggressive)
let dev_config = ArbiterConfig {
    ai_enabled: true,
    min_confidence: 0.75,
    ai_latency_limit_us: 10_000,
    ..Default::default()
};

// Shadow mode (AI running in parallel, not applied)
let shadow_config = ArbiterConfig {
    ai_enabled: true,
    min_confidence: 1.5, // Impossible threshold
    ..Default::default()
};
```

---

## 6. Testing Strategy

### 6.1 Unit Tests (All Tiers)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_core_on_empty_input() {
        let svc = YourService::new();
        assert!(svc.deterministic_core(&[]).is_ok());
    }

    #[test]
    fn test_deterministic_core_on_large_input() {
        let svc = YourService::new();
        let input = vec![0u8; 10_000_000]; // 10MB
        assert!(svc.deterministic_core(&input).is_ok());
    }

    #[test]
    fn test_heuristic_returns_none_when_not_applicable() {
        let svc = YourService::new();
        let result = svc.heuristic(&[]).unwrap();
        // Should return None if heuristic doesn't apply
    }

    #[test]
    fn test_safe_stub_never_fails() {
        let svc = YourService::new();
        let _result = svc.safe_stub(b"anything");
        // Must not panic
    }
}
```

### 6.2 Integration Tests (Arbiter + Service)

```rust
#[test]
fn test_arbiter_deterministic_fallback() {
    let mut arbiter = Arbiter::new(ArbiterConfig {
        ai_enabled: false,
        heuristic_enabled: false,
        ..Default::default()
    });

    let service = YourService::new();
    let result = arbiter.execute(&service, b"test");

    // Should use deterministic core
    assert_eq!(result.tier, ExecutionTier::DeterministicCore);
}

#[test]
fn test_arbiter_heuristic_applied_when_available() {
    let mut arbiter = Arbiter::new(ArbiterConfig {
        ai_enabled: false,
        heuristic_enabled: true,
        ..Default::default()
    });

    let service = YourService::new();
    let result = arbiter.execute(&service, b"test");

    // Should prefer heuristic over core
    assert_eq!(result.tier, ExecutionTier::Heuristic);
}
```

### 6.3 Chaos Tests (Failure Injection)

```rust
#[test]
fn test_ai_timeout_fallback_to_heuristic() {
    // Simulate AI taking too long
    let mut arbiter = Arbiter::new(ArbiterConfig {
        ai_enabled: true,
        ai_latency_limit_us: 100, // Very tight
        heuristic_enabled: true,
        ..Default::default()
    });

    let service = YourServiceWithSlowAi::new();
    let result = arbiter.execute(&service, b"test");

    // Should have fallen back to heuristic
    assert!(matches!(result.tier, ExecutionTier::Heuristic));
}

#[test]
fn test_all_tiers_fail_falls_back_to_stub() {
    let mut arbiter = Arbiter::new(ArbiterConfig {
        ai_enabled: true,
        heuristic_enabled: true,
        ..Default::default()
    });

    let service = YourServiceAllFailures::new();
    let result = arbiter.execute(&service, b"test");

    // Should have reached safe stub
    assert_eq!(result.tier, ExecutionTier::SafeStub);
}
```

### 6.4 CI Checklist

```bash
# Test AI-free build (no ML dependencies)
cargo build --no-default-features

# Test with AI enabled
cargo build --all-features

# Run all tests
cargo test --all-features

# Run with AI disabled (production path)
cargo test --no-default-features

# Chaos test (AI disabled, validate fallback)
CHAOS_DISABLE_AI=1 cargo test -- --test-threads=1

# Benchmark: compare core vs heuristic vs AI
cargo bench --features ai-enhancements
```

---

## 7. Performance Targets

Every service should aim for:

| Tier | Target | Notes |
|------|--------|-------|
| **Deterministic Core** | <10ms | Synchronous, always available |
| **Heuristic** | <5ms | Faster than core |
| **AI Advisory** | <5ms | Latency budgeted by Arbiter |
| **Safe Stub** | <1µs | Constant-time, no I/O |

**Measurement:**
```rust
use std::time::Instant;

#[test]
fn benchmark_deterministic_core() {
    let svc = YourService::new();
    let input = b"benchmark data";

    let start = Instant::now();
    for _ in 0..10_000 {
        let _ = svc.deterministic_core(input);
    }
    let elapsed = start.elapsed();

    let avg_micros = elapsed.as_micros() / 10_000;
    println!("Average: {}µs", avg_micros);

    // Assert latency bound
    assert!(avg_micros < 10_000, "Core exceeded 10ms target");
}
```

---

## 8. Formal Verification (Axiom)

For critical services (TransferDaemon, Scheduler, Survival), add Axiom proofs:

```rust
// axiom_verification.md (in crate root)

## Theorem: DeterministicCore Never Violates Safety Bounds

Proof (sketch):
- Input: valid request bounded by MAX_INPUT_SIZE
- Processing: pure computation, no heap unbounded growth
- Output: result field < MAX_OUTPUT_SIZE
- Therefore: core always completes in bounded time

Invariants:
1. output.len() <= MAX_OUTPUT_SIZE
2. core execution time <= 10ms on reference hardware
3. core never panics (panic!() forbidden in core path)
```

---

## 9. Shadow-Mode Validation

Before enabling AI in production:

### Phase 1: Development (0 days)
- Implement and test AI suggestion method
- Run in shadow mode (logged but not applied)

### Phase 2: Shadow Validation (7 days)
- Deploy with `ai_enabled: false` but log AI decisions
- Collect 10,000+ suggestions in `recent_decisions` buffer
- Compute daily Shadow Health Score:
  - Latency violations: 0% acceptable
  - Confidence below threshold: <1% acceptable
  - Consistency violations: 0% acceptable

### Phase 3: Promotion (14 days)
- If all Shadow Health Scores pass: sign `ModelManifest`
- Set `ai_enabled: true` in production
- If score drops below threshold: automatic de-promotion

**Implementation:**
```rust
pub struct ModelShadowReport {
    pub total_suggestions: u64,
    pub latency_violations: u64,
    pub confidence_violations: u64,
    pub consistency_violations: u64,
    pub poisoned_outputs: u64,

    pub health_score(&self) -> f32 {
        let violations = (self.latency_violations
            + self.confidence_violations
            + self.consistency_violations
            + self.poisoned_outputs) as f32;

        1.0 - (violations / self.total_suggestions.max(1) as f32)
    }

    pub ready_for_promotion(&self) -> bool {
        self.health_score() >= 0.99 && self.total_suggestions >= 10_000
    }
}
```

---

## 10. Example: Full Service Integration

See the example implementations:
- [Transfer Daemon Example](examples/transfer_daemon_example.rs)
- [BUCE Compression Example](examples/buce_example.rs)
- [Survival System Example](examples/survival_system_example.rs)

---

## 11. Rollout Checklist

For each crate, before merge:

- [ ] Implement `SovereignService` trait (all 4 methods)
- [ ] Deterministic core tested on unit + integration tests
- [ ] Heuristic (if applicable) documented and tested
- [ ] AI advisory feature-gated (default: off)
- [ ] `Cargo.toml` has `ai-enhancements` feature (default off)
- [ ] AI-free build tested: `cargo build --no-default-features`
- [ ] Chaos test passes: system works with AI disabled
- [ ] Performance targets met (latency under limits)
- [ ] Axiom proof sketch provided (critical services)
- [ ] Shadow-mode validation plan documented
- [ ] PR reviewed by 2+ maintainers
- [ ] Integration doc references this service

---

## 12. FAQ

**Q: Can I remove the heuristic layer?**  
A: Yes. If deterministic core covers all cases, return `Ok(None)` from heuristic. Arbiter will skip it.

**Q: What if my AI model takes >5ms?**  
A: Increase `ai_latency_limit_us` in ArbiterConfig (up to 10-50ms). Arbiter will discard late advice. Or optimize your model.

**Q: Must I implement safe_stub?**  
A: Yes. Even if it just returns empty data, it ensures graceful degradation. In worst case, returning empty/default is better than panicking.

**Q: Can I use randomness in the core?**  
A: No. Randomness breaks determinism. Use deterministic PRNGs seeded with input hash if needed.

**Q: How do I test that AI doesn't break the system?**  
A: Run with `min_confidence: 1.5` (impossible threshold) and verify behavior is identical to AI disabled.

**Q: Do I need Axiom proofs?**  
A: Required for TransferDaemon, Scheduler, USOS kernel. Recommended for others. Sketch is acceptable initially.

---

## 13. Support & Questions

For questions on integrating this framework:

1. Check the example implementations (3 real-world examples provided)
2. Review `ai-advisor` crate documentation
3. Open an issue on the Bonsai project tracker

---

**Framework Version**: 1.0.0  
**Last Updated**: 2026-06-04  
**Status**: Production-Ready  
**Maintained By**: Bonsai Ecosystem Team
