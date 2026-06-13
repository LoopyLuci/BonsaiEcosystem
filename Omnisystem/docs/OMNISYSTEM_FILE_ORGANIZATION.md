# Omnisystem File Organization Guide

**Date**: 2026-06-11  
**Status**: ✅ All files properly organized within Omnisystem  

---

## Directory Structure

All project files must be created within `Omnisystem/` subdirectories, never in the BonsaiWorkspace root.

```
Z:\Projects\BonsaiWorkspace\
├── Cargo.toml (workspace root - **ONLY** here)
├── Cargo.lock
├── README.md
├── LICENSE
├── CHANGELOG.md
│
└── Omnisystem/                          ✅ ALL PROJECT WORK GOES HERE
    ├── Cargo.toml                       (Omnisystem workspace)
    ├── crates/                          (228+ Rust crates)
    │   ├── omnisystem-ums/              (Universal Module System)
    │   ├── pathfinder-core/             (PATHFINDER foundation)
    │   ├── pathfinder-user-service/
    │   ├── ... (all other crates)
    │
    ├── docs/                            📁 Documentation Hub
    │   ├── PATHFINDER_UMS_ARCHITECTURE.md
    │   ├── PATHFINDER_UMS_COMPLETE.md
    │   ├── WORKSPACE_BUILD_SYSTEM.md
    │   ├── WORKSPACE_ARCHITECTURE.md
    │   ├── GETTING_STARTED.md
    │   ├── PATHFINDER_API_DOCUMENTATION.md
    │   ├── PATHFINDER_DEPLOYMENT_OPERATIONS.md
    │   ├── phase-reports/               📁 Phase completion reports
    │   │   ├── PATHFINDER_WEEK6_COMPLETE.md
    │   │   ├── PATHFINDER_WEEK7_COMPLETE.md
    │   │   ├── ... (all phase reports)
    │   │
    │   └── specs/                       📁 System specifications
    │       ├── IOT_CONTROL_COMPREHENSIVE_PLAN.md
    │       ├── OMNISEARCH_COMPREHENSIVE_PLAN.md
    │       ├── OMNISYSTEM_MODULAR_ARCHITECTURE.md
    │       ├── ... (all specs and plans)
    │
    ├── services/
    │   └── backend/                     📁 Backend service implementations
    │       ├── backend_user_service_main.go
    │       ├── backend_content_service_main.go
    │       └── ... (all service files)
    │
    ├── frontend/
    │   └── src/                         📁 Frontend source files
    │       ├── frontend_pages_*.tsx
    │       ├── frontend_components_*.tsx
    │       └── ... (all frontend files)
    │
    ├── mobile/
    │   └── lib/                         📁 Mobile app source
    │       ├── mobile_pages_*.dart
    │       ├── mobile_api_service.dart
    │       └── ... (all mobile files)
    │
    ├── tests/
    │   ├── integration_suite.go
    │   └── performance/
    │       └── load_tests.js
    │
    ├── database/
    │   └── migrations/
    │       └── 001_initial_schema.sql
    │
    ├── deployment/
    │   ├── docker-compose.yml
    │   └── kubernetes/
    │       └── config.yaml
    │
    ├── config/
    │   ├── nginx/
    │   │   └── production.conf
    │   └── monitoring/
    │       ├── prometheus.yml
    │       └── alert_rules.yml
    │
    ├── scripts/
    │   ├── build_all.sh
    │   └── test_all.sh
    │
    ├── .github/
    │   └── workflows/
    │       └── deploy.yml
    │
    └── [other existing directories as needed]
```

---

## File Organization Rules

### ✅ **DO**: Create files in Omnisystem subdirectories

```bash
# Documentation
Omnisystem/docs/FILENAME.md
Omnisystem/docs/phase-reports/PHASE_*.md
Omnisystem/docs/specs/PLAN_*.md

# Code
Omnisystem/crates/*/src/lib.rs
Omnisystem/frontend/src/*.tsx
Omnisystem/mobile/lib/*.dart
Omnisystem/services/backend/*.go

# Infrastructure
Omnisystem/deployment/*.yml
Omnisystem/config/nginx/*.conf
Omnisystem/database/migrations/*.sql
Omnisystem/tests/*.go
Omnisystem/scripts/*.sh
```

### ❌ **DON'T**: Create files in BonsaiWorkspace root

