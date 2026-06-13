# Bonsai Linting System – Phase Roadmap

**Total Scope:** 5 Phases to complete the production-grade universal linting platform  
**Current Status:** Phase A Complete ✓ (2026-06-01)

---

## Phase A: Self-Improving Rules ✓ COMPLETE

**Objective:** Enable continuous learning from user feedback to improve rule accuracy  
**Duration:** ~4 weeks (completed)  
**Status:** All deliverables complete and tested

### Deliverables
- ✓ EternalTrainingLoop orchestrator (6-stage pipeline)
- ✓ FeedbackCollector (IDE integration points)
- ✓ RuleConfidenceCalculator (dynamic scoring algorithm)
- ✓ RuleConfidenceAdjuster (severity updates)
- ✓ RuleRefiner (pattern mutation proposals)
- ✓ ETLStorage (persistence layer)
- ✓ UniverseEventEmitter (observability)
- ✓ ETL Daemon (24-hour scheduling)
- ✓ 25+ integration tests
- ✓ Comprehensive documentation

### Key Metrics Enabled
- Rule confidence scores (0.0–1.0 per rule)
- False positive rate tracking per rule
- Rule dismissal patterns
- Fix success rates
- Cross-rule performance comparison

### Example Outputs
```
Rule: "unused-import"
  Confidence: 0.92
  Action: promote_to_error
  True Positives: 245
  False Positives: 18
  Dismissed: 5
  Fix Success Rate: 97%

Rule: "clippy-pedantic"
  Confidence: 0.58
  Action: demote_to_hint
  True Positives: 120
  False Positives: 95
  Dismissed: 40
  Fix Success Rate: 65%
  Mutation Proposal: Refine pattern to reduce FPs by ~50%
```

---

## Phase B: Persistent Knowledge + Collaborative Linting

**Objective:** Enable cross-project rule learning and team collaboration  
**Estimated Duration:** 2–3 weeks  
**Start:** Post-Phase A integration (Week 4)

### Key Components

**B1: Persistent Salsa Index**
- Incremental parse cache across sessions
- Workspace-level AST memoization
- Blast radius tracking for file changes
- Expected improvement: 10x faster re-linting of large codebases

**B2: Knowledge Database (KDB) Integration**
- Store rule metrics in .kmod files
- Cross-project rule performance aggregation
- Industry-wide rule benchmarks
- Expected improvement: Rules pre-trained on 1,000+ open-source projects

**B3: Collaborative Linting**
- Team rule profiles
- Shared rule customizations
- Cross-team rule voting
- Expected improvement: 5–10% better coverage through crowd feedback

### Architecture
```
Phase A (ETL) → KDB (metrics storage) → Phase B (persistence)
                     ↓
            Cross-project aggregation
                     ↓
            Industry benchmarks
                     ↓
            Collaborative voting
```

### Deliverables (Est. 30 Rust files, 5,000+ LOC)
- [ ] Salsa query group for parse cache
- [ ] Incremental file dependency tracking
- [ ] Blast radius calculation engine
- [ ] KDB write/read layer
- [ ] Rule aggregation engine
- [ ] Collaborative voting system
- [ ] Rule conflict resolution
- [ ] Performance dashboards
- [ ] Integration tests (50+)

---

## Phase C: Axiom Formal Verification + Predictive Linting

**Objective:** Enable formal verification and AI-powered predictive suggestions  
**Estimated Duration:** 4–6 weeks  
**Start:** After Phase B stabilizes

### Key Components

**C1: Axiom Integration**
- Formal verification of rule correctness
- Proof-based rule validation
- Type system integration
- Expected improvement: Zero false positives for critical rules

**C2: Predictive Linting**
- ML model predicts issues before they occur
- Intent inference from code patterns
- Code smell early detection
- Expected improvement: 2–3x more proactive issue detection

**C3: Natural Language Rule Generation**
- Users describe rules in English
- AI generates patterns and enforcement code
- Expected improvement: Rule creation time from 2 hours → 5 minutes

### Axiom Omnisystem Languages
- **Titan** (Type system formalization)
- **Aether** (Code transformation proofs)
- **Sylva** (Graph-based reasoning)
- **Axiom** (Constraint satisfaction)

