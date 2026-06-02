# Extension Migration Strategy: Complete Design Package

> A 12-week, phased plan to migrate Copilot/Claude Code users to Bonsai Sovereign Proxy with zero feature loss and 100% user control.

---

## Overview

**Goal:** Transition existing VS Code extension users from cloud-dependent Copilot/Claude Code to the Bonsai Sovereign Proxy without breaking workflows or losing data.

**Strategy:** 4-phase migration (12 weeks)
1. **Phase 1 (Weeks 1-2):** Proxy installation in idle mode (no behavior change)
2. **Phase 2 (Weeks 3-8):** Local inference opt-in with cloud fallback
3. **Phase 3 (Weeks 9-12):** Hybrid mode as default (but cloud-only is always available)
4. **Phase 4 (Month 4+):** Full offline sovereignty for advanced users (opt-in)

**Key Properties:**
- ✅ 100% optional at every step
- ✅ Zero feature loss; full rollback capability
- ✅ Measurable telemetry at every phase
- ✅ Automatic fallback if quality degrades
- ✅ Enterprise admin controls and compliance logging

---

## Documents in This Package

### 1. **[16-EXTENSION-MIGRATION-STRATEGY.md](16-EXTENSION-MIGRATION-STRATEGY.md)** — Strategic Plan (2,000 words)

**Read this for:**
- Complete 4-phase roadmap with success criteria
- Feature flag specification and dependencies
- Telemetry design and dashboard layout
- Risk matrix with mitigation strategies
- Backward compatibility guarantees
- User communication templates (in-app notifications, FAQ, support guides)
- Admin/enterprise controls for org-wide deployment

**Key Sections:**
1. Migration Roadmap (phases 1-4, weeks 1-12)
2. Feature Flag Specification (complete flag list, hot-reload, validation)
3. Telemetry & Monitoring (metrics, dashboard examples, alert conditions)
4. Risk Assessment (likelihood/impact matrix, contingency plans)
5. Backward Compatibility (extension version matrix, sunset schedule)
6. User Communication Plan (notifications, FAQ, tutorial)
7. Admin & Compliance (org policies, audit logging, data residency)
8. Success Metrics & Go/No-Go Criteria

---

### 2. **[16-EXTENSION-MIGRATION-IMPLEMENTATION.md](16-EXTENSION-MIGRATION-IMPLEMENTATION.md)** — Code Design (1,500 words)

**Read this for:**
- Rust code structure for feature flags, proxy routing, telemetry
- Tauri command interface (get/set/reload config)
- Extension proxy architecture (local → cloud fallback logic)
- Telemetry collection implementation
- Phase-by-phase development checklist
- Testing strategy (unit, integration, UAT)
- Monitoring setup (Prometheus metrics, alert rules)

**Key Sections:**
1. Feature Flag Architecture (`extension_config.rs`)
2. Proxy Traffic Routing (`extension_proxy.rs`)
3. Telemetry Collection (`extension_telemetry.rs`)
4. Phase-by-Phase Development Checklist
5. Testing Checklist (unit, integration, UAT)
6. Monitoring & Alerting (Prometheus, alert rules)

---

### 3. **[16-EXTENSION-MIGRATION-CHECKLIST.md](16-EXTENSION-MIGRATION-CHECKLIST.md)** — Execution Plan (2,000 words)

**Read this for:**
- Week-by-week execution tasks
- Daily/hourly monitoring schedules
- Go/no-go decision criteria for each phase
- Telemetry dashboard examples
- Risk response playbooks (fallback rate spike, NPS drop, data loss)
- Sign-offs required at each phase gate

**Key Sections:**
1. Phase 1 Execution (Weeks 1-2, internal test → general release)
2. Phase 2 Execution (Weeks 3-8, 1k → 5% → 20% rollout)
3. Phase 3 Execution (Weeks 9-12, flip default to local)
4. Phase 4 Execution (Month 4+, full offline opt-in)
5. Ongoing Monitoring (daily, weekly, monthly cadence)
6. Risk Response Playbooks (3 scenarios: fallback spike, NPS drop, data loss)
7. Sign-Offs & Approvals (engineering, product, ops, support)

