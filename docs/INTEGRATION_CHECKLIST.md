# Mobile Remote Desktop Integration Checklist

Pre-deployment verification and benchmarking procedure.

## Phase 1: Core Functionality Verification

### [ ] MCP Tools Registration
- [ ] `mobile_start_remote_session` tool registered
- [ ] `mobile_stop_remote_session` tool registered
- [ ] `mobile_inject_text` tool registered
- [ ] `mobile_take_screenshot` tool registered
- [ ] `mobile_get_session_stats` tool registered
- [ ] `mobile_list_available_peers` tool registered
- [ ] All tools appear in `/mcp/tools/list` endpoint
- [ ] All tools have complete input schemas
- [ ] All tools have proper error handling

**Verification Command**:
```bash
curl -X POST http://localhost:8000/mcp/tools/list \
  -H "Authorization: Bearer $TOKEN"
# Response should list all 6 mobile tools
```

### [ ] Session Management
- [ ] SessionRegistry initializes correctly
- [ ] Sessions can be created with `create_session()`
- [ ] Sessions can transition through all states
- [ ] Sessions can be retrieved by ID
- [ ] Sessions can be properly closed
- [ ] Peer registration works
- [ ] Peer filtering by status works

**Test Command**:
```bash
cargo test -p bonsai-mcp-server --lib mobile_session -- --nocapture
# All tests should PASS
```

### [ ] UACS Integration
- [ ] RemoteSessionStarted event emits
- [ ] RemoteSessionEnded event emits
- [ ] RemoteFileTransferRequest event emits
- [ ] RemoteClipboardAccess event emits
- [ ] RemoteTunnelCreated event emits
- [ ] RemoteSessionStats event emits
- [ ] HITL approval works for sensitive operations
- [ ] WebSocket events stream correctly

**Test**:
1. Connect mobile session
2. Verify events appear in WebSocket listener:
```bash
websocat ws://localhost:8000/ws/events
# Should show RemoteSessionStarted event
```

### [ ] BTI Commands
- [ ] `:remote connect <peer_id>` executes
- [ ] `:remote disconnect <session_id>` executes
- [ ] `:remote list` lists all peers
- [ ] `:remote stats <session_id>` shows statistics
- [ ] `:remote screenshot <session_id>` captures screen
- [ ] All commands have helpful error messages
- [ ] Commands work in BTI terminal

**Test**:
```bash
# In BTI terminal
:remote list
# Should display available peers
```

---

## Phase 2: Security Review

### [ ] Authentication & Authorization
- [ ] Capability tokens verified at session start
- [ ] Token expiration enforced
- [ ] Token scope validation works
- [ ] Revocation check implemented
- [ ] Unauthorized access rejected with 401

**Test**:
```bash
# Attempt connection with expired token
curl -X POST http://localhost:8000/api/remote/session/start \
  -H "Authorization: Bearer EXPIRED_TOKEN" \
  -d '{"peer_id":"test"}'
# Should respond with 401 Unauthorized
```

### [ ] Encryption
- [ ] TLS 1.3 enforced
- [ ] ChaCha20-Poly1305 enabled for application layer
- [ ] Certificate pinning configured
- [ ] No unencrypted traffic
- [ ] Session keys rotated properly

**Test**:
```bash
# Check TLS version
openssl s_client -connect localhost:5900 -tls1_3
# Should show: TLSv1.3
```

### [ ] Input Validation
- [ ] Text injection validates input length (<10KB)
- [ ] Session IDs validated as UUIDs
- [ ] Peer IDs validated
- [ ] File paths validated (no directory traversal)
- [ ] Screenshot quality bounds checked (0-100)

**Test**:
```bash
# Attempt invalid input
curl -X POST http://localhost:8000/api/remote/inject-text \
  -d '{"session_id":"invalid", "text":"test"}'
# Should respond with validation error
```

### [ ] HITL Modal Security
- [ ] File transfer requires explicit approval
- [ ] Clipboard access requires approval
- [ ] Tunnel creation requires approval
- [ ] Approval timeout after 30 seconds
- [ ] Denial logged and audited

**Test**:
1. Request sensitive operation (file transfer)
2. Verify HITL modal appears on desktop
3. Deny operation
4. Verify denial is logged

---

## Phase 3: Performance Benchmarking

### [ ] Video Decode Performance

**Setup**: Redmi Note 12 Pro 5G connected to local WiFi network

