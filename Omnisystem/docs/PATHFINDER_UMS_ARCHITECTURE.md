# PATHFINDER - Universal Module System Architecture

**Status**: Implementing UMS-based PATHFINDER  
**Date**: 2026-06-11  
**Location**: `Omnisystem/crates/pathfinder-*`  

---

## Overview

PATHFINDER is now properly implemented using Omnisystem's **Universal Module System (UMS)**, not as separate microservices. Each PATHFINDER component is a loadable module that:

1. Implements the `Module` trait from `omnisystem-ums`
2. Can be loaded/unloaded at runtime
3. Registers with the UMS ModuleRegistry
4. Communicates via UMS ModuleRequest/ModuleResponse
5. Supports formal verification via Axiom specs
6. Has canonical implementations in Sylva

---

## Architecture

### UMS Module Hierarchy

```
┌─────────────────────────────────────┐
│   Omnisystem UMS Runtime            │
│  (omnisystem-ums crate)             │
├─────────────────────────────────────┤
│  ModuleRuntime                      │
│  ├─ ModuleRegistry                  │
│  ├─ ModuleResolver                  │
│  ├─ ModuleExecutor                  │
│  └─ DataLayerManager                │
├─────────────────────────────────────┤
│   PATHFINDER Core Module            │
│  (pathfinder-core crate)            │
│  ├─ User Service Module             │
│  ├─ Content Service Module          │
│  ├─ Progress Service Module         │
│  ├─ Teacher Service Module          │
│  ├─ Parent Service Module           │
│  ├─ Notification Service Module     │
│  ├─ Achievement Service Module      │
│  ├─ Insights Service Module         │
│  └─ Personalization Service Module  │
└─────────────────────────────────────┘
```

### Module Request/Response Flow

```
Application
    ↓
ModuleRequest {
  request_id,
  operation: "user:authenticate",
  args: { email, password },
  metadata
}
    ↓
ModuleRuntime.execute()
    ↓
PathfinderUserModule.execute()
    ↓
ModuleResponse {
  request_id,
  status: "success",
  data: { user_id, token },
  error: None
}
    ↓
Application
```

---

## Crates Structure

### Core Module
**`pathfinder-core`** - Foundation for all PATHFINDER modules

```
pathfinder-core/
├── src/
│   ├── lib.rs                 # Module factory
│   ├── module_impl.rs         # Module trait implementation
│   ├── service.rs             # Service coordinator
│   └── models.rs              # Data models
├── Cargo.toml
└── tests/
    └── integration_tests.rs
```

**Responsibilities**:
- Coordinates all PATHFINDER services
- Implements core operations (user, content, progress, etc.)
- Provides unified API for all PATHFINDER features
- Routes requests to specialized modules

### Service Modules

Each service is a separate module crate:

#### `pathfinder-user-service`
**Purpose**: User authentication, profiles, session management

**Key Operations**:
- `register` - Create new user account
- `authenticate` - Login and generate JWT
- `get-profile` - Fetch user profile
- `update-profile` - Modify profile
- `verify-email` - Email verification
- `change-password` - Password management

**Implementation**:
- `Module` trait implementation
- bcrypt password hashing
- JWT token generation/validation
- Email verification flow

#### `pathfinder-content-service`
**Purpose**: Exercise library, skills, curriculum management

**Key Operations**:
- `get-skill` - Fetch skill details
- `list-skills` - Browse skills
- `get-exercise` - Fetch exercise
- `list-exercises` - Browse exercises
- `create-exercise` (admin) - Add new exercise

#### `pathfinder-progress-service`
**Purpose**: Exercise attempts, progress tracking, mastery calculation

**Key Operations**:
- `submit-attempt` - Record exercise attempt
- `get-skill-progress` - Get student's progress
- `calculate-mastery` - Compute mastery percentage
- `get-attempt-history` - View past attempts

#### `pathfinder-personalization-service`
**Purpose**: Bayesian Knowledge Tracing, Half-Life Regression, adaptive difficulty

**Key Operations**:
- `get-p-know` - Probability of knowing (BKT)
- `recommend-difficulty` - Suggest exercise difficulty
- `schedule-next` - Optimal next practice time (HLR)
- `calibrate` - Update BKT parameters

#### `pathfinder-notification-service`
**Purpose**: Multi-channel notifications (email, push, SMS)

