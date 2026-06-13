# Phases 17B-19: Detailed Implementation Outline
## Complete Titanium Zigbee, Aether Z-Wave, and Integration

---

## PHASE 17B: TITANIUM MAC (Medium Access Control)
**Duration**: 1.5 weeks | **LOC**: 4,000 | **Crates**: 7 | **Tests**: 90

### omnisystem-titanium-mac-core (1,200 LOC)

```rust
/// CSMA/CA with QoS support
pub struct CsmaCA {
    cw: u8,                    // Contention Window
    nb: u8,                    // Backoff count
    be: u8,                    // Backoff Exponent
    qos_queue: Vec<QueuedFrame>,
    stats: MacStats,
}

impl CsmaCA {
    pub async fn send(&mut self, frame: &Frame, priority: u8) -> Result<(), String> {
        // Queue frame by priority (0=highest, 15=lowest)
        self.qos_queue.push(QueuedFrame {
            frame: frame.clone(),
            priority,
            timestamp: now_us(),
        });

        // Sort by priority
        self.qos_queue.sort_by_key(|f| std::cmp::Reverse(f.priority));

        loop {
            // Clear Channel Assessment
            if self.radio.cca().await? {
                // Channel clear, send immediately
                self.radio.transmit(&frame).await?;
                self.stats.frames_sent += 1;
                return Ok(());
            }

            // Channel busy, backoff
            let backoff_time = Self::calculate_backoff(self.be);
            tokio::time::sleep(Duration::from_micros(backoff_time as u64)).await;

            // Increase backoff exponent (exponential backoff)
            self.be = std::cmp::min(self.be + 1, 8);

            // Track attempts
            self.nb += 1;
            if self.nb > 5 {
                self.stats.frames_dropped += 1;
                return Err("Max retries exceeded".to_string());
            }
        }
    }

    fn calculate_backoff(be: u8) -> u16 {
        // Standard IEEE 802.15.4 backoff: 2^BE * UNIT_BACKOFF_PERIOD
        // UNIT_BACKOFF_PERIOD = 320 microseconds
        let max_backoff = (1 << be) as u16;
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        let random = RandomState::new().build_hasher().finish() as u16;
        (random % max_backoff) * 320
    }
}

/// ACK handling with predictive timing
pub struct AckManager {
    ack_timeout_us: u32,
    expected_ack_time: HashMap<u16, u64>,  // SeqNum → expected time
}

impl AckManager {
    pub async fn wait_for_ack(&self, seq_num: u16, device_sleep_info: Option<&SleepInfo>) 
        -> Result<bool, String> 
    {
        let timeout = if let Some(sleep) = device_sleep_info {
            // Predict when device will wake to receive ACK
            let wake_time = sleep.next_wake_us();
            std::cmp::max(self.ack_timeout_us, wake_time as u32)
        } else {
            // Device always awake, standard timeout
            self.ack_timeout_us
        };

        // Wait for ACK or timeout
        // ...
        Ok(true)
    }
}
```

**Key Innovations**:
- QoS-aware queuing (4 priority levels)
- Predictive ACK timing for sleepy devices
- Sub-100ms duty cycle support
- Piggyback ACK optimization (ACK + data in one frame)

### omnisystem-titanium-frame (800 LOC)

```rust
/// IEEE 802.15.4 frame structure with Titanium extensions
pub struct IeeeFrame {
    // MAC header
    frame_control: u16,
    seq_number: u8,
    
    // Addressing
    dest_pan_id: u16,
    dest_addr: Address,
    src_pan_id: u16,
    src_addr: Address,
    
    // Payload
    payload: Vec<u8>,
    
    // Security (if needed)
    security_header: Option<SecurityHeader>,
    
    // Titanium extensions
    qos_header: Option<QosHeader>,
    timestamp: Option<u64>,
    
    // FCS (Frame Check Sequence)
    fcs: u16,
}

impl IeeeFrame {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Frame control
        bytes.extend_from_slice(&self.frame_control.to_le_bytes());
        bytes.push(self.seq_number);
        
        // Addressing
        bytes.extend_from_slice(&self.dest_pan_id.to_le_bytes());
        bytes.extend_from_slice(&self.dest_addr.to_bytes());
        // ... etc
        
        // Payload
        bytes.extend_from_slice(&self.payload);
        
        // Calculate and append FCS
        let fcs = Self::calculate_fcs(&bytes);
        bytes.extend_from_slice(&fcs.to_le_bytes());
        
        bytes
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        // Parse from raw bytes with error checking
        Ok(Self { /* ... */ })
    }

    fn calculate_fcs(data: &[u8]) -> u16 {
        // CRC-16-CCITT
        let mut crc: u16 = 0;
        for byte in data {
            for i in 0..8 {
                let bit = (*byte >> i) & 1;
                let c15 = (crc >> 15) & 1;
                crc <<= 1;
                if (c15 ^ bit as u16) != 0 {
                    crc ^= 0x1021;
                }
            }
        }
        crc
    }
}

/// Titanium QoS header extension
pub struct QosHeader {
    priority: u8,               // 0-15
    latency_requirement_ms: u16,
    reliability_level: u8,      // 0=best effort, 3=guaranteed
}
```

