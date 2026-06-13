# Mobile Apps – Bonsai on Android

Bonsai brings its local-first AI platform to Android, with two companion apps designed for different use cases.

---

## Bonsai Workspace Android

A mobile-first IDE and AI platform. For developers and power users who need to work on the go.

### Project Dashboard
The home screen shows glanceable cards for your active projects:

```
┌──────────────────────────────────┐
│ 🌿 Bonsai Workspace              │
├──────────────────────────────────┤
│  📁 BonsaiWorkspace    Active ●  │
│  Last edit: 2 min ago            │
│  Training: Safety DPO 72%        │
├──────────────────────────────────┤
│  📁 side-project       Synced ✓  │
│  Last sync: Desktop (LAN)        │
├──────────────────────────────────┤
│  [+ New Project]                 │
└──────────────────────────────────┘
```

Tap a card to open the project.

### File Browser & Editor
- Tree view of your project files, synced from desktop via TransferDaemon.
- Tap any file to open the mobile code editor (CodeMirror-based, touch-optimised).
- Syntax highlighting for common languages.
- Swipe-based keyboard shortcuts (swipe right on a line to duplicate, swipe left to delete).
- Auto-save every 5 seconds.

### Chat with BonsAI
Full access to BonsAI from your phone:
- Type or use voice input (Whisper for speech-to-text).
- BonsAI responses stream token-by-token.
- Tool calls work: BonsAI can read/write files in the project, run shell commands on the linked desktop.
- Switch between on-device inference (small model) and remote inference (full model on desktop) seamlessly.

### Training Monitor
Track model training running on your desktop:
- Phase progress bars
- Live loss graph
- Milestone notifications (push notification when a phase completes)
- Brain Age indicator

### Automations (Bonsai-Bot)
Create phone-triggered automations:
- "When I arrive at work (GPS) → open project → start Docker → pull latest"
- "At 11 PM → start overnight training → notify when done"
- "When battery hits 80% on charger → donate 30% CPU to compute project"

### Linking to Desktop
1. On your phone, open **Settings → Link Device**.
2. Tap **Scan QR Code**.
3. On your desktop, open **Settings → Linked Devices → Show QR**.
4. Point phone at screen. Devices exchange Ed25519 public keys and establish a TransferDaemon session.

Once linked, your phone appears in the desktop's **Device Manager** and vice versa. Files sync automatically when on the same Wi-Fi network.

---

## Bonsai Buddy Android

A focused, always-available AI assistant. For anyone who wants quick access to BonsAI without opening a full IDE.

### Main Interface
Clean, minimal:

```
┌──────────────────────────────────┐
│  🌿 Bonsai Buddy                 │
│                                  │
│  ┌────────────────────────────┐  │
│  │ How can I help you today?  │  │
│  └────────────────────────────┘  │
│                                  │
│  [Quick actions →]               │
│  [🎤 Hold to speak]              │
│  [⌨ Type a message]              │
└──────────────────────────────────┘
```

### Voice Activation
Say **"Hey Bonsai"** when the app is in the background (always-on listening, runs locally on NPU, consumes ~1% CPU). BonsAI wakes up, ready to listen.

### Push-to-Talk
Press and hold the microphone button to dictate. Whisper transcribes in real time. Release to send.

### Quick Actions Panel
Swipe right to reveal:

| Action | Effect |
|---|---|
| 📋 Summarise clipboard | Reads clipboard, summarises |
| 🌤 Weather | Gets weather for current location |
| ⏱ Timer | "Set a 15-minute timer" |
| 📅 Calendar | "What's on my calendar today?" |
| 🖥 Desktop command | Sends a command to your linked desktop |
| 📷 Describe image | Takes a photo, BonsAI describes it |

### Home Screen Widget
Add the Bonsai widget to your home screen:
- Shows BonsAI status (online / training / idle)
- Quick input field for one-tap queries
- Tap the microphone to start voice input without opening the app

The widget is built with Jetpack Compose Glance.

### Notifications
When BonsAI completes a task you requested (from desktop or another app), you receive a push notification. Tap to see the result.

---

## Building from Source

### Prerequisites

```bash
# Rust with Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi

# cargo-ndk (cross-compilation helper)
cargo install cargo-ndk

# Android SDK + NDK (install via Android Studio or sdkmanager)
# Set environment variables:
export ANDROID_HOME=$HOME/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk/26.0.10792818

# JDK 17
```

### Building the Rust core

```bash
cd crates/bonsai-core-android
cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs build --release
```

### Building the Android app

```bash
cd android
./gradlew assembleRelease
# Output: app/build/outputs/apk/release/app-release.apk
```

### UniFFI bindings
The Rust core exposes its API to Kotlin via UniFFI. Bindings are generated automatically during the build:

```bash
cargo run --bin uniffi-bindgen generate \
  --library target/aarch64-linux-android/release/libbonsai_core.so \
  --language kotlin \
  --out-dir android/app/src/main/java/com/bonsai/core/
```

### Installing

```bash
adb install android/app/build/outputs/apk/release/app-release.apk
```

Or sideload the APK directly from the GitHub Releases page.

---

## Mobile-Specific Considerations

### Battery Management
- Bonsai Buddy listens for "Hey Bonsai" using the NPU, consuming < 1% battery per hour.
- Inference runs on the NPU (Qualcomm Hexagon, MediaTek APU, Google Tensor) when available — 3–5× more power-efficient than CPU.
- Compute Fabric tasks pause automatically when battery drops below your configured threshold.

### Thermal Management
- Bonsai monitors the device temperature via Android's `PowerManager.ThermalStatus` API.
- At `THERMAL_STATUS_MODERATE`, CPU governor steps down and GPU clock reduces.
- At `THERMAL_STATUS_SEVERE`, all compute tasks pause until temperature drops.

### Background Execution
- Long-running tasks (training, file sync, compute) use `WorkManager` with a `ForegroundService` so Android doesn't kill them.
- A persistent notification shows what's running with a progress bar and Cancel button.

### GPU and NPU Acceleration
- Vulkan compute shaders via `wgpu` for GPU inference.
- ONNX Runtime with `QnnExecutionProvider` for NPU inference (Qualcomm and MediaTek).
- Automatic fallback to CPU if GPU/NPU is unavailable or thermal-throttled.

---

*← [Compute Fabric](08-COMPUTE-FABRIC.md) · [Sovereignty →](10-SOVEREIGNTY.md)*