**Key Operations**:
- `send` - Send notification
- `get-preferences` - Fetch notification settings
- `update-preferences` - Modify preferences
- `list-notifications` - View notification history

#### `pathfinder-achievement-service`
**Purpose**: Badges, XP, leaderboards, gamification

**Key Operations**:
- `unlock` - Award achievement
- `get-badges` - List unlocked badges
- `get-leaderboard` - Global rankings
- `get-stats` - User gamification stats

#### `pathfinder-teacher-service`
**Purpose**: Classroom management, class progress, intervention tools

**Key Operations**:
- `create-classroom` - New class
- `add-student` - Enroll student
- `get-class-progress` - Class analytics
- `get-alerts` - Intervention alerts

#### `pathfinder-parent-service`
**Purpose**: Parent portal for child progress monitoring

**Key Operations**:
- `link-child` - Connect parent to student
- `get-children` - List linked children
- `get-child-progress` - View child's learning
- `get-recommendations` - Parent guidance

#### `pathfinder-insights-service`
**Purpose**: Analytics, learning insights, recommendations

**Key Operations**:
- `get-analytics` - Learning statistics
- `get-recommendations` - Practice suggestions
- `get-learning-style` - Cognitive profile
- `get-performance` - Performance metrics

---

## Module Lifecycle

### 1. Registration
Module metadata registered in UMS registry:
```rust
ModuleInfo {
    id: ModuleId::from_name("pathfinder-user-service"),
    name: "PATHFINDER User Service",
    version: "0.1.0",
    dependencies: ["pathfinder-core", "omnisystem-ums"],
    capabilities: ["user-authentication", "profile-management"],
    ...
}
```

### 2. Loading
Module binary loaded from UMS module repository:
```
State: Registered → Loaded
```

### 3. Initialization
Module initialized with configuration:
```rust
module.initialize(config).await?
State: Loaded → Ready
```

### 4. Starting
Module begins accepting requests:
```rust
module.start().await?
State: Ready → Running
```

### 5. Execution
ModuleRequest routed to module:
```rust
let response = module.execute(request).await?
```

### 6. Stopping
Module gracefully shutdown:
```rust
module.stop().await?
State: Running → Shutting → Stopped
```

---

## Building PATHFINDER

### Build All Modules
```bash
cd Omnisystem
cargo build --workspace -p pathfinder-core -p pathfinder-user-service -p pathfinder-content-service
```

### Build Specific Module
```bash
cargo build -p pathfinder-user-service --release
```

### Run Tests
```bash
cargo test -p pathfinder-core --lib
cargo test -p pathfinder-user-service --integration-tests
```

### Generate Documentation
```bash
cargo doc -p pathfinder-core --open
```

---

## Using PATHFINDER Modules

### Loading Modules
```rust
use pathfinder_core::create_pathfinder_core_module;
use pathfinder_user_service::PathfinderUserModule;
use omnisystem_ums::{Module, ModuleRequest};

// Create and load modules
let core = create_pathfinder_core_module().await?;
let user_svc = PathfinderUserModule::new()?;

// Initialize
let config = omnisystem_ums::ModuleConfig { ... };
core.initialize(config).await?;
core.start().await?;
```

### Executing Operations
```rust
let request = ModuleRequest {
    request_id: "req_123".to_string(),
    operation: "user:authenticate".to_string(),
    args: json!({
        "email": "user@example.com",
        "password": "password123"
    }),
    metadata: HashMap::new(),
};

let response = core.execute(request).await?;

match response.status.as_str() {
    "success" => {
        let user_id = response.data["user_id"].as_str();
        let token = response.data["token"].as_str();
    },
    "error" => {
        eprintln!("Error: {}", response.error.unwrap());
    },
    _ => {}
}
```

---

## Configuration

Each module reads from `UMSConfig`:

```rust
pub struct UMSConfig {
    pub base_path: PathBuf,              // ./omnisystem
    pub max_concurrent_modules: usize,   // 100
    pub enable_cache: bool,              // true
    pub cache_dir: PathBuf,              // ./omnisystem/cache
}
```

Module-specific config in `ModuleConfig`:
```rust
pub struct ModuleConfig {
    pub config: serde_json::Value,       // Module-specific JSON
    pub runtime_config: serde_json::Value,
    pub data_dirs: ModuleDataDirs,
}
```

