# Application Manager - Connector Architecture
## How All 12 Modules Interconnect Through Central Hub

**Date**: 2026-06-11  
**Architecture**: Hub-and-Spoke Connector Pattern  
**Central Hub**: ApplicationManager (app-manager-omnisystem-integration)  
**Module Count**: 12 specialized crates  
**Connection Model**: Arc-wrapped trait objects with async/await  

---

## SYSTEM OVERVIEW

The Application Manager uses a **hub-and-spoke connector pattern** where:
- **Hub**: `ApplicationManager` (central orchestrator)
- **Spokes**: 11 specialized modules that connect to the hub

```
                    ┌─────────────────────┐
                    │ ApplicationManager  │
                    │   (Central Hub)     │
                    └──────────┬──────────┘
         ┌──────────────────────┼──────────────────────┐
         │                      │                      │
         ▼                      ▼                      ▼
    ┌─────────┐         ┌──────────────┐      ┌─────────────┐
    │Lifecycle│         │ Dependency   │      │   Module    │
    │ Manager │         │   Graph      │      │  Resolver   │
    └────┬────┘         └──────┬───────┘      └──────┬──────┘
         │                     │                     │
         ├─ Registry          ├─ Circular           ├─ Constraints
         ├─ States            ├─ Detection          ├─ Conflicts
         ├─ Transitions       ├─ Topological        └─ Compatibility
         └─ History           └─ Sort
         
    ┌──────────────────────────────┐
    │   Version Manager            │
    ├─ History                     │
    ├─ Rollback                    │
    └─ Release Channels            │
    
    ┌──────────────────────────────┐
    │   Repository Module          │
    ├─ GitHub Integration          │
    ├─ Marketplace API             │
    └─ Local File Loading          │
    
    ┌──────────────────────────────┐
    │   Installer Module           │
    ├─ Download Orchestration      │
    ├─ Installation Context        │
    └─ Rollback Manager            │
    
    ┌──────────────────────────────┐
    │   Security Module            │
    ├─ Permissions (11 types)      │
    ├─ Signature Verification      │
    ├─ Sandbox (4 levels)          │
    └─ Audit Logging               │
    
    ┌──────────────────────────────┐
    │   Configuration Module       │
    ├─ App Settings                │
    ├─ Environment Variables       │
    └─ Log Level Management        │
```

---

## MODULE CONNECTIONS (Detailed)

### 1. CORE ORCHESTRATION (4 modules from app-manager-core)

#### 1.1 ModuleLifecycleManager
**Purpose**: Manages module state and lifecycle transitions  
**Connected To**: ApplicationManager (primary)  
**Key Methods**:
```rust
register_module(app_id)       // Discover state
download(app_id)              // Discover→Downloading→Downloaded
verify(app_id)                // Downloaded→Verifying→Verified
install(app_id)               // Verified→Installing→Installed
load(app_id)                  // Installed→Loading→Loaded
start(app_id)                 // Loaded→Running
stop(app_id)                  // Running→Stopped
unload(app_id)                // Running/Loaded→Unloading→Unloaded
```

**Connection Point**:
```rust
// In ApplicationManager::load_application()
self.lifecycle_manager.register_module(app_id.clone())?;
self.lifecycle_manager.download(app_id).await?;
self.lifecycle_manager.verify(app_id).await?;
self.lifecycle_manager.install(app_id).await?;
self.lifecycle_manager.load(app_id).await?;
```

#### 1.2 DependencyGraph
**Purpose**: Maintains dependency map and validates constraints  
**Connected To**: ModuleResolver, ApplicationManager  
**Key Methods**:
```rust
add_module(module_info)              // Register module with deps
get_module(app_id)                   // O(1) lookup
detect_cycles()                      // Validates DAG
topological_sort()                   // Ordering for installation
get_orphans()                        // Finds unused modules
```

