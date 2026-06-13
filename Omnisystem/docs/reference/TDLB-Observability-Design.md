# TDLB Observability & Metrics System Design

## Executive Summary

The **TransferDaemon Load Balancer (TDLB)** Observability & Metrics System transforms TDLB from a "black box" network component into a fully transparent, observable system. Every scheduling decision, connection state change, and device status update is captured as a first-class metric or Universe event, enabling operators to understand system behavior, debug issues, and ensure compliance.

This design provides complete visibility through real-time metrics collection, event logging, alerting, and visualization—all integrated with Bonsai's Universe event system and accessible from the Workspace UI.

---

## 1. Metrics Architecture

### 1.1 Collection Strategy

TDLB metrics are collected at multiple scales:

**Connection-level (per-chunk):**
- Collected at chunk send/receive time
- Minimal overhead: ~2-5 bytes per metric event
- Aggregated into per-connection summaries

**Device-level (per-heartbeat):**
- Sampled once per heartbeat (typically 1Hz = every 1 second)
- CPU, memory, bandwidth, connection count
- Battery, thermal, network interface status

**Transport-level (per-lane):**
- Collected when lane health changes or at 1/10Hz sampling
- QUIC congestion events, WebRTC ICE candidate events
- Tor node information, Bluetooth range/interference

**Global (every ~100ms):**
- Aggregate metrics: total active connections, pending requests
- Computed from per-connection and per-device metrics
- Used for dashboard refresh and alerting evaluation

### 1.2 Storage Architecture

#### Time-Series Database (TSDB)

A lightweight SQLite-based TSDB stores high-frequency metrics:

```rust
// Metrics are stored in a columnar format for efficient queries
// Schema (conceptual):
// metrics(
//   timestamp_ns: u64,           // nanosecond precision
//   metric_type: enum,            // "latency", "throughput", "cpu", ...
//   device_id: string,            // "device-abc123" or NULL for global
//   transport_kind: enum,         // "quic", "webrtc", "tor", "bluetooth"
//   connection_id: string,        // "conn-xyz789" or NULL for aggregates
//   value: f64,                   // metric value
//   percentile: u8,               // 50, 95, 99 for latency/throughput histograms
//   tags: json,                   // custom tags for filtering
// )

// Indexes:
// - (timestamp_ns, metric_type, device_id) → fast range queries
// - (device_id) → device-specific dashboards
// - (connection_id) → per-connection debugging
```

**Retention Policy:**
- **Raw metrics** (1-second resolution): Keep for 7 days
- **1-minute aggregates**: Keep for 30 days
- **1-hour aggregates**: Keep for 1 year
- **Daily summaries**: Keep indefinitely (compressed)

Automatic downsampling jobs run hourly:
```
0 * * * * → downsample raw metrics from [now-70m, now-10m] to 1-minute aggregates
0 0 * * * → downsample 1-minute from [31 days ago, now] to 1-hour aggregates
```

#### Universe Events as Audit Trail

Every significant event is also logged to Universe for full causality tracking:

```json
{
  "event_id": "blake3(timestamp:target:summary)",
  "timestamp_ns": 1685740800000000000,
  "category": "ComputeEvent",
  "source": {
    "component": "TDLB"
  },
  "summary": "Connection scheduled on QUIC/home→work",
  "target": "connection:conn-abc123",
  "metadata": {
    "tdlb_event_type": "scheduling_decision",
    "connection_id": "conn-abc123",
    "source_extension": "copilot",
    "chosen_transport": "quic",
    "lanes_available": ["quic/home→work", "webrtc/home→cloud"],
    "metrics_at_decision": {
      "quic/home→work": {
        "latency_p50_ms": 45.2,
        "latency_p99_ms": 120.5,
        "bandwidth_mbps": 45.3,
        "packet_loss_pct": 0.1,
        "in_flight_chunks": 12
      },
      "webrtc/home→cloud": {
        "latency_p50_ms": 85.1,
        "latency_p99_ms": 250.0,
        "bandwidth_mbps": 25.0,
        "packet_loss_pct": 2.5,
        "in_flight_chunks": 5
      }
    },
    "decision_reason": "quic has lowest estimated completion time",
    "eta_secs": 0.95
  }
}
```

---

## 2. Universe Event Schema for TDLB

### 2.1 Event Categories

All TDLB events use `category: ComputeEvent` with structured `metadata` to distinguish types:

#### Connection Lifecycle Events

```yaml
EventType: connection.created
Summary: "Transfer started from Copilot to home device"
Target: "connection:conn-abc123"
Metadata:
  tdlb_event_type: connection_created
  connection_id: string
  source_extension: "copilot" | "claude_code" | ...
  source_peer: string (peer ID or "local")
  destination_peer: string
  expected_bytes: integer
  is_critical: boolean
  created_at_ns: u64

---

EventType: connection.migrated
Summary: "Connection migrated from QUIC/home→work to WebRTC/home→cloud"
Target: "connection:conn-abc123"
Metadata:
  tdlb_event_type: connection_migrated
  connection_id: string
  old_transport: string
  new_transport: string
  reason: "device_offline" | "better_latency" | "congestion" | "link_failure"
  migration_delay_ms: float
  migrated_at_ns: u64
  parent_event: "device_offline:device-xyz" (causal link)

---

EventType: connection.completed
Summary: "Transfer completed: 4.5 MB in 2.3s via QUIC"
Target: "connection:conn-abc123"
Metadata:
  tdlb_event_type: connection_completed
  connection_id: string
  total_bytes: integer
  duration_ms: float
  transports_used: [string]
  final_transport: string
  chunks_sent: integer
  chunks_retransmitted: integer
  peak_throughput_mbps: float
  avg_latency_ms: float
  completed_at_ns: u64

---

EventType: connection.failed
Summary: "Transfer failed: timeout after 30s, 2.1 MB transferred"
Target: "connection:conn-abc123"
Metadata:
  tdlb_event_type: connection_failed
  connection_id: string
  reason: "all_transports_failed" | "timeout" | "retransmit_exhausted"
  bytes_transferred: integer
  bytes_lost: integer
  last_transport: string
  error_detail: string
  failed_at_ns: u64
  parent_events: ["device_offline:device-xyz", "network_unavailable:network-abc"]
```

#### Scheduling Decision Events

```yaml
EventType: scheduling.decision
Summary: "Chunk 123 scheduled on QUIC (ETA: 0.95s)"
Target: "scheduling:conn-abc123:chunk-123"
Metadata:
  tdlb_event_type: scheduling_decision
  connection_id: string
  chunk_gsn: integer
  chunk_size: integer
  is_critical: boolean
  chosen_primary: string (lane name: "quic/home→work")
  chosen_mirror: string | null
  estimated_eta_secs: float
  lanes_considered:
    - name: string
      available: boolean
      eta_secs: float | null
      latency_p99_ms: float
      bandwidth_mbps: float
      in_flight_chunks: integer
      reorder_gap: integer
      rejection_reason: string | null
  decision_algorithm: "ecf_rg"
  decision_at_ns: u64
```

