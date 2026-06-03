# 🚀 START GPU TRAINING NOW — Watch It Happen in Real-Time

**Hardware:** Ryzen 9 5900X, 64GB RAM, RX 7900 XTX (24GB VRAM)  
**What's About to Happen:** Build USOS kernel, Bonsai IDE, and TRAIN Psychopathy Octopus on GPU  
**Duration:** 6-10 hours (4-6 hours is GPU training)  
**What You'll See:** Real-time training output showing loss decreasing as model learns

---

## ⚡ Quick Start (3 Steps)

### Step 1: Install Python 3.11+ Dependencies

**Copy & paste this into PowerShell:**

```powershell
cd Z:\Projects\BonsaiWorkspace
.\install-python.ps1
```

This will:
- Download and install Python 3.11.9
- Install PyTorch with DirectML (AMD GPU support)
- Install training dependencies (transformers, peft, datasets, etc.)
- Verify everything works

**Time:** 3-5 minutes

**Expected output when done:**
```
✅ All dependencies installed!
  ✓ PyTorch: 2.1.0
  ✓ Transformers: 4.35.0
  ✓ PEFT: 0.7.1
  ✓ Datasets: 2.15.0
✅ Python environment ready for GPU training!
```

---

### Step 2: Launch Complete GPU Build

**Copy & paste this into PowerShell:**

```powershell
cd Z:\Projects\BonsaiWorkspace
.\windows-gpu-build.ps1 -LaunchStack
```

This will execute all 6 phases:
1. **Build USOS kernel** (5 min)
2. **Build Bonsai IDE** (15 min)
3. **Prepare 1.6M training examples** (1-2 hours, CPU)
4. **Train on GPU** (4-6 hours, GPU) ← **YOU'LL SEE THIS HAPPEN**
5. **Merge LoRA + convert to GGUF** (30 min)
6. **Launch IDE** (automatic)

**Time:** 6-10 hours total

---

### Step 3: Watch GPU Training in Real-Time

**While the script is running:**

1. **Open Task Manager** (Ctrl+Shift+Esc)
2. **Click "Performance" tab**
3. **Click "GPU" on the left**
4. **Watch your RX 7900 XTX utilization:**
   - Should climb to 85-95% during training
   - VRAM usage: 18-22 GB
   - Temperature: 60-75°C (normal)

**What you'll see in PowerShell output:**

```
Step 10/600: loss=4.23, lr=2e-04
Step 20/600: loss=3.87, lr=2e-04
Step 30/600: loss=3.65, lr=2e-04
...
Step 600/600: loss=1.23, lr=2e-04
✅ Training complete!
```

Loss should **steadily decrease** as training progresses.

---

## 📊 Real-Time Monitoring

### During Training (Phase 4)

**GPU Usage:**
- Task Manager → Performance → GPU
- Watch utilization: Should be 85-95%
- VRAM: 18-22 GB out of 24 GB
- Power: ~250W from GPU

**CPU Usage:**
- 4 cores at 60-80% (data loading)
- 8 cores at <20% (other tasks)
- RAM: 12 GB peak (out of 64 GB)

**Temperature:**
- GPU: 60-75°C (safe)
- CPU: 50-65°C (safe)

### Expected Duration

- **Phase 1-3:** ~2 hours (mostly CPU)
- **Phase 4:** 4-6 hours (GPU training)
- **Phase 5:** 30 min (CPU merge)
- **Total:** 6-10 hours

---

## 📈 What the Training Output Means

### Loss Decreasing = Good!

```
Step 10/600: loss=4.23   ← Starting point (high loss)
Step 20/600: loss=3.87   ← Getting better
Step 30/600: loss=3.65   ← Still improving
...
Step 600/600: loss=1.23  ← Final loss (low = well trained)
```

**Goal:** Loss should drop from 4+ to <2 by step 600.

### If Loss Increases

- This is **rare** with LoRA training
- Indicates a learning rate problem
- Script will keep trying different settings
- It will still produce a working model

### If You See Errors

- **CUDA out of memory:** Reduce batch size in script
- **Connection timeout:** Internet issue, script will retry
- **Module not found:** Check install-python.ps1 completed successfully

---

## 🎯 After Training Completes

### 1. IDE Will Open Automatically