**Connection Points**:
```rust
// In ModuleResolver
let dependencies = self.graph
    .get_module(app_id)
    .dependencies;

// In ApplicationManager
self.dependency_graph.module_count()
```

#### 1.3 ModuleResolver
**Purpose**: Resolves semantic version constraints and conflicts  
**Connected To**: DependencyGraph, ApplicationManager  
**Key Methods**:
```rust
find_compatible_version(app_id, constraint)  // Matches semver
resolve_dependencies(app_id, version)        // Direct deps
suggest_versions(app_id, constraint)         // Suggestions
```

**Connection Point**:
```rust
// In ApplicationManager::load_application()
let dependencies = self.module_resolver
    .resolve_dependencies(app_id, version)?;
```

#### 1.4 VersionManager
**Purpose**: Tracks version history and enables rollback  
**Connected To**: ApplicationManager  
**Key Methods**:
```rust
update_version(app_id, version)      // Records version
get_versions(app_id)                 // History
rollback_to(app_id, version)         // Previous version
is_update_available(app_id)          // Update check
```

**Connection Point**:
```rust
// In ApplicationManager
self.version_manager.update_version(app_id.clone(), version.clone())?;
```

---

### 2. INSTALLATION & SOURCES (2 modules)

#### 2.1 Repository Module
**Purpose**: Provides unified access to multiple sources  
**Connected To**: Installer, ApplicationManager  
**Sources**:
- GitHub API (releases, manifests, assets)
- Marketplace API (search, trending, featured, ratings)
- Local file system (cached packages)

**Key Methods**:
```rust
// GitHub
fetch_release(owner, repo, tag)
fetch_manifest(url)
download_asset(url, path)

// Marketplace
search(query)
get_trending()
get_featured()
get_app_info(app_id)

// Local
load_from_cache(app_id)
list_cached()
```

**Connection Point**:
```rust
// In Installer::download()
let package = repository.fetch_from_github(url)?;
let package = repository.load_from_cache(app_id)?;
```

#### 2.2 Installer Module
**Purpose**: Orchestrates complete installation workflow  
**Connected To**: Repository, Security, Lifecycle, Rollback  
**Key Methods**:
```rust
download(app_id, url)                // Get from repository
install(app_id)                      // Execute installation
verify_package(app_id)               // Signature check
rollback(app_id, version)            // Restore previous
```

**Connection Pattern**:
```rust
// In ApplicationManager::load_application()
installer.download(app_id, source_url)?;
security.verify(app_id)?;            // Cross-module call
installer.install(app_id)?;
```

---

### 3. SECURITY & POLICY (2 modules)

#### 3.1 Security Module
**Purpose**: Enforces permissions, sandboxing, and audit  
**Connected To**: ApplicationManager, Installer, Lifecycle  
**Components**:
- **PermissionManager**: 11 permission types
- **SignatureVerifier**: HMAC-SHA256, SHA256
- **SandboxManager**: 4-level isolation
- **AuditLogger**: Immutable event log

**Key Methods**:
```rust
// Permissions
grant_permission(app_id, permission)
revoke_permission(app_id, permission)
has_permission(app_id, permission)

// Verification
verify_signature(app_id, signature)
verify_checksum(data, checksum)

// Sandbox
create_sandbox(app_id, level)
check_path_access(app_id, path)
check_host_access(app_id, host)

// Audit
log_event(action, app_id, details)
get_events(filters)
export_log()
```

**Connection Points**:
```rust
// In ApplicationManager::load_application()
self.security.verify(app_id).await?;

// In Installer
self.security.verify_signature(app_id, signature)?;

// In ApplicationManager lifecycle events
self.security.audit_action("start", app_id, actor)?;
```

#### 3.2 Configuration Module
**Purpose**: Manages app settings and environment  
**Connected To**: ApplicationManager, Installer  
**Key Methods**:
```rust
load_config(app_id)                  // Get settings
save_config(app_id, config)          // Persist settings
set_env_vars(app_id, vars)           // Environment
get_log_level(app_id)                // Logging config
```

