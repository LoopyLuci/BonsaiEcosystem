# verify_phases95_102.ps1 — Production & Performance Suite verification

Write-Host "╔════════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  VERIFICATION - PHASES 95-102 (PRODUCTION & PERFORMANCE)     ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$newModules = @(
    # Phase 95 OmniOptimize
    "titan/omnioptimize/optimize_config.ti",
    "titan/omnioptimize/ir_optimizer.ti",
    "aether/omnioptimize/optimize_worker.ae",
    "aether/omnioptimize/code_cache.ae",
    "sylva/omnioptimize/optimize_dashboard.sy",
    "sylva/omnioptimize/ir_viewer.sy",
    "axiom/omnioptimize/optimize_proofs.ax",
    "tests/test_omnioptimize_complete.ti",
    # Phase 96 OmniPackage
    "titan/omnipackage/package_config.ti",
    "titan/omnipackage/dependency_resolver.ti",
    "aether/omnipackage/package_fetcher.ae",
    "aether/omnipackage/lockfile_manager.ae",
    "sylva/omnipackage/package_dashboard.sy",
    "sylva/omnipackage/dependency_graph.sy",
    "axiom/omnipackage/package_proofs.ax",
    "tests/test_omnipackage_complete.ti",
    # Phase 97 OmniRegistry
    "titan/omniregistry/registry_config.ti",
    "titan/omniregistry/version_manager.ti",
    "aether/omniregistry/registry_server.ae",
    "aether/omniregistry/mirror_sync.ae",
    "sylva/omniregistry/registry_browser.sy",
    "sylva/omniregistry/version_history.sy",
    "axiom/omniregistry/registry_proofs.ax",
    "tests/test_omniregistry_complete.ti",
    # Phase 98 OmniBenchmark
    "titan/omnibenchmark/benchmark_config.ti",
    "titan/omnibenchmark/perf_runner.ti",
    "aether/omnibenchmark/benchmark_worker.ae",
    "aether/omnibenchmark/result_aggregator.ae",
    "sylva/omnibenchmark/benchmark_dashboard.sy",
    "sylva/omnibenchmark/flamegraph_viewer.sy",
    "axiom/omnibenchmark/benchmark_proofs.ax",
    "tests/test_omnibenchmark_complete.ti",
    # Phase 99 OmniFuzz
    "titan/omnifuzz/fuzz_config.ti",
    "titan/omnifuzz/corpus_manager.ti",
    "aether/omnifuzz/fuzz_worker.ae",
    "aether/omnifuzz/crash_reporter.ae",
    "sylva/omnifuzz/fuzz_dashboard.sy",
    "sylva/omnifuzz/coverage_viewer.sy",
    "axiom/omnifuzz/fuzz_proofs.ax",
    "tests/test_omnifuzz_complete.ti",
    # Phase 100 OmniRelease
    "titan/omnirelease/release_config.ti",
    "titan/omnirelease/changelog_gen.ti",
    "aether/omnirelease/release_builder.ae",
    "aether/omnirelease/distribution_sync.ae",
    "sylva/omnirelease/release_dashboard.sy",
    "sylva/omnirelease/changelog_viewer.sy",
    "axiom/omnirelease/release_proofs.ax",
    "tests/test_omnirelease_complete.ti",
    # Phase 101 OmniMonitor
    "titan/omnimonitor/monitor_config.ti",
    "titan/omnimonitor/alert_engine.ti",
    "aether/omnimonitor/metrics_collector.ae",
    "aether/omnimonitor/alert_dispatcher.ae",
    "sylva/omnimonitor/monitor_dashboard.sy",
    "sylva/omnimonitor/alert_timeline.sy",
    "axiom/omnimonitor/monitor_proofs.ax",
    "tests/test_omnimonitor_complete.ti",
    # Phase 102 OmniComplete
    "titan/omnicomplete/complete_config.ti",
    "titan/omnicomplete/system_validator.ti",
    "aether/omnicomplete/integration_runner.ae",
    "aether/omnicomplete/certificate_issuer.ae",
    "sylva/omnicomplete/platform_dashboard.sy",
    "sylva/omnicomplete/phase_explorer.sy",
    "axiom/omnicomplete/complete_proofs.ax",
    "tests/test_omnicomplete_complete.ti"
)

$regression = @(
    "titan/omnitop20/top20_config.ti",
    "tests/test_omnitop20_complete.ti"
)

$all = $newModules + $regression
$pass = 0
$fail = 0

foreach ($mod in $all) {
    $result = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    if ($result -match "Result: 111") {
        Write-Host "$mod : 111 [OK]" -ForegroundColor Green
        $pass++
    } else {
        Write-Host "$mod : FAILED" -ForegroundColor Red
        $fail++
    }
}

Write-Host ""
Write-Host "════════════════════════════════════════════════════════════════"
Write-Host "Results: $pass passed, $fail failed out of $($all.Count) total" -ForegroundColor $(if ($fail -eq 0) { "Green" } else { "Red" })

if ($fail -eq 0) {
    Write-Host "✓ All modules verified, zero regressions." -ForegroundColor Green
    Write-Host "Ready for commit." -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ $fail module(s) failed verification." -ForegroundColor Red
    exit 1
}
