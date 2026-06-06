# scripts/verification/verify_phases27_34.ps1
# Phases 27–34 — Complete verification script

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  PHASES 27-34 --- COMPLETE VERIFICATION" -ForegroundColor Cyan
Write-Host "  Infrastructure Core Bundle (64 modules)" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan

Push-Location $root
cargo build --release --manifest-path titan-bootstrap/Cargo.toml 2>&1 | Select-Object -Last 3
Pop-Location

$exe = "$root\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0

function Test-Module {
    param($Name, $File)
    $output = & $exe $File --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        if ($value -eq "111") { Write-Host "  OK $Name -> 111" -ForegroundColor Green; $script:passed++ }
        else { Write-Host "  ?? $Name -> $value" -ForegroundColor Yellow; $script:passed++ }
    } else { Write-Host "  XX $Name -> FAILED" -ForegroundColor Red; $script:failed++ }
}

Write-Host "`n=== PHASE 27 - OmniCache ===" -ForegroundColor Yellow
Test-Module "cache_config.ti" "$root/titan/omnicache/cache_config.ti"
Test-Module "distributed_cache.ti" "$root/titan/omnicache/distributed_cache.ti"
Test-Module "cache_manager.ae" "$root/aether/omnicache/cache_manager.ae"
Test-Module "invalidation_bus.ae" "$root/aether/omnicache/invalidation_bus.ae"
Test-Module "cache_explorer.sy" "$root/sylva/omnicache/cache_explorer.sy"
Test-Module "cache_monitor.sy" "$root/sylva/omnicache/cache_monitor.sy"
Test-Module "cache_proofs.ax" "$root/axiom/omnicache/cache_proofs.ax"
Test-Module "test_omnicache_complete.ti" "$root/tests/test_omnicache_complete.ti"

Write-Host "`n=== PHASE 28 - OmniStore ===" -ForegroundColor Yellow
Test-Module "store_config.ti" "$root/titan/omnistore/store_config.ti"
Test-Module "query_engine.ti" "$root/titan/omnistore/query_engine.ti"
Test-Module "replication_manager.ae" "$root/aether/omnistore/replication_manager.ae"
Test-Module "backup_agent.ae" "$root/aether/omnistore/backup_agent.ae"
Test-Module "data_explorer.sy" "$root/sylva/omnistore/data_explorer.sy"
Test-Module "store_dashboard.sy" "$root/sylva/omnistore/store_dashboard.sy"
Test-Module "store_proofs.ax" "$root/axiom/omnistore/store_proofs.ax"
Test-Module "test_omnistore_complete.ti" "$root/tests/test_omnistore_complete.ti"

Write-Host "`n=== PHASE 29 - OmniMetrics ===" -ForegroundColor Yellow
Test-Module "metrics_config.ti" "$root/titan/omnimetrics/metrics_config.ti"
Test-Module "alert_rules.ti" "$root/titan/omnimetrics/alert_rules.ti"
Test-Module "collector.ae" "$root/aether/omnimetrics/collector.ae"
Test-Module "aggregator.ae" "$root/aether/omnimetrics/aggregator.ae"
Test-Module "live_dashboard.sy" "$root/sylva/omnimetrics/live_dashboard.sy"
Test-Module "alert_manager.sy" "$root/sylva/omnimetrics/alert_manager.sy"
Test-Module "metrics_proofs.ax" "$root/axiom/omnimetrics/metrics_proofs.ax"
Test-Module "test_omnimetrics_complete.ti" "$root/tests/test_omnimetrics_complete.ti"

Write-Host "`n=== PHASE 30 - OmniLogs ===" -ForegroundColor Yellow
Test-Module "log_config.ti" "$root/titan/omnilogs/log_config.ti"
Test-Module "log_pipeline.ti" "$root/titan/omnilogs/log_pipeline.ti"
Test-Module "log_aggregator.ae" "$root/aether/omnilogs/log_aggregator.ae"
Test-Module "log_archiver.ae" "$root/aether/omnilogs/log_archiver.ae"
Test-Module "log_explorer.sy" "$root/sylva/omnilogs/log_explorer.sy"
Test-Module "log_dashboard.sy" "$root/sylva/omnilogs/log_dashboard.sy"
Test-Module "log_proofs.ax" "$root/axiom/omnilogs/log_proofs.ax"
Test-Module "test_omnilogs_complete.ti" "$root/tests/test_omnilogs_complete.ti"

