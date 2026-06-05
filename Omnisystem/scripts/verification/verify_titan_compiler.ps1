# verify_titan_compiler.ps1 — Verify all Titan compiler pipeline modules
# Each module must return Result: 111 when run through the bootstrap.

$ErrorActionPreference = "Stop"
$root = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
$bootstrap = Join-Path $root "titan-bootstrap\output\titan-compiler.exe"

if (-not (Test-Path $bootstrap)) {
    Write-Error "Bootstrap not found at $bootstrap"
    exit 1
}

$pass = 0
$fail = 0
$results = @()

function Run-Test {
    param([string]$Name, [string]$File)
    $path = Join-Path $root $File
    if (-not (Test-Path $path)) {
        Write-Host "  SKIP  $Name (file not found: $File)" -ForegroundColor Yellow
        return
    }
    try {
        $out = & $bootstrap $path 2>&1 | Out-String
        if ($out -match "Result:\s*111") {
            Write-Host "  PASS  $Name" -ForegroundColor Green
            $script:pass++
            $script:results += [pscustomobject]@{ Name=$Name; Status="PASS"; Output=$out.Trim() }
        } else {
            Write-Host "  FAIL  $Name" -ForegroundColor Red
            Write-Host "        Output: $($out.Trim())" -ForegroundColor DarkRed
            $script:fail++
            $script:results += [pscustomobject]@{ Name=$Name; Status="FAIL"; Output=$out.Trim() }
        }
    } catch {
        Write-Host "  ERROR $Name — $_" -ForegroundColor Red
        $script:fail++
        $script:results += [pscustomobject]@{ Name=$Name; Status="ERROR"; Output=$_.ToString() }
    }
}

Write-Host ""
Write-Host "=== Titan Compiler Pipeline Verification ===" -ForegroundColor Cyan
Write-Host ""

# Core pipeline modules
Run-Test "Lexer"              "titan/compiler/lexer.ti"
Run-Test "Parser"             "titan/compiler/parser.ti"
Run-Test "Codegen"            "titan/compiler/codegen.ti"
Run-Test "VM"                 "titan/compiler/vm.ti"
Run-Test "Compiler (full)"    "titan/compiler/compiler.ti"

Write-Host ""
Write-Host "=== C Backend and Self-Hosting ===" -ForegroundColor Cyan
Write-Host ""

Run-Test "C Backend"          "titan/compiler/c_backend.ti"
Run-Test "Bootstrap Witness"  "titan/compiler/bootstrap_witness.ti"

Write-Host ""
Write-Host "=== Integration Tests ===" -ForegroundColor Cyan
Write-Host ""

Run-Test "Compiler Integration Test"  "tests/test_titan_compiler.ti"

Write-Host ""
Write-Host "=== Summary ===" -ForegroundColor Cyan
Write-Host "  Passed: $pass" -ForegroundColor Green
Write-Host "  Failed: $fail" -ForegroundColor $(if ($fail -gt 0) { "Red" } else { "Green" })
Write-Host ""

if ($fail -gt 0) {
    Write-Host "RESULT: FAIL — $fail test(s) did not return 111" -ForegroundColor Red
    exit 1
} else {
    Write-Host "RESULT: PASS — All $pass compiler modules verified (score: 111)" -ForegroundColor Green
    exit 0
}