### Deliverables (Est. 40 Rust files, 8,000+ LOC)
- [ ] Axiom interpreter integration
- [ ] Rule proof generator
- [ ] Type system validator
- [ ] ML model training pipeline
- [ ] Intent classifier
- [ ] Natural language parser
- [ ] Pattern generator from NL
- [ ] Proof verification tests
- [ ] Integration with IDE (new diagnostic level: "predictive")

---

## Phase D: Grammar Checking + Plugin Marketplace

**Objective:** Enable spell-checking and extensibility via plugin system  
**Estimated Duration:** 3–4 weeks  
**Start:** After Phase C reaches stability

### Key Components

**D1: Spell Checking System**
- Hunspell LSP server (for 80+ languages)
- Code-aware text extraction (ignoring identifiers)
- Multi-language detection (whatlang)
- Expected improvement: Catch documentation typos, string mistakes

**D2: Grammar Analysis**
- LanguageTool integration
- Configurable grammar rules
- Documentation-aware enforcement
- Expected improvement: Professional-quality code comments

**D3: Plugin Marketplace**
- .bkp plugin distribution
- Community rule sharing
- Rating and review system
- Expected improvement: 10,000+ community rules available

### Deliverables (Est. 25 Rust files, 4,000+ LOC)
- [ ] Hunspell LSP server
- [ ] Multi-language detection
- [ ] Code-text splitting
- [ ] LanguageTool integration
- [ ] Plugin manager UI
- [ ] Rule marketplace infrastructure
- [ ] Plugin signing and verification
- [ ] Auto-update system
- [ ] Plugin dependency resolution

---

## Phase E: Formal SLAs + Auto-Calibration

**Objective:** Enable production-grade observability and self-tuning  
**Estimated Duration:** 3–4 weeks  
**Start:** After Phase D integration

### Key Components

**E1: Formal SLAs**
- Rule performance guarantees
- False positive rate contracts
- Coverage targets by language
- Expected improvement: Rules meet SLO commitments

**E2: Auto-Calibration**
- Automatic rule tuning based on user feedback
- Severity self-adjustment
- Pattern evolution over time
- Expected improvement: Rules improve without manual intervention

**E3: Production Observability**
- Real-time metrics dashboards
- Alerting on rule degradation
- Rule health scoring
- Expected improvement: Immediate visibility into rule performance

### Deliverables (Est. 30 Rust files, 6,000+ LOC)
- [ ] SLA contract system
- [ ] Performance tracker
- [ ] Auto-tuning engine
- [ ] Dashboard UI
- [ ] Alerting system
- [ ] Metrics aggregation
- [ ] Historical analysis
- [ ] Forecasting models
- [ ] Cost analysis tools

---

## Technology Stack Progression

| Phase | Core Tech | Added Tech | Remove Tech |
|-------|-----------|-----------|-------------|
| A | Tokio, Serde, Chrono | Tracing, DashMap | – |
| B | + Salsa DB | SQLx, HNSW | – |
| C | + Axiom | TensorFlow, ONNX | – |
| D | + LSP Server | Hunspell, LanguageTool | – |
| E | + Metrics | Prometheus, Grafana | – |

---

## Feature Matrix

| Feature | Phase A | Phase B | Phase C | Phase D | Phase E |
|---------|---------|---------|---------|---------|---------|
| Real-time linting | ✓ | ✓✓ | ✓✓ | ✓✓ | ✓✓ |
| Rule confidence | ✓ | ✓ | ✓ | ✓ | ✓ |
| Incremental parsing | ✓ | ✓✓ | ✓✓ | ✓✓ | ✓✓ |
| Persistence | ✓ | ✓✓ | ✓✓ | ✓✓ | ✓✓ |
| Cross-project learning | – | ✓ | ✓ | ✓ | ✓ |
| Formal verification | – | – | ✓ | ✓ | ✓ |
| Predictive analysis | – | – | ✓ | ✓ | ✓ |
| Spell checking | – | – | – | ✓ | ✓ |
| Plugin system | – | – | – | ✓ | ✓ |
| SLA enforcement | – | – | – | – | ✓ |
| Auto-calibration | – | – | – | – | ✓ |
| Production monitoring | – | – | – | – | ✓ |

---

## Resource Requirements by Phase

| Phase | Engineers | Duration | Language Coverage | Rules Generated |
|-------|-----------|----------|-------------------|-----------------|
| A | 1 | 4 weeks | 30+ | 80+ |
| B | 2 | 3 weeks | 30+ | 500+ |
| C | 2 | 6 weeks | 30+ | 5,000+ |
| D | 1 | 4 weeks | 30+ | 10,000+ |
| E | 1 | 4 weeks | 30+ | 10,000+ |

