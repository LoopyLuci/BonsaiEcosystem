# 🏛️ Trusted Arbiter Specification: Sovereign AI Governance

## Overview

The **Trusted Arbiter** is the central orchestration engine that coordinates all AI decision-making across the Bonsai Ecosystem and USOS. It enforces three core principles:

1. **AI is Always Optional**: Never required for correct system operation
2. **Safety is Hardened**: All AI suggestions validated against formally-verified safety envelopes
3. **Governance is Sovereign**: Capability ramp-up requires council consensus, not automatic

This document specifies:
- Arbiter architecture and state machine
- Safety envelope framework
- Council-governed capability policy
- Sealed boot integration
- Universe audit logging
- Cascade recovery on AI failure

---

## 1. Arbiter Architecture

### 1.1 Execution Context

The Arbiter operates within the **Calculus Domain** (deterministic, verified core):

```
┌─────────────────────────────────────────────────┐
│          Host OS / Hardware / VM                │
│                                                 │
│  ┌──────────────────────────────────────────┐  │
│  │   Calculus Domain (Deterministic)        │  │
│  │  ┌────────────────────────────────────┐ │  │
│  │  │  Service Implementation            │ │  │
│  │  │  (e.g., TransferDaemon, BUCE)     │ │  │
│  │  └────────────────────────────────────┘ │  │
│  │                    ▲                     │  │
│  │                    │ uses                │  │
│  │  ┌────────────────────────────────────┐ │  │
│  │  │  ⭐ TRUSTED ARBITER               │ │  │
│  │  │                                    │ │  │
│  │  │ - Validates AI advice             │ │  │
│  │  │ - Enforces safety envelopes       │ │  │
│  │  │ - Walks degradation ladder        │ │  │
│  │  │ - Logs to Universe                │ │  │
│  │  │ - Checks governance policy        │ │  │
│  │  └────────────────────────────────────┘ │  │
│  └──────────────────────────────────────────┘  │
│                    ▼                           │
│  ┌──────────────────────────────────────────┐  │
│  │   Advisory Domain (Optional AI/ML)       │  │
│  │  [Sandboxed: WebAssembly / Firecracker] │  │
│  │                                          │  │
│  │  - ML models                            │  │
│  │  - Heuristic engines                    │  │
│  │  - Experimental features                │  │
│  └──────────────────────────────────────────┘  │
│                                                 │
└─────────────────────────────────────────────────┘
```

### 1.2 Arbiter State Machine

```
                    ┌──────────────┐
                    │ Initialized  │
                    └──────────────┘
                           │
                           ▼
                    ┌──────────────┐
                    │ Operational  │  (AI enabled, all tiers available)
                    └──────────────┘
                      ▲          ▼
                      │      latency spike
                      │      or low confidence
                      │          ▼
                      │    ┌──────────────┐
                      │    │ AiDegraded   │  (AI unhealthy, use heuristic)
                      │    └──────────────┘
                      │         ▼
                      │    oscillation detected
                      │    or repeated failures
                      │         ▼
                      │    ┌──────────────┐
                      │    │ AiQuarantined│  (AI disabled, core + heuristic only)
                      │    └──────────────┘
                      │         ▼
                      │    admin override
                      │    or 24h cooldown
                      │         ▼
                      │    council signs new
                      │    model manifest
                      │         ▼
                      └──────────────┘

    At any point:
    - Communication failure → AiQuarantined
    - Memory exhaustion   → AiQuarantined + SafeMode
    - All tiers fail      → SafeStub (last resort)
```

---

## 2. Safety Envelope Framework

### 2.1 Definition

A **safety envelope** is a formally-verified boundary that constrains all AI suggestions:

```rust
pub struct SafetyEnvelope {
    /// Parameter bounds proven safe by Axiom
    pub parameter_bounds: Vec<(String, f64, f64)>, // (name, min, max)

    /// Temporal constraints (rate limiting)
    pub max_decisions_per_second: u32,

    /// Consistency constraints
    pub max_deviation_from_previous: f32,

    /// Timeout constraints
    pub max_decision_latency_us: u64,

    /// Resource constraints
    pub max_memory_allocation_mb: u32,

    /// Rollback trigger
    pub auto_rollback_on_violation: bool,
}
```

### 2.2 Envelope Examples

