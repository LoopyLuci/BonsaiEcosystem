$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "═══════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  OMNI STUDIO VERIFICATION" -ForegroundColor Cyan
Write-Host "═════════════════════════════════════════" -ForegroundColor Cyan

$exe = ".\titan-bootstrap\target\release\titan-bootstrap.exe"

$tests = @(
    "titan/studio/studio_shell.ti",
    "titan/studio/agentic_editor.ti",
    "titan/studio/module_marketplace.ti",
    "tests/test_omni_studio_complete.ti"
)

$passed = 0
foreach ($test in $tests) {
    $output = & $exe $test --run 2>&1
    $result = $output | Select-String "Result:" | Select-Object -First 1
    if ($result) {
        $value = ($result -replace ".*Result: ", "").Trim()
        if ($value -eq "111") {
            Write-Host "✓ $(Split-Path $test -Leaf) → $value" -ForegroundColor Green
            $passed++
        } else {
            Write-Host "⚠ $(Split-Path $test -Leaf) → $value" -ForegroundColor Yellow
        }
    } else {
        Write-Host "✗ $(Split-Path $test -Leaf) → FAILED" -ForegroundColor Red
    }
}

Write-Host "═══════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  OMNI STUDIO: $passed/4 modules verified" -ForegroundColor Green
Write-Host "═══════════════════════════════════════" -ForegroundColor Cyan