**Features**:
- Compressed headers (save 5-8 bytes vs standard)
- Extension fields for QoS/timing
- Metadata tagging (RSSI, LQI)
- CRC error detection

### omnisystem-titanium-addressing (600 LOC)
- PAN ID management
- Dynamic short address allocation
- IEEE extended (64-bit) addressing
- Group addressing
- Broadcast handling

### omnisystem-titanium-ack-optimization (700 LOC)
- Immediate ACK (standard)
- Delayed ACK for sleepy devices
- Piggyback ACK (combine with response data)
- Negative ACK (explicit rejection)
- Predictive ACK timing

### omnisystem-titanium-qos (500 LOC)
- 4-level priority queuing
- Latency guarantees per frame
- Bandwidth reservation
- Congestion detection and handling

### omnisystem-titanium-power (600 LOC)
- Duty cycle tracking and reporting
- Wake scheduling for sleepy devices
- Power state management (TX, RX, Sleep)
- Energy budgeting

### omnisystem-titanium-mac-diagnostics (600 LOC)
- Frame counters and sequencing
- Link quality metrics (LQI)
- Interference detection
- Performance statistics

---

## PHASE 17C: TITANIUM NETWORK LAYER (6LoWPAN & AODV Routing)
**Duration**: 2 weeks | **LOC**: 5,000 | **Crates**: 8 | **Tests**: 120

### omnisystem-titanium-6lowpan-core (1,200 LOC)

```rust
/// IPv6 Header Compression (IPHC)
pub struct IphcCompressor;

impl IphcCompressor {
    /// Compress IPv6 header from 40 bytes to ~2-5 bytes in most cases
    pub fn compress(ipv6_header: &Ipv6Header) -> Vec<u8> {
        let mut compressed = Vec::new();
        
        // IPHC dispatch byte
        let mut dispatch = 0x60u8;  // IPv6 compressed
        
        // Version, Traffic Class, Flow Label compression
        if ipv6_header.traffic_class == 0 && ipv6_header.flow_label == 0 {
            dispatch |= 0x01;  // Both can be elided
        }
        
        compressed.push(dispatch);
        
        // Address compression
        // Link-local addresses can be compressed to just interface ID (8 bytes vs 16)
        // Multicast addresses can be compressed
        // Global addresses with known prefix can be compressed
        
        // ... conditional fields based on compression flags
        
        compressed
    }

    pub fn decompress(compressed: &[u8]) -> Result<Ipv6Header, String> {
        // Reverse of compression
        Ok(Ipv6Header { /* ... */ })
    }
}

/// Fragment reassembly with timeout
pub struct FragmentReassembler {
    active_reassemblies: HashMap<(u16, u8), PendingFragment>,
}

impl FragmentReassembler {
    pub fn add_fragment(&mut self, fragment: Fragment) -> Option<Vec<u8>> {
        let key = (fragment.tag, fragment.src_addr);
        
        let reassembly = self.active_reassemblies
            .entry(key)
            .or_insert_with(|| PendingFragment::new(fragment.total_size));
        
        reassembly.add_fragment(fragment);
        
        if reassembly.is_complete() {
            self.active_reassemblies.remove(&key);
            return Some(reassembly.get_data());
        }
        
        None
    }
}
```

**Compression Examples**:
- IPv6 header: 40 bytes → 2-5 bytes (95% compression)
- UDP header: 8 bytes → 1-2 bytes
- Full UDP/IPv6: 48 bytes → 3-7 bytes

### omnisystem-titanium-routing (1,500 LOC)