#### Device Status Events

```yaml
EventType: device.online
Summary: "Device home-ubuntu came online"
Target: "device:device-abc123"
Metadata:
  tdlb_event_type: device_online
  device_id: string
  device_name: string
  last_offline_at_ns: u64 | null
  offline_duration_ms: integer | null
  online_at_ns: u64

---

EventType: device.offline
Summary: "Device home-ubuntu went offline (no heartbeat for 30s)"
Target: "device:device-abc123"
Metadata:
  tdlb_event_type: device_offline
  device_id: string
  device_name: string
  reason: "heartbeat_timeout" | "explicit_disconnect" | "network_lost"
  offline_at_ns: u64
  expected_reconnection_secs: integer | null
  affected_connections: [string] (connection IDs affected)

---

EventType: device.overloaded
Summary: "Device home-ubuntu is overloaded (CPU 95%, mem 88%)"
Target: "device:device-abc123"
Metadata:
  tdlb_event_type: device_overloaded
  device_id: string
  device_name: string
  cpu_utilization_pct: float
  memory_utilization_pct: float
  active_connection_count: integer
  max_connection_capacity: integer
  bottleneck: "cpu" | "memory" | "bandwidth"
  overloaded_at_ns: u64
```

#### Transport Lane Events

```yaml
EventType: lane.degraded
Summary: "QUIC/home→work latency spike (p99: 800ms, was 120ms)"
Target: "lane:quic/home→work"
Metadata:
  tdlb_event_type: lane_degraded
  lane_name: string
  transport_kind: string
  severity: "warning" | "critical"
  old_metrics:
    latency_p99_ms: float
    bandwidth_mbps: float
    packet_loss_pct: float
  new_metrics:
    latency_p99_ms: float
    bandwidth_mbps: float
    packet_loss_pct: float
  degraded_at_ns: u64
```

#### Policy Decision Events

```yaml
EventType: policy.matched
Summary: "Policy 'prefer_local' matched for connection"
Target: "connection:conn-abc123"
Metadata:
  tdlb_event_type: policy_matched
  connection_id: string
  policy_id: string
  policy_name: string
  criteria_matched: [string]
  recommended_lanes: [string]
  priority: integer
  matched_at_ns: u64
```

### 2.2 Event Query Language

Universe events can be queried using a simple filter DSL:

```sql
-- Find all connections that failed in the last hour
SELECT * FROM universe_events
WHERE category = 'ComputeEvent'
  AND metadata.tdlb_event_type = 'connection_failed'
  AND timestamp_ns > (now_ns - 3600e9)

-- Find the causal chain of a failed connection
WITH RECURSIVE causal_chain AS (
  SELECT event_id, parent_event_ids, metadata
  FROM universe_events
  WHERE target = 'connection:conn-abc123'
  UNION ALL
  SELECT e.event_id, e.parent_event_ids, e.metadata
  FROM universe_events e
  INNER JOIN causal_chain c ON e.event_id = c.parent_event_ids[0]
)
SELECT * FROM causal_chain

-- Find all connections affected by device going offline
SELECT DISTINCT metadata->>'connection_id' as conn_id
FROM universe_events
WHERE metadata->>'tdlb_event_type' = 'device_offline'
  AND metadata->>'device_id' = 'device-abc123'
  AND timestamp_ns > ?
UNION
SELECT DISTINCT parent_event.metadata->>'connection_id'
FROM universe_events
WHERE metadata->>'device_id' = 'device-abc123'
  AND metadata->>'tdlb_event_type' = 'device_offline'
```

---

## 3. Real-time Dashboard Design

### 3.1 Dashboard Components

The TDLB dashboard is a Bonsai Workspace panel built with Svelte/SvelteKit. It updates every 100-500ms.

#### 3.1.1 System Health Overview (Top Section)

```
┌─────────────────────────────────────────────────────────────┐
│  TDLB Status: HEALTHY ✓  |  Uptime: 47d 3h 22m  |  Alerts: 1 │
│                                                               │
│  Connections: 247 active  │  Throughput: 850 Mbps  │  P99 Latency: 87ms
│  In-flight: 1,250 chunks  │  Device Count: 18      │  Failover Rate: 0.2%
│  Queue Depth: 45 pending  │  Transport Mix: Q:60% WR:35% T:5%
└─────────────────────────────────────────────────────────────┘
```

**Real-time Updates:**
- Connection count: from `COUNT(*) WHERE status='active'` in metrics TSDB
- Throughput: from `SUM(bytes) / interval` over last 5 seconds
- P99 latency: from latency histogram, 99th percentile
- Device count: from device registry
- Failover rate: `COUNT(migrations) / COUNT(completed) × 100` over last 5 minutes

#### 3.1.2 Connection Timeline (Historical Replay)

```
Timeline: ──────────────────────────────────────────────────────→ now
                                                          ▼
                                  Event Log (scroll to see details)
12:34:56.123  ▼  conn-abc123 scheduled on QUIC/home→work (ETA: 0.95s)
12:34:57.012  ✓  QUIC ACK: chunks 1-50
12:34:57.456  ✗  device-xyz offline (heartbeat timeout)
12:34:57.501  ↻  conn-abc123 migrated to WebRTC/home→cloud
12:34:59.234  ✓  conn-abc123 completed (4.5 MB, 3.1s)
```

**Interaction:**
- Click on a time range to zoom in
- Click on an event to show full JSON
- Click "Replay" to visually replay the timeline (shows connection state transitions)

#### 3.1.3 Device Status Grid

```
┌────────────────────────────────────────────────────────────────┐
│ Devices                                                          │
├────────────────────────────────────────────────────────────────┤
│ Device           Status   CPU    Mem    Conns   Throughput  v  │
│ home-ubuntu      🟢 UP    45%    62%    45      120 Mbps       │
│ home-rpi         🟢 UP    78%    55%    12       15 Mbps       │
│ work-laptop      🟠 SLOW  92%    88%    0        0 Mbps        │
│ cloud-server-1   🟢 UP    23%    45%    140      450 Mbps      │
│ cloud-server-2   🟡 WARN  88%    91%    102      380 Mbps      │
│ phone            🔴 DOWN  -      -      0        -             │
└────────────────────────────────────────────────────────────────┘
```

**Interaction:**
- Click on a device to see detailed metrics (CPU/mem/network over time)
- Hover over status icon to see reason for state
- Click "Details" to see active connections and recent events

#### 3.1.4 Transport Lane Health (per Lane)

```
┌──────────────────────────────────────────────────────────────┐
│ Transport Lanes                                                │
├──────────────────────────────────────────────────────────────┤
│ Lane                  Health  Latency  BW      Pkts   Conns  │
│ QUIC/home→work       🟢      45ms     45 Mbps 0.1%   60      │
│ QUIC/home→cloud      🟢      95ms     35 Mbps 0.3%   20      │
│ WebRTC/home→cloud    🟠      250ms    25 Mbps 2.5%   35      │
│ Tor/exit→anywhere    🟡      800ms    5 Mbps  5.2%   8       │
│ Bluetooth/home→phone 🔴      -        -       -      -       │
└──────────────────────────────────────────────────────────────┘
```

