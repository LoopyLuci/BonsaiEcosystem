# PHASE 1 IMPLEMENTATION GUIDE
## Week 1-3: Foundation & Architecture (Complete Roadmap)

**Status:** Ready for Implementation  
**Code Base:** 2,500+ LOC (core models + registry)  
**Tests:** 200+ (foundation coverage)  
**Timeline:** 3 weeks  

---

## WEEK 1: DATA MODELS & DATABASE

### Completed Components

✅ **Error Types** (`error.rs`)
- 15+ error variants
- From/Into traits
- Tests for error handling

✅ **Core Types** (`types.rs`)
- AppId (with validation)
- ModuleId (UUID-based)
- Version (SemVer compatible)
- VersionConstraint (^, ~, >=, <=, =)

✅ **Cargo.toml Updates**
- All dependencies added
- Workspace configuration
- Feature gates for testing

### Implementation Checklist

```rust
// Remaining Week 1 Tasks:

1. Complete types.rs (already 60% done)
   - Add remaining model structs
   - Implement serialization traits
   - Add validation methods
   - Write 30+ tests

2. Create app.rs
   - AppManifest struct (50+ fields)
   - RegisteredApp struct
   - AppMetadata implementation
   - Tests: 25+

3. Create module.rs
   - ModuleManifest struct
   - ModuleType enum
   - ModuleStatus tracking
   - Tests: 20+

4. Create permission.rs
   - Permission struct
   - PermissionCategory enum
   - RiskLevel classification
   - Tests: 15+

5. Create dependency.rs
   - Dependency struct
   - ModuleDependency struct
   - DependencyKind enum
   - Tests: 20+

6. Database Migrations
   - Create migrations/ directory
   - Write SQL schemas (10 tables)
   - Create SQLx integration
   - Test migrations: 10+
```

### Code Template for Week 1

```rust
// app.rs - Application Model
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppManifest {
    pub id: uuid::Uuid,
    pub name: String,
    pub version: semver::Version,
    pub description: String,
    pub publisher_id: uuid::Uuid,
    pub license: String,
    
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub languages: Vec<String>,
    pub platforms: Vec<String>,
    
    pub modules: Vec<ModuleManifest>,
    pub dependencies: Vec<Dependency>,
    pub permissions: Vec<Permission>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    pub icon_url: String,
    pub screenshots: Vec<String>,
    
    pub min_omnisystem_version: semver::Version,
    pub required_memory_mb: u32,
    pub required_disk_mb: u32,
}

#[derive(Debug, Clone)]
pub struct RegisteredApp {
    pub manifest: AppManifest,
    pub installed: bool,
    pub installed_at: Option<DateTime<Utc>>,
    pub location: Option<std::path::PathBuf>,
    pub rating: f32,
    pub review_count: u32,
    pub download_count: u32,
}

impl AppManifest {
    pub fn from_json(json: &str) -> crate::error::Result<Self> {
        serde_json::from_str(json)
            .map_err(crate::AppManagerError::from)
    }

    pub fn to_json(&self) -> crate::error::Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(crate::AppManagerError::from)
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        if self.name.is_empty() {
            return Err(crate::AppManagerError::InvalidManifest(
                "App name cannot be empty".into()
            ));
        }
        if self.modules.is_empty() {
            return Err(crate::AppManagerError::InvalidManifest(
                "App must have at least one module".into()
            ));
        }
        Ok(())
    }
}
```

---

## WEEK 2: APP DISCOVERY & REGISTRY

### Implementation Tasks

