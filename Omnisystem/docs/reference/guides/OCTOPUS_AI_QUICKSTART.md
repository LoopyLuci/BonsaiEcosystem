# 🐙 Octopus AI + Bonsai Workspace — Quick Start Guide

**Get a working Octopus AI model running today with the Bonsai Workspace IDE.**

---

## What You're Getting

✅ **Octopus AI v1** — Expert server management and computer science assistant  
✅ **Bonsai Workspace IDE** — Native desktop application for development and AI chat  
✅ **Model Selector** — Switch between Octopus AI and other models instantly  
✅ **Incremental Learning** — Automatically improves from user feedback  
✅ **CPU-First** — Runs on standard hardware, no GPU required for inference  

---

## Prerequisites

- **Windows 10/11 or Linux or macOS**
- **Rust 1.96+** — `rustup update`
- **Node.js 20+** + **pnpm** — `npm install -g pnpm`
- **~50 GB free storage** (for code, models, builds)
- **8+ GB RAM** (32 GB recommended)

---

## Launch (5 Minutes)

### Option 1: Automated Build & Launch (Recommended)

```powershell
# In PowerShell, from BonsaiWorkspace directory:
.\build-and-launch.ps1
```

This will:
1. Check dependencies
2. Build Rust crates (first time: 20-30 min)
3. Build frontend
4. Launch Bonsai Workspace IDE
5. Model selector shows Octopus AI v1 (default)

### Option 2: Manual Steps

```bash
# Build Rust workspace
cargo build --release

# Build frontend
cd bonsai-workspace
pnpm install
pnpm build

# Launch IDE
pnpm tauri dev
```

---

## Using Octopus AI in the Workspace

1. **IDE Launches** — A native window opens with file explorer, code editor, and chat panel
2. **Select Model** — Click the model dropdown (top bar or sidebar)
3. **Choose Octopus AI v1** — The default intelligent server assistant
4. **Start Chatting** — Type questions like:
   - "How do I restart a Docker container?"
   - "Check my system's disk usage"
   - "Explain NixOS configuration"
   - "Debug this Python code"

---

## Model Selector

All available models appear in the dropdown:

- **octopus-v1** — Octopus AI (default)
- **llama-3-8b** — Base large language model
- **mistral-7b** — Fast, efficient model
- **neural-chat-7b** — Chat-optimized model

Switch models instantly with **hot-reload** — no restart needed.

---

## Configuration

Edit `~/.bonsai/config.toml` to customize:

```toml
[models]
default = "octopus-v1"          # Default model on startup
model_dir = "~/.bonsai/models"  # Where models are stored

[api]
port = 11425                    # API server port
inference_port = 4000           # Model inference port

[ui]
show_model_descriptions = true  # Show model details
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Tauri build fails on Linux | Install: `sudo apt install libwebkit2gtk-4.0-dev build-essential libssl-dev libgtk-3-dev` |
| Model not appearing | Check `~/.bonsai/models/` has `.bkp` files, restart IDE |
| Out of memory | Reduce batch size: set `max_cached_models = 1` in config |
| Slow inference | Use quantized model (q4_k_m); GPU inference coming soon |

---

## Continuous Improvement

Octopus AI improves automatically via **EternalTrainingLoop**:

### Nightly Automatic Improvement

The included `scripts/improve-octopus.ps1` script:
1. Collects user feedback/corrections
2. Fine-tunes LoRA adapters
3. Validates improved model
4. Hot-swaps if validation passes

Schedule it to run nightly:

```powershell
# Create Windows scheduled task (3 AM daily)
$action = New-ScheduledTaskAction -Execute "pwsh" -Argument "-File `"$(pwd)\scripts\improve-octopus.ps1`""
$trigger = New-ScheduledTaskTrigger -Daily -At 3am
Register-ScheduledTask -TaskName "OctopusAI-Improvement" -Action $action -Trigger $trigger
```

Or run manually anytime:

```powershell
.\scripts\improve-octopus.ps1
```

---

## Architecture

```
Bonsai Workspace (Tauri Desktop App)
  ├─ Model Selector (dropdown menu)
  │   └─ octopus-v1 (intelligent server assistant)
  │   └─ llama-3-8b (general model)
  │   └─ ... (other models)
  ├─ Code Editor (with syntax highlighting)
  ├─ File Explorer
  └─ AI Chat Panel
       └─ Real-time responses from selected model
       └─ Knowledge module for server context
       └─ Safety checks (asks for confirmation on destructive ops)
```

---

## First Steps

1. **Run build script** — `.\build-and-launch.ps1` (or manual steps above)
2. **Wait for IDE to launch** — Usually <10 seconds after build completes
3. **Select Octopus AI** — From model dropdown
4. **Ask a question** — "How do I check Docker logs?"
5. **Get expert response** — Based on trained knowledge + server context

---

## Next: Deploy on Server

When ready to deploy on your friend's NixOS server:

1. Export trained model — `cp models/octopus-v1.bkp <server>:/var/lib/bonsai/models/`
2. Deploy Nix flake — `nix flake update && nixos-rebuild switch`
3. Enable EternalTrainingLoop — Runs nightly, improves from real usage
4. Monitor via Universe dashboard — Real-time observability

---

## Performance Targets

✅ **Inference latency** — <500ms p95 (CPU)  
✅ **Memory** — <12 GB peak (with cache)  
✅ **Safety** — ≥99% compliance (refuses dangerous ops)  
✅ **Accuracy** — ≥95% on domain tests  

---

## Support

- **Logs** — Check `~/.bonsai/logs/` for error messages
- **Config** — Edit `~/.bonsai/config.toml`
- **Models** — Place `.bkp` files in `~/.bonsai/models/`
- **Feedback** — Use thumbs up/down in chat to improve the model

---

**Status**: ✅ Production-ready  
**Build Time**: 20-30 min (first time), <1 sec (incremental)  
**Model Size**: 3-8 GB (depending on quantization)  
**No GPU Required**: Optimized for CPU inference  

🚀 **Ready to go. Launch now and start testing Octopus AI!**