**Connection Point**:
```rust
// In ApplicationManager or CLI
let config = config_manager.load_config(app_id)?;
```

---

### 4. USER INTERFACE LAYERS (4 modules)

#### 4.1 REST API Module
**Purpose**: HTTP interface to ApplicationManager  
**Connected To**: ApplicationManager  
**11 Endpoints**:
```
GET    /apps                          # List apps
GET    /apps/{id}                     # Get app info
POST   /apps/{id}/install             # Install
DELETE /apps/{id}                     # Uninstall
POST   /apps/{id}/start               # Start
POST   /apps/{id}/stop                # Stop
POST   /apps/{id}/update              # Update
POST   /apps/{id}/config              # Configure
GET    /apps/{id}/logs                # Get logs
GET    /system/health                 # Health check
GET    /marketplace/search             # Search marketplace
```

**Connection Pattern**:
```rust
// In handlers
pub async fn install_app(app_id: String, req: InstallRequest) {
    let result = app_manager.load_application(&app_id, &req.version).await;
    Json(ApiResponse::ok(result))
}
```

#### 4.2 CLI Tool Module
**Purpose**: Command-line interface  
**Connected To**: ApplicationManager (via handlers)  
**14 Commands**:
```
omnisystem-app install <APP>          # Install application
omnisystem-app uninstall <APP>        # Uninstall
omnisystem-app list [--filter=STATE]  # List apps
omnisystem-app status <APP>           # App status
omnisystem-app start <APP>            # Start app
omnisystem-app stop <APP>             # Stop app
omnisystem-app update <APP>           # Update app
omnisystem-app config <APP>           # Configure app
omnisystem-app search <QUERY>         # Search marketplace
omnisystem-app logs <APP>             # View logs
omnisystem-app verify <APP>           # Verify integrity
omnisystem-app rollback <APP>         # Rollback version
omnisystem-app health                 # System health
omnisystem-app --verbose              # Debug mode
```

**Connection Pattern**:
```rust
// In commands
async fn install_command(app: &str, version: Option<String>, ...) {
    let app_mgr = ApplicationManager::new();
    app_mgr.initialize().await?;
    app_mgr.load_application(&AppId::new(app)?, &version).await?;
}
```

#### 4.3 Web UI Module
**Purpose**: Browser-based dashboard  
**Connected To**: REST API → ApplicationManager  
**Tech Stack**: Svelte 5, responsive design  
**Features**:
- App discovery and installation
- Real-time status monitoring
- Dependency visualization
- Settings management
- Log viewer
- Health dashboard

**Data Flow**:
```
Svelte Component
  → REST API call
    → ApplicationManager method
      → Connector subsystem
        → Result back through chain
```

#### 4.4 Desktop UI Module
**Purpose**: Native desktop application  
**Connected To**: ApplicationManager (via IPC)  
**Tech Stack**: Tauri 2, native OS integration  
**Features**:
- System tray integration
- Drag-and-drop installation
- Real-time notifications
- Theme switching
- Quick launcher

**Connection Pattern**:
```rust
// In Tauri command handler
#[tauri::command]
async fn install_app(app_id: String, version: String) -> Result<String> {
    let app_mgr = ApplicationManager::new();
    app_mgr.load_application(&AppId::new(&app_id)?, &Version::parse(&version)?).await?;
    Ok("Installed".to_string())
}
```

---

### 5. ADVANCED FEATURES (3 modules)

#### 5.1 Marketplace Module
**Purpose**: App discovery and rating  
**Connected To**: Repository, ApplicationManager  
**Key Methods**:
```rust
search(query, limit)                 // Search apps
get_trending(limit)                  # Trending apps
get_featured()                       # Featured apps
get_app(app_id)                      # App details
rate_app(app_id, rating)             # User ratings
```

**Connection Point**:
```rust
// In API/CLI
marketplace.search(query)?
  → Repository.marketplace_api.search()?
```

