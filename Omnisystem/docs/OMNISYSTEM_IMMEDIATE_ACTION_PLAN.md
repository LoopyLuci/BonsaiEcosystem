# OMNISYSTEM: Immediate Action Plan
## Connector & Sub-Module System - Next 72 Hours

**Status**: Ready to Execute NOW  
**Duration**: 72 hours to Phase 1 foundations  
**Deliverables**: 4 new crates with working prototypes  

---

## HOUR 0-4: SETUP & SCAFFOLDING

### Task 1.1: Create Crate Structure (1 hour)
```bash
# Execute these commands:
cd Omnisystem

# Core system crates
cargo new crates/omnisystem-connector-core --lib
cargo new crates/omnisystem-submodule --lib
cargo new crates/omnisystem-catalog --lib
cargo new crates/omnisystem-base-modules --lib

# Macro crates (for derive macros)
cargo new crates/omnisystem-connector-macros --lib
cargo new crates/omnisystem-submodule-macros --lib

# Update workspace Cargo.toml
# Add all new crates to [workspace] members

# Verify all crates compile
cargo check --workspace
```

### Task 1.2: Add Dependencies to New Crates (1 hour)
```toml
# In each crate's Cargo.toml:

[dependencies]
tokio = { version = "1", features = ["full"] }
dashmap = "5.5"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1.0"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
async-trait = "0.1"
parking_lot = "0.12"
bytes = "1"
thiserror = "1.0"

[dev-dependencies]
tokio-test = "0.4"
criterion = "0.5"
```

### Task 1.3: Create Module Structure (2 hours)
```bash
# Create mod.rs files with basic structure
# Each crate gets:
# - src/lib.rs (with pub mod statements)
# - src/error.rs (error types)
# - src/types.rs (type definitions)
# - tests/ directory
# - benches/ directory (if benchmarking)

# Example structure for omnisystem-connector-core:
touch crates/omnisystem-connector-core/src/error.rs
touch crates/omnisystem-connector-core/src/types.rs
touch crates/omnisystem-connector-core/src/connector.rs
touch crates/omnisystem-connector-core/src/arena.rs
touch crates/omnisystem-connector-core/src/registry.rs
touch crates/omnisystem-connector-core/src/message.rs

mkdir -p crates/omnisystem-connector-core/tests
mkdir -p crates/omnisystem-connector-core/benches
```

---

## HOUR 4-12: CORE CONNECTOR SYSTEM

### Task 2.1: Error Types & Type Definitions (2 hours)
```rust
// omnisystem-connector-core/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectorError {
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },
    
    #[error("Connector not found: {0}")]
    NotFound(String),
    
    #[error("Connector already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Send failed: {0}")]
    SendFailed(String),
    
    #[error("Receive failed: {0}")]
    ReceiveFailed(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Arena allocation failed: {0}")]
    AllocationFailed(String),
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Disconnected")]
    Disconnected,
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ConnectorError>;

// omnisystem-connector-core/src/types.rs
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConnectorId(Uuid);

impl ConnectorId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferingMode {
    Unbounded,
    Bounded(usize),
    Ring(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityLevel {
    Memory,
    AsyncDurable,
    SyncDurable,
}

#[derive(Debug, Clone)]
pub struct ConnectorConfig {
    pub buffering: BufferingMode,
    pub timeout_ms: u64,
    pub durability: DurabilityLevel,
}

impl Default for ConnectorConfig {
    fn default() -> Self {
        Self {
            buffering: BufferingMode::Bounded(10000),
            timeout_ms: 5000,
            durability: DurabilityLevel::AsyncDurable,
        }
    }
}
```