---

## Quick Reference

### Feature Flags (At a Glance)

**Phase 1-2:**
```yaml
bonsai.extensions.copilot:
  enabled: true
  use_local_inference: false      # Phase 2: user can opt-in
  cloud_fallback: true             # Always available
  offline_only: false              # Phase 4 only
```

**Phase 3+ (defaults flipped):**
```yaml
bonsai.extensions.copilot:
  enabled: true
  use_local_inference: true        # Default in Phase 3
  cloud_fallback: true             # Transparent fallback
  offline_only: false              # Phase 4: user can opt-in
```

**Phase 4 (advanced users):**
```yaml
bonsai.extensions.copilot:
  enabled: true
  use_local_inference: true
  cloud_fallback: false            # Disabled; no cloud fallback
  offline_only: true               # Full offline mode
  feature_flags:
    predict_urv: true              # AI-driven scheduling
    traffic_classification: true    # ML-based prioritization
```

### Phase Success Criteria

| Phase | Duration | Key Metric | Target | Go Decision |
|-------|----------|-----------|--------|---|
| **1** | Weeks 1-2 | Uninstall rate | <1% | Telemetry stable, 0 crashes |
| **2** | Weeks 3-8 | Fallback rate | <2% | Adoption >20%, satisfaction >4.0 |
| **3** | Weeks 9-12 | Opt-out rate | <30% | Adoption >70%, NPS stable, <0.1% support tickets |
| **4** | Month 4+ | Offline adoption | >10% | Zero data loss, feature usage >5% |

### Timeline at a Glance

```
Week 1-2:   Phase 1 (idle mode installation)
            └─ Internal test → Early adopters → General release
            └─ Go/no-go: telemetry pipeline working?

Week 3-8:   Phase 2 (local inference opt-in)
            └─ Week 3-4: 1,000 users (1%)
            └─ Week 5-6: 50,000 users (5%)
            └─ Week 7-8: 200,000 users (20%)
            └─ Go/no-go: fallback <2%, satisfaction >4.0?

Week 9-12:  Phase 3 (hybrid as default)
            └─ Flip default to use_local_inference: true
            └─ Monitor opt-out reasons
            └─ Go/no-go: adoption >70%, NPS stable?

Month 4+:   Phase 4 (full offline sovereignty)
            └─ Enable offline_only: true
            └─ Advanced feature flags unlock
            └─ Target power users & privacy advocates
```

---

## Key Design Decisions

### 1. Why Four Phases?

**Phase 1 (Idle Mode):** Establish baseline telemetry without any behavior change. This lets us validate the telemetry infrastructure and ensure cloud APIs are stable before introducing local inference.

**Phase 2 (Opt-In):** Give early adopters the choice. This builds confidence in the local model while giving us a fallback to cloud. We can measure quality and performance at scale before making it the default.

**Phase 3 (Default):** Flip to local by default, but preserve cloud fallback. This is where most users transition. Cloud remains a safety net.

**Phase 4 (Offline):** For advanced users who want zero cloud dependency. Fully opt-in; not pushed on anyone.

### 2. Why Feature Flags Over Hard Toggles?

**Feature flags are hot-reloadable.** Users can change `use_local_inference` in VS Code settings and the change takes effect immediately (no restart). This is critical for iterating quickly during rollout and letting users test local vs cloud side-by-side.

### 3. Why Cloud Fallback is Essential

**Local inference will sometimes fail:** timeouts, OOM, model bugs, network-of-things issues. Transparent cloud fallback means users **never see a failure**. The proxy silently retries on cloud. They just get their completion a bit slower.

This is what makes the migration risk-free: if local breaks, cloud catches it.

### 4. Why Telemetry is Detailed

