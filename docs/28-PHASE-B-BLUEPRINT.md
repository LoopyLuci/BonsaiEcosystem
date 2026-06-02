# Phase B: Persistent Knowledge + Collaborative Linting – Blueprint

**Scope:** 2–3 weeks  
**Goal:** Enable cross-session performance, cross-project learning, and team collaboration  
**Status:** Design phase

---

## Vision

Phase B transforms Phase A's real-time learning into **institutional knowledge**:

1. **Persistent Salsa Index** – Parse trees cached across sessions (10x speedup)
2. **Dependency Tracking** – Understand code relationships for smart re-linting
3. **KDB Integration** – Rules trained on 1,000+ open-source projects
4. **Team Collaboration** – Share rule configurations, vote on improvements

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Persistent Salsa Index                      │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Parse Cache (SQLite)                              │  │
│  │  - Serialize Tree objects to bincode              │  │
│  │  - Cache hit on file hash match                   │  │
│  │  - Expected: 10x faster re-linting                │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Dependency Graph                                  │  │
│  │  - Track imports/references                       │  │
│  │  - Compute blast radius efficiently               │  │
│  │  - Identify affected files on change              │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│           Knowledge Database (KDB) Integration          │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Rule Performance Aggregation                      │  │
│  │  - Collect metrics from all installations         │  │
│  │  - Industry benchmarks (FP rate, severity)        │  │
│  │  - Consensus scoring across teams                 │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Cross-Project Learning                            │  │
│  │  - Rules perform differently per language/domain  │  │
│  │  - Share best-performing configurations           │  │
│  │  - Auto-optimize rules for new projects           │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│             Collaborative Linting System                 │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Team Rule Profiles                                │  │
│  │  - Team-specific rule configurations              │  │
│  │  - Override global severity/confidence            │  │
│  │  - Inherit from organization defaults             │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Collaborative Voting System                       │  │
│  │  - Vote on rule mutations (approve/reject)        │  │
│  │  - Suggest rule improvements                      │  │
│  │  - Track voting history                           │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Shared Rule Library                               │  │
│  │  - Community rules (best-in-class)                │  │
│  │  - Organization rules (internal standards)        │  │
│  │  - Team rules (project-specific)                  │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

---

## Component 1: Persistent Salsa Index

### Goal
Cache parsed ASTs and dependency graphs across sessions for **10x faster** re-linting.

### Implementation

**New Files:**
- `crates/bonsai-lint/src/engine/persistent_cache.rs` (300 LOC)
- `crates/bonsai-lint/migrations/persistent_index.sql` (50 LOC)

**Key Structures:**

```rust
pub struct PersistentParseCache {
    db_path: PathBuf,
    memory_cache: Arc<DashMap<PathBuf, CachedTree>>,
}

#[derive(Serialize, Deserialize)]
pub struct CachedTree {
    path: PathBuf,
    source_hash: u64,
    tree_bytes: Vec<u8>,           // Serialized Tree
    timestamp: DateTime<Utc>,
    language: String,
}

pub struct DependencyGraph {
    edges: Arc<DashMap<PathBuf, Vec<PathBuf>>>,  // file -> imports
    reverse_edges: Arc<DashMap<PathBuf, Vec<PathBuf>>>,  // import -> files
}
```

**Database Schema:**

```sql
CREATE TABLE parse_cache (
    file_path TEXT PRIMARY KEY,
    source_hash TEXT NOT NULL,
    tree_bytes BLOB NOT NULL,
    timestamp DATETIME NOT NULL,
    language TEXT NOT NULL,
    file_size_bytes INTEGER,
    parse_time_ms INTEGER
);

CREATE TABLE dependencies (
    file_path TEXT NOT NULL,
    imported_from TEXT NOT NULL,
    import_type TEXT,  -- "use", "import", "require", etc.
    PRIMARY KEY (file_path, imported_from)
);

CREATE INDEX idx_cache_timestamp ON parse_cache(timestamp DESC);
CREATE INDEX idx_deps_imported FROM ON dependencies(imported_from);
```

**Performance Characteristics:**
- Cache hit rate: 80–95% on unchanged files
- Memory usage: ~50KB per cached file (AST is compact)
- Disk usage: ~100MB per 1,000 files
- Re-parse time: <1ms (cache hit) vs ~10ms (parse)

### Methods

