# Git Branch Management

This document describes the branch strategy and current branch status for the Bonsai Ecosystem repository.

## Branch Cleanup Summary (2026-06-06)

### Action Taken
- **Deleted 24 merged local branches** - Branches that had been integrated into `main`
- **Deleted 4 merged remote branches** - Remote tracking branches (`origin/*`) that were obsolete
- **Preserved 10 active unmerged branches** - Branches with ongoing work or planned features

### Before Cleanup
- Local branches: 43
- Remote branches: 36
- Merged branches: 32

### After Cleanup
- Local branches: 11 (1 main + 10 unmerged)
- Remote branches: 32 (tracking origin)
- Fully merged and cleaned: 28 branches removed

---

## Current Active Branches

### Primary Development
- **main** - Main development branch (production-ready code)

### Feature Development (Unmerged)

| Branch | Age | Status | Notes |
|--------|-----|--------|-------|
| `feat/bonsai-core-phase1` | 13 days | Active | 11 commits ahead, ongoing work |
| `feat/p3-future-improvements` | 5 weeks | Review | 1 commit, planned improvements |
| `fix/security-hardening` | 5 weeks | Pending | 1 commit, security enhancements |
| `perf/build-optimizations` | 5 weeks | Pending | 2 commits, build performance |
| `pr/tasks-1-7-complete` | 6 weeks | Review | 27 commits, feature implementation |

### Bug Fixes & Cleanup
| Branch | Age | Status | Notes |
|--------|-----|--------|-------|
| `fix/warnings-wasm-example` | 6 weeks | Stale | 27 commits, variant of wasm fixes |
| `fix/warnings-wasm-example-clean` | 7 weeks | Stale | 18 commits, cleanup attempt |
| `fix/warnings-wasm-example-clean2` | 7 weeks | Stale | 13 commits, cleanup variant |
| `fix/warnings-wasm-example-clean3` | 7 weeks | Stale | 13 commits, cleanup variant |
| `fix/warnings-wasm-example-patch` | 7 weeks | Stale | 15 commits, patch variant |

---

## Branch Classification

### Active Branches (Keep & Monitor)
These branches have recent commits or represent important work:
- `feat/bonsai-core-phase1` - Core phase work with recent activity

### Planning Branches (Evaluate & Decide)
These branches represent planned but not-yet-started work:
- `feat/p3-future-improvements` - Phase 3 improvements
- `perf/build-optimizations` - Build system optimization
- `pr/tasks-1-7-complete` - Task implementation

### Maintenance Branches (Review Before Use)
These branches need review for continuing or closing:
- `fix/security-hardening` - Security improvements
- `fix/warnings-wasm-example*` - WASM build warning fixes (5 variants)

---

## Branch Strategy

### Main Branch
- Production-ready code only
- Fully tested and integrated
- All hotfixes merged to main first, then to feature branches

### Feature Branches
- Format: `feat/<feature-name>`
- Branched from: `main`
- Merged back to: `main` (via PR review)
- Lifetime: Until feature is complete and merged

### Fix Branches
- Format: `fix/<issue-description>`
- Branched from: `main`
- Merged back to: `main` (via PR review)
- Lifetime: Until fix is tested and merged

### Chore/Documentation Branches
- Format: `chore/<task>` or `docs/<topic>`
- Branched from: `main`
- Lifetime: Short-lived, merged quickly

---

## Recommendations for Next Steps

### Immediate Actions
1. ✅ **Completed**: Deleted merged branches (24 local, 4 remote)
2. ✅ **Completed**: Cleaned up remote tracking branches
3. **TODO**: Review and decide on 5 wasm-example variant branches
   - Option A: Delete if no longer needed
   - Option B: Merge the best variant and delete others
   - Option C: Keep one as reference

4. **TODO**: Review stale branches (5+ weeks old)
   - `feat/p3-future-improvements` - Still planned?
   - `fix/security-hardening` - Still needed?
   - `perf/build-optimizations` - Still needed?
   - `pr/tasks-1-7-complete` - Should this be merged?

### Long-Term Strategy
1. **Regular Cleanup**: Delete merged branches immediately after PR merge
2. **Stale Branch Monitoring**: Review branches older than 1 month
3. **Documentation**: Keep this file updated with branch status
4. **CI/CD Integration**: Use branch naming conventions for automatic PR workflows

---

## Deleted Branches (Archived)

The following branches were successfully deleted (merged into main):
- audit/clojure-python-integration
- backup/* (pre-debranding branches)
- chore/workspace-cargo
- docs/new-features-update
- feat/agent-host-and-command-gating
- feat/bonsai-buddy-android (and sprint variants)
- feat/bonsai-everywhere-extension
- feat/bonsaibot-* (all wave variants)
- feat/ecosystem-integration-tests
- feat/exe-builder-and-buddy-fix
- feat/feature-flags-and-typed-ipc
- feat/inference-mode-system
- feat/rag-model-memory-upgrades
- feat/ux-and-task-queue
- fix/build-local-permission
- fix/clojure-python-p1-backlog
- fix/gpu-crash-auto-fallback
- fix/harmonize-libsqlite3-sys (remote)
- fix/launcher-and-slot-ready
- fix/model-loading-ux
- fix/model-warmup-crash
- fix/p0-critical-security
- fix/p1-high-priority
- fix/p2-quality-improvements
- fix/queue-bar-and-builder
- fix/swarm-core-improvements (remote)
- pr/tasks-1-7-clean (remote)

All commits from these branches are preserved in main through merge commits or cherry-picks.

---

## References

- [Contributing Guidelines](CONTRIBUTING.md)
- [Repository Organization](../ORGANIZATION.md)
- Git branch naming: Follow `type/description` format (feat/, fix/, chore/, docs/, etc.)

---

**Last Updated**: 2026-06-06  
**Status**: Branch cleanup completed successfully
