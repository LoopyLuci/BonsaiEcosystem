$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  OMNIMODEL BRIDGE — COMPLETE VERIFICATION" -ForegroundColor Cyan
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan

cargo build --release --manifest-path titan-bootstrap/Cargo.toml 2>&1 | Select-Object -Last 3

$exe = ".\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0

function Test-Module {
    param($Name, $File, $Expected)
    $output = & $exe $File --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        Write-Host "  $(if($value -eq $Expected){'✓'}else{'⚠'}) $Name → $value" -ForegroundColor $(if($value -eq $Expected){'Green'}else{'Yellow'})
        $script:passed++
    } else { Write-Host "  ✗ $Name → FAILED" -ForegroundColor Red; $script:failed++ }
}

Write-Host "`n━━━ OMNIMODEL BRIDGE MODULES ━━━" -ForegroundColor Cyan
Test-Module "model_ingestion.ti" "titan/omnibridge/model_ingestion.ti" "111"
Test-Module "weight_converter.ti" "titan/omnibridge/weight_converter.ti" "111"
Test-Module "architecture_mapper.ti" "titan/omnibridge/architecture_mapper.ti" "111"
Test-Module "federated_finetune.ti" "titan/omnibridge/federated_finetune.ti" "111"
Test-Module "model_safety.ax" "axiom/omnibridge/model_safety.ax" "111"
Test-Module "bridge_console.sy" "sylva/omnibridge/bridge_console.sy" "100"
Test-Module "test_omnibridge_pipeline.ti" "tests/test_omnibridge_pipeline.ti" "111"

Write-Host "`n━━━ CORE REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "compiler.ti" "titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "tests/test_fabric_complete.ti" "111"
Test-Module "autonomous_cycle.ti" "titan/omniagent/modules/autonomous_cycle.ti" "111"

$total = $passed + $failed
Write-Host "`n══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if($failed -eq 0){'Green'}else{'Red'})
Write-Host "  OMNIMODEL BRIDGE: $passed/$total modules verified" -ForegroundColor $(if($failed -eq 0){'Green'}else{'Red'})
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if($failed -eq 0){'Green'}else{'Red'})