#### 3.1.5 Scheduling Decision Heatmap

Shows which lanes were chosen for chunks over time:

```
Time  │ QUIC/H→W │ QUIC/H→C │ WebRTC/H→C │ Tor │ BT
12:34 │ ████████ │ ███      │ ██         │ █   │
12:35 │ ███████  │ ███      │ ████       │ -   │
12:36 │ ██████   │ ████     │ ███        │ █   │
12:37 │ █████    │ ███      │ █████      │ █   │ █
      └──────────┴──────────┴────────────┴─────┴─
```

Width of bar = number of chunks scheduled on that lane in that minute.
Color intensity = relative preference (darker = more preferred by scheduler).

#### 3.1.6 Connection Quality Heatmap (P99 Latency by Hour)

```
       Mon   Tue   Wed   Thu   Fri   Sat   Sun
00:00  🟢    🟢    🟢    🟢    🟡    🟠    🟢
01:00  🟢    🟢    🟢    🟢    🟢    🟠    🟢
...
20:00  🟠    🟠    🟡    🟠    🟠    🟢    🟡
21:00  🟡    🟡    🟠    🟡    🟠    🟢    🟢
22:00  🟢    🟢    🟢    🟢    🟡    🟢    🟢
```

🟢 = P99 < 100ms, 🟡 = 100-500ms, 🟠 = 500-1s, 🔴 = > 1s

#### 3.1.7 Alerts Panel

```
┌─────────────────────────────────────────────────────────────┐
│ ALERTS                                                        │
├─────────────────────────────────────────────────────────────┤
│ ⚠ HIGH LATENCY (WARNING)  [12:37:01]  [Dismiss] [Investigate]│
│   P99 latency > 1s on QUIC/home→work for 5m                  │
│   Suggested action: Reduce load or check network               │
│                                                               │
│ ℹ DEVICE OVERLOADED (INFO) [12:31:22]                         │
│   home-ubuntu CPU at 95%, consider migrating connections     │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Advanced View: Connection Trace

Click on a connection ID to see the full trace:

```
Connection: conn-abc123 (Source: Copilot→Home)
Duration: 3.1s | Bytes: 4.5 MB | Status: ✓ completed

Timeline:
[12:34:56.100] Created
              [Considered lanes: QUIC/home→work, WebRTC/home→cloud, Tor]
              [Metrics: QUIC latency=45ms bw=45Mbps, WebRTC latency=85ms bw=25Mbps]
              [Decision: QUIC (ETA 0.95s)]

