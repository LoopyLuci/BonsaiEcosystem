$ErrorActionPreference = "Stop"
Set-Location "$PSScriptRoot\..\.."

$compiler = ".\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $compiler)) {
    Write-Host "Missing compiler: $compiler" -ForegroundColor Red
    exit 1
}

$modules = @(
    "titan/omnibrowser/omniml_parser.ti",
    "titan/omnibrowser/style.ti",
    "titan/omnibrowser/layout.ti",
    "titan/omnibrowser/compositor.ti",
    "titan/omnibrowser/browser.ti",
    "tests/test_omnibrowser.ti"
)

$regression = @(
    "titan/omnishell/window.ti",
    "titan/omniview/renderer.ti"
)

$all = $modules + $regression
$failed = 0

foreach ($mod in $all) {
    if (-not (Test-Path $mod)) {
        Write-Host "$mod : MISSING" -ForegroundColor Red
        $failed++
        continue
    }

    $result = & $compiler $mod --run 2>&1
    if ($result -match "Result:\s*111") {
        Write-Host "$mod : 111 [OK]" -ForegroundColor Green
    } else {
        Write-Host "$mod : FAILED (got $result)" -ForegroundColor Red
        $failed++
    }
}

if ($failed -eq 0) {
    Write-Host "`nOmniBrowser: ALL MODULES VERIFIED" -ForegroundColor Green
    exit 0
}

Write-Host "`n$failed module(s) failed." -ForegroundColor Red
exit 1