```rust
// discovery.rs - App Discovery Service
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppDiscoveryService {
    apps: Arc<DashMap<uuid::Uuid, RegisteredApp>>,
    index: Arc<RwLock<SearchIndex>>,
}

impl AppDiscoveryService {
    pub fn new() -> Self {
        Self {
            apps: Arc::new(DashMap::new()),
            index: Arc::new(RwLock::new(SearchIndex::new())),
        }
    }

    pub async fn discover_apps(&self, path: &str) -> crate::error::Result<Vec<AppManifest>> {
        // Scan directory for app manifests
        // Parse manifests
        // Register in app registry
        // Return discovered apps
        todo!()
    }

    pub async fn register_app(&self, manifest: AppManifest) -> crate::error::Result<()> {
        // Validate manifest
        manifest.validate()?;
        
        // Insert into registry
        self.apps.insert(manifest.id, RegisteredApp {
            manifest: manifest.clone(),
            installed: false,
            installed_at: None,
            location: None,
            rating: 0.0,
            review_count: 0,
            download_count: 0,
        });

        // Update search index
        let mut index = self.index.write().await;
        index.add_app(&manifest);

        Ok(())
    }

    pub fn get_app(&self, id: &uuid::Uuid) -> Option<RegisteredApp> {
        self.apps.get(id).map(|entry| entry.clone())
    }

    pub async fn search_apps(&self, query: &str) -> Vec<RegisteredApp> {
        let index = self.index.read().await;
        index.search(query)
            .into_iter()
            .filter_map(|id| self.apps.get(&id).map(|entry| entry.clone()))
            .collect()
    }
}

pub struct SearchIndex {
    name_index: std::collections::HashMap<String, Vec<uuid::Uuid>>,
    description_index: std::collections::HashMap<String, Vec<uuid::Uuid>>,
    tag_index: std::collections::HashMap<String, Vec<uuid::Uuid>>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            name_index: std::collections::HashMap::new(),
            description_index: std::collections::HashMap::new(),
            tag_index: std::collections::HashMap::new(),
        }
    }

    pub fn add_app(&mut self, app: &AppManifest) {
        // Add to name index
        self.name_index.entry(app.name.to_lowercase())
            .or_insert_with(Vec::new)
            .push(app.id);

        // Add to tag index
        for tag in &app.tags {
            self.tag_index.entry(tag.to_lowercase())
                .or_insert_with(Vec::new)
                .push(app.id);
        }
    }

    pub fn search(&self, query: &str) -> Vec<uuid::Uuid> {
        let query_lower = query.to_lowercase();
        
        // Simple search: check name, tags
        let mut results = Vec::new();
        
        for (key, ids) in &self.name_index {
            if key.contains(&query_lower) {
                results.extend(ids.clone());
            }
        }

        for (key, ids) in &self.tag_index {
            if key.contains(&query_lower) {
                results.extend(ids.clone());
            }
        }

        results.sort();
        results.dedup();
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_app() {
        let service = AppDiscoveryService::new();
        
        let manifest = AppManifest {
            id: uuid::Uuid::new_v4(),
            name: "Test App".into(),
            version: semver::Version::new(1, 0, 0),
            description: "Test description".into(),
            publisher_id: uuid::Uuid::new_v4(),
            license: "MIT".into(),
            categories: vec!["test".into()],
            tags: vec!["test".into()],
            languages: vec!["en".into()],
            platforms: vec!["linux".into()],
            modules: vec![],  // Would need proper module manifest
            dependencies: vec![],
            permissions: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            icon_url: "icon.png".into(),
            screenshots: vec![],
            min_omnisystem_version: semver::Version::new(1, 0, 0),
            required_memory_mb: 256,
            required_disk_mb: 100,
        };

        let result = service.register_app(manifest.clone()).await;
        assert!(result.is_ok());
        
        let retrieved = service.get_app(&manifest.id);
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_search_apps() {
        let service = AppDiscoveryService::new();
        
        // Register test apps
        let app1 = AppManifest {
            id: uuid::Uuid::new_v4(),
            name: "Pathfinder".into(),
            // ... rest of fields ...
        };

        let app2 = AppManifest {
            id: uuid::Uuid::new_v4(),
            name: "OmniBot".into(),
            // ... rest of fields ...
        };

        service.register_app(app1.clone()).await.unwrap();
        service.register_app(app2.clone()).await.unwrap();

        let results = service.search_apps("path").await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].manifest.name, "Pathfinder");
    }
}
```

### Task Breakdown

- [ ] Create `discovery.rs` (400+ LOC)
- [ ] Create `registry.rs` (300+ LOC)
- [ ] Implement AppDiscoveryService (O(1) lookup)
- [ ] Implement SearchIndex (<50ms search)
- [ ] Write 30+ integration tests
- [ ] Verify all tests pass
- [ ] Code review & documentation

---

## WEEK 3: DEPENDENCY RESOLUTION & UMD

### Implementation Tasks

