//! Integration tests for Omni-Bot API
//! Tests all 16 REST endpoints across Environments and Modules

use omni_bot_api::models::*;
use omni_bot_api::handlers::{init_store, init_registry};
use omni_bot_api::handlers::{
    create_environment, delete_environment, execute_command, get_environment_status,
    list_environments, migrate_environment, restore_environment, snapshot_environment,
    start_environment, stop_environment,
};
use omni_bot_api::handlers::{
    get_module_info, get_operation_progress, install_module, search_modules, uninstall_module,
    update_module, verify_module_signature,
};
use axum::extract::{Path, Query, State};
use axum::Json;
use std::collections::HashMap;

// ============================================================================
// ENVIRONMENT TESTS
// ============================================================================

#[tokio::test]
async fn test_create_environment_success() {
    let store = init_store();

    let spec = EnvironmentSpec {
        id: "test-env-1".to_string(),
        name: "Test Environment".to_string(),
        env_type: "container".to_string(),
        resources: ResourceAllocation {
            memory_mb: 1024,
            cpu_cores: 4,
            cpu_percent_max: 100,
            disk_mb: 5120,
            iops_limit: 2000,
            bandwidth_mbps: 200,
        },
        env_vars: vec![("DEBUG".to_string(), "true".to_string())]
            .into_iter()
            .collect(),
        modules: None,
        snapshot_id: None,
        metadata: None,
        tags: Some(vec!["test".to_string(), "demo".to_string()]),
    };

    let result = create_environment(State(store), Json(spec)).await;
    assert!(result.is_ok());
    let (status, env) = result.unwrap();
    assert_eq!(status.as_u16(), 201);
    assert_eq!(env.state, EnvironmentState::Created);
}

#[tokio::test]
async fn test_list_environments_with_filtering() {
    let store = init_store();
    let mut envs = store.write().await;

    for i in 1..=3 {
        let env = EnvironmentInfo {
            id: format!("env-{}", i),
            name: format!("Environment {}", i),
            env_type: "container".to_string(),
            state: if i % 2 == 0 {
                EnvironmentState::Running
            } else {
                EnvironmentState::Stopped
            },
            resources: ResourceAllocation::default(),
            resource_usage: ResourceUsage::default(),
            env_vars: HashMap::new(),
            modules: Vec::new(),
            snapshots: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            last_started: None,
            tags: Some(vec!["production".to_string()]),
            metadata: None,
        };
        envs.insert(format!("env-{}", i), env);
    }
    drop(envs);

    let query = crate::handlers::environments::ListEnvironmentsQuery {
        state: Some("running".to_string()),
        env_type: None,
        tags: None,
        page: None,
        per_page: None,
        sort_by: None,
        sort_order: None,
    };

    let result = list_environments(State(store), Query(query)).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.items.iter().filter(|e| e.state.is_running()).count(), 1);
}

#[tokio::test]
async fn test_start_environment() {
    let store = init_store();
    let mut envs = store.write().await;

    let env = EnvironmentInfo {
        id: "test-start".to_string(),
        name: "Test Start".to_string(),
        env_type: "container".to_string(),
        state: EnvironmentState::Created,
        resources: ResourceAllocation::default(),
        resource_usage: ResourceUsage::default(),
        env_vars: HashMap::new(),
        modules: Vec::new(),
        snapshots: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_started: None,
        tags: None,
        metadata: None,
    };
    envs.insert("test-start".to_string(), env);
    drop(envs);

    let result = start_environment(State(store), Path("test-start".to_string())).await;
    assert!(result.is_ok());
    let env = result.unwrap();
    assert_eq!(env.state, EnvironmentState::Starting);
}

#[tokio::test]
async fn test_stop_environment() {
    let store = init_store();
    let mut envs = store.write().await;

    let env = EnvironmentInfo {
        id: "test-stop".to_string(),
        name: "Test Stop".to_string(),
        env_type: "container".to_string(),
        state: EnvironmentState::Running,
        resources: ResourceAllocation::default(),
        resource_usage: ResourceUsage::default(),
        env_vars: HashMap::new(),
        modules: Vec::new(),
        snapshots: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_started: Some(chrono::Utc::now()),
        tags: None,
        metadata: None,
    };
    envs.insert("test-stop".to_string(), env);
    drop(envs);

    let result = stop_environment(State(store), Path("test-stop".to_string())).await;
    assert!(result.is_ok());
    let env = result.unwrap();
    assert_eq!(env.state, EnvironmentState::Stopping);
}