**Test 1: Baseline Decode Latency**
```bash
# Measure frame decode time (p95)
# Expected: <30ms for local H.264

# Command on desktop:
bonsai benchmark --test remote-video-decode --duration 60 --bitrate 8500

# Read results:
cat benchmark-results.json | jq '.metrics.decode_latency_p95_ms'
# Expected: < 30
```

**Test 2: Long-Duration Stability (1 hour)**
```bash
# Run session for 1 hour, monitor for stalls/crashes
bonsai benchmark --test remote-session-stability --duration 3600

# Check for drops/restarts:
grep "ERROR\|RESTART\|DROP" benchmark.log
# Expected: 0 errors
```

### [ ] Touch Input Latency

**Test 1: Touch-to-Visible Latency (p95)**
```bash
# On mobile, run latency test
adb shell am instrument -w \
  com.bonsai.remote_desktop/androidx.test.runner.AndroidJUnitRunner \
  -e class com.bonsai.remote_desktop.tests.LatencyTest

# Read results:
adb shell cat /data/local/tmp/latency-test.json | jq '.p95_ms'
# Expected: < 50ms for local network
# Expected: < 100ms for BRDF tunnel
```

**Test 2: Touch Input Under Load**
```bash
# Simulate heavy touch input (10 taps/sec for 5 minutes)
adb shell am start -W -n com.bonsai.remote_desktop/.tests.TouchInputStressTest \
  --es duration 300 --ei taps_per_sec 10

# Check for input queuing:
adb logcat | grep "touch.*queue\|input.*backlog"
# Expected: No backlog warnings
```

### [ ] Network Performance

**Test 1: Local Network (WiFi 5GHz)**
```bash
# Measure bitrate adaptation
bonsai benchmark --test remote-network --network local --duration 300

# Expected metrics:
# - RTT: 1-5ms
# - Packet loss: <0.1%
# - Bitrate achieved: 8-9 Mbps
# - Frame rate sustained: 58-60 fps
```

**Test 2: Remote Network (BRDF via WAN)**
```bash
# Simulate distant connection (BRDF tunnel)
bonsai benchmark --test remote-network --network brdf --duration 300 \
  --brdf-endpoint us-east-1.brdf.bonsai.local

# Expected metrics:
# - RTT: 50-100ms
# - Bitrate adaptation: Drops to 4-6 Mbps
# - Frame rate: 45-60 fps (adaptive)
# - No timeout errors
```

**Test 3: Poor Network Conditions**
```bash
# Simulate packet loss & jitter with tc (Linux)
sudo tc qdisc add dev eth0 root netem loss 5% latency 100ms jitter 20ms

# Run benchmark
bonsai benchmark --test remote-network --duration 300

# Expected:
# - Bitrate reduces to 2-3 Mbps
# - FPS drops to 30 fps
# - No session disconnects
# - Error rate <1%

# Cleanup
sudo tc qdisc del dev eth0 root
```

### [ ] Memory & Battery Usage

**Test 1: Memory Footprint**
```bash
# Measure peak memory usage over 1-hour session
adb shell dumpsys meminfo com.bonsai.remote_desktop > memory-before.txt

# Run session for 60 minutes
# Every 10 minutes: adb shell top -n 1 >> memory-samples.txt

adb shell dumpsys meminfo com.bonsai.remote_desktop > memory-after.txt

# Analyze
grep "TOTAL\|Native\|Java" memory-after.txt
# Expected: < 150 MB peak
# Expected: < 20 MB growth per 10 minutes (leak detection)
```

**Test 2: Battery Drain**
```bash
# Measure battery drain rate
adb shell dumpsys battery > battery-before.txt
# Note: level = X %

# Run session for 60 minutes
# At 60 minutes:
adb shell dumpsys battery > battery-after.txt
# Note: level = Y %

# Calculate drain rate
# drain_percent = X - Y
# Expected: 8-12% per hour in normal mode
# Expected: 5-8% per hour in Battery Saver mode
```

**Test 3: Thermal Load**
```bash
# Monitor temperature during 1-hour session
adb shell dumpsys thermal > thermal-baseline.txt

# Run session, collect samples every 5 minutes
for i in {1..12}; do
  sleep 300
  adb shell dumpsys thermal >> thermal-samples.txt
done

# Analyze peak temperature
grep "CurrentTemperature_millidegrees_C" thermal-samples.txt | \
  awk -F: '{print $2}' | sort -rn | head -1
# Expected: < 50°C peak
# Expected: < 45°C sustained average
```

