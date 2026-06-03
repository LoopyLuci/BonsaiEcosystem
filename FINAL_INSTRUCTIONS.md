# 🚀 GPU TRAINING BUILD — FINAL INSTRUCTIONS

**BUILD STATUS:** ✅ **RUNNING NOW** (Task: bnolehqo3)  
**Location:** Z:\Projects\BonsaiWorkspace  
**Hardware:** Ryzen 9 5900X, RX 7900 XTX (24GB VRAM), 64GB RAM

---

## 📊 WATCH BUILD LIVE

### **Option 1: Real-Time Output (Recommended)**

Open PowerShell and run:

```powershell
Get-Content C:\Users\limpi\AppData\Local\Temp\claude\z--Projects-BonsaiWorkspace\c7ae2a7a-5206-469e-8d6b-97fc5255ee90\tasks\bnolehqo3.output -Wait
```

This streams everything as it happens, including:
- Kernel compilation
- IDE building
- **GPU training output with loss values**
- Model conversion

### **Option 2: Check Individual Logs**

```powershell
# After each phase, check logs:
Get-Content Z:\Projects\BonsaiWorkspace\phase1.log -Tail 20   # Kernel
Get-Content Z:\Projects\BonsaiWorkspace\phase2.log -Tail 20   # IDE
Get-Content Z:\Projects\BonsaiWorkspace\phase3.log -Tail 20   # Data
Get-Content Z:\Projects\BonsaiWorkspace\training.log -Wait    # GPU Training
Get-Content Z:\Projects\BonsaiWorkspace\merge-convert.log -Tail 20
```

---

## 🎮 MONITOR GPU DURING TRAINING

**When Phase 4 starts** (4-6 hours from now):

1. **Ctrl+Shift+Esc** (Open Task Manager)
2. **Click "Performance" tab**
3. **Click "GPU"** on the left
4. **Watch RX 7900 XTX:**
   - Utilization: 85-95% ✓
   - VRAM: 18-22 GB / 24 GB ✓
   - Temperature: 60-75°C ✓
   - Power: ~250W ✓

---

## 📈 TRAINING PROGRESS

### **Expected Loss Progression**

When you see training output like this, **loss should steadily decrease**:

```
Step 10/600: loss=4.23, lr=2e-04   ← High at start
Step 20/600: loss=3.87, lr=2e-04   ← Improving
Step 30/600: loss=3.65, lr=2e-04   ← Still improving
...
Step 600/600: loss=1.23, lr=2e-04  ← Final (well trained)
```

**Good sign:** Loss goes DOWN over time  
**OK sign:** Loss stays roughly flat  
**Rare:** Loss goes UP (unusual but model still works)

---

## ⏱️ TIMELINE

```
NOW:           Phase 1 (Kernel) - 5 min
+5 min:        Phase 2 (IDE) - 15 min
+20 min:       Phase 3 (Data) - 1-2 hours
+1.5-2.5 hrs:  Phase 4 (GPU TRAINING) - 4-6 hours ⚡
+5.5-8.5 hrs:  Phase 5 (Merge) - 30 min
+6-10 hrs:     Phase 6 (IDE Launch) - IDE opens! 🎉
```

**Total: 6-10 hours**

---

## ✅ SUCCESS CHECKLIST

### During Build

- [ ] See "Python 3.11.9" at start
- [ ] Phase 1: Kernel compilation completes
- [ ] Phase 2: IDE builds and compiles
- [ ] Phase 3: "Data preparation complete"
- [ ] Phase 4: See "Step 10/600: loss=4.23..." appear
- [ ] Phase 4: GPU shows 85-95% in Task Manager
- [ ] Phase 4: Loss decreases over 600 steps
- [ ] Phase 4: "Training complete!"
- [ ] Phase 5: "Model ready: psychopathy-octopus-v1.Q4_K_M.gguf"

### After Build

- [ ] Bonsai Workspace IDE opens (desktop window)
- [ ] Model selector shows 4 models
- [ ] Select "octopus-v1" (Psychopathy Octopus)
- [ ] Type test question in chat
- [ ] Get server-specific response

---

## 🧪 TEST QUESTIONS (After IDE Opens)

Try these to verify training worked:

```
Q: What containers are running on the Octopus server?
Expected: Server-specific container list

Q: How do I safely restart a Docker container?
Expected: docker restart <name> or docker-compose restart <service>

Q: What are the security vulnerabilities?
Expected: References to actual incidents and CVEs from knowledge module

Q: Explain NixOS configuration
Expected: Detailed explanation of flake.nix and modules
```

---

## 📂 BUILD OUTPUT FILES

All phases save logs:

```
Z:\Projects\BonsaiWorkspace\
├── phase1.log           (Kernel build)
├── phase2.log           (Rust crates)
├── phase2-frontend.log  (Tauri build)
├── phase3.log           (Data preparation)
├── training.log         (GPU training output) ← Main event
├── merge-convert.log    (GGUF conversion)
└── full-build.log       (Complete output)
```

---

## 🚨 If Something Goes Wrong

### Build Seems Hung for >15 Minutes

**Check logs:**
```powershell
Get-Content C:\Users\limpi\AppData\Local\Temp\claude\z--Projects-BonsaiWorkspace\c7ae2a7a-5206-469e-8d6b-97fc5255ee90\tasks\bnolehqo3.output | Select-Object -Last 100
```

**Common causes:**
- Rust compilation (can take 10+ min for large crates)
- Downloading Python packages (5+ min)
- GPU warming up (1-2 min)

### GPU Training Slow or Not Using GPU

**Check Task Manager:**
- GPU utilization should be 85-95% during Phase 4
- If <50%: GPU issue, check driver
- If 0%: Verify RX 7900 XTX appears in Performance tab

### IDE Won't Launch

```powershell
# Try manual launch:
cd Z:\Projects\BonsaiWorkspace\bonsai-workspace
pnpm tauri dev
```

---

## 💬 WHAT TO DO AFTER BUILD

### 1. Test Psychopathy Octopus

In the IDE chat:
- Ask server management questions
- Test code-related queries
- Verify responses reference server knowledge

### 2. Collect Feedback (Optional)

- React with 👍 / 👎 to responses
- This trains future improvements

### 3. Schedule Nightly Improvement (Optional)

```powershell
# Auto-train every night at 3 AM:
$action = New-ScheduledTaskAction -Execute "pwsh" -Argument "-File Z:\Projects\BonsaiWorkspace\scripts\improve-octopus.ps1"
$trigger = New-ScheduledTaskTrigger -Daily -At 3am
Register-ScheduledTask -TaskName "OctopusAI-Improvement" -Action $action -Trigger $trigger
```

### 4. Deploy to Server (When Ready)

```bash
# Copy trained model to friend's server:
scp Z:\Projects\BonsaiWorkspace\psychopathy-octopus-v1.Q4_K_M.gguf user@server:/var/lib/bonsai/models/

# On server, restart the inference:
bonsai-api-gateway --model /var/lib/bonsai/models/psychopathy-octopus-v1.Q4_K_M.gguf
```

---

## 🎯 RIGHT NOW

```
Do this:

1. Open PowerShell

2. Run this to watch build:
   Get-Content C:\Users\limpi\AppData\Local\Temp\claude\z--Projects-BonsaiWorkspace\c7ae2a7a-5206-469e-8d6b-97fc5255ee90\tasks\bnolehqo3.output -Wait

3. When you see "Step 10/600: loss=4.23...", open Task Manager:
   Ctrl+Shift+Esc → Performance → GPU
   
4. Watch GPU usage climb to 85-95%

5. Check back in 6-10 hours!
   IDE will open automatically when done.
```

---

## 📞 QUICK REFERENCE

| What | Command |
|------|---------|
| Watch live build | `Get-Content ...bnolehqo3.output -Wait` |
| Check GPU status | Task Manager → Performance → GPU |
| View training log | `Get-Content Z:\...\training.log -Wait` |
| Manual IDE launch | `cd bonsai-workspace && pnpm tauri dev` |
| View model file | `ls Z:\...\psychopathy-octopus-v1.Q4_K_M.gguf` |

---

## 🚀 BUILD IS RUNNING NOW

**Task ID:** bnolehqo3  
**Status:** ✅ Active  
**Next:** Phase 1 (Kernel) running  
**Watch:** Get-Content ...bnolehqo3.output -Wait

**Come back in 30 minutes for Phase 2, or check in 6-10 hours for IDE launch!** ⚡

---

All build output is being saved. The training will complete automatically. When done, IDE opens and Psychopathy Octopus is ready for testing.

**Go watch the GPU train!** 🎮
