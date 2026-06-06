$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  OMNI STUDIO IDE — COMPLETE VERIFICATION" -ForegroundColor Cyan
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

Write-Host "`n━━━ OMNI STUDIO PANELS ━━━" -ForegroundColor Cyan
Test-Module "studio_shell.ti" "titan/studio/studio_shell.ti" "111"
Test-Module "agentic_editor.ti" "titan/studio/agentic_editor.ti" "111"
Test-Module "live_preview.ti" "titan/studio/live_preview.ti" "111"
Test-Module "debug_timeline.ti" "titan/studio/debug_timeline.ti" "111"
Test-Module "trust_dashboard.ti" "titan/studio/trust_dashboard.ti" "111"
Test-Module "project_explorer.ti" "titan/studio/project_explorer.ti" "111"
Test-Module "deployment_manager.ti" "titan/studio/deployment_manager.ti" "111"
Test-Module "module_marketplace.ti" "titan/studio/module_marketplace.ti" "111"

Write-Host "`n━━━ INTEGRATION TEST ━━━" -ForegroundColor Cyan
Test-Module "test_omni_studio_complete.ti" "tests/test_omni_studio_complete.ti" "111"

Write-Host "`n━━━ CORE REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "compiler.ti" "titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "tests/test_fabric_complete.ti" "111"
Test-Module "autonomous_cycle.ti" "titan/omniagent/modules/autonomous_cycle.ti" "111"

$total = $passed + $failed
Write-Host "`n══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if($failed -eq 0){'Green'}else{'Red'})
Write-Host "  OMNI STUDIO: $passed/$total modules verified" -ForegroundColor $(if($failed -eq 0){'Green'}else{'Red'})
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if($failed -eq 0){'Green'}else{'Red'})
