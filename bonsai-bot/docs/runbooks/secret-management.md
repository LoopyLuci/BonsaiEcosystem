# Runbook: Token Lifecycle and Secret Management

## Secret storage model
All secrets are stored exclusively in the OS keychain (Windows Credential Manager on Windows,
Keychain on macOS). The keyring service name is `"bonsai-bot"`.

| Account key              | Content                                        |
|--------------------------|------------------------------------------------|
| `discord_token`          | Discord bot token                              |
| `telegram_token`         | Telegram bot token                             |
| `matrix_password`        | Matrix account password or access token        |
| `matrix_key_backup_pass` | Matrix key backup recovery passphrase          |
| `email_imap_password`    | IMAP app password                              |
| `email_smtp_password`    | SMTP app password                              |
| `bot_admin_token`        | Local admin API Bearer token (auto-generated)  |

**Nothing secret ever appears in `bonsai-bot-config.json`, environment variables, or logs.**

## First-time setup
On first start, `bot_admin_token` is auto-generated (UUID v4) and stored in the keychain.
Platform tokens must be entered via SettingsPanel → Messaging Bots before platforms activate.

## Retrieving the admin token (for curl access)
```powershell
# Windows PowerShell — read from Credential Manager
$cred = [System.Runtime.InteropServices.Marshal]
Add-Type -AssemblyName System.Security
$entry = New-Object System.Management.Automation.PSCredential(
  "bonsai-bot/bot_admin_token",
  (Get-StoredCredential -Target "bonsai-bot" -AsCredentialObject).Password | ConvertTo-SecureString -AsPlainText -Force
)
```
Or use the `keyring` CLI if installed:
```sh
keyring get bonsai-bot bot_admin_token
```

## Rotating the admin token
```sh
curl -X POST -H "Authorization: Bearer <current_token>" \
  http://127.0.0.1:11421/config/rotate-admin-token
# Response: {"status":"rotated"}
# Old token is immediately invalid. New token is in keychain.
```

## Revealing the Matrix key backup passphrase
The Matrix key backup passphrase is revealed only via the Tauri command
`get_matrix_key_backup_passphrase`, which requires supplying the current `bot_admin_token`
as proof. The passphrase is returned once and displayed in a modal — never stored in state
or emitted as a Tauri event. An audit log entry is written on each reveal.

If access to the passphrase is needed outside the UI:
1. Retrieve `bot_admin_token` from keychain (see above)
2. Call the Tauri command from the app

## Revoking a platform token
To fully disable a platform:
1. Clear the token in SettingsPanel → Messaging Bots → [Platform]: clear field → Save
2. The bot disables that platform on next `/config/reload` (or restart)
3. Optionally revoke the token on the platform side as well

## Security invariants
- Admin API binds to `127.0.0.1` only — no remote access
- All `/status`, `/sessions`, `/broadcast`, `/config/*` routes require Bearer token
- Only `/health` is unauthenticated
- Token rotation is logged to `tracing` at INFO level with no token value in the message
- Matrix key backup reveal is logged to the audit log (`assistant_audit_log`)
- Raw user message content is never written to logs — only platform, user ID, and rejection reason
