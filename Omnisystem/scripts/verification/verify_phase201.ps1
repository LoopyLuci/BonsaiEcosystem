# verify_phase201.ps1 — Phase 201: LSP Server with Deep Semantic Analysis
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/lsp/lsp_protocol.ti",
    "titan/lsp/semantic_analyzer.ti",
    "titan/lsp/semantic_tokens.ti",
    "titan/lsp/completion_engine.ti",
    "titan/lsp/lsp_server.ti",
    "tests/test_lsp.ti"
)
$regression = @(
    "titan/compiler/compiler.ti",
    "titan/compiler/vm.ti",
    "titan/std/vec.ti",
    "titan/std/map.ti"
)
$failed = 0
Write-Host "`n=== Phase 201: LSP Modules ===" -ForegroundColor Cyan
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
    Write-Host "RESULT: ALL $total VERIFIED — Phase 201 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