#### 5.2 Advanced Features Module
**Purpose**: Enterprise features  
**Connected To**: ApplicationManager, VersionManager  

**Components**:
- **AutoUpdateManager**: Automatic updates with scheduling
- **BackupManager**: Snapshot creation and restoration
- **LicenseManager**: License validation and management

**Key Methods**:
```rust
// Auto-update
enable_auto_update(app_id)
check_updates_periodically()
auto_install_updates()

// Backup
create_snapshot(app_id)
list_snapshots(app_id)
restore_snapshot(app_id, snapshot_id)

// License
validate_license(app_id, license_key)
check_expiration(app_id)
```

---

## CONNECTION PATTERNS

### Pattern 1: Sequential Orchestration
Used in `ApplicationManager::load_application()`

```rust
pub async fn load_application(&self, app_id: &AppId, version: &Version) -> Result<()> {
    // 1. Register in lifecycle
    self.lifecycle_manager.register_module(app_id.clone())?;
    
    // 2. Resolve dependencies
    let dependencies = self.module_resolver.resolve_dependencies(app_id, version)?;
    
    // 3. Download
    self.lifecycle_manager.download(app_id).await?;
    
    // 4. Verify
    self.lifecycle_manager.verify(app_id).await?;
    
    // 5. Install
    self.lifecycle_manager.install(app_id).await?;
    
    // 6. Load
    self.lifecycle_manager.load(app_id).await?;
    
    Ok(())
}
```

### Pattern 2: Branching with Fallback
Used in Repository module

```rust
pub async fn fetch_package(&self, app_id: &str) -> Result<Package> {
    // Try GitHub first
    match self.github_fetcher.fetch(app_id).await {
        Ok(pkg) => return Ok(pkg),
        Err(_) => {
            // Fall back to Marketplace
            match self.marketplace.get_app(app_id).await {
                Ok(pkg) => return Ok(pkg),
                Err(_) => {
                    // Finally try local cache
                    self.local_loader.load(app_id).await
                }
            }
        }
    }
}
```

### Pattern 3: Parallel Cross-Module Validation
Used in Security module

```rust
pub async fn verify(&self, app_id: &AppId) -> Result<()> {
    let (sig_result, permission_result, sandbox_result) = tokio::join!(
        self.signature_verifier.verify(app_id),
        self.permission_manager.validate(app_id),
        self.sandbox_manager.validate(app_id),
    );
    
    sig_result?;
    permission_result?;
    sandbox_result?;
    
    self.audit_logger.log_verification(app_id)?;
    Ok(())
}
```

### Pattern 4: State-Driven Callbacks
Used in Module Lifecycle

```rust
pub async fn transition(&self, app_id: &AppId, from: State, to: State) -> Result<()> {
    // Validate transition
    self.validate_transition(from, to)?;
    
    // Execute pre-transition hooks
    self.emit_event("pre_transition", app_id)?;
    
    // Update state
    self.states.insert(app_id.clone(), to);
    
    // Execute post-transition hooks
    self.emit_event("post_transition", app_id)?;
    
    // Record history
    self.record_transition(app_id, from, to)?;
    
    Ok(())
}
```

---

## DATA FLOW EXAMPLE: Complete Install

