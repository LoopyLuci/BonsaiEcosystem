# Redmi Note 12 Pro 5G Setup Guide

Complete step-by-step instructions for setting up Bonsai Remote Desktop on Redmi Note 12 Pro 5G.

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Desktop Setup](#desktop-setup)
3. [Mobile App Installation](#mobile-app-installation)
4. [QR Code Pairing](#qr-code-pairing)
5. [Network Configuration](#network-configuration)
6. [Testing & Verification](#testing--verification)
7. [Optimization](#optimization)

---

## Prerequisites

### On Desktop
- **OS**: Windows 10+ / macOS 10.15+ / Linux (Ubuntu 18.04+)
- **Network**: Stable WiFi 5/6 or wired Ethernet
- **Bonsai Daemon**: Running with `--enable-remote-desktop` flag
- **Port 5900**: Open and accessible (firewall rule added)

### On Mobile (Redmi Note 12 Pro 5G)
- **Android Version**: 12.0 or higher
- **Network**: WiFi or 5G capable
- **Storage**: 150 MB free space
- **Battery**: >30% charge (recommended >50%)
- **USB (Optional)**: For development/debugging

---

## Desktop Setup

### Step 1: Install Bonsai Daemon with Remote Desktop

**Windows**:
```powershell
# Download and extract Bonsai
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri "https://bonsai.local/releases/bonsai-daemon-latest-x64.zip" `
  -OutFile "$env:USERPROFILE\Downloads\bonsai-daemon.zip"
Expand-Archive -Path "$env:USERPROFILE\Downloads\bonsai-daemon.zip" `
  -DestinationPath "C:\Program Files\Bonsai"

# Run daemon with remote desktop enabled
& "C:\Program Files\Bonsai\bonsai.exe" daemon `
  --enable-remote-desktop `
  --remote-desktop-port 5900 `
  --local-bind 127.0.0.1
```

**macOS**:
```bash
# Using Homebrew
brew install bonsai-daemon

# Or download manually
curl -L https://bonsai.local/releases/bonsai-daemon-latest-macos.tar.gz | tar xz
sudo mv bonsai /usr/local/bin/

# Run daemon
bonsai daemon \
  --enable-remote-desktop \
  --remote-desktop-port 5900 \
  --local-bind 127.0.0.1
```

**Linux (Ubuntu/Debian)**:
```bash
# Add repository
sudo curl -fsSL https://bonsai.local/install.sh | sudo bash

# Install
sudo apt-get install bonsai-daemon

# Run daemon
sudo systemctl start bonsai-daemon
sudo systemctl enable bonsai-daemon

# Configure
sudo systemctl edit bonsai-daemon
# Add to [Service] section:
# Environment="BONSAI_ENABLE_REMOTE_DESKTOP=1"
# Environment="BONSAI_REMOTE_DESKTOP_PORT=5900"
```

### Step 2: Verify Daemon is Running

```bash
# Check service status
curl http://localhost:8080/health
# Expected response: {"status":"healthy","services":["remote_desktop"]}

# Check remote desktop is listening
netstat -tlnp | grep 5900
# Expected: LISTEN on 127.0.0.1:5900 (local)
#           or 0.0.0.0:5900 (if network-accessible)
```

### Step 3: Generate Pairing QR Code

Get the pairing QR code from the desktop daemon:

```bash
# Get pairing token
curl http://localhost:8080/api/remote-desktop/pairing-token
# Response:
# {
#   "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
#   "expires_in_seconds": 600,
#   "qr_code": "data:image/png;base64,iVBORw0KGgoAAAA..."
# }

# Or display QR code in web UI
open http://localhost:8080/ui/remote-desktop
# Click "Show Pairing QR"
```

### Step 4: Configure Firewall (if network-accessible)

**Windows (Command Prompt - Admin)**:
```cmd
# Allow incoming on port 5900
netsh advfirewall firewall add rule name="Bonsai Remote Desktop" `
  dir=in action=allow protocol=tcp localport=5900 profile=any

# Verify
netsh advfirewall firewall show rule name="Bonsai Remote Desktop"
```

**macOS**:
```bash
# PF firewall rules
sudo pfctl -e
sudo pfctl -f /etc/pf.conf
# Or use System Preferences > Security & Privacy > Firewall Options
```

**Linux (UFW)**:
```bash
sudo ufw allow 5900/tcp comment "Bonsai Remote Desktop"
sudo ufw status
```

---

## Mobile App Installation

### Step 1: Download APK

Download the Bonsai Remote Desktop APK for Android:

**Method A: From Bonsai Website**
1. Visit https://bonsai.local/download/android
2. Download: `bonsai-remote-desktop-latest.apk`
3. Transfer to Redmi via:
   - USB cable: Connect to computer, drag-and-drop to `Downloads`
   - QR code: Scan to download link
   - Email/cloud: Send yourself the APK, download on device

**Method B: Via ADB (if computer connected)**
```bash
adb install -r bonsai-remote-desktop-latest.apk
# Output: Success
```

### Step 2: Grant Permissions

Launch the app and grant required permissions:

```
Bonsai Remote Desktop > Grant Permissions
├─ ✓ Camera (for QR code scanning)
├─ ✓ Storage (for file transfer)
├─ ✓ Internet (required for connectivity)
├─ ✓ Accessibility Service (for text injection)
└─ ✓ Display over other apps (for floating toolbar)
```

**If permissions dialog doesn't appear**:
1. Go to Settings > Apps > Bonsai Remote Desktop
2. Tap "Permissions"
3. Toggle ON: Camera, Storage, Internet, Accessibility
4. Settings > Advanced > Display over other apps > ON

### Step 3: Initial Setup Wizard

The app will display a setup wizard:

```
┌─────────────────────────────────────────┐
│   Welcome to Bonsai Remote Desktop     │
├─────────────────────────────────────────┤
│                                         │
│  Step 1/4: Permissions                  │
│                                         │
│  You'll need to grant these permissions │
│  for the app to work properly:          │
│                                         │
│  [✓] Camera        [✓] Storage          │
│  [✓] Internet      [✓] Accessibility   │
│  [✓] Display over other apps           │
│                                         │
│          [< Back]  [Next >]             │
└─────────────────────────────────────────┘
```

Proceed through all 4 steps:
1. **Permissions**: Grant all
2. **Device Info**: Confirm device name and model
3. **Network**: Choose WiFi network
4. **Pairing**: Ready for QR code scan

---

## QR Code Pairing

### Step 1: Start Pairing Mode on Desktop

```bash
# Start pairing server (valid for 10 minutes)
curl -X POST http://localhost:8080/api/remote-desktop/pairing-start
# Response: {"pairing_token": "...", "qr_code_data_url": "..."}

# Display QR code
open http://localhost:8080/ui/remote-desktop/pairing
```

Or use the Bonsai Desktop UI:
1. Open System Tray > Bonsai
2. Click "Remote Desktop"
3. Click "Start Pairing" (green button)
4. QR code appears on screen

### Step 2: Scan QR Code on Mobile

On the Redmi:
1. Open Bonsai Remote Desktop app
2. Tap "Scan QR Code" (camera icon)
3. Frame the desktop's QR code
4. App automatically decodes and connects

```
Camera View on Mobile:
┌──────────────────────────────────────┐
│                                      │
│                                      │
│        ╔════════════════╗           │
│        ║   QR Code     ║           │
│        ║   Area        ║           │
│        ╚════════════════╝           │
│                                      │
│   "Move phone closer or adjust..."  │
│   [❌ Cancel]  [✓ Detected!]       │
│                                      │
└──────────────────────────────────────┘
```

### Step 3: Confirm Pairing

Both devices display confirmation:

**Desktop**:
```
User wants to pair:
Device: iPhone 14 Pro
Status: Detected at 192.168.1.42

[Waiting for user action on mobile...]
[Timeout in 30s]

[Cancel]
```

**Mobile**:
```
Pairing with Desktop
Device: Desktop #1 (Windows)
IP: 192.168.1.100:5900
Status: Verifying...

[Cancel]  [Confirm Pairing]
```

Tap "Confirm Pairing" on mobile. Desktop automatically confirms.

### Step 4: Verify Connected

After pairing, both show:

**Desktop**:
```
✓ Pairing Successful
Device: Redmi Note 12 Pro 5G
ID: 550e8400-e29b-41d4-a716-446655440000
Connection: Local (192.168.1.42)
Status: Ready
```

**Mobile**:
```
Connected to Desktop #1
Status: Ready
Connection: Local
Latency: 2.3 ms
Signal: ▁▃▅▇ (Excellent)

[Start Remote Session]
```

---

## Network Configuration

### Local Network (Recommended for Setup)

Best for home/office networks with WiFi 6 or 5.

**On Redmi**:
1. Settings > WiFi > (Select your network)
2. Keep connection active: Settings > Battery > Battery Saver > OFF
3. Keep WiFi awake: Settings > WiFi > Advanced > WiFi Sleep Policy > Never

**Typical Latency**: 2-5ms
**Expected Quality**: Crystal clear, no noticeable lag

### WiFi Network Selection

**5GHz (Recommended)**:
- Pros: Higher bandwidth, lower latency, less interference
- Cons: Shorter range
- Use for: Desktop-like responsiveness
- Command: Connect to "YourNetwork-5G"

**2.4GHz (Fallback)**:
- Pros: Better range, penetrates walls
- Cons: Lower bandwidth, higher latency, more interference
- Use for: Extended range, mobile positioning
- Command: Connect to "YourNetwork"

**5G Cellular** (Optional):
- Pros: Works anywhere with 5G coverage
- Cons: Higher latency (50-100ms), may be expensive
- Use for: Outdoor/mobile emergency access
- Note: Requires BRDF tunnel on desktop

### Router Configuration (Optional)

For optimal performance:

**WiFi Settings**:
- **Band**: 5GHz preferred (if device supports)
- **Channel Width**: 80 MHz (160 MHz if available)
- **Channel**: 36, 40, 44, 48 (less interference)
- **Transmit Power**: Max (100%)
- **QoS**: Optional priority rule for Bonsai (port 5900)

```
Router > QoS Settings:
Application: "Bonsai Remote Desktop"
Port: 5900
Priority: High
Bandwidth Limit: Unlimited
```

### Remote Network Setup (Advanced)

For accessing desktop from office/remote location:

**Step 1**: Enable BRDF on desktop
```bash
bonsai daemon --enable-remote-desktop --enable-brdf
# Registers desktop with BRDF infrastructure
```

**Step 2**: Obtain BRDF pairing code
```bash
curl http://localhost:8080/api/remote-desktop/brdf-code
# {"brdf_code": "XXXX-YYYY-ZZZZ", "expires_in_seconds": 3600}
```

**Step 3**: Pair mobile via BRDF code
On mobile: Bonsai > Add Peer > Enter BRDF Code > Confirm

**Latency over BRDF**: 40-100ms (depends on region distance)

---

## Testing & Verification

### Quick Connectivity Test

**Step 1**: Start a session
Mobile > Connect to Desktop > "Desktop #1"

Expected sequence:
```
[Connecting...] (2 sec)
  ↓
[Downloading video codec...] (1 sec)
  ↓
[Requesting frame...] (0.5 sec)
  ↓
[Connected!] (Desktop screen visible)
```

### Manual Latency Test

Use the built-in latency tester:

**Mobile**:
1. Open Bonsai Remote Desktop
2. Menu > Tools > Latency Test
3. Tap the red circle as fast as possible
4. App measures touch-to-response time

```
Latency Test Results:
├─ Tap-to-visible: 28ms (p50)
├─ Network RTT: 2.3ms
├─ Frame decode: 16.2ms
└─ Display update: 9.5ms
```

**Expected**:
- Local network: 20-40ms
- Remote (BRDF): 50-100ms
- Unacceptable: >150ms

### Video Quality Check

Mobile > Connected > Settings > Test Patterns

Test patterns available:
- **Color Bars**: Check color accuracy
- **Checkerboard**: Check for compression artifacts
- **Gradient**: Check for banding
- **Text**: Check for clarity

Look for:
- ✓ No pixelation
- ✓ No color banding
- ✓ Sharp text edges
- ✓ Smooth gradients

### Performance Benchmarking

Run benchmarks via terminal:

**On Mobile (via ADB)**:
```bash
adb shell am instrument -w \
  com.bonsai.remote_desktop.test/androidx.test.runner.AndroidJUnitRunner \
  -e class com.bonsai.remote_desktop.tests.PerformanceBenchmark
```

Results saved to `/data/local/tmp/benchmark.json`:
```json
{
  "test_name": "remote_desktop_benchmark",
  "duration_seconds": 60,
  "metrics": {
    "avg_fps": 59.8,
    "avg_bitrate_kbps": 8420,
    "decode_latency_p99_ms": 28.5,
    "memory_peak_mb": 92,
    "battery_drain_percent_per_hour": 10.2
  }
}
```

---

## Optimization

### Performance Tuning

After 1-2 sessions, open Settings to optimize for your network:

**Settings > Performance**:
- **Resolution**: 1080p (default) - adjust if needed
  - Mobile is too small for 1080p? Use 720p
  - Network is bad? Use 480p
- **FPS**: 60 (default)
  - Smooth scrolling? Keep at 60
  - Looks choppy? May be network issue, not FPS
  - Battery critical? Lower to 30
- **Bitrate**: Auto (recommended)
  - Manual: 5-10 Mbps typical

**Settings > Network**:
- **WiFi Band**: Auto
  - Force 5GHz if network supports
  - Fallback to 2.4GHz for range
- **Adaptive Bitrate**: ON (recommended)
  - Automatically reduces bitrate on poor network

### Battery Optimization

**Step 1**: Enable Battery Saver
Settings > Power > Battery Saver > Remote Desktop

**Step 2**: Adjust session settings
Settings > Remote Desktop > Battery Saver Options:
- Adaptive FPS: ON (drops to 30fps on idle)
- Dim while idle: ON (after 30s inactivity)
- CPU throttle: ON (reduces thermal load)

**Expected Impact**:
- Normal mode: 10-12% drain per hour
- Battery Saver: 6-8% drain per hour

### Heat Management

If device gets warm (>45°C):

1. Take a 5-minute break
2. Enable Battery Saver mode
3. Close background apps (Settings > App Management > Memory)
4. Lower FPS to 30 (Settings > Performance > FPS)

Monitor temperature via ADB:
```bash
adb shell dumpsys thermal
# Look for "CurrentTemperature_millidegrees_C"
```

---

## Troubleshooting Quick Reference

| Problem | Solution |
|---------|----------|
| Can't scan QR code | Enable camera permission, improve lighting |
| Black screen after connect | Restart desktop daemon, check firewall |
| Very high latency (>100ms) | Switch to WiFi 5GHz, move closer to router |
| Video stuttering | Lower FPS or resolution, close background apps |
| Touch feels unresponsive | Check latency test, verify WiFi signal strength |
| App crashes on startup | Update Android OS, clear app cache, reinstall |
| Rapid battery drain | Enable Battery Saver, reduce FPS/resolution |
| Keyboard input not working | Grant Accessibility permission again |

---

## Support

If you encounter issues not covered above:

1. Check logs: ADB > `adb logcat | grep -i bonsai`
2. File issue: https://github.com/bonsai-project/bonsai/issues
3. Contact support: support@bonsai.local

---

**Last Updated**: 2024-06-30
**Tested On**: Redmi Note 12 Pro 5G (MIUI 14.0.1)