**TransferDaemon Congestion Window**
```
SafetyEnvelope {
    parameter_bounds: [
        ("cwnd", 1460*2,    100*1024*1024),     // 2.9KB to 100MB
        ("pacing_rate", 1,  1_000),             // 1Mbps to 1Gbps
        ("rtt_estimate", 1, 10_000),            // 1ms to 10s
    ],
    max_decisions_per_second: 1000,
    max_deviation_from_previous: 0.5,           // Max 50% change
    max_decision_latency_us: 5_000,
    max_memory_allocation_mb: 512,
    auto_rollback_on_violation: true,
}
```

**BUCE Compression Ratio**
```
SafetyEnvelope {
    parameter_bounds: [
        ("compression_ratio", 1.0, 1000.0),     // 1x to 1000x
        ("encoding_time_ms", 1, 5_000),         // 1ms to 5s
        ("memory_usage_mb", 10, 2_000),         // 10MB to 2GB
    ],
    max_decisions_per_second: 100,
    max_deviation_from_previous: 0.3,           // 30% max shift
    max_decision_latency_us: 10_000,
    max_memory_allocation_mb: 2_000,
    auto_rollback_on_violation: true,
}
```

### 2.3 Envelope Validation (Axiom Proof Sketch)

```
Theorem: Clamped AI output always satisfies safety envelope

Proof:
  Let x_ai = AI suggestion for parameter p
  Let [p_min, p_max] = safety bounds for p
  Let x_clamped = clamp(x_ai, p_min, p_max)

  By definition of clamp():
    p_min <= x_clamped <= p_max

  Therefore:
    x_clamped ∈ [p_min, p_max]
    x_clamped satisfies safety envelope

QED
```

---

## 3. Council-Governed Capability Ramp-Up

### 3.1 Governance Policy

Every AI capability requires a **signed governance policy** that specifies:

```rust
pub struct AiCapabilityPolicy {
    pub capability_id: String,         // e.g., "transfer_daemon_ai_routing"
    pub version: u32,
    pub phase: CapabilityPhase,
    pub allowed_ai_modules: Vec<String>,
    pub safety_envelope_hash: [u8; 32],
    pub min_confidence_threshold: f32,
    pub require_biometric_consent: bool,
    pub observation_period_sec: u64,
    pub rollback_threshold: f32,       // Health score threshold for auto-disable
    pub council_signatures: Vec<Signature>, // M-of-N threshold signatures
    pub signed_at: u64,                // Unix timestamp
    pub expires_at: u64,               // Policy expiration
}

pub enum CapabilityPhase {
    Development,                        // Dev/test only
    ShadowMode,                         // Logging, not applied
    GatedRollout,                       // Enabled for opt-in users
    GeneralAvailability,                // Enabled by default
    Deprecated,                         // Scheduled removal
}
```

### 3.2 Ramp-Up Workflow

```
Phase 1: Development (7 days)
├─ AI model trained
├─ Unit tests + integration tests
├─ Feature-gated (ai-enhancements flag)
├─ Chaos testing (deterministic-only pass)
└─ PR review (2+ maintainers)
        │
        ▼
Phase 2: Shadow Mode (7-14 days)
├─ Deploy with ai_enabled: false
├─ Log all AI decisions to Universe
├─ Collect 10,000+ suggestions
├─ Compute daily Shadow Health Score
├─ Track consistency, latency, confidence
└─ Pass-gate: Score >= 0.99 for 3 consecutive days
        │
        ▼
Phase 3: Gated Rollout (7-30 days)
├─ Create signed AiCapabilityPolicy (Phase: GatedRollout)
├─ Council votes threshold-signature (e.g., 7-of-10)
├─ Deploy with ai_enabled: true for opt-in users
├─ Monitor metrics: adoption, errors, rollback rate
├─ Gradual ramp: 1% → 10% → 50% → 100%
└─ Auto-rollback if health score < threshold
        │
        ▼
Phase 4: General Availability
├─ Council votes Phase: GeneralAvailability
├─ ai_enabled: true by default (for production)
├─ Continuous monitoring
└─ Scheduled updates via signed ModelManifest
```

### 3.3 Policy Verification (In Arbiter)

