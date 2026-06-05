# verify_phase202.ps1 — Phase 202: RAG Engine: Semantic Code Understanding
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/rag/chunker.ti",
    "titan/rag/embedding.ti",
    "titan/rag/vector_store.ti",
    "titan/rag/hybrid_search.ti",
    "tests/test_rag.ti"
)
$regression = @(
    "titan/compiler/compiler.ti",
    "titan/compiler/vm.ti",
    "titan/std/vec.ti",
    "titan/lsp/lsp_server.ti"
)
$failed = 0
Write-Host "`n=== Phase 202: RAG Engine Modules ===" -ForegroundColor Cyan
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
    Write-Host "RESULT: ALL $total VERIFIED — Phase 202 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