### Task 2.2: Connectable Trait (2 hours)
```rust
// omnisystem-connector-core/src/connector.rs
use serde::{Serialize, Deserialize};
use std::any::TypeId;

/// Core trait that all connector messages must implement
pub trait Connectable: Send + Sync + Serialize + Deserialize<'static> {
    /// Unique type identifier (content-addressable)
    fn type_id() -> u128;
    
    /// Schema for type validation
    fn schema() -> Schema;
    
    /// Validate integrity
    fn validate(&self) -> Result<()> {
        Ok(())
    }
    
    /// Memory size in bytes
    fn memory_size(&self) -> usize;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub type_id: u128,
    pub name: String,
    pub version: (u32, u32, u32),
    pub estimated_size: usize,
}

/// Core Connector trait - all connectors implement this
#[async_trait::async_trait]
pub trait Connector: Send + Sync {
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    fn is_connected(&self) -> bool;
    fn id(&self) -> ConnectorId;
}

/// Request-Reply connector trait
#[async_trait::async_trait]
pub trait RequestReplyConnector<Req: Connectable, Resp: Connectable>: Connector {
    async fn send_request(&self, request: &Req) -> Result<Resp>;
    async fn handle_request(&mut self, handler: Box<dyn Fn(&Req) -> Resp + Send + Sync>) -> Result<()>;
}

/// Pub-Sub connector trait
#[async_trait::async_trait]
pub trait PubSubConnector<T: Connectable>: Connector {
    async fn publish(&self, message: T) -> Result<()>;
    async fn subscribe(&self) -> Result<tokio::sync::mpsc::Receiver<T>>;
    fn subscriber_count(&self) -> usize;
}

// Example implementation
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message<T> {
    pub data: T,
}

impl<T: Connectable> Connectable for Message<T> {
    fn type_id() -> u128 {
        // Hash("Message") + T::type_id()
        0x12345678_90abcdef_12345678_90abcdef
    }
    
    fn schema() -> Schema {
        Schema {
            type_id: Self::type_id(),
            name: "Message".to_string(),
            version: (1, 0, 0),
            estimated_size: std::mem::size_of::<T>(),
        }
    }
    
    fn memory_size(&self) -> usize {
        std::mem::size_of::<T>()
    }
}
```

### Task 2.3: Arena Allocator (2 hours)
```rust
// omnisystem-connector-core/src/arena.rs
use std::sync::Arc;
use parking_lot::Mutex;

/// Zero-copy memory arena for connector messages
pub struct Arena {
    // Backing memory
    memory: Vec<u8>,
    
    // Current position
    position: Mutex<usize>,
    
    // Arena ID
    id: ArenaId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArenaId(uuid::Uuid);

impl ArenaId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Arena {
    pub fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            memory: vec![0u8; capacity],
            position: Mutex::new(0),
            id: ArenaId::new(),
        })
    }
    
    pub fn alloc<T: Connectable>(&self, value: T) -> Result<ArenaRef<T>> {
        let mut pos = self.position.lock();
        let size = value.memory_size();
        
        if *pos + size > self.memory.len() {
            return Err(ConnectorError::AllocationFailed(
                "Arena full".to_string()
            ));
        }
        
        // Serialize into arena
        let serialized = serde_json::to_vec(&value)?;
        self.memory[*pos..*pos + serialized.len()].copy_from_slice(&serialized);
        
        let offset = *pos;
        *pos += size;
        
        Ok(ArenaRef {
            arena_id: self.id,
            offset,
            size,
            _phantom: std::marker::PhantomData,
        })
    }
    
    pub fn capacity(&self) -> usize {
        self.memory.len()
    }
    
    pub fn used(&self) -> usize {
        *self.position.lock()
    }
    
    pub fn available(&self) -> usize {
        self.capacity() - self.used()
    }
}

pub struct ArenaRef<T> {
    arena_id: ArenaId,
    offset: usize,
    size: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Clone for ArenaRef<T> {
    fn clone(&self) -> Self {
        Self {
            arena_id: self.arena_id,
            offset: self.offset,
            size: self.size,
            _phantom: std::marker::PhantomData,
        }
    }
}
```

