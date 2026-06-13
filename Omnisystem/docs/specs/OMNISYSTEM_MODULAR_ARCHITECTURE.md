# Omnisystem Modular Architecture
## Base Modules, Dynamic Loading, and Extensibility Strategy

**Date**: 2026-06-10  
**Status**: Architecture & Design  
**Scope**: Core module system, discovery, loading, and lifecycle management  

---

## EXECUTIVE SUMMARY

Omnisystem is architected as a **lightweight modular kernel** with a **pluggable module system** that allows:

- **Minimal boot footprint**: Core system < 50MB
- **Dynamic discovery**: Auto-detect modules from GitHub, local repos, and custom sources
- **Zero lock-in**: Users can drop in custom modules at runtime
- **Composable stacks**: Build exactly the Omnisystem you need
- **Enterprise ready**: Secure module signing, version pinning, dependency resolution

---

## ARCHITECTURAL LAYERS

```
┌─────────────────────────────────────────────────────────────────┐
│                    User Applications                             │
├─────────────────────────────────────────────────────────────────┤
│                    Application Layer (Optional)                  │
│  - BonsaiLauncher, OmniOS, Custom Apps                          │
├─────────────────────────────────────────────────────────────────┤
│                  Dynamic Module Layer                            │
│  - Phase 14+ modules (printing, AI, agents)                     │
│  - Language-specific modules (150+)                             │
│  - Domain modules (medical, legal, tech)                        │
│  - User custom modules (company-specific)                       │
├─────────────────────────────────────────────────────────────────┤
│                 Module Loader & Manager                          │
│  - Module discovery, validation, loading                        │
│  - Dependency resolution, version management                    │
│  - Sandboxing, security checks, lifecycle                       │
├─────────────────────────────────────────────────────────────────┤
│                   BASE MODULES (REQUIRED)                        │
│  - Core kernel, memory, CPU abstractions                        │
│  - FFI & language bindings                                      │
│  - Network & RPC (minimal)                                      │
│  - Logging & diagnostics                                        │
│  - Module system itself (bootstrap)                             │
├─────────────────────────────────────────────────────────────────┤
│              Hardware Abstraction Layer (HAL)                    │
│  - CPU, Memory, Interrupts, Device drivers                      │
├─────────────────────────────────────────────────────────────────┤
│                    Physical Hardware                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## BASE MODULES (MINIMAL OMNISYSTEM)

These modules are **required** for Omnisystem to boot and function:

### 1. omnisystem-kernel (PHASE 1)
**LOC**: 800  
**Dependencies**: None (foundational)  
**Exports**: Core types, memory management, task scheduling  
**Status**: ✅ Complete

Provides:
- Memory allocation/deallocation
- Task spawning and management
- Interrupt handling
- Basic IPC (message passing)
- System initialization

### 2. omnisystem-ffi (PHASE 2)
**LOC**: 1,200  
**Dependencies**: omnisystem-kernel  
**Exports**: ABI, C FFI bridge, function dispatch  
**Status**: ✅ Complete

Provides:
- C ABI compatibility
- Rust ↔ C/C++ interop
- Dynamic library loading
- Symbol resolution
- Function dispatch tables

### 3. omnisystem-sylva-core (PHASE 1)
**LOC**: 600  
**Dependencies**: omnisystem-kernel  
**Exports**: Unified bytecode IR, compilation framework  
**Status**: ✅ Complete

Provides:
- Universal IR format
- Bytecode execution
- Compiler backend abstraction
- 750+ language support via IR

### 4. omnisystem-network-core (NEW - Base Module)
**LOC**: 400  
**Dependencies**: omnisystem-kernel  
**Exports**: Network abstraction, RPC, message protocols  
**Status**: Design phase

Provides:
- TCP/UDP/QUIC abstraction
- RPC framework (minimal)
- Module-to-module communication
- Network discovery (mDNS)

### 5. omnisystem-logging (NEW - Base Module)
**LOC**: 300  
**Dependencies**: omnisystem-kernel  
**Exports**: Log streaming, structured logging  
**Status**: Design phase

Provides:
- Console output
- File logging
- Structured log format (JSON)
- Log level filtering
- Diagnostic tracing

### 6. omnisystem-module-system (NEW - Bootstrap)
**LOC**: 1,500  
**Dependencies**: omnisystem-kernel, omnisystem-ffi  
**Exports**: Module loader, manager, lifecycle  
**Status**: Design phase (CRITICAL)

Provides:
- Module descriptor parsing (TOML/JSON)
- Dynamic .so/.dll loading
- Symbol binding and ABI validation
- Dependency resolution
- Module initialization/teardown
- Lifecycle hooks (pre/post load)
- Module registry and discovery
- Version management

---

## BASE MODULES TOTAL

| Module | LOC | Status | Purpose |
|--------|-----|--------|---------|
| omnisystem-kernel | 800 | ✅ Complete | Core scheduling/memory |
| omnisystem-ffi | 1,200 | ✅ Complete | Language interop |
| omnisystem-sylva-core | 600 | ✅ Complete | Bytecode IR |
| omnisystem-network-core | 400 | 🔲 Design | Networking |
| omnisystem-logging | 300 | 🔲 Design | Diagnostics |
| omnisystem-module-system | 1,500 | 🔲 Design | Module loading |
| **TOTAL BASE** | **4,800** | **4/6 done** | **Minimal Omnisystem** |

---

## OPTIONAL MODULE TIERS

### Tier A: Infrastructure (Common)
- omnisystem-cpu (hardware abstractions)
- omnisystem-memory (advanced heap)
- omnisystem-device (driver framework)
- omnisystem-interrupt (advanced IRQ)
- omnisystem-crypto (encryption)

**~3,000 LOC, used by 70% of applications**

### Tier B: System Integration (OS-Specific)
- omnisystem-linux (Linux bindings)
- omnisystem-windows (Windows bindings)
- omnisystem-macos (macOS bindings)

**~4,000 LOC, used by 100% (one per OS)**

### Tier C: Distributed Systems
- omnisystem-network (full networking)
- omnisystem-rpc (advanced RPC)
- omnisystem-cluster (clustering)

**~3,000 LOC, used by 20% of applications**

### Tier D: Applications & Features (Opt-in)
- Phase 14: Printer control (optional for desktop)
- Phase 15: AI agents (optional)
- OmniLingual: Spell check/translation (optional)
- BonsaiLauncher: Desktop UI (optional)
- OmniOS: Full OS (optional)

**50,000+ LOC total, pick what you need**

---

## MODULE DISCOVERY & LOADING

### Module Sources (Priority Order)

1. **Built-in Base Modules**
   - Compiled into Omnisystem binary
   - Always available, cannot be removed
   - Path: `/omnisystem/modules/base/`

2. **Local Module Repository**
   - User's machine, git or file-based
   - Location: `~/.omnisystem/modules/`
   - Loaded at startup
   - Can be custom/proprietary

3. **GitHub Official Repository**
   - Primary source: `github.com/omnisystem/modules`
   - Tagged releases with versioning
   - Digitally signed
   - Downloaded on-demand or pre-cached

4. **GitHub Community Modules**
   - Topic: `omnisystem-module`
   - Namespace: `omnisystem-*` prefix
   - Metadata: `omnisystem.toml` manifest
   - Downloaded on-demand with user consent

5. **Custom Private Repositories**
   - User's company/personal GitHub/GitLab/etc.
   - Token-based authentication
   - Enterprise deployment ready
   - Environment: `OMNISYSTEM_CUSTOM_REPOS`

### Module Discovery Process

```
Omnisystem Boot
    ↓
