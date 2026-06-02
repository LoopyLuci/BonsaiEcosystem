# Master Build Script: Build All Teams in Parallel
# Executes all 11 team builds concurrently using PowerShell jobs

$ErrorActionPreference = "Stop"

$teams = @(
    @{ name = "A"; crate = "bonsai-bedf-fuzzing"; desc = "Fuzzing Engine" },
    @{ name = "B"; crate = "bonsai-bedf-concurrency"; desc = "Concurrency Testing" },
    @{ name = "C"; crate = "bonsai-bedf-sanitizers"; desc = "Memory Sanitizers" },
    @{ name = "D"; crate = "bonsai-bedf-property"; desc = "Property Testing" },
    @{ name = "E"; crate = "bonsai-bedf-pentest"; desc = "Penetration Testing" },
    @{ name = "F"; crate = "bonsai-bedf-sandbox"; desc = "Sandbox Orchestration" },
    @{ name = "G"; crate = "bonsai-bedf-triage"; desc = "Triage & AI" },
    @{ name = "H"; crate = "bonsai-bedf-mcp"; desc = "MCP Tools" },
    @{ name = "I"; crate = "bonsai-bedf-enhancements"; desc = "Advanced Enhancements" },
    @{ name = "J"; crate = "bonsai-survival-system-ext"; desc = "Survival System" },
    @{ name = "K"; crate = "bonsai-kdb-ext"; desc = "Knowledge Database" }
)

Write-Host "🚀 BEDF Parallel Build System" -ForegroundColor Cyan
Write-Host "=============================" -ForegroundColor Cyan
Write-Host "Building all 11 teams in parallel..." -ForegroundColor Yellow
Write-Host ""

$startTime = Get-Date
$jobs = @()

# Start all jobs in parallel
foreach ($team in $teams) {
    Write-Host "Team $($team.name): Starting build for $($team.desc)..." -ForegroundColor Cyan

    $job = Start-Job -ScriptBlock {
        param($team)
        cd Z:\Projects\BonsaiWorkspace

        $result = @{
            team = $team.name
            crate = $team.crate
            desc = $team.desc
            buildPass = $false
            testPass = $false
            lintPass = $false
            errors = @()
        }

        # Build
        $buildOutput = cargo build --package $team.crate --release 2>&1
        if ($LASTEXITCODE -eq 0) {
            $result.buildPass = $true
        } else {
            $result.errors += "Build failed: $buildOutput"
        }

        # Test
        $testOutput = cargo test --package $team.crate --release 2>&1
        if ($LASTEXITCODE -eq 0) {
            $result.testPass = $true
        } else {
            $result.errors += "Tests failed: $testOutput"
        }

        # Lint
        $lintOutput = cargo clippy --package $team.crate -- -D warnings 2>&1
        if ($LASTEXITCODE -eq 0) {
            $result.lintPass = $true
        }

        return $result
    } -ArgumentList $team

    $jobs += $job
}

Write-Host ""
Write-Host "⏳ Waiting for all builds to complete..." -ForegroundColor Yellow
Write-Host ""

# Wait for all jobs and collect results
$results = @()
$failureCount = 0

foreach ($job in $jobs) {
    $result = Wait-Job -Job $job | Receive-Job
    $results += $result

    $status = "✅"
    if (-not $result.buildPass -or -not $result.testPass) {
        $status = "❌"
        $failureCount++
    }

    Write-Host "$status Team $($result.team): $($result.desc) - Build: $(if ($result.buildPass) {'✅'} else {'❌'}) Test: $(if ($result.testPass) {'✅'} else {'❌'}) Lint: $(if ($result.lintPass) {'✅'} else {'⚠️ '})" -ForegroundColor $(if ($result.buildPass -and $result.testPass) {'Green'} else {'Red'})

    if ($result.errors.Count -gt 0) {
        foreach ($error in $result.errors) {
            Write-Host "   Error: $error" -ForegroundColor Red
        }
    }
}

$duration = (Get-Date) - $startTime

Write-Host ""
Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         PARALLEL BUILD SUMMARY          ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host "Total Teams: $($teams.Count)" -ForegroundColor White
Write-Host "Passed: $($teams.Count - $failureCount)" -ForegroundColor Green
Write-Host "Failed: $failureCount" -ForegroundColor $(if ($failureCount -eq 0) {'Green'} else {'Red'})
Write-Host "Duration: $([math]::Round($duration.TotalSeconds, 2))s" -ForegroundColor Cyan
Write-Host ""

if ($failureCount -eq 0) {
    Write-Host "✅ All teams built successfully!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "❌ Some teams failed. Check errors above." -ForegroundColor Red
    exit 1
}