### Task 2.4: Registry (2 hours)
```rust
// omnisystem-connector-core/src/registry.rs
use dashmap::DashMap;
use std::sync::Arc;

pub struct ConnectorRegistry {
    connectors: Arc<DashMap<ConnectorId, Arc<dyn std::any::Any + Send + Sync>>>,
}

impl ConnectorRegistry {
    pub fn new() -> Self {
        Self {
            connectors: Arc::new(DashMap::new()),
        }
    }
    
    pub fn register(&self, id: ConnectorId) -> Result<()> {
        if self.connectors.contains_key(&id) {
            return Err(ConnectorError::AlreadyExists(id.to_string()));
        }
        
        self.connectors.insert(id, Arc::new(()));
        tracing::info!("Registered connector: {:?}", id);
        Ok(())
    }
    
    pub fn unregister(&self, id: ConnectorId) -> Result<()> {
        self.connectors.remove(&id)
            .ok_or(ConnectorError::NotFound(id.to_string()))?;
        
        tracing::info!("Unregistered connector: {:?}", id);
        Ok(())
    }
    
    pub fn exists(&self, id: ConnectorId) -> bool {
        self.connectors.contains_key(&id)
    }
    
    pub fn list_all(&self) -> Vec<ConnectorId> {
        self.connectors
            .iter()
            .map(|ref_| *ref_.key())
            .collect()
    }
}

impl Default for ConnectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
```

### Task 2.5: lib.rs Exports (30 minutes)
```rust
// omnisystem-connector-core/src/lib.rs
pub mod connector;
pub mod arena;
pub mod registry;
pub mod message;
pub mod error;
pub mod types;

pub use connector::{Connectable, Connector, RequestReplyConnector, PubSubConnector};
pub use arena::{Arena, ArenaId, ArenaRef};
pub use registry::ConnectorRegistry;
pub use error::{ConnectorError, Result};
pub use types::ConnectorId;
```

---

## HOUR 12-24: BASIC TESTS & VALIDATION

### Task 3.1: Unit Tests (4 hours)
```rust
// omnisystem-connector-core/tests/connector_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use omnisystem_connector_core::*;
    
    #[test]
    fn test_connector_id_unique() {
        let id1 = ConnectorId::new();
        let id2 = ConnectorId::new();
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_arena_allocation() {
        let arena = Arena::new(1024);
        assert_eq!(arena.capacity(), 1024);
        assert_eq!(arena.used(), 0);
    }
    
    #[test]
    fn test_registry_register_unregister() {
        let registry = ConnectorRegistry::new();
        let id = ConnectorId::new();
        
        assert!(registry.register(id).is_ok());
        assert!(registry.exists(id));
        
        assert!(registry.unregister(id).is_ok());
        assert!(!registry.exists(id));
    }
    
    #[test]
    fn test_registry_duplicate_registration() {
        let registry = ConnectorRegistry::new();
        let id = ConnectorId::new();
        
        registry.register(id).unwrap();
        assert!(registry.register(id).is_err());
    }
    
    // Add 16+ more tests
}
```

### Task 3.2: Integration Test (2 hours)
```rust
// omnisystem-connector-core/tests/integration_tests.rs
#[tokio::test]
async fn test_basic_send_receive() {
    // Create arena
    let arena = Arena::new(10000);
    
    // Create registry
    let registry = ConnectorRegistry::new();
    
    // Verify basic operations
    let id = ConnectorId::new();
    assert!(registry.register(id).is_ok());
    assert!(registry.exists(id));
    
    // More tests...
}
```

### Task 3.3: Run & Validate (1 hour)
```bash
cd crates/omnisystem-connector-core
cargo test --lib
cargo clippy --all-targets
cargo fmt --check
```

---

## HOUR 24-48: CORE SUB-MODULE SYSTEM