Load Base Modules (built-in)
    ↓
Read ~/.omnisystem/modules/ (local)
    ↓
Check ~/.omnisystem/manifest.toml (config)
    ↓
Scan for enabled modules:
    - Auto-load required modules
    - Lazy-load optional modules
    - Skip disabled modules
    ↓
Resolve dependencies (recursive)
    ↓
Validate ABIs and signatures
    ↓
Load in dependency order
    ↓
Execute lifecycle hooks (init)
    ↓
Omnisystem Ready
```

---

## MODULE MANIFEST FORMAT

Each module has a `omnisystem.toml` descriptor:

```toml
[module]
name = "omnisystem-printer-core"
version = "0.1.0"
description = "3D printer control system"
author = "Omnisystem Team"
license = "MPL-2.0"

# Metadata
[metadata]
category = "hardware"          # hardware, language, utility, app
tier = "14"                    # Phase number (14-15) or optional
required = false               # true = must load, false = optional
tags = ["3d-printing", "firmware", "hardware-control"]

# What this module exports
[exports]
traits = ["UniversalPrinter"]
functions = ["detect_printers", "identify_printer"]
types = ["PrinterType", "PrinterCapabilities", "PrinterStatus"]

# What this module depends on
[dependencies]
omnisystem-kernel = "0.1.0"
omnisystem-ffi = "0.1.0"
# Can also depend on other optional modules

