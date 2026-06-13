# 🎯 Team Leads Quick Reference Guide

**Purpose:** 5-minute summary for each team lead  
**Last Updated:** 2026-06-02  
**Status:** ✅ Ready for Week 1 Kickoff

---

## 🚀 Your Team is Ready to Go

Your crate is fully scaffolded. Your build script exists. Your CI/CD is active.

**No setup required.** You can start development today.

---

## 📍 Team Directory

### Team A: Fuzzing Engine
- **Crate:** `crates/bonsai-bedf-fuzzing`
- **Build Script:** `scripts/build/build-team-a.ps1`
- **Deliverable:** Coverage-guided fuzzing (libFuzzer/AFL++)
- **Timeline:** 8 weeks
- **Size:** 2 FTE
- **Spec:** `BEDF_ARCHITECTURE.md` → Fuzzing Engine section
- **Dependencies:** libFuzzer, AFL++, tokio, dashmap

### Team B: Concurrency Testing
- **Crate:** `crates/bonsai-bedf-concurrency`
- **Build Script:** `scripts/build/build-team-b.ps1`
- **Deliverable:** Loom + Shuttle testing framework
- **Timeline:** 6 weeks
- **Size:** 2 FTE
- **Spec:** `BEDF_ARCHITECTURE.md` → Concurrency Testing section
- **Dependencies:** loom, shuttle, tokio

### Team C: Memory Sanitizers
- **Crate:** `crates/bonsai-bedf-sanitizers`
- **Build Script:** `scripts/build/build-team-c.ps1`
- **Deliverable:** ASAN/MSAN/TSAN/LSAN integration
- **Timeline:** 4 weeks
- **Size:** 2 FTE
- **Spec:** `BEDF_ARCHITECTURE.md` → Memory Sanitizers section
- **Dependencies:** Rust nightly, cargo-sanitizer

### Team D: Property Testing
- **Crate:** `crates/bonsai-bedf-property`
- **Build Script:** `scripts/build/build-team-d.ps1`
- **Deliverable:** Proptest harness + generative testing
- **Timeline:** 4 weeks
- **Size:** 1 FTE
- **Spec:** `BEDF_ARCHITECTURE.md` → Property Testing section
- **Dependencies:** proptest

### Team E: Penetration Testing
- **Crate:** `crates/bonsai-bedf-pentest`
- **Build Script:** `scripts/build/build-team-e.ps1`
- **Deliverable:** OWASP ZAP + protocol fuzzing + RESTful API testing
- **Timeline:** 8 weeks
- **Size:** 2 FTE
- **Spec:** `BEDF_ARCHITECTURE.md` → Penetration Testing section
- **Dependencies:** OWASP ZAP, RESTler, tokio

### Team F: Sandbox Orchestration
- **Crate:** `crates/bonsai-bedf-sandbox`
- **Build Script:** `scripts/build/build-team-f.ps1`
- **Deliverable:** Sanctum vault orchestration + seccomp
- **Timeline:** 8 weeks
- **Size:** 2 FTE
- **Spec:** `BEDF_ARCHITECTURE.md` → Sandbox Orchestration section
- **Dependencies:** Sanctum APIs, seccomp

### Team G: Triage & AI
- **Crate:** `crates/bonsai-bedf-triage`
- **Build Script:** `scripts/build/build-team-g.ps1`
- **Deliverable:** Crash deduplication, AI explanation, auto-fixes
- **Timeline:** 8 weeks
- **Size:** 2 FTE
- **Spec:** `BEDF_ARCHITECTURE.md` → Triage Engine section
- **Dependencies:** BonsAI V2, blake3, SQLite

### Team H: MCP Tools
- **Crate:** `crates/bonsai-bedf-mcp`
- **Build Script:** `scripts/build/build-team-h.ps1`
- **Deliverable:** 8 MCP tools for AI integration
- **Timeline:** 4 weeks
- **Size:** 1 FTE
- **Spec:** `BEDF_ARCHITECTURE.md` → MCP Tools section
- **Dependencies:** MCP SDK, tokio, serde_json

