# Troubleshooting & FAQ

This guide covers the most common issues and how to resolve them. If your issue isn't here, check the Activity Log for details and open a [GitHub issue](https://github.com/LoopyLuci/BonsaiWorkspace/issues).

---

## Common Issues

### Port conflicts

**Symptom**: `Error: port 11369 already in use` or `address already in use (os error 98)`

**Cause**: A previous Bonsai instance or another process is occupying the port.

**Fix**:
```powershell
# Windows — find and kill the process on the port
netstat -ano | findstr :11369
taskkill /PID <pid> /F

# Linux / macOS
lsof -ti:11369 | xargs kill -9
```

Bonsai will automatically try the next available port if the default is busy. If this happens repeatedly, the Survival System will learn to handle it automatically.

---

### GPU not detected

**Symptom**: Model loads slowly, GPU shows 0% usage, or `CUDA error: no kernel image for this device`

**Cause**: Missing or mismatched GPU drivers.

**Fix**:
1. Verify your GPU is supported: NVIDIA (CUDA 12+), AMD (ROCm 6+ on Linux), Apple Silicon (Metal).
2. Check driver: `nvidia-smi` (NVIDIA) or `rocminfo` (AMD).
3. In Settings → Models, set **GPU Layers** to a positive number (e.g., 35 for a 7B model on 8 GB VRAM).
4. Check `llama-server` logs in the Activity Log (filter to `model` category).

If the GPU still isn't detected, try running with `RUST_LOG=debug just run` and look for `CUDA` in the logs.

---

### Training fails with ACCESS_VIOLATION or SIGSEGV

**Symptom**: Training job crashes mid-epoch with an access violation (Windows) or segmentation fault (Linux/macOS).

**Cause**: Batch size or sequence length too large for available RAM/VRAM.

**Fix** — reduce these values in `config/training.yaml`:

```yaml
training:
  max_length: 128      # reduce from default 512
  batch_size: 1        # reduce from default 4
  gradient_accumulation_steps: 8  # increase to compensate
```

On CPU-only machines, also add `--no-cuda` to the trainer invocation.

---

### Model download fails

**Symptom**: "Failed to download model" or timeout during model download.

**Fix**:
1. Check your internet connection.
2. Verify the model path in `config/model_registry.yaml` uses the correct HuggingFace repo ID.
3. If HuggingFace is blocked in your region, set a mirror:
   ```powershell
   $env:HF_ENDPOINT = "https://hf-mirror.com"
   ```
4. Use the **offline import** option: download the GGUF manually and drag it into Settings → Models.

---

### Chat responses are very slow

**Symptom**: Tokens generate at < 1 token/second.

**Causes and fixes**:

| Cause | Fix |
|---|---|
| Model too large for VRAM | Use a smaller model or Q4_K_M quantisation |
| GPU layers = 0 | Set GPU Layers to max in Settings → Models |
| Context too long | Clear chat or reduce context window in Settings |
| Other apps using GPU | Close GPU-intensive apps (games, video editors) |
| Thermal throttling | Check GPU temperature; improve cooling |

Run `just bench` to see tokens/second for your current configuration.

---

### Collaboration — can't find peer

**Symptom**: Entering an invitation code gives "Invalid or expired invitation code" or "Connection timeout".

**Fix**:
1. Make sure both devices are on the same LAN **or** the host device has a relay configured.
2. Check the host's firewall allows UDP on port 14000–14100 (TransferDaemon's default range).
3. On Windows, allow Bonsai through Windows Defender Firewall.
4. Try the relay fallback: in Settings → Transfer → Relay Server, set `wss://relay.bonsai.local` (run `bonsai-relay` on a local server).

---

### WebSocket connection refused

**Symptom**: VSCode extension or external tool reports "Connection refused" on the daemon WebSocket.

**Fix**:
```powershell
# Start the headless daemon
just run-daemon

# Check it's listening
netstat -ano | findstr 14999
```

The daemon port is written to `~/.bonsai/vscode_port`. The authentication token is in `~/.bonsai/vscode_token`.

---

### Svelte check errors after pulling changes

**Symptom**: `npx svelte-check` reports type errors after a `git pull`.

**Fix**:
```powershell
cd bonsai-workspace
pnpm install   # update deps
npx svelte-check
```

If errors persist, they may be intentional (a work-in-progress PR). Check the branch's PR description.

---

### `cargo check --workspace` fails after pulling

**Symptom**: Compile errors after pulling the latest main branch.

**Fix**:
```powershell
cargo clean    # clear the build cache
cargo check --workspace
```

If the error is in a generated file (e.g., `gen/schemas/`), run:
```powershell
cargo tauri dev --no-watch   # regenerates Tauri schemas
```

---

## Reading the Activity Log for Debugging

1. Open the Activity Log panel (bottom toolbar, clock icon).
2. Filter to **Errors** by clicking the red ● toggle.
3. Find the relevant event. Click it to expand the full JSON payload.
4. The `command` field tells you which Tauri command was called. The `error` field has the Rust error message.
5. Copy the JSON and paste into a GitHub issue when asking for help.

---

## Manually Triggering Survival Repairs

If auto-repair isn't kicking in:

```powershell
# Run a survival scan from the terminal
just survival-repair

# Or via the Tauri command (from browser console in dev mode):
window.__TAURI__.core.invoke('repair_error', { errorMessage: 'your error here' })

# List all known repair rules
just survival-list

# Add a new rule manually
just survival-learn --symptom "..." --fix "..."
```

---

## Log Files

When opening a GitHub issue, please include:

| File | Location | Contains |
|---|---|---|
| App log | `~/.bonsai/app.log` | Rust backend logs |
| Activity log | Exported from Activity Log panel | UI and command events |
| Training log | `~/.bonsai/training/latest.log` | Training output |
| Watchdog log | `~/.bonsai/watchdog.log` | Crash and repair history |

---

## Getting Help

- **GitHub Issues**: [github.com/LoopyLuci/BonsaiWorkspace/issues](https://github.com/LoopyLuci/BonsaiWorkspace/issues)
- **GitHub Discussions**: for questions, ideas, and general discussion
- **Ask BonsAI directly**: BonsAI has knowledge of its own codebase — try asking it to diagnose your issue

For security vulnerabilities, see [11-SECURITY.md §Vulnerability Reporting](11-SECURITY.md).

---

*← [Developer Guide](12-DEVELOPER.md) · [Glossary →](14-GLOSSARY.md)*