[12:34:56.120] Chunk 0-1000 bytes → QUIC
[12:34:56.234] Chunk 1000-2000 → QUIC
[12:34:56.456] Chunk 2000-3000 → QUIC [reordered, received as #5]
[12:34:57.012] ACKs: chunks 0-50
              [Update metrics: latency=52ms, bw=44Mbps]

[12:34:57.456] Device home-rpi went offline
              [Causal event: device_offline:device-xyz123]
              [Trigger migration evaluation]
              [Current QUIC metrics: latency=60ms, bw=42Mbps]
              [Available lanes: WebRTC/home→cloud only]

[12:34:57.501] Migrated to WebRTC/home→cloud
              [Reason: better_latency (WebRTC: 85ms → 75ms after device offline)]
              [Resend buffered chunks 51-150]

[12:34:58.100] WebRTC ICE complete, DTLS handshake: 120ms
[12:34:58.234] Chunk 150-151000 → WebRTC
[12:34:59.234] All chunks ACKed, connection completed
```

### 3.3 Performance Characteristics

**Dashboard update frequency:**
- System health bar: 100ms (from memory metrics)
- Connection timeline: 500ms (append-only log)
- Device grid: 1s (sample device metrics)
- Lane health: 1s (from lane health snapshots)
- Alerts: 500ms (trigger evaluation every 500ms)

**Data fetching:**
- Pre-aggregate data at collection time (no expensive queries at view time)
- Use caching: device metrics cached in memory, device-specific metrics cached for 5s
- Lazy load detailed traces (only fetch when user clicks)

---

## 4. Alerting System

### 4.1 Alert Rules

Alerts are defined as conditions on time-series metrics or event streams:

```yaml
# Alert: High Latency
alert: HighLatency
  condition: p99_latency > 1000ms for 5m
  severity: WARNING
  channels: [workspace_ui, slack]
  action: "Load is high, consider adding capacity or reducing request rates"
  deduplication_key: "lane:<lane_name>"
  dedup_window_secs: 300
  escalation:
    after_5m: notify_supervisor
    after_15m: page_on_call

# Alert: Device Offline
alert: DeviceOffline
  condition: heartbeat_missing for 30s
  severity: CRITICAL
  channels: [workspace_ui, email]
  action: "Migrate active connections, notify device owner"
  deduplication_key: "device:<device_id>"
  dedup_window_secs: 60
  escalation:
    immediate: notify_user
    after_2m: notify_supervisor

# Alert: Failover Rate High
alert: HighFailoverRate
  condition: (count(migrated) / count(completed)) > 5% for 10m
  severity: WARNING
  channels: [workspace_ui]
  action: "Network conditions are unstable, check ISP status"
  dedup_window_secs: 600

# Alert: All Lanes Unavailable
alert: AllLanesDown
  condition: count(available_lanes) == 0 for 10s
  severity: CRITICAL
  channels: [workspace_ui, email, sms]
  action: "TDLB cannot route traffic, check network connectivity"
  escalation:
    immediate: page_on_call

# Alert: SLO Violation
alert: SloViolation
  condition: p99_latency > slo_threshold for 5m
  severity: ERROR
  channels: [workspace_ui, slack]
  action: "SLO violation on <lane>, investigate performance"
  dedup_window_secs: 300

# Alert: Device Overloaded
alert: DeviceOverloaded
  condition: (cpu_utilization > 90% AND memory_utilization > 85%) for 2m
  severity: WARNING
  channels: [workspace_ui]
  action: "Device approaching capacity, consider load reduction"
  dedup_window_secs: 300

# Alert: Suspicious Traffic Pattern
alert: SuspiciousTraffic
  condition: throughput > 10x 99th percentile for 30s
  severity: INFO
  channels: [workspace_ui]
  action: "Unusual traffic pattern detected, verify it's legitimate"
  dedup_window_secs: 300
```

### 4.2 Alert Deduplication

Alerts are grouped by `deduplication_key` within a `dedup_window_secs`:

```rust
// Pseudocode
fn should_alert(alert_rule: AlertRule, current_state: MetricSnapshot) -> bool {
    if !evaluate_condition(&alert_rule.condition, &current_state) {
        return false;
    }
    
    let dedup_key = format_dedup_key(&alert_rule.deduplication_key, &current_state);
    let last_alert = alert_history.get(&dedup_key);
    
    // Fire alert only if last alert was outside dedup_window_secs
    if let Some(ts) = last_alert {
        if now - ts < alert_rule.dedup_window_secs {
            return false;  // Suppress duplicate
        }
    }
    
    alert_history.insert(dedup_key, now);
    true
}
```

### 4.3 Escalation

If an alert is not acknowledged, escalate:

```rust
enum AlertStatus {
    Active,          // Alert firing, not acknowledged
    Acknowledged,    // User marked as aware
    Resolved,        // Condition no longer met
}

fn escalate_alert(alert: &mut Alert) {
    if alert.status == AlertStatus::Active {
        if now - alert.fired_at > alert.escalation.after_5m {
            send_notification(alert, "supervisor");
        }
        if now - alert.fired_at > alert.escalation.after_15m {
            page_on_call(alert);
        }
    }
}
```

### 4.4 Alert Channels

**Workspace UI:**
- Alerts appear in dashboard panel
- User can acknowledge, snooze (5 min), or dismiss

**Slack/Email:**
- Formatted message with alert context
- Links to dashboard for investigation
- Acknowledge button (updates Workspace UI)

**SMS/Push Notification:**
- Only for CRITICAL alerts
- Includes short action description

### 4.5 Alert Context

When an alert fires, include full context:

```json
{
  "alert_rule": "HighLatency",
  "severity": "WARNING",
  "fired_at_ns": 1685740800000000000,
  "condition": "p99_latency > 1000ms for 5m",
  "affected_metric": {
    "lane": "quic/home→work",
    "p99_latency_ms": 1240.5,
    "p95_latency_ms": 780.2,
    "p50_latency_ms": 240.1
  },
  "context": {
    "active_connections": 45,
    "in_flight_chunks": 890,
    "device_cpu_pct": 78.5,
    "device_mem_pct": 82.3,
    "network_congestion": "high"
  },
  "suggested_actions": [
    "Check if a large transfer is running",
    "Verify ISP connection quality",
    "Consider load rebalancing to alternate transports"
  ],
  "investigation_links": [
    "/dashboard/tdlb?lane=quic/home→work&time_range=last_5m",
    "/universe/events?target=lane:quic/home→work&limit=100"
  ]
}
```

---

## 5. Historical Analysis & Reporting

### 5.1 Query API

Universe events are queryable via a REST API:

```
GET /api/universe/events
  ?category=ComputeEvent
  &metadata.tdlb_event_type=connection_failed
  &timestamp_after_ns=<epoch_ns>
  &timestamp_before_ns=<epoch_ns>
  &limit=100
  &order=desc

GET /api/universe/events/:event_id
  → Returns full event with hashes, signatures, parent/child links

POST /api/universe/query (advanced queries)
  {
    "filter": {
      "category": "ComputeEvent",
      "metadata.tdlb_event_type": {"$in": ["connection_failed", "connection_completed"]},
      "timestamp_ns": {"$gte": <epoch_ns>}
    },
    "limit": 1000,
    "order_by": "-timestamp_ns"
  }
```

### 5.2 Example Queries

**Query 1: Find all connections that used Tor in the last 24 hours**

```sql
SELECT event_id, timestamp_ns, metadata
FROM universe_events
WHERE category = 'ComputeEvent'
  AND metadata->>'tdlb_event_type' = 'scheduling_decision'
  AND metadata->>'chosen_primary' LIKE '%tor%'
  AND timestamp_ns > (now_ns - 86400e9)
ORDER BY timestamp_ns DESC
```

Returns: 47 connections used Tor in the last 24h, across 3 exit nodes, averaging 15s completion time.

**Query 2: Show the causal chain of a failed connection**

```sql
WITH RECURSIVE causal_chain(event_id, parent_event_ids, depth) AS (
  SELECT event_id, parent_event_ids, 0
  FROM universe_events
  WHERE target = 'connection:conn-abc123' AND metadata->>'tdlb_event_type' = 'connection_failed'
  UNION ALL
  SELECT e.event_id, e.parent_event_ids, cc.depth + 1
  FROM universe_events e
  INNER JOIN causal_chain cc ON e.event_id = cc.parent_event_ids[0]
  WHERE cc.depth < 10
)
SELECT DISTINCT event_id, metadata FROM causal_chain
ORDER BY depth DESC
```

Returns: failure was caused by device_offline, which was caused by network_lost.

**Query 3: Which device had the most uptime last month?**

```sql
SELECT
  metadata->>'device_id' as device_id,
  SUM(CASE WHEN metadata->>'tdlb_event_type' = 'device_online' THEN 1 ELSE 0 END) as online_count,
  SUM(CASE WHEN metadata->>'tdlb_event_type' = 'device_offline' THEN 1 ELSE 0 END) as offline_count,
  -- Calculate uptime: initial_time + (online_duration - offline_duration)
  ...
FROM universe_events
WHERE category = 'ComputeEvent'
  AND metadata->>'tdlb_event_type' IN ('device_online', 'device_offline')
  AND timestamp_ns > (now_ns - 30*86400e9)
GROUP BY metadata->>'device_id'
ORDER BY uptime_pct DESC
```

**Query 4: Peak latency by hour of day (for pattern analysis)**

```sql
SELECT
  HOUR(datetime(timestamp_ns / 1e9)) as hour_of_day,
  PERCENTILE_CONT(0.99) WITHIN GROUP (ORDER BY latency_p99_ms) as p99_latency_ms,
  COUNT(*) as samples
FROM metrics
WHERE metric_type = 'latency' AND timestamp_ns > (now_ns - 7*86400e9)
GROUP BY hour_of_day
ORDER BY p99_latency_ms DESC
```

### 5.3 Automated Reports

**Weekly Report** (sent every Monday 9am):

```
TDLB Weekly Summary - Week of 2026-05-31

Executive Summary:
- Uptime: 99.98%
- Total connections: 8,247
- Total data transferred: 142 GB
- Failover events: 12 (0.15%)
- SLO violations: 0

Connection Trends:
- 15% increase in Copilot connections (6,234 vs 5,412 last week)
- Average completion time: 2.3s (was 2.1s)
- Fastest connection: 0.1s, Slowest: 45.6s
- Peak simultaneous connections: 287 (Tuesday 14:32)

Device Utilization:
- Most active device: cloud-server-1 (2,345 connections)
- Least active device: work-laptop (12 connections)
- Average per-device utilization: 68% CPU, 72% memory
- Device offline events: 3 (home-rpi x2, phone x1)

Transport Performance:
- QUIC: 4,800 connections, avg latency 45ms, zero failures
- WebRTC: 2,140 connections, avg latency 95ms, 2 failures
- Tor: 287 connections, avg latency 800ms, 10 failures
- Bluetooth: 20 connections, avg latency 120ms, 1 failure

Cost Analysis (if using paid transports):
- Tor bandwidth: 45 GB, $4.50 cost
- Cloud egress: 85 GB, $7.65 cost
- Total cost: $12.15

Incidents:
- 2026-05-31 12:34 - High latency spike on QUIC/home→work (duration: 3min, resolved)
- No unresolved incidents

Recommendations:
- Latency is trending upward, monitor network conditions
- Consider adding a second local device to distribute load
```

**Monthly Report** (sent first day of month):

```
TDLB Monthly Summary - May 2026

Uptime: 99.97% (28 minutes downtime)
Total connections: 247,654
Total data transferred: 4.2 TB
Revenue (if applicable): $1,245.60 from credits

Top SLO Violations:
1. High latency (5 incidents, avg duration 2.3min)
2. Device offline (3 incidents, avg recovery 1.8min)
3. Failover rate > 5% (1 incident, 1.2min)

Most Reliable Device: cloud-server-1 (99.99% uptime)
Least Reliable Device: phone (95.2% uptime - outdoor usage patterns)

Transport Evolution:
- QUIC usage up 20% (was 55%, now 65%)
- WebRTC usage down 10% (was 30%, now 20%)
- Tor stable (5% → 5%)
- Bluetooth stable (5% → 5%)

Geographic Patterns (if using geo-aware scheduling):
- Peak usage times: 14:00-16:00 and 20:00-22:00
- Geographic anomaly: unexpectedly high Tor usage from 2026-05-25 (event?)

Cost Breakdown (if applicable):
- Bandwidth: $156
- Compute: $89.45
- Storage: $12.30
- Total: $257.75

Compliance:
- 100% of connections stayed in-country ✓
- Zero encryption violations ✓
- All policy requirements met ✓
```

### 5.4 Query Performance

Target: <500ms for typical queries over 1 million events

**Optimization strategies:**

1. **Index by metadata fields:** Create SQLite partial indexes on frequently-queried fields:
   ```sql
   CREATE INDEX idx_tdlb_event_type ON universe_events(
     json_extract(metadata, '$.tdlb_event_type')
   ) WHERE category = 'ComputeEvent';
   
   CREATE INDEX idx_device_id ON universe_events(
     json_extract(metadata, '$.device_id')
   ) WHERE category = 'ComputeEvent';
   ```

2. **Partition by time:** Split events into monthly tables, archive old data
   ```sql
   universe_events_2026_05
   universe_events_2026_04
   universe_events_archive_2026_q1
   ```

3. **Pre-aggregate common queries:** Store daily summaries
   ```sql
   SELECT
     DATE(datetime(timestamp_ns / 1e9)) as date,
     metadata->>'tdlb_event_type' as event_type,
     COUNT(*) as count,
     ... other aggregates
   FROM universe_events
   GROUP BY date, event_type
   ```

---

## 6. Compliance & Audit Reporting

### 6.1 Audit Trail

Every significant decision is logged and cryptographically signed:

```json
{
  "audit_event": {
    "event_id": "blake3(...)",
    "timestamp_ns": 1685740800000000000,
    "action": "scheduling_decision",
    "actor": "TDLB scheduler",
    "target": "connection:conn-abc123",
    "decision": {
      "chose_transport": "quic",
      "alternatives": ["webrtc", "tor"],
      "reason": "lowest_eta"
    },
    "state_before": { ... },
    "state_after": { ... },
    "signature": "ed25519(...)",
    "signed_by": "tdlb-daemon:device-abc123"
  }
}
```

### 6.2 Compliance Report Template

```yaml
Compliance Report: May 2026

Regulatory Requirements:
- GDPR Data Residency: All user data stayed in EU ✓
- CCPA Opt-out: 3 users opted out, zero tracking ✓
- HIPAA Encryption: All connections encrypted, zero plaintext ✓
- SOC2 Audit Trail: All events logged with signatures ✓

Certification:
- Report generated: 2026-06-01T00:00:00Z
- Signature: ed25519(blake3(report_content))
- Signed by: compliance-officer@example.com
- Key ID: compliance-key-2026-05
- Valid until: 2026-06-01T23:59:59Z

Detailed Findings:
1. Data Residency Compliance
   - Connections routing through Tor: 287 (0.3%)
   - All verified to stay in-country via exit node logs ✓
   - Zero policy violations

2. Encryption Compliance
   - TLS/DTLS on all public transports: 100% ✓
   - Tor encryption: verified for 287 connections ✓
   - Bluetooth: weak encryption, approved for non-sensitive data ✓

3. Access Control
   - Total API calls: 2,456,789
   - Authenticated: 100% ✓
   - Rate limited: 45 clients throttled ✓
   - Denied: 12 unauthorized access attempts ✓

Export Formats:
- PDF (human-readable)
- JSON (machine-readable)
- CSV (for spreadsheet import)
```

---

## 7. Integration with Bonsai Components

### 7.1 Universe Integration

TDLB writes events to Universe using the standard `UniverseEvent` struct:

```rust
// In TDLB scheduler when making a scheduling decision
let event = UniverseEvent::new(
    EventSource::System { component: "TDLB".into() },
    EventCategory::ComputeEvent,
    "Chunk scheduled on QUIC",
    format!("connection:{}", conn_id),
    device_id,
)
.with_metadata(serde_json::json!({
    "tdlb_event_type": "scheduling_decision",
    "connection_id": conn_id,
    "chunk_gsn": gsn,
    "chosen_primary": lane_name,
    "estimated_eta_secs": eta,
    // ... more context
}))
.with_parents(vec![prev_decision_event_id]);

universe.emit(event).await?;
```

**Universe stores these as:**
- In SQLite (primary storage)
- Synced to Bonsai Cloud (for compliance archival)
- Queryable from Workspace UI

### 7.2 Compute Fabric Integration

TDLB metrics feed into Compute Fabric's load management:

```rust
// In Compute Fabric when scheduling a new task
let tdlb_metrics = fabric.query_tdlb_metrics(
    QueryTdlbMetrics {
        devices: active_devices,
        time_window_secs: 60,
        percentile: 99,
    }
).await?;

// Use latency metrics to predict task completion time
let task_eta = base_time + tdlb_metrics.p99_latency_ms / 1000.0;

// Use failover rate to assess risk
if tdlb_metrics.failover_rate_pct > 5.0 {
    warn!("High failover rate, task may not complete in SLA");
}
```

### 7.3 Survival System Integration

The Survival System uses TDLB health metrics to decide whether to restart:

```rust
// In Survival System health check
fn should_restart_tdlb() -> bool {
    let health = get_tdlb_health();
    
    // Restart if TDLB is unhealthy for > 2 minutes
    health.is_critical && health.duration_critical_secs > 120
        || health.cpu_utilization > 95.0 && health.duration_secs > 60
        || health.error_rate > 10.0
}

fn get_tdlb_health() -> TdlbHealth {
    // Query recent metrics from TSDB
    let recent_metrics = metrics_db.query_recent(Duration::from_secs(300));
    
    TdlbHealth {
        is_critical: recent_metrics.connection_success_rate < 95.0,
        cpu_utilization: recent_metrics.cpu_avg,
        error_rate: recent_metrics.error_count / recent_metrics.total_count * 100.0,
        duration_critical_secs: ...,
        duration_secs: ...,
    }
}
```

### 7.4 Credits System Integration

TDLB tracks cost data for each connection:

```rust
// When completing a connection
let cost = Cost {
    bytes_transferred: total_bytes,
    duration_secs: duration,
    primary_transport: "quic",
    bandwidth_cost_credit: 0.5,  // From Tor or cloud
    compute_cost_credit: 0.1,    // TDLB CPU
};

// Emit to Credits System
credits_system.record_usage(
    source_peer,
    destination_peer,
    cost,
).await?;

// Also log to Universe for audit
let event = UniverseEvent::new(
    EventSource::System { component: "TDLB".into() },
    EventCategory::CreditTransaction,
    format!("Connection completed: {:.2}$ cost", cost.total()),
    format!("connection:{}", conn_id),
    device_id,
);
```

---

## 8. Visualization Components

### 8.1 Real-time Line Graph

```
Latency Over Time (Last 60 minutes)
┌─────────────────────────────────────────────────────┐
│                                                     │
│ 1000ms │                                            │
│   900ms │  ╱╲                                        │
│   800ms │ ╱  ╲                        ╱╲             │
│   700ms │╱    ╲                      ╱  ╲            │
│   600ms │      ╲                    ╱    ╲           │
│   500ms │       ╲__                ╱      ╲__        │
│   400ms │          ╲_             ╱           ╲_     │
│   300ms │            ╲_          ╱              ╲    │
│   200ms │_______________╲_______╱________________│    │
│   100ms │                                        │    │
│     0ms │────────────────────────────────────────│    │
│         └─────────────────────────────────────────┘   │
│         00:00     12:00     00:00     12:00     00:00 │
│         — QUIC (5 min avg)  — WebRTC  — Tor          │
└─────────────────────────────────────────────────────┘
```

**Interactions:**
- Zoom: scroll to change time range
- Pan: drag left/right to shift time window
- Hover: show exact value and timestamp
- Click legend: toggle lanes on/off
- Draw box: select time range for detailed view

### 8.2 Device Utilization Heatmap

```
Device Utilization (Last 7 days)
┌─────────────────────────────────────────────────────┐
│                                                     │
│ home-ubuntu    ░░░░░░░░░░░░░░▓▓▓▓▓▓▓▓▓▓░░░░░░░░░│
│ home-rpi       ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│
│ work-laptop    ░░░░░░░▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░│
│ cloud-server-1 ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│
│ cloud-server-2 ░░░░░░░░░░░░▓▓▓▓▓▓▓▓▓▓░░░░░░░░░│
│ phone          ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│
│                                                     │
│ ░ < 50% CPU   ▒ 50-75% CPU   ▓ 75-90% CPU   █ > 90%
└─────────────────────────────────────────────────────┘
```

### 8.3 Topology Graph

```
Topology Visualization (Real-time)

                    ┌─────────────────┐
                    │  cloud-server-1 │
                    │  CPU: 45%       │
                    │  Conns: 140     │
                    └────────┬────────┘
                             │ QUIC (450 Mbps)
                    ┌────────┴────────┐
        ┌───────────┤                 ├──────────┐
        │           │                 │          │
    ┌───▼────┐  ┌───▼───┐  ┌─────┐  ┌──▼──┐  ┌──▼────┐
    │ home-  │  │ home-  │  │Tor  │  │work-│  │cloud- │
    │ubuntu  │  │  rpi   │  │exit │  │laptop  │server │
    │CPU 45% │  │CPU 78% │  │node │  │     │  │CPU 23%│
    │4 conns │  │12 cns. │  │8 cn │  │0 cn │  │102 cn │
    └────────┘  └────────┘  └─────┘  └─────┘  └───────┘
        │            │         │         │        │
        └────────────┴────┬────┴─────────┴────────┘
                          │
                    ┌─────▼──────┐
                    │   Phone    │
                    │  (offline) │
                    └────────────┘

Legend:
  — Strong (latency < 100ms)
  - - Degraded (latency 100-500ms)
  ··· Poor (latency > 500ms)
  ×××  Failed
```

### 8.4 Event Timeline with Replay

```
Replay Timeline: Connection conn-abc123

Timeline Position: ────●─────────────────────────── now
                     ↑ (click to jump)

Event Log:
[12:34:56.100] ▶ Created
              [Show connection state at this time]
[12:34:56.500] ▶ Chunk 0-50 scheduled on QUIC
[12:34:56.750] ▶ Chunk 51-100 scheduled on QUIC
[12:34:57.456] ▶ Device home-rpi went offline [pause playback]
[12:34:57.501] ▶ Connection migrated to WebRTC
[12:34:59.234] ▶ Completed

Play Speed: 1x [dropdown: 0.1x, 0.5x, 1x, 2x, 5x]
[Play] [Pause] [Reset]

Visual Replay (shown on connected device):
┌────────────────────────────┐
│ Connection State at 12:34:57.001 │
│                            │
│ Primary: QUIC/home→work    │
│ Status: Active             │
│ Bytes sent: 1.2 MB         │
│ P99 Latency: 45ms          │
│ Throughput: 43 Mbps        │
│                            │
│ Available Lanes:           │
│ ✓ QUIC/home→work           │
│ ✓ WebRTC/home→cloud        │
│ ✗ Tor (too slow)           │
└────────────────────────────┘

[At 12:34:57.456, fade QUIC to red and show "offline"]

[At 12:34:57.501, migrate to WebRTC, show re-routing]

[At 12:34:59.234, show ✓ Completed]
```

### 8.5 Detail Table with Filtering

```
Recent Connections

┌───────────────────────────────────────────────────────────┐
│ ID       │ Source    │ Duration │ Bytes  │ Status │ Trans │
├───────────────────────────────────────────────────────────┤
│ conn-123 │ Copilot   │ 3.1s     │ 4.5 MB │ ✓      │ QUIC  │
│ conn-124 │ CC        │ 5.2s     │ 12 MB  │ ✓      │ QR    │
│ conn-125 │ Copilot   │ (active) │ 2.3 MB │ ⟳      │ QUIC  │
│ conn-126 │ CC        │ 2.8s     │ 1.2 MB │ ✗      │ TOR   │
│ conn-127 │ Copilot   │ 8.5s     │ 25 MB  │ ✓      │ Q+WR  │
└───────────────────────────────────────────────────────────┘

Filters: [Source: Copilot ▼] [Status: All ▼] [Duration: > 1s ▼]

Search: __ [Search by ID or peer]

Click row to see detailed trace
```

---

## 9. Performance & Scalability

### 9.1 Performance Targets

| Operation | Target | Current (est.) |
|-----------|--------|----------------|
| Metric collection overhead | < 5% CPU | 2-3% |
| Metric write latency | < 10ms | 5-7ms |
| Dashboard update | 100-500ms | 200-400ms |
| Typical event query | < 500ms | 300-600ms |
| Complex query (with joins) | < 2s | 1-2s |
| Alert evaluation | < 100ms | 50-100ms |

### 9.2 Scalability Analysis

**Assumptions:**
- 1,000+ concurrent connections
- 1Hz sampling for fast metrics
- 30Hz+ for per-chunk events (scheduling decisions)
- 10-year retention of events

**Data volumes:**

```
Metrics generated per second:
- Per-connection metrics: 1,000 connections × 1 metric/s = 1,000 metrics/s
- Per-device metrics: 20 devices × 1 metric/s = 20 metrics/s
- Per-transport metrics: 10 lanes × 1 metric/s = 10 metrics/s
- Per-chunk events (Universe): 1,000 connections × 30 chunks/s = 30,000 events/s

Total: ~31,000 events/s + 1,030 metrics/s

Daily event count: 31,000 × 86,400 = 2.7 billion events/day
Annual event count: 2.7B × 365 = 985 billion events/year

Storage required (at 500 bytes/event): 500 GB/year
With compression (10x): 50 GB/year
```

### 9.3 Storage Optimization

1. **Differential encoding:**
   - Store only changes to metrics (delta-compressed)
   - Example: if latency was 45ms, next sample is 46ms, store only +1ms
   - Achieves 10-50x compression for smooth metrics

2. **Columnar storage:**
   - Store metrics in columnar format (all latencies together, all bandwidths together)
   - Enables better compression and faster aggregation queries

3. **Time-based partitioning:**
   - Split data by month or week
   - Move old partitions to read-only storage
   - Archive to object storage (S3, GCS) after 1 year

4. **Event aggregation:**
   - Aggregate per-chunk scheduling events into per-connection summaries
   - Keep raw events for recent data (7 days), aggregates for historical

### 9.4 Query Performance

**Indexes for fast queries:**

```sql
-- Metric queries by type and time
CREATE INDEX idx_metrics_type_time ON metrics(metric_type, timestamp_ns DESC);

-- Device-specific metrics
CREATE INDEX idx_metrics_device ON metrics(device_id, timestamp_ns DESC)
  WHERE metric_type IN ('cpu', 'memory', 'bandwidth');

-- Connection-level queries
CREATE INDEX idx_metrics_connection ON metrics(connection_id, timestamp_ns DESC)
  WHERE connection_id IS NOT NULL;

-- Universe event queries
CREATE INDEX idx_event_type_time ON universe_events(
  json_extract(metadata, '$.tdlb_event_type'),
  timestamp_ns DESC
) WHERE category = 'ComputeEvent';

CREATE INDEX idx_event_device_time ON universe_events(
  json_extract(metadata, '$.device_id'),
  timestamp_ns DESC
) WHERE category = 'ComputeEvent';
```

**Query caching:**

- Cache device metrics in memory (LRU, max 100MB)
- Cache global aggregates with 1-minute TTL
- Use query result cache for dashboard queries (5-minute TTL)

---

## 10. User Experience

### 10.1 Typical Workflows

**Workflow 1: Investigating a Latency Spike**

1. User sees alert: "High latency on QUIC/home→work"
2. Clicks alert to go to dashboard
3. Dashboard shows latency graph with red zone highlighted
4. User zooms into the spike (12:34-12:39)
5. Device grid shows home-ubuntu at 95% CPU during that time
6. User clicks "Show events for this time range"
7. Sees universe events: "home-ubuntu processing large task", "device overloaded"
8. User acknowledges alert and waits for device to cool down
9. Dashboard shows latency returning to normal (green zone)
10. User marks alert as resolved

**Workflow 2: Debugging a Failed Connection**

1. User queries events: "Show failed connections last 24h"
2. Sees 3 failures, all on Tor transport
3. Clicks on one failure (conn-abc123) to see full trace
4. Trace shows: connection started on Tor, then Tor node degraded (latency 800ms), then timeout
5. User checks Tor exit node status: "Exit node has ISP issues"
6. User recommends: "Add fallback to WebRTC before Tor times out"
7. Implements change and verifies fix works on next test

**Workflow 3: Generating Compliance Report**

1. User navigates to "Reports" section
2. Selects "Compliance Report for May 2026"
3. System generates report:
   - Queries all events from May
   - Verifies data residency (no cross-border transfers)
   - Checks encryption on all connections
   - Verifies access control logs
4. Generates PDF with digital signature
5. User exports as PDF and submits to auditors

### 10.2 Accessibility

- **Color-blind friendly:** Use patterns (dotted, dashed) in addition to colors
- **Mobile responsive:** Dashboard works on tablet/phone
- **Dark mode:** Support dark theme for low-light environments
- **Keyboard navigation:** All actions accessible via keyboard
- **Screen reader support:** Proper ARIA labels and semantic HTML

### 10.3 Customization

Users can create custom dashboards:

```yaml
Dashboard: My Team Dashboard

Panels:
  - Type: metric_graph
    Metrics: [latency_p99, throughput]
    Devices: [home-ubuntu, cloud-server-1]
    Time_range: last_24h
    Refresh: 1m

  - Type: alert_list
    Severity: [WARNING, CRITICAL]
    Acknowledge_action: auto_dismiss_after_30m

  - Type: device_grid
    Columns: [status, cpu, memory, active_connections]
    Sort_by: cpu_utilization DESC

  - Type: event_log
    Filters:
      - tdlb_event_type: [device_offline, connection_failed]
      - severity: high
    Limit: 50
    Auto_scroll: true
```

---

## 11. Data Privacy & Security

### 11.1 Privacy Guarantees

**Metrics never leak user identity or code:**

```rust
// ✓ OK to log
metrics.latency_p99_ms = 120.5;
metrics.bytes_transferred = 4500000;
metrics.transport_kind = "quic";

// ✗ NEVER log
metrics.file_path = "/home/user/secret.txt";
metrics.code_snippet = "fn my_algorithm() { ... }";
metrics.user_name = "alice@example.com";
metrics.peer_ip = "192.168.1.100";  // Unless aggregated/anonymized
```

**Anonymization strategy:**

```rust
// All identifiers are hashed
let device_id = blake3::hash(format!("device:{}", raw_device_name)).to_hex();
let connection_id = blake3::hash(format!("conn:{}", raw_uuid)).to_hex();
let peer_id = blake3::hash(format!("peer:{}", raw_peer_id)).to_hex();
```

### 11.2 Capability Token Enforcement

Only users with `tdlb.view_metrics` capability can query metrics:

```rust
async fn query_metrics(
    req: MetricsQuery,
    user: AuthenticatedUser,
) -> Result<Vec<Metric>> {
    // Check capability
    if !user.has_capability("tdlb.view_metrics") {
        return Err(Unauthorized);
    }
    
    // Check if user can see this device
    let device = devices.get(&req.device_id)?;
    if !user.can_access_device(&device) {
        return Err(Forbidden);
    }
    
    // Query and return metrics
    metrics_db.query(&req).await
}
```

**Capability levels:**

- `tdlb.view_summary` — Can see global metrics only
- `tdlb.view_metrics` — Can see all metrics
- `tdlb.view_events` — Can see Universe events
- `tdlb.acknowledge_alerts` — Can acknowledge alerts
- `tdlb.modify_policies` — Can change scheduling policies
- `tdlb.export_reports` — Can export compliance reports

### 11.3 Audit Log

Every query and action is logged:

```json
{
  "audit_action": "metrics_query",
  "user": "alice@example.com",
  "timestamp_ns": 1685740800000000000,
  "action_detail": {
    "query_type": "device_metrics",
    "device_id": "device-abc123",
    "time_range": "last_24h",
    "query_result_count": 1440
  },
  "ip_address": "192.168.1.100",
  "signature": "ed25519(...)"
}
```

---

## 12. Testing Strategy

### 12.1 Unit Tests

**Metric collection:**

```rust
#[test]
fn test_metric_aggregation() {
    let mut agg = MetricAggregator::new();
    agg.record(Metric { latency_ms: 50.0 });
    agg.record(Metric { latency_ms: 100.0 });
    agg.record(Metric { latency_ms: 150.0 });
    
    assert_eq!(agg.p50(), 100.0);
    assert_eq!(agg.p99(), 150.0);
    assert_eq!(agg.avg(), 100.0);
}

#[test]
fn test_alert_deduplication() {
    let mut alert_hist = AlertHistory::new();
    
    assert!(alert_hist.should_alert("key1", 1000));  // First alert
    assert!(!alert_hist.should_alert("key1", 1100)); // Duplicate
    assert!(alert_hist.should_alert("key1", 2000));  // After dedup window
}
```

### 12.2 Integration Tests

**Verify metrics are generated correctly for various scenarios:**

```rust
#[tokio::test]
async fn test_scheduling_decision_logged() {
    let scheduler = EcfRgScheduler::new();
    let universe = UniverseEventEmitter::new();
    
    let assignment = scheduler.assign(gsn, chunk_size, is_critical).await?;
    
    // Verify event was logged
    let events = universe.query_recent(
        QueryFilter {
            target: format!("connection:{}", conn_id),
            event_type: "scheduling_decision",
        }
    ).await?;
    
    assert_eq!(events.len(), 1);
    assert_eq!(
        events[0].metadata["chosen_primary"],
        assignment.primary
    );
}

#[tokio::test]
async fn test_connection_lifecycle() {
    // Create connection
    let conn = Connection::create(...).await?;
    assert_has_event("connection_created", &conn.id);
    
    // Send chunk
    conn.send_chunk(...).await?;
    assert_has_event("scheduling_decision", &conn.id);
    
    // Migrate
    conn.migrate(...).await?;
    assert_has_event("connection_migrated", &conn.id);
    
    // Complete
    conn.finish().await?;
    assert_has_event("connection_completed", &conn.id);
}
```

### 12.3 Load Tests

**Verify performance under high load:**

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_high_frequency_metrics() {
    let metrics_db = MetricsDatabase::new();
    
    // Simulate 1000 connections, 1Hz sampling for 60 seconds
    for _ in 0..1000 {
        tokio::spawn(async {
            for i in 0..60 {
                let metric = Metric {
                    device_id: "dev-123",
                    latency_ms: 50.0 + (i as f64) * 0.1,
                };
                metrics_db.record(metric).await.unwrap();
            }
        });
    }
    
    // Measure write latency
    let start = Instant::now();
    metrics_db.flush().await?;
    let duration = start.elapsed();
    
    // Should handle 1000 writes/sec with < 10ms flush time
    assert!(duration < Duration::from_millis(10));
}
```

### 12.4 Replay Tests

**Use Universe events to verify metrics are correct:**

```rust
#[tokio::test]
async fn test_metrics_match_events() {
    // Load historical events from Universe
    let events = universe.query_all(
        QueryFilter {
            category: "ComputeEvent",
            time_range: (now - 24h, now),
        }
    ).await?;
    
    // Replay events through metrics system
    for event in events {
        match event.metadata["tdlb_event_type"].as_str() {
            "scheduling_decision" => {
                let metric = event.to_metric()?;
                metrics_db.record(metric).await?;
            }
            _ => {}
        }
    }
    
    // Verify metrics match expected aggregates
    let total_connections = events.iter()
        .filter(|e| e.metadata["tdlb_event_type"] == "connection_completed")
        .count();
    
    let metric_connections = metrics_db.query(MetricsQuery {
        metric_type: "connection_count",
        aggregation: Sum,
    }).await?[0].value as usize;
    
    assert_eq!(total_connections, metric_connections);
}
```

---

## 13. Implementation Roadmap

### Phase 1: Foundation (Weeks 1-4)
- [ ] Implement TSDB schema in SQLite
- [ ] Add metric collection to TDLB scheduler
- [ ] Extend Universe event schema with TDLB types
- [ ] Create basic dashboard with system health panel

### Phase 2: Dashboard & Alerts (Weeks 5-8)
- [ ] Build full dashboard with all panels
- [ ] Implement alerting system and deduplication
- [ ] Add alert acknowledgment and escalation
- [ ] Create query API for events

### Phase 3: Reporting (Weeks 9-12)
- [ ] Implement automated report generation
- [ ] Add compliance reporting
- [ ] Create export formats (PDF, JSON, CSV)
- [ ] Add digital signatures to reports

### Phase 4: Advanced Features (Weeks 13+)
- [ ] Timeline replay visualization
- [ ] Custom dashboard builder
- [ ] Predictive alerting (anomaly detection)
- [ ] Integration with ML for optimization recommendations

---

## Success Criteria

1. ✓ **Complete visibility** — Every scheduling decision is visible in metrics/events
2. ✓ **Low overhead** — Metrics collection uses <5% CPU/memory
3. ✓ **Fast queries** — Typical queries return in <500ms
4. ✓ **User-friendly** — Operators can understand system state at a glance
5. ✓ **Compliance-ready** — Audit reports are sufficient for regulatory compliance
6. ✓ **Extensible** — Easy to add new metrics, alerts, dashboards

---

## Conclusion

The TDLB Observability & Metrics System provides complete visibility into the load balancer's behavior through real-time metrics collection, comprehensive event logging to Universe, intelligent alerting, and powerful visualization. By integrating with Bonsai's existing components (Universe, Compute Fabric, Survival System, Credits System), it enables operators to understand system behavior, debug issues quickly, and maintain regulatory compliance—all while adding minimal overhead.

The system is designed to scale to 1000+ concurrent connections with billions of events per year, and provides flexible querying, reporting, and alerting capabilities to meet the needs of operators, developers, and compliance auditors.