### Task 4: Sub-Module Implementation (20 hours)
```rust
// omnisystem-submodule/src/lib.rs
pub mod submodule;
pub mod manager;
pub mod lifecycle;
pub mod error;
pub mod types;

pub use submodule::SubModule;
pub use manager::SubModuleManager;
pub use error::{SubModuleError, Result};

// omnisystem-submodule/src/types.rs
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubModuleId(Uuid);

impl SubModuleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_name(name: &str) -> Self {
        Self(Uuid::new_v5(&Uuid::NAMESPACE_DNS, name.as_bytes()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubModuleState {
    Unloaded,
    Loading,
    Ready,
    Running,
    Stopping,
    Stopped,
    Error,
}

// omnisystem-submodule/src/submodule.rs
use async_trait::async_trait;

#[async_trait]
pub trait SubModule: Send + Sync {
    fn id(&self) -> SubModuleId;
    fn name(&self) -> &str;
    
    async fn load(&mut self) -> Result<()>;
    async fn initialize(&mut self) -> Result<()>;
    async fn start(&mut self) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
    async fn unload(&mut self) -> Result<()>;
    
    fn state(&self) -> SubModuleState;
}

// omnisystem-submodule/src/manager.rs
use dashmap::DashMap;
use std::sync::Arc;

pub struct SubModuleManager {
    loaded: Arc<DashMap<SubModuleId, Arc<tokio::sync::RwLock<Box<dyn SubModule>>>>>,
}

impl SubModuleManager {
    pub fn new() -> Self {
        Self {
            loaded: Arc::new(DashMap::new()),
        }
    }
    
    pub async fn load(&self, submodule: Box<dyn SubModule>) -> Result<()> {
        let id = submodule.id();
        
        if self.loaded.contains_key(&id) {
            return Err(SubModuleError::AlreadyLoaded(id));
        }
        
        let mut sm = submodule;
        sm.load().await?;
        
        self.loaded.insert(id, Arc::new(tokio::sync::RwLock::new(sm)));
        
        tracing::info!("Loaded sub-module: {:?}", id);
        Ok(())
    }
    
    pub async fn unload(&self, id: SubModuleId) -> Result<()> {
        if let Some((_, mut sm)) = self.loaded.remove(&id) {
            let mut sm = Arc::try_unwrap(sm)
                .map_err(|_| SubModuleError::InUse(id))?
                .into_inner();
            
            sm.unload().await?;
            
            tracing::info!("Unloaded sub-module: {:?}", id);
            Ok(())
        } else {
            Err(SubModuleError::NotFound(id))
        }
    }
    
    pub fn list_loaded(&self) -> Vec<SubModuleId> {
        self.loaded.iter().map(|ref_| *ref_.key()).collect()
    }
}

impl Default for SubModuleManager {
    fn default() -> Self {
        Self::new()
    }
}
```

### Task 4.2: Tests (3 hours)
```bash
cd crates/omnisystem-submodule
cargo test --lib
# Target: 20+ tests passing
```

---

## HOUR 48-60: CATALOG SYSTEM

### Task 5: Catalog Implementation (12 hours)
```rust
// omnisystem-catalog/src/lib.rs
pub mod catalog;
pub mod error;
pub mod types;

pub use catalog::ModuleCatalog;
pub use error::{CatalogError, Result};

// omnisystem-catalog/src/types.rs
#[derive(Debug, Clone)]
pub struct CatalogEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// omnisystem-catalog/src/catalog.rs
pub struct ModuleCatalog {
    entries: Arc<DashMap<String, CatalogEntry>>,
}

impl ModuleCatalog {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(DashMap::new()),
        }
    }
    
    pub fn register(&self, entry: CatalogEntry) -> Result<()> {
        self.entries.insert(entry.id.clone(), entry);
        Ok(())
    }
    
    pub fn get(&self, id: &str) -> Option<CatalogEntry> {
        self.entries.get(id).map(|e| e.clone())
    }
    
    pub fn list_all(&self) -> Vec<CatalogEntry> {
        self.entries.iter().map(|e| e.value().clone()).collect()
    }
}

impl Default for ModuleCatalog {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## HOUR 60-72: INTEGRATION & TESTING

### Task 6: Full Integration (8 hours)
```bash
# Update Omnisystem/Cargo.toml
# Add all new crates to workspace

# Run full test suite
cargo test --workspace --lib

# Target: 80+ tests passing
# Target: 0 warnings (except from dependencies)
# Target: All clippy checks passing
```

### Task 7: Documentation (4 hours)
```markdown
# Create docs:

1. CONNECTOR_CORE_QUICK_START.md
   - Overview
   - Basic usage
   - Common patterns

