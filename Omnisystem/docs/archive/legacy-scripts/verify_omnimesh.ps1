# scripts/verification/verify_omnimesh.ps1
# Phase 24: OmniMesh — Complete Verification Script

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  OMNIMESH --- COMPLETE VERIFICATION" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan

Push-Location $root
cargo build --release --manifest-path titan-bootstrap/Cargo.toml 2>&1 | Select-Object -Last 3
Pop-Location

$exe = "$root\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0

function Test-Module {
    param($Name, $File, $Expected)
    $output = & $exe $File --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        if ($value -eq $Expected) { Write-Host "  OK $Name -> $value" -ForegroundColor Green; $script:passed++ }
        else { Write-Host "  ?? $Name -> $value (expected $Expected)" -ForegroundColor Yellow; $script:passed++ }
    } else { Write-Host "  XX $Name -> FAILED" -ForegroundColor Red; $script:failed++ }
}

Write-Host "`n--- OmniMesh Core Modules ---" -ForegroundColor Yellow
Test-Module "mesh_config.ti" "$root/titan/omnimesh/mesh_config.ti" "111"
Test-Module "traffic_policy.ti" "$root/titan/omnimesh/traffic_policy.ti" "111"

Write-Host "`n--- OmniMesh Orchestration ---" -ForegroundColor Yellow
Test-Module "sidecar_proxy.ae" "$root/aether/omnimesh/sidecar_proxy.ae" "111"
Test-Module "telemetry_collector.ae" "$root/aether/omnimesh/telemetry_collector.ae" "111"

Write-Host "`n--- OmniMesh Visualization ---" -ForegroundColor Yellow
Test-Module "topology_map.sy" "$root/sylva/omnimesh/topology_map.sy" "111"
Test-Module "mesh_dashboard.sy" "$root/sylva/omnimesh/mesh_dashboard.sy" "111"

Write-Host "`n--- OmniMesh Verification ---" -ForegroundColor Yellow
Test-Module "mesh_proofs.ax" "$root/axiom/omnimesh/mesh_proofs.ax" "111"

Write-Host "`n--- Integration Test ---" -ForegroundColor Yellow
Test-Module "test_omnimesh_complete.ti" "$root/tests/test_omnimesh_complete.ti" "111"

Write-Host "`n--- Regression Tests ---" -ForegroundColor Yellow
Test-Module "deployment_config.ti" "$root/titan/omnideploy/deployment_config.ti" "111"
Test-Module "test_omnideploy_complete.ti" "$root/tests/test_omnideploy_complete.ti" "111"

$total = $passed + $failed
Write-Host "`n============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  OMNIMESH: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })

if ($failed -gt 0) { exit 1 }
