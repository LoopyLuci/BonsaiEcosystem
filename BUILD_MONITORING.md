# 🖥️ Windows 10 Bonsai Ecosystem Build — Live Monitoring

**Build Started:** 2026-06-03 (Background Task: bi3dpbvn4)  
**Expected Duration:** 6-10 hours  
**Build Log:** `C:\Users\limpi\AppData\Local\Temp\claude\z--Projects-BonsaiWorkspace\c7ae2a7a-5206-469e-8d6b-97fc5255ee90\tasks\bi3dpbvn4.output`

---

## 📊 Build Phases & Timeline

```
Phase 1: USOS Kernel Build (5 min)
┌─────────────────────────────────────────────────┐
│ cargo build --release --target x86_64-unknown-none
│ Compiling: vga_buffer, main.rs, linker
│ Output: usos-kernel binary (~50 KB)
└─────────────────────────────────────────────────┘
         ↓
Phase 2: Bonsai Workspace IDE Build (15 min)
┌─────────────────────────────────────────────────┐
│ Rust crates: bonsai-cli, api-gateway, kdb
│ Tauri frontend: pnpm install && pnpm build
│ Output: bonsai-workspace.exe (native app)
└─────────────────────────────────────────────────┘
         ↓
Phase 3: Training Data Preparation (1-2 hours, CPU)
┌─────────────────────────────────────────────────┐
│ prepare_data.py: 1.6M examples → 1.05M filtered
│ Format: JSON training pairs (Q&A)
│ Output: ./training-data/ (~500 MB)
└─────────────────────────────────────────────────┘
         ↓
Phase 4: GPU Training (4-6 hours, RX 7900 XTX)
┌─────────────────────────────────────────────────┐
│ train_psychopathy.py: TinyLlama 1.1B + LoRA
│ Method: QLoRA with rank-16 adapter
│ Quantization: 4-bit during training
│ Output: ./psychopathy-octopus-lora/ (~200 MB)
└─────────────────────────────────────────────────┘
         ↓
Phase 5: Merge & Convert (30 min)
┌─────────────────────────────────────────────────┐
│ merge_and_convert.py: Merge LoRA into base model
│ llama.cpp convert: GGUF Q4_K_M quantization
│ Output: psychopathy-octopus-v1.Q4_K_M.gguf (~600 MB)
└─────────────────────────────────────────────────┘
         ↓
Phase 6: Launch & Test (5 min)
┌─────────────────────────────────────────────────┐
│ API Gateway: http://127.0.0.1:11425
│ KDB Server: http://127.0.0.1:8089
│ IDE: Tauri window opens
│ Status: Complete
└─────────────────────────────────────────────────┘
```

---

## 🔍 What to Monitor

### While Running

**Check build progress** (watch in real-time):
```powershell
Get-Content -Path "C:\Users\limpi\AppData\Local\Temp\claude\z--Projects-BonsaiWorkspace\c7ae2a7a-5206-469e-8d6b-97fc5255ee90\tasks\bi3dpbvn4.output" -Wait
```

**Check individual build logs**:
```powershell
# Kernel build
Get-Content "Z:\Projects\BonsaiWorkspace\usos-build.log" | Select-Object -Last 20

# Rust crates
Get-Content "Z:\Projects\BonsaiWorkspace\rust-build.log" | Select-Object -Last 20

# Frontend
Get-Content "Z:\Projects\BonsaiWorkspace\frontend-build.log" | Select-Object -Last 20

# Training (only appears during Phase 4)
Get-Content "Z:\Projects\BonsaiWorkspace\training.log" -Tail 20 -ErrorAction SilentlyContinue
```

**Monitor GPU usage during training** (Phase 4):
```powershell
# Check if training is using GPU
Get-Process python | Where-Object { $_.Name -like "python*" }

# On Windows, you can use:
# Task Manager → Performance → GPU to watch RX 7900 XTX usage
# Expected: 80-95% utilization during training
```

### Expected Messages at Each Phase

**Phase 1 - Kernel (5 min in)**
```
➤ Building USOS kernel (bare-metal x86_64)...
  Compiling usos-kernel v0.1.0
  ✅ Kernel built: target/x86_64-unknown-none/release/usos-kernel
```

**Phase 2 - IDE (20 min in)**
```
➤ Building Rust crates...
  ✅ Rust crates built
➤ Building Tauri desktop app...
  ✅ Tauri app built
  Executable: bonsai-workspace\src-tauri\target\release\bonsai-workspace.exe
```

**Phase 3 - Data Prep (1-2 hours in)**
```
➤ Generating 1.6M training examples...
  ✅ Training data prepared
```

**Phase 4 - Training (starts ~2-3 hours in)**
```
➤ Starting GPU training (RX 7900 XTX, 24GB VRAM)...
  Expected duration: 4-6 hours
  Model: TinyLlama 1.1B (QLoRA with rank-16)
  
  Step 10/600: loss=4.23, lr=2e-04
  Step 20/600: loss=3.87, lr=2e-04
  ...
  Step 600/600: loss=1.23, lr=2e-04
  ✅ Training complete
```

