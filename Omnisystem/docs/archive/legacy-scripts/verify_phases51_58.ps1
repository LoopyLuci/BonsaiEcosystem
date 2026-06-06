# Verification script for Phases 51-58 (Operations & Governance Suite)
# Tests 64 new modules + 4 regression modules (68 total)
# All must return "Result: 111"

$ErrorActionPreference = "Stop"

# Test counter
$passed = 0
$failed = 0

# Phase 51: OmniCompliance (8 modules)
$phase51_files = @(
    "titan/omnicompliance/compliance_config.ti",
    "titan/omnicompliance/policy_engine.ti",
    "aether/omnicompliance/audit_collector.ae",
    "aether/omnicompliance/remediation_worker.ae",
    "sylva/omnicompliance/compliance_dashboard.sy",
    "sylva/omnicompliance/evidence_viewer.sy",
    "axiom/omnicompliance/compliance_proofs.ax",
    "tests/test_omnicompliance_complete.ti"
)

# Phase 52: OmniBackup (8 modules)
$phase52_files = @(
    "titan/omnibackup/backup_config.ti",
    "titan/omnibackup/restore_engine.ti",
    "aether/omnibackup/backup_agent.ae",
    "aether/omnibackup/dr_orchestrator.ae",
    "sylva/omnibackup/backup_dashboard.sy",
    "sylva/omnibackup/restore_progress.sy",
    "axiom/omnibackup/backup_proofs.ax",
    "tests/test_omnibackup_complete.ti"
)

# Phase 53: OmniMigrate (8 modules)
$phase53_files = @(
    "titan/omnimigrate/migration_config.ti",
    "titan/omnimigrate/schema_diff.ti",
    "aether/omnimigrate/migration_runner.ae",
    "aether/omnimigrate/traffic_switcher.ae",
    "sylva/omnimigrate/migration_dashboard.sy",
    "sylva/omnimigrate/schema_viewer.sy",
    "axiom/omnimigrate/migration_proofs.ax",
    "tests/test_omnimigrate_complete.ti"
)

# Phase 54: OmniOrchestrate (8 modules)
$phase54_files = @(
    "titan/omniorchestrate/orchestrate_config.ti",
    "titan/omniorchestrate/resource_scheduler.ti",
    "aether/omniorchestrate/service_controller.ae",
    "aether/omniorchestrate/node_manager.ae",
    "sylva/omniorchestrate/cluster_dashboard.sy",
    "sylva/omniorchestrate/pod_viewer.sy",
    "axiom/omniorchestrate/orchestrate_proofs.ax",
    "tests/test_omniorchestrate_complete.ti"
)

# Phase 55: OmniNetwork (8 modules)
$phase55_files = @(
    "titan/omninetwork/network_config.ti",
    "titan/omninetwork/dns_manager.ti",
    "aether/omninetwork/network_controller.ae",
    "aether/omninetwork/load_balancer.ae",
    "sylva/omninetwork/network_topology.sy",
    "sylva/omninetwork/traffic_analyzer.sy",
    "axiom/omninetwork/network_proofs.ax",
    "tests/test_omninetwork_complete.ti"
)

# Phase 56: OmniQueue (8 modules)
$phase56_files = @(
    "titan/omniqueue/queue_config.ti",
    "titan/omniqueue/message_router.ti",
    "aether/omniqueue/queue_broker.ae",
    "aether/omniqueue/consumer_group.ae",
    "sylva/omniqueue/queue_monitor.sy",
    "sylva/omniqueue/message_inspector.sy",
    "axiom/omniqueue/queue_proofs.ax",
    "tests/test_omniqueue_complete.ti"
)

# Phase 57: OmniStream (8 modules)
$phase57_files = @(
    "titan/omnistream/stream_config.ti",
    "titan/omnistream/stream_processor.ti",
    "aether/omnistream/stream_engine.ae",
    "aether/omnistream/windowing_engine.ae",
    "sylva/omnistream/stream_dashboard.sy",
    "sylva/omnistream/event_timeline.sy",
    "axiom/omnistream/stream_proofs.ax",
    "tests/test_omnistream_complete.ti"
)

# Phase 58: OmniVault (8 modules)
$phase58_files = @(
    "titan/omnivault/vault_config.ti",
    "titan/omnivault/crypto_service.ti",
    "aether/omnivault/key_rotator.ae",
    "aether/omnivault/hsm_bridge.ae",
    "sylva/omnivault/key_dashboard.sy",
    "sylva/omnivault/audit_viewer.sy",
    "axiom/omnivault/vault_proofs.ax",
    "tests/test_omnivault_complete.ti"
)

# Regression tests (4 modules)
$regression_files = @(
    "titan/omnigit/git_config.ti",
    "titan/omnicollaborate/collab_config.ti",
    "tests/test_omnigit_complete.ti",
    "tests/test_omnicollaborate_complete.ti"
)

# Combine all files
$all_files = $phase51_files + $phase52_files + $phase53_files + $phase54_files + $phase55_files + $phase56_files + $phase57_files + $phase58_files + $regression_files

Write-Host "Starting verification of Phases 51-58 + Regression tests..."
Write-Host "Total modules to test: $($all_files.Count)"
Write-Host ""

# Test each file
foreach ($file in $all_files) {
    $full_path = Join-Path "z:\Projects\Omnisystem" $file
    
    if (-not (Test-Path $full_path)) {
        Write-Host "❌ MISSING: $file"
        $failed++
        continue
    }
    
    # Execute with bootstrap
    $output = & ".\titan-bootstrap\target\release\titan-bootstrap.exe" $full_path --run 2>&1
    $output_str = $output | Out-String
    
    if ($output_str -match "Result:\s*111") {
        Write-Host "✅ PASS: $file"
        $passed++
    } else {
        Write-Host "❌ FAIL: $file"
        Write-Host "  Output: $output_str"
        $failed++
    }
}

Write-Host ""
Write-Host "=========================================="
Write-Host "Verification Results"
Write-Host "=========================================="
Write-Host "Passed: $passed / $($all_files.Count)"
Write-Host "Failed: $failed / $($all_files.Count)"

if ($failed -eq 0) {
    Write-Host ""
    Write-Host "All 68 modules verified, zero regressions. [OK]"
    exit 0
} else {
    Write-Host ""
    Write-Host "VERIFICATION FAILED: $failed module(s) did not return 111"
    exit 1
}