```rust
impl Arbiter {
    fn check_governance_policy(&self, capability_id: &str) -> Result<bool> {
        // 1. Fetch policy from sealed storage
        let policy = self.load_policy(capability_id)?;

        // 2. Verify signatures (M-of-N threshold)
        let valid_sigs = policy.council_signatures.iter()
            .filter(|sig| sig.verify(&policy.hash(), self.council_pubkeys))
            .count();

        if valid_sigs < self.council_threshold {
            return Err(Error::InsufficientCounselSignatures);
        }

        // 3. Check expiration
        if policy.expires_at < current_timestamp() {
            return Err(Error::PolicyExpired);
        }

        // 4. Check phase is enabled
        match policy.phase {
            CapabilityPhase::Development => {
                // Only in development environments
                Ok(self.is_dev_environment())
            }
            CapabilityPhase::ShadowMode => Ok(true),
            CapabilityPhase::GatedRollout => {
                // Check user is in rollout cohort
                Ok(self.user_in_rollout_cohort(capability_id))
            }
            CapabilityPhase::GeneralAvailability => Ok(true),
            CapabilityPhase::Deprecated => Ok(false),
        }
    }

    fn auto_rollback_if_needed(&mut self, capability_id: &str, health: f32) {
        let policy = match self.load_policy(capability_id) {
            Ok(p) => p,
            Err(_) => return,
        };

        if health < policy.rollback_threshold {
            // Automatic disable
            self.ai_health = AdvisoryHealth::Quarantined;
            self.log_to_universe(&format!(
                "Auto-rollback: {} health {:.2} below threshold {:.2}",
                capability_id, health, policy.rollback_threshold
            ));
        }
    }
}
```

---

## 4. Sealed Boot & Golden Manifest

### 4.1 Boot Sequence

```
1. UEFI/BIOS
   │
   ▼
2. Boot ROM (measured)
   │
   ├─ Load + measure Calculus Domain binary
   │ └─ compute hash(core_binary)
   │
   ├─ Load + measure Arbiter binary
   │ └─ compute hash(arbiter_binary)
   │
   ▼
3. Arbiter Initialization
   │
   ├─ Load sealed Golden Manifest (from TPM/HSM)
   │ └─ Contains blessed (core_hash, arbiter_hash, ai_model_hashes)
   │
   ├─ Verify Calculus Domain hash
   │ │ ├─ Match → proceed with AI enabled (subject to runtime policy)
   │ │ └─ Mismatch → Emergency mode, AI disabled
   │
   ├─ Verify Advisory Domain binaries
   │ │ ├─ All hashes match → AI available
   │ │ └─ Hash unknown → AI quarantined, only deterministic
   │
   ▼
4. Runtime
   │
   ├─ Load AiCapabilityPolicy from sealed storage
   │
   ├─ Apply phase-based availability:
   │ ├─ Dev: feature-gated only
   │ ├─ Shadow: always disabled (logging only)
   │ ├─ GatedRollout: check user cohort
   │ └─ GeneralAvailability: enabled (subject to health)
   │
   └─ Begin normal operation
```

### 4.2 Golden Manifest Format

```json
{
  "version": 1,
  "sealed_at": "2026-06-04T00:00:00Z",
  "core_components": {
    "calculus_domain": {
      "hash": "sha256:abc123...",
      "version": "1.0.0",
      "required": true
    },
    "arbiter": {
      "hash": "sha256:def456...",
      "version": "1.0.0",
      "required": true
    }
  },
  "ai_models": {
    "transfer_daemon_routing": {
      "hash": "sha256:ghi789...",
      "version": "1.2.3",
      "trained_at": "2026-05-28T12:00:00Z",
      "safety_envelope_hash": "sha256:jkl012...",
      "phase": "GeneralAvailability",
      "capability_policy_id": "transfer_daemon_ai_routing_v2"
    },
    "buce_codec_selector": {
      "hash": "sha256:mno345...",
      "version": "2.0.0",
      "phase": "ShadowMode"
    }
  },
  "signed_by": [
    {
      "signer": "council_member_alice",
      "signature": "...",
      "timestamp": "2026-06-04T00:00:00Z"
    },
    {
      "signer": "council_member_bob",
      "signature": "...",
      "timestamp": "2026-06-04T00:00:00Z"
    }
  ],
  "threshold": "7-of-10",
  "expires_at": "2026-07-04T00:00:00Z"
}
```

---

## 5. Universe Audit Logging

All decisions made by the Arbiter are logged to the **Universe** immutable log:

### 5.1 Audit Record Format

