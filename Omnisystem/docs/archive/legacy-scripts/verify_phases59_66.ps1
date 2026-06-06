#!/usr/bin/env pwsh
# scripts/verification/verify_phases59_66.ps1
# Combined verification for Phases 59-66 (64 modules) + 4 regression tests = 68 total

$bootstrap = ".\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0
$modules = @()

# Phase 59: OmniThreat (8 modules)
$modules += "titan/omnitheat/threat_config.ti"
$modules += "titan/omnitheat/anomaly_engine.ti"
$modules += "aether/omnitheat/threat_hunter.ae"
$modules += "aether/omnitheat/incident_responder.ae"
$modules += "sylva/omnitheat/threat_dashboard.sy"
$modules += "sylva/omnitheat/incident_timeline.sy"
$modules += "axiom/omnitheat/threat_proofs.ax"
$modules += "tests/test_omnitheat_complete.ti"

# Phase 60: OmniScanner (8 modules)
$modules += "titan/omniscanner/scanner_config.ti"
$modules += "titan/omniscanner/dependency_check.ti"
$modules += "aether/omniscanner/scan_worker.ae"
$modules += "aether/omniscanner/remediation_tracker.ae"
$modules += "sylva/omniscanner/scan_dashboard.sy"
$modules += "sylva/omniscanner/vulnerability_viewer.sy"
$modules += "axiom/omniscanner/scanner_proofs.ax"
$modules += "tests/test_omniscanner_complete.ti"

# Phase 61: OmniWAF (8 modules)
$modules += "titan/omniwaf/waf_config.ti"
$modules += "titan/omniwaf/bot_detection.ti"
$modules += "aether/omniwaf/waf_engine.ae"
$modules += "aether/omniwaf/traffic_analyzer.ae"
$modules += "sylva/omniwaf/waf_dashboard.sy"
$modules += "sylva/omniwaf/attack_map.sy"
$modules += "axiom/omniwaf/waf_proofs.ax"
$modules += "tests/test_omniwaf_complete.ti"

# Phase 62: OmniFinOps (8 modules)
$modules += "titan/omnifinops/finops_config.ti"
$modules += "titan/omnifinops/savings_advisor.ti"
$modules += "aether/omnifinops/cost_collector.ae"
$modules += "aether/omnifinops/budget_alerter.ae"
$modules += "sylva/omnifinops/cost_dashboard.sy"
$modules += "sylva/omnifinops/savings_opportunities.sy"
$modules += "axiom/omnifinops/finops_proofs.ax"
$modules += "tests/test_omnifinops_complete.ti"

# Phase 63: OmniTenant (8 modules)
$modules += "titan/omnitenant/tenant_config.ti"
$modules += "titan/omnitenant/quota_enforcer.ti"
$modules += "aether/omnitenant/tenant_controller.ae"
$modules += "aether/omnitenant/quota_monitor.ae"
$modules += "sylva/omnitenant/tenant_dashboard.sy"
$modules += "sylva/omnitenant/quota_viewer.sy"
$modules += "axiom/omnitenant/tenant_proofs.ax"
$modules += "tests/test_omnitenant_complete.ti"

# Phase 64: OmniI18n (8 modules)
$modules += "titan/omnii18n/i18n_config.ti"
$modules += "titan/omnii18n/translation_engine.ti"
$modules += "aether/omnii18n/locale_resolver.ae"
$modules += "aether/omnii18n/translation_sync.ae"
$modules += "sylva/omnii18n/translation_editor.sy"
$modules += "sylva/omnii18n/locale_preview.sy"
$modules += "axiom/omnii18n/i18n_proofs.ax"
$modules += "tests/test_omnii18n_complete.ti"

# Phase 65: OmniPlugin (8 modules)
$modules += "titan/omniplugin/plugin_config.ti"
$modules += "titan/omniplugin/marketplace_api.ti"
$modules += "aether/omniplugin/plugin_loader.ae"
$modules += "aether/omniplugin/sandbox_runtime.ae"
$modules += "sylva/omniplugin/marketplace_ui.sy"
$modules += "sylva/omniplugin/plugin_manager.sy"
$modules += "axiom/omniplugin/plugin_proofs.ax"
$modules += "tests/test_omniplugin_complete.ti"

# Phase 66: OmniHealth (8 modules)
$modules += "titan/omnihealth/health_config.ti"
$modules += "titan/omnihealth/incident_manager.ti"
$modules += "aether/omnihealth/health_checker.ae"
$modules += "aether/omnihealth/status_aggregator.ae"
$modules += "sylva/omnihealth/health_dashboard.sy"
$modules += "sylva/omnihealth/status_page.sy"
$modules += "axiom/omnihealth/health_proofs.ax"
$modules += "tests/test_omnihealth_complete.ti"

# Regression tests (Phases 51-58)
$modules += "titan/omnicompliance/compliance_config.ti"
$modules += "tests/test_omnicompliance_complete.ti"
$modules += "titan/omnivault/vault_config.ti"
$modules += "tests/test_omnivault_complete.ti"

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "Verifying Phases 59–66 (64 modules) + Regression (4 modules)" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

foreach ($module in $modules) {
    if (-not (Test-Path $module)) {
        Write-Host "✗ SKIP: $module (file not found)" -ForegroundColor Yellow
        continue
    }

    $output = & $bootstrap $module --run 2>&1
    
    if ($output -like "*Result: 111*") {
        Write-Host "✓ PASS: $module" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "✗ FAIL: $module" -ForegroundColor Red
        Write-Host "  Output: $output" -ForegroundColor Red
        $failed++
    }
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "Total: $($passed + $failed) modules | Passed: $passed | Failed: $failed" -ForegroundColor Cyan

if ($failed -eq 0 -and $passed -eq 68) {
    Write-Host "All 68 modules verified, zero regressions. [OK]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "Verification incomplete." -ForegroundColor Red
    exit 1
}