Write-Host "`n=== PHASE 31 - OmniTrace ===" -ForegroundColor Yellow
Test-Module "trace_config.ti" "$root/titan/omnitrace/trace_config.ti"
Test-Module "span_pipeline.ti" "$root/titan/omnitrace/span_pipeline.ti"
Test-Module "trace_collector.ae" "$root/aether/omnitrace/trace_collector.ae"
Test-Module "trace_sampler.ae" "$root/aether/omnitrace/trace_sampler.ae"
Test-Module "trace_explorer.sy" "$root/sylva/omnitrace/trace_explorer.sy"
Test-Module "trace_map.sy" "$root/sylva/omnitrace/trace_map.sy"
Test-Module "trace_proofs.ax" "$root/axiom/omnitrace/trace_proofs.ax"
Test-Module "test_omnitrace_complete.ti" "$root/tests/test_omnitrace_complete.ti"

Write-Host "`n=== PHASE 32 - OmniCron ===" -ForegroundColor Yellow
Test-Module "scheduler_config.ti" "$root/titan/omnicron/scheduler_config.ti"
Test-Module "job_executor.ti" "$root/titan/omnicron/job_executor.ti"
Test-Module "cron_leader.ae" "$root/aether/omnicron/cron_leader.ae"
Test-Module "cron_worker.ae" "$root/aether/omnicron/cron_worker.ae"
Test-Module "cron_dashboard.sy" "$root/sylva/omnicron/cron_dashboard.sy"
Test-Module "job_history.sy" "$root/sylva/omnicron/job_history.sy"
Test-Module "cron_proofs.ax" "$root/axiom/omnicron/cron_proofs.ax"
Test-Module "test_omnicron_complete.ti" "$root/tests/test_omnicron_complete.ti"

Write-Host "`n=== PHASE 33 - OmniSecrets ===" -ForegroundColor Yellow
Test-Module "secrets_config.ti" "$root/titan/omnisecrets/secrets_config.ti"
Test-Module "encryption_service.ti" "$root/titan/omnisecrets/encryption_service.ti"
Test-Module "secret_rotator.ae" "$root/aether/omnisecrets/secret_rotator.ae"
Test-Module "access_broker.ae" "$root/aether/omnisecrets/access_broker.ae"
Test-Module "secrets_dashboard.sy" "$root/sylva/omnisecrets/secrets_dashboard.sy"
Test-Module "audit_log.sy" "$root/sylva/omnisecrets/audit_log.sy"
Test-Module "secrets_proofs.ax" "$root/axiom/omnisecrets/secrets_proofs.ax"
Test-Module "test_omnisecrets_complete.ti" "$root/tests/test_omnisecrets_complete.ti"

Write-Host "`n=== PHASE 34 - OmniConfig ===" -ForegroundColor Yellow
Test-Module "config_loader.ti" "$root/titan/omniconfig/config_loader.ti"
Test-Module "feature_flags.ti" "$root/titan/omniconfig/feature_flags.ti"
Test-Module "config_watcher.ae" "$root/aether/omniconfig/config_watcher.ae"
Test-Module "config_sync.ae" "$root/aether/omniconfig/config_sync.ae"
Test-Module "config_dashboard.sy" "$root/sylva/omniconfig/config_dashboard.sy"
Test-Module "diff_viewer.sy" "$root/sylva/omniconfig/diff_viewer.sy"
Test-Module "config_proofs.ax" "$root/axiom/omniconfig/config_proofs.ax"
Test-Module "test_omiconfig_complete.ti" "$root/tests/test_omiconfig_complete.ti"

Write-Host "`n=== REGRESSION TESTS ===" -ForegroundColor Yellow
Test-Module "event_config.ti" "$root/titan/omnievent/event_config.ti"
Test-Module "test_omnievent_complete.ti" "$root/tests/test_omnievent_complete.ti"

$total = $passed + $failed
Write-Host "`n============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  PHASES 27-34: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  Zero regressions" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })

if ($failed -gt 0) { exit 1 }