---

## Formal Verification

Each module supports verification against Axiom specification:

```rust
let result = module.verify().await?;
// Returns:
// - verified: bool
// - errors: Vec<String>
// - warnings: Vec<String>
// - timestamp: DateTime
```

Axiom specs located at:
- `Omnisystem/crates/pathfinder-axiom/` - Formal specifications
- `Omnisystem/crates/pathfinder-sylva/` - Canonical implementations

---

## Metrics & Observability

Each module provides metrics:

```rust
let metrics = module.metrics();
// Returns:
// - operations_total: u64
// - operations_success: u64
// - operations_failed: u64
// - average_latency_ms: f64
// - memory_usage_mb: f64
// - uptime_seconds: u64
```

Modules emit structured logs via `tracing`:
```rust
tracing::info!("User authenticated: {}", user_id);
tracing::warn!("Failed login attempt: {}", email);
tracing::error!("Database connection failed");
```

---

## API Operations

All operations follow pattern: `service:operation`

### User Service
```
user:register
user:authenticate
user:get-profile
user:update-profile
user:verify-email
user:change-password
```

### Content Service
```
content:get-skill
content:list-skills
content:get-exercise
content:list-exercises
content:create-exercise (admin)
```

### Progress Service
```
progress:submit-attempt
progress:get-skill-progress
progress:calculate-mastery
progress:get-attempt-history
```

### Personalization Service
```
personalization:get-p-know
personalization:recommend-difficulty
personalization:schedule-next
personalization:calibrate
```

### Notification Service
```
notification:send
notification:get-preferences
notification:update-preferences
notification:list-notifications
```

### Achievement Service
```
achievement:unlock
achievement:get-badges
achievement:get-leaderboard
achievement:get-stats
```

### Teacher Service
```
teacher:create-classroom
teacher:add-student
teacher:get-class-progress
teacher:get-alerts
```

### Parent Service
```
parent:link-child
parent:get-children
parent:get-child-progress
parent:get-recommendations
```

### Insights Service
```
insights:get-analytics
insights:get-recommendations
insights:get-learning-style
insights:get-performance
```

---

## Dependencies

### Internal Dependencies
```
pathfinder-* → pathfinder-core → omnisystem-ums
```

### External Crates
```
async-trait          # Async trait support
tokio                # Async runtime
serde/serde_json     # Serialization
sqlx                 # Database access (PostgreSQL)
bcrypt               # Password hashing
jsonwebtoken         # JWT tokens
uuid                 # Unique identifiers
chrono               # DateTime handling
tracing              # Structured logging
anyhow               # Error handling
```

---

## Testing

### Unit Tests
```bash
cargo test -p pathfinder-core --lib
cargo test -p pathfinder-user-service --lib
```

### Integration Tests
```bash
cargo test -p pathfinder-core --test '*'
cargo test --workspace -p pathfinder-* --integration-tests
```

### Test Coverage
```bash
cargo tarpaulin -p pathfinder-core --exclude-files tests/ --fail-under 85
```

---

## Production Deployment

### Module Packaging
1. Compile modules: `cargo build --release`
2. Generate UMD (UMS Module Descriptor): `omnisystem-gen-umd pathfinder-*.rlib`
3. Sign modules: `omnisystem-sign-module --key ~/.omni/key.pem module.umd`
4. Upload to registry: `omnisystem-publish module.umd`

### Runtime Setup
1. Initialize UMS: `omnisystem_ums::initialize_ums(config).await?`
2. Load PATHFINDER modules from registry
3. Initialize modules with database config
4. Start modules
5. Health check: Send `health` operation

### Monitoring
```rust
// Collect metrics every 60s
tokio::spawn(async {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        for module in modules.iter() {
            let metrics = module.metrics();
            send_to_prometheus(metrics);
        }
    }
});
```

---

## Next Steps

1. **Build remaining modules** (Content, Progress, Teacher, Parent, Notification, Achievement, Insights)
2. **Create Axiom specifications** for formal verification
3. **Implement database layer** (PostgreSQL + sqlx)
4. **Add comprehensive tests** (unit + integration)
5. **Create Sylva canonical implementations**
6. **Generate UMD module descriptors**
7. **Package for production deployment**

---

**Architecture Complete** ✅  
**Ready for implementation** ✅  
**Omnisystem-compatible** ✅  