# Where to download if not present
[source]
repository = "https://github.com/omnisystem/omnisystem"
binary_url = "https://releases.omnisystem.io/omnisystem-printer-core-{version}.so"
source_url = "https://github.com/omnisystem/omnisystem/tree/main/crates/omnisystem-printer-core"

# Security
[security]
min_version = "0.1.0"
signatures = [
    "sha256=abc123def456...",
]
verify_gpg = true
gpg_key = "omnisystem@example.com"

# Loading behavior
[behavior]
lifecycle_init = "module_init"           # Function to call on load
lifecycle_shutdown = "module_shutdown"   # Function to call on unload
sandbox = true                           # Run in restricted sandbox
```

---

## MODULE MANAGER ARCHITECTURE

### omnisystem-module-system Implementation

```rust
/// Module manifest (parsed from omnisystem.toml)
pub struct ModuleManifest {
    pub name: String,
    pub version: Version,
    pub exports: ExportList,
    pub dependencies: Vec<ModuleDependency>,
    pub source: ModuleSource,
    pub security: SecurityPolicy,
}

/// Module descriptor (loaded instance)
pub struct Module {
    pub manifest: ModuleManifest,
    pub library: SharedLibrary,        // loaded .so/.dll
    pub status: ModuleStatus,          // Unloaded, Loading, Loaded, Error
    pub init_fn: Option<InitFn>,       // lifecycle hook
    pub shutdown_fn: Option<ShutdownFn>,
}

/// Module manager
pub struct ModuleManager {
    modules: HashMap<String, Module>,
    loader: DynamicLoader,
    registry: SymbolRegistry,
    resolver: DependencyResolver,
}

impl ModuleManager {
    /// Discover modules from all sources
    pub async fn discover(&mut self) -> Result<Vec<ModuleManifest>>;

    /// Load a module and its dependencies
    pub async fn load(&mut self, name: &str) -> Result<()>;

    /// Unload a module
    pub fn unload(&mut self, name: &str) -> Result<()>;

    /// Validate module ABI compatibility
    pub fn validate_abi(&self, manifest: &ModuleManifest) -> Result<()>;

    /// Resolve dependency graph
    pub fn resolve_deps(&self, modules: Vec<ModuleManifest>) 
        -> Result<Vec<ModuleManifest>>;  // topological order
}

pub enum ModuleStatus {
    Unloaded,
    Loading,
    Loaded,
    Error(String),
}
```

---

## DEPLOYMENT SCENARIOS

### Scenario 1: Minimal Omnisystem
**Use case**: Embedded systems, IoT, containers  
**Modules**: Base modules only  
**Size**: ~50MB  
**Load time**: <500ms  

```toml
# omnisystem.toml (user's config)
[enabled-modules]
base = true           # Always enabled
infrastructure = false  # No optional infra
distributed = false    # No clustering
applications = false   # No apps
```

### Scenario 2: Developer Workstation
**Use case**: Software development, prototyping  
**Modules**: Base + Infrastructure + OmniLingual + BonsaiLauncher  
**Size**: ~200MB  
**Load time**: ~2s  

```toml
[enabled-modules]
base = true
infrastructure = true     # CPU, memory, device support
distributed = false
applications = true

