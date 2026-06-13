# Extension Migration & Feature Flag Strategy
## Transitioning Copilot/Claude Code Users to Bonsai Sovereign Proxy

> A phased, risk-minimized plan to migrate existing extension users from cloud-dependent Copilot/Claude Code to the Bonsai Sovereign Proxy without breaking existing workflows or losing any user data.

---

## 1. Executive Summary

### Challenge
Existing users have investments in Copilot and Claude Code extensions:
- Cloud API credentials stored securely in VS Code
- Conversation history and session data in cloud services
- Custom extension configurations and organization deployments
- Deep muscle memory in existing workflows

Migrating these users to Bonsai must be **100% optional**, **zero-friction**, and **rollback-safe** at every step.

### Solution
A **4-phase, 12-week migration** that:
1. Installs Bonsai Sovereign Proxy in "idle mode" (no impact)
2. Gradually introduces local inference opt-in via feature flags
3. Flips to hybrid mode as default (cloud fallback always available)
4. Enables advanced users to opt into full offline sovereignty

**Key properties:**
- Users can rollback to pure cloud mode anytime with 1-click
- Zero feature loss; all existing extensions continue working
- Measurable telemetry at every phase
- Automatic fallback if local inference quality degrades
- Enterprise controls for organization-wide deployment

---

## 2. Migration Roadmap

### Phase 1: Idle Mode Installation (Weeks 1-2)

**Objective:** Deploy Bonsai extension to 100% of VS Code user base; collect baseline telemetry.

**What happens:**
- Bonsai Sovereign Proxy extension installs alongside Copilot/Claude Code
- Extension runs in **idle mode**: all traffic routes through existing cloud APIs
- No performance impact; no behavior change; no storage impact
- User sees optional notification: "Bonsai Sovereign Proxy installed. All requests still use cloud. [Learn more]"

**Rollout strategy:**
1. **Internal testing (Days 1-3):** Bonsai team + QA
2. **Early adopter ring (Days 4-7):** 100 volunteer power users
3. **General release (Days 8-14):** Deploy to all VS Code users

**Telemetry collected:**
- Baseline latency (p50, p95, p99) for cloud requests
- Baseline throughput (completions/hour, chat messages/hour)
- Baseline error rate and failure modes
- Network quality metrics (packet loss, jitter)
- Device specifications (CPU, memory, OS version)

**Success criteria:**
- 0 uninstalls due to Bonsai extension
- Telemetry pipeline stable and >95% accurate
- No regression in cloud API latency or reliability
- 1-week retention >95%

**Rollback plan:**
If >1% of users report crashes or >5% uninstall in first week:
- Pull extension from marketplace
- Pause rollout
- Investigate root cause (max 48 hours)
- Deploy fix and resume rollout to next ring

---

### Phase 2: Gradual Local Inference Opt-In (Weeks 3-8)

**Objective:** Introduce local inference capability; measure quality and performance; build user confidence.

**What happens:**
- **New feature flag:** `use_local_inference: false` (default; cloud-only)
- Users can toggle flag in VS Code settings or Blueprint
- Flag is **hot-reloadable** (no restart required)
- When enabled, proxy routes completion requests locally; chat remains optional
- If local inference fails, proxy **silently falls back** to cloud (user never sees failure)

**Feature flag schema:**
```yaml
# .vscode/settings.json or Bonsai Blueprint
{
  "bonsai.extensions.copilot": {
    "enabled": true,
    "use_local_inference": false,       # Phase 2 default
    "cloud_fallback": true,              # Always enabled in Phase 2-3
    "offline_only": false                # Phase 4 only
  },
  "bonsai.extensions.claude_code": {
    "enabled": true,
    "use_local_inference": false,
    "cloud_fallback": true,
    "checkpoint_via_universe": false,    # Phase 2 opt-in (KDB persistence)
    "kdb_semantic_mentions": false       # Phase 2 opt-in (local semantic indexing)
  }
}
```

**Rollout cadence:**
1. **Week 3:** Deploy to Bonsai team internally
2. **Week 3-4:** Early adopters (1000 volunteer users) can opt-in via settings
3. **Week 5-6:** Expand to 5% of user base (invite via in-app notification)
4. **Week 7-8:** Expand to 20% of user base

**Telemetry collected:**
| Metric | Purpose | Threshold |
|--------|---------|-----------|
| `completions_local` | Adoption rate | Target: 500+ per user/week by week 6 |
| `completions_cloud_fallback` | Reliability | Goal: <2% fallback rate |
| `latency_local_p50/p99` | Performance | Goal: <500ms p50, <2s p99 |
| `latency_cloud_p50/p99` | Comparison | Baseline from Phase 1 |
| `failures_local` | Quality | Goal: <1% error rate |
| `user_satisfaction_rating` | Engagement | Target: ≥4.0/5.0 |
| `rejection_rate_local_vs_cloud` | Quality comparison | Goal: <10% delta |
| `battery_drain_delta` | Device impact | Goal: <5% increase on battery estimates |

**Dashboard & alerting:**
Create Phase 2 dashboard accessible to eng team showing:
- Real-time adoption (% of users with flag enabled)
- Latency heatmap (local vs cloud by geography)
- Fallback rate (% of requests that fell back to cloud)
- Error breakdown (timeout, OOM, model overload, etc.)
- NPS and satisfaction trends

