# Rollout Criteria and Rollback Procedure

## Release Channels

| Channel | Binary source          | Purpose                                    |
|---------|------------------------|--------------------------------------------|
| Dev     | `cargo build` (debug)  | Rapid iteration; verbose logging; assertions enabled |
| Beta    | `cargo build --release`| Pre-release validation; selective platforms enabled |
| Stable  | `cargo build --release`| Full MVP; all platforms; migration-safe    |

## Promotion Gates

### Dev → Beta
- [ ] `cargo check --features default` and `--features all`: 0 errors
- [ ] `cargo test --features default`: all tests pass
- [ ] `cargo audit`: 0 high/critical CVEs
- [ ] `cargo clippy --features default -- -D warnings`: 0 errors
- [ ] Bot starts cleanly on target machine (DB migrates, admin API binds)
- [ ] At least one platform adapter confirmed working end-to-end (message in → reply out)
- [ ] Confirmation flow (approve + deny) verified on at least one platform

### Beta → Stable
All Dev→Beta gates, plus:
- [ ] All 4 MVP platforms (Discord, Telegram, Matrix, Email) receive and reply to messages
- [ ] Confirmation nonce replay prevention tested (stale button click ignored)
- [ ] Session continuity across bot restart confirmed
- [ ] Rate limiting: 11th message in 60s rejected
- [ ] Buddy circuit breaker: 5 failures → open; recovery within 35s after Buddy restarts
- [ ] Queue backpressure: burst > 1024 handled without crash
- [ ] `cargo audit --features all`: 0 high/critical CVEs
- [ ] All acceptance gates from §26 of the plan pass

## Rollback Procedure

### Immediate rollback (bot only)
1. Stop the running bot:
   ```powershell
   Stop-Process -Name bonsai-bot -Force
   ```
2. Replace `bonsai-bot.exe` with the previous release binary (keep a copy of each release in `releases/vX.Y.Z/`)
3. Restart via launcher or directly:
   ```powershell
   Start-Process bonsai-bot.exe -WindowStyle Hidden
   ```
4. Verify `/health` and `/status` return expected state.

### Database rollback
The SQLite schema uses additive migrations — columns are never removed, only added.
Rolling back the binary never requires rolling back the DB schema.
If a schema migration was destructive (should not happen under this policy), restore from backup:
```powershell
Stop-Process -Name bonsai-bot -Force
Copy-Item "$env:APPDATA\bonsai\bonsai-bot.db.backup-YYYYMMDD" "$env:APPDATA\bonsai\bonsai-bot.db" -Force
```

### Workspace rollback (Tauri app)
Bot Tauri commands are backward-compatible — a newer bot binary works with an older app
as long as admin API port discovery (via `bonsai-bot-port.json`) is intact.

## Configuration rollback
`bonsai-bot-config.json` is versioned via `schema_version`. If a new release writes an
incompatible config, the old binary ignores unknown fields (serde default). No action needed
for minor version bumps. For major version bumps (breaking field renames), a migration note
must accompany the release.

## Monitoring rollback success
```sh
# 1. Health check
curl http://127.0.0.1:11421/health

# 2. Status check
curl -H "Authorization: Bearer <token>" http://127.0.0.1:11421/status

# 3. Metrics baseline (all counters should be low / incrementing normally)
curl -H "Authorization: Bearer <token>" http://127.0.0.1:11421/metrics
```