---

## Milestones

### Q2 2026 (Current)
- [x] Phase A implementation (2026-06-01)
- [ ] Phase A integration (by 2026-06-14)
- [ ] Phase A production deployment (by 2026-06-21)

### Q3 2026
- [ ] Phase B implementation (by 2026-07-19)
- [ ] Phase B integration & deployment (by 2026-08-02)
- [ ] Phase C implementation (by 2026-09-13)

### Q4 2026
- [ ] Phase C integration & deployment (by 2026-10-11)
- [ ] Phase D implementation (by 2026-11-08)
- [ ] Phase E implementation (by 2026-12-06)

### 2027
- [ ] Full production deployment (by 2027-01-31)
- [ ] 10,000+ rules library
- [ ] 10M+ monthly active users
- [ ] Industry-wide adoption

---

## Success Criteria

### Phase A
- [x] Confidence scoring accuracy > 95%
- [x] False positive detection working
- [x] Dismissal factor affecting score
- [x] 25+ tests passing
- [ ] IDE integration working (in progress)
- [ ] 100+ feedback events/day captured (pending integration)

### Phase B
- [ ] Parse cache hit rate > 80%
- [ ] Re-linting speedup 10x
- [ ] KDB storing metrics for 100+ rules
- [ ] Cross-project rule matching > 80% accuracy
- [ ] Collaborative voting working

### Phase C
- [ ] Axiom proof generation working
- [ ] Formal verification > 99% accurate
- [ ] ML model accuracy > 85%
- [ ] Predictive suggestions > 70% relevant
- [ ] NL rule generation working

### Phase D
- [ ] Spell checking > 95% precision
- [ ] Grammar rules for 80+ languages
- [ ] Plugin system supporting 100+ plugins
- [ ] Marketplace with 1,000+ community rules
- [ ] Plugin auto-update working

### Phase E
- [ ] SLA enforcement > 99.9%
- [ ] Auto-calibration improving rules 5%/month
- [ ] Dashboard real-time updates < 100ms
- [ ] Alerting latency < 1 minute
- [ ] Forecasting accuracy > 80%

---

## Dependencies & Blockers

### Phase B Dependencies
- [x] Phase A complete
- [ ] Rule registry with update capability
- [ ] KDB module available
- [ ] Salsa integration guide

### Phase C Dependencies
- [x] Phase A complete
- [ ] Phase B complete
- [ ] Axiom Omnisystem bindings
- [ ] ML training infrastructure

### Phase D Dependencies
- [x] Phase A complete
- [ ] Phase B complete
- [ ] Plugin architecture defined
- [ ] Marketplace backend

### Phase E Dependencies
- [x] Phase A complete
- [ ] Phase B complete
- [ ] Phase C complete
- [ ] Phase D complete
- [ ] Metrics infrastructure

---

## Risk Assessment

| Risk | Impact | Likelihood | Mitigation |
|------|--------|-----------|-----------|
| Axiom integration delays | HIGH | MEDIUM | Start C in parallel with B |
| Plugin security issues | HIGH | LOW | Implement signing/sandboxing early |
| Confidence accuracy degradation | MEDIUM | LOW | Comprehensive testing in A |
| Cross-project data leaks | HIGH | LOW | Privacy controls in B |
| Rule performance regression | MEDIUM | MEDIUM | E2E testing for all phases |

---

## Open Questions for Discussion

1. **Phase B Timeline:** Start immediately after Phase A integration or wait for production stability?
2. **Plugin Marketplace:** In-house only or open to community submissions?
3. **Axiom Formalization:** Focus on critical rules only or all rules?
4. **SLA Enforcement:** Hard constraints or soft targets?
5. **Multi-Organization:** Support for enterprise deployments in Phase C or Phase E?

---

## References

- **Phase A Implementation:** `docs/26-PHASE-A-IMPLEMENTATION-SUMMARY.md`
- **Linter Architecture:** `docs/22-UNIVERSAL-LINTER.md`
- **Integration Guide:** `docs/23-LINTER-INTEGRATION.md`
- **Blueprint:** `docs/25-SELF-IMPROVING-RULES-BLUEPRINT.md`

---

**Last Updated:** 2026-06-01  
**Next Review:** Post-Phase A integration (2026-06-21)
