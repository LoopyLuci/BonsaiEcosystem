# 🚀 Bonsai Android Bridge — Next-Generation Mobile Control System

**Status:** Implementation in progress  
**Timeline:** 12-week phased rollout  
**Scope:** Complete transformation of Android device control from ADB to capability-secure, low-latency, production-grade system  

---

## Executive Summary

The Bonsai Android Bridge replaces the aging ADB system with a **sovereign, fully-integrated, production-grade mobile development powerhouse** that enables:

✅ **Zero-configuration device discovery** via mDNS  
✅ **Capability-based security** with revocable permission tokens  
✅ **Sub-50ms screen streaming** with hardware acceleration  
✅ **Instant app deployment** with hot-reload (<2s)  
✅ **Multi-device management** (1-1000+ devices)  
✅ **AI-driven automation** via MCP + UACS + BTI  
✅ **Enterprise compliance** with full audit trails  
✅ **Self-healing infrastructure** via Survival System  

---

## Core Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              Bonsai Ecosystem (Desktop/Server)              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │        Android Bridge Controller (Rust Weave)        │  │
│  │  • Connection Manager (mDNS + Capability Pairing)    │  │
│  │  • Screen Stream Decoder (Hardware-accelerated)      │  │
│  │  • Input Injector (Touch/Keyboard/Mouse)             │  │
│  │  • File Syncer (CAS Delta Compression)               │  │
│  │  • App Deployer (Blueprint + Hot-Reload)             │  │
│  └──────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │        Bonsai IDE Android Panel (Svelte)             │  │
│  │  • Live Screen Mirror with Touch-Through             │  │
│  │  • Device Management Dashboard                       │  │
│  │  • Sensor Data Readouts                              │  │
│  │  • Deploy/Reload/Debug Controls                      │  │
│  └──────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │     MCP Tools + UACS Approval + BTI Commands         │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                         │ TransferDaemon
                         │ (QUIC/WebRTC)
                         │ Noise encryption
                         ▼
┌─────────────────────────────────────────────────────────────┐
│           Android Device (USOS-Compatible)                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         Bonsai Android Agent (Kotlin/Compose)        │  │
│  │  • Screen Encoder (MediaCodec H.264/H.265)           │  │
│  │  • Input Receiver (Touch/Keyboard via Accessibility) │  │
│  │  • File Sync Endpoint (CAS Reconstruction)           │  │
│  │  • App Installer (PackageInstaller API)              │  │
│  │  • Sensor Access (Accelerometer, GPS, etc.)          │  │
│  │  • Capability Validator (Signed Tokens)              │  │
│  └──────────────────────────────────────────────────────┘  │
│  Running in Sanctum compartment or background service      │
└─────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Core Connection (Weeks 1-3)

### Deliverables
- ✅ `crates/bonsai-android-bridge/` crate with full module structure
- ✅ mDNS discovery for automatic device detection
- ✅ Capability-based pairing (one-time code + QR scan)
- ✅ Encrypted TransferDaemon channel establishment
- ✅ Android agent basic service skeleton
- ✅ Unit tests (95%+ coverage)

### Key Components

**Connection Manager:**
```rust
pub struct ConnectionManager {
    pub async fn discover() -> Vec<DeviceInfo>;
    pub async fn pair(device_id: &str, pairing_code: &str) -> Result<DeviceHandle>;
    pub async fn connect(device_id: &str) -> Result<DeviceHandle>;
    pub async fn list_devices() -> Vec<DeviceStatus>;
}
```

**Security Model:**
- Device ID derived from hardware fingerprint (SHA-256 of device identifiers)
- Pairing exchange uses Ed25519 keys
- Session encryption via Noise protocol with per-session ephemeral keys
- Capability tokens are RSA-signed, revocable, and time-limited
- All pairing is recorded in Bonsai Universe for audit trail

---

## Phase 2: Screen Streaming (Weeks 4-6)

### Deliverables
- ✅ H.264/H.265 encoder on Android (MediaCodec integration)
- ✅ Hardware-accelerated decoder on desktop (VAAPI/DXVA2/VideoToolbox)
- ✅ Adaptive bitrate control based on network stats
- ✅ WebRTC integration for remote streaming
- ✅ Live screen mirror in IDE with <50ms latency
- ✅ Screenshot/screen recording capabilities

### Performance Targets
- **Glass-to-glass latency:** <50ms (measured from screen change to display)
- **Resolution:** Up to 1440p
- **Frame rate:** 30-60 fps
- **Adaptive quality:** Automatically adjusts based on bandwidth

### Network Adaptation
```
Desktop → RTT/Jitter/Loss measurements → Android Agent
Android Agent → Adjust encoder bitrate and resolution
Bitrate range: 500 kbps (1080p, 15fps) to 15 Mbps (1440p, 60fps)
```

---

## Phase 3: Input & File Sync (Weeks 7-9)