### Team I: Advanced Enhancements
- **Crate:** `crates/bonsai-bedf-enhancements`
- **Build Script:** `scripts/build/build-team-i.ps1`
- **Deliverable:** 10 strategic enhancements
- **Timeline:** 12 weeks
- **Size:** 2 FTE
- **Spec:** `BEDF_ADVANCED_ENHANCEMENTS.md` (all 10 enhancements)
- **Dependencies:** All core teams (after week 10)

### Team J: Survival System Integration
- **Crate:** `crates/bonsai-survival-system-ext`
- **Build Script:** `scripts/build/build-team-j.ps1`
- **Deliverable:** Bug memory, confidence scoring, permanent learning
- **Timeline:** 6 weeks
- **Size:** 1 FTE
- **Spec:** `BUG_CATALOGUE_COMPLETE.md` + integration docs
- **Dependencies:** Core BEDF + Survival System APIs

### Team K: Knowledge Database Integration
- **Crate:** `crates/bonsai-kdb-ext`
- **Build Script:** `scripts/build/build-team-k.ps1`
- **Deliverable:** Cross-project rules, embeddings, pattern matching
- **Timeline:** 6 weeks
- **Size:** 1 FTE
- **Spec:** `BUG_HUNTER_ERROR_DATABASE.md` + integration docs
- **Dependencies:** Core BEDF + KDB APIs

---

## 🛠️ Quick Start (5 Minutes)

### Step 1: Understand Your Role (1 min)
```bash
# Read this once
cat GETTING_STARTED.md          # Week 1 overview
# Then read your spec:
grep -A 50 "Team A:" BEDF_ARCHITECTURE.md
```

### Step 2: Explore Your Crate (1 min)
```bash
cd crates/YOUR_CRATE_NAME
ls -la
```

You'll see:
- ✅ `Cargo.toml` – workspace-inherited config
- ✅ `src/lib.rs` – ready for implementation
- ✅ `src/interfaces.rs` – your trait definitions
- ✅ `src/config.rs` – configuration struct
- ✅ `tests/integration_test.rs` – test skeleton

### Step 3: Run Your Build (1 min)
```powershell
.\scripts\build\build-team-X.ps1
```

Should output:
```
✅ Build successful
✅ All tests passed
✅ No clippy warnings
```

### Step 4: Create First Branch (1 min)
```bash
git checkout -b team/X/setup
# Make your first edit
# Commit and push!
```

### Step 5: Open First PR (1 min)
```
GitHub → Pull Requests → New PR
Title: "[Team X] Initial scaffolding"
Description: Link to your spec section
```

---

## 📊 Success Checklist

### By End of Week 1
- [ ] Read `GETTING_STARTED.md`
- [ ] Reviewed my team's spec section
- [ ] Ran `.\scripts\build\build-team-X.ps1` successfully
- [ ] Created team Slack channel
- [ ] Implemented 1 interface method
- [ ] Opened 1 PR with tests
- [ ] Daily standup updates started

### By End of Week 2
- [ ] 3-5 interface methods implemented
- [ ] 15+ unit tests written
- [ ] >80% code coverage
- [ ] All clippy warnings resolved
- [ ] Team velocity established

### By End of Week 8
- [ ] All core features implemented
- [ ] >80% code coverage maintained
- [ ] Ready for first integration (week 9)

---

## 🔗 No Blocking Dependencies (Weeks 1-8)

**Key:** Your team works independently.

**You do NOT block on:**
- Other teams' code
- Interface changes from other teams
- Integration tests

**You DO need to:**
- Follow frozen interfaces (from week 2)
- Write mock/stub implementations if needed
- Be ready for integration week 9

**Example:** Team G (Triage) doesn't need Team A's (Fuzzing) code yet. You can mock the fuzzing interface and implement triage logic independently.

---

## 📚 Documentation Map

| Question | Document | Section |
|----------|----------|---------|
| "What's my deliverable?" | `PARALLEL_BUILD_MANIFEST.md` | Team assignments |
| "How do I build?" | `GETTING_STARTED.md` | Build scripts |
| "What's the architecture?" | `BEDF_ARCHITECTURE.md` | Your team section |
| "How long do I have?" | `MASTER_DELIVERY_INDEX.md` | Timeline |
| "What are the advanced features?" | `BEDF_ADVANCED_ENHANCEMENTS.md` | Enhancements 1-10 |
| "What bugs should I know?" | `BUG_CATALOGUE_COMPLETE.md` | 55 documented bugs |
| "How do I integrate?" | `PARALLEL_BUILD_MANIFEST.md` | Integration section |

