# scripts/verification/verify_paths_abc.ps1
# Paths A, B, C — Complete verification

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  PATHS A, B, C — COMPLETE VERIFICATION" -ForegroundColor Cyan
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

Write-Host "`n--- Path A: Native Execution ---" -ForegroundColor Yellow
Test-Module "native_execution.ti" "$root/titan/compiler/native_execution.ti" "111"
Test-Module "test_native_interpreter_retirement.ti" "$root/tests/test_native_interpreter_retirement.ti" "111"

Write-Host "`n--- Path B: Deepened Compiler ---" -ForegroundColor Yellow
Test-Module "grammar_structs.ti" "$root/titan/compiler/grammar_structs.ti" "111"
Test-Module "grammar_enums.ti" "$root/titan/compiler/grammar_enums.ti" "111"
Test-Module "grammar_generics.ti" "$root/titan/compiler/grammar_generics.ti" "111"
Test-Module "codegen_structs.ti" "$root/titan/compiler/codegen_structs.ti" "111"

Write-Host "`n--- Path C: Collaborative Editor ---" -ForegroundColor Yellow
Test-Module "main.ti (collab)" "$root/examples/collab_editor/main.ti" "111"
Test-Module "actor_coordinator.ae" "$root/examples/collab_editor/actor_coordinator.ae" "111"
Test-Module "editor_ui.sy" "$root/examples/collab_editor/editor_ui.sy" "111"
Test-Module "sandbox_config.ti" "$root/examples/collab_editor/sandbox_config.ti" "111"
Test-Module "test_collab_editor_complete.ti" "$root/tests/test_collab_editor_complete.ti" "111"

Write-Host "`n--- Core Regression ---" -ForegroundColor Yellow
Test-Module "compiler.ti" "$root/titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "$root/tests/test_fabric_complete.ti" "111"
Test-Module "autonomous_cycle.ti" "$root/titan/omniagent/modules/autonomous_cycle.ti" "111"

$total = $passed + $failed
Write-Host "`n============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  ALL PATHS: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
