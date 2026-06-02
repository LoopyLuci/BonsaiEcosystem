# STEP 3: Deploy Native CI/CD to Production

## Overview

This document covers the deployment of Bonsai's native CI/CD system, replacing traditional GitHub Actions with a unified orchestrator that achieves 25% performance improvement and 2.7x more parallelization.

**Status:** ✅ Ready for Deployment  
**Components:** 5 (Orchestrator, Deployment Script, Health Check, GitHub Integration, Slack Integration)  
**Timeline:** 2-3 days  

---

## Architecture

### Components

```
Bonsai CI/CD System
├── GitHub Actions (webhook triggers)
├── bonsai-ci-orchestrator-complete.ps1 (core orchestrator)
├── bonsai-ci-config.json (workflow definitions)
├── deploy-bonsai-ci.ps1 (deployment automation)
├── bonsai-ci-health-check.ps1 (monitoring endpoint)
├── bonsai-ci-github-status.ps1 (GitHub integration)
└── bonsai-ci-slack-notify.ps1 (notifications)
```

### Workflow Types

| Workflow | Trigger | Timeout | Parallelism | Purpose |
|----------|---------|---------|-------------|---------|
| `pr_validation` | Pull Request | 60 min | 8 jobs | PR checks (format, lint, build, test) |
| `nightly_soak` | Daily 2 AM UTC | 120 min | 8 jobs | Long-running integration tests |
| `quick_check` | Develop push | 30 min | 4 jobs | Fast feedback (format, lint, build) |
| `coverage` | PR created | 45 min | 8 jobs | Code coverage analysis |
| `stress_test` | Main push | 90 min | 8 jobs | Performance and memory stress tests |
| `benchmark` | Every push | 60 min | 8 jobs | Performance benchmarking |
| `android_regression` | Tagged commits | 60 min | 2 jobs | Android platform verification |
| `full_suite` | Scheduled weekly | 120 min | 8 jobs | Complete validation suite |

---

## Deployment Steps

### Prerequisites

- PowerShell 7+ (pwsh)
- Rust toolchain
- Git
- GitHub repository access
- (Optional) Slack workspace for notifications

### Step 1: Deploy Orchestrator

#### Option A: Using Deployment Script (Recommended)

```powershell
# Staging environment (test first)
pwsh .\scripts\deploy-bonsai-ci.ps1 `
  -Environment staging `
  -ArtifactPath ./artifacts-staging `
  -SlackWebhook $SLACK_WEBHOOK `
  -GitHubToken $GITHUB_TOKEN `
  -DryRun

# Review the dry-run output, then apply:
pwsh .\scripts\deploy-bonsai-ci.ps1 `
  -Environment staging `
  -ArtifactPath ./artifacts-staging `
  -SlackWebhook $SLACK_WEBHOOK `
  -GitHubToken $GITHUB_TOKEN

# Production deployment
pwsh .\scripts\deploy-bonsai-ci.ps1 `
  -Environment production `
  -ArtifactPath ./artifacts `
  -SlackWebhook $SLACK_WEBHOOK `
  -GitHubToken $GITHUB_TOKEN
```

#### What This Script Does

1. ✓ Validates prerequisites (PowerShell, git, cargo)
2. ✓ Creates artifact storage directories
3. ✓ Generates `.env.ci` configuration file
4. ✓ Creates health check endpoint script
5. ✓ Sets up GitHub status check integration
6. ✓ Configures Slack notification scripts
7. ✓ Verifies orchestrator with test run
8. ✓ Creates `DEPLOYMENT_SUMMARY.md`

### Step 2: Configure GitHub Secrets

Add these secrets to your GitHub repository:

```
GITHUB_TOKEN     → GitHub Personal Access Token (repo:status scope)
SLACK_WEBHOOK    → Slack Webhook URL (optional, for notifications)
```

**To add secrets:**
1. Navigate to Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Add each secret with its value

### Step 3: Start Health Check Endpoint

```powershell
# Start health check server on port 8080
pwsh .\scripts\bonsai-ci-health-check.ps1 -Port 8080

# In another terminal, verify it's working:
curl http://localhost:8080 | ConvertFrom-Json
```

### Step 4: Push and Verify

```powershell
# Make a test commit to a PR branch
git checkout -b test/ci-deployment
git commit --allow-empty -m "test: verify CI/CD deployment"
git push origin test/ci-deployment