### [ ] Session Stability

**Test 1: Connection Drop Recovery**
```bash
# Test recovery from network interruption
# Start session
# After 30 seconds:
# - Disable WiFi for 10 seconds
# - Re-enable WiFi

# Monitor logs:
adb logcat | grep -i "disconnect\|reconnect\|resume"

# Expected:
# - Immediate "Disconnected" message
# - Auto-reconnect within 5 seconds
# - Session resumes without user intervention
```

**Test 2: Long Session Stability (4 hours)**
```bash
# Run session for 4 hours continuously
# Every 30 minutes, perform light activity
# (tap screen, scroll, take screenshot)

# Monitor for:
grep "ERROR\|CRASH\|RESTART" app.log
# Expected: 0 errors

# Expected FPS variation: < 3% deviation
# Expected latency variation: < 10% deviation
```

---

## Phase 4: Functional Testing

### [ ] Session Lifecycle

- [ ] Create session with valid peer_id
- [ ] Verify SessionStatus transitions:
  - [ ] Connecting → Connected
  - [ ] Connected → Streaming
  - [ ] Streaming → Paused (network issue)
  - [ ] Paused → Streaming (recovered)
  - [ ] Streaming → Disconnected
- [ ] Close session gracefully
- [ ] Verify session removed from registry

**Test Commands**:
```bash
# Create session
SESSION_ID=$(curl -s -X POST http://localhost:8000/api/remote/session \
  -d '{"peer_id":"test-peer"}' | jq -r '.session_id')

# Check status
curl http://localhost:8000/api/remote/session/$SESSION_ID | jq '.status'

# Close session
curl -X DELETE http://localhost:8000/api/remote/session/$SESSION_ID
```

### [ ] Input Injection

- [ ] Touch input accepted
- [ ] Text injection works for ASCII
- [ ] Text injection works for Unicode
- [ ] Very long text (10KB) rejected
- [ ] Key codes properly mapped
- [ ] Multi-touch recognized

**Test**:
```bash
# Inject text
curl -X POST http://localhost:8000/api/remote/inject-text \
  -d '{"session_id":"'$SESSION_ID'", "text":"Hello World"}'
# Expected: Text appears on desktop screen

# Inject touch
curl -X POST http://localhost:8000/api/remote/inject-touch \
  -d '{"session_id":"'$SESSION_ID'", "x":500, "y":1000, "action":"DOWN"}'
# Expected: Cursor appears at coordinates
```

### [ ] Screenshot Capture

- [ ] Screenshot returns base64-encoded image
- [ ] Quality parameter affects file size
  - [ ] Quality 50: ~200KB
  - [ ] Quality 85: ~400KB
  - [ ] Quality 100: ~600KB
- [ ] Format conversion (JPEG/PNG) works
- [ ] Timestamp accurate

**Test**:
```bash
# Take screenshot
curl -X POST http://localhost:8000/api/remote/screenshot \
  -d '{"session_id":"'$SESSION_ID'"}' \
  | jq -r '.image_base64' \
  | base64 -d > screenshot.jpg

# Check file size and validity
file screenshot.jpg
# Expected: JPEG image data
```

### [ ] Statistics Streaming

- [ ] Stats contain all required fields:
  - [ ] fps
  - [ ] bitrate_mbps
  - [ ] latency_ms
  - [ ] bandwidth_usage_mb
  - [ ] frames_decoded
  - [ ] frames_dropped
  - [ ] connection_uptime_secs
  - [ ] battery_drain_percent_per_hour
- [ ] Stats update every 1 second
- [ ] Values are realistic and monotonic

**Test**:
```bash
# Get stats
curl http://localhost:8000/api/remote/session/$SESSION_ID/stats

# Expected response:
# {
#   "fps": 59.8,
#   "bitrate_mbps": 8.4,
#   "latency_ms": 2.3,
#   "bandwidth_usage_mb": 42.5,
#   "frames_decoded": 3600,
#   "frames_dropped": 2,
#   "connection_uptime_secs": 60,
#   "battery_drain_percent_per_hour": 10.2
# }
```

### [ ] Peer Discovery

- [ ] List all peers without filter
- [ ] Filter peers by status (online/offline)
- [ ] Peer info contains all fields:
  - [ ] peer_id
  - [ ] device_name
  - [ ] device_model
  - [ ] last_seen
  - [ ] status
  - [ ] local_ip
  - [ ] is_trusted
