# verify_omnitranslateml.ps1 — Phase 93 OmniTranslateML verification

Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  VERIFICATION - PHASE 93 OMNITRANSLATEML                    ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$modules = @(
    "titan/omnitranslateml/auto_translate.ti",
    "titan/omnitranslateml/human_review.ti",
    "aether/omnitranslateml/translation_worker.ae",
    "aether/omnitranslateml/review_queue.ae",
    "sylva/omnitranslateml/translate_dashboard.sy",
    "sylva/omnitranslateml/review_interface.sy",
    "axiom/omnitranslateml/translate_ml_proofs.ax",
    "tests/test_omnitranslateml_complete.ti"
)

$regression = @(
    "titan/omnilocale/locale_factory.ti",
    "tests/test_omnilocale_complete.ti"
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
