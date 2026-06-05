$ErrorActionPreference = "Stop"
Set-Location "$PSScriptRoot\..\.."

$compiler = ".\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $compiler)) {
    Write-Host "Missing compiler: $compiler" -ForegroundColor Red
    exit 1
}

$modules = @(
    "titan/omnishell_ps/ps_lexer.ti",
    "titan/omnishell_ps/ps_parser.ti",
    "titan/omnishell_ps/ps_exec.ti",
    "tests/test_omnishell_ps.ti"
)

$regression = @(
    "titan/omnishell/window.ti",
    "titan/omnishell/ipc.ti"
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
    Write-Host "`nOmniShell PS: ALL MODULES VERIFIED" -ForegroundColor Green
    exit 0
}

Write-Host "`n$failed failed" -ForegroundColor Red
exit 1
