# Runbook: Buddy API Outage

## Symptoms
- Bot replies "⚠️ Bonsai is currently unavailable. Try again shortly." to all messages
- `/status` shows `"buddy_circuit": "open"` (or buddy_errors rising in `/metrics`)
- SettingsPanel may show circuit-open badge

## Cause
The Bonsai Buddy API (port 11420) is unreachable or returning repeated 5xx errors.
After 5 consecutive failures (configurable: `circuit_breaker.open_after_failures`) the
circuit breaker opens and all Buddy calls short-circuit immediately.

## Resolution

### Step 1 — Verify Buddy is actually down
```sh
curl http://127.0.0.1:11420/health
```
- `200 {"status":"ok"}` → Buddy is up; problem is transient or circuit hasn't closed yet.
- Connection refused / timeout → Buddy is down.

### Step 2 — Restart Buddy
From the Bonsai Workspace launcher or Tauri app, restart the assistant backend.
Or via PowerShell (if running standalone):
```powershell
# Find and restart the buddy process
Get-Process -Name "bonsai-workspace" | Stop-Process -Force
# Then relaunch via the normal launcher
.\Launch-BonsaiWorkspace.ps1
```

### Step 3 — Wait for circuit to close
The circuit probes Buddy every 30 seconds (`circuit_breaker.half_open_probe_secs`).
After one successful probe, the circuit closes and normal operation resumes automatically.
No bot restart required.

### Step 4 — Confirm recovery
```sh
# Check /metrics for buddy_errors stabilized and buddy_circuit_opens not incrementing:
curl -H "Authorization: Bearer <token>" http://127.0.0.1:11421/metrics
```

## Configuration
Circuit breaker thresholds in `bonsai-bot-config.json`:
```json
"circuit_breaker": {
  "open_after_failures": 5,
  "half_open_probe_secs": 30,
  "close_on_successes": 1
}
```
Increase `open_after_failures` if transient network hiccups are tripping the breaker.
Decrease `half_open_probe_secs` for faster recovery in stable environments.