- Tauri desktop window opens
- Shows file explorer, code editor, chat panel
- Model selector dropdown in top bar

### 2. Test Psychopathy Octopus

**In the IDE chat panel:**

```
You: How do I restart a Docker container safely?

Octopus: To safely restart a Docker container:
1. Use 'docker restart <container-name>' for a graceful restart
2. Or with docker-compose: 'docker-compose restart <service>'
3. Always check logs first: 'docker logs <container-name>'
...
```

**It should know about your friend's server** (octopus-cortex, nginx, etc.)

### 3. Collect Feedback

- React with 👍 or 👎 to answers
- Feedback automatically collected for nightly improvement

### 4. (Optional) Schedule Nightly Improvement

Auto-train on feedback every night at 3 AM:

```powershell
$action = New-ScheduledTaskAction -Execute "pwsh" -Argument "-File Z:\Projects\BonsaiWorkspace\scripts\improve-octopus.ps1"
$trigger = New-ScheduledTaskTrigger -Daily -At 3am
Register-ScheduledTask -TaskName "OctopusAI-Improvement" -Action $action -Trigger $trigger
```

Or run manually:
```powershell
.\scripts\improve-octopus.ps1
```

---

## 🚨 If Something Goes Wrong

### Python Installation Failed

```powershell
# Manually download and install:
# https://www.python.org/downloads/release/python-3119/
# Check "Add Python to PATH" during installation
# Then run: .\install-python.ps1 again
```

### Training Fails

```powershell
# Check the training log:
Get-Content Z:\Projects\BonsaiWorkspace\training.log -Tail 50

# If GPU out of memory, try smaller batch size:
# Edit crates/octopus-ai/train_psychopathy.py
# Change: per_device_train_batch_size=4 → per_device_train_batch_size=1
```

### IDE Won't Launch

```powershell
# Try manual launch:
cd Z:\Projects\BonsaiWorkspace\bonsai-workspace
pnpm tauri dev
```

---

## ✅ Success Checklist

After build completes, you should have:

- ✅ `psychopathy-octopus-v1.Q4_K_M.gguf` (~600 MB)
- ✅ Bonsai Workspace IDE running
- ✅ Model selector showing 4 models
- ✅ Can chat with Octopus AI
- ✅ Model knows about server configuration
- ✅ Responses include server-specific details

---

## 📞 Need Help?

| Issue | Solution |
|-------|----------|
| Python not found | Run `.\install-python.ps1` first |
| GPU not detected | Check Task Manager → GPU shows RX 7900 XTX |
| Training too slow | Check GPU utilization; reduce batch size if <80% |
| Model response is generic | Knowledge module may not have loaded; restart IDE |
| IDE won't launch | Check `frontend-build.log` in workspace directory |

---

## 🎬 Ready? Start Now!

```powershell
# Step 1: Install Python
.\install-python.ps1

# Step 2: Start GPU training (watch it happen!)
.\windows-gpu-build.ps1 -LaunchStack

# Step 3: Wait for IDE to open, then test!
# "What containers run on the Octopus server?"
```

**6-10 hours from now, you'll have a fully trained Octopus AI running locally on your Windows machine.** 🚀

---

## 📝 Logs to Monitor

```powershell
# Watch training in real-time:
Get-Content Z:\Projects\BonsaiWorkspace\training.log -Wait

# Or check any log file:
Get-Content Z:\Projects\BonsaiWorkspace\usos-build.log
Get-Content Z:\Projects\BonsaiWorkspace\rust-build.log
Get-Content Z:\Projects\BonsaiWorkspace\prepare-data.log
Get-Content Z:\Projects\BonsaiWorkspace\merge-convert.log
```

---

## 🎉 Deployment Ready

Once testing is complete, deploy to your friend's server:

```bash
# On your Windows machine:
scp Z:\Projects\BonsaiWorkspace\psychopathy-octopus-v1.Q4_K_M.gguf user@server:/var/lib/bonsai/models/

# On the server:
bonsai-api-gateway --model /var/lib/bonsai/models/psychopathy-octopus-v1.Q4_K_M.gguf
```

**That's it! Your friend now has Psychopathy Octopus running on their NixOS server.** 🐙

---

**Let's go! Execute Step 1 now:** `.\install-python.ps1`