### Input Injection
- Touch events (down/move/up with pressure/size)
- Keyboard events (physical key injection)
- Mouse movements (for desktop mirroring)
- Gesture sequences (swipe, pinch, rotate)
- Latency: <10ms from user action to device response

### File Synchronization
```
Desktop: File change → CAS Delta computation
Desktop → (only delta) → Android Agent
Android: Apply delta → File reconstruction
One-way sync (desktop → device) for rapid iteration
```

### Use Cases
- Live code editing + hot-reload (change Java → 500ms reload)
- Asset updates without rebuilding APK
- Database schema changes without reinstall
- Configuration file updates

---

## Phase 4: App Deploy & Hot-Reload (Weeks 10-12)

### Deployment Flow
```
User clicks "Deploy" in IDE
    ↓
Blueprint builds APK (or incremental DEX)
    ↓
APK pushed to device via CAS
    ↓
PackageInstaller installs silently
    ↓
App launched or activity restarted
    ↓
Event broadcast to listening agents
Time: ~3-5 seconds for full APK, <500ms for DEX-only hot-reload
```

### Hot-Reload Mechanism
- Detects which files changed (DEX, resources, assets, config)
- For DEX changes: replaces only changed classes
- For resource changes: reloads resources and invalidates layout cache
- For config changes: sends Intent to app with new configuration
- Falls back to full APK rebuild if dependencies are cyclic

---

## Phase 5: Multi-Device & Sensor Access (Weeks 13-15)

### Device Dashboard
```
┌─────────────────────────────────────────┐
│  Device Grid (4-16 devices per view)    │
├─────────────────────────────────────────┤
│ [Google Pixel 7]  [iPhone 14]  [iPad]  │
│ Connected ✓       Pairing... ⏳  Offline│
│ Battery: 85%      Network: 4G  N/A     │
│ 👤 1 User         🔒 Locked    ❌      │
│ [Deploy] [Reload] [Screen]      [...]  │
├─────────────────────────────────────────┤
│ Filter: Connected | All | iOS | Android│
└─────────────────────────────────────────┘
```

### Sensor Access
- Accelerometer/Gyroscope/Magnetometer streams
- GPS location updates
- Camera frame access (if capability granted)
- Battery/temperature monitoring
- Network signal strength

### Capability Tokens for Sensors
Each sensor has an individual revocable token:
```
device/pixel-7/cap/accelerometer (read, 60 Hz sample rate)
device/pixel-7/cap/camera (read, H.264 stream)
device/pixel-7/cap/location (read, 5s update interval)
```

---

## Security & Compliance

### Threat Model
| Threat | Mitigation |
|--------|-----------|
| Man-in-the-middle (network interception) | Noise protocol with perfect forward secrecy |
| Rogue device claiming device ID | Hardware fingerprint + public key binding |
| Unauthorized capability use | RSA-signed, time-limited capability tokens |
| Credential theft | Tokens stored in OS keychain, never in plaintext |
| Device compromise | Sanctum vault isolation + SELinux policies |

### HITL Integration
Sensitive operations require UACS approval:
- **Automatic:** Screen capture, input injection, file transfer
- **With HITL:** Camera access, location access, app installation, system settings changes
- **Always audit:** All operations logged to Bonsai Universe CAS

### Audit Trail
Every action recorded:
```json
{
  "timestamp": "2026-05-31T14:32:15Z",
  "device": "pixel-7",
  "capability": "screen_capture",
  "action": "start_stream",
  "duration_ms": 1200,
  "bytes_transferred": 45000,
  "user": "alice@bonsai.dev",
  "status": "success",
  "hash": "sha256:abc123..."
}
```

---

## Integration with Bonsai Infrastructure

### Sentinel Core
- Monitor device health (battery, temperature, connectivity)
- Automatic reconnection with exponential backoff
- Failure detection and self-healing

### Sanctum
- Desktop controller runs in isolated Sanctum vault
- Android agent runs in work profile or Sanctum compartment
- Resource limits: 2 CPU cores, 512MB RAM, 1Gbps network

### Weave
- Android Bridge packaged as Weave component
- Declarative startup: `weave start android-bridge --devices 8`
- Service discovery: `weave resolve android-bridge`

### Blueprint
- APK builds use Blueprint manifest system
- Incremental builds for fast iteration
- Device-specific variants (armv8, x86_64)

### TransferDaemon
- Core communication substrate
- Handles QUIC, WebRTC, TCP fallback
- Automatic mDNS service advertisement

### Universe Events
- Every device action is a Universe event
- Time-travel debugging: replay any session
- ML training: learn optimal device selection, test prioritization

### Credits System
- Device usage metered in $WORK credits
- Active streaming: 10 credits/hour
- Deployment: 5 credits per APK
- File sync: 0.1 credits per MB transferred

