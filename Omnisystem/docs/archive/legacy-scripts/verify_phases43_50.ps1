# Verification script for Phases 43–50 (Developer Productivity Suite) + Regression
# Tests 64 new modules (8 phases × 8 modules) + 4 regression modules = 68 total
# All modules must return "Result: 111"

$TITAN_BOOTSTRAP = ".\titan-bootstrap\target\release\titan-bootstrap.exe"
$MODULES_TO_TEST = @(
    # Phase 43 — OmniGit (8 modules)
    "titan\omnigit\git_config.ti",
    "titan\omnigit\commit_graph.ti",
    "aether\omnigit\merge_orchestrator.ae",
    "aether\omnigit\repo_sync.ae",
    "sylva\omnigit\git_explorer.sy",
    "sylva\omnigit\diff_viewer.sy",
    "axiom\omnigit\git_proofs.ax",
    "tests\test_omnigit_complete.ti",
    
    # Phase 44 — OmniCI (8 modules)
    "titan\omnici\pipeline_config.ti",
    "titan\omnici\build_cache.ti",
    "aether\omnici\pipeline_runner.ae",
    "aether\omnici\artifact_store.ae",
    "sylva\omnici\pipeline_dashboard.sy",
    "sylva\omnici\build_log_viewer.sy",
    "axiom\omnici\ci_proofs.ax",
    "tests\test_omnici_complete.ti",
    
    # Phase 45 — OmniReview (8 modules)
    "titan\omnireview\review_config.ti",
    "titan\omnireview\static_analyzer.ti",
    "aether\omnireview\review_bot.ae",
    "aether\omnireview\approval_gate.ae",
    "sylva\omnireview\review_dashboard.sy",
    "sylva\omnireview\diff_annotator.sy",
    "axiom\omnireview\review_proofs.ax",
    "tests\test_omnireview_complete.ti",
    
    # Phase 46 — OmniSearch (8 modules)
    "titan\omnisearch\search_config.ti",
    "titan\omnisearch\query_parser.ti",
    "aether\omnisearch\index_builder.ae",
    "aether\omnisearch\search_engine.ae",
    "sylva\omnisearch\search_ui.sy",
    "sylva\omnisearch\result_explorer.sy",
    "axiom\omnisearch\search_proofs.ax",
    "tests\test_omnisearch_complete.ti",
    
    # Phase 47 — OmniRefactor (8 modules)
    "titan\omnirefactor\refactor_config.ti",
    "titan\omnirefactor\code_transformer.ti",
    "aether\omnirefactor\refactor_worker.ae",
    "aether\omnirefactor\batch_refactor.ae",
    "sylva\omnirefactor\refactor_dashboard.sy",
    "sylva\omnirefactor\change_preview.sy",
    "axiom\omnirefactor\refactor_proofs.ax",
    "tests\test_omnirefactor_complete.ti",
    
    # Phase 48 — OmniDebug (8 modules)
    "titan\omnidebug\debug_config.ti",
    "titan\omnidebug\time_travel.ti",
    "aether\omnidebug\debug_session.ae",
    "aether\omnidebug\remote_debug.ae",
    "sylva\omnidebug\debug_ui.sy",
    "sylva\omnidebug\variable_inspector.sy",
    "axiom\omnidebug\debug_proofs.ax",
    "tests\test_omnidebug_complete.ti",
    
    # Phase 49 — OmniProfile (8 modules)
    "titan\omniprofile\profile_config.ti",
    "titan\omniprofile\heap_analyzer.ti",
    "aether\omniprofile\profile_collector.ae",
    "aether\omniprofile\profile_compare.ae",
    "sylva\omniprofile\flamegraph_viewer.sy",
    "sylva\omniprofile\profile_dashboard.sy",
    "axiom\omniprofile\profile_proofs.ax",
    "tests\test_omniprofile_complete.ti",
    
    # Phase 50 — OmniCollaborate (8 modules)
    "titan\omnicollaborate\collab_config.ti",
    "titan\omnicollaborate\crdt_engine.ti",
    "aether\omnicollaborate\collab_session.ae",
    "aether\omnicollaborate\presence_tracker.ae",
    "sylva\omnicollaborate\collab_editor.sy",
    "sylva\omnicollaborate\presence_map.sy",
    "axiom\omnicollaborate\collab_proofs.ax",
    "tests\test_omnicollaborate_complete.ti",
    
    # Regression — 4 modules from earlier phases
    "titan\omniapi\gateway_config.ti",
    "titan\omnirepl\repl_config.ti",
    "tests\test_omniapi_complete.ti",
    "tests\test_omnirepl_complete.ti"
)

$PASSED = 0
$FAILED = 0
$FAILED_MODULES = @()

Write-Host "Verifying Phases 43–50 (Developer Productivity Suite) + Regression Tests" -ForegroundColor Cyan
Write-Host "Total modules to test: $($MODULES_TO_TEST.Count)" -ForegroundColor Cyan
Write-Host "" -ForegroundColor White

foreach ($module in $MODULES_TO_TEST) {
    Write-Host "Testing: $module" -ForegroundColor Yellow
    $OUTPUT = & $TITAN_BOOTSTRAP $module --run 2>&1
    
    if ($OUTPUT -match "Result: 111") {
        Write-Host "✓ PASS: $module [Result: 111]" -ForegroundColor Green
        $PASSED += 1
    } else {
        Write-Host "✗ FAIL: $module" -ForegroundColor Red
        Write-Host "  Output: $OUTPUT" -ForegroundColor Red
        $FAILED += 1
        $FAILED_MODULES += $module
    }
}

Write-Host "" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Verification Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Passed: $PASSED / $($MODULES_TO_TEST.Count)" -ForegroundColor Green
Write-Host "Failed: $FAILED / $($MODULES_TO_TEST.Count)" -ForegroundColor $(if ($FAILED -eq 0) { "Green" } else { "Red" })

if ($FAILED -gt 0) {
    Write-Host "" -ForegroundColor White
    Write-Host "Failed Modules:" -ForegroundColor Red
    foreach ($module in $FAILED_MODULES) {
        Write-Host "  - $module" -ForegroundColor Red
    }
    exit 1
} else {
    Write-Host "" -ForegroundColor White
    Write-Host "All $($MODULES_TO_TEST.Count) modules verified [OK]" -ForegroundColor Green
    exit 0
}