- [ ] Trusted peers marked correctly

**Test**:
```bash
curl http://localhost:8000/api/remote/peers | jq '.peers | length'
# Expected: > 0 (at least desktop registered)

curl 'http://localhost:8000/api/remote/peers?status=online' | jq '.peers'
# Expected: Array of online peers
```

---

## Phase 5: UI/UX Testing

### [ ] Mobile Interface
- [ ] App launches without crashes
- [ ] Settings panel accessible
- [ ] Performance sliders functional
- [ ] Session history shows all past connections
- [ ] Floating toolbar doesn't interfere
- [ ] Notifications appear correctly

**Manual Test**:
1. Open Bonsai Remote Desktop app
2. Navigate: Settings > Remote Desktop
3. Change: Resolution, FPS, Bitrate
4. Verify: Changes apply to active session
5. Check: No lag or stutter during changes

### [ ] Desktop HITL Interface
- [ ] HITL modals appear for sensitive ops
- [ ] Modal shows 30-second countdown
- [ ] Approve button works
- [ ] Deny button works
- [ ] Timeout auto-denies
- [ ] Approved operations logged

**Manual Test**:
1. Request file transfer from mobile
2. HITL modal should appear on desktop
3. Verify: Operation details shown
4. Tap "Approve" or let timeout
5. Check: Operation proceeds/denied accordingly

---

## Phase 6: Deployment Readiness

### [ ] Code Quality
- [ ] All tests pass: `cargo test --workspace`
- [ ] No clippy warnings: `cargo clippy --all`
- [ ] No unsafe code (or marked with // SAFETY)
- [ ] Error handling complete (no unwrap() in production)
- [ ] Documentation complete (all pub functions documented)

### [ ] Build Artifacts
- [ ] Linux daemon builds: `cargo build --release -p bonsai-mcp-server`
- [ ] APK builds: `./build-apk.sh --release`
- [ ] APK size reasonable (<50 MB)
- [ ] No debug symbols in release build
- [ ] Signatures correct

**Build Verification**:
```bash
# Build release daemon
cargo build --release -p bonsai-mcp-server
ls -lh target/release/bonsai-mcp-server
# Expected: < 10 MB (after strip)

# Check APK size
./build-apk.sh --release
ls -lh *.apk
# Expected: < 40 MB
```

### [ ] Documentation
- [ ] MOBILE_REMOTE_DESKTOP.md complete (1000+ lines)
- [ ] REDMI_SETUP_GUIDE.md complete (500+ lines)
- [ ] API_REFERENCE.md complete with examples
- [ ] Troubleshooting guide comprehensive
- [ ] Code examples work and are tested

### [ ] Security Audit
- [ ] No hardcoded credentials
- [ ] No debug logs in production
- [ ] Certificate pinning configured
- [ ] Token validation enforced
- [ ] Rate limiting configured
- [ ] Input validation complete
- [ ] Error messages don't leak info

**Quick Audit**:
```bash
# Check for hardcoded secrets
grep -r "password\|secret\|token\|key" src/ --include="*.rs" | \
  grep -v "test\|example\|TODO" | head -5

# Check for debug prints
grep -r "println!\|dbg!\|eprintln!" src/ | grep -v "test"

# Check for unsafe code
grep -r "unsafe" src/ | grep -v "// SAFETY"
```

---

## Phase 7: Release Checklist

### [ ] Final Verification
- [ ] All checklist items above completed
- [ ] All tests passing
- [ ] Performance benchmarks acceptable
- [ ] Security audit passed
- [ ] Documentation reviewed
- [ ] Example use cases validated

### [ ] Release Notes
- [ ] Version number bumped
- [ ] Changelog updated
- [ ] Known issues listed
- [ ] Breaking changes documented
- [ ] Migration guide provided (if applicable)

### [ ] Release Build
- [ ] Production build created
- [ ] Artifacts signed
- [ ] Checksums calculated
- [ ] Release notes published
- [ ] Update channels notified

**Checklist Sign-off**:

```
Release: v1.0.0
Date: ________________
By: ________________

Approved By:
- Technical Lead: ________________
- Security Review: ________________
- Product Manager: ________________

Status: ☐ READY FOR PRODUCTION
```

---

**Last Updated**: 2024-06-30
