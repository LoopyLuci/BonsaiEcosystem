# verify_phases35_42.ps1 — Application Platform Layer verification script
# Phase 35: OmniAPI, Phase 36: OmniGraph, Phase 37: OmniML, Phase 38: OmniEdge
# Phase 39: OmniCLI, Phase 40: OmniDocs, Phase 41: OmniTest, Phase 42: OmniRepl

Write-Host "╔════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  VERIFICATION — PHASES 35–42 (64 NEW MODULES)        ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$newModules = @(
    # Phase 35 OmniAPI (8 modules)
    "titan/omniapi/gateway_config.ti",
    "titan/omniapi/openapi_gen.ti",
    "aether/omniapi/request_router.ae",
    "aether/omniapi/rate_limiter.ae",
    "sylva/omniapi/api_explorer.sy",
    "sylva/omniapi/gateway_dashboard.sy",
    "axiom/omniapi/gateway_proofs.ax",
    "tests/test_omniapi_complete.ti",
    # Phase 36 OmniGraph (8 modules)
    "titan/omnigraph/graph_config.ti",
    "titan/omnigraph/graph_query.ti",
    "aether/omnigraph/graph_engine.ae",
    "aether/omnigraph/graph_partition.ae",
    "sylva/omnigraph/graph_explorer.sy",
    "sylva/omnigraph/visualizer.sy",
    "axiom/omnigraph/graph_proofs.ax",
    "tests/test_omnigraph_complete.ti",
    # Phase 37 OmniML (8 modules)
    "titan/omniml/model_config.ti",
    "titan/omniml/inference_service.ti",
    "aether/omniml/training_orchestrator.ae",
    "aether/omniml/model_server.ae",
    "sylva/omniml/ml_dashboard.sy",
    "sylva/omniml/experiment_tracker.sy",
    "axiom/omniml/ml_proofs.ax",
    "tests/test_omniml_complete.ti",
    # Phase 38 OmniEdge (8 modules)
    "titan/omniedge/edge_config.ti",
    "titan/omniedge/wasm_runtime.ti",
    "aether/omniedge/edge_orchestrator.ae",
    "aether/omniedge/cdn_purger.ae",
    "sylva/omniedge/edge_map.sy",
    "sylva/omniedge/cdn_dashboard.sy",
    "axiom/omniedge/edge_proofs.ax",
    "tests/test_omniedge_complete.ti",
    # Phase 39 OmniCLI (8 modules)
    "titan/omnicli/cli_config.ti",
    "titan/omnicli/cli_executor.ti",
    "aether/omnicli/cli_session.ae",
    "aether/omnicli/cli_daemon.ae",
    "sylva/omnicli/interactive_shell.sy",
    "sylva/omnicli/command_palette.sy",
    "axiom/omnicli/cli_proofs.ax",
    "tests/test_omnicli_complete.ti",
    # Phase 40 OmniDocs (8 modules)
    "titan/omnidocs/docs_config.ti",
    "titan/omnidocs/code_parser.ti",
    "aether/omnidocs/doc_builder.ae",
    "aether/omnidocs/search_indexer.ae",
    "sylva/omnidocs/doc_explorer.sy",
    "sylva/omnidocs/api_playground.sy",
    "axiom/omnidocs/docs_proofs.ax",
    "tests/test_omnidocs_complete.ti",
    # Phase 41 OmniTest (8 modules)
    "titan/omnitest/test_config.ti",
    "titan/omnitest/benchmark_runner.ti",
    "aether/omnitest/test_runner.ae",
    "aether/omnitest/flaky_detector.ae",
    "sylva/omnitest/test_dashboard.sy",
    "sylva/omnitest/coverage_viewer.sy",
    "axiom/omnitest/test_proofs.ax",
    "tests/test_omnitest_complete.ti",
    # Phase 42 OmniRepl (8 modules)
    "titan/omnirepl/repl_config.ti",
    "titan/omnirepl/language_server.ti",
    "aether/omnirepl/repl_session.ae",
    "aether/omnirepl/repl_sync.ae",
    "sylva/omnirepl/terminal_ui.sy",
    "sylva/omnirepl/variable_inspector.sy",
    "axiom/omnirepl/repl_proofs.ax",
    "tests/test_omnirepl_complete.ti"
)

$regression = @(
    # Regression tests from previous phases
    "titan/omnicache/cache_config.ti",
    "tests/test_omnicache_complete.ti",
    "tests/test_omiconfig_complete.ti",
    "tests/test_omnisecrets_complete.ti"
)

$all = $newModules + $regression
$passed = 0
$failed = 0
$failed_modules = @()

Write-Host "Testing $($all.Count) modules (64 new + 4 regression)..." -ForegroundColor Yellow
Write-Host ""

foreach ($mod in $all) {
    $result = & ".\titan-bootstrap\target\release\titan-bootstrap.exe" $mod --run 2>&1
    if ($result -match "Result: 111") {
        Write-Host "✓ $mod [111]" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "✗ $mod [FAILED]" -ForegroundColor Red
        $failed++
        $failed_modules += $mod
    }
}

Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  VERIFICATION RESULTS                                  ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "New modules:        $($newModules.Count)" -ForegroundColor White
Write-Host "Regression tests:   $($regression.Count)" -ForegroundColor White
Write-Host "Total:              $($all.Count)" -ForegroundColor White
Write-Host ""
Write-Host "Passed:             $passed" -ForegroundColor Green
Write-Host "Failed:             $failed" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host ""

if ($failed -gt 0) {
    Write-Host "Failed modules:" -ForegroundColor Red
    $failed_modules | ForEach-Object { Write-Host "  - $_" -ForegroundColor Red }
    Write-Host ""
    exit 1
} else {
    Write-Host "✓ All $($all.Count) modules verified successfully!" -ForegroundColor Green
    Write-Host "✓ Zero regressions!" -ForegroundColor Green
    Write-Host ""
    exit 0
}
