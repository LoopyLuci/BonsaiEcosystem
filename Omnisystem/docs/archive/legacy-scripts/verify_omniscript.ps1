# verify_omniscript.ps1 — Phase 91 OmniScript verification

Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  VERIFICATION - PHASE 91 OMNISCRIPT                         ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$modules = @(
    "titan/omniscript/unicode_normalizer.ti",
    "titan/omniscript/universal_tokenizer.ti",
    "aether/omniscript/keyword_mapper.ae",
    "aether/omniscript/locale_loader.ae",
    "sylva/omniscript/locale_editor.sy",
    "sylva/omniscript/script_dashboard.sy",
    "axiom/omniscript/omniscript_proofs.ax",
    "tests/test_omniscript_complete.ti"
)

$regression = @(
    "titan/omnifinale/finale_config.ti",
    "tests/test_omnifinale_complete.ti"
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