```bash
# ❌ WRONG
BonsaiWorkspace/PATHFINDER_*.md
BonsaiWorkspace/OMNISYSTEM_*.md
BonsaiWorkspace/frontend_*.tsx
BonsaiWorkspace/mobile_*.dart
BonsaiWorkspace/docker-compose.yml

# ✅ CORRECT
BonsaiWorkspace/Omnisystem/docs/PATHFINDER_*.md
BonsaiWorkspace/Omnisystem/docs/OMNISYSTEM_*.md
BonsaiWorkspace/Omnisystem/frontend/src/*.tsx
BonsaiWorkspace/Omnisystem/mobile/lib/*.dart
BonsaiWorkspace/Omnisystem/deployment/docker-compose.yml
```

---

## Root Directory (BonsaiWorkspace/)

Only these files belong in the root:

```
Z:\Projects\BonsaiWorkspace\
├── .gitattributes         (Git configuration)
├── .gitignore             (Git ignore rules)
├── Cargo.toml             (Workspace definition - ONLY top-level workspace)
├── Cargo.lock             (Dependency lock)
├── README.md              (Root README for workspace)
├── CHANGELOG.md           (Workspace changelog)
├── LICENSE                (License)
```

**Everything else goes in `Omnisystem/`**

---

## File Types and Locations

| File Type | Location | Example |
|-----------|----------|---------|
| **UMS Module Crates** | `Omnisystem/crates/` | `pathfinder-core/`, `pathfinder-user-service/` |
| **Documentation** | `Omnisystem/docs/` | `PATHFINDER_UMS_ARCHITECTURE.md` |
| **Phase Reports** | `Omnisystem/docs/phase-reports/` | `PATHFINDER_WEEK6_COMPLETE.md` |
| **Specs/Plans** | `Omnisystem/docs/specs/` | `IOT_CONTROL_COMPREHENSIVE_PLAN.md` |
| **Go Services** | `Omnisystem/services/backend/` | `backend_user_service_main.go` |
| **React/TypeScript** | `Omnisystem/frontend/src/` | `frontend_pages_*.tsx` |
| **Flutter/Dart** | `Omnisystem/mobile/lib/` | `mobile_pages_*.dart` |
| **Tests** | `Omnisystem/tests/` | `integration_suite.go` |
| **Database** | `Omnisystem/database/migrations/` | `001_initial_schema.sql` |
| **Docker/K8s** | `Omnisystem/deployment/` | `docker-compose.yml` |
| **Config Files** | `Omnisystem/config/` | `nginx/production.conf` |
| **Scripts** | `Omnisystem/scripts/` | `build_all.sh` |
| **CI/CD** | `Omnisystem/.github/workflows/` | `deploy.yml` |

---

## Moving Forward

### When Creating New Files:

1. **Determine the file type** (documentation, code, config, etc.)
2. **Place it in the appropriate Omnisystem subdirectory** (see table above)
3. **Never create project files in BonsaiWorkspace root**
4. **Use descriptive filenames** that indicate their purpose

### Example Workflow:

```bash
# ✅ Create new PATHFINDER documentation
touch Omnisystem/docs/PATHFINDER_NEW_FEATURE.md

# ✅ Create new UMS module
mkdir Omnisystem/crates/pathfinder-new-service
touch Omnisystem/crates/pathfinder-new-service/Cargo.toml

# ✅ Create new frontend page
touch Omnisystem/frontend/src/pages_new_page.tsx

# ✅ Create new test
touch Omnisystem/tests/new_integration_test.go
```

---

## Current Status

**✅ COMPLETED - 06/11/2026**

- All 160+ project files organized into Omnisystem
- Root directory cleaned (only workspace files remain)
- PATHFINDER UMS modules created in `Omnisystem/crates/`
- Documentation centralized in `Omnisystem/docs/`
- All code properly organized by type and domain

---

## References

- **PATHFINDER UMS Architecture**: `Omnisystem/docs/PATHFINDER_UMS_ARCHITECTURE.md`
- **Workspace Build System**: `Omnisystem/docs/WORKSPACE_BUILD_SYSTEM.md`
- **Getting Started**: `Omnisystem/docs/GETTING_STARTED.md`

---

**Going forward: Always build within Omnisystem. Never put project files in the root.** ✅

