# Runbook: Database Corruption Recovery

## Symptoms
- Bot fails to start with `SQLite open` or `DB migrate` panic
- Bot logs: `database disk image is malformed` or `file is not a database`
- Tauri event `bot-db-corrupt` fires

## Cause
`bonsai-bot.db` (SQLite WAL mode) has been corrupted. Common causes:
- Hard power loss during a write
- Disk full condition
- Antivirus quarantining the WAL file

## Immediate check
```sh
# Location: {OS config dir}/bonsai/bonsai-bot.db  (Windows: %APPDATA%\bonsai\bonsai-bot.db)
sqlite3 "$env:APPDATA\bonsai\bonsai-bot.db" "PRAGMA integrity_check;"
```
- `ok` → DB is fine; error is something else (check logs)
- `Parse error` or list of problems → DB is corrupt

## Resolution

### Option A — Recover what you can (preferred)
```powershell
$db = "$env:APPDATA\bonsai\bonsai-bot.db"
# 1. Stop bot
Stop-Process -Name bonsai-bot -Force -ErrorAction SilentlyContinue

# 2. Backup corrupt file
Copy-Item $db "$db.corrupt-$(Get-Date -Format 'yyyyMMdd-HHmmss')"

# 3. Attempt SQLite dump+restore
sqlite3 $db ".dump" | sqlite3 "$db.recovered"
Move-Item "$db.recovered" $db -Force

# 4. Verify
sqlite3 $db "PRAGMA integrity_check;"
```

### Option B — Fresh start (data loss: session mappings only)
```powershell
$db = "$env:APPDATA\bonsai\bonsai-bot.db"
Stop-Process -Name bonsai-bot -Force -ErrorAction SilentlyContinue
Rename-Item $db "$db.corrupt-$(Get-Date -Format 'yyyyMMdd-HHmmss')"
# Restart bot — it will create a fresh DB and migrate automatically
```

## Impact of fresh start
- **Session mappings** are lost: next message from each user creates a new Buddy session.
  Prior Buddy conversation history is stored in Bonsai's `assistant_store`, not here —
  that data is unaffected.
- **Pending confirmations** are lost: any unresolved confirmations are gone. Users
  who were waiting for approval will need to resend their original request.
- **Platform tokens** are in the OS keychain — unaffected.

## WAL file cleanup
If `bonsai-bot.db-wal` or `bonsai-bot.db-shm` exists alongside the DB, delete them
before starting recovery — they belong to the corrupt state.

## Prevention
- Ensure the disk where `%APPDATA%\bonsai\` lives has adequate free space (>500MB)
- Exclude `bonsai-bot.db` and its `-wal`/`-shm` siblings from real-time AV scanning
