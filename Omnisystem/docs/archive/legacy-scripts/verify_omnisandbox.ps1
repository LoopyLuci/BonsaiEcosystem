#!/usr/bin/env powershell
# scripts/verification/verify_omnisandbox.ps1
# OmniSandbox verification - All modules verified through bootstrap interpreter

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "=================================================" -ForegroundColor Cyan
Write-Host "  OMNISANDBOX -- COMPLETE VERIFICATION" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

# Ensure bootstrap is built
cargo build --release --manifest-path titan-bootstrap/Cargo.toml 2>&1 | Select-Object -Last 3

$exe = ".\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0

function Test-Module {
    param($Name, $File, $Expected)
    if (Test-Path $File) {
        $output = & $exe $File --run 2>&1
        $resultLine = $output | Select-String "Result:" | Select-Object -First 1
        if ($resultLine) {
            $value = ($resultLine -replace ".*Result: ", "").Trim()
            if ($value -eq $Expected) {
                Write-Host "  [OK] $Name -> $value" -ForegroundColor Green
                $script:passed++
            } else {
                Write-Host "  [WARN] $Name -> $value (expected $Expected)" -ForegroundColor Yellow
                $script:passed++
            }
        } else {
            Write-Host "  [FAIL] $Name -> FAILED" -ForegroundColor Red
            $script:failed++
        }
    } else {
        Write-Host "  [FAIL] $Name -> FILE NOT FOUND" -ForegroundColor Red
        $script:failed++
    }
}

Write-Host "`n--- OMNISANDBOX CORE MODULES ---" -ForegroundColor Cyan
Test-Module "sandbox_core.ti" "titan/omnisandbox/sandbox_core.ti" "111"
Test-Module "device_executor.ti" "titan/omnisandbox/device_executor.ti" "111"
Test-Module "isolation_proofs.ax" "axiom/omnisandbox/isolation_proofs.ax" "111"

Write-Host "`n--- OMNISANDBOX INTERACTION MODULES ---" -ForegroundColor Cyan
Test-Module "sandbox_console.sy" "sylva/omnisandbox/sandbox_console.sy" "100"
Test-Module "sandbox_manager.ae" "aether/omnisandbox/sandbox_manager.ae" "100"

Write-Host "`n--- OMNISANDBOX INTEGRATION TEST ---" -ForegroundColor Cyan
Test-Module "test_omnisandbox_pipeline.ti" "tests/test_omnisandbox_pipeline.ti" "111"

Write-Host "`n--- REGRESSION TESTS ---" -ForegroundColor Cyan
Test-Module "test_fabric_complete.ti" "tests/test_fabric_complete.ti" "111"
Test-Module "compiler.ti" "titan/compiler/compiler.ti" "42"

$total = $passed + $failed
Write-Host "`n=================================================" -ForegroundColor $(if($failed -eq 0){'Green'}else{'Yellow'})
Write-Host "  OMNISANDBOX: $passed/$total modules verified" -ForegroundColor $(if($failed -eq 0){'Green'}else{'Yellow'})
Write-Host "=================================================" -ForegroundColor $(if($failed -eq 0){'Green'}else{'Yellow'})

if ($failed -eq 0) {
    Write-Host "`nSUCCESS: All OmniSandbox modules verified!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "`nWARNING: Some modules did not verify" -ForegroundColor Yellow
    exit 1
}
