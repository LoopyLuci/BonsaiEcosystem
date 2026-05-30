# Distributed Computing – Bonsai Compute Fabric

> **"Turn all your devices into a single supercomputer."**

The Bonsai Compute Fabric lets you pool the CPU, GPU, and memory of every device you own — desktop, laptop, phone, tablet — and direct that combined power at big tasks: model training, video rendering, code compilation, inference at scale.

---

## Concept

```
Desktop (Layer 0)    Laptop (Layer 0)
    ████ ████ ████       ████ ████
    GPU + CPU            CPU

         ▼                  ▼
    ┌────────────────────────────────┐
    │   Coordinator (on host device) │
    │   Task scheduler               │
    │   Result aggregator            │
    └────────────────────────────────┘
         ▲                  ▲
    Phone A (Layer 1)    Phone B (Layer 1)
    ██ ██                ██ ██
    CPU/NPU              CPU/NPU
```

Devices communicate directly over TransferDaemon (WebRTC for control, libp2p/QUIC for data) — no cloud intermediary.

---

## Architecture

### Layer 0 – Computers
Desktops and laptops. Always available when plugged in. Can use full GPU, large RAM allocations, and run long jobs. Typically the **Coordinator** lives here.

### Layer 1 – Phones and Tablets
Mobile devices. Opportunistic — available when charging, idle, and within thermal limits. Contribute CPU, NPU (neural processing unit), and moderate RAM. Best for short tasks that can be paused.

### The Coordinator
A lightweight actor (`CoordinatorActor`) on the host device that:
- Registers all participating devices and their capabilities
- Splits the job into tasks
- Assigns tasks to devices based on resource availability
- Monitors progress, handles failures (re-assigns failed tasks)
- Aggregates results

The Coordinator communicates with **Worker** actors on each participating device via `TaskDistributeStream`.

---

## Creating a Compute Project

1. In Bonsai Workspace, open **Tools → Compute Fabric → New Project**.
2. Fill in:
   - **Project name** – e.g., "Overnight training run"
   - **Task type** – see [Task Types](#task-types) below
   - **Executable** – a WASM binary or Docker image
   - **Input data** – select files from CAS or upload
   - **Resource requirements** – min CPU cores, min RAM, GPU required?
3. Click **Create**. An **Invitation Code** is generated.
4. Share the code with other devices.

---

## Joining a Project

On any device (desktop or mobile) with Bonsai installed:

1. Open Bonsai → **Compute Fabric → Join Project**.
2. Enter the invitation code.
3. Set your **resource donation sliders**:

```
CPU Cores:    [════════░░] 80%
RAM:          [══════░░░░] 4 GB
GPU:          [███████░░░] 70% (if available)
Storage:      [████░░░░░░] 10 GB
Max temp:     60°C
Battery min:  20% (pause below this)
Time window:  Always / Charging only / 10PM–6AM
Data cap:     Unlimited / 5 GB/day
```

4. Click **Donate Resources**. Your device registers with the Coordinator and becomes available for tasks.

---

## Resource Donation

Resource donation is designed to be safe and unintrusive:

### Dynamic re-negotiation
If your device gets hot (temperature sensor above threshold), Bonsai automatically reduces or pauses task execution. Tasks are gracefully migrated to other participating devices — no work is lost.

### Mobile-specific limits
- Tasks pause when battery drops below your configured threshold.
- Thermal throttling respects Android's `PowerManager` thermal API.
- Background execution uses `WorkManager` to survive app backgrounding.
- Screen-off optimization reduces update frequency to save battery.

### Withdrawing resources
Click **Stop Donating** at any time. Your device gracefully drains its current tasks, returns results to the Coordinator, and disconnects.

---

## Task Types

| Type | Description | Best device |
|---|---|---|
| **Compute** | General CPU/GPU computation (numeric, WASM) | Any |
| **Inference** | Run LLM inference on a batch of prompts | GPU desktop |
| **Train** | Fine-tune a model (DPO, SFT) | Desktop with GPU |
| **Render** | 3D rendering, video encoding | Desktop with GPU |
| **Compile** | Compile source code (Rust, C++, etc.) | Multi-core CPU |
| **Transcode** | Convert audio/video formats | Any |
| **Custom** | Your own WASM binary | Any capable device |

---

## Scheduling

### Current: Round-Robin
Tasks are assigned in sequence to available devices, weighted by device capability score:

```
Capability score = CPU_cores × 1.0
                 + GPU_available × 4.0
                 + RAM_GB × 0.5
                 - thermal_pressure × 2.0
```

### Planned: AI-Driven Predictive Scheduler
The scheduler will learn from historical task execution times and resource usage to predict optimal assignments. Devices with a proven fast track record for a specific task type get priority.

---

## Sandboxing

All task executables run in an isolated environment. There are three tiers:

### Tier 1 – WASM (Default)
- Task is a WebAssembly binary.
- Runs in a WASM sandbox — no file system access, no network, no system calls unless explicitly granted.
- Cross-platform: the same WASM binary runs on Windows, macOS, Linux, and Android.
- Overhead: ~5–10% compared to native.

### Tier 2 – Container (gVisor)
- Task runs in a Linux container with gVisor userspace kernel.
- Stronger isolation than native containers.
- Requires the host to have Docker or containerd installed.
- Not available on Android.

### Tier 3 – Native with TEE
- Task runs natively on the host CPU.
- **Trusted Execution Environment** attestation verifies the binary has not been tampered with.
- Requires consent from the device owner (explicit "I trust this executable" dialog).
- Maximum performance — no sandbox overhead.

---

## Use Case Examples

### Distributed LLM Training
Split a DPO training job into 100 mini-batches. Each device processes 10 batches, computes gradients, sends LoRA delta to the Coordinator. Coordinator aggregates deltas (federated averaging) and broadcasts updated weights. Result: a trained adapter in 30% of the time it would take on one machine.

### Video Rendering Farm
You have a 4K video project with 1000 frames. The Coordinator assigns 100 frames to each of 10 devices (phones + laptops). Each renders its frames and uploads to CAS. The Coordinator stitches them together.

### Collaborative Code Compilation
A large Rust workspace is split into crates. Each device compiles a subset of crates in parallel (using `sccache` for shared caching). Link time on the host machine.

### Personal BOINC-style Computing
Donate device resources to a shared scientific computation (protein folding, prime search) that other Bonsai users are coordinating.

---

## Monitoring

The **Compute Dashboard** shows:

```
Project: Overnight Training Run
Status: Running — 3 devices active

Device graph:
  Desktop (host)  [████████░░]  80% CPU, 60% GPU
  Laptop          [██████░░░░]  60% CPU
  Phone A         [████░░░░░░]  40% CPU (charging)

Task progress:
  Total tasks: 200
  Completed:   147
  Running:     18
  Queued:      35
  Failed:      0

Gantt chart: [visual bar chart of task timelines]

Estimated completion: 2h 14m
Data transferred: 12.4 GB
```

---

*← [Collaboration](07-COLLABORATION.md) · [Mobile Apps →](09-MOBILE.md)*