**Alert conditions** (trigger automatic investigation):
- Fallback rate >5%: pause expansion to next ring
- P99 latency increase >50% vs Phase 1: auto-disable flag for affected users, post notification
- Error rate >3%: pause expansion, investigate model quality
- Battery drain increase >10% on mobile: adjust inference batch size or pause on battery saver mode

**Go/No-Go decision gates:**
- **After week 4:** Can we expand to 5%? (Require: fallback <2%, satisfaction >3.5, latency <2s p99)
- **After week 6:** Can we expand to 20%? (Require: fallback <2%, satisfaction >4.0, latency <1.5s p99)
- **After week 8:** Ready for Phase 3? (Require: adoption >50k users, all metrics above thresholds, <0.5% serious issues)

**Success criteria:**
- Fallback rate stabilizes at <2%
- Satisfaction rating reaches 4.0+/5.0
- Latency improvement >20% vs cloud
- 0 conversation data loss incidents
- Early adopter NPS >40

**Rollback plan:**
If any red-line metric fails:
1. Immediately disable flag for affected users
2. Post in-app notification: "Local inference temporarily paused due to [reason]. Cloud mode active."
3. Investigate root cause (48-hour SLA)
4. Deploy fix and resume rollout to same ring

---

### Phase 3: Hybrid Mode as Default (Weeks 9-12)

**Objective:** Flip default to local inference; demonstrate production stability; achieve >70% adoption.

**What happens:**
- **Flip default:** `use_local_inference: true` (hybrid mode)
- Cloud fallback remains **always enabled and transparent**
- Users can opt-out: `cloud_only: true` for 100% cloud mode
- Users can opt-out: `offline_only: false` to enable full offline (Phase 4 preview)
- Telemetry now tracks adoption from opt-out angle

**Extended feature flag schema:**
```yaml
{
  "bonsai.extensions.copilot": {
    "enabled": true,
    "use_local_inference": true,           # Phase 3+ default
    "cloud_fallback": true,                # Always transparent
    "offline_only": false,                 # Phase 4 preview
    "feature_flags": {
      "predict_urv": false,                # AI-driven scheduling (Phase 4)
      "traffic_classification": false,     # ML-based flow prioritization (Phase 4)
      "federated_load_balancing": false    # Multi-TDLB coordination (Phase 4+)
    }
  },
  "bonsai.extensions.claude_code": {
    "enabled": true,
    "use_local_inference": true,
    "cloud_fallback": true,
    "checkpoint_via_universe": false,      # Phase 2-3 opt-in
    "kdb_semantic_mentions": false,        # Phase 2-3 opt-in
    "feature_flags": {}
  }
}
```

**Rollout to all users over 2 weeks:**
- Week 9: Deploy to all users; monitor closely
- Week 10-12: Verify stability; support incoming issues; gather feedback

**New telemetry:**
- Track opt-out reasons (users who set `cloud_only: true`): feedback form + auto-analysis
- Measure fallback frequency by failure reason (timeout, OOM, model overload, etc.)
- Measure session retention (do users stay in local mode?)
- Measure new feature adoption (checkpoint, semantic indexing)

**Support changes:**
- Support team trains on local vs cloud troubleshooting
- Knowledge base article: "Why is local inference slower? [Troubleshooting]"
- Auto-escalation: if user reports >5 fallbacks/session, offer diagnostic collection

**Go/No-Go decision:**
- **After week 12:** Continue to Phase 4 or stay in Phase 3? 
  - Requirement: adoption >70%, NPS stable/increasing, <1% critical issues
  - If not ready: stay in Phase 3 for 4 more weeks; re-evaluate

**Success criteria:**
- >70% of users on local inference mode (opt-out <30%)
- Cloud fallback rate <5% across all regions
- NPS stays same or increases
- User support tickets <0.1% of user base
- Session retention >80% (users stick with local mode)

**Rollback plan:**
If critical issue surfaces:
1. Flip default back to `use_local_inference: false`
2. Users on local mode stay on local (respect their choice)
3. Investigate issue
4. Re-flip default when fixed

---

### Phase 4: Full Sovereignty (Month 4+)

**Objective:** Enable advanced users to opt into offline-only mode; complete migration arc.