#[tokio::test]
async fn test_snapshot_environment() {
    let store = init_store();
    let mut envs = store.write().await;

    let env = EnvironmentInfo {
        id: "test-snapshot".to_string(),
        name: "Test Snapshot".to_string(),
        env_type: "container".to_string(),
        state: EnvironmentState::Running,
        resources: ResourceAllocation::default(),
        resource_usage: ResourceUsage {
            cpu_percent: 25.0,
            memory_mb: 512,
            disk_mb: 2048,
            bandwidth_mbps: 50.0,
        },
        env_vars: HashMap::new(),
        modules: Vec::new(),
        snapshots: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_started: Some(chrono::Utc::now()),
        tags: None,
        metadata: None,
    };
    envs.insert("test-snapshot".to_string(), env);
    drop(envs);

    let result = snapshot_environment(State(store), Path("test-snapshot".to_string())).await;
    assert!(result.is_ok());
    let snapshot = result.unwrap();
    assert!(snapshot.id.contains("snap"));
}

#[tokio::test]
async fn test_restore_environment() {
    let store = init_store();
    let mut envs = store.write().await;

    let snapshot = SnapshotInfo {
        id: "snap-001".to_string(),
        created_at: chrono::Utc::now(),
        size_mb: 2048,
        description: Some("Test snapshot".to_string()),
        cas_hash: "blake3-abc123".to_string(),
    };

    let mut env = EnvironmentInfo {
        id: "test-restore".to_string(),
        name: "Test Restore".to_string(),
        env_type: "container".to_string(),
        state: EnvironmentState::Running,
        resources: ResourceAllocation::default(),
        resource_usage: ResourceUsage::default(),
        env_vars: HashMap::new(),
        modules: Vec::new(),
        snapshots: vec![snapshot],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_started: Some(chrono::Utc::now()),
        tags: None,
        metadata: None,
    };
    envs.insert("test-restore".to_string(), env);
    drop(envs);

    let req = RestoreSnapshotRequest {
        snapshot_id: "snap-001".to_string(),
    };

    let result =
        restore_environment(State(store), Path("test-restore".to_string()), Json(req)).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "started");
}

#[tokio::test]
async fn test_migrate_environment() {
    let store = init_store();
    let mut envs = store.write().await;

    let env = EnvironmentInfo {
        id: "test-migrate".to_string(),
        name: "Test Migrate".to_string(),
        env_type: "container".to_string(),
        state: EnvironmentState::Running,
        resources: ResourceAllocation::default(),
        resource_usage: ResourceUsage::default(),
        env_vars: HashMap::new(),
        modules: Vec::new(),
        snapshots: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_started: Some(chrono::Utc::now()),
        tags: None,
        metadata: None,
    };
    envs.insert("test-migrate".to_string(), env);
    drop(envs);

    let req = MigrationRequest {
        destination: "us-east-1".to_string(),
        keep_source: Some(false),
        options: None,
    };

    let result =
        migrate_environment(State(store), Path("test-migrate".to_string()), Json(req)).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.progress_url.is_some());
}

#[tokio::test]
async fn test_delete_environment() {
    let store = init_store();
    let mut envs = store.write().await;

    let env = EnvironmentInfo {
        id: "test-delete".to_string(),
        name: "Test Delete".to_string(),
        env_type: "container".to_string(),
        state: EnvironmentState::Stopped,
        resources: ResourceAllocation::default(),
        resource_usage: ResourceUsage::default(),
        env_vars: HashMap::new(),
        modules: Vec::new(),
        snapshots: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_started: None,
        tags: None,
        metadata: None,
    };
    envs.insert("test-delete".to_string(), env);
    drop(envs);

    let result = delete_environment(State(store), Path("test-delete".to_string())).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_u16(), 204);
}

#[tokio::test]
async fn test_get_environment_status() {
    let store = init_store();
    let mut envs = store.write().await;

    let env = EnvironmentInfo {
        id: "test-status".to_string(),
        name: "Test Status".to_string(),
        env_type: "container".to_string(),
        state: EnvironmentState::Running,
        resources: ResourceAllocation::default(),
        resource_usage: ResourceUsage {
            cpu_percent: 42.5,
            memory_mb: 256,
            disk_mb: 1024,
            bandwidth_mbps: 25.0,
        },
        env_vars: HashMap::new(),
        modules: Vec::new(),
        snapshots: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_started: Some(chrono::Utc::now()),
        tags: None,
        metadata: None,
    };
    envs.insert("test-status".to_string(), env);
    drop(envs);

    let result = get_environment_status(State(store), Path("test-status".to_string())).await;
    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.state, EnvironmentState::Running);
}