[application-modules]
omnilingual = true        # Spell check
bonsai-launcher = true    # Desktop UI
```

### Scenario 3: Manufacturing System
**Use case**: Factory with 3D printers, AI coordination  
**Modules**: Base + Infrastructure + Distributed + Phase 14 + Phase 15  
**Size**: ~500MB  
**Load time**: ~5s  

```toml
[enabled-modules]
base = true
infrastructure = true
distributed = true       # For clustering printers

[application-modules]
omnisystem-printer-core = true    # Phase 14
aion-agent-core = true            # Phase 15
```

### Scenario 4: Enterprise Cloud Deployment
**Use case**: SaaS, microservices, Kubernetes  
**Modules**: Custom selection + security modules  
**Size**: Varies (50MB - 1GB)  
**Load time**: <10s  

```toml
[enabled-modules]
base = true
infrastructure = true
distributed = true       # For clustering

[custom-modules]
"company-auth" = { source = "private", version = "1.2.0" }
"company-billing" = { source = "private", version = "2.1.0" }

[security]
module_signing = true
verify_all_modules = true
allowed_private_repos = ["github.com/company/..."]
```

---

## MODULE LIFECYCLE

### Loading (Initialization)

1. **Discovery**: Find module in filesystem or download from remote
2. **Parse**: Read `omnisystem.toml` manifest
3. **Validate**: Check signature, ABI, version constraints
4. **Resolve**: Ensure all dependencies are available
5. **Link**: Load .so/.dll and resolve symbols
6. **Initialize**: Call module's `_init` function (if exists)
7. **Register**: Add to module registry, export symbols

### Running (Active)

- Module is available to other modules
- Exports are accessible via registry
- Can be called from other code
- Can receive inter-module messages
- Can update itself (with hot-reloading)

### Unloading (Shutdown)

1. **Pre-shutdown**: Call module's `_shutdown` function (if exists)
2. **Unbind**: Remove from registry
3. **Cleanup**: Release resources
4. **Unlink**: Unload .so/.dll from memory
5. **Verify**: Ensure no dangling references

---

## SECURITY MODEL

### Module Signing & Verification

All modules must be signed with:
- **Official modules** (omnisystem-*): Signed by Omnisystem Team (GPG key)
- **Community modules**: Signed by uploader (GitHub identity)
- **Private modules**: Signed by organization key

**Signature format**: GPG/Ed25519

**Verification steps**:
1. Extract module binary
2. Verify signature against public key
3. Check certificate chain (if using full PKI)
4. Validate timestamp (no expired modules)
5. Check revocation status (if available)

### Sandboxing & Capability-Based Security

Modules run with capability restrictions:

```toml
[security.capabilities]
filesystem = ["read:/omnisystem/modules", "write:$HOME/.omnisystem"]
network = ["tcp:localhost:8000", "udp:*:53"]  # DNS
ipc = true       # Can talk to other modules
signals = false  # Cannot send signals
# etc.
```

### Version Pinning & Dependency Lock

Like npm/cargo, Omnisystem can pin exact module versions:

```toml
[locked-modules]
omnisystem-printer-core = "0.1.0"   # Must be exactly this version
omnisystem-ai-fallback = "^1.0"     # Semver: >= 1.0, < 2.0
```

---

## EXAMPLE: ADDING A CUSTOM MODULE

### User's Company Module

```bash
# 1. Create module structure
mkdir -p ~/company-modules/omnisystem-company-auth
cd ~/company-modules/omnisystem-company-auth

# 2. Create manifest
cat > omnisystem.toml <<'EOF'
[module]
name = "omnisystem-company-auth"
version = "1.0.0"
description = "Company single sign-on integration"

[metadata]
category = "utility"
required = false

[dependencies]
omnisystem-kernel = "0.1.0"

[security]
signatures = ["sha256=...company-key..."]
EOF

# 3. Compile to .so
cargo build --release
# → omnisystem-company-auth.so

