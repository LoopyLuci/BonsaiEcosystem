# Scripts Quick Reference

**Repository**: Omnisystem  
**Version**: 1.0.0  

Quick reference for common build and test commands.

---

## 🚀 One-Liners

### Build Everything
```bash
# Windows
.\scripts\build\build_omni.ps1

# Linux/macOS
./scripts/build/build_all.sh
```

### Run All Tests
```bash
# Windows
.\scripts\test-all.ps1

# Linux/macOS
./scripts/test_all.sh
```

### Verify Build
```bash
.\scripts\verification\master_verify.ps1
```

### Create Launchers
```bash
.\scripts\build\create_launchers.ps1
```

### Package Release
```bash
.\scripts\build\package_release.ps1
```

---

## 📋 Common Commands

| Task | Command | Platform |
|------|---------|----------|
| Full Build | `.\scripts\build\build_omni.ps1` | Windows |
| Full Build | `./scripts/build/build_all.sh` | Unix |
| Test All | `.\scripts\test-all.ps1` | Windows |
| Test All | `./scripts/test_all.sh` | Unix |
| Verify | `.\scripts\verification\master_verify.ps1` | Windows |
| Launchers | `.\scripts\build\create_launchers.ps1` | Windows |
| Release | `.\scripts\build\package_release.ps1` | Windows |
| Stress Test | `./scripts/run_stress_tests.sh` | Unix |
| Validation | `.\scripts\run_full_validation.ps1` | Windows |
| Bootstrap Check | `.\scripts\ci\check_bootstrap_invariants.ps1` | Windows |
| Stub Check | `.\scripts\ci\verify_no_stubs.ps1` | Windows |

---

## 🔄 Build Phases

```bash
# Phase 1 (Foundation)
./scripts/build/BUILD_PHASE1.sh

# Phase 2 (Core)
./scripts/build/BUILD_PHASE2.sh

# Phase 3 (Advanced)
.\scripts\build\BUILD_PHASE3.ps1

# Complete Phase 3
.\scripts\build\BUILD_PHASE3_COMPLETE.ps1
```

---

## 🔧 Development Workflow

```bash
# 1. Build once
.\scripts\build\build_omni.ps1

# 2. Edit code (use cargo for incremental builds)
cargo build --lib

# 3. Test changes
cargo test --lib

# 4. Verify before commit
.\scripts\verification\master_verify.ps1
```

---

## 📦 Release Workflow

```bash
# 1. Clean build
cargo clean
.\scripts\build\build_omni.ps1

# 2. Run full tests
.\scripts\test-all.ps1

# 3. Verify
.\scripts\verification\master_verify.ps1

# 4. Package
.\scripts\build\package_release.ps1
```

---

## 🐛 Quick Troubleshooting

```bash
# Clean and rebuild
cargo clean
.\scripts\build\build_omni.ps1

# Check permissions (Unix)
chmod +x scripts/build/*.sh

# Allow execution (Windows)
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# See what will build (dry run)
cargo build --workspace --dry-run
```

---

## 📁 Script Locations

```
Omnisystem/scripts/
├── build/                      # Build scripts
│   ├── build_omni.ps1         # Main build
│   ├── build_all.sh           # Unix build
│   ├── create_launchers.ps1   # Create launchers
│   ├── package_release.ps1    # Package release
│   └── ...
├── ci/                         # CI/CD scripts
├── verification/               # Test scripts
├── README.md                   # Full documentation
└── QUICK_START.md             # This file
```

---

## ⚡ Fast Commands

```bash
# Quick build (debug)
cargo build --workspace

# Fast test (lib only)
cargo test --lib

# Specific crate
cargo build -p launcher-core

# With logging
RUST_LOG=debug .\scripts\build\build_omni.ps1
```

---

## 🎯 Most Used

**For daily development:**
```bash
cargo build --workspace
cargo test --lib --workspace
cargo fmt --all
cargo clippy --all
```

**Before committing:**
```bash
.\scripts\verification\master_verify.ps1
```

**For release:**
```bash
.\scripts\build\package_release.ps1
```

---

**More Details**: See [README.md](README.md)
