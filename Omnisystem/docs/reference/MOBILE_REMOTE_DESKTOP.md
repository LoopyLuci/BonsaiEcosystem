# Mobile Remote Desktop Integration Guide

## Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Security Model](#security-model)
3. [Hardware Optimization](#hardware-optimization)
4. [Performance Tuning](#performance-tuning)
5. [Troubleshooting](#troubleshooting)
6. [Advanced Topics](#advanced-topics)

---

## Architecture Overview

### System Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    BONSAI REMOTE DESKTOP                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌──────────────────┐           ┌──────────────────────────┐   │
│  │   Mobile (iOS/   │           │    Desktop Host          │   │
│  │   Android)       │           │   (Windows/Mac/Linux)    │   │
│  │                  │           │                          │   │
│  │ ┌──────────────┐ │           │  ┌──────────────────┐   │   │
│  │ │ H.264 Video  │ │           │  │ Screen Capture   │   │   │
│  │ │ Decoder      │ │           │  │ (H.264/H.265)    │   │   │
│  │ └──────────────┘ │           │  └──────────────────┘   │   │
│  │        ▲         │           │           ▼              │   │
│  │        │         │           │  ┌──────────────────┐   │   │
│  │ ┌──────────────┐ │           │  │ Video Encoder    │   │   │
│  │ │ Touch Input  │ │           │  │ (VP8/H.264)      │   │   │
│  │ │ Processor    │ │           │  └──────────────────┘   │   │
│  │ └──────────────┘ │           │           ▲              │   │
│  │        ▼         │           │           │              │   │
│  │ ┌──────────────┐ │           │  ┌──────────────────┐   │   │
│  │ │ TLS 1.3 +    │◄────────────┼─►│ TLS 1.3 +        │   │   │
│  │ │ E2E Encrypt  │ │  BRDF/TCP │  │ E2E Encrypt      │   │   │
│  │ └──────────────┘ │           │  └──────────────────┘   │   │
│  │        ▲         │           │           ▲              │   │
│  │        │         │           │           │              │   │
│  └────────┼─────────┘           └───────────┼──────────────┘   │
│           │                                 │                    │
│           └─────────────────────────────────┘                    │
│                    Network Link (LAN/Internet)                   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Connection Types

The system supports three types of connections:

#### 1. Local (LAN/WiFi)
- **Latency**: 1-5ms typical
- **Bandwidth**: Limited by WiFi 5/6
- **Use Case**: Home/office networks, high-speed LAN
- **Security**: Local ARP spoofing risk (mitigation: TLS certificate pinning)
- **Device Discovery**: mDNS/Bonjour

#### 2. Remote (Internet via BRDF)
- **Latency**: 30-100ms typical (depends on geographic distance)
- **Bandwidth**: Variable (ISP-dependent)
- **Use Case**: Remote work, cross-office access
- **Security**: BRDF tunnel endpoint verification + E2E encryption
- **Connection**: Relay through BRDF (Bonsai Remote Desktop Federation)

#### 3. P2P (Direct Peer-to-Peer)
- **Latency**: 5-50ms (after NAT traversal)
- **Bandwidth**: Full ISP speed
- **Use Case**: Direct connection when possible
- **Security**: Requires successful NAT traversal + E2E encryption
- **Negotiation**: STUN/TURN via BRDF signaling

### Component Interactions

```
Mobile App Layer
├── Video Decoder (Hardware-accelerated)
├── Touch Event Handler
├── Text Input Handler
├── Session Manager
└── HITL Modal System

┌─ Network Layer ─────────────────────
│  TLS 1.3 Session
│  ├─ Certificate Pinning (BRDF CA)
│  ├─ ALPN: h2/h2c (HTTP/2)
│  └─ ECE Encryption (ChaCha20-Poly1305)
│
└─ BRDF Tunnel (optional)
   ├─ Endpoint Discovery
   ├─ NAT Traversal
   └─ Relay (fallback)

Desktop Agent Layer
├─ Screen Capture (Windows: DXGI, macOS: Metal, Linux: X11)
├─ Input Handler (Windows: SendInput, macOS: CGEvent, Linux: uinput)
├─ Session Registry
└─ Event Streaming
```

---

## Security Model

### Capability Tokens

Access is controlled via **capability tokens** (JWT-based), not just authentication:

```json
{
  "iss": "bonsai-brdf",
  "sub": "device-uuid",
  "aud": "desktop-peer-uuid",
  "scope": [
    "screen:view",
    "input:touch",
    "input:text",
    "clipboard:read",
    "clipboard:write",
    "file:transfer"
  ],
  "exp": 1719864000,
  "iat": 1719777600,
  "nbf": 1719777600,
  "iss_cert": "https://brdf.bonsai.local/ca/cert.pem"
}
```

### Token Verification

Tokens are verified at connection time:

1. **Signature Verification**: Check BRDF CA signature
2. **Time Validation**: `exp > now > nbf`
3. **Audience Check**: Token's `aud` matches this desktop's peer ID
4. **Scope Validation**: Requested operation is in token's `scope`
5. **Revocation Check**: Query BRDF revocation API (cached)

### Encryption

All traffic is encrypted with **ChaCha20-Poly1305**:

```
Session Layer:
  TLS 1.3
  ├─ Cipher: TLS_CHACHA20_POLY1305_SHA256
  ├─ Key Exchange: X25519
  └─ Certificate: BRDF-signed

Application Layer:
  ChaCha20-Poly1305 (additional layer)
  ├─ Key: Derived from TLS session key
  └─ Nonce: Incremental counter (no IV reuse)
```

### HITL (Human-In-The-Loop) Modals

Sensitive operations trigger HITL modals on the desktop:

```
┌─────────────────────────────────────────┐
│  Sensitive Operation Requested          │
├─────────────────────────────────────────┤
│                                         │
│  Mobile Device: "iPhone 14 Pro"         │
│  Peer ID: 550e8400-e29b-41d4-...       │
│  Operation: File Transfer               │
│                                         │
│  Details:                               │
│  Source: /Users/me/Documents/budget.xls│
│  Direction: download                    │
│  Size: 2.3 MB                           │
│  Time Remaining: 5s for response        │
│                                         │
│  [🚫 DENY]  [✓ ALLOW]                  │
│                                         │
└─────────────────────────────────────────┘
```

**HITL Categories**:
- **File Transfer**: Any file move (up/down/delete)
- **Clipboard Access**: Reading/writing system clipboard
- **Tunnel Creation**: Setting up new network tunnels
- **Sensitive Input**: Typing in password fields

### Risk Assessment Matrix

Operations are classified by risk level:

| Operation | Risk Level | HITL Required | Auto-Deny After |
|-----------|-----------|---------------|-----------------|
| View screen | Low | No | N/A |
| Touch input | Medium | Optional | N/A |
| Text input | Medium | Optional | N/A |
| Clipboard read | High | Yes | 30s |
| File transfer | High | Yes | 30s |
| Clipboard write | Critical | Yes | 10s |
| Tunnel creation | Critical | Yes | 10s |

---

## Hardware Optimization

### Redmi Note 12 Pro 5G

**Device Specifications**:
- **SoC**: Snapdragon 6 Gen 1
- **RAM**: 6-8 GB LPDDR4X
- **Storage**: 128 GB UFS 3.1
- **Display**: 6.67" FHD+ AMOLED, 120Hz
- **Battery**: 5000 mAh
- **Codec Support**: H.264 (baseline/main), H.265 (main), VP8, VP9

#### Video Decoding Optimization

```cpp
// Hardware accelerators available on Redmi Note 12 Pro 5G:
// - Adreno 710 GPU (H.264 Main profile, H.265 Main/Main10)
// - Video Hardware Decoder (QVDEC)

// Recommended settings for this device:
H264Codec {
  profile: "Main",
  level: "4.2",
  bitrate_target: 8_500_000,  // 8.5 Mbps
  fps_target: 60,
  width: 1080,
  height: 2340,
  slice_mode: "Multi-slice",
  // Key frame interval: 2 seconds
  keyframe_interval: 120,
  // Entropy coding: CABAC (lower bitrate)
  entropy_coding: "CABAC",
}

// GPU acceleration via MediaCodec:
MediaCodec codec = MediaCodec.createDecoderByType("video/avc");
MediaFormat format = new MediaFormat();
format.setString(MediaFormat.KEY_MIME, "video/avc");
format.setInteger(MediaFormat.KEY_WIDTH, 1080);
format.setInteger(MediaFormat.KEY_HEIGHT, 2340);
format.setInteger(MediaFormat.KEY_FRAME_RATE, 60);
codec.configure(format, surface, null, 0);
```

#### Memory Management

On Redmi Note 12 Pro 5G with 6GB RAM:

```
Total Available: ~6000 MB
├─ System/Services: ~1500 MB
├─ Other Apps: ~800 MB (if minimized)
├─ Bonsai Remote Desktop:
│  ├─ Decoding Buffers (3x 4MB): 12 MB
│  ├─ Display Surface: 8 MB
│  ├─ Touch Event Queue: 1 MB
│  ├─ Network Buffers: 5 MB
│  └─ Heap Memory: ~50-100 MB
└─ Headroom: ~3000 MB
```

**Memory Optimization Strategies**:
1. **Ring Buffer for Video Frames**: Reuse decoded frame buffers
2. **Lazy Decoding**: Only decode visible regions for split screens
3. **Compressed Event Queue**: Delta compression for touch events
4. **Periodic Memory Cleanup**: GC after each 30-frame batch

#### Battery Optimization

**Battery Drain Baseline**:
- Idle (screen on): ~15 mW/min
- Video decoding (60fps): ~120 mW/min
- Remote session (typical): ~180-250 mW/min
- Expected 1-hour session: 8-12% drain

**Optimization Strategies**:

```
Battery-Efficient Settings:
├─ Adaptive FPS
│  ├─ Motion detected: 60fps
│  ├─ Static content: 30fps
│  └─ Idle (no touch): 15fps
├─ Resolution Scaling
│  ├─ WiFi 6/5G: 1080p
│  ├─ WiFi 4/LTE: 720p
│  └─ 3G/Weak: 480p
├─ Screen Refresh Optimization
│  ├─ Link to device display: 120Hz
│  ├─ Update only changed regions (dirty rects)
│  └─ Batch updates (max 16.67ms intervals)
└─ Power State Management
   ├─ Keep-alive: Prevent suspend during session
   ├─ CPU affinity: Pin decoder to P-cores
   └─ GPU throttling: Use energy-efficient modes
```

#### Thermal Management

```
Target Temperature Range: 35-45°C (optimal performance)
Throttle Threshold: 50°C
Emergency Shutdown: >60°C

Thermal Monitoring:
├─ CPU temp sensor: TZ-MONITOR-CPU
├─ GPU temp sensor: TZ-MONITOR-GPU
├─ Battery temp sensor: TZ-MONITOR-BATT
└─ Action:
   ├─ 45°C: Reduce FPS to 45
   ├─ 50°C: Reduce FPS to 30, bitrate -25%
   └─ 55°C: Pause session, notify user
```

---

## Performance Tuning

### Bitrate Adaptation

The system automatically adjusts bitrate based on network conditions:

```
Network Monitor:
├─ Bandwidth estimation (NADA algorithm)
│  └─ Sample RTT every 200ms
├─ Packet loss detection
│  └─ Count NACK frames
└─ Jitter measurement
   └─ Track arrival time variance

Adaptation Logic:
If (packet_loss > 5% || bandwidth_estimate < target):
  bitrate = bitrate * 0.9  // -10%
  keyframe_interval *= 1.2 // Less frequent
Else if (packet_loss < 1% && available_bw > bitrate * 1.2):
  bitrate = bitrate * 1.05 // +5%
  keyframe_interval *= 0.95 // More frequent

Limits:
├─ Min: 500 Kbps
├─ Max: 25 Mbps
└─ Typical: 5-10 Mbps
```

### Frame Skipping Strategy

```
decode_frame(frame):
  if frame.is_keyframe:
    skip_count = 0
    return decode(frame)
  
  if skip_count < max_skip:
    // Check if network is congested
    if network.estimated_bitrate < target * 0.8:
      skip_count += 1
      return skip(frame)
  
  skip_count = 0
  return decode(frame)

max_skip:
  = floor(1000 / target_fps) / frame_interval_ms
  = floor(16.67 / 16.67) = 1 frame max skip at 60fps
```

### Latency Benchmarks

Expected latencies on Redmi Note 12 Pro 5G:

```
Scenario                      Touch-to-Visible  p95  p99
─────────────────────────────────────────────────────────
Local Network (WiFi 6)        25-35ms           40ms 50ms
Local Network (WiFi 5)        30-45ms           60ms 75ms
Remote (BRDF, same region)    40-60ms           80ms 120ms
Remote (BRDF, distant region) 100-150ms         200ms 250ms
Internet (P2P via TURN)       50-100ms          150ms 200ms
```

### Tuning Parameters

```toml
[remote_desktop.performance]
# Video encoding
h264_profile = "main"
h264_level = "4.2"
target_bitrate_kbps = 8500
target_fps = 60
keyframe_interval_secs = 2

# Network
rtt_probe_interval_ms = 200
bitrate_window_ms = 1000
adaptation_rate = 0.05

# Decoding (mobile)
decode_buffer_count = 3
frame_skip_max = 1
jitter_buffer_ms = 100

# Display
dirty_rect_enabled = true
dirty_rect_merge_distance = 10
refresh_rate_hz = 120

# Touch
touch_debounce_ms = 5
touch_coalescence_ms = 8

# Power management
adaptive_fps = true
fps_static_content = 30
fps_idle = 15
battery_saver_threshold_percent = 20
```

---

## Troubleshooting

### Connection Issues

#### Problem: "Peer offline" message

**Diagnosis**:
```bash
# Check desktop status
adb shell am broadcast -a "com.bonsai.REMOTE_DESKTOP" -e "action" "status"

# Check network connectivity
ping desktop-ip

# Check firewall
sudo firewall-cmd --list-all
```

**Solutions**:
1. Ensure desktop is running Bonsai daemon: `bonsai daemon --remote-desktop`
2. Check NAT/firewall rules (ensure port 5900 accessible)
3. Verify WiFi/network connection
4. Try switching networks (WiFi ↔ cellular)
5. Restart both apps

#### Problem: Slow video/laggy response

**Diagnosis**:
```bash
# Check network latency
ping -c 10 desktop-ip
# Look for >100ms latency or packet loss

# Check video decode performance
adb logcat | grep -i "remote.*decode"

# Check bitrate adaptation
adb logcat | grep -i "remote.*bitrate"
```

**Solutions**:
1. Move closer to WiFi router (signal strength should be >-50 dBm)
2. Switch from 2.4GHz to 5GHz WiFi band
3. Disable other bandwidth-heavy apps
4. Reduce resolution: Settings > Remote Desktop > Resolution > 720p
5. Lower target FPS: Settings > Remote Desktop > FPS > 30fps
6. Check remote device CPU load (may be overloaded)

#### Problem: Video won't decode / black screen

**Diagnosis**:
```bash
adb logcat | grep -E "(MediaCodec|video|decode)"
# Look for: "Error -2147483648" = codec initialization failure
#           "Error 1" = input buffer not available
```

**Solutions**:
1. Verify codec support: `adb shell dumpsys media.audio_flinger | grep -A 10 "h264"`
2. Update device firmware
3. Restart MediaCodec by disconnecting and reconnecting
4. Try VP8 codec instead (if available)

### Audio Issues

#### Problem: No audio transmission

**Status**: Audio support is planned for v2.0

**Workaround**: Use phone's native speaker for desktop audio via system speakers

### Battery Drain

#### Problem: Rapid battery drain (>20% per hour)

**Diagnosis**:
```bash
adb shell dumpsys battery
# Check: Current_now (mA)
# Multiply by voltage (typically 4.35V) = Power (W)

adb shell top -n 1 | grep -i bonsai
# Check CPU usage
```

**Solutions**:
1. Enable Battery Saver: Settings > Remote Desktop > Battery Saver
2. Reduce FPS to 30 fps
3. Use 720p instead of 1080p
4. Close background apps
5. Reduce screen brightness (especially for AMOLED)

---

## Advanced Topics

### Custom Codec Profiles

For specialized use cases, create custom H.264 profiles:

```
[remote_desktop.codec_profiles.high_latency]
# For high-latency networks (>100ms RTT)
profile = "baseline"
bitrate_kbps = 4000
fps = 30
keyframe_interval_secs = 1  # More frequent key frames
encoding_mode = "adaptive_b_frame"

[remote_desktop.codec_profiles.low_bandwidth]
# For cellular/poor networks
profile = "baseline"
bitrate_kbps = 1000
fps = 15
width = 480
height = 854
slice_mode = "Single-slice"
entropy_coding = "CAVLC"  # Lower bitrate than CABAC
```

### Custom Event Processors

Extend input handling with custom processors:

```rust
pub trait InputProcessor {
  fn process_touch(&mut self, event: TouchEvent) -> Result<ProcessedEvent>;
  fn process_key(&mut self, event: KeyEvent) -> Result<ProcessedEvent>;
  fn process_text(&mut self, event: TextEvent) -> Result<ProcessedEvent>;
}

// Example: Swipe gesture recognizer
struct SwipeGestureProcessor {
  touch_history: VecDeque<TouchEvent>,
  threshold_velocity: f32,
}

impl InputProcessor for SwipeGestureProcessor {
  fn process_touch(&mut self, event: TouchEvent) -> Result<ProcessedEvent> {
    self.touch_history.push_back(event);
    
    // Detect swipe after 3+ points
    if self.touch_history.len() >= 3 {
      let velocity = self.calculate_velocity();
      if velocity > self.threshold_velocity {
        return Ok(ProcessedEvent::Swipe(self.get_direction()));
      }
    }
    
    Ok(ProcessedEvent::Touch(event))
  }
  
  // ... other methods
}
```

### Custom Network Transports

Implement custom transport layers:

```rust
pub trait RemoteTransport {
  async fn connect(&mut self, peer_id: &str, token: &str) -> Result<()>;
  async fn send_frame(&mut self, frame: &Frame) -> Result<()>;
  async fn recv_frame(&mut self) -> Result<Frame>;
  async fn close(&mut self) -> Result<()>;
}

// Example: QUIC transport (lower latency)
struct QUICTransport {
  endpoint: quinn::Endpoint,
  connection: Option<quinn::Connection>,
}

impl RemoteTransport for QUICTransport {
  // ... implementation
}
```

### Metrics & Monitoring

Monitor session health with real-time metrics:

```
POST /metrics/session/{session_id}
Content-Type: application/json

{
  "timestamp": "2024-06-30T10:30:45Z",
  "session_id": "...",
  "metrics": {
    "video": {
      "fps_actual": 58,
      "fps_target": 60,
      "bitrate_actual": 8240,
      "bitrate_target": 8500,
      "decoded_frames": 3480,
      "dropped_frames": 2,
      "decode_time_avg_ms": 16.2,
      "decode_time_max_ms": 28.5
    },
    "network": {
      "latency_p50_ms": 2.1,
      "latency_p95_ms": 4.3,
      "latency_p99_ms": 5.8,
      "packet_loss_percent": 0.1,
      "available_bitrate_kbps": 9200,
      "jitter_ms": 0.8
    },
    "input": {
      "touch_events_sec": 5.2,
      "key_events_sec": 12.1,
      "touch_to_visible_latency_p95_ms": 35
    },
    "device": {
      "cpu_usage_percent": 42,
      "memory_usage_mb": 85,
      "gpu_usage_percent": 78,
      "temperature_celsius": 38,
      "battery_drain_percent_per_hour": 10.2
    }
  }
}
```

---

## Support & Resources

- **Community Forum**: https://forum.bonsai.local/remote-desktop
- **Issue Tracker**: https://github.com/bonsai-project/bonsai/issues
- **Security Reports**: security@bonsai.local
- **API Documentation**: See `API_REFERENCE.md`

---

**Last Updated**: 2024-06-30
**Version**: 1.0