```rust
impl PersistentParseCache {
    pub async fn get_or_parse(&self, path: &Path) -> Result<Tree>;
    pub async fn invalidate(&self, path: &Path) -> Result<()>;
    pub async fn compute_blast_radius(&self, changed_file: &Path) -> Result<Vec<PathBuf>>;
    pub async fn cleanup_old_entries(&self, days: u32) -> Result<usize>;
}

impl DependencyGraph {
    pub async fn add_edge(&self, from: &Path, to: &Path, import_type: &str) -> Result<()>;
    pub async fn transitive_dependents(&self, changed_file: &Path) -> Result<Vec<PathBuf>>;
}
```

---

## Component 2: Knowledge Database Integration

### Goal
Share rule performance metrics across 1,000+ projects and enable cross-project learning.

### Implementation

**New Files:**
- `crates/bonsai-kdb-sync/Cargo.toml` (new crate)
- `crates/bonsai-kdb-sync/src/lib.rs` (400 LOC)
- `crates/bonsai-kdb-sync/src/rule_aggregator.rs` (300 LOC)
- `crates/bonsai-kdb-sync/src/benchmark.rs` (200 LOC)

**Key Structures:**

```rust
pub struct RuleMetricsAggregator {
    kdb: Arc<KnowledgeDatabase>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AggregatedMetrics {
    rule_id: String,
    // From Phase A's RuleConfidenceMetrics
    confidence_distribution: Vec<f32>,     // Histogram
    avg_confidence: f32,
    median_confidence: f32,
    // Cross-project stats
    project_count: usize,
    language_distribution: HashMap<String, usize>,
    // Effectiveness metrics
    avg_fp_rate: f32,
    avg_tp_rate: f32,
    domains: Vec<String>,  // "web", "systems", "data", etc.
}

pub struct RuleBenchmark {
    rule_id: String,
    language: String,
    domain: String,
    confidence_mean: f32,
    confidence_std: f32,
    fp_rate_mean: f32,
    project_count: usize,
}
```

**KDB Module Format (.kmod):**

```
rule-performance.kmod/
├── metadata.json
│   {
│     "version": "1.0",
│     "generated": "2026-06-01T12:00:00Z",
│     "projects_included": 1247,
│     "languages": ["rust", "python", "typescript"],
│     "domains": ["web", "systems", "data"]
│   }
├── rules/
│   ├── unused-import.json
│   │   {
│   │     "rule_id": "unused-import",
│   │     "confidence_mean": 0.92,
│   │     "confidence_std": 0.08,
│   │     "fp_rate": 0.03,
│   │     "language": "rust",
│   │     "project_count": 500,
│   │     "recommended_severity": "error",
│   │     "cross_project_variants": [
│   │       {
│   │         "domain": "web",
│   │         "confidence": 0.94,
│   │         "fp_rate": 0.02
│   │       }
│   │     ]
│   │   }
└── domains.json
    {
      "web": { "projects": 400, "languages": ["rust", "typescript"] },
      "systems": { "projects": 300, "languages": ["rust", "c"] }
    }
```

**Metrics Aggregation Flow:**

```
Each Deployment
  ├─ Sends metrics to KDB service (nightly)
  ├─ rule_id, confidence, fp_rate, language, domain
  └─ Anonymized (no code/users)
         ↓
KDB Service (Central)
  ├─ Aggregates across all deployments
  ├─ Computes confidence distribution per rule
  ├─ Computes fp_rate by domain
  ├─ Identifies rule variants (works better in domain X)
  └─ Publishes updated .kmod file
         ↓
Each Deployment (Weekly)
  ├─ Downloads updated rule-performance.kmod
  ├─ Extracts recommended severity for each rule
  ├─ Auto-adjusts rule severity globally
  └─ Reports metrics for next cycle
```

---

## Component 3: Collaborative Linting System

### Goal
Enable teams to share rule configurations and vote on improvements.

### Implementation

**New Files:**
- `crates/bonsai-collaboration/Cargo.toml` (new crate)
- `crates/bonsai-collaboration/src/lib.rs` (400 LOC)
- `crates/bonsai-collaboration/src/team_profiles.rs` (250 LOC)
- `crates/bonsai-collaboration/src/voting.rs` (200 LOC)
- `crates/bonsai-collaboration/src/shared_library.rs` (250 LOC)

**Key Structures:**