**We can't improve what we don't measure.** Phase 2 is opt-in, so adoption will be low (20-25%). To know if we should expand, we need to measure:
- Fallback rate (is local stable?)
- Latency percentiles (is it actually faster?)
- Rejection rate (do users like the quality?)
- Device impact (battery drain, CPU usage)
- User satisfaction (NPS, 1-5 ratings)

With this data, we can make go/no-go decisions with confidence.

---

## What Makes This Plan Risky-Minimal

### Risk #1: Local Inference is Slower Than Cloud
**Mitigation:** Phase 2 measures latency. If local is >50% slower in P99, we pause expansion and optimize. If optimization doesn't help, we extend Phase 2.

### Risk #2: Local Inference Quality is Worse
**Mitigation:** Phase 2 tracks rejection rate (% of completions user rejects). If >10% worse than cloud, we don't proceed. If it's acceptable, we continue.

### Risk #3: User Confusion During Migration
**Mitigation:** Clear in-app notifications at each phase transition. Detailed FAQ. Support training. Video tutorials.

### Risk #4: Organization Blocks Local Mode (Compliance)
**Mitigation:** Admin can enforce `cloud_only: true` org-wide. Audit logging shows which users are on local vs cloud. Attestation reports for compliance.

### Risk #5: Conversation Data Loss
**Mitigation:** This is critical. We **never** enable checkpoint features until we're 100% sure it's safe. Phase 2 is opt-in preview. Phase 3 is still opt-out. Phase 4 requires explicit enabling. If any data loss happens, we immediately disable the feature and investigate.

---

## How to Use This Design

### For Engineering

1. **Start with [Implementation Guide](16-EXTENSION-MIGRATION-IMPLEMENTATION.md)**
   - Understand the code structure (feature flags, proxy, telemetry)
   - Implement Phase 1 core (extension_config.rs, idle mode proxy)
   - Write unit tests for flag validation and routing logic

2. **Reference [Execution Checklist](16-EXTENSION-MIGRATION-CHECKLIST.md) Weekly**
   - Week 1: complete Phase 1 dev tasks
   - Week 2: internal testing and monitor dashboards
   - Week 3-8: Phase 2 expansion (watch metrics closely)
   - Weeks 9-12: Phase 3 monitoring and opt-out analysis

3. **Create the Monitoring Dashboard**
   - Export Prometheus metrics (latency percentiles, fallback rate, error rate)
   - Build web dashboard (see examples in Strategy guide)
   - Set up alert conditions (red-line metrics that trigger pause/rollback)

### For Product/PM

1. **Read [Strategy Guide](16-EXTENSION-MIGRATION-STRATEGY.md) sections:**
   - Phase roadmap (1-4)
   - Telemetry & success metrics
   - User communication plan
   - Risk matrix and contingencies

2. **Prepare User Communications**
   - In-app notifications for each phase (templates provided)
   - FAQ articles (templates provided)
   - Support team training materials

3. **Plan Go/No-Go Reviews**
   - End of Phase 1: telemetry pipeline ready? Crashes? → GO / NO-GO
   - Week 4 (Phase 2): fallback <2%? satisfaction >3.5? → GO / NO-GO
   - Week 8 (Phase 2): ready to flip default? → GO / NO-GO
   - Week 12 (Phase 3): adoption >70%? NPS stable? → GO to Phase 4 / STAY in Phase 3

### For Ops/Reliability

1. **Implement Monitoring & Alerting** ([Implementation Guide](16-EXTENSION-MIGRATION-IMPLEMENTATION.md))
   - Prometheus metrics for latency, fallback rate, errors
   - Real-time dashboard (check hourly during Phase 1-2)
   - Alert rules: fallback >5%, latency regression >50%, error >3%

2. **Plan Rollback Procedures**
   - If Phase 2 fallback rate >5%: can we roll back the flag?
   - If Phase 3 crashes spike: can we re-enable cloud-only?
   - Keep rollback plans ready but don't execute unless necessary

