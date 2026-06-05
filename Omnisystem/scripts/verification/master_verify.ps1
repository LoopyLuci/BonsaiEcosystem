# master_verify.ps1
# Master verification script — runs all phase verifications in sequence.
# Usage: .\scripts\verification\master_verify.ps1

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  OMNISYSTEM MASTER VERIFICATION" -ForegroundColor Cyan
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan

# Build bootstrap once
Write-Host "`n[BUILD] Building bootstrap interpreter..." -ForegroundColor Yellow
Push-Location $root
cargo build --release --manifest-path titan-bootstrap/Cargo.toml 2>&1 | Select-Object -Last 3
Pop-Location

$exe = "$root\titan-bootstrap\target\release\titan-bootstrap.exe"
$totalPassed = 0
$totalFailed = 0

function Test-Module {
    param($Name, $File, $Expected)
    $output = & $exe $File --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        $status = if ($value -eq $Expected) { "✓" } else { "⚠" }
        $color = if ($value -eq $Expected) { "Green" } else { "Yellow" }
        Write-Host "  $status $Name → $value" -ForegroundColor $color
        $script:totalPassed++
    } else {
        Write-Host "  ✗ $Name → FAILED" -ForegroundColor Red
        $script:totalFailed++
    }
}

# Core system
Write-Host "`n━━━ CORE SYSTEM ━━━" -ForegroundColor Cyan
Test-Module "compiler.ti" "$root/titan/compiler/compiler.ti" "42"
Test-Module "test_full_self_compile.ti" "$root/tests/test_full_self_compile.ti" "42"
Test-Module "test_actor_runtime.ti" "$root/tests/test_actor_runtime.ti" "111"
Test-Module "test_fabric_complete.ti" "$root/tests/test_fabric_complete.ti" "111"

# Production hardening
Write-Host "`n━━━ PRODUCTION HARDENING ━━━" -ForegroundColor Cyan
Test-Module "native_bootstrap.ti" "$root/titan/compiler/native_bootstrap.ti" "105"
Test-Module "omni_production.ti" "$root/titan/toolchain/omni_production.ti" "110"
Test-Module "multi_node.ti" "$root/aether/runtime/multi_node.ti" "111"
Test-Module "hardening.ti" "$root/titan/security/hardening.ti" "111"

# Aion AI
Write-Host "`n━━━ AION AI ━━━" -ForegroundColor Cyan
Test-Module "deployment_pipeline.ti" "$root/titan/aion/deployment_pipeline.ti" "111"
Test-Module "self_improving_factory.ti" "$root/titan/aion/self_improving_factory.ti" "111"

# OmniModel Bridge
Write-Host "`n━━━ OMNIMODEL BRIDGE ━━━" -ForegroundColor Cyan
Test-Module "model_ingestion.ti" "$root/titan/omnibridge/model_ingestion.ti" "111"
Test-Module "weight_converter.ti" "$root/titan/omnibridge/weight_converter.ti" "111"

# Omni Studio IDE
Write-Host "`n━━━ OMNI STUDIO IDE ━━━" -ForegroundColor Cyan
Test-Module "studio_shell.ti" "$root/titan/studio/studio_shell.ti" "111"
Test-Module "trust_dashboard.ti" "$root/titan/studio/trust_dashboard.ti" "111"
Test-Module "deployment_manager.ti" "$root/titan/studio/deployment_manager.ti" "111"

$total = $totalPassed + $totalFailed
Write-Host "`n══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($totalFailed -eq 0) { "Green" } else { "Red" })
Write-Host "  MASTER VERIFICATION: $totalPassed/$total modules verified" -ForegroundColor $(if ($totalFailed -eq 0) { "Green" } else { "Red" })
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($totalFailed -eq 0) { "Green" } else { "Red" })