#[tokio::test]
async fn test_execute_command() {
    let store = init_store();
    let mut envs = store.write().await;

    let env = EnvironmentInfo {
        id: "test-exec".to_string(),
        name: "Test Exec".to_string(),
        env_type: "container".to_string(),
        state: EnvironmentState::Running,
        resources: ResourceAllocation::default(),
        resource_usage: ResourceUsage::default(),
        env_vars: HashMap::new(),
        modules: Vec::new(),
        snapshots: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_started: Some(chrono::Utc::now()),
        tags: None,
        metadata: None,
    };
    envs.insert("test-exec".to_string(), env);
    drop(envs);

    let req = ExecuteCommandRequest {
        command: "echo hello".to_string(),
        args: Some(vec!["arg1".to_string()]),
        working_dir: Some("/home".to_string()),
        env: None,
        timeout_seconds: Some(30),
    };

    let result = execute_command(State(store), Path("test-exec".to_string()), Json(req)).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.exit_code, 0);
}

// ============================================================================
// MODULE TESTS
// ============================================================================

#[tokio::test]
async fn test_search_modules() {
    let registry = init_registry();

    let params = ModuleSearchRequest {
        query: None,
        tags: None,
        author: None,
        min_version: None,
        max_version: None,
        page: None,
        per_page: None,
        sort_by: None,
        sort_order: None,
    };

    let result = search_modules(State(registry), Query(params)).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.total, 0);
}

#[tokio::test]
async fn test_install_module() {
    let registry = init_registry();

    let req = ModuleInstallRequest {
        name: "test-module".to_string(),
        version: Some("1.0.0".to_string()),
        force: None,
        verify_signature: Some(true),
        options: None,
    };

    let result = install_module(State(registry), Json(req)).await;
    assert!(result.is_ok());
    let (status, response) = result.unwrap();
    assert_eq!(status.as_u16(), 202);
    assert!(response.task_id.starts_with("install-"));
}

#[tokio::test]
async fn test_uninstall_module_not_found() {
    let registry = init_registry();

    let result = uninstall_module(State(registry), Path("nonexistent".to_string())).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_module_not_found() {
    let registry = init_registry();

    let req = ModuleUpdateRequest {
        target_version: Some("2.0.0".to_string()),
        force: None,
        options: None,
    };

    let result =
        update_module(State(registry), Path("nonexistent".to_string()), Json(req)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_verify_module_signature() {
    let registry = init_registry();

    let req = ModuleVerifyRequest {
        name: "test-module".to_string(),
        version: "1.0.0".to_string(),
        signature: "sig-valid-123".to_string(),
        key_id: "key-001".to_string(),
    };

    let result = verify_module_signature(State(registry), Json(req)).await;
    assert!(result.is_ok());
    let verification = result.unwrap();
    assert!(verification.valid);
}

#[tokio::test]
async fn test_get_operation_progress() {
    let result = get_operation_progress(Path("install-test-123".to_string())).await;
    assert!(result.is_ok());
    let progress = result.unwrap();
    assert_eq!(progress.operation_type, "install");
}

// ============================================================================
// EDGE CASES AND ERROR HANDLING
// ============================================================================

#[tokio::test]
async fn test_create_environment_invalid_resources() {
    let store = init_store();

    let spec = EnvironmentSpec {
        id: "bad-env".to_string(),
        name: "Bad Environment".to_string(),
        env_type: "container".to_string(),
        resources: ResourceAllocation {
            memory_mb: 0, // Invalid
            cpu_cores: 2,
            cpu_percent_max: 100,
            disk_mb: 1024,
            iops_limit: 1000,
            bandwidth_mbps: 100,
        },
        env_vars: HashMap::new(),
        modules: None,
        snapshot_id: None,
        metadata: None,
        tags: None,
    };

    let result = create_environment(State(store), Json(spec)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_start_environment_invalid_state() {
    let store = init_store();
    let mut envs = store.write().await;

    let env = EnvironmentInfo {
        id: "running-env".to_string(),
        name: "Running".to_string(),
        env_type: "container".to_string(),
        state: EnvironmentState::Running, // Already running
        resources: ResourceAllocation::default(),
        resource_usage: ResourceUsage::default(),
        env_vars: HashMap::new(),
        modules: Vec::new(),
        snapshots: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_started: Some(chrono::Utc::now()),
        tags: None,
        metadata: None,
    };
    envs.insert("running-env".to_string(), env);
    drop(envs);

    let result = start_environment(State(store), Path("running-env".to_string())).await;
    assert!(result.is_err());
}
