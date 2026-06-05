# verify_phase204.ps1 — Phase 204: Diff-First Editing & Time-Travel Debugging
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/diff/semantic_diff.ti",
    "titan/diff/atomic_rollback.ti",
    "titan/timetravel/execution_recorder.ti",
    "titan/timetravel/reverse_debugger.ti",
    "titan/timetravel/timeline_view.ti",
    "tests/test_time_travel.ti"
)
$regression = @(
    "titan/rag/chunker.ti",
    "titan/rag/vector_store.ti",
    "titan/lsp/lsp_server.ti",
    "titan/std/vec.ti",
    "titan/agent_core/task_decomposer.ti",
    "titan/agent_core/orchestrator.ti"
)
$failed = 0
Write-Host "`n=== Phase 204: Diff-First Editing & Time-Travel Debugging ===" -ForegroundColor Cyan
foreach ($mod in $modules) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") {
        Write-Host "  PASS  $mod" -ForegroundColor Green
    } else {
        Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red
        $failed++
    }
}
Write-Host "`n=== Regression ===" -ForegroundColor Cyan
foreach ($mod in $regression) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") {
        Write-Host "  PASS  $mod" -ForegroundColor Green
    } else {
        Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red
        $failed++
    }
}
$total = $modules.Count + $regression.Count
Write-Host ""
if ($failed -eq 0) {
    Write-Host "RESULT: ALL $total VERIFIED — Phase 204 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
