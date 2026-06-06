# verify_all_parallel.ps1
# Complete verification of all parallel implementations.
# Run from z:\Projects\Omnisystem

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

# Rebuild bootstrap (needed if interpreter intrinsics were added)
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  REBUILDING BOOTSTRAP INTERPRETER" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
cargo build --release --manifest-path titan-bootstrap/Cargo.toml 2>&1 | Select-Object -Last 3

$exe = ".\titan-bootstrap\target\release\titan-bootstrap.exe"
$pass = 0
$fail = 0

# Priority 1: Native Compilation
Write-Host "`n[P1] Native Compilation Pipeline" -ForegroundColor Yellow
$result = & $exe titan/compiler/native_compile.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

$result = & $exe tests/test_native_binary_production.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

# Priority 2: Concurrent Actors
Write-Host "`n[P2] Concurrent Aether Actors" -ForegroundColor Yellow
$result = & $exe tests/test_real_actors_concurrent.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

# Priority 3: Interactive REPL
Write-Host "`n[P3] Interactive Sylva REPL" -ForegroundColor Yellow
$result = & $exe tests/test_repl_parser.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

$result = & $exe tests/test_repl_variables.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

$result = & $exe tests/test_repl_debugger.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

# Priority 4: OmniView
Write-Host "`n[P4] OmniView Visible UI" -ForegroundColor Yellow
$result = & $exe titan/omniview/render_visible.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

$result = & $exe tests/test_omniview_components.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

# Phase 9: Interpreter Retirement
Write-Host "`n[P9] Interpreter Retirement" -ForegroundColor Yellow
$result = & $exe tests/test_interpreter_retirement.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

# Real-World Example
Write-Host "`n[RW] Real-World Data Pipeline" -ForegroundColor Yellow
$result = & $exe tests/test_real_world_data_pipeline.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

# Core Regression
Write-Host "`n[REG] Core Regression" -ForegroundColor Yellow
$result = & $exe titan/compiler/compiler.ti --run 2>&1 | Select-String "Result:"
if ($result -match "42") { $pass++ } else { $fail++ }
Write-Host $result

$result = & $exe tests/test_fabric_complete.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

$result = & $exe tests/test_full_self_compile.ti --run 2>&1 | Select-String "Result:"
if ($result -match "42") { $pass++ } else { $fail++ }
Write-Host $result

$result = & $exe tests/test_actor_runtime.ti --run 2>&1 | Select-String "Result:"
if ($result -match "111") { $pass++ } else { $fail++ }
Write-Host $result

Write-Host "`n═══════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host "  $pass PASSED, $fail FAILED" -ForegroundColor $(if ($fail -eq 0) { "Green" } else { "Red" })
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Green
