$ErrorActionPreference = "Stop"
Set-Location "$PSScriptRoot\..\.."

$compiler = ".\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $compiler)) {
    Write-Host "Missing compiler: $compiler" -ForegroundColor Red
    exit 1
}

$modules = @(
    "titan/omnishell/window.ti",
    "titan/omniview/renderer.ti",
    "titan/omnishell/ipc.ti",
    "sylva/ui/components.ti",
    "titan/omnishell/app.ti",
    "tests/test_omnishell.ti"
)

$regression = @(
    "titan/std/vec.ti",
    "titan/std/map.ti",
    "titan/std/queue.ti",
    "titan/std/sort.ti"
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
    Write-Host "`nOmniShell Phase 200: ALL MODULES VERIFIED" -ForegroundColor Green
    exit 0
}

Write-Host "`n$failed module(s) failed." -ForegroundColor Red
exit 1