**What happens:**
- **New feature flag:** `offline_only: true` (no cloud access at all)
- Advanced users can enable for maximum privacy/compliance
- Local KDB semantic indexing fully enabled
- Conversation checkpointing via Universe (Bonsai's distributed state store)
- Advanced feature flags unlock (predict-URV, traffic classification, federated LB)

**Extended flag schema:**
```yaml
{
  "bonsai.extensions.copilot": {
    "enabled": true,
    "use_local_inference": true,
    "cloud_fallback": false,               # Disabled in offline mode
    "offline_only": true,                  # Phase 4 only
    "feature_flags": {
      "predict_urv": true,                 # AI-driven scheduling
      "traffic_classification": true,      # ML-based flow prioritization
      "federated_load_balancing": true,    # Multi-TDLB coordination
      "universe_checkpoint": true,         # Distributed checkpointing
      "memory_replication": false          # (Phase 4+ experimental)
    }
  },
  "bonsai.extensions.claude_code": {
    "enabled": true,
    "use_local_inference": true,
    "cloud_fallback": false,
    "offline_only": true,
    "checkpoint_via_universe": true,
    "kdb_semantic_mentions": true,
    "feature_flags": {}
  }
}
```

**Rollout:**
- Month 4: Internal-only; stability testing
- Month 5+: Available to power users; opt-in documentation

**Success criteria:**
- >10% of existing users opt into offline mode
- Offline mode session retention >75%
- Zero data loss incidents
- User NPS >50 for offline mode

**Backward compatibility:**
- Users can still switch to hybrid or cloud-only mode anytime
- Conversations from offline mode can be synced to cloud if user re-enables cloud fallback
- No forced upgrade; purely opt-in

---

## 3. Feature Flag Specification

### Location and Storage

**Primary storage:** VS Code Settings
```json
// .vscode/settings.json
{
  "bonsai.extensions.copilot.enabled": true,
  "bonsai.extensions.copilot.use_local_inference": false,
  "bonsai.extensions.copilot.cloud_fallback": true,
  "bonsai.extensions.copilot.offline_only": false,
  // ... more flags
}
```

**Secondary storage (redundancy):** Bonsai Blueprint (server-side)
- Used for admins to enforce org-wide policies
- Used for experiments (A/B tests, canary rollouts)
- Synced to local settings; local settings always take precedence

**Hot-reload:**
- All flags are reloadable **without VS Code restart**
- Blueprint sync happens every 5 minutes (or on-demand via command palette: "Bonsai: Refresh Settings")
- Local setting changes are applied immediately

### Complete Flag Reference

#### Phase 1-2: Cloud Mode with Optional Local Inference

| Flag | Type | Default | Phase | Description |
|------|------|---------|-------|-------------|
| `bonsai.extensions.copilot.enabled` | bool | `true` | 1+ | Copilot extension active |
| `bonsai.extensions.copilot.use_local_inference` | bool | `false` | 2+ | Route completions to local model |
| `bonsai.extensions.copilot.cloud_fallback` | bool | `true` | 2-3 | Fallback to cloud on local failure |
| `bonsai.extensions.copilot.offline_only` | bool | `false` | 4+ | Disable cloud access entirely |
| `bonsai.extensions.claude_code.enabled` | bool | `true` | 1+ | Claude Code extension active |
| `bonsai.extensions.claude_code.use_local_inference` | bool | `false` | 2+ | Route chat to local model |
| `bonsai.extensions.claude_code.cloud_fallback` | bool | `true` | 2-3 | Fallback to cloud on local failure |
| `bonsai.extensions.claude_code.offline_only` | bool | `false` | 4+ | Disable cloud access entirely |
| `bonsai.extensions.claude_code.checkpoint_via_universe` | bool | `false` | 2+ | Persist conversations to Universe KDB |
| `bonsai.extensions.claude_code.kdb_semantic_mentions` | bool | `false` | 2+ | Local semantic search of KDB |

#### Phase 4: Advanced Feature Flags

| Flag | Type | Default | Phase | Description |
|------|------|---------|-------|-------------|
| `bonsai.extensions.copilot.feature_flags.predict_urv` | bool | `false` | 4+ | AI-driven URI-to-request scheduling |
| `bonsai.extensions.copilot.feature_flags.traffic_classification` | bool | `false` | 4+ | ML-based completions priority |
| `bonsai.extensions.copilot.feature_flags.federated_load_balancing` | bool | `false` | 4+ | Multi-TDLB coordination |
| `bonsai.extensions.copilot.feature_flags.universe_checkpoint` | bool | `false` | 4+ | Distributed state replication |

### Flag Dependencies and Invariants

```rust
// Rust validation rules (enforced in config.rs)

pub fn validate_feature_flags(flags: &FeatureFlags) -> Result<(), String> {
    // offline_only ⟹ use_local_inference ∧ ¬cloud_fallback
    if flags.offline_only && (!flags.use_local_inference || flags.cloud_fallback) {
        return Err("offline_only requires use_local_inference=true and cloud_fallback=false".into());
    }
    
    // ¬use_local_inference ⟹ ¬checkpoint_via_universe
    if !flags.use_local_inference && flags.checkpoint_via_universe {
        return Err("checkpoint_via_universe requires use_local_inference=true".into());
    }
    
    // ¬use_local_inference ⟹ ¬kdb_semantic_mentions
    if !flags.use_local_inference && flags.kdb_semantic_mentions {
        return Err("kdb_semantic_mentions requires use_local_inference=true".into());
    }
    
    // predict_urv ⟹ use_local_inference (AI scheduling only for local)
    if flags.predict_urv && !flags.use_local_inference {
        return Err("predict_urv requires use_local_inference=true".into());
    }
    
    Ok(())
}
```

### Updating Flags Dynamically

**User-initiated:**
```
User opens VS Code Settings > Search "bonsai.extensions" > Toggle flag > Proxy hot-loads
```

**Admin-initiated (via Blueprint):**
```bash
# Update org policy via Blueprint API
curl -X POST https://blueprint.api.bonsai.local/v1/org/policies \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -d '{
    "policy_id": "copilot-local-inference",
    "applies_to_users": ["user@company.com"],
    "flags": { "use_local_inference": true },
    "enforcement": "soft",  # soft=user can override, hard=enforced
    "notify_users": true
  }'
```

**Experiment/Canary (via flag control plane):**
```bash
# A/B test: 50% of users get local inference
./bonsai-cli feature-gate copilot.use_local_inference \
  --rollout 0.5 \
  --region us-east \
  --min-build-age 7d
```

---

## 4. Telemetry & Monitoring

### Collection Infrastructure

**Backend:** Bonsai Telemetry Service (anonymous, opt-out available)
- Collects metrics from proxy extension every 5 minutes (or on-demand)
- Metrics are **anonymized** (no user ID, no code content, no paths)
- Data is **encrypted in transit** (TLS 1.3)
- Users can **opt-out globally** via checkbox in settings

**Rust telemetry struct:**
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionTelemetry {
    // Session metadata
    pub session_id: String,                  // Anonymous UUID
    pub timestamp: i64,                      // Unix milliseconds
    pub phase: u8,                           // Migration phase (1-4)
    pub local_model_version: String,         // e.g., "bonsai-llama-7b-v1.2"
    
    // Usage metrics
    pub completions_requested: u64,          // Total completion requests in window
    pub completions_local: u64,              // Routed to local model
    pub completions_cloud: u64,              // Routed to cloud
    pub completions_cloud_fallback: u64,     // Local failure, fell back to cloud
    pub chat_messages: u64,                  // Chat messages in window
    pub chat_local: u64,                     // Chat routed locally
    pub chat_cloud: u64,                     // Chat routed to cloud
    
    // Performance metrics
    pub latency_local_p50_ms: f64,
    pub latency_local_p95_ms: f64,
    pub latency_local_p99_ms: f64,
    pub latency_cloud_p50_ms: f64,
    pub latency_cloud_p95_ms: f64,
    pub latency_cloud_p99_ms: f64,
    pub latency_delta_local_vs_cloud: f64,   // Negative = local is faster
    
    // Reliability metrics
    pub failures_local: u64,                 // Model errors, timeouts, OOM
    pub failures_cloud: u64,                 // Cloud API errors
    pub rollback_to_cloud_count: u64,        // Local failure → cloud fallback
    pub fallback_rate: f64,                  // rollback / (local + rollback)
    
    // Device metrics
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub battery_percent: Option<f64>,        // Mobile only
    pub battery_drain_estimate: Option<f64>, // W/hr
    pub network_latency_ms: f64,
    pub network_loss_percent: f64,
    
    // User engagement
    pub user_satisfaction_rating: Option<u8>, // 1-5 star, opt-in prompt
    pub reported_issues: Vec<IssueReport>,
    pub feature_flags_enabled: Vec<String>,    // Which flags user has on
    
    // Computed derived metrics
    pub local_inference_quality_score: f64,    // 0-100, based on acceptance rate
    pub overall_satisfaction_score: f64,       // 0-100, composite metric
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueReport {
    pub title: String,                       // User-entered title
    pub description: String,                 // User-entered description
    pub category: String,                    // bug|slow|crash|unclear
    pub severity: String,                    // minor|medium|critical
    pub timestamp: i64,
}
```

### Metrics & Dashboards

**Real-time dashboard (for eng team):**
```
┌─────────────────────────────────────────────────────────────────┐
│ Bonsai Extension Migration Dashboard (Phase 2)                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Adoption & Traffic                                              │
│ ├─ Users with local inference enabled:    52,340 (4.2%)        │
│ ├─ Completions routed locally today:      1.2M (5.3%)          │
│ ├─ Fallback rate (7-day avg):             1.8%                 │
│ └─ Cloud error rate (7-day avg):          0.12%                │
│                                                                 │
│ Performance Comparison                                          │
│ ├─ Local latency (p50/p99):               280ms / 1,240ms      │
│ ├─ Cloud latency (p50/p99):               520ms / 2,100ms      │
│ ├─ Local faster than cloud:               46% of requests      │
│ └─ Latency improvement vs cloud:          +65% (p50)           │
│                                                                 │
│ Quality Metrics                                                 │
│ ├─ User satisfaction (local):             4.1/5.0 ⭐⭐⭐⭐☆   │
│ ├─ User satisfaction (cloud):             4.3/5.0 ⭐⭐⭐⭐☆   │
│ ├─ Rejection rate (local vs cloud):       9% delta             │
│ └─ Critical issues in local:              0.02%                │
│                                                                 │
│ Device Impact                                                   │
│ ├─ CPU usage delta (local on):            +3.2%                │
│ ├─ Memory usage delta:                    +12 MB               │
│ ├─ Battery drain delta (mobile):          +4.1%                │
│ └─ Thermal impact:                        +2°C (avg)           │
│                                                                 │
│ Geographic Breakdown (7-day active users)                      │
│ ├─ North America:      25k users, 1.8% fallback               │
│ ├─ Europe:             16k users, 2.1% fallback               │
│ ├─ APAC:               11k users, 2.3% fallback               │
│ └─ Other:              0.3k users, 2.5% fallback              │
│                                                                 │
│ Top Issues (Last 24h)                                           │
│ ├─ 1. Timeout on large files (42 reports)                      │
│ ├─ 2. Slow on weak WiFi (28 reports)                           │
│ └─ 3. OOM on 4GB RAM devices (15 reports)                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Phase-specific metrics:**

**Phase 1 (Baseline):**
- Cloud API latency (p50, p95, p99) by region
- Cloud API error rate (4xx, 5xx, timeout)
- Network quality (jitter, loss, RTT)
- Device inventory (OS version, CPU, memory distribution)

**Phase 2 (Local inference opt-in):**
- Adoption rate (% of users with flag enabled)
- Fallback rate (% of local requests that fell back to cloud)
- Latency comparison (local vs cloud)
- Error breakdown by type (timeout, OOM, model overload, invalid input)
- User satisfaction (NPS, ratings, open feedback)
- Quality comparison (completion acceptance rate local vs cloud)

**Phase 3 (Hybrid default):**
- Opt-out rate (% of users who set `cloud_only: true`)
- Session retention (do users stay in local mode?)
- Regional variance (fallback rate by geography)
- Feature adoption (checkpoint, semantic indexing)
- Support ticket volume and categories

**Phase 4 (Full offline):**
- Offline mode adoption (% of users on `offline_only: true`)
- Offline mode retention (session duration)
- Advanced feature usage (predict-URV, traffic classification)
- Sync reliability (Universe checkpoint success rate)

### Alert Conditions & Auto-remediation

| Alert | Condition | Action | SLA |
|-------|-----------|--------|-----|
| **High fallback rate** | >5% of local requests fall back | Pause expansion to next ring; investigate | 1h |
| **Latency regression** | Local p99 latency >50% worse than cloud | Auto-disable flag for affected users; post notification | 30m |
| **High error rate** | >3% of local requests error | Pause expansion; investigate model | 1h |
| **OOM on low-RAM devices** | >2% of requests OOM on <4GB devices | Reduce batch size or pause on low-RAM devices | 2h |
| **Battery drain** | >10% increase on mobile | Adjust inference batch size; pause on battery saver | 2h |
| **Data loss** | Any conversation data loss | Immediate rollback; incident review | 15m |
| **Cloud fallback unavailable** | Cloud fallback itself fails | Disable local mode; alert on-call | 15m |
| **NPS decline** | Satisfaction rating drops >0.3 points | Investigate top issues; post hotfix | 4h |

### User Feedback Collection

**Automatic trigger:** After every 10 completions (opt-in, user can disable)
```
┌─────────────────────────────┐
│ Bonsai Proxy Quality Check  │
├─────────────────────────────┤
│                             │
│ How did we do?              │
│ ⭐ ⭐ ⭐ ⭐ ☆ (4 stars)      │
│                             │
│ [Optional] What went wrong? │
│ ☐ Too slow                  │
│ ☐ Wrong answer              │
│ ☐ Out of context            │
│ ☐ Other: ___________        │
│                             │
│ [Never show] [Submit] [Skip]│
│                             │
└─────────────────────────────┘
```

**Analysis:** Automated feedback categorization using simple keyword matching
- Extract themes (slow, wrong, context loss, etc.)
- Correlate with technical metrics (latency, error type, model version)
- Generate weekly digest for eng team

---

## 5. Risk Assessment & Mitigation

### Risk Matrix

| # | Risk | Likelihood | Impact | Mitigation | Contingency |
|---|------|-----------|--------|-----------|------------|
| 1 | Local model latency worse than cloud | Medium | High | Measure P99 latency before/after; auto-disable if >50% regression | Rollback phase; improve model batching |
| 2 | Local model quality degrades | Medium | High | A/B test completions; track rejection rate | Train new model; longer Phase 2 |
| 3 | Local inference OOM on low-RAM devices | High | Medium | Monitor memory; reduce batch size for <4GB devices | Pause on low-RAM; document min specs |
| 4 | Unexpected battery drain on mobile | High | Medium | Collect telemetry on battery usage; set thresholds | Reduce inference frequency; pause on battery saver |
| 5 | User confusion during migration | Medium | Medium | Clear in-app notifications; FAQ doc; support training | Create video tutorials; increase support staff |
| 6 | Organization blocks local mode (data residency policy) | Low | High | Provide enterprise admin control; document compliance | Offer "cloud-only" enforcement; audit logging |
| 7 | Conversation data loss during cloud↔local sync | Low | Critical | Test sync thoroughly; log all state transitions | Require user explicit opt-in for data sync; transaction logs |
| 8 | Local model corrupts on upgrade | Low | Critical | Verify model checksums; atomic swap with rollback | Automatic rollback on checksum mismatch |
| 9 | Cloud fallback mechanism fails | Low | Critical | Test fallback path in every integration test | Graceful degradation; user notification |
| 10 | Telemetry pipeline fails (silent data loss) | Low | Medium | Deduplicate metrics; track pipeline health | Alert eng team; suspend collection if pipeline down |
| 11 | Copilot/Claude Code extension breaking change | Low | High | Version negotiation; compatibility matrix | Maintain backward-compatible proxy interface |
| 12 | Regional latency variance (non-US users slower) | Medium | Medium | Collect geo-specific metrics; regional model caching | Deploy regional proxy replicas; CDN caching |

### Contingency Plans

**Scenario 1: Local model is consistently 20% slower than cloud**
1. Identify root cause (model inference time vs network latency vs batching)
2. If inference time: retrain model with knowledge distillation (2-week delay)
3. If batching: increase batch size; measure throughput impact
4. Decision: stay in Phase 2 for 4 more weeks, re-test; or extend Phase 2 by 1 month

**Scenario 2: OOM crashes on 4GB devices (1% of devices)**
1. Add device-specific memory check
2. For devices <4GB: reduce batch size to 1, disable semantic caching
3. Document minimum specs: "Bonsai local inference requires 4GB RAM"
4. Option to opt into cloud-only mode for low-RAM users

**Scenario 3: Organization policy blocks local inference (data residency)**
1. Add admin control: `require_cloud_fallback: true` (org-wide policy)
2. Add compliance audit logging: which users use local vs cloud
3. Deploy enterprise offering: "Bonsai Cloud Proxy" (cloud-hosted local model)

**Scenario 4: Conversation data loss (even one incident)**
1. Immediate rollback: disable checkpoint via Universe
2. Root cause analysis: review sync logic
3. Add transaction logging: every state change is logged in immutable ledger
4. Redesign sync to use two-phase commit
5. Re-enable only after 1000-user beta with zero data loss

**Scenario 5: NPS drops from 45 to 40 in Phase 3**
1. Analyze feedback: what's causing dissatisfaction?
2. If slowness: implement local inference caching; reduce P99 latency
3. If quality: collect failed completions; retrain model
4. If UX confusion: post in-app tutorial; update docs
5. If none of above: consider staying in hybrid mode longer (no Phase 4)

---

## 6. Backward Compatibility Matrix

### Extension Version Compatibility

| Bonsai Proxy | Copilot | Claude Code | Status | Notes |
|------|---------|-----------|--------|-------|
| v0.1 (Phase 1) | v1.200+ | v0.1+ | ✅ Compatible | Idle mode; no inference |
| v0.2 (Phase 2) | v1.200+ | v0.1+ | ✅ Compatible | Local inference opt-in |
| v0.3 (Phase 3) | v1.200+ | v0.1+ | ✅ Compatible | Local inference default |
| v0.4 (Phase 4) | v1.200+ | v0.1+ | ✅ Compatible | Full offline mode |
| **Sunset:** v0.1 | v1.200+ | v0.1+ | ⚠️ Deprecated (6mo) | Migrate to v0.2+ |

### API Key & Credential Preservation

**Copilot API key:**
- Stored in VS Code secret storage (unchanged)
- Proxy reads key at startup; never exposed to disk
- If user disables local inference, Copilot uses key directly (bypasses proxy)
- Key is **never logged or transmitted** outside VS Code process

**Claude Code credentials:**
- Stored in VS Code secret storage (unchanged)
- Proxy reads credentials; uses them for cloud fallback
- User can revoke proxy access: "Settings > Bonsai > Revoke Cloud Credentials"

**Cloud conversation history:**
- Accessible via web UI even if using local mode
- Can be synced back to cloud via opt-in feature (Phase 2+)
- No automatic sync; user controls privacy

### Gradual Rollout & Sunset Schedule

**Version rollout timeline:**
```
Phase 1 (v0.1): Weeks 1-2     →  Reach 100% of user base
Phase 2 (v0.2): Weeks 3-8     →  Reach 20% of user base
  └─ v0.1 sunset warning posted in week 5
Phase 3 (v0.3): Weeks 9-12    →  Reach 100% of user base
  └─ v0.1 sunset deadline (12 weeks from release)
  └─ v0.2 sunset warning posted in week 20
Phase 4 (v0.4): Month 4+      →  Opt-in only
  └─ v0.2 sunset deadline (6 months from Phase 2 release)
```

**Deprecation policy:**
- Minor versions (v0.2.x, v0.3.x) receive bug fixes for 12 months
- After 12 months: users on deprecated version see in-app prompt to upgrade
- After 18 months: deprecated version stops receiving fallback support
- Users can always manually upgrade via VS Code extension marketplace

---

## 7. User Communication Plan

### Phase 1: Installation Notification

**In-app notification (one-time):**
```
┌─────────────────────────────────────────────────────┐
│ Bonsai Sovereign Proxy Ready                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│ We've installed Bonsai Sovereign Proxy for your     │
│ Copilot and Claude Code extensions.                 │
│                                                     │
│ Right now, all requests still use cloud APIs.       │
│ Local inference is completely optional.             │
│                                                     │
│ ℹ️  What's Bonsai Sovereign Proxy?                  │
│ An optional privacy-first local inference layer     │
│ that keeps your code and conversations private.     │
│                                                     │
│ [Learn More] [Settings] [Dismiss]                   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Phase 2: Local Inference Opt-In

**In-app notification + context menu:**
```
┌─────────────────────────────────────────────────────┐
│ Try Bonsai Local Inference (Beta)                   │
├─────────────────────────────────────────────────────┤
│                                                     │
│ Bonsai Sovereign Proxy is now offering beta access  │
│ to local code completions. Completions will be      │
│ processed entirely on your device.                  │
│                                                     │
│ Benefits:                                           │
│ ✓ Faster completions (typically <500ms)            │
│ ✓ Your code never leaves your machine               │
│ ✓ Works offline                                     │
│ ⚠️  Beta quality: some completions may be weaker    │
│                                                     │
│ [Enable Beta] [Not Now] [FAQ]                       │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**FAQ document:**

**Q: Will this break my existing setup?**
A: No. Local inference is completely optional. If you don't enable it, everything works exactly as before. Even if you enable it, Copilot will automatically fall back to cloud if there's any issue.

**Q: Is my code private?**
A: Yes. When local inference is enabled, code completions are processed entirely on your device. Your code never leaves your machine unless you explicitly enable cloud fallback (which happens automatically only if local inference fails).

**Q: What if local inference is slower?**
A: We measure latency carefully. Local inference is typically 200-400ms faster than cloud. But if your device is slower or network is fast, we'll measure that and you can disable local mode anytime.

**Q: What about my cloud history?**
A: Your existing Copilot and Claude Code conversations in the cloud remain untouched and accessible. You can still view them on the web. Local inference doesn't affect cloud storage.

**Q: Can I roll back?**
A: Yes, anytime. Just disable the feature in settings and all traffic goes back to cloud immediately.

### Phase 3: Default Switch Notification

**In-app notification (one-time, when default flips):**
```
┌─────────────────────────────────────────────────────┐
│ Bonsai Local Inference Now Active by Default        │
├─────────────────────────────────────────────────────┤
│                                                     │
│ We've enabled local code inference for all users.   │
│ Your completions are now processed on your device.  │
│                                                     │
│ Cloud fallback is still enabled. If anything goes   │
│ wrong, we'll automatically fall back to cloud.      │
│                                                     │
│ Performance summary from your usage:                │
│ • Local latency: 320ms (avg)                        │
│ • Cloud latency: 680ms (avg)                        │
│ • Speed improvement: 53%                            │
│ • Privacy: 100% (code stays on device)              │
│                                                     │
│ [Disable Local Mode] [Settings] [Feedback]          │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Phase 4: Offline Mode Documentation

**Tutorial: "Full Offline Sovereign Mode"**
```markdown
# Going Fully Offline

If you want maximum privacy and zero cloud dependency, 
Bonsai Sovereign Proxy offers offline-only mode.

## Enable Full Offline Mode

1. Open VS Code Settings
2. Search for "bonsai.extensions.copilot.offline_only"
3. Toggle to ON

## What Changes

- Code completions are processed entirely locally
- No cloud fallback (if local inference fails, you get an error)
- All conversations are stored locally in your Bonsai KDB
- You can export conversations to a file anytime
- Your code never touches the internet

## Minimum Requirements

- 4GB RAM (8GB recommended)
- 2GB free disk space for model
- Local inference adds ~3-5% CPU usage during completions

## Reverting to Hybrid Mode

Toggle "offline_only" to OFF in settings. 
Cloud fallback reactivates immediately.

## Exporting Offline Data

Run: `bonsai-cli export conversations --format zip`
Creates a ZIP file with all offline conversations and settings.
```

### Support Training Materials

**Knowledge base articles:**
1. "Bonsai Local Inference FAQ" (see above)
2. "Why is local inference slow?" (troubleshooting guide)
3. "Data privacy in local inference mode" (security explainer)
4. "Troubleshooting local inference fallbacks" (support flowchart)
5. "Enabling Bonsai for enterprise: admin guide" (deployment guide)

**Support escalation path:**
```
User reports issue
    ↓
Auto-reply: "Thanks for reporting. Is this about [slow/wrong/crash/privacy]?"
    ↓
User selects category
    ↓
Auto-run diagnostic:
  - Collect local logs (without code content)
  - Check device specs (CPU, RAM, OS)
  - Check network latency and loss
  - Check model version and checksums
    ↓
If diagnostic shows known issue:
  → Auto-reply with fix (e.g., "Update to v0.2.3")
    ↓
If not known issue:
  → Escalate to engineer with diagnostic bundle
```

---

## 8. Admin & Enterprise Controls

### Organization-Wide Policies

**Admin blueprint control:**
```bash
# Org policy: Enforce local inference for all users
bonsai-cli org-policy set \
  --name "local-inference-mandatory" \
  --apply-to-users "group:engineering@company.com" \
  --flags '{"use_local_inference": true}' \
  --enforcement "hard"  # User cannot override

# Org policy: Require cloud-only for compliance
bonsai-cli org-policy set \
  --name "cloud-only-compliance" \
  --apply-to-users "group:finance@company.com" \
  --flags '{"offline_only": false, "cloud_fallback": true}' \
  --enforcement "hard"

# Org policy: Optional local inference with cloud fallback (default)
bonsai-cli org-policy set \
  --name "hybrid-mode-default" \
  --apply-to-users "group:all@company.com" \
  --flags '{"use_local_inference": true, "cloud_fallback": true}' \
  --enforcement "soft"  # User can override
```

### Audit Logging

**Every policy change is logged:**
```json
{
  "timestamp": "2026-06-01T14:23:45Z",
  "admin_id": "alice@company.com",
  "action": "org-policy-set",
  "policy_name": "local-inference-mandatory",
  "affected_users": 450,
  "flags": {
    "use_local_inference": true
  },
  "enforcement": "hard",
  "approved_by": "bob@company.com",
  "audit_trail_id": "audit-2026-06-01-142345"
}
```

**Per-user usage audit:**
```bash
bonsai-cli audit log \
  --user "alice@company.com" \
  --date-range "2026-05-01..2026-06-01" \
  --output csv

# Output:
# timestamp,action,inference_mode,success
# 2026-05-31T09:15:30Z,completion,local,true
# 2026-05-31T09:15:45Z,completion,local,false→cloud,true
# 2026-05-31T09:16:02Z,chat,cloud,true
```

### Compliance & Data Residency

**Data residency attestation:**
```bash
# Verify all data for org stays in region
bonsai-cli compliance verify-data-residency \
  --org "company.com" \
  --region "eu-west-1" \
  --report-format "pdf"

# Report:
# ✅ Completions processed locally: 98.7%
# ✅ Cloud fallbacks routed to eu-west-1: 100%
# ✅ No data stored outside region: PASS
# ✅ Audit trail encrypted: PASS
# Attestation: Valid until 2026-08-01
```

---

## 9. Success Metrics & Go/No-Go Criteria

### Phase 1 (Idle Mode)

| Metric | Target | Go | No-Go |
|--------|--------|----|----|
| Extension install rate | >80% of active users | ≥80% | <60% |
| Uninstall rate (1 week) | <1% | <1% | >3% |
| Cloud latency regression | 0% | <2% worse | >5% worse |
| Extension crash rate | 0% | <0.01% | >0.05% |
| Telemetry accuracy | >95% | >95% | <90% |

### Phase 2 (Local Inference Opt-In)

| Metric | Week 4 | Week 6 | Week 8 | Go Decision |
|--------|--------|--------|--------|---|
| Adoption rate | 2% | 5% | 20% | ≥20% → Phase 3 |
| Fallback rate | <3% | <2% | <2% | <2% → Phase 3 |
| Satisfaction | 3.5/5 | 4.0/5 | 4.0/5 | ≥4.0/5 → Phase 3 |
| P99 latency | <2.5s | <1.5s | <1.5s | <1.5s → Phase 3 |
| Error rate | <2% | <1% | <1% | <1% → Phase 3 |
| Critical issues | 0 | 0 | 0 | 0 issues → Phase 3 |

**No-Go criteria:** Fail any metric on week 8 → extend Phase 2 by 4 weeks and re-test.

### Phase 3 (Hybrid Default)

| Metric | Target | Success | Failure |
|--------|--------|---------|---------|
| Adoption (implicit, not opted out) | 70% | ≥70% | <50% |
| Opt-out rate | <30% | ≤30% | >40% |
| Fallback rate | <5% | ≤5% | >10% |
| NPS (change from Phase 2) | stable or ↑ | ≥ | ↓ |
| Support tickets | <0.1% of users | <0.1% | >0.5% |
| Session retention | >80% | ≥80% | <60% |

**Go to Phase 4:** All metrics pass after 4 weeks in Phase 3.

### Phase 4 (Full Offline)

| Metric | Target | Success | Failure |
|--------|--------|---------|---------|
| Offline adoption | 10% | ≥10% | <5% |
| Offline retention | 75% | ≥75% | <60% |
| Data loss incidents | 0 | 0 | >0 |
| Feature adoption (predict-URV) | 5% | ≥5% | <1% |

---

## 10. Implementation Roadmap

### Code Changes Required

**Bonsai workspace (Tauri):**
1. `bonsai-workspace/src-tauri/src/features.rs` → Add extension feature flags
2. `bonsai-workspace/src-tauri/src/lib.rs` → Expose flag getters/setters to frontend
3. New file: `bonsai-workspace/src-tauri/src/extension_proxy.rs` → Proxy traffic routing logic
4. New file: `bonsai-workspace/src-tauri/src/extension_telemetry.rs` → Telemetry collection

**VS Code extension (new or updated):**
1. Create: `bonsai-extensions/extension/src/config.ts` → Feature flag schema
2. Create: `bonsai-extensions/extension/src/telemetry.ts` → Client-side telemetry
3. Create: `bonsai-extensions/extension/src/proxy-mode-indicator.ts` → UI showing inference mode
4. Create: `bonsai-extensions/extension/src/settings-ui.ts` → Settings panel

**Backend telemetry service:**
1. Create new service: `crates/bonsai-extension-telemetry/`
2. Ingest telemetry from proxy
3. Calculate aggregate metrics (adoption, fallback rate, latency percentiles)
4. Publish to dashboard

### Timeline

| Week | Deliverable |
|------|-------------|
| 1 | Implement feature flags in `features.rs`; expose to frontend |
| 2 | Build extension proxy routing logic; idle mode working |
| 3 | Implement telemetry collection; Phase 1 dashboard |
| 4 | Deploy Phase 1 to internal testing |
| 5 | Deploy Phase 1 to early adopters; monitor |
| 6 | Deploy Phase 1 to general release |
| 7 | Implement Phase 2 feature flags; local inference routing |
| 8 | Deploy Phase 2 code; opt-in UI in settings |
| 9-10 | Phase 2 early adopter testing; telemetry validation |
| 11-12 | Phase 2 expansion to 5%→20% |
| 13-14 | Phase 3 code: flip defaults |
| 15-16 | Phase 3 deployment and monitoring |
| 17 | Phase 4 design and planning |

---

## 11. Glossary

| Term | Definition |
|------|-----------|
| **Bonsai Sovereign Proxy** | VS Code extension that intercepts Copilot/Claude Code requests and routes them to local inference |
| **Idle Mode** | Phase 1: proxy installed but all traffic routes to cloud |
| **Hybrid Mode** | Phase 2-3: proxy routes some traffic locally, falls back to cloud on failure |
| **Offline Mode** | Phase 4: proxy routes all traffic locally, no cloud fallback |
| **Feature Flag** | Runtime-controllable boolean that enables/disables features (e.g., `use_local_inference`) |
| **Cloud Fallback** | Automatic routing to cloud API if local inference fails |
| **Telemetry** | Anonymized usage metrics (latency, error rate, adoption, etc.) |
| **Go/No-Go** | Decision point: does migration proceed to next phase? |
| **Adoption Rate** | % of users with `use_local_inference: true` |
| **Fallback Rate** | % of local inference requests that failed and fell back to cloud |
| **NPS** | Net Promoter Score (user satisfaction metric) |

---

## 12. Success Criteria (Final)

A successful migration delivers:

1. ✅ **Zero feature loss** — Users can rollback to 100% cloud mode anytime with 1 click
2. ✅ **Smooth UX** — Users barely notice the transition; in-app guidance is clear
3. ✅ **High adoption** — >70% of users opt into local inference within 12 weeks
4. ✅ **Low regression** — Error rate and latency don't degrade (or improve)
5. ✅ **Measurable improvement** — Average latency improves by >30% vs pure cloud
6. ✅ **User confidence** — NPS score increases or stays stable (>40)
7. ✅ **Enterprise ready** — Admins can enforce policies; compliance logging works
8. ✅ **No data loss** — Zero conversation loss incidents across all phases

---

*← [OS Core Blueprint](10-OS-CORE-BLUEPRINT.md) · [Sovereignty →](10-SOVEREIGNTY.md)*
