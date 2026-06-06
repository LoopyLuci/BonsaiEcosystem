$root = Resolve-Path "$PSScriptRoot\..\.."
$exe = "$root\titan-bootstrap\target\release\titan-bootstrap.exe"

$modules = @(
    @{ name = "titan/std/rle.ti"; test = "titan/std/rle.ti" },
    @{ name = "titan/std/delta.ti"; test = "titan/std/delta.ti" },
    @{ name = "titan/std/huffman.ti"; test = "titan/std/huffman.ti" },
    @{ name = "tests/test_omni_lib_compression.ti"; test = "tests/test_omni_lib_compression.ti" }
)

$regression = @(
    "titan/std/vec.ti",
    "titan/std/map.ti",
    "titan/std/queue.ti",
    "titan/std/sort.ti",
    "titan/std/crypto.ti",
    "titan/std/channel.ti",
    "titan/std/atomic.ti"
)

$failed = 0

Write-Host "=== PHASE 124 VERIFICATION — 11 Modules ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Phase 124 New Modules (4):" -ForegroundColor Yellow

foreach ($entry in $modules) {
    $mod = $entry.name
    $test = "$root\$($entry.test)"
    $result = & $exe $test --run 2>&1
    if ($result -match "Result:\s*111") {
        Write-Host "  ✓ $mod : 111 [OK]" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $mod : FAILED" -ForegroundColor Red
        Write-Host "    Output: $result" -ForegroundColor DarkRed
        $failed++
    }
}

Write-Host ""
Write-Host "Regression Modules (7):" -ForegroundColor Yellow

foreach ($mod in $regression) {
    $test = "$root\$mod"
    $result = & $exe $test --run 2>&1
    if ($result -match "Result:\s*111") {
        Write-Host "  ✓ $mod : 111 [OK]" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $mod : FAILED" -ForegroundColor Red
        Write-Host "    Output: $result" -ForegroundColor DarkRed
        $failed++
    }
}

Write-Host ""
if ($failed -eq 0) {
    Write-Host "All 11 modules verified, zero regressions." -ForegroundColor Green
    exit 0
} else {
    Write-Host "$failed module(s) failed." -ForegroundColor Red
    exit 1
}