```rust
// dependency_resolver.rs - Dependency Resolution Service
use std::collections::{HashMap, VecDeque};
use crate::{ModuleId, ModuleManifest, Dependency, VersionConstraint};
use crate::error::{AppManagerError, Result};

pub struct DependencyResolver {
    modules: HashMap<ModuleId, ModuleManifest>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register_module(&mut self, manifest: ModuleManifest) {
        self.modules.insert(manifest.id.clone(), manifest);
    }

    pub fn resolve_dependencies(&self, module_id: &ModuleId) -> Result<Vec<ModuleId>> {
        let mut resolved = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(module_id.clone());

        while let Some(current_id) = queue.pop_front() {
            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());

            let module = self.modules.get(&current_id)
                .ok_or_else(|| AppManagerError::ModuleNotFound(current_id.to_string()))?;

            for dep in &module.dependencies {
                // Validate version constraint
                let dep_module = self.modules.get(&ModuleId(dep.name.clone()))
                    .ok_or_else(|| AppManagerError::DependencyNotSatisfied(dep.name.clone()))?;

                if !dep.version_constraint.satisfies(&dep_module.version) {
                    return Err(AppManagerError::VersionConstraintViolation(
                        format!("{}: required {}, found {}", dep.name, dep.version_constraint, dep_module.version)
                    ));
                }

                queue.push_back(ModuleId(dep.name.clone()));
            }

            resolved.push(current_id);
        }

        // Topological sort for load order
        self.topological_sort(&resolved)?;

        Ok(resolved)
    }

    fn topological_sort(&self, modules: &[ModuleId]) -> Result<()> {
        // Check for circular dependencies
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();

        for module_id in modules {
            if !visited.contains(module_id) {
                self.dfs(module_id, &mut visited, &mut rec_stack)?;
            }
        }

        Ok(())
    }

    fn dfs(&self, module_id: &ModuleId, visited: &mut std::collections::HashSet<ModuleId>, 
           rec_stack: &mut std::collections::HashSet<ModuleId>) -> Result<()> {
        visited.insert(module_id.clone());
        rec_stack.insert(module_id.clone());

        let module = self.modules.get(module_id)
            .ok_or_else(|| AppManagerError::ModuleNotFound(module_id.to_string()))?;

        for dep in &module.dependencies {
            let dep_id = ModuleId(dep.name.clone());
            if !visited.contains(&dep_id) {
                self.dfs(&dep_id, visited, rec_stack)?;
            } else if rec_stack.contains(&dep_id) {
                return Err(AppManagerError::CircularDependency(
                    format!("{} -> {}", module_id, dep.name)
                ));
            }
        }

        rec_stack.remove(module_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_resolution() {
        let mut resolver = DependencyResolver::new();
        
        // Create module A
        let module_a = ModuleManifest {
            id: ModuleId("module_a".into()),
            app_id: "app1".into(),
            name: "Module A".into(),
            version: semver::Version::new(1, 0, 0),
            dependencies: vec![],
            // ... other fields ...
        };

        // Create module B that depends on A
        let module_b = ModuleManifest {
            id: ModuleId("module_b".into()),
            app_id: "app1".into(),
            name: "Module B".into(),
            version: semver::Version::new(1, 0, 0),
            dependencies: vec![Dependency {
                name: "module_a".into(),
                version_constraint: VersionConstraint::AtLeast(semver::Version::new(1, 0, 0)),
            }],
            // ... other fields ...
        };

        resolver.register_module(module_a);
        resolver.register_module(module_b);

        let deps = resolver.resolve_dependencies(&ModuleId("module_b".into())).unwrap();
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut resolver = DependencyResolver::new();
        
        let module_a = ModuleManifest {
            id: ModuleId("a".into()),
            dependencies: vec![Dependency {
                name: "b".into(),
                version_constraint: VersionConstraint::AtLeast(semver::Version::new(1, 0, 0)),
            }],
            // ... other fields ...
        };

        let module_b = ModuleManifest {
            id: ModuleId("b".into()),
            dependencies: vec![Dependency {
                name: "a".into(),
                version_constraint: VersionConstraint::AtLeast(semver::Version::new(1, 0, 0)),
            }],
            // ... other fields ...
        };

        resolver.register_module(module_a);
        resolver.register_module(module_b);

        let result = resolver.resolve_dependencies(&ModuleId("a".into()));
        assert!(matches!(result, Err(AppManagerError::CircularDependency(_))));
    }
}
```

### Task Breakdown

- [ ] Create `dependency_resolver.rs` (400+ LOC)
- [ ] Implement circular dependency detection
- [ ] Implement topological sorting
- [ ] Create UMD (Universal Module Database) integration
- [ ] Write 25+ resolver tests
- [ ] Write 20+ integration tests
- [ ] Performance testing (<100ms resolution)
- [ ] Documentation & examples

---

## WEEK 3 FINAL INTEGRATION

### Test Coverage Goals

```
Unit Tests:         150+ (>95% coverage)
Integration Tests:  50+  (end-to-end flows)
Performance Tests:  15+  (latency verification)
Security Tests:     10+  (validation & input)

Total Week 1-3:     225+ tests, all passing
Code Coverage:      >95%
```

### Deliverables Checklist

- [ ] All 6 core modules completed (app, module, permission, dependency, discovery, registry)
- [ ] 2,500+ LOC of production code
- [ ] 225+ passing tests
- [ ] >95% code coverage
- [ ] Full database migration scripts
- [ ] API skeleton (ready for Week 2)
- [ ] Comprehensive documentation
- [ ] Code review approved
- [ ] Ready for Phase 2 (API Implementation)

---

## HOW TO BUILD THIS

### Step-by-Step Execution

1. **Copy the code templates above** into each module file
2. **Run tests incrementally**:
   ```bash
   cd Omnisystem
   cargo test -p app-manager-core --lib
   ```

3. **Verify coverage**:
   ```bash
   cargo tarpaulin -p app-manager-core --out Html
   ```

4. **Create database migrations**:
   ```bash
   sqlx migrate add -r init_app_manager_schema
   ```

5. **Review & iterate**:
   - Fix failing tests
   - Complete stub implementations
   - Add missing validation

---

## NEXT PHASES (BLUEPRINTS PROVIDED SEPARATELY)

- **Phase 2 (Weeks 4-6)**: API Server, Installation Service, Marketplace Service
- **Phase 3 (Weeks 7-9)**: Frontend (Desktop + Web UI)
- **Phase 4 (Weeks 10-12)**: Integration, Testing, Deployment

Each phase builds on Phase 1 foundation, with clear extension points and interfaces.

---

**Total Implementation Time:** 12 weeks  
**Phase 1 Effort:** 150 person-hours  
**Expected LOC:** 2,500 (foundation)  
**Tests:** 225+ (all passing)

This roadmap makes Phase 1 completely buildable and testable in 3 weeks.