```rust
pub struct TeamRuleProfile {
    team_id: String,
    organization_id: String,
    rules: HashMap<String, TeamRuleConfig>,
    inherit_from: Option<String>,  // Inherit from org profile
    created_at: DateTime<Utc>,
    last_modified: DateTime<Utc>,
}

pub struct TeamRuleConfig {
    rule_id: String,
    enabled: bool,
    severity: Severity,  // Override global
    confidence_threshold: f32,
    metadata: HashMap<String, String>,
}

pub struct RuleVote {
    vote_id: String,
    proposal_id: String,
    rule_id: String,
    voter_id: String,
    vote: VoteType,  // Approve, Reject, Abstain
    reason: Option<String>,
    timestamp: DateTime<Utc>,
}

pub enum VoteType {
    Approve { reason: String },
    Reject { reason: String },
    Abstain,
}

pub struct SharedRule {
    rule_id: String,
    organization_id: Option<String>,  // Org-specific or community
    rating: f32,  // 0-5 stars
    rating_count: u32,
    downloads: u32,
    created_by: String,
    description: String,
    rule_content: String,
    tags: Vec<String>,
}
```

**Database Schema:**

```sql
CREATE TABLE team_profiles (
    profile_id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    inherit_from TEXT,
    created_at DATETIME NOT NULL,
    last_modified DATETIME NOT NULL
);

CREATE TABLE team_rule_configs (
    config_id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    rule_id TEXT NOT NULL,
    enabled BOOLEAN NOT NULL,
    severity TEXT NOT NULL,
    confidence_threshold REAL NOT NULL,
    metadata JSON,
    FOREIGN KEY (profile_id) REFERENCES team_profiles(profile_id)
);

CREATE TABLE rule_votes (
    vote_id TEXT PRIMARY KEY,
    proposal_id TEXT NOT NULL,
    rule_id TEXT NOT NULL,
    voter_id TEXT NOT NULL,
    vote_type TEXT NOT NULL,
    reason TEXT,
    timestamp DATETIME NOT NULL
);

CREATE TABLE shared_rules (
    rule_id TEXT PRIMARY KEY,
    organization_id TEXT,
    rating REAL NOT NULL,
    rating_count INTEGER NOT NULL,
    downloads INTEGER NOT NULL,
    created_by TEXT NOT NULL,
    description TEXT NOT NULL,
    rule_content TEXT NOT NULL,
    tags JSON NOT NULL
);

CREATE INDEX idx_votes_proposal ON rule_votes(proposal_id);
CREATE INDEX idx_votes_rule ON rule_votes(rule_id);
CREATE INDEX idx_shared_rules_rating ON shared_rules(rating DESC);
CREATE INDEX idx_shared_rules_org ON shared_rules(organization_id);
```

**Voting System:**

```rust
pub struct VotingEngine {
    storage: Arc<CollaborationStorage>,
}

impl VotingEngine {
    /// Cast a vote on a rule mutation proposal
    pub async fn vote(
        &self,
        proposal_id: &str,
        voter_id: &str,
        vote: VoteType,
    ) -> Result<()>;

    /// Get vote summary for a proposal
    pub async fn get_summary(&self, proposal_id: &str) -> Result<VoteSummary>;

    /// Auto-approve/reject based on vote threshold (e.g., >66%)
    pub async fn evaluate_proposal(&self, proposal_id: &str) -> Result<ProposalDecision>;
}

pub struct VoteSummary {
    approvals: usize,
    rejections: usize,
    abstentions: usize,
    approval_rate: f32,  // approvals / (approvals + rejections)
}

pub enum ProposalDecision {
    Approved { consensus: f32 },
    Rejected { consensus: f32 },
    Pending,
}
```

---

## Component 4: Phase B Integration Flow

### Scenario: Team Adopts Improved Rule Configuration

```
1. Organization publishes rule-performance.kmod
   └─ Says "clippy-pedantic: FP rate 15%, avg confidence 0.58"

2. Team A downloads KDB metrics
   └─ Sees recommendation: demote clippy-pedantic to HINT

3. Team B proposes mutation: "Refine clippy-pedantic pattern"
   └─ Creates RuleMutationProposal

4. All teams vote on proposal
   └─ Team A: ✓ Approve (helps them)
   └─ Team C: ✗ Reject (breaks their use case)
   └─ Team D: ◆ Abstain
   └─ Result: 66% approval → APPROVED

5. Refined rule published to KDB
   └─ Updates rule-performance.kmod
   └─ Next download: all teams get improved rule

6. Phase A ETL applies rule to each team's context
   └─ Team A: Severity demoted to HINT (per KDB)
   └─ Team B: Severity kept as WARNING (per team profile)
   └─ Team C: Rule disabled (per team profile)

7. Next ETL cycle improves metrics
   └─ FP rate drops from 15% to 8%
   └─ Confidence increases from 0.58 to 0.75
   └─ Updates KDB for next teams
```

---

## Data Structure: Phase A → Phase B Continuity

