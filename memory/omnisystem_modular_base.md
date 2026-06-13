---
name: omnisystem_modular_base
description: Omnisystem modular base design - minimal boot, dynamic module loading, custom extensibility
metadata:
  type: project
---

## Omnisystem Modular Architecture (Critical Requirement)

**Date**: 2026-06-10  
**User Requirement**: "Ensure that the Omnisystem can be run with just Base Modules...has the ability to download new modules from GitHub repo as well as add custom modules from a users repo"

**Status**: Architecture Complete, Ready for Implementation

### Base Modules (Required for boot)

Must be <50MB total to ensure minimal footprint:

1. **omnisystem-kernel** (800 LOC, ✅ done): Memory, scheduling, IPC
2. **omnisystem-ffi** (1,200 LOC, ✅ done): C/FFI interop
3. **omnisystem-sylva-core** (600 LOC, ✅ done): Bytecode IR
4. **omnisystem-network-core** (400 LOC, 🔲 design): Basic networking
5. **omnisystem-logging** (300 LOC, 🔲 design): Diagnostics
6. **omnisystem-module-system** (1,500 LOC, 🔲 design): **CRITICAL - Module loader/manager**

### Total Base: ~4,800 LOC, 4/6 complete

### Key Architecture Principles

**Why modular**: Different users need different features
- Embedded device: Just base modules (50MB)
- Developer workstation: Base + Infrastructure + OmniLingual
- Manufacturing: Base + Infrastructure + Phase 14 (printing) + Phase 15 (AI)
- Enterprise: Base + custom company modules

**Module sources (discovery order)**:
1. Built-in base (compiled in, always available)
2. Local ~/.omnisystem/modules/ (user's custom)
3. GitHub official: github.com/omnisystem/modules (signed)
4. GitHub community: Topic `omnisystem-module` (community)
5. Private repos: `$OMNISYSTEM_CUSTOM_REPOS` (enterprise)

**Module manifest**: Each module has `omnisystem.toml` declaring:
- Exports (functions, types, traits)
- Dependencies (other modules)
- Security (GPG signatures, sandbox policy)
- Source (where to download if missing)

### How to Apply

**User perspective**:
- Install Omnisystem base (50MB)
- It boots in <1 second
- Config file lists desired modules: `[enabled-modules]`
- Missing modules auto-download from GitHub on first boot
- Can add custom company modules to `~/.omnisystem/modules/`
- Omnisystem auto-discovers and loads them at startup

**Module developer perspective**:
- Create crate with `omnisystem.toml` manifest
- Implement `omnisystem_module_init()` and `omnisystem_module_shutdown()`
- Publish to GitHub with topic `omnisystem-module`
- Or keep private and point Omnisystem to custom repo via env var
- Users can directly reference it: `omnisystem load-module my-company:omnisystem-custom-billing`

### Timeline

Phase 4A (3 weeks): omnisystem-module-system core + discovery  
Phase 4B (2 weeks): Remote source integration (GitHub, custom repos)  
Phase 4C (2 weeks): Security (signing, sandbox, capability checks)  

**Total: 7 weeks for production-grade module system**

### Why This Matters

- **Flexibility**: Users get exactly what they need, nothing more
- **Scalability**: Grows from 50MB to 1GB+ as needed
- **Sovereignty**: Users control what modules run
- **Enterprise**: Private/company modules work identically to official
- **Community**: Anyone can extend Omnisystem without permission
- **Compatibility**: Same module system across all 5 OS families

### Success Criteria

✅ Base system boots in <1s  
✅ Minimal footprint <50MB  
✅ Module discovery from 5 sources  
✅ Auto-download from GitHub with signature verification  
✅ Custom modules work identically to official  
✅ Dependency resolution is deterministic and fast  
✅ Security model (signing, sandbox) is enterprise-grade  