```rust
/// AODV (Ad Hoc On-Demand Distance Vector) routing
pub struct AodvRouter {
    routing_table: HashMap<Ipv6Addr, RouteEntry>,
    seq_number: u32,
    rreq_id: u16,
}

#[derive(Clone)]
pub struct RouteEntry {
    pub destination: Ipv6Addr,
    pub next_hop: Ipv6Addr,
    pub hop_count: u8,
    pub seq_number: u32,
    pub lifetime_ms: u64,
    pub created_at: u64,
}

impl AodvRouter {
    pub async fn find_route(&mut self, dest: Ipv6Addr) -> Option<Ipv6Addr> {
        // Check if route exists and not expired
        if let Some(entry) = self.routing_table.get(&dest) {
            if !entry.is_expired() {
                return Some(entry.next_hop);
            }
        }

        // Route not found, initiate RREQ (Route Request)
        self.send_rreq(dest).await;

        // Wait for RREP (Route Reply)
        // ...

        self.routing_table
            .get(&dest)
            .map(|e| e.next_hop)
    }

    async fn send_rreq(&mut self, dest: Ipv6Addr) {
        self.rreq_id = self.rreq_id.wrapping_add(1);
        
        let rreq = RouteRequest {
            rreq_id: self.rreq_id,
            seq_number: self.seq_number,
            hop_count: 0,
            dest_addr: dest,
            dest_seq: 0,
        };

        // Broadcast RREQ
        // Receiving nodes will forward if not destination
        // Destination will send RREP back to source
    }

    pub fn handle_rrep(&mut self, rrep: &RouteReply) {
        let entry = RouteEntry {
            destination: rrep.dest_addr,
            next_hop: rrep.src_addr,
            hop_count: rrep.hop_count,
            seq_number: rrep.seq_number,
            lifetime_ms: 5000,
            created_at: now_ms(),
        };

        self.routing_table.insert(rrep.dest_addr, entry);
    }

    pub fn handle_rerr(&mut self, rerr: &RouteError) {
        // Route broken, remove from table
        for dest in &rerr.unreachable_dests {
            self.routing_table.remove(dest);
        }
    }
}
```

**Features**:
- On-demand route discovery (lazy)
- Link quality routing (avoid bad links)
- Multipath routing (up to 3 backup paths)
- Self-healing (detect failures, reroute in <100ms)

### omnisystem-titanium-neighbor-discovery (700 LOC)
- Router advertisements
- Prefix management
- Duplicate address detection (DAD)

### omnisystem-titanium-icmpv6 (600 LOC)
- Echo Request/Reply (ping)
- Destination Unreachable
- Time Exceeded
- Neighbor Solicitation/Advertisement

### omnisystem-titanium-fragmentation (500 LOC)
- Smart fragment sizing
- Reordering
- Timeout handling

### omnisystem-titanium-rpl (800 LOC)
- RPL (IPv6 Routing Protocol for Low-Power Devices)
- DAG (Directed Acyclic Graph) construction
- Root selection
- DODAG repair

### omnisystem-titanium-mesh-repair (700 LOC)
- Continuous link monitoring
- Broken link detection
- Automatic rerouting
- Sub-second healing time

### omnisystem-titanium-network-diagnostics (400 LOC)
- Topology visualization
- Hop count monitoring
- Route stability metrics

---

## PHASE 17D: TITANIUM APS (Application Support Sublayer)
**Duration**: 1.5 weeks | **LOC**: 3,500 | **Crates**: 5 | **Tests**: 80

### omnisystem-titanium-aps-core (1,000 LOC)
- Frame structure and parsing
- Routing decisions
- Delivery confirmation
- Endpoint management

### omnisystem-titanium-endpoints (700 LOC)
- Endpoint discovery
- Cluster definitions
- Attribute definitions

### omnisystem-titanium-binding (800 LOC)
- Source binding (device → device)
- Destination binding
- Group binding
- Smart binding suggestions

### omnisystem-titanium-aps-security (600 LOC)
- Link keys (device-to-device)
- Network key (all devices)
- Key rotation and updates
- Trust center integration

### omnisystem-titanium-aps-diagnostics (400 LOC)
- Binding statistics
- Delivery success rates
- Latency tracking

---

## PHASE 17E: TITANIUM ZCL (Zigbee Cluster Library)
**Duration**: 1.5 weeks | **LOC**: 4,000 | **Crates**: 9 | **Tests**: 100

Each cluster crate (~400-500 LOC):
- Color Control (RGB, HSV, effects)
- Brightness Control (0-254 levels)
- Thermostat (heating, cooling, setpoints)
- Lock/Unlock
- Sensor readings (temperature, humidity, motion)
- Window Covering (position, tilt)
- Diagnostics (link quality, routing)

Plus **omnisystem-titanium-zcl-custom** for user-defined clusters

