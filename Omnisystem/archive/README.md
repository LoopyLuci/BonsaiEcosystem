# Archive Directory

This directory contains archived, obsolete, and historical documentation and artifacts that are no longer actively maintained but are kept for reference purposes.

## Contents

### `/docs/` - Archived Documentation
Contains documentation from previous phases and obsolete specifications. These files are maintained in the repository for historical reference but should **not** be relied upon for current development.

**Categories of archived docs**:
- Phase 1-4 completion summaries (superseded by current documentation)
- Obsolete architecture designs (replaced by Omnisystem specifications)
- Legacy system documentation (e.g., old BPCF, BMF versions)
- Historical delivery reports and summaries
- Deprecated feature specifications

**Current reference documentation** is in the root `/docs/` folder. For active development, always check there first.

### `/logs/` - Historical Build and Execution Logs
Contains build logs, execution traces, and system logs from previous runs. These are useful for:
- Debugging historical issues
- Understanding past performance
- Tracing system evolution
- Reference during failure analysis

**Note**: Current logs should be in `/logs/` at the root level if available.

### `/results/` - Test and Benchmark Results
Contains test result artifacts, benchmarking data, and execution reports:
- `polyglot-pong-results-*.json` - Language compatibility test results
- Historical performance benchmarks
- System validation reports

These are kept for:
- Historical performance tracking
- Regression analysis
- Reference comparisons
- Documentation of past achievements

## Guidelines

### When to Use Archive

✅ **Use archived docs when**:
- You need historical context about a design decision
- Debugging a legacy issue that was documented
- Understanding how a deprecated component worked
- Reviewing past performance metrics

❌ **Don't use archived docs when**:
- Building new features (check `/docs/` instead)
- Setting up development (use root `GETTING_STARTED.md`)
- Understanding current architecture (use active docs)
- Making design decisions (use active specifications)

### Organization

Archive files are organized by type:
- `docs/` - All documentation by content type
- `logs/` - All historical logs
- `results/` - All test and benchmark results

This keeps the archive browsable and useful for reference.

### Maintenance

Archive files are:
- ✅ Kept for historical reference
- ✅ Available in Git history
- ✅ Indexed and discoverable
- ❌ Not actively maintained
- ❌ Not updated for current changes
- ❌ Not used for current development

---

**Last Updated**: 2026-06-06  
**Purpose**: Historical reference and version control