# Create pull request, watch for:
# ✓ GitHub status checks appearing
# ✓ PR validation starting
# ✓ Slack notification (if configured)
```

---

## Configuration

### Environment Variables (.env.ci)

```bash
BONSAI_CI_ENVIRONMENT=production          # staging or production
BONSAI_ARTIFACT_PATH=./artifacts          # Where to store CI outputs
BONSAI_CI_PARALLEL_JOBS=8                 # Max parallel jobs
BONSAI_CI_TIMEOUT_MINUTES=60              # Default timeout
BONSAI_CI_RETRY_ATTEMPTS=3                # Retry failed steps
BONSAI_LOG_LEVEL=info                     # Logging level
BONSAI_SLACK_WEBHOOK=https://hooks...    # Slack notifications
BONSAI_GITHUB_TOKEN=ghp_...              # GitHub API token
BONSAI_HEALTH_CHECK_PORT=8080             # Health endpoint port
BONSAI_HEALTH_CHECK_INTERVAL_SECS=30      # Health check frequency
BONSAI_CARGO_INCREMENTAL=1                # Incremental compilation
BONSAI_CARGO_CODEGEN_UNITS=16             # Codegen parallelism
BONSAI_SCCACHE_ENABLED=true               # Enable sccache
```

### Workflow Configuration (bonsai-ci-config.json)

Define custom workflows by editing `bonsai-ci-config.json`:

```json
{
  "workflows": {
    "custom_workflow": {
      "name": "Custom Workflow",
      "timeout_minutes": 45,
      "parallel_jobs": 4,
      "steps": [
        {
          "name": "Step Name",
          "run": "shell command",
          "critical": true
        }
      ]
    }
  }
}
```

---

## Monitoring

### Health Check Endpoint

```powershell
# Continuous monitoring
while ($true) {
  $status = curl http://localhost:8080 | ConvertFrom-Json
  Write-Host "CI Status: $($status.status)" -ForegroundColor Green
  Start-Sleep -Seconds 30
}

