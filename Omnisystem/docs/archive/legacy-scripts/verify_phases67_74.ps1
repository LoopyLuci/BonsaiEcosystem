# verify_phases67_74.ps1 — Web Platform Suite
$newModules = @(
    # Phase 67 OmniWeb
    "titan/omniweb/web_config.ti",
    "titan/omniweb/component_builder.ti",
    "aether/omniweb/dev_server.ae",
    "aether/omniweb/asset_pipeline.ae",
    "sylva/omniweb/web_ide.sy",
    "sylva/omniweb/component_preview.sy",
    "axiom/omniweb/web_proofs.ax",
    "tests/test_omniweb_complete.ti",
    # Phase 68 OmniHost
    "titan/omnihost/host_config.ti",
    "titan/omnihost/preview_server.ti",
    "aether/omnihost/site_manager.ae",
    "aether/omnihost/traffic_router.ae",
    "sylva/omnihost/site_dashboard.sy",
    "sylva/omnihost/live_preview.sy",
    "axiom/omnihost/host_proofs.ax",
    "tests/test_omnihost_complete.ti",
    # Phase 69 OmniDeployWeb
    "titan/omnideployweb/deployweb_config.ti",
    "titan/omnideployweb/deploy_engine.ti",
    "aether/omnideployweb/deploy_orchestrator.ae",
    "aether/omnideployweb/cache_invalidator.ae",
    "sylva/omnideployweb/deploy_dashboard.sy",
    "sylva/omnideployweb/deploy_history.sy",
    "axiom/omnideployweb/deployweb_proofs.ax",
    "tests/test_omnideployweb_complete.ti",
    # Phase 70 OmniWasm
    "titan/omniwasm/wasm_config.ti",
    "titan/omniwasm/wasm_optimizer.ti",
    "aether/omniwasm/wasm_runner.ae",
    "aether/omniwasm/wasm_linker.ae",
    "sylva/omniwasm/wasm_explorer.sy",
    "sylva/omniwasm/performance_profiler.sy",
    "axiom/omniwasm/wasm_proofs.ax",
    "tests/test_omniwasm_complete.ti",
    # Phase 71 OmniSEO
    "titan/omniseo/seo_config.ti",
    "titan/omniseo/seo_auditor.ti",
    "aether/omniseo/crawler_agent.ae",
    "aether/omniseo/structured_data.ae",
    "sylva/omniseo/seo_dashboard.sy",
    "sylva/omniseo/keyword_analyzer.sy",
    "axiom/omniseo/seo_proofs.ax",
    "tests/test_omniseo_complete.ti",
    # Phase 72 OmniAnalytics
    "titan/omnianalytics/analytics_config.ti",
    "titan/omnianalytics/custom_events.ti",
    "aether/omnianalytics/collector.ae",
    "aether/omnianalytics/privacy_guard.ae",
    "sylva/omnianalytics/analytics_dashboard.sy",
    "sylva/omnianalytics/realtime_viewer.sy",
    "axiom/omnianalytics/analytics_proofs.ax",
    "tests/test_omnianalytics_complete.ti",
    # Phase 73 OmniCDN
    "titan/omnicdn/cdn_config.ti",
    "titan/omnicdn/cache_warmer.ti",
    "aether/omnicdn/cdn_manager.ae",
    "aether/omnicdn/edge_deployer.ae",
    "sylva/omnicdn/cdn_dashboard.sy",
    "sylva/omnicdn/performance_map.sy",
    "axiom/omnicdn/cdn_proofs.ax",
    "tests/test_omnicdn_complete.ti",
    # Phase 74 OmniDesign
    "titan/omnidesign/design_config.ti",
    "titan/omnidesign/component_registry.ti",
    "aether/omnidesign/design_sync.ae",
    "aether/omnidesign/a11y_checker.ae",
    "sylva/omnidesign/component_playground.sy",
    "sylva/omnidesign/theme_editor.sy",
    "axiom/omnidesign/design_proofs.ax",
    "tests/test_omnidesign_complete.ti"
)

$regression = @(
    "titan/omniweb/web_config.ti",
    "titan/omnihost/host_config.ti",
    "tests/test_omniweb_complete.ti",
    "tests/test_omnihost_complete.ti"
)

Write-Host "╔════════════════════════════════════════════════╗`n║  VERIFICATION - PHASES 67-74 WEB PLATFORM SUITE║`n╚════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host "Testing $($newModules.Count) new modules + $($regression.Count) regression..." -ForegroundColor Yellow

$all = $newModules + $regression
$failed = 0
$passed = 0

foreach ($mod in $all) {
    $result = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    if ($result -match "Result: 111") {
        $passed++
        Write-Host "✓ $mod" -ForegroundColor Green
    } else {
        $failed++
        Write-Host "✗ $mod" -ForegroundColor Red
        Write-Host "  Output: $result" -ForegroundColor DarkRed
    }
}

Write-Host "`n════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "Results: $passed passed, $failed failed out of $($all.Count) total" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })

if ($failed -eq 0) {
    Write-Host "✓ All modules verified, zero regressions." -ForegroundColor Green
    Write-Host "Ready for commit." -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ $failed module(s) failed verification." -ForegroundColor Red
    exit 1
}
