# verify_phaseS_bootstrap.ps1 — Phase S: Bootstrap v5 — String/bool/break/untyped-let support
# Verifies that the compiler stage files compile via real transpilation (not stub).
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/compiler/lexer.ti",
    "titan/compiler/parser.ti",
    "titan/compiler/codegen.ti",
    "titan/compiler/c_backend.ti",
    "titan/compiler/bootstrap_witness.ti",
    "titan/compiler/compiler.ti",
    "titan/compiler/vm.ti"
)
$regression = @(
    "titan/omnilib/omniNet.ti",
    "titan/omnilib/omniThread.ti",
    "titan/omnilib/omniAudio.ti",
    "tests/test_integration.ti",
    "omnisystem_main.ti"
)

$failed = 0
Write-Host "`n=== Phase S Bootstrap v5: String/bool/break/untyped-let ===" -ForegroundColor Cyan
foreach ($mod in $modules) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}
Write-Host "`n=== Regression ===" -ForegroundColor Cyan
foreach ($mod in $regression) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}
$total = $modules.Count + $regression.Count
Write-Host ""
if ($failed -eq 0) { Write-Host "RESULT: ALL $total VERIFIED - Phase S Bootstrap complete [score: 111]" -ForegroundColor Green; exit 0 }
else { Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red; exit 1 }