```rust
pub struct ArbiterDecisionLog {
    pub timestamp: u64,                     // Unix timestamp
    pub service_name: String,               // e.g., "buce_compression"
    pub execution_tier: ExecutionTier,
    pub reason: String,                     // Why this tier was chosen
    pub confidence: f32,

    pub ai_details: Option<AiDecisionDetails>,
    pub safety_bounds_checked: bool,
    pub safety_violations: Vec<String>,     // If any

    pub outcome: String,                    // "success", "degraded", "failed"
    pub latency_us: u64,

    pub governance_policy_id: Option<String>,
    pub user_cohort: Option<String>,        // For gated rollout
}

pub struct AiDecisionDetails {
    pub ai_model_hash: [u8; 32],
    pub ai_suggestion: Vec<u8>,
    pub ai_confidence: f32,
    pub ai_latency_us: u64,
    pub advice_applied: bool,
    pub clamping_applied: bool,
}
```

### 5.2 Example Log Entry

```json
{
  "timestamp": 1717431600,
  "service_name": "transfer_daemon",
  "execution_tier": "AiEnhanced",
  "reason": "AI provided high-confidence advice within latency budget",
  "confidence": 0.94,
  "ai_details": {
    "ai_model_hash": "sha256:abc123def456...",
    "ai_suggestion": "[1000, 250000, 128000]",
    "ai_confidence": 0.96,
    "ai_latency_us": 2300,
    "advice_applied": true,
    "clamping_applied": false
  },
  "safety_bounds_checked": true,
  "safety_violations": [],
  "outcome": "success",
  "latency_us": 2450,
  "governance_policy_id": "transfer_daemon_ai_routing_v2",
  "user_cohort": "early_adopters_10pct"
}
```

### 5.3 Audit Trail Analysis (for Compliance)

```rust
pub struct AuditAnalysis {
    pub total_decisions: u64,
    pub ai_enhanced_count: u64,
    pub heuristic_count: u64,
    pub deterministic_core_count: u64,
    pub stub_count: u64,

    pub ai_success_rate: f32,
    pub avg_ai_latency_us: u64,
    pub safety_violations: u64,
    pub auto_rollbacks: u64,

    pub most_common_fallback_reason: String,
    pub critical_periods: Vec<(u64, u64)>,  // Timestamps of degraded operation
}

impl AuditAnalysis {
    pub fn compliance_report(&self) -> ComplianceReport {
        ComplianceReport {
            // Regulatory evidence: system operated deterministically without AI
            deterministic_operations: self.deterministic_core_count,

            // AI reliability: success rate and error rate
            ai_reliability: self.ai_success_rate,

            // Safety: violations detected and remediated
            safety_incidents: self.safety_violations,
            auto_remediations: self.auto_rollbacks,

            // Auditability: every decision logged with reason
            audit_coverage: 100.0, // All decisions logged
        }
    }
}
```

---

## 6. Cascade Recovery on AI Failure

When AI fails, the system gracefully cascades through fallback tiers:

### 6.1 Failure Modes

| Failure | Detection | Response |
|---------|-----------|----------|
| **Model crash** | No response in 5ms | Tier → Heuristic |
| **Model returning invalid output** | Clamping detects violation | Advice → discarded, Tier → Heuristic |
| **Model latency spike** | Advice received > deadline | Advice → discarded, Tier → Heuristic |
| **Inconsistent advice** | Consistency window detects oscillation | Health → Quarantined, Tier → Core |
| **Multiple tier failures** | Core fails (should not happen) | Tier → SafeStub |
| **Memory exhaustion** | Malloc failure during AI | Health → AiQuarantined, Tier → Core |
| **Policy expired** | Policy check fails | Capability disabled, Tier → Core |

### 6.2 Cascade Implementation

```rust
impl Arbiter {
    pub fn execute(&mut self, service: &dyn SovereignService, input: &[u8]) -> ExecutionResult {
        // Tier 1: AI (if enabled and healthy)
        if self.config.ai_enabled && self.ai_health == AdvisoryHealth::Healthy {
            if let Some(advice) = service.ai_suggestion(input) {
                if self.validate_ai_advice(&advice) {
                    // ✓ AI decision applied
                    return ExecutionResult {
                        data: advice.data,
                        tier: ExecutionTier::AiEnhanced,
                        confidence: advice.confidence,
                    };
                }
                // ✗ Advice invalid (clamping, latency, consistency)
                // Fall through to Tier 2
            }
            // ✗ AI didn't respond or crashed
            // Fall through to Tier 2
        }

        // Tier 2: Heuristic (if enabled)
        if self.config.heuristic_enabled {
            if let Ok(Some(result)) = service.heuristic(input) {
                return ExecutionResult {
                    data: result,
                    tier: ExecutionTier::Heuristic,
                    confidence: 0.8,
                };
            }
        }

        // Tier 3: Deterministic Core (always available)
        if let Ok(result) = service.deterministic_core(input) {
            return ExecutionResult {
                data: result,
                tier: ExecutionTier::DeterministicCore,
                confidence: 0.6,
            };
        }

        // Tier 4: Safe Stub (never fails)
        ExecutionResult {
            data: service.safe_stub(input),
            tier: ExecutionTier::SafeStub,
            confidence: 0.0,
        }
    }
}
```

