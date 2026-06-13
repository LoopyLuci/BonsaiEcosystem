# Bonsai Ecosystem Android Apps — Complete Design Document

**Vision:** Two Android apps that bring the full Bonsai Ecosystem to mobile with childishly simple UX,
seamless desktop integration, and a distributed computing fabric that turns every phone and tablet into
a first-class compute node. Every feature is either native on the device or offloaded to the desktop
through a secure, zero-configuration connection. Nothing requires a manual.

---

## Table of Contents

1. [App Overview](#1-app-overview)
2. [Bonsai Workspace Android App](#2-bonsai-workspace-android-app)
3. [Bonsai Buddy Android App](#3-android-runtime-app)
4. [Inter-App Integration](#4-inter-app-integration)
5. [Bonsai-Bot Mobile Automation](#5-bonsai-bot-mobile-automation)
6. [Distributed Computing Platform (Layers 0–3)](#6-distributed-computing-platform)
7. [Shared Technical Architecture](#7-shared-technical-architecture)
8. [Security & Privacy Model](#8-security--privacy-model)
9. [Childishly Simple UX Audit](#9-childishly-simple-ux-audit)
10. [Development Roadmap](#10-development-roadmap)

---

## 1. App Overview

| App | Role | Standalone | With Desktop |
|-----|------|-----------|--------------|
| **Bonsai Workspace** | Full mobile IDE + project control center | File browser, code editor, local chat (small model), terminal (local sandbox), training monitor (view), automations | Remote file sync, remote terminal/SSH, full model inference, training control, swarm orchestration, GPU tasks |
| **Bonsai Buddy** | Conversational AI companion | Voice/text chat (local 0.5B model), quick actions, push notifications, home screen widget, wake word | Heavy reasoning via desktop model, desktop automation triggers, project commands, remote build/deploy |

Both apps share: identity keys (Ed25519 in Android Keystore), Rust core (compiled via cargo-ndk), and the
distributed computing fabric (Layer 1 mobile cluster).

---

## 2. Bonsai Workspace Android App

### 2.1 Design Philosophy

**One screen, one job.** The app has five top-level destinations via bottom navigation. Every
destination is a single focused task. Complexity lives inside, never on the surface. A user who has
never touched a computer should be able to open the app, see their projects, and chat with BonsAI
within sixty seconds — without reading anything.

### 2.2 Navigation Structure

```
Bottom Nav
├── Home (Dashboard)       — project cards, status, quick actions
├── Chat                   — BonsAI conversation (mobile-optimised bubbles)
├── Files                  — local + remote file tree
├── Terminal               — command line (local sandbox or remote SSH)
└── More                   — training, models, peers, automations, settings
```

No tabs within tabs. No modal stacks deeper than two levels. Back gesture always works.

### 2.3 Screen-by-Screen Design

#### 2.3.1 Home (Dashboard)

- **Header:** Bonsai tree logo (animated; pulses gently when connected to desktop). Connection status
  dot (green = desktop linked, yellow = local-only, grey = offline).
- **Content:** `LazyColumn` of project cards. Each card shows:
  - Project name (large, bold)
  - One-line status summary ("Training: 47% complete", "All good", "1 crash auto-fixed")
  - Status dot: green / yellow (working) / red (needs attention)
  - Three quick-action icon buttons: Chat, Open Files, Run
- **FAB:** Large "+" button → new project wizard (3 steps: name, type, connect source)
- **Pull to refresh:** Syncs project status from desktop.
- **Empty state:** Illustration of a bonsai seedling. Text: "No projects yet. Tap + to start one."

#### 2.3.2 Chat

- **Layout:** Full-screen chat (same mental model as WhatsApp). Message bubbles, timestamps.
  User messages right-aligned (blue), BonsAI messages left-aligned (green/dark).
- **Input bar:** `TextField` + microphone button + send button. Always pinned to bottom above keyboard.
- **Voice:** Tap mic → hold-to-talk OR tap-to-toggle listening. Animated waveform while recording.
  Transcription appears in text field before sending (user can edit).
- **Context-aware suggestions:** When viewing a file in Files tab, opening Chat pre-fills: "Ask about
  [filename.rs]". Chat retains file context as an attachment bubble.
- **Tool call UI:** When BonsAI needs to run a command, it shows a card: action name, one-line
  description, [Allow] [Deny] buttons. Never raw JSON.
- **Code blocks:** Syntax-highlighted, scrollable horizontally. Long-press → "Copy", "Open in editor",
  "Run on desktop".
- **Model indicator:** Small chip at top showing which model is active ("Local 0.5B" / "Desktop 7B" /
  "Desktop 14B"). Tap to switch if connected.

#### 2.3.3 Files

- **Two tabs:** "This Device" and "Desktop" (greyed out with lock icon if not connected).
- **File tree:** Folders collapse/expand with a tap. Icons by file type (Rust 🦀, Python 🐍, JSON 📋, etc.).
- **Swipe actions:** Swipe left on any file → "Edit" / "Share" / "Delete". Swipe right → "Upload to desktop" (if on local tab).
- **Long-press:** Context menu with: Rename, Move, Copy path, View info, Open in other app.
- **Editor:** Opens full-screen. Monaco-lite (via WebView) with syntax highlighting for Rust, Python,
  TypeScript, JSON, TOML, Markdown. Keyboard toolbar: Tab, undo, redo, find/replace, save.
  Auto-save every 30 seconds.
- **Large file guard:** Files > 1MB on mobile show a banner: "This file is large. Open on desktop for
  best experience?" with [Open Here] [Send to Desktop] buttons.
- **FAB:** New File / New Folder picker.

#### 2.3.4 Terminal

- **Mode selector at top:** "Local Sandbox" / "Desktop SSH" (greyed out if not connected).
- **Local sandbox:** Runs a restricted shell via the Rust core. Available commands: file ops, git
  status/diff/log, cargo check (if project is local), bonsai-cli commands. No network access.
  No root. No system access outside project directory.
- **Remote SSH:** Connects to desktop via Bonsai's secure tunnel. Full bash/PowerShell session.
  Streams output in real time. Ctrl+C via button in toolbar.
- **Command bar:** Scrollable history with tap-to-reuse. Pre-built project buttons: [Build] [Test]
  [Lint] [Format] always visible above input.
- **Font size:** Pinch to zoom. Default 14sp.
- **Copy/paste:** Long-press selects text. Selection handles. System clipboard integration.

#### 2.3.5 More (Training, Models, Peers, Automations, Settings)

**Training Panel:**
- Active training card: phase name, step N/total, loss curve sparkline, ETA.
- [Pause] [Stop] [Deploy Adapter] buttons.
- History list: past training runs with final loss and duration.
- "Start new training" button → opens wizard that runs on desktop.

**Model Manager:**
- List of available models with size, type (local GGUF / remote), last used.
- "Download" button for models already on desktop → streams to phone via Bonsai tunnel.
- "Set as default" toggle.
- Disk usage indicator at bottom.

**Peers:**
- List of linked devices (desktop, other phones/tablets). Status dots, last-seen time.
- [Link new device] → shows QR code to scan on desktop or another phone.
- Tap a peer → send message, send file, open shared terminal.

**Automations:**
- List of automation recipes. See §5 for full design.

**Settings:**
- Profile: identity key fingerprint, display name.
- Connection: desktop address (auto-discovered or manual), connection timeout.
- Storage: cache size, sync paths.
- Notifications: per-event toggles.
- Compute donation: resource sliders for distributed computing (see §6).
- Theme: system / light / dark.
- About, licenses, version.

### 2.4 Remote Connection Protocol

**Discovery (zero-config):**
1. Desktop Bonsai daemon broadcasts `_bonsai._tcp` via mDNS on port 11369.
2. Mobile scans for 3 seconds on app open. Found devices appear as "Nearby Bonsai" cards.
3. One-tap → pairing initiated.

**Pairing (first time):**
1. Desktop displays a QR code: `{ "addr": "192.168.1.5:11369", "token": "<32-byte-random>", "pubkey": "<ed25519-pub>" }`.
2. Mobile scans QR → connects via WSS with self-signed TLS.
3. Both sides exchange Ed25519 public keys. Token consumed (single-use).
4. A shared Noise_XX handshake establishes a session key. All future messages encrypted.
5. Desktop shows: "Bonsai Mobile [device name] connected. [Trust] [Block]".

**Reconnection (known device):**
- Mobile stores `(address, peer_pubkey)`. On reconnect, Noise_IK handshake (no re-pairing needed).
- If address changed (dynamic IP), mDNS rediscovery resolves it.

**Remote access (over internet):**
- Optional: Desktop can expose a relay endpoint via a lightweight Bonsai relay server
  (self-hosted or Bonsai-provided). Mobile connects via relay using same Noise protocol.
- Alt: Tailscale or Wireguard integration for users who want full VPN.

### 2.5 Offline Mode

- **Chat:** Local 0.5B GGUF model handles conversations. Quality is reduced but functional.
- **Files:** Edit local files. Changes queued in SQLite. On reconnect, two-way sync with conflict resolution (last-write-wins by default; user prompted on conflicts).
- **Queued actions:** "Run tests on desktop" queued locally. When reconnected, executed in order.
- **Indicator:** When offline, a banner at top: "Not connected to desktop. Using local mode."

---

## 3. Bonsai Buddy Android App

### 3.1 Design Philosophy

**The entire app is a chat.** Nothing else is visible on first launch. Buddy is not an IDE; it is
a conversational companion that happens to control everything. Complexity is expressed only through
conversation. A child who can speak can use Buddy.

### 3.2 Screen Design

#### 3.2.1 Main Screen (Chat)

- **Full-screen chat**, no bottom navigation bar.
- **Avatar:** Animated glowing bonsai tree at top-center. Idle: slow glow. Listening: branches
  sway gently. Thinking: leaves shimmer. Speaking: synchronized mouth-like glow pulse.
- **Chat bubbles:** Same style as Workspace but even more minimal. No timestamps visible by default
  (tap a message to reveal).
- **Large circular mic button:** Bottom-center, always visible. Tap = toggle listening. Hold = push-to-talk.
  Vibrates on start/stop. Color: green when listening, blue when idle.
- **Text input:** Appears above mic button when user taps the keyboard icon. Auto-hides when mic is used.
- **Gestures:**
  - Swipe right from edge → Quick Actions panel
  - Swipe left from edge → Notification History
  - Pull down on chat → scroll to top (latest project status summary)

#### 3.2.2 Quick Actions Panel (swipe right)

A translucent overlay with large tiles. Default tiles (user-configurable):
- 🔨 **Build** — "Run build on desktop"
- ✅ **Test** — "Run tests"
- 🚀 **Deploy** — "Deploy latest model"
- 📊 **Status** — "Show project status"
- 📁 **Files** — Opens Workspace app on Files tab (if installed)
- ➕ **Custom** — user-defined action with name + command

Tap any tile → Buddy executes the action and replies in chat. No confirmation dialogs unless action
is destructive.

#### 3.2.3 Notification History (swipe left)

Chronological list of system events from desktop:
- "Training complete (Phase 2 of 8) — 3 min ago"
- "BonsAI auto-fixed a crash in bonsai-watchdog — 12 min ago"
- "File sync complete — 1 hr ago"

Tap any event → jump to the chat message where BonsAI reported it with full detail.

#### 3.2.4 Settings (gear icon, top-right)

- **Connection:** Link to Workspace / desktop (same QR flow as Workspace app)
- **Wake word:** Toggle "Listen for Hey Bonsai" (persistent notification when on)
- **Voice:** TTS voice selection, speed, pitch
- **Quick Actions:** Reorder, rename, add, delete tiles
- **Notifications:** Per-event toggles
- **Privacy:** Disable local transcript storage, clear history
- **Linked apps:** Shows if Bonsai Workspace is installed and linked

### 3.3 Voice Pipeline

```
Microphone input
    ↓
Silero VAD (on-device, ~200KB, Rust/ONNX)  — end-of-speech detection
    ↓
Whisper.cpp (on-device, whisper-tiny, ~30MB) — speech-to-text transcription
    ↓
Transcription text → BonsAI inference (local 0.5B or remote via Workspace)
    ↓
Response text
    ↓
Piper TTS (on-device, ~20MB neural voice) — text-to-speech
    ↓
Audio output (speaker / earpiece / Bluetooth)
```

Total on-device voice stack: ~55MB. Latency target: < 800ms from end-of-speech to first audio.

**Wake word ("Hey Bonsai"):**
- openWakeWord (TFLite, ~500KB model) runs as a foreground service with persistent notification.
- Only processes audio locally for the trigger pattern; never streams raw audio.
- Activates full listening mode on detection.
- Disable toggle in settings; off by default.

### 3.4 Home Screen Widget

**2×2 widget (Glance API — Jetpack Compose for Widgets):**
```
┌─────────────────────────┐
│  🌿 Bonsai Buddy         │
│  "All systems good"      │
│  ┌───────┐  ┌────────┐  │
│  │  Ask  │  │  🎙️   │  │
└──┴───────┴──┴────────┴──┘
```

- "Ask" button → opens Buddy directly to chat.
- Mic button → activates voice directly without opening app (uses Buddy service).
- Status line updated every 5 minutes via WorkManager fetch.

**4×2 widget (expanded):**
- Same as above plus last 2 notifications and last user message.

### 3.5 Text-to-Speech Responses

Buddy speaks its replies aloud by default (configurable). Audio plays through the selected
audio route (speaker / earpiece / connected Bluetooth device). Volume respects media stream.
When headphones connected: auto-switches to earpiece-style private voice.

---

## 4. Inter-App Integration

### 4.1 Shared Identity

Both apps use the **same Ed25519 identity key** for the device, stored in Android Keystore
(hardware-backed on devices with StrongBox). Shared via `AccountManager` or a bound service:

```
BonsaiIdentityService (background service in Workspace app)
    └── provides identity to Buddy via IPC (AIDL or Messenger)
```

If only Buddy is installed, it creates and manages its own identity.

### 4.2 Linking (Buddy → Workspace)

1. In Buddy: Settings → "Connect to Bonsai Workspace"
2. In Workspace: More → Settings → "Link Buddy" → displays QR
3. Buddy scans QR → receives Workspace's local service address + auth token
4. Buddy registers as a trusted client of Workspace's local WebSocket server
5. Both apps now share: session context, connection to desktop, automation triggers

After linking, Buddy can:
- Request Workspace to open a file (Android Intent: `bonsai.workspace.action.OPEN_FILE`)
- Request Workspace to run a terminal command and stream output back
- Transfer the active chat session to Workspace chat

### 4.3 Session Handoff

**Chat context transfer:** Buddy can say "Open this in Workspace" → the current conversation
context (last N messages as JSON) is sent via a local Intent with the conversation payload.
Workspace opens to Chat tab with the session pre-loaded.

**Reverse:** In Workspace Chat → "Continue in Buddy" button → same payload sent to Buddy.

### 4.4 Shared Notification Channel

Both apps register the same notification channel group: `bonsai.ecosystem`. System events
(training complete, crash fixed, file sync, peer message) are routed to whichever app is active.
If both apps are installed, the primary notification target is configurable (default: Buddy).

---

## 5. Bonsai-Bot Mobile Automation

### 5.1 Automation Model

Automation = **Trigger** + **Condition** + **Action** + **Notification**

No code required. Everything is selected from menus. An automation that a 10-year-old can create.

### 5.2 Trigger Types

| Trigger | Example |
|---------|---------|
| **Voice command** | "When I say 'deploy'" |
| **Schedule** | "Every day at 9 AM" / "Weekdays at midnight" |
| **Device event** | "When I connect to Wi-Fi 'HomeNetwork'" |
| **Desktop event** | "When training completes" / "When a crash is detected" |
| **File event** | "When file X is saved" (desktop) |
| **Battery event** | "When battery reaches 100% (fully charged)" |
| **Location** | "When I arrive home" (geofence, uses Fused Location) |
| **App event** | "When I open the app" / "When app goes to background" |
| **Incoming message** | "When someone sends me a peer message" |
| **Manual** | "When I tap this tile" (quick action) |

### 5.3 Action Types

| Action | Where it runs |
|--------|---------------|
| **Run script** | Desktop (by name: `build.ps1`, `daily_report.sh`) |
| **Run command** | Desktop terminal (arbitrary shell command) |
| **Train model** | Desktop training pipeline |
| **Deploy adapter** | Desktop model swap |
| **Send peer message** | To any linked peer |
| **Open file in editor** | Workspace app, local or remote |
| **Speak response** | Buddy TTS |
| **Send notification** | Android notification |
| **Run WASM function** | Local (sandboxed, from automation library) |
| **HTTP request** | External webhook (e.g., GitHub, Discord) |
| **Start/stop compute donation** | Distributed computing participation (see §6) |

### 5.4 Automation Editor UI (Workspace → More → Automations)

```
New Recipe Screen:
┌─────────────────────────────────────────┐
│  🏷️ Name: [Daily Deploy         ]       │
│                                         │
│  ⚡ WHEN                               │
│  ┌────────────────────────────────┐    │
│  │ 🕐 Every day at 11 PM          │    │
│  │           [Change]             │    │
│  └────────────────────────────────┘    │
│                                         │
│  🔍 IF (optional condition)            │
│  ┌────────────────────────────────┐    │
│  │ Battery > 50% AND on Wi-Fi     │    │
│  └────────────────────────────────┘    │
│                                         │
│  ▶ DO                                  │
│  ┌────────────────────────────────┐    │
│  │ Run script: deploy.ps1         │    │
│  │           [Change]             │    │
│  └────────────────────────────────┘    │
│                                         │
│  🔔 THEN NOTIFY ME  [ON]               │
│                                         │
│  [Save Recipe]    [Test Now]            │
└─────────────────────────────────────────┘
```

- Each section is a large tappable card that opens a bottom sheet picker.
- [Test Now] runs the action immediately regardless of trigger.
- Recipes are stored locally as JSON and synced to desktop's automation engine.

### 5.5 Bot Execution

- **On-device execution:** WorkManager schedules triggers. Bonsai-Bot Rust core evaluates conditions and fires actions locally or forwards to desktop.
- **Desktop execution:** Workspace app forwards the action request via WebSocket to desktop's `bonsai-bot` daemon which runs the script.
- **Logs:** Every execution logged with timestamp, result (success/failure), duration. Viewable in Automations → History.

---

## 6. Distributed Computing Platform

### 6.1 Layer Model

```
Layer 3 — Cloud Nodes      (rented VMs, cloud GPUs, managed Kubernetes)
Layer 0 — Desktop Cluster  (your PCs, workstations, home servers)
Layer 1 — Mobile Cluster   (phones, tablets — THIS SECTION)
Layer 2 — Edge Nodes       (Raspberry Pi, NAS, router-class devices)
```

All layers run the same **BonsaiGrid** core (Rust crate: `bonsai-grid`) and communicate via
the same protocol. The distinction is only in capability profiles and resource policies.

**Layer 1 (mobile) is a first-class compute tier**, not an afterthought. Phones contribute:
NPU inference, CPU parallel tasks, storage, sensor data, and network bandwidth — but only
when the user explicitly authorises it with granular controls.

### 6.2 Core Architecture

**Hybrid topology: hierarchical control plane + P2P data plane**

```
┌─────────────────────────────────────────────────────────────┐
│  CONTROL PLANE (coordinator — runs on desktop or cloud)     │
│  - Project registry, device registry, task queue            │
│  - Scheduler: assigns tasks to capable devices              │
│  - Health monitor: heartbeats, fault detection              │
│  - CAS: content-addressed storage for inputs/outputs        │
└────────────────────────┬────────────────────────────────────┘
                         │ WebSocket / QUIC
        ┌────────────────┼────────────────┐
        ▼                ▼                ▼
   [Desktop]         [Phone A]        [Tablet B]
   Worker node       Worker node      Worker node
        │                │                │
        └────────────────┴────────────────┘
              Direct P2P data transfer (libp2p / QUIC)
              for task inputs, outputs, checkpoints
```

**Why hybrid?** P2P data plane avoids coordinator bottleneck for large data. Coordinator-only
control plane ensures consistency, discovery, and scheduling without full consensus overhead.

### 6.3 Project & Device Joining

**Create a project (Workspace app — More → Compute → New Grid Project):**
1. Name the project
2. Choose project type (template: AI inference, video render, build, etc.)
3. Upload or select executable (WASM module, Docker image, script)
4. Set resource requirements per task (CPU cores, RAM MB, GPU %, estimated duration)
5. Generate **invite code** (8 alphanumeric chars, e.g., `BX9F2KM3`)
6. Code expires in 7 days or until revoked

**Join a project (any device):**
1. Open Workspace or grid companion app
2. Tap "Join Grid Project" → enter or scan invite code
3. Set resource limits for this project (sliders — see §6.4)
4. Device authenticates with coordinator (Ed25519 + signed join request)
5. Device receives project task profile and starts pulling tasks

### 6.4 Resource Allocation UI

```
Resource Donation for "MyAI Training"
┌─────────────────────────────────────────┐
│  CPU Cores     ████░░░░  2 of 8 cores   │
│  RAM           ███░░░░░  512 MB          │
│  GPU           ██░░░░░░  15%             │
│  Storage       █░░░░░░░  1.0 GB (temp)  │
│  Network       ███░░░░░  50 MB/day       │
│                                         │
│  ⚡ Only when:  ☑ Charging              │
│                ☑ On Wi-Fi              │
│                ☑ Battery > 30%         │
│                ☑ Screen off            │
│  📅 Time window: 11PM–7AM only         │
│                                         │
│  [Save Settings]                        │
└─────────────────────────────────────────┘
```

All sliders are debounced and changes take effect within one heartbeat cycle (30 seconds).

**Dynamic re-negotiation:**
- Device sends heartbeat with live metrics: `{ cpu_free: 45%, battery: 72%, thermal: normal, charging: true }`
- If battery drops below configured threshold → device sends `resource_reduce` event → coordinator
  stops assigning new tasks, existing tasks complete or checkpoint and pause.
- If device goes into power saver mode → tasks pause immediately, coordinator marks device as
  temporarily unavailable.
- Thermal throttling detected → GPU allocation drops to 0 automatically.

### 6.5 Scheduler Design

**Two-level scheduling:**

**Level 1 — Project Scheduler (coordinator):**
- Maintains a task DAG per project. Nodes are tasks, edges are dependencies.
- For embarrassingly parallel work: all tasks independent, assigned round-robin by capability score.
- For Map-Reduce: map tasks distributed first; reduce tasks scheduled after all map outputs arrive in CAS.
- For build pipelines (DAG): topological order; no task assigned until its dependencies' outputs are in CAS.
- Device capability matching: task declares `{ min_cpu: 2, min_ram_mb: 256, requires_gpu: false, requires_npu: true }`.
  Coordinator matches only devices that advertise those capabilities.

**Level 2 — Device Scheduler (local, on each device):**
- Manages tasks assigned to this device.
- Priority queue: real-time tasks > active project tasks > background batch tasks.
- Respects resource quotas: if a task would exceed RAM limit, pauses lower-priority tasks first.
- Work stealing: if device is idle, pulls tasks from coordinator's ready queue proactively.

### 6.6 Sandboxing & Execution

**Mobile (Android):**
- **WASM tasks** (preferred): Run via wasmtime-android (JNI binding to Rust wasmtime). WASI-limited
  capabilities: read-only access to task input directory, write-only access to task output directory.
  No network, no system calls beyond allowed WASI set.
- **Native tasks** (pre-approved signed binaries only): Run as isolated Android process with seccomp-BPF
  profile. Only project-specific temp directory accessible.
- **NPU tasks** (AI inference): Uses Android NNAPI delegate; model loaded from CAS, inference runs
  on NPU/GPU, output written to task output dir.

**Desktop (Linux/Windows):**
- **WASM**: wasmtime with WASI
- **Container**: OCI containers via containerd + rootless podman; optional Firecracker microVM for
  maximum isolation
- **Native**: Direct execution in per-task temp directory with seccomp on Linux

**All tiers:**
- Every task output is BLAKE3-hashed and compared against expected hash (if known) before
  being accepted into CAS.
- Tasks cannot access the internet unless explicitly granted `net_egress` capability.

### 6.7 Fault Tolerance

**Checkpointing:**
- Long-running tasks (> 2 min) must call `bonsai_grid::checkpoint(state_bytes)` periodically.
  The checkpoint is stored in CAS. Coordinator records the latest checkpoint key per task.
- On device disconnect: task marked as `interrupted`. New device assigned picks up from latest
  checkpoint automatically.

**Speculative execution:**
- For latency-critical tasks: coordinator assigns to 2 devices simultaneously, takes first valid
  result, cancels the other.
- Controlled by task flag `speculative: true`. Only for small tasks (< 10 CPU-seconds).

**Health scoring:**
- Each device has a `reliability_score` (0–100) based on: task success rate (60%), uptime
  consistency (20%), resource delivery accuracy (20%).
- Low-score devices get fewer tasks but are not excluded (might improve).

**Rebalancing:**
- Coordinator runs rebalance every 60 seconds: identifies devices with empty queues and
  devices with full queues; migrates tasks (cancel + checkpoint + reassign).

### 6.8 Networking

**Control plane:** HTTPS + WebSocket over TCP/443. Works through all firewalls.

**Data plane (P2P):**
- libp2p with multiaddr: tries direct TCP first, then QUIC (UDP hole-punching), then relay.
- QUIC gives 0-RTT resumption for mobile handoffs (Wi-Fi → 5G → Wi-Fi).
- Multipath QUIC on devices with multiple interfaces: aggregates Wi-Fi + 5G bandwidth.

**Large file distribution (task inputs):**
- BitTorrent-style chunked transfer via libp2p's bitswap protocol.
- Inputs stored in CAS; devices fetch only the chunks they need.

**Relay (fallback):**
- Coordinator provides TURN-like relay endpoints.
- Power users can self-host relay on a VPS.
- Relay traffic is encrypted (coordinator cannot read it).

### 6.9 Mobile-Specific Optimisations

**Android lifecycle integration:**
- `WorkManager` constraint: `requiresCharging()`, `requiresUnmeteredNetwork()`, `requiresDeviceIdle()`.
- Tasks executed in a `CoroutineWorker` that checks thermal status before starting and monitors it
  during execution.
- `PowerManager.WakeLock` (partial) held only during active task execution; released on pause.
- Doze mode: coordinator-side scheduling is aware of Doze; pre-fetches task + input before
  expected Doze window.

**Hardware acceleration:**
- **NPU (Neural Engine):** NNAPI delegate for WASM AI inference tasks; coordinator routes
  `task_type: ai_inference` to devices advertising `capability: npu`.
- **GPU (Vulkan compute):** `wgpu` (Rust, via JNI) for compute shaders. Used for image processing,
  AI inference (when NPU unavailable), and rendering tasks.
- **DSP:** Android AAudio for audio processing tasks; routed automatically by task type.

**Thermal management:**
- Thermal zone polling every 10 seconds via `/sys/class/thermal/thermal_zone*/temp` (Linux/Android).
- If skin temperature > 42°C: pause GPU tasks. > 45°C: pause all tasks. Report `thermal_critical`
  to coordinator.
- Resume automatically when temperature drops below 38°C.

**Battery protection:**
- Hard minimum: never contribute when battery < configured threshold (default 20%), even if task
  was mid-execution (checkpoint and pause).
- Charging detection via `BatteryManager.isCharging()`.

### 6.10 Project Type Universe

The following table shows supported project types, required resources, and scheduling strategy.

| Category | Project Type | Key Resources | Strategy |
|----------|-------------|---------------|----------|
| **AI/ML** | Federated learning (private on-device) | CPU/NPU, RAM | Gradient-aggregate: each device trains on local data, sends encrypted LoRA deltas to coordinator |
| **AI/ML** | Distributed hyperparameter search | CPU, RAM | Embarrassingly parallel; each device tests one config |
| **AI/ML** | LLM inference split (model sharding) | RAM, NPU/GPU | Pipeline parallel: layer groups assigned to devices |
| **AI/ML** | Embedding generation for KDB | NPU/CPU, RAM | Batch parallel; input text chunks → embedding vectors |
| **Build** | Android ROM build | CPU (8+ cores), 8GB+ RAM, 20GB storage | DAG; per-module parallel compilation |
| **Build** | Large Rust workspace compile | CPU, RAM | cargo's unit graph → task per crate |
| **Build** | Cross-compilation (ARM/RISC-V) | CPU, storage | Embarrassingly parallel per target |
| **Multimedia** | Blender render farm | GPU/CPU, RAM | Per-frame tasks; deadline scheduling |
| **Multimedia** | FFmpeg video transcode cluster | CPU, storage | Per-segment; split input, combine output |
| **Multimedia** | Stable Diffusion batch generation | GPU/NPU | Per-image tasks; speculative for speed |
| **Multimedia** | Audio mastering cluster | DSP/CPU, RAM | Per-track segment; sequential merge |
| **Science** | Protein folding (AlphaFold lite) | CPU, RAM, storage | DAG; MSA → structure stages |
| **Science** | Weather modelling (grid simulation) | CPU, RAM, network | Domain decomposition; ghost-cell exchange |
| **Science** | Molecular dynamics | CPU, GPU | Spatial decomposition; all-reduce for forces |
| **Data** | Distributed SQL (log analysis) | CPU, storage | Map-Reduce; per-file scan → aggregate |
| **Data** | Collaborative data labelling | CPU, minimal | Assign example batches per device |
| **Data** | Distributed web scraping | Network, CPU | URL-partitioned; politeness-respecting |
| **Security** | Hashcat distributed password recovery | GPU | Key-space partition; embarrassingly parallel |
| **Security** | Network fuzzing cluster | CPU, network | Per-target assignment; result correlation |
| **Creative** | Generative art (SD XL cluster) | GPU, RAM | Per-image; speculative for latency |
| **Creative** | 3D scene lightmap baking | GPU/CPU | Per-surface-patch; DAG for light bounces |
| **Web** | Load testing farm | Network, CPU | Coordinated ramp-up; result aggregation |
| **Education** | BOINC-style volunteer science | CPU/GPU | Background only; contribution score tracked |
| **Bonsai** | Training Agent data generation | CPU, RAM | Per-scenario batch; teacher model on desktop |
| **Bonsai** | KDB module building (embedding pipeline) | NPU/CPU | Per-document batch; index built on coordinator |

### 6.11 Security Model for Grid

**Zero-trust between devices:**
- Task code is signed by the project creator. Signature verified before execution.
- Task inputs are encrypted with project key. Only authorized project members can decrypt.
- Task outputs encrypted before leaving device. Coordinator and non-member devices cannot read.
- No device can see another device's raw data — only CAS pointers (opaque hashes).

**Isolation per task:**
- Separate temp directory per task, wiped immediately after completion.
- No access to device's user data (contacts, photos, documents) under any circumstances.
- WASM capability model: explicitly whitelist `read_task_input`, `write_task_output`. All else denied.

**Project authorization:**
- Joining a project requires the invite code + coordinator acceptance.
- Project creator can revoke any device at any time → device stops receiving tasks immediately.
- All previous task outputs from revoked device are flagged for re-verification.

**Coordinator trust:**
- Coordinator sees task metadata (type, size, priority) but never task data content if
  end-to-end encryption is used.
- For maximum privacy: run coordinator locally on desktop. Cloud coordinator is optional.

---

## 7. Shared Technical Architecture

### 7.1 Native Rust Core (bonsai-core-android)

All Rust crates from BonsaiWorkspace compile to Android targets via `cargo-ndk`:
- `aarch64-linux-android` (ARM64, all modern phones)
- `armv7-linux-androideabi` (older 32-bit devices, optional)
- `x86_64-linux-android` (emulator support)

The Rust core is exposed to Kotlin via **UniFFI** (Mozilla's Rust→Kotlin/Swift binding generator).
The `.udl` interface file declares all functions and types that Kotlin can call.

```
bonsai-core-android/
├── src/
│   ├── lib.rs          — UniFFI entry point, all exposed functions
│   ├── inference.rs    — local model inference (llama.cpp via JNI)
│   ├── voice.rs        — whisper.cpp + VAD + Piper TTS
│   ├── grid.rs         — distributed computing worker
│   ├── sync.rs         — file sync engine
│   └── crypto.rs       — Ed25519, X25519, AES-GCM (bonsai-crypto)
├── bonsai_core.udl     — UniFFI interface definition
└── build.rs            — cargo-ndk build script
```

Key exposed functions (Kotlin-visible):
```
fun startInference(modelPath: String, prompt: String): Flow<String>
fun transcribeAudio(pcmBytes: ByteArray): String
fun synthesizeSpeech(text: String): ByteArray
fun connectToDesktop(address: String, token: String): ConnectionHandle
fun syncFiles(localPath: String, remotePath: String): SyncResult
fun gridStartWorker(projectId: String, resourceLimits: ResourceLimits): WorkerHandle
fun gridSubmitTask(task: TaskRequest): Flow<TaskProgress>
```

### 7.2 Android UI Stack

```
Language:        Kotlin 2.x
UI framework:    Jetpack Compose (Material 3)
Navigation:      Navigation Compose (type-safe destinations)
Async:           Kotlin Coroutines + Flow
Background:      WorkManager (recurring sync, compute donation)
Storage:         Room (local SQLite for chats, automations, queue)
Preferences:     DataStore (Proto)
Network:         OkHttp (control plane HTTP), custom Rust QUIC (data plane via JNI)
Voice:           SpeechRecognizer (fallback), Rust whisper.cpp (preferred)
TTS:             Piper TTS via Rust core
Camera/QR:       CameraX + ML Kit barcode scanning
Notifications:   NotificationCompat, FCM (optional, for remote notifications)
Widgets:         Glance API (Jetpack Compose for home screen widgets)
```

### 7.3 Project Structure (Android)

```
android/
├── workspace/              — Bonsai Workspace app
│   ├── app/
│   │   ├── src/main/
│   │   │   ├── kotlin/com/bonsai/workspace/
│   │   │   │   ├── MainActivity.kt
│   │   │   │   ├── ui/
│   │   │   │   │   ├── home/HomeScreen.kt
│   │   │   │   │   ├── chat/ChatScreen.kt
│   │   │   │   │   ├── files/FilesScreen.kt
│   │   │   │   │   ├── terminal/TerminalScreen.kt
│   │   │   │   │   ├── training/TrainingScreen.kt
│   │   │   │   │   ├── automations/AutomationsScreen.kt
│   │   │   │   │   └── settings/SettingsScreen.kt
│   │   │   │   ├── data/
│   │   │   │   │   ├── db/BonsaiDatabase.kt
│   │   │   │   │   ├── sync/FileSyncWorker.kt
│   │   │   │   │   └── grid/GridWorker.kt
│   │   │   │   └── BonsaiApplication.kt
│   │   └── build.gradle.kts
│   └── core-android/       — Rust JNI library (shared with Buddy)
│       ├── src/
│       └── Cargo.toml
├── buddy/                  — Bonsai Buddy app
│   ├── app/
│   │   ├── src/main/
│   │   │   ├── kotlin/com/bonsai/buddy/
│   │   │   │   ├── MainActivity.kt
│   │   │   │   ├── ui/
│   │   │   │   │   ├── ChatScreen.kt
│   │   │   │   │   ├── QuickActionsPanel.kt
│   │   │   │   │   └── NotificationHistory.kt
│   │   │   │   ├── voice/
│   │   │   │   │   ├── VoiceService.kt
│   │   │   │   │   └── WakeWordService.kt
│   │   │   │   └── widget/
│   │   │   │       ├── BuddyWidget.kt
│   │   │   │       └── BuddyWidgetReceiver.kt
│   │   └── build.gradle.kts
│   └── (uses core-android/ from workspace module)
└── build.gradle.kts        — root (shared core-android dep)
```

### 7.4 Build System

```toml
# android/core-android/Cargo.toml
[package]
name = "bonsai-core-android"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
bonsai-hnsw   = { path = "../../crates/bonsai-hnsw" }
bonsai-kdb    = { path = "../../crates/bonsai-kdb" }
bonsai-crypto = { path = "../../crates/bonsai-crypto" }
uniffi        = { version = "0.27", features = ["build"] }
tokio         = { version = "1", features = ["full"] }
# whisper-rs, piper-rs, etc.
```

Build command:
```
cargo ndk -t arm64-v8a -t x86_64 -o ../workspace/app/src/main/jniLibs build --release
```

---

## 8. Security & Privacy Model

### 8.1 Identity

- **Device identity:** Ed25519 keypair generated on first launch. Private key stored in Android
  Keystore with `HARDWARE_BACKED` flag (protected by device secure enclave when available).
- **User identity:** Same as desktop Bonsai identity. Can import from desktop via secure QR transfer.
- **No accounts required:** Apps work fully without any Bonsai account or cloud registration.
  Cloud features (relay, FCM) are opt-in.

### 8.2 Data at Rest

- Android file-based encryption (FBE) protects all app data at the OS level.
- Chat history and automation configs additionally encrypted with a key derived from the
  device identity key.
- Downloaded models and KDB files stored in app-private storage (not accessible to other apps).

### 8.3 Data in Transit

- All desktop connections: Noise_XX protocol (Ed25519 identity + X25519 session key + AES-256-GCM).
- Grid data plane: same Noise protocol over libp2p.
- Control plane API calls: HTTPS with certificate pinning (coordinator's public key pinned).

### 8.4 Automation Safety

- Destructive automation actions (delete files, run arbitrary shell commands) require a confirmation
  prompt on first creation AND each time they run with a new command.
- Actions are sandboxed to project scope on desktop (cannot access files outside the project).
- "Trusted automation" flag: user can disable per-action confirmations for automations they have
  explicitly trusted.

### 8.5 Grid Trust Boundaries

- Task code must be signed by project creator. Unsigned code is rejected by the local worker.
- No task can access device contacts, SMS, location (beyond geofence triggers the user set up),
  photos, or any user personal data.
- Grid contribution can be disabled globally with a single toggle in Settings (immediate effect).
- Users can see exactly which tasks have run on their device in Grid → History.

---

## 9. Childishly Simple UX Audit

Every screen in both apps must pass all of these before release:

### Rule 1: One Primary Action Per Screen
Every screen has exactly one "main thing to do". If there are two important buttons, one of them
is wrong. The exception is the Dashboard (intentionally a list), where each card still has one tap target.

### Rule 2: No Jargon Anywhere

| Technical term | Plain English shown in UI |
|---------------|--------------------------|
| Ed25519 identity | "Your Bonsai ID" |
| LoRA adapter | "A brain update for BonsAI" |
| Training epoch | "Learning session" |
| WASM sandbox | (never shown to user) |
| Heartbeat timeout | "Lost connection — reconnecting…" |
| CAS hash | (never shown; shown as filename only) |
| Grid worker | "Sharing my device to help" |
| Federated learning | "Teaching BonsAI from my device privately" |

### Rule 3: Immediate Feedback
Every tap must produce a visible response within 100ms. Loading states use skeleton screens
(greyed-out content shapes), never blank screens with a spinner.

### Rule 4: Voice Works Everywhere
Mic button on every screen. Pressing it in any context starts a voice command relevant to
that context (not a generic "what do you want to do?").

### Rule 5: Errors Are Human
No error message shows a code, stack trace, or technical description to the user. Every error
shows: an emoji, a one-sentence plain-English explanation, and a [Try Again] button.

```
Example:
❌ "Couldn't connect to your desktop right now.
    Make sure both devices are on the same Wi-Fi."
    [Try Again]  [Connect Later]
```

### Rule 6: Onboarding in Under 2 Minutes
First launch flow (Workspace):
1. "Welcome to Bonsai! 🌿" → [Get Started] (10 sec)
2. "Your Bonsai ID was created. It's like your password — stored securely on this device." → [Got it] (10 sec)
3. "Connect to your computer?" → [Scan QR from desktop] / [Skip for now] (30 sec)
4. "Create your first project or browse your desktop's projects." → lands on Dashboard (done)

First launch (Buddy):
1. "Meet Bonsai Buddy 🌿" — avatar animates. → [Start] (5 sec)
2. "Say 'Hey Bonsai' or tap the mic to talk." — mic button glows. → [Try it] (20 sec)
3. Buddy replies to first voice input → user is in the flow. (done)

### Rule 7: No Mandatory Registration
Both apps work from first launch with no account, no email, no password, no cloud sign-in.
Cloud features (relay, cross-internet connection) are unlocked later if desired.

---

## 10. Development Roadmap

### Phase 1 — Rust Core Foundation (Month 1–2)
- `cargo-ndk` build pipeline, UniFFI bindings generated
- Ed25519 identity creation + Android Keystore storage
- Basic WebSocket client (connect to desktop)
- Local 0.5B GGUF inference via llama.cpp JNI
- Chat UI (Workspace): text only, local model only

**Deliverable:** Workspace app with local BonsAI chat working on a phone.

### Phase 2 — Workspace Core Features (Month 3–4)
- Remote connection + Noise handshake to desktop
- File browser (local + remote via desktop connection)
- Code editor (Monaco-lite WebView)
- Remote terminal (SSH via Bonsai tunnel)
- Dashboard with project cards from desktop
- Training monitor (view only)

**Deliverable:** Full remote-connected Workspace app; can browse files and run terminal commands
on desktop from phone.

### Phase 3 — Buddy App (Month 5)
- Voice pipeline: Whisper.cpp + VAD + Piper TTS (Rust core)
- Buddy chat UI (avatar animations, bubbles)
- Quick Actions panel
- Buddy ↔ Workspace link (local IPC)
- Home screen widget (Glance)
- Push notifications (desktop → phone via WebSocket)

**Deliverable:** Buddy app, fully voice-interactive, linked to Workspace, home screen widget.

### Phase 4 — Automation & Grid (Month 6)
- Automation editor UI (trigger + condition + action)
- WorkManager-based trigger execution
- Desktop script forwarding via WebSocket
- Grid worker (WASM task execution, resource sliders)
- Grid project creation + invite code
- Mobile contribution: CPU + NPU tasks

**Deliverable:** Automation recipes working end-to-end; phones contributing CPU/NPU tasks to grid projects.

### Phase 5 — Polish & Release (Month 7)
- Childishly Simple Audit: every screen reviewed and fixed
- Wake word ("Hey Bonsai") with openWakeWord
- Offline mode completeness (queue, sync reconciliation)
- Performance profiling: jank removal, memory optimization
- Accessibility: content descriptions, minimum touch targets 48dp, font scaling
- Play Store submission (or F-Droid for open-source track)

**Deliverable:** Two production-quality apps in the Play Store.

### Total: 7 months to two production-ready apps.

---

## Appendix A: Android Manifest Permissions

```xml
<!-- Both apps -->
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
<uses-permission android:name="android.permission.RECEIVE_BOOT_COMPLETED" />
<uses-permission android:name="android.permission.POST_NOTIFICATIONS" />
<uses-permission android:name="android.permission.FOREGROUND_SERVICE" />
<uses-permission android:name="android.permission.FOREGROUND_SERVICE_DATA_SYNC" />

<!-- Workspace only -->
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
<uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
<uses-permission android:name="android.permission.CHANGE_NETWORK_STATE" />
<!-- mDNS nearby devices (Android 12+) -->
<uses-permission android:name="android.permission.NEARBY_WIFI_DEVICES"
    android:usesPermissionFlags="neverForLocation" />

<!-- Buddy only -->
<uses-permission android:name="android.permission.RECORD_AUDIO" />
<uses-permission android:name="android.permission.VIBRATE" />

<!-- Optional (grid + automations) -->
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.BATTERY_STATS" />
<uses-permission android:name="android.permission.USE_BIOMETRIC" />
```

## Appendix B: Grid Resource Accounting Schema

```sql
CREATE TABLE grid_contributions (
    id          TEXT PRIMARY KEY,
    project_id  TEXT NOT NULL,
    task_id     TEXT NOT NULL,
    started_at  TEXT NOT NULL,
    finished_at TEXT,
    cpu_ms      INTEGER DEFAULT 0,
    gpu_ms      INTEGER DEFAULT 0,
    npu_ms      INTEGER DEFAULT 0,
    ram_mb_peak INTEGER DEFAULT 0,
    net_bytes   INTEGER DEFAULT 0,
    status      TEXT NOT NULL,  -- 'running', 'completed', 'failed', 'checkpoint'
    result_hash TEXT
);

CREATE INDEX idx_grid_project ON grid_contributions(project_id);
CREATE INDEX idx_grid_status  ON grid_contributions(status);
```

## Appendix C: Minimum Device Requirements

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| Android version | 10 (API 29) | 13+ (API 33) |
| RAM | 3 GB | 8 GB |
| Storage (app) | 200 MB | 500 MB (+ model downloads) |
| CPU | ARMv8 (any 2018+ phone) | Cortex-A75+ (2020+) |
| GPU | Any OpenGL ES 3.1 | Vulkan 1.1 (2020+) |
| NPU | Not required | NNAPI delegate support |
| Network | 4G LTE | Wi-Fi 5 / 5G |
| Secure enclave | Not required (key in TEE) | StrongBox (Pixel 3+, Samsung S20+) |