```
Phase A Output (ETL Cycle)
├─ RuleConfidenceMetrics { confidence, fp_rate, tp_rate }
├─ Feedback Events { user_id, rule_id, file, feedback_type }
└─ RuleMutationProposal { rule_id, pattern, expected_improvement }
        ↓
Phase B Processing
├─ Aggregate metrics across team (all developers)
├─ Aggregate metrics across organization
├─ Aggregate metrics across industry (KDB)
├─ Apply team profile overrides
├─ Apply vote-based adjustments
└─ Publish to shared library
        ↓
Phase A Re-Input (Next Cycle)
├─ Updated metrics from KDB
├─ Team-specific rule configurations
├─ Community votes and ratings
└─ Cross-project lessons learned
```

---

## Deployment Architecture

### Deployment Option 1: Single Organization

```
Deployment Instances (5 teams, 50 devs)
├─ All share local KDB cache
├─ All publish to central org KDB
├─ Team profiles managed by org
└─ Voting within organization only
```

### Deployment Option 2: Open Source Ecosystem

```
Public KDB Server (Central)
├─ Aggregates metrics from 1,000+ projects
├─ Publishes anonymized rule-performance.kmod
├─ Tracks cross-project rule variants

Each Project (Rust, Python, etc.)
├─ Downloads public rule-performance.kmod weekly
├─ Runs local Phase A ETL
├─ Publishes anonymized metrics
├─ Can override with project-specific profiles
```

### Deployment Option 3: Hybrid Enterprise + Open Source

```
Internal KDB Server
├─ Private metrics (closed-source projects)
├─ Team voting system
├─ Custom rule library

Public KDB Server
├─ Open-source metrics
├─ Community voting
├─ Published rules

Sync Mechanism
├─ Enterprise learns from public (weekly)
├─ Public learns from non-sensitive enterprise patterns
└─ Privacy boundaries respected (PII, code never shared)
```

---

## Success Metrics for Phase B

### Performance
- **Cache hit rate:** 80–95% on unchanged codebases
- **Re-lint speedup:** 10x faster on cached parse trees
- **Query latency:** <100ms for team profile lookups

### Knowledge Sharing
- **Rules with KDB data:** 80%+ of rules
- **Cross-project variance:** Identify 10+ rule variants per domain
- **Auto-optimization:** 5–10% improvement in confidence post-KDB update

### Collaboration
- **Voting participation:** 70%+ of proposals get votes
- **Proposal approval rate:** 60–80%
- **Shared rule adoption:** 50%+ of teams using 20%+ community rules

---

## Implementation Roadmap

**Week 1–2: Persistent Salsa + Dependency Tracking**
- [ ] Implement PersistentParseCache
- [ ] Serialize/deserialize Tree objects
- [ ] Track file dependencies
- [ ] Compute blast radius efficiently
- [ ] Benchmark cache hit rates

**Week 2–3: KDB Integration**
- [ ] Create KDB sync crate
- [ ] Implement RuleMetricsAggregator
- [ ] Build .kmod publishing pipeline
- [ ] Create dashboard for KDB metrics
- [ ] Test with Phase A metrics

**Week 3–4: Collaborative Linting**
- [ ] Implement TeamRuleProfile system
- [ ] Build voting engine
- [ ] Create shared rule library
- [ ] Add organization management
- [ ] Build team dashboard

**Week 4+: Integration & Testing**
- [ ] Wire persistent cache to Phase A ETL
- [ ] Integrate KDB downloads into Phase A
- [ ] Add team profile application to linting
- [ ] Comprehensive testing (10+ test suites)
- [ ] Documentation and deployment guides

---

## Dependencies

### New Crates
- `bonsai-persistence` – Persistent cache with SQLx
- `bonsai-kdb-sync` – KDB aggregation and syncing
- `bonsai-collaboration` – Team profiles and voting

### External Dependencies
- `bincode` – Serialize/deserialize Tree objects
- `blake3` – File hashing for cache validation
- `tokio` – Async operations
- `sqlx` – Database operations

---

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Tree serialization issues | Test with 10,000+ real files |
| Cache invalidation bugs | Version cache format, auto-rebuild on mismatch |
| Privacy (KDB sharing) | Never share code/PII, only metrics |
| Voting manipulation | Rate-limit votes, add reputation system |
| KDB sync bottleneck | Cache .kmod locally, weekly updates |

---

## References

- **Phase A:** `docs/26-PHASE-A-IMPLEMENTATION-SUMMARY.md`
- **Linter Architecture:** `docs/22-UNIVERSAL-LINTER.md`
- **ETL Pipeline:** `docs/25-SELF-IMPROVING-RULES-BLUEPRINT.md`

---

**Phase B Blueprint Complete** ✓

Proceed with implementation?