---

## PHASE 17F: TITANIUM DEVICE ROLES
**Duration**: 1 week | **LOC**: 2,500 | **Crates**: 4 | **Tests**: 70

### Coordinator (Network founder)
```rust
pub struct Coordinator {
    network: Network,
    children: Vec<u16>,
    security_processor: SecurityProcessor,
}

impl Coordinator {
    pub async fn form_network(&mut self, channel: u32) -> Result<(), String> {
        // Initialize PAN ID
        self.network.pan_id = rand::random();
        
        // Select best channel
        // Start advertising as coordinator
        // Accept joining devices
        Ok(())
    }

    pub async fn add_device(&mut self, device_addr: u64) -> Result<u16, String> {
        // Allocate short address
        let short_addr = self.allocate_short_address();
        
        // Generate network key
        // Send device configuration
        
        Ok(short_addr)
    }
}
```

### Router (Mesh node)
- Route forwarding
- Child device management
- Link status tracking

### End Device
- Parent selection
- Rejoining logic
- No routing responsibility

### Sleepy End Device
- Battery-optimized
- Wake-on-demand
- Parent maintains state

---

## PHASE 17G: TITANIUM SECURITY
**Duration**: 1 week | **LOC**: 3,000 | **Crates**: 6 | **Tests**: 85

### Advanced Key Management
```rust
pub struct KeyManager {
    network_key: [u8; 16],
    link_keys: HashMap<u64, [u8; 16]>,
    install_code_hashes: HashMap<u64, [u8; 16]>,
}

impl KeyManager {
    pub fn derive_link_key(install_code: &[u8]) -> [u8; 16] {
        // HMAC-SHA256(install_code, salt) → key
        // Prevents brute force attacks
    }

    pub fn rotate_network_key(&mut self) -> KeyUpdate {
        // Create new network key
        let new_key = rand::random();
        
        // Broadcast key update procedure
        // Devices acknowledge and switch
        
        KeyUpdate {
            old_key: self.network_key,
            new_key,
            sequence: 0,
        }
    }
}
```

### Encryption & Authentication
- AES-128 (standard Zigbee)
- AES-256 (enterprise)
- AES-GCM (authenticated encryption)
- Counter mode with CBC-MAC

### Trust Center Functions
- Device joining
- Key distribution
- Certificate management
- Trust establishment

---

## PHASE 18: AETHER Z-WAVE (Similar 8-week breakdown)

**Timeline**:
- 18A (1.5 weeks): Physical layer (900MHz, Turbo mode, FEC)
- 18B (1.5 weeks): MAC (20ms response, 500+ devices)
- 18C (2 weeks): Routing (multi-path, fast healing)
- 18D (1 week): Transport & sessions
- 18E (1.5 weeks): Command classes (12 crates)
- 18F (1.5 weeks): Device roles
- 18G (1.5 weeks): Security (S2 encryption + enhancements)