3. **Support the Phased Rollout**
   - Week 1-2: internal testing (on-call engineers monitoring)
   - Week 3-8: expand cohorts (pause expansion if any red-line metric fails)
   - Week 9-12: Phase 3 on all users (respond quickly to issues)

---

## Next Steps

1. **Week 1 (Starting Now)**
   - Engineering: read Implementation Guide, start Phase 1 code
   - Product: prepare user communication templates
   - Ops: design monitoring dashboard, alert rules

2. **Week 2**
   - Engineering: complete Phase 1 code, unit tests
   - Product: finalize notification copy, FAQ
   - Ops: deploy dashboard, test alert firing

3. **Week 3**
   - Engineering: Phase 1 internal testing (Bonsai team)
   - Product: prepare Phase 2 opt-in messaging
   - Ops: validate telemetry pipeline

4. **Week 4+**
   - Follow [Execution Checklist](16-EXTENSION-MIGRATION-CHECKLIST.md) week-by-week

---

## FAQ

**Q: Why not just push local inference to everyone immediately?**
A: Risk. If local inference has bugs, we break existing workflows for millions of users. Phased approach lets us validate quality incrementally.

**Q: Can users always rollback to cloud?**
A: Yes. At any point, user can set `cloud_only: true` in VS Code settings. All traffic routes to cloud immediately. No restart needed.

**Q: What if local inference is slower?**
A: Phase 2 measures P50, P95, P99 latency. If local P99 is >50% worse than cloud, we pause expansion and optimize (or reconsider approach).

**Q: What if users hate the migration?**
A: Phase 3 default flip can be delayed or abandoned. We'll know by week 12 (Phase 3 go/no-go decision) whether adoption is healthy (>70%) and NPS is stable/increasing.

**Q: Do we need user consent to collect telemetry?**
A: Yes. Telemetry is anonymized (no user ID, no code content, just metrics). Users can opt-out via checkbox in settings.

---

## Success Definition

A successful migration delivers:

- ✅ **Zero feature loss** — Users can rollback to 100% cloud mode anytime
- ✅ **Smooth UX** — Users barely notice the transition
- ✅ **High adoption** — >70% of users on local inference within 12 weeks
- ✅ **Low regression** — Error rate and latency don't degrade (or improve)
- ✅ **Measurable improvement** — Average latency improves by >30% vs pure cloud
- ✅ **User confidence** — NPS score increases or stays stable (>40)
- ✅ **Enterprise ready** — Admins can enforce policies; compliance logging works
- ✅ **No data loss** — Zero conversation loss incidents across all phases

---

## Document Cross-References

| Document | Purpose | Audience |
|----------|---------|----------|
| **[16-EXTENSION-MIGRATION-STRATEGY.md](16-EXTENSION-MIGRATION-STRATEGY.md)** | Complete strategic plan, phase roadmap, feature flag spec, telemetry design, risk matrix, user comms, success criteria | PMs, Design, Eng leads |
| **[16-EXTENSION-MIGRATION-IMPLEMENTATION.md](16-EXTENSION-MIGRATION-IMPLEMENTATION.md)** | Code design, Rust structures, proxy routing, telemetry collection, testing strategy | Engineers, QA |
| **[16-EXTENSION-MIGRATION-CHECKLIST.md](16-EXTENSION-MIGRATION-CHECKLIST.md)** | Week-by-week execution tasks, daily monitoring, go/no-go decisions, risk playbooks, approval sign-offs | Engineers, Ops, PMs, Leads |
| **(This document)** | High-level overview, navigation guide, quick reference, next steps | Everyone |

---

## Contact & Escalation

**For questions on strategy:** Contact Product Lead
**For implementation details:** Contact Engineering Lead
**For monitoring/ops:** Contact Reliability Engineer
**For user comms:** Contact Product Manager

---

*← [OS Core Blueprint](10-OS-CORE-BLUEPRINT.md) · [Sovereignty →](10-SOVEREIGNTY.md)*