### Survival System
- Automatic session recovery on network loss
- Checkpoint: device state snapshot before high-risk ops
- Rollback: revert failed deployments

---

## MCP Tools

```
android_list_devices()
  → List all discovered and paired devices

android_connect(device_id)
  → Establish connection to a device

android_start_screen_stream(device_id, bitrate_kbps, resolution)
  → Begin receiving screen frames

android_stop_screen_stream(device_id)
  → Stop screen streaming

android_inject_touch(device_id, x, y, action)
  → Inject touch event (down/move/up)

android_inject_key(device_id, keycode, down)
  → Inject keyboard event

android_install_app(device_id, apk_path)
  → Install APK on device

android_hot_reload(device_id, changed_files)
  → Hot-reload changed code/assets

android_take_screenshot(device_id)
  → Capture current screen as PNG

android_read_sensor(device_id, sensor_type, duration_sec)
  → Stream accelerometer/gyro/GPS data

android_run_shell(device_id, command)
  → Execute shell command on device (with approval)

android_deploy_blueprint(device_id, blueprint_path)
  → Build and deploy app from Blueprint manifest
```

---

## BTI Commands

```
:android list
  List all devices

:android connect <device>
  Connect to a specific device

:android disconnect <device>
  Disconnect from device

:android screen <device>
  Start screen mirroring in terminal (ASCII art)

:android deploy
  Deploy current app to selected device

:android reload
  Hot-reload current app on device

:android shell <device> <command>
  Run shell command on device

:android sensors <device> <type> [duration]
  Stream sensor data (accel, gyro, gps, temp)

:android screenshot <device>
  Capture and save screenshot

:android pair <device> <code>
  Pair with a device using code or QR
```

---

## Bonsai IDE Integration

### New Panel: Android Devices

**Location:** Bottom-right of editor, above BTI panel  
**Keyboard:** `Ctrl+Shift+A` to toggle

**Components:**
1. **Device List**
   - Shows all discovered/paired devices
   - Status indicator: green (connected), yellow (pairing), gray (offline)
   - Actions: connect, disconnect, pair

2. **Screen Mirror**
   - Live feed from selected device
   - Touch-through: click on screen to inject touch
   - Resolution/bitrate info overlay
   - Recording button (record screen as MP4)

3. **Toolbar**
   - Deploy button (build + install + launch)
   - Hot-Reload button (push changed files)
   - Screenshot button
   - Settings (bitrate, resolution, update interval)

4. **Sensor Panel**
   - Expandable readouts: battery, network, temperature
   - Graphs: accelerometer, GPS path
   - Network latency and packet loss

5. **Console**
   - Live logcat (filtered by app process)
   - Crash detection and highlighting
   - Launch commands (Intent dumping)

---

## Implementation Roadmap

| Phase | Timeline | Status | Owner |
|-------|----------|--------|-------|
| **1 – Core Connection** | Weeks 1-3 | 🔄 In Progress | Agent (Background) |
| **2 – Screen Streaming** | Weeks 4-6 | ⏳ Scheduled | TBD |
| **3 – Input & File Sync** | Weeks 7-9 | ⏳ Scheduled | TBD |
| **4 – App Deploy & Hot-Reload** | Weeks 10-12 | ⏳ Scheduled | TBD |
| **5 – Multi-Device & Sensors** | Weeks 13-15 | ⏳ Scheduled | TBD |
| **6 – Production Hardening** | Weeks 16-18 | ⏳ Scheduled | TBD |

---

## Competitive Analysis

| Feature | Scrcpy | OpenSTF | Android Bridge |
|---------|--------|---------|-----------------|
| **Latency** | 30-50ms | 100-200ms | <50ms |
| **Scaling** | 1-5 devices | 100+ devices | 1-1000+ devices |
| **AI Integration** | No | No | ✅ MCP + UACS |
| **Hot-Reload** | No | No | ✅ <500ms |
| **Capability Tokens** | No | No | ✅ Signed + Revocable |
| **Zero-Config** | No | No | ✅ mDNS Discovery |
| **Self-Healing** | No | No | ✅ Survival System |
| **Audit Trail** | No | No | ✅ Universe Events |

---

## Success Metrics

- **Connection:** Device discovery < 2s, pairing < 10s, reconnection < 5s
- **Screen:** <50ms latency, 60fps at 1080p, adaptive bitrate
- **Input:** Touch injection < 10ms, keyboard < 5ms
- **Deploy:** APK install < 5s, hot-reload < 500ms
- **Scaling:** Manage 100+ devices simultaneously
- **Reliability:** 99.9% uptime, auto-recovery from network failures
- **Security:** Zero security incidents, 100% audit coverage
- **Developer Experience:** One-click deploy, integrated in IDE

---

## Immediate Next Steps

1. **Agent Implementation** — Creating complete Phase 1 crate in background
2. **Code Review** — Once agent completes, verify all modules
3. **Unit Tests** —