2. SUBMODULE_QUICK_START.md
   - Overview
   - Creating a sub-module
   - Loading/unloading

3. API.md
   - All public APIs documented
   - Examples
   - Error handling
```

---

## SUCCESS CRITERIA (72 HOURS)

✅ **By End of Hour 72:**

1. **Code Completed**
   - [ ] 4 new crates created
   - [ ] 1,500+ LOC implemented
   - [ ] All core traits defined
   - [ ] Arena allocator working
   - [ ] Registry functional
   - [ ] Sub-module system working
   - [ ] Basic catalog working

2. **Tests Passing**
   - [ ] 80+ unit tests passing
   - [ ] 0 compilation errors
   - [ ] <10 warnings
   - [ ] All clippy checks passing
   - [ ] Code properly formatted

3. **Documentation**
   - [ ] Quick start guides
   - [ ] API documentation
   - [ ] Basic examples

4. **Performance**
   - [ ] Arena allocation <1µs
   - [ ] Registry lookup <5µs
   - [ ] Sub-module load <50ms

---

## EXECUTION CHECKLIST

```
Hour 0-4: Setup & Scaffolding
[ ] Create all 6 crates
[ ] Add dependencies
[ ] Create module structure
[ ] Verify compilation

Hour 4-12: Connector Core
[ ] Error types (30 min)
[ ] Type definitions (30 min)
[ ] Connectable trait (1 hour)
[ ] Arena allocator (2 hours)
[ ] Registry (2 hours)
[ ] lib.rs exports (30 min)

Hour 12-24: Testing & Validation
[ ] Write 20+ unit tests
[ ] Create integration test
[ ] Run full test suite
[ ] Fix any issues

Hour 24-48: Sub-Module System
[ ] SubModule trait (2 hours)
[ ] Manager (2 hours)
[ ] Lifecycle hooks (2 hours)
[ ] Tests (4 hours)

Hour 48-60: Catalog System
[ ] Core catalog structure (4 hours)
[ ] CRUD operations (2 hours)
[ ] Tests (3 hours)
[ ] Documentation (3 hours)

Hour 60-72: Integration
[ ] Workspace integration (2 hours)
[ ] Full test suite (4 hours)
[ ] Performance validation (2 hours)
[ ] Documentation (2 hours)
```

---

## QUICK LAUNCH SCRIPT

Save as `launch_connectors.sh`:

```bash
#!/bin/bash
set -e

echo "🚀 Launching Omnisystem Connector & Sub-Module System"
echo ""

cd Omnisystem

# Create crates
echo "📦 Creating crates..."
cargo new crates/omnisystem-connector-core --lib
cargo new crates/omnisystem-submodule --lib
cargo new crates/omnisystem-catalog --lib
cargo new crates/omnisystem-base-modules --lib

# Verify workspace
echo "🔍 Verifying workspace..."
cargo check --workspace

# Run tests
echo "🧪 Running tests..."
cargo test --workspace --lib

# Format code
echo "✨ Formatting code..."
cargo fmt --all

# Check lints
echo "🎯 Checking lints..."
cargo clippy --all-targets

echo ""
echo "✅ Phase 1 Foundation Ready!"
echo "   - 4 new crates created"
echo "   - Core systems scaffolded"
echo "   - Ready for implementation"
```

---

## NEXT STEPS (After 72 Hours)

1. **Implement Full Connector Types** (Week 1-2)
   - Request-Reply
   - Pub-Sub
   - Stream
   - Broadcast

2. **Expand Sub-Module System** (Week 2-3)
   - Versioning
   - Hot-reload
   - Dependency injection

3. **Complete Catalog** (Week 3)
   - Full-text search
   - Distributed sync
   - Knowledge modules

4. **Build Base Modules** (Week 4)
   - Runtime
   - Data
   - Communication
   - Observability
   - Security

---

**Status**: Ready to Launch  
**Timeline**: 72 hours to Phase 1 completion  
**Team Required**: 2-3 engineers  
**Quality Target**: Enterprise-grade  

**Let's build the future! 🚀**