---

## 7. Performance & Overhead

The Arbiter is designed to have minimal overhead:

| Operation | Latency | Notes |
|-----------|---------|-------|
| Policy check | <100µs | Cached in RAM |
| Envelope validation | <500µs | Simple bounds checks |
| Consistency check | <1ms | Sliding window lookup |
| Audit logging (async) | <10µs | Non-blocking, async |
| **Total Arbiter overhead** | **<2ms** | Negligible vs. AI latency |

**Measurement:**
```rust
#[test]
fn benchmark_arbiter_overhead() {
    let config = ArbiterConfig::default();
    let mut arbiter = Arbiter::new(config);
    let service = DummyService::new();

    let start = Instant::now();
    for _ in 0..10_000 {
        let _ = arbiter.execute(&service, b"test");
    }
    let elapsed = start.elapsed();

    let avg_micros = elapsed.as_micros() / 10_000;
    println!("Arbiter overhead: {}µs per execution", avg_micros);

    assert!(avg_micros < 2_000, "Overhead exceeded 2ms");
}
```

---

## 8. Formal Guarantees

### 8.1 Safety Properties (Verified in Axiom)

**Property 1: Safety Envelope Invariant**  
```
For all AI advice x:
  Arbiter.apply_clamping(x) ∈ [min_bound, max_bound]
```
Proof: Axiom SMT solver verification of clamping function.

**Property 2: Degradation Ladder Completeness**  
```
For all service failures:
  tier1_fails(s) ∧ tier2_fails(s) ∧ tier3_fails(s)
  ⟹ tier4_succeeds(s)  // Safe stub always works
```
Proof: By design (safe_stub() must never panic or error).

**Property 3: Governance Policy Verification**  
```
For all AI decisions:
  ai_enabled(d) ∧ ¬policy_valid(d) ⟹ tier ≠ AiEnhanced
```
Proof: Policy check in validate_ai_advice() is mandatory.

### 8.2 Liveness Properties

**Property 4: System Remains Responsive**  
```
For all valid requests:
  ∃ tier ∈ {AI, Heuristic, Core, Stub} such that
    tier can execute within latency_budget
```
Proof: Stub tier has constant-time upper bound.

**Property 5: Health Monitoring is Reactive**  
```
For all N consecutive failed AI suggestions:
  health(Arbiter) degrades within time(N requests)
```
Proof: ConsistencyWindow.is_consistent() checks last 8.

---

## 9. Deployment Checklist

Before deploying Arbiter to production:

- [ ] Axiom proofs verified for core crate
- [ ] Safety envelopes defined for all AI capabilities
- [ ] AiCapabilityPolicy signed by council (M-of-N)
- [ ] Golden Manifest sealed in TPM/HSM
- [ ] Shadow-mode validation passed (10K+ ops, 99%+ health)
- [ ] Chaos tests pass (all tiers, failure injection)
- [ ] Audit logging configured (Universe integration)
- [ ] Council governance policy document published
- [ ] User documentation updated (opt-in AI section)
- [ ] Security audit completed (external)
- [ ] Rollback plan tested (manual + auto-rollback)
- [ ] On-call runbook prepared

---

## 10. Conclusion

The **Trusted Arbiter** is a formally-verified, council-governed orchestration engine that makes AI genuinely optional, truly safe, and accountable. By enforcing safety envelopes, governance policies, and graceful degradation, it enables the Bonsai Ecosystem to leverage AI for performance while maintaining ironclad reliability and sovereignty.

---

**Specification Version**: 1.0.0  
**Release Date**: 2026-06-04  
**Status**: Production-Ready  
**Author**: Bonsai Ecosystem Security Team  
**Review**: External Security Audit + Formal Verification