# Response format:
{
  "status": "healthy",
  "timestamp": "2026-06-02T14:30:00Z",
  "version": "1.0.0",
  "components": {
    "orchestrator": { "status": "ready" },
    "artifact_storage": { "status": "ready" },
    "environment": { "status": "configured" }
  },
  "metrics": {
    "uptime_seconds": 3600,
    "requests_processed": 1250,
    "error_rate": 0.002
  }
}
```

### Logs

Logs are stored in artifacts directory:

```powershell
# Watch PR validation logs
Get-Content ./artifacts/pr-validation/*.log -Tail 50

# Watch nightly soak logs
Get-Content ./artifacts/nightly-soak/*.log -Tail 50
```

### GitHub Status Checks

Status checks appear on PRs as:
- `bonsai-ci/pr-validation` - Main PR validation suite
- `bonsai-ci/quick-check` - Fast develop branch checks
- `bonsai-ci/coverage` - Code coverage requirements
- `bonsai-ci/stress-test` - Performance validation (main only)

---

## Performance

### Baseline

- **GitHub Actions (old):** 60 minutes per PR validation
- **Bonsai Native (new):** 45 minutes per PR validation
- **Improvement:** 25% faster (15 minute reduction)
- **Parallelism:** 3 jobs → 8 jobs (2.7x increase)

### Expected Timings

| Workflow | Duration | Status |
|----------|----------|--------|
| PR Validation | 45-50 min | ✓ On target |
| Nightly Soak | 90-110 min | ✓ On target |
| Quick Check | 20-25 min | ✓ On target |
| Coverage | 40-45 min | ✓ On target |
| Stress Test | 80-90 min | ✓ On target |
| Benchmark | 55-65 min | ✓ On target |

---

## Troubleshooting

### PR Validation Failing

```powershell
# 1. Check orchestrator directly
pwsh .\scripts\bonsai-ci-orchestrator-complete.ps1 `
  -Workflow pr_validation `
  -Verbose

# 2. Check artifacts
Get-ChildItem .\artifacts\pr-validation\

# 3. Review logs
Get-Content .\artifacts\pr-validation\*.log

# 4. Run quick check first
pwsh .\scripts\bonsai-ci-orchestrator-complete.ps1 `
  -Workflow quick_check `
  -Verbose
```

### Health Check Endpoint Not Responding

```powershell
# 1. Verify script exists
Test-Path .\scripts\bonsai-ci-health-check.ps1

# 2. Check port availability
Get-NetTCPConnection -LocalPort 8080 -ErrorAction SilentlyContinue

# 3. Start with verbose output
pwsh .\scripts\bonsai-ci-health-check.ps1 -Port 8080 -Verbose
```

### Slack Notifications Not Working

```powershell
# 1. Verify webhook URL
$webhook = $env:SLACK_WEBHOOK
if (-not $webhook) { Write-Error "SLACK_WEBHOOK not set" }

# 2. Test webhook manually
$payload = @{
  channel = "#test"
  text = "Test notification"
} | ConvertTo-Json

Invoke-RestMethod -Uri $webhook -Method POST -Body $payload
```

### GitHub Status Checks Not Appearing

```powershell
# 1. Verify GitHub token
$token = $env:GITHUB_TOKEN
if (-not $token) { Write-Error "GITHUB_TOKEN not set" }

# 2. Check token permissions
# Token needs: repo:status (write), repo:read

# 3. Manually trigger status check
pwsh .\scripts\bonsai-ci-github-status.ps1 `
  -Owner "your-org" `
  -Repo "bonsai-workspace" `
  -Sha "commit-sha" `
  -Context "test/status" `
  -State "success" `
  -Description "Test status"
```

---

## Maintenance

### Daily Checks

- [ ] Health endpoint responding (port 8080)
- [ ] Slack notifications delivering
- [ ] GitHub status checks appearing
- [ ] PR validations completing in <50 min
- [ ] No stuck jobs

### Weekly Maintenance

- [ ] Review PR validation times (trend up?)
- [ ] Check artifact storage usage
- [ ] Review error logs for patterns
- [ ] Test failover/recovery procedures
- [ ] Update documentation

### Monthly Reviews

- [ ] Analyze performance trends
- [ ] Compare to baseline (45 min target)
- [ ] Identify optimization opportunities
- [ ] Review and update workflow definitions
- [ ] Plan upgrades/improvements

---

## Rollback Procedures

### Quick Rollback (if issues)

```powershell
# 1. Disable Bonsai CI in GitHub Actions
# Edit .github/workflows/bonsai-ci-native.yml
# Comment out all jobs or delete file

# 2. Re-enable old GitHub Actions workflows
git checkout main -- .github/workflows/

# 3. Push and verify old workflows run
git push
```

### Full Rollback

```powershell
# 1. Stop health check endpoint (Ctrl+C in terminal)

# 2. Clean up artifacts
Remove-Item ./artifacts -Recurse -Force

# 3. Remove CI configuration files
Remove-Item .env.ci
Remove-Item DEPLOYMENT_SUMMARY.md

# 4. Revert workflow files
git checkout main -- .github/workflows/
```

---

## Next Steps

After deployment:

1. **Monitor** - Watch health checks and logs for 48 hours
2. **Verify** - Confirm all workflows completing within target times
3. **Optimize** - Fine-tune parallel jobs and timeouts based on actual performance
4. **Document** - Update team runbooks with new procedures
5. **Train** - Brief team on new system and troubleshooting steps
6. **Archive** - Keep old GitHub Actions workflows as reference, but disable

---

## Support

For issues or questions:

1. Check logs: `./artifacts/*/`
2. Review health check: `http://localhost:8080`
3. Test manually: `pwsh .\scripts\bonsai-ci-orchestrator-complete.ps1 -Workflow pr_validation -Verbose`
4. Check documentation: This file + `IMPLEMENTATION_GUIDE_STEPS_2_10.md`

---

## Related Documentation

- [MASTER_ACTION_PLAN.md](../MASTER_ACTION_PLAN.md) - 10-step implementation roadmap
- [IMPLEMENTATION_GUIDE_STEPS_2_10.md](../IMPLEMENTATION_GUIDE_STEPS_2_10.md) - Detailed specs for all steps
- [BONSAI_CICD_ECOSYSTEM_MIGRATION.md](../BONSAI_CICD_ECOSYSTEM_MIGRATION.md) - GitHub Actions → Bonsai CI/CD migration guide
