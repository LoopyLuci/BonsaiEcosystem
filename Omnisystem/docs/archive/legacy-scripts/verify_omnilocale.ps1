# verify_omnilocale.ps1 — Phase 92 OmniLocale verification

Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  VERIFICATION - PHASE 92 OMNILOCALE                         ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$modules = @(
    "titan/omnilocale/locale_factory.ti",
    "titan/omnilocale/locale_validator.ti",
    "aether/omnilocale/pack_distributor.ae",
    "aether/omnilocale/translation_sync.ae",
    "sylva/omnilocale/locale_browser.sy",
    "sylva/omnilocale/coverage_viewer.sy",
    "axiom/omnilocale/locale_proofs.ax",
    "tests/test_omnilocale_complete.ti"
)

$regression = @(
    "titan/omniscript/unicode_normalizer.ti",
    "tests/test_omniscript_complete.ti"
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