**Phase 5 - Merge/Convert (30 min after training done)**
```
➤ Merging LoRA adapter and converting to GGUF...
  Loading base model...
  Loading LoRA adapter...
  Merging LoRA into base model...
  Running llama.cpp conversion...
  ✅ GGUF model ready: psychopathy-octopus-v1.Q4_K_M.gguf (600 MB)
```

**Phase 6 - Launch (starts ~6-10 hours in)**
```
🎯 YOUR BONSAI ECOSYSTEM IS READY
  Location: Z:\Projects\BonsaiWorkspace
  GPU Model: RX 7900 XTX (24 GB)
  Training Data: 1.6M examples
  Model: Psychopathy Octopus (TinyLlama 1.1B LoRA)
  IDE: Tauri (native Windows app)

TO LAUNCH THE COMPLETE STACK:
  1. Start inference API: cargo run --release -p bonsai-api-gateway
  2. Start IDE: cd bonsai-workspace && pnpm tauri dev
```

---

## ✅ Success Indicators

After build completes, look for:

1. **Files Created**
   ```powershell
   Test-Path "Z:\Projects\BonsaiWorkspace\crates\usos-kernel\target\x86_64-unknown-none\release\usos-kernel"
   # Should return: True
   
   Test-Path "Z:\Projects\BonsaiWorkspace\psychopathy-octopus-v1.Q4_K_M.gguf"
   # Should return: True (~600 MB)
   
   Test-Path "$env:USERPROFILE\.bonsai\models\psychopathy-octopus-v1.Q4_K_M.gguf"
   # Should return: True
   ```

2. **IDE Launches**
   - Tauri window opens automatically
   - Shows file explorer, code editor, chat panel
   - Model selector in top bar

3. **Model Appears in Selector**
   - Dropdown shows: octopus-v1, llama-3-8b, mistral-7b, neural-chat-7b
   - octopus-v1 is pre-selected

---

## 🧪 Test Immediately After Build

Once the IDE launches:

```powershell
# In another PowerShell terminal, start the API server (should already be running)
cargo run --release -p bonsai-api-gateway -- --host 127.0.0.1 --port 11425

# Test via curl
curl http://127.0.0.1:11425/v1/chat/completions `
  -H "Content-Type: application/json" `
  -d '{
    "model":"psychopathy-octopus-v1",
    "messages":[
      {"role":"user","content":"What containers are running on the Octopus server?"}
    ]
  }' | ConvertFrom-Json | Select-Object -ExpandProperty choices
```

Expected response: Server container list from knowledge module

---

## ⚠️ If Something Fails

### Training Fails (Phase 4)
- Check GPU memory: Task Manager → Performance → GPU
- Reduce batch size in train_psychopathy.py: `per_device_train_batch_size = 1`
- Ensure PyTorch is using GPU: `python -c "import torch; print(torch.cuda.is_available())"`

### Compilation Fails (Phase 1-2)
- Clear cargo cache: `cargo clean`
- Update Rust: `rustup update`
- Rebuild: Re-run script

### Merge/Convert Fails (Phase 5)
- Ensure llama.cpp is cloned: `git clone https://github.com/ggerganov/llama.cpp`
- Build llama.cpp: `cd llama.cpp && make`
- Check Python has transformers: `pip install transformers peft`

### IDE Fails to Launch
- Install dependencies: `cd bonsai-workspace && pnpm install`
- Try manual build: `pnpm tauri dev`
- Check logs: `C:\Projects\BonsaiWorkspace\frontend-build.log`

---

## 📈 Resource Usage Expected

| Phase | CPU | GPU | RAM | Duration |
|-------|-----|-----|-----|----------|
| Kernel | 4 cores | — | 200 MB | 5 min |
| IDE | 8 cores | — | 2 GB | 15 min |
| Data prep | 12 cores | — | 8 GB | 1-2 hours |
| Training | 4 cores | 95% | 12 GB | 4-6 hours |
| Merge/Convert | 8 cores | — | 6 GB | 30 min |
| Total (peak) | 12 cores | 95% | 12 GB | 6-10 hours |

---

## 🎯 Next Steps After Build

1. **Test the model** in the IDE (ask server-related questions)
2. **Collect feedback** (thumbs up/down in chat interface)
3. **Run nightly improvement** to fine-tune on feedback:
   ```powershell
   .\scripts\improve-octopus.ps1
   ```
4. **Deploy to server** when satisfied (copy .gguf file to friend's server)

---

## 📱 Real-Time Notifications

- Build complete: You'll receive a notification
- Model ready: IDE opens automatically
- Any errors: Check the log files above

**Build is now running in the background. Check back in 6-10 hours for completion.** 🚀
