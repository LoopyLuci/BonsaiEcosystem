# Omnisystem Build Scripts

**Repository**: Omnisystem  
**Team**: Omnisystem Team  
**Version**: 1.0.0  
**Last Updated**: 2026-06-12  
**Status**: ✅ Production Ready  

Complete build and deployment automation for the Omnisystem project.

---

## 📁 Directory Structure

```
scripts/
├── README.md                          # This file
├── QUICK_START.md                     # Quick reference guide
├── build/                             # Build automation scripts
├── ci/                                # CI/CD pipeline scripts
├── verification/                      # Verification and testing
├── cleanup/                           # Cleanup and maintenance
└── deploy/                            # Deployment scripts
```

---

## 🚀 Quick Start

### Build Everything
```bash
# PowerShell (Windows)
.\scripts\build\build_omni.ps1

# Bash (macOS/Linux)
./scripts/build/build_all.sh
```

### Build Specific Component
```bash
# Launcher system
.\scripts\build\create_launchers.ps1

# Package release
.\scripts\build\package_release.ps1
```

### Run Tests
```bash
# All tests
.\scripts\test-all.ps1
./scripts/test_all.sh

# Specific test suite
.\scripts\run_full_validation.ps1
```

### Verify Installation
```bash
.\scripts\verification\master_verify.ps1
```

---

## 📂 Folder Organization

### 1. `build/` - Build Scripts

Primary build automation for Omnisystem components.

| Script | Purpose | Platform |
|--------|---------|----------|
| `build_omni.ps1` | Build entire Omnisystem | Windows |
| `build_all.sh` | Build entire Omnisystem | Linux/macOS |
| `build_omnisystem.ps1` | Quick Omnisystem build | Windows |
| `build_native_compiler.ps1` | Build native compiler | Windows |
| `BUILD_PHASE1.sh` | Phase 1 build (foundation) | Linux/macOS |
| `BUILD_PHASE2.sh` | Phase 2 build (core) | Linux/macOS |
| `BUILD_PHASE3.ps1` | Phase 3 build (advanced) | Windows |
| `BUILD_PHASE3_COMPLETE.ps1` | Complete Phase 3 | Windows |
| `build_system.sh` | Build system utilities | Linux/macOS |
| `create_launchers.ps1` | Create launcher executables | Windows |
| `package_release.ps1` | Package release binaries | Windows |

### 2. `ci/` - CI/CD Pipeline

Continuous integration and automated testing scripts.

| Script | Purpose |
|--------|---------|
| `check_bootstrap_invariants.ps1` | Verify bootstrap integrity |
| `verify_no_stubs.ps1` | Check for incomplete implementations |

### 3. `verification/` - Testing & Validation

Comprehensive verification and testing suite.

| Script | Purpose |
|--------|---------|
| `master_verify.ps1` | Complete verification suite |
| `verify_aether.ps1` | Aether language verification |
| `verify_AX1.ps1` | AX1 verification |

### 4. `cleanup/` - Maintenance

Build artifact cleanup and workspace maintenance.

---

## 🔧 Common Tasks

### Clean Build
```bash
# PowerShell
.\scripts\build\build_omni.ps1 -Clean

# Bash
./scripts/build/build_system.sh clean
```

### Build Release Binary
```bash
# Create optimized release
.\scripts\build\package_release.ps1
```

### Create Launcher Executables
```bash
# Build CLI and web launchers
.\scripts\build\create_launchers.ps1
```

### Run All Tests
```bash
# PowerShell
.\scripts\test-all.ps1

# Bash
./scripts/test_all.sh
```

### Verify Installation
```bash
.\scripts\verification\master_verify.ps1
```

### Stress Test
```bash
./scripts/run_stress_tests.sh
```

---

## 🛠️ Usage Patterns

### Pattern 1: Full Build & Test
```bash
# Build everything and run tests
.\scripts\build\build_omni.ps1
.\scripts\test-all.ps1
.\scripts\verification\master_verify.ps1
```

### Pattern 2: Development Build
```bash
# Quick build for development
cargo build --workspace
cargo test --lib
```

### Pattern 3: Release Build
```bash
# Optimized release build
.\scripts\build\package_release.ps1
```

### Pattern 4: CI/CD Pipeline
```bash
# Automated CI checks
.\scripts\ci\check_bootstrap_invariants.ps1
.\scripts\ci\verify_no_stubs.ps1
.\scripts\test-all.ps1
```

---

## 📋 Windows Scripts (PowerShell)

All PowerShell scripts use `.ps1` extension and can be run as:

```powershell
# From PowerShell console
.\scripts\build\build_omni.ps1

# With parameters
.\scripts\build\build_omni.ps1 -Release -Parallel
```

### Execution Policy

If you get execution policy errors:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

---

## 📋 Unix Scripts (Bash)

All Bash scripts use `.sh` extension and can be run as:

```bash
# Make executable if needed
chmod +x scripts/build/build_all.sh

# Run
./scripts/build/build_all.sh

# With environment variables
RELEASE=1 ./scripts/build/build_all.sh
```

---

## 🔄 Build Phases

The build process is organized into phases:

### Phase 1: Foundation
- Core types and traits
- Base modules
- Kernel components

**Script**: `BUILD_PHASE1.sh` or `build_omnisystem.ps1`

### Phase 2: Core Systems
- Compiler infrastructure
- Runtime systems
- Standard library

**Script**: `BUILD_PHASE2.sh`

### Phase 3: Advanced Systems
- Language features
- Optimization passes
- System integration

**Script**: `BUILD_PHASE3.ps1` or `BUILD_PHASE3_COMPLETE.ps1`

---

## 🎯 Recommended Workflows

### For Development
```bash
# 1. Build once
.\scripts\build\build_omni.ps1

# 2. Work on code with cargo
cargo test --lib --workspace

# 3. Run verification before commit
.\scripts\verification\master_verify.ps1
```

### For Release
```bash
# 1. Clean build
.\scripts\build\build_omni.ps1 -Clean

# 2. Package release
.\scripts\build\package_release.ps1

# 3. Verify integrity
.\scripts\verification\master_verify.ps1

# 4. Run full test suite
.\scripts\test-all.ps1
```

### For CI/CD
```bash
# 1. Check bootstrap
.\scripts\ci\check_bootstrap_invariants.ps1

# 2. Verify no stubs
.\scripts\ci\verify_no_stubs.ps1

# 3. Run tests
.\scripts\test-all.ps1

# 4. Package
.\scripts\build\package_release.ps1
```

---

## 🔗 Dependencies

Scripts depend on:
- **Rust 1.70+** (for cargo commands)
- **Node.js 18+** (for npm scripts)
- **Git 2.30+** (for version control)
- **PowerShell 7+** (Windows) or **Bash 4+** (Unix)

---

## 📊 Script Statistics

| Category | Count | Total LOC |
|----------|-------|-----------|
| Build Scripts | 11 | ~3,000 |
| CI/CD Scripts | 2 | ~500 |
| Verification Scripts | 3 | ~2,000 |
| Test Scripts | 2 | ~500 |
| **Total** | **18** | **~6,000** |

---

## 🐛 Troubleshooting

### Script Not Found
```bash
# Check if script exists
ls scripts/build/

# Make sure you're in Omnisystem directory
pwd  # Should end with /Omnisystem
```

### Permission Denied (Unix)
```bash
# Make script executable
chmod +x scripts/build/*.sh
```

### Execution Policy (Windows)
```powershell
# Allow script execution
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Build Failure
```bash
# Clean and rebuild
cargo clean
.\scripts\build\build_omni.ps1
```

---

## 🔍 What Each Script Does

### build_omni.ps1
Comprehensive build of entire Omnisystem workspace
- Builds all crates in dependency order
- Compiles Rust components
- Generates documentation
- Creates artifacts

### build_all.sh
Unix equivalent of build_omni.ps1
- Cross-platform build support
- Bash-based implementation
- Same functionality as PowerShell version

### create_launchers.ps1
Creates standalone launcher executables
- Compiles CLI launcher
- Compiles Web server launcher
- Copies to C:\Launcher
- Creates batch/PowerShell scripts

### package_release.ps1
Creates optimized release binaries
- Strips debug symbols
- Optimizes code
- Creates installers
- Packages artifacts

### test-all.ps1 / test_all.sh
Runs entire test suite
- Unit tests
- Integration tests
- Documentation tests
- Stress tests

### master_verify.ps1
Complete verification suite
- Checks build integrity
- Runs test suite
- Verifies documentation
- Validates artifacts

---

## 📝 Contributing

When adding new scripts:

1. **Naming**: Use clear, descriptive names
2. **Comments**: Include header with purpose
3. **Error Handling**: Use proper exit codes
4. **Documentation**: Update this README
5. **Organization**: Place in appropriate subdirectory
6. **Consistency**: Follow existing script patterns

---

## 🚀 Next Steps

1. **Explore**: Browse `build/`, `ci/`, `verification/` folders
2. **Learn**: Read individual script headers for details
3. **Use**: Start with `build_omni.ps1` or `build_all.sh`
4. **Automate**: Integrate into your CI/CD pipeline
5. **Improve**: Contribute enhancements and fixes

---

## 📞 Support

For issues with scripts:
- Check script header comments
- Review error messages carefully
- Ensure prerequisites are installed
- Run with verbose flags if available
- Check GitHub issues for solutions

---

## 📄 Related Documentation

- [QUICK_START.md](QUICK_START.md) — Quick reference
- [../README.md](../README.md) — Project overview
- [../CHANGELOG.md](../CHANGELOG.md) — Version history

---

**Last Updated**: 2026-06-12  
**Status**: ✅ Production Ready

All scripts are tested and production-ready. Use with confidence! 🎉
