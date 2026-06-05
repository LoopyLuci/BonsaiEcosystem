$ErrorActionPreference = "Stop"
Set-Location "$PSScriptRoot\..\.."

$compiler = ".\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $compiler)) {
    Write-Host "Missing compiler: $compiler" -ForegroundColor Red
    exit 1
}

$modules = @(
    "titan/omnibrowser/omniml_to_html.ti",
    "titan/omnibrowser/omnistyle_to_css.ti",
    "tests/test_transpilers.ti"
)

$regression = @(
    "titan/omnibrowser/omniml_parser.ti",
    "titan/omnibrowser/style.ti"
)

$all = $modules + $regression
$failed = 0

foreach ($mod in $all) {
    if (-not (Test-Path $mod)) {
        Write-Host "$mod : MISSING" -ForegroundColor Red
        $failed++
        continue
    }

    $r = & $compiler $mod --run 2>&1
    if ($r -match "Result:\s*111") {
        Write-Host "$mod : 111 [OK]" -ForegroundColor Green
    } else {
        Write-Host "$mod : FAILED" -ForegroundColor Red
        $failed++
    }
}

if ($failed -eq 0) {
    Write-Host "`nTranspilers: ALL MODULES VERIFIED" -ForegroundColor Green
    exit 0
}

Write-Host "`n$failed failed" -ForegroundColor Red
exit 1
