# Runbook: Platform Token Invalidation

## Symptoms
- A specific platform stops responding (Discord/Telegram/Matrix/Email)
- Bot logs show auth errors: `[discord] Token invalid`, `[telegram] Unauthorized 401`, etc.
- SettingsPanel platform badge shows red/disconnected
- `bot-token-invalid` Tauri event fires (badge in UI)
- `/status` shows platform state as `"error"` or `"disconnected"`

## Cause
The platform token stored in the OS keychain has been revoked, expired, or regenerated
on the platform side without updating the bot.

## Resolution

### Discord
1. Go to https://discord.com/developers/applications → your bot → Bot → Reset Token
2. Copy the new token
3. In SettingsPanel → Messaging Bots → Discord: paste new token → Save Discord
4. The bot config auto-reloads. Discord will reconnect automatically.

### Telegram
1. Message @BotFather on Telegram → `/mybots` → select your bot → API Token → Revoke current token
2. Copy the new token
3. In SettingsPanel → Messaging Bots → Telegram: paste new token → Save Telegram

### Matrix
1. Log into your Matrix account, revoke the bot device session
2. In SettingsPanel → Messaging Bots → Matrix: paste new password → Save Matrix
3. The bot will create a new device session and re-upload cross-signing keys

### Email (IMAP/SMTP)
1. Generate a new app-specific password in your email provider's security settings
2. In SettingsPanel → Messaging Bots → Email: paste new IMAP/SMTP passwords → Save Email

## Manual keychain update (CLI fallback)
If SettingsPanel is unavailable, update the keychain directly:
```powershell
# Windows — using the Credential Manager PowerShell module or keyring CLI
# The keyring service name is "bonsai-bot"
# Account names: discord_token, telegram_token, matrix_password, email_imap_password, email_smtp_password
```

## Admin token rotation
If the `bot_admin_token` needs rotation (e.g., suspected compromise):
```sh
curl -X POST -H "Authorization: Bearer <current_token>" \
  http://127.0.0.1:11421/config/rotate-admin-token
```
The new token is written to the keychain immediately and the old token is invalidated.
Retrieve the new token from the keychain (`bonsai-bot` service, `bot_admin_token` account).