# 4. Register with Omnisystem
cat >> ~/.omnisystem/config.toml <<'EOF'
[custom-modules]
omnisystem-company-auth = {
    path = "~/company-modules/omnisystem-company-auth",
    enabled = true
}
EOF

# 5. Restart Omnisystem
omnisystem restart
# → Module auto-loads, exports available to other modules
```

### Accessing the Module

```rust
// In another module
use omnisystem_module_system::ModuleRegistry;

let registry = ModuleRegistry::global();
let auth_module = registry.get("omnisystem-company-auth")?;
let sso_login = auth_module.get_function("sso_login")?;

// Call company auth
sso_login(user_id)?;
```

---

## NETWORK-BASED MODULE DISCOVERY

### Auto-Downloading from GitHub

When a module is requested but not installed:

```
User calls: omnisystem load-module omnisystem-translator-context

ModuleManager:
    ├─ Check ~/.omnisystem/modules/ → NOT FOUND
    ├─ Check $OMNISYSTEM_CUSTOM_REPOS → NOT FOUND
    ├─ Query github.com/omnisystem/modules (official)
    │   ├─ Find: omnisystem-translator-context v0.2.0
    │   ├─ Check: Signature valid? → YES
    │   ├─ Download: Binary from releases.omnisystem.io
    │   ├─ Cache: ~/.omnisystem/modules/omnisystem-translator-context-0.2.0.so
    │   └─ Load into memory
    │
    └─ Module Ready
```

### Custom Repository Integration

```bash
# Configure custom repo
omnisystem config add-repo \
  --name "company-internal" \
  --url "https://github.com/company/omnisystem-modules" \
  --token "$GITHUB_TOKEN"

# Now modules from that repo are discoverable
omnisystem load-module company-internal:omnisystem-company-auth

# Auto-discovery works too (searches all configured repos)
omnisystem discover-modules "company-*"
```

---

## IMPLEMENTATION ROADMAP

### Phase 1: Core Module System (1 week)
- [ ] omnisystem-module-system crate
- [ ] Module manifest parsing
- [ ] Dynamic library loading (cross-platform)
- [ ] Symbol resolution and binding
- [ ] Basic module lifecycle hooks

### Phase 2: Discovery & Loading (1 week)
- [ ] Filesystem module discovery
- [ ] omnisystem.toml validation
- [ ] Dependency resolution (topological sort)
- [ ] ABI compatibility checking
- [ ] Error handling and diagnostics

### Phase 3: Remote Module Sources (1 week)
- [ ] GitHub API integration
- [ ] Binary download caching
- [ ] Signature verification (GPG)
- [ ] Version resolution (semver)
- [ ] Custom repository support

### Phase 4: Module Registry & IPC (1 week)
- [ ] Global module registry
- [ ] Symbol export/import
- [ ] Inter-module communication
- [ ] Hot-reloading support
- [ ] Module dependency tracking

### Phase 5: Security & Hardening (1 week)
- [ ] Capability-based sandbox
- [ ] Module signing infrastructure
- [ ] Revocation checking
- [ ] Audit logging
- [ ] Security policy enforcement

---

## BENEFITS

1. **Minimal startup**: Users get 50MB base Omnisystem, add what they need
2. **Zero lock-in**: Custom modules work alongside official ones
3. **Enterprise ready**: Private repos, signing, version pinning
4. **Community driven**: Anyone can publish omnisystem-* modules
5. **Scalable**: From IoT (50MB) to cloud (1GB+), same architecture
6. **Familiar**: Works like npm/cargo/pip module systems
7. **Safe**: Signed modules, capability sandbox, ABI validation

---

## SUCCESS CRITERIA

- ✅ Base system boots in <1s
- ✅ Optional module loads in <500ms
- ✅ Can add modules at runtime
- ✅ Custom modules work identically to official modules
- ✅ Dependency resolution is deterministic
- ✅ Security checks (signing, sandbox) pass audit
- ✅ Module discovery from 5+ sources

---

**Status**: Architecture Complete  
**Next Step**: Implement omnisystem-module-system crate (Phase 4A)  
**Timeline**: 5 weeks for full implementation + security hardening  