---

## 🚨 Build Troubleshooting (30 seconds)

### Build fails immediately:
```bash
cd Z:\Projects\BonsaiWorkspace
cargo update
cargo build --workspace
# If still fails, check Slack #devops
```

### Tests fail:
```bash
cargo test --package YOUR_CRATE -- --nocapture
# Check for async deadlocks, timeouts
# If needed: add #[tokio::time::timeout(...)]
```

### Clippy warns:
```bash
cargo clippy --package YOUR_CRATE --fix
# Auto-fixes most issues
# Commit the result
```

### CI/CD failed:
```
GitHub Actions → Workflows → bedf-teams-parallel
Click your team's job → Check "Run cargo build..." step
```

---

## 💬 Daily Communication

### Slack Channels
- `#team-X-YOUR-AREA` – Daily standup
- `#bedf-architecture` – Design questions
- `#devops` – Build issues
- `#blockers` – Escalations

### GitHub
- Issues: `[Team X] Title`
- PRs: Link to your spec section
- Milestones: Your deliverables

### Weekly Sync
- **Monday 9 AM:** All-hands standup
- **Tuesday 10 AM:** Team leads sync
- **Friday 4 PM:** Backlog grooming

---

## 🎯 Metrics You'll Measure

**Weekly:**
- Lines of code written
- Tests written (per method)
- Code coverage (aim: >80%)
- Build time (should be <2 min)
- PR review time (target: <24 hours)

**By Week 8:**
- All interface methods implemented
- Integration ready (mocks sufficient)
- Zero clippy warnings
- 100% tests passing

---

## 🏆 You're Succeeding If...

✅ **Daily:** Your builds pass, your tests pass, no new warnings  
✅ **Weekly:** 10-15 new methods implemented, >80% coverage  
✅ **Monthly:** On schedule, no blockers, team is happy  
✅ **Week 8:** Ready for integration with other teams  
✅ **Week 24:** Zero bugs, production ready  

---

## 📞 When You Need Help

| Issue | First Try | If That Fails |
|-------|-----------|--------------|
| "How do I build?" | `GETTING_STARTED.md` | Slack #devops |
| "What's my spec?" | `BEDF_ARCHITECTURE.md` | Slack #bedf-architecture |
| "Build won't compile" | `cargo update` → `cargo build` | Slack #devops |
| "Tests timeout" | Add `#[tokio::time::timeout(...)]` | Ask Team G |
| "CI/CD failed" | Check GitHub Actions logs | Slack #devops |
| "Need early integration" | Work with Team X directly | Escalate to #blockers |

---

## 🚀 You Are Ready

Everything is scaffolded. Your build script is ready. Your CI/CD is active.

**Start coding today.** Your team can be productive by EOD.

---

## 🎬 Final Checklist (Before Week 1)

### Pre-Kickoff
- [ ] Team lead: Have my GitHub and Slack accounts
- [ ] Team lead: Read `GETTING_STARTED.md`
- [ ] Team lead: Read my team's spec section
- [ ] Team lead: Ran my build script once successfully
- [ ] All team members: Know their role
- [ ] All team members: Have access to codebase
- [ ] All team members: Slack channel created

### Day 1
- [ ] Team standup at 9 AM
- [ ] Create `team/X/...` branches
- [ ] Start implementing first method
- [ ] First commit by EOD
- [ ] Celebrate! 🎉

---

## ✅ Status: READY TO GO

🎉 **Your crate is initialized. Your tools are ready. Your team is set.**

**Let's build the safest software platform on Earth.** 🛡️

---

**Questions?** Check the relevant document above.  
**Need help?** Ask in your team's Slack channel.  
**Ready to start?** `git checkout -b team/X/first-feature`

---

**Timeline:** 24 weeks, 18 FTE  
**Deadline:** 2026-10-24  
**Success Metric:** Zero bugs, 99%+ detection rate  
**Vision:** Most resilient software platform on Earth  

Let's make it happen! 🚀