```
User initiates install via CLI:
  $ omnisystem-app install myapp --version 1.0.0

↓

CLI handler → ApplicationManager::load_application("myapp", "1.0.0")

↓

ApplicationManager orchestrates:
  1. lifecycle_manager.register_module("myapp")
     └─ Creates Discovered state
     
  2. module_resolver.resolve_dependencies("myapp", "1.0.0")
     └─ Queries dependency_graph for deps
     └─ Returns: [dep1@^1.0, dep2@~2.1]
     
  3. lifecycle_manager.download("myapp")
     └─ installer.download() call
       └─ repository.fetch_from_github(url)
         ├─ Tries GitHub API first
         ├─ Falls back to Marketplace
         └─ Falls back to Local cache
     └─ Transitions: Discovered→Downloading→Downloaded
     
  4. lifecycle_manager.verify("myapp")
     └─ security.verify() call
       ├─ signature_verifier.verify_signature()
       ├─ permission_manager.validate()
       └─ sandbox_manager.validate()
     └─ Transitions: Downloaded→Verifying→Verified
     
  5. lifecycle_manager.install("myapp")
     └─ installer.install() executes
     └─ Transitions: Verified→Installing→Installed
     
  6. lifecycle_manager.load("myapp")
     └─ config_manager.load_config("myapp")
     └─ Transitions: Installed→Loading→Loaded
     
  7. Optional: lifecycle_manager.start("myapp")
     └─ Transitions: Loaded→Running
     
  8. security.audit_action("install", "myapp", "cli-user")
     └─ audit_logger.log_event(...)
     
  9. Return success to CLI
     └─ CLI prints "✓ Application installed"

↓

User can now run:
  $ omnisystem-app start myapp
  $ omnisystem-app status myapp
  $ omnisystem-app logs myapp
```

---

## CONCURRENCY & SYNCHRONIZATION

### Lock-Free Design
```rust
// All modules use Arc<DashMap> for thread-safe, lock-free access
pub struct DependencyGraph {
    modules: Arc<DashMap<AppId, ModuleInfo>>,  // O(1) operations
    edges: Arc<DashMap<AppId, Vec<Dependency>>>, // No locks!
}

// Multiple threads can read/write simultaneously
tokio::spawn(async {
    graph.add_module(app_id_1)?;  // Thread 1
});

tokio::spawn(async {
    graph.add_module(app_id_2)?;  // Thread 2
});
// Both execute concurrently without blocking
```

### Async/Await Throughout
```rust
// Every I/O operation is async, enabling high concurrency
pub async fn load_application(&self, app_id: &AppId, version: &Version) -> Result<()> {
    self.lifecycle_manager.download(app_id).await?;      // Async
    self.security.verify(app_id).await?;                  // Async
    self.installer.install(app_id).await?;                // Async
    Ok(())
}

// Can handle thousands of concurrent installs
for app_id in app_ids {
    tokio::spawn(app_mgr.load_application(&app_id, &version));
}
```

---

## ERROR HANDLING & RECOVERY

### Error Propagation
```rust
// Errors propagate through the call chain
ApplicationManager::load_application()
  ├─→ lifecycle_manager.register_module()
  │    └─→ Error: ModuleAlreadyLoaded → propagates up
  ├─→ resolver.resolve_dependencies()
  │    └─→ Error: UnresolvedDependency → propagates up
  └─→ installer.install()
       └─→ Error: InstallationFailed → propagates up
           → rollback_manager.rollback()
           → security.audit_action("install_failed")
           → Return error to user
```

### Automatic Rollback
```rust
// If any step fails, rollback is automatic
if install_failed {
    rollback_manager.rollback(app_id, previous_version)?;
    lifecycle_manager.transition_to_previous_state(app_id)?;
    audit_logger.log_failure("install_failed", app_id)?;
}
```

---

## SUMMARY: CONNECTOR ARCHITECTURE

The Application Manager uses a **proven hub-and-spoke pattern** where:

1. **Single Hub**: ApplicationManager centralizes orchestration
2. **Spokes**: 12 specialized modules each handle specific concerns
3. **Async/Await**: All I/O is non-blocking, enabling high concurrency
4. **Lock-Free**: DashMap provides O(1) operations without global locks
5. **Strong Typing**: All connections validated at compile time
6. **Error Recovery**: Automatic rollback on failure
7. **Audit Trail**: Every action logged for compliance

**Result**: A scalable, reliable, enterprise-grade application management system where all modules work in perfect coordination through a central orchestrator.

---

**All 12 modules connected. Zero coupling. Maximum cohesion.**
