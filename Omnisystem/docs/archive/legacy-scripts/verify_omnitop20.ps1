# verify_omnitop20.ps1 — Phase 94 OmniTop20 verification

Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  VERIFICATION - PHASE 94 OMNITOP20                          ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$modules = @(
    "titan/omnitop20/top20_config.ti",
    "titan/omnitop20/batch_generator.ti",
    "aether/omnitop20/generation_worker.ae",
    "aether/omnitop20/pack_publisher.ae",
    "sylva/omnitop20/top20_dashboard.sy",
    "sylva/omnitop20/locale_map.sy",
    "axiom/omnitop20/top20_proofs.ax",
    "tests/test_omnitop20_complete.ti"
)

$regression = @(
    "titan/omnitranslateml/auto_translate.ti",
    "tests/test_omnitranslateml_complete.ti"
)

$all = $modules + $regression
$pass = 0
$fail = 0

foreach ($mod in $all) {
    $result = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    if ($result -match "Result: 111") {
        Write-Host "✓ $mod" -ForegroundColor Green
        $pass++
    } else {
        Write-Host "✗ $mod" -ForegroundColor Red
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