**Key Differences from Titanium**:
- 900MHz band (vs 2.4GHz)
- Turbo mode (256 kbps vs Zigbee's 250 kbps)
- Different routing (mesh tree vs AODV)
- Different command structure
- Different security model (S2 vs Zigbee)

---

## PHASE 19: INTEGRATION & TRANSFERDAEMON BRIDGE
**Duration**: 2 weeks | **LOC**: 5,500 | **Crates**: 9 | **Tests**: 160

### omnisystem-iot-multi-protocol (1,200 LOC)
```rust
pub struct MultiProtocolRouter {
    zigbee: TitaniumCoordinator,
    zwave: AetherController,
    thread: ThreadBorderRouter,
    ble: BleGateway,
}

impl MultiProtocolRouter {
    pub async fn route_command(&self, cmd: Command) -> Result<Response, String> {
        match cmd.protocol {
            Protocol::Zigbee => self.zigbee.send(cmd).await,
            Protocol::ZWave => self.zwave.send(cmd).await,
            Protocol::Thread => self.thread.send(cmd).await,
            Protocol::Ble => self.ble.send(cmd).await,
        }
    }

    /// Create scene with mixed-protocol devices
    pub async fn execute_scene(&self, scene: &Scene) -> Result<(), String> {
        for action in &scene.actions {
            // Route each device command to appropriate protocol
            self.route_command(action.command.clone()).await?;
        }
        Ok(())
    }

    /// Fallback routing if primary protocol fails
    pub async fn execute_with_fallback(&self, cmd: Command) -> Result<Response, String> {
        match self.route_command(cmd.clone()).await {
            Ok(resp) => Ok(resp),
            Err(_) => {
                // Try alternate protocol
                let fallback_cmd = self.translate_to_fallback(&cmd)?;
                self.route_command(fallback_cmd).await
            }
        }
    }
}
```

### omnisystem-iot-bridging (700 LOC)
- Protocol translation (Zigbee ↔ Z-Wave)
- Address mapping
- Capability mapping
- State synchronization

### omnisystem-iot-edge-compute (800 LOC)
```rust
pub struct EdgeCompute {
    rules_engine: RulesEngine,
    state_cache: StateCache,
    transferdaemon_link: TransferDaemonBridge,
}

impl EdgeCompute {
    pub async fn evaluate_rules(&self) -> Result<(), String> {
        // Evaluate automation rules locally
        for rule in self.rules_engine.rules() {
            if rule.condition.evaluate(&self.state_cache) {
                // Execute action locally (<10ms latency)
                rule.execute_locally().await?;
                
                // Sync to cloud asynchronously
                self.transferdaemon_link.sync_state().await;
            }
        }
        Ok(())
    }
}
```

### omnisystem-iot-sync (500 LOC)
- State synchronization
- Conflict resolution
- Version tracking
- Event replication

### omnisystem-iot-mesh-network (200 LOC)
- Device mesh topology
- Multi-path routing through TransferDaemon
- Health monitoring

### omnisystem-iot-api-gateway (400 LOC)
- REST API
- WebSocket for real-time
- gRPC for performance

### omnisystem-iot-fallback (600 LOC)
- Automatic failover between protocols
- Device supports both Zigbee and Z-Wave
- Seamless handoff

### omnisystem-iot-automation (100 LOC)
- Rule evaluation
- Scene execution
- Scheduling

---

## TESTING STRATEGY

### Phase 16 Tests (180)
- Device registry operations
- State management
- Device drivers
- Integration scenarios

### Phase 17 Tests (650)
- Radio transmission/reception
- Frame encoding/decoding
- Routing (1000+ simulated nodes)
- MAC protocol (collision handling)
- Security (key exchange)
- Multi-hop communication

### Phase 18 Tests (555)
- Similar breakdown for Z-Wave
- Protocol interoperability tests
- Multi-path routing validation
- Command execution

### Phase 19 Tests (160)
- Multi-protocol routing
- Scene execution (mixed devices)
- Fallback mechanisms
- EdgeCompute rule evaluation
- TransferDaemon bridge sync

---

## HARDWARE SIMULATION

```rust
#[cfg(test)]
mod network_simulation {
    use super::*;

    #[tokio::test]
    async fn test_1000_device_mesh() {
        // Simulate 1,000 devices
        let coordinator = TitaniumCoordinator::new();
        let mut devices = Vec::new();

        for i in 0..1000 {
            let device = SimulatedDevice::new(format!("device_{}", i));
            devices.push(device);
        }

        // Form network
        coordinator.form_network(15).await.unwrap();

        // Send broadcast message
        // Measure: latency, delivery rate, re-routing time
        
        // Drop some nodes
        devices.remove(500);

        // Verify self-healing
        // Measure: time to recover
    }

    #[tokio::test]
    async fn test_interference_mitigation() {
        let radio = TitaniumRadio::new(simulator);

        // Simulate interference on channel 15
        simulator.add_interference(Channel::Ch15, 100);

        // Radio should detect and switch channels
        radio.optimize().await.unwrap();

        assert_ne!(radio.current_channel(), Channel::Ch15);
    }
}
```

---

## PRODUCTION READINESS CHECKLIST

- [ ] All 1,545+ tests passing
- [ ] Memory profiling (< 50MB base system)
- [ ] Performance benchmarks (< 50ms latency verified)
- [ ] Security audit (third-party)
- [ ] Interoperability testing (real Zigbee/Z-Wave devices)
- [ ] Long-term stability testing (7-day uptime)
- [ ] Battery life validation (5+ years on AA)
- [ ] Range testing (300+ meters)
- [ ] Documentation complete
- [ ] Team training complete

---

## DEPLOYMENT CHECKLIST

- [ ] CI/CD pipeline configured
- [ ] Automated testing on each commit
- [ ] Binary releases prepared
- [ ] Update mechanism verified
- [ ] Rollback procedure tested
- [ ] Emergency revert plan ready

---

**Timeline**: 24 weeks from start to production

**Team**: 5-6 engineers

**Cost**: ~$20K hardware + tools

**Target Launch**: End of Q3 2026

