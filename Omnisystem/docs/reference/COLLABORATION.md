# Bonsai Collaboration System — Complete Design Document

**Vision:** Every Bonsai user is a sovereign node. Real-time collaboration — video chat,
co-editing, shared terminals, distributed training, model building — flows through a single
hardened pipe: TransferDaemon. No central server. No cloud dependency. Post-quantum encrypted.
Childishly simple to use.

---

## Table of Contents

1. [Design Principles](#1-design-principles)
2. [Architecture Overview](#2-architecture-overview)
3. [TransferDaemon as Universal Protocol](#3-transferdaemon-as-universal-protocol)
4. [Session & Room Model](#4-session--room-model)
5. [Real-Time Messaging & Chat](#5-real-time-messaging--chat)
6. [Voice & Video Calling](#6-voice--video-calling)
7. [Collaborative Code Editing (CRDT)](#7-collaborative-code-editing-crdt)
8. [Shared File System & Workspace Sync](#8-shared-file-system--workspace-sync)
9. [Shared Terminal](#9-shared-terminal)
10. [Collaborative AI & Training](#10-collaborative-ai--training)
11. [Presence & Notifications](#11-presence--notifications)
12. [Security & Permissions Model](#12-security--permissions-model)
13. [UX Design — Childishly Simple](#13-ux-design--childishly-simple)
14. [Ecosystem-Wide TransferDaemon Unification](#14-ecosystem-wide-transferdaemon-unification)
15. [Implementation Roadmap](#15-implementation-roadmap)
16. [New Crate: bonsai-collab](#16-new-crate-bonsai-collab)

---

## 1. Design Principles

| Principle | How It's Achieved |
|-----------|-------------------|
| **Secure by default** | Every byte encrypted with session keys derived from TransferDaemon's Noise_XX handshake (X25519 + AES-256-GCM). Group keys use per-member sealed envelopes. Future: ML-KEM post-quantum hybrid. |
| **Decentralised** | No central server. mDNS for LAN discovery, libp2p/Kademlia DHT for WAN, Relay (TURN-style) for symmetric NAT. Works air-gapped over LAN. |
| **Offline-first** | All collaborative state is CRDT-backed and stored locally in CAS. Edits made offline are replayed on reconnect. Messages queued in mailbox pending delivery. |
| **One protocol for everything** | TransferDaemon is the only inter-device channel: chat, video, file sync, training tasks, KDB streaming, survival alerts — one audited security boundary. |
| **Permissioned, role-based** | Invitation codes scope what a joining peer can do. Roles: Owner, Admin, Contributor, Viewer. The `CapabilityRegistry` enforces every action. |
| **Integrated with existing tools** | Collaboration enhances each existing feature rather than duplicating it. Chat uses `AgentMailbox`. Video uses the existing `WebRtc` `LaneKind`. CRDT uses `bonsai-crdt`. |

---

## 2. Architecture Overview

```
┌────────────────────────────────────────────────────────────────────────────┐
│                        Bonsai Collaboration Layer                           │
│                                                                              │
│   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌───────────────┐  │
│   │   Messaging  │  │  CRDT Sync   │  │  Workspace   │  │   Presence    │  │
│   │  (chat, DM,  │  │  (code edit, │  │  (file tree, │  │  (heartbeat,  │  │
│   │   reactions) │  │   cursors)   │  │   terminal)  │  │   status)     │  │
│   └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬────────┘  │
│          │                 │                  │                 │           │
│          ├─────────────────┴──────────────────┴─────────────────┤           │
│          ▼                                                       ▼           │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │              TransferDaemon Unified Transport                        │   │
│   │  LaneKind: WebRTC · TCP · Relay · WifiDirect · Onion · InProcess    │   │
│   │  ECF-RG Scheduler · Reassembly Window · GsnAllocator                │   │
│   │  AgentMailbox (signed envelopes) · BonsaiIdentity (Ed25519)         │   │
│   │  StreamType: Chat · CrdtSync · Media · Terminal · Presence · Health │   │
│   └───────────────────────────────┬─────────────────────────────────────┘   │
│                                   │                                          │
│          ┌────────────────────────┼────────────────────────┐                │
│          ▼                        ▼                         ▼                │
│   ┌─────────────┐      ┌──────────────────┐      ┌──────────────────┐      │
│   │ Desktop App │      │  Android Workspace│      │   Buddy App      │      │
│   │ (Tauri)     │      │  (bonsai-core-   │      │  (Buddy API +    │      │
│   │             │      │   android/UniFFI) │      │   bonsai-collab) │      │
│   └─────────────┘      └──────────────────┘      └──────────────────┘      │
└────────────────────────────────────────────────────────────────────────────┘
```

All collaboration state lives in actors (`bonsai-actors`) that are supervised, checkpointed
to CAS, and hot-reloadable. If the initiator drops, the room migrates to the next most-connected peer.

---

## 3. TransferDaemon as Universal Protocol

TransferDaemon (`p2p-core` + `p2p-crypto` + `bonsai-mailbox` + `bonsai-transfer-store`) is extended with two additions for collaboration:

### 3.1 StreamType Enum

Added to `p2p-core/src/lane.rs` as a logical multiplexer layer above the physical `LaneKind`. A single QUIC connection carries multiple `StreamType` channels simultaneously.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamType {
    FileTransfer,     // existing
    Chat,             // group/DM text messages
    CrdtSync,         // CRDT document deltas
    Media,            // WebRTC SDP signaling (actual media is RTP over WebRTC)
    Terminal,         // PTY output stream
    Presence,         // cursor positions, heartbeats, online status
    Health,           // survival system inter-node alerts
    TaskDistribute,   // distributed training task assignment
    KnowledgeFetch,   // KDB module streaming
}
```

### 3.2 Current Inter-Device Communication — Unification Map

Every feature currently using a non-TransferDaemon transport is migrated:

| Feature | Old Transport | Unified TransferDaemon Channel |
|---------|--------------|-------------------------------|
| Desktop ↔ Buddy IPC | Tauri event bus (same-machine) | `InProcess` lane (zero-copy, same machine) |
| Desktop ↔ Android | ADB / custom HTTP | `WebRtc` or `Tcp` lane via mDNS/Kademlia discovery |
| Agent messaging | `AgentMailbox` (local only) | `AgentMailbox` over `Relay` lane for remote peers |
| Survival inter-node | Not yet built | `Health` stream over `TransferDaemon` |
| Training coordination | Not yet built | `TaskDistribute` stream; worker devices pull task chunks |
| KDB module sharing | Not yet built | `KnowledgeFetch` stream; peers request modules by BLAKE3 hash |
| File transfer | Manual OS file share | `FileTransfer` stream; resume-capable, integrity-verified |
| Bot relay (Discord/Telegram) | Direct HTTP from bonsai-bot | Bot publishes via `Chat` stream; app subscribes |
| Model hot-reload notification | `tauri::emit` (local) | `Presence` stream with `ModelLoaded` event type |
| Distributed grid tasks | Not yet built | `TaskDistribute` stream + CAS for input/output |

### 3.3 Presence Heartbeat Protocol

Every connected peer broadcasts a `PresenceHeartbeat` every 15 seconds:

```rust
#[derive(Serialize, Deserialize)]
pub struct PresenceHeartbeat {
    pub peer_id: String,          // Ed25519 fingerprint
    pub display_name: String,
    pub session_ids: Vec<String>, // which rooms this peer is in
    pub active_file: Option<String>,
    pub cursor: Option<CursorPos>,
    pub is_speaking: bool,
    pub has_video: bool,
    pub device_type: DeviceType,
    pub ts: u64,                  // unix ms
}
```

Three missed heartbeats → peer marked offline in room CRDT state.

---

## 4. Session & Room Model

### 4.1 Room Abstraction

A **Room** is the core unit of collaboration. It is:
- A set of peers sharing a symmetric session key.
- A CRDT map of `participant_id → ParticipantInfo` (eventual consistency across all members).
- An invite code that expires and grants scoped permissions.
- Stored as a CAS snapshot so it survives any single peer going offline.

```
Room lifecycle:
  Owner creates → generates invite code → broadcasts room_id via QR/link
  Joiner enters code → authenticates with TransferDaemon (Noise_XX)
  → receives session key (sealed with joiner's public key)
  → room CRDT merges joiner's ParticipantInfo
  → all streams activate
```

### 4.2 Invite Code Generation

A 6-word BIP-39 passphrase (from `transfer_generate_phrase`) encodes:
- Room ID (UUID, 16 bytes)
- A one-time token (16 bytes) that expires after 24 hours or first use
- A permission scope bitmask

Example: `bonsai://join/turtle-river-apex-golden-lamp-frost`

Also available as a QR code (same format as the existing `generate_pair_qr` command).

### 4.3 Role Permissions

```
Owner:       all permissions + invite management + kick
Admin:       all permissions except kick owner
Contributor: edit + speak + video + screen share
Viewer:      read + speak (no edit, no execute, no screen share)
```

The `CapabilityRegistry` in `bonsai-capability-registry` enforces every action before it
executes. A `Contributor` attempting to run a terminal command gets a `CapabilityDenied` error.

---

## 5. Real-Time Messaging & Chat

### 5.1 Group Mailbox

The existing `AgentMailbox` (1:1 signed envelopes) is extended with a `GroupMailbox`:

```rust
pub struct GroupMailbox {
    pub group_id: String,          // uuid
    pub members: Vec<AgentId>,
    pub history: Arc<Mutex<Vec<MailEnvelope>>>,  // CRDT-reconciled
    pub sqlite_db: Arc<Mutex<Connection>>,       // local FTS index
}
```

A message to `group:<id>` is forwarded by the sender's `MailboxHub` to all members.
Each member stores it locally in SQLite with FTS5 for search.

### 5.2 Message Types (via envelope `topic` field)

| topic | Payload |
|-------|---------|
| `collab.chat.text` | `{ text, reply_to?, mentions[] }` |
| `collab.chat.reaction` | `{ message_id, emoji }` |
| `collab.chat.typing` | `{ is_typing }` (ephemeral, not stored) |
| `collab.chat.read_receipt` | `{ up_to_message_id }` |
| `collab.chat.delete` | `{ message_id }` |
| `collab.presence` | `PresenceHeartbeat` |
| `collab.crdt.delta` | CRDT operation bytes |
| `collab.media.signal` | `MediaSignal` (SDP offer/answer/ICE) |
| `collab.terminal.output` | PTY output bytes |
| `collab.terminal.input` | PTY input (from permitted peer) |
| `collab.file.event` | `{ kind: created|modified|deleted, path, cas_hash }` |
| `collab.training.progress` | `{ phase, step, loss, eta_secs }` |
| `collab.kdb.module_updated` | `{ module_name, new_version, cas_hash }` |

### 5.3 Tauri Commands

```rust
collab_create_session(project_path, permissions) -> SessionInfo
collab_join_session(invite_code, display_name) -> SessionInfo
collab_leave_session(session_id)
collab_send_message(session_id, text, reply_to?)
collab_send_reaction(session_id, message_id, emoji)
collab_get_history(session_id, limit, before_id?) -> Vec<MessageDto>
collab_search_messages(session_id, query) -> Vec<MessageDto>
collab_update_permissions(session_id, peer_id, role)
collab_kick_participant(session_id, peer_id)
collab_get_participants(session_id) -> Vec<ParticipantSummary>
collab_broadcast_cursor(session_id, file, line, column)
```

---

## 6. Voice & Video Calling

### 6.1 WebRTC Media via TransferDaemon Signaling

The existing `LaneKind::WebRtc` in `p2p-core` handles data. For media, we use
WebRTC's built-in RTP transport (audio/video) but route the **SDP signaling** through
`AgentMailbox` (topic: `collab.media.signal`), eliminating the need for a separate
signaling server.

```
Call flow:
  Alice clicks "Call" in Bonsai
  → CallManager sends MediaSignal::Offer{sdp} via mailbox to all room members
  → Bob's Bonsai shows incoming call notification
  → Bob accepts → sends MediaSignal::Answer{sdp}
  → ICE candidates exchanged via collab.media.signal envelopes
  → WebRTC peer connection established (DTLS-SRTP encrypted RTP)
  → Audio/video flows directly P2P
```

The signaling uses the existing `transfer_send_message` infrastructure. No new server needed.

### 6.2 Media Capture (Native Rust)

Desktop media capture via:
- **Audio:** `cpal` (already in `Cargo.toml`) → raw PCM → Opus encode → RTP packetize
- **Video:** `nokhwa` crate → raw BGRA frames → VP8/H.264 encode via `openh264` or `libvpx` bindings
- **Screen share:** `scrap` crate (already referenced in architecture) → frame capture → VP8

All encoded as WebRTC media tracks on a `RTCPeerConnection` created by the `webrtc-rs` crate.

### 6.3 SFU for Groups > 2

For groups > 2 participants, a **Selective Forwarding Unit** actor runs on the most powerful
node in the room (typically the room Owner's desktop). Each peer sends audio/video to the SFU;
the SFU forwards the appropriate streams to each subscriber. This halves the upload requirement
compared to full mesh.

The SFU is implemented as a `bonsai-actors` actor with a mailbox. When the SFU host leaves,
the room automatically elects a new SFU host via a simple leader election (highest uptime score).

### 6.4 AI Call Assistance

During a call, BonsAI can be activated as a silent participant:
- **Live transcription:** Whisper sidecar processes local audio; transcript streamed to chat.
- **Translation:** Local LLM translates transcription to a target language, appended to chat.
- **Post-call summary:** After call ends, transcript sent to BonsAI → generates meeting notes
  as a Markdown file saved to CAS and added to the project file tree.
- **Meeting notes** automatically become a KDB knowledge module (optional, user consent).

### 6.5 UI: CallPanel.svelte

```
┌─────────────────────────────────────────────────────────────────┐
│  🔴 Live · 14:37  [Alice] [Bob] [Charlie +2]                    │
│                                                                   │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐                 │
│  │  [video]   │  │  [video]   │  │  [screen]  │                 │
│  │  Alice 🎤  │  │  Bob 🔇    │  │  Charlie   │                 │
│  └────────────┘  └────────────┘  └────────────┘                 │
│                                                                   │
│  [🎤 Mute] [📷 Video off] [🖥 Share screen] [📝 Transcript] [✕] │
│                                                                   │
│  Live transcript:                                                 │
│  Alice: "Let's look at the training loss curve..."               │
│  Bob: "The validation loss diverged at step 400..."              │
└─────────────────────────────────────────────────────────────────┘
```

The window is detachable (always-on-top) and can float above the code editor.

---

## 7. Collaborative Code Editing (CRDT)

### 7.1 Sequence CRDT

`bonsai-crdt` already has `GCounter`, `LWWRegister`, `OrSet`. We add a `Sequence<T>` type
implementing the **Replicated Growable Array (RGA)** algorithm:

```rust
pub struct Sequence<T: Clone + Serialize> {
    elements: Vec<RgaNode<T>>,
    tombstones: HashSet<OperationId>,
    site_id: SiteId,
    sequence_counter: u64,
}

pub struct RgaNode<T> {
    id: OperationId,        // (site_id, counter)
    parent: Option<OperationId>,
    value: T,
    deleted: bool,
}
```

This gives correct insert/delete semantics for concurrent edits without locks.

### 7.2 Editor Integration

The `MonacoEditor` Svelte component gets a new `collaborative` prop. When active:
1. Every Monaco `onDidChangeModelContent` event is converted to an RGA operation.
2. The op is serialized and sent via `collab.crdt.delta` envelope.
3. Incoming ops from peers are applied to the RGA, then the resulting diff is applied
   to the Monaco model via `editor.executeEdits()`.

Cursor positions are broadcast via `collab.presence` every 100ms while the editor is focused.
Remote cursors are rendered using Monaco's `deltaDecorations()` API with peer-specific colors.

### 7.3 Offline Support

The CRDT state is persisted to CAS after every 10 operations or 5 seconds. A newly joining peer
requests the latest snapshot via `KnowledgeFetch` stream, then replays pending ops from the
room's operation log (also stored in CAS).

### 7.4 Conflict-Free File Tree

The project file tree is a CRDT `Map<path, FileMetadata>`:

```rust
pub struct FileMetadata {
    pub cas_hash: String,      // BLAKE3 of current content
    pub modified_at: u64,      // unix ms
    pub modified_by: AgentId,
    pub size: u64,
}
```

When a file is saved:
1. Content stored in CAS → returns BLAKE3 hash
2. `FileMetadata` updated in CRDT map
3. `collab.file.event` envelope broadcast to all peers

Peers sync the file lazily: file tree shows the update immediately (via CRDT), but content is
fetched only when opened (via `KnowledgeFetch` stream requesting the CAS hash).

---

## 8. Shared File System & Workspace Sync

### 8.1 WorkspaceActor

A new `bonsai-actors` actor:

```rust
pub struct WorkspaceActor {
    room_id: String,
    file_tree: CrdtMap<String, FileMetadata>,
    cas: Arc<CasStore>,
    watcher: RecommendedWatcher,    // notify crate
    mailbox_tx: AgentMailboxTx,
}
```

The watcher monitors the project directory. On each file change:
1. Hash new content → store in CAS.
2. Update CRDT map.
3. Broadcast `collab.file.event`.

The actor checkpoints its CRDT state to CAS every 60 seconds (survive app restarts).

### 8.2 Partial Sync (Mobile-Friendly)

Users can mark specific subtrees as "local only" (not synced). Configuration stored in
`.bonsai/sync.toml`:

```toml
[sync]
exclude = ["build/", "target/", "*.gguf", "models/"]
include_only = ["src/", "docs/"]    # for viewers
```

Mobile peers (identified by `DeviceType::Mobile` in their `PresenceHeartbeat`) automatically
get an exclusion for binaries > 10MB unless they opt in.

### 8.3 Conflict Resolution

If two peers modify the same file concurrently (both have modified CAS hash after a fork):
1. Both hashes are present in the CRDT map as a conflict marker.
2. The app shows a merge diff dialog (like `git merge` UI).
3. User picks or edits the resolution.
4. Resolved content stored in CAS; CRDT map updated.

Text files can also be auto-merged using the three-way merge algorithm if a common ancestor
CAS hash is known.

---

## 9. Shared Terminal

### 9.1 Terminal Multiplexer Actor

```rust
pub struct SharedTerminalActor {
    session_id: String,
    pty: Arc<Mutex<PtySession>>,
    subscribers: Vec<AgentId>,    // who receives output
    input_allowed: HashSet<AgentId>, // who can send input
    ring_buffer: VecDeque<u8>,   // last 64KB of output for late joiners
}
```

When a peer joins the shared terminal:
1. They receive the ring buffer (last 64KB of output) for context.
2. Output from the PTY is broadcast via `collab.terminal.output` envelope to all subscribers.
3. Input from permitted peers arrives via `collab.terminal.input` → forwarded to PTY.

### 9.2 Access Control

The `input_allowed` set is managed by the Owner/Admin. UI shows:
- A "Take control" button (requests input permission from owner).
- "Pass control" button (relinquishes).
- Owner/Admin can grant/revoke via the collaboration panel.

Only one peer at a time has input control (mutex). Output is always broadcast to all.

---

## 10. Collaborative AI & Training

### 10.1 Shared Training Jobs

The `TrainingLoop` actor accepts a `RemoteTaskRequest` envelope (topic: `collab.training.task`)
from peers who have `can_execute_commands` permission. This enables:

- **Owner** starts a distributed training run.
- **Contributor devices** receive task chunks (e.g., gradient computation on a data shard).
- **Progress events** (`collab.training.progress`) broadcast live to all participants.
- Loss curves visible in every participant's training dashboard.

### 10.2 Collaborative Model Builder

The `ModelBuilder.svelte` component gets a `session_id` prop. When active:
- The list of active knowledge modules is a `CrdtOrSet<String>` (no-conflict add/remove).
- Any Contributor can add/remove modules.
- The current configuration is shown to all participants with colored indicators per user.
- "Deploy" button available to Owner/Admin only.

### 10.3 Shared Knowledge Modules

When a peer adds new entries to a `.kmod`:
1. The module is rebuilt (embeddings recomputed via the Python pipeline).
2. New version stored in CAS.
3. `collab.kdb.module_updated` broadcast with `{ module_name, new_version, cas_hash }`.
4. Other peers receive the update notification and can pull the new module on demand.

This enables **collaborative knowledge building**: a team can collectively grow a shared
domain knowledge module, with each member contributing from their local data.

---

## 11. Presence & Notifications

### 11.1 Presence Heartbeat

Every 15 seconds, each peer broadcasts a `PresenceHeartbeat` via `collab.presence` to all
room members. Three missed heartbeats → peer shown as "offline" in the participants list.

### 11.2 Push Notifications

**Desktop:** When a user is not focused on Bonsai, collaboration events are delivered as
system notifications (using `tauri-plugin-notification`, already in `Cargo.toml`).

**Android/iOS (Buddy app):** Two modes:
1. **WebSocket long-poll:** Buddy maintains a persistent connection to the TransferDaemon relay.
   Events arrive in real time as long as the app is in the foreground or recent background.
2. **Firebase Cloud Messaging (opt-in):** For battery-efficient background notifications.
   The relay sends an FCM trigger to the device when a message arrives; the app wakes up,
   fetches the message via TransferDaemon, and dismisses the FCM.

### 11.3 Offline Message Delivery

When a peer is offline:
1. Sender stores the envelope in their local CAS with the recipient's fingerprint.
2. Sender's relay (if running) also stores it.
3. When recipient reconnects, their `AgentMailbox` pulls pending envelopes from all known
   relays and direct peers.

This is functionally equivalent to the existing `transfer_poll_inbox` command,
extended to cover group messages.

---

## 12. Security & Permissions Model

### 12.1 Session Key Distribution

1. Room creator generates a 32-byte symmetric session key `K_session`.
2. For each joining peer, `K_session` is sealed with the peer's Ed25519 public key
   (using X25519 key agreement + AES-256-GCM).
3. The sealed bundle is sent via `AgentMailbox` (`topic: collab.session.key`).
4. Key rotation: `K_session` is rotated when a peer is kicked or every 24 hours.

### 12.2 Zero-Trust Enforcement

Every incoming envelope is:
1. **Signature-verified** against sender's registered `AgentId` (Ed25519 `verify`).
2. **Timestamp-checked** (reject envelopes older than 5 minutes to prevent replay).
3. **Role-checked** against the room's CRDT participant map before any action executes.

### 12.3 Audit Log

All collaboration events are appended to a tamper-evident WAL in CAS:
`~/.bonsai/collab/<room_id>/audit.log`

Each entry: `{ ts, peer_id, action, target?, detail }` signed by the acting peer.
The Owner can export the audit log at any time. This supports post-incident review
and compliance for teams using Bonsai in regulated environments.

### 12.4 Confidential Projects

For sensitive work, the room can require **hardware attestation** before a peer joins:
- Android: uses Android Keystore + SafetyNet/Play Integrity API.
- Desktop Linux: TPM 2.0 quote (optional).
- If attestation fails, the joiner is assigned `Viewer` role only.

---

## 13. UX Design — Childishly Simple

### 13.1 Starting a Session

**"Share" button** in the top toolbar (next to "🧠 Builder"):

```
[ Share ▾ ]
  ├─ 🎤 Voice call
  ├─ 📹 Video call
  └─ 🤝 Share project for editing
```

Clicking "Share project" → session created immediately → panel opens showing:
```
  Invite: turtle-river-apex-golden-lamp-frost
  [ Copy code ]  [ Show QR ]
  
  Waiting for participants...
```

### 13.2 Joining a Session

From main menu: "Join session" → text field for code or QR scan →
connection happens in < 3 seconds on local network.

If the project doesn't exist locally: file tree streams on-demand as files are opened
(no full upfront sync required).

### 13.3 In-Session Collaboration Panel

Persistent panel (left sidebar or floating):

```
┌─────────────────────────────────────────────┐
│  🤝 Session: My Project                     │
│  ● 3 participants  · 14 min                 │
│                                             │
│  PARTICIPANTS                               │
│  ● Alice (you, Owner)                       │
│  ● Bob   [editing: main.rs:42]              │
│  ○ Carol [offline]                          │
│                                             │
│  [ 🎤 Start call ]  [ + Invite ]            │
│                                             │
│  ACTIVITY                                   │
│  Bob: edited main.rs (+3 -1 lines)          │
│  Alice: saved config.toml                   │
│  System: Carol disconnected                  │
│                                             │
│  MESSAGES                                   │
│  Bob: "Should I refactor this?"             │
│  ─────────────────────────────────          │
│  [ Type a message… ]           [Send]       │
└─────────────────────────────────────────────┘
```

### 13.4 Collaborative Editor Indicators

In the code editor:
- Remote cursors shown as blinking vertical lines in the peer's color.
- Remote selections shown as colored highlights.
- A small avatar chip above each cursor (peer's name + color).
- Tooltip on hover: "Bob is here".

### 13.5 Error States — Human Language

All collaboration errors shown as plain-language toasts:
- "Can't reach Bob right now — he might be offline. Messages will be delivered when he reconnects."
- "Carol's invitation code expired. Ask them to generate a new one."
- "Your changes are saved locally. They'll sync when you reconnect to the internet."

---

## 14. Ecosystem-Wide TransferDaemon Unification

Beyond the collaboration feature itself, every cross-device interaction in Bonsai should
route through TransferDaemon. This table defines the complete unification target:

### 14.1 Integration Map

| Subsystem | Current State | Target State |
|-----------|--------------|--------------|
| Desktop ↔ Buddy chat | Buddy API HTTP (`buddy_api_server.rs`) | `Chat` stream over `InProcess` lane (same machine) or `WebRtc` lane (remote Buddy) |
| Desktop ↔ Android Workspace | ADB + custom HTTP bridge | `WebRtc`/`Tcp` lane via mDNS discovery; same `AgentMailbox` |
| Survival event propagation | Local only | `Health` stream to all known peers; survival fix broadcast so others can learn |
| Distributed grid tasks | Not built | `TaskDistribute` stream; coordinator sends `TaskRequest`, worker sends `TaskResult` |
| KDB module distribution | Manual `.bkp` import | `KnowledgeFetch` stream; request module by BLAKE3 hash; receive chunks like a file transfer |
| File transfer | `transfer_send_file_loopback` (loopback test only) | Real `Tcp`/`WebRtc` lane for actual peer-to-peer |
| Bot relay | bonsai-bot direct HTTP | Bot registers as a `MailboxHub` agent; receives `Chat` envelopes; bridges to Discord/Telegram |
| Model adapter updates | Manual file copy | `FileTransfer` stream with version metadata; receiver hot-reloads adapter |
| Training progress events | Tauri emit (local UI only) | `TaskDistribute` stream broadcasts to all session peers |
| CAS content retrieval | Local filesystem only | `KnowledgeFetch` stream allows peer CAS lookup (BitTorrent-like, content-addressed) |

### 14.2 bonsai-transfer-store as Universal Session Persistence

The `EncryptedStore` (keyed K/V store) becomes the persistence layer for collaboration state:

- `collab/rooms/<room_id>/state` — CRDT-serialized room state
- `collab/rooms/<room_id>/messages` — message history
- `collab/rooms/<room_id>/audit` — audit log
- `collab/rooms/<room_id>/crdt/<file_path>` — per-file RGA state
- `peers/<fingerprint>/profile` — cached peer display name + avatar hash

All keys are accessible via the existing `transfer_store_put`/`transfer_store_get` commands,
or the new `collab_*` commands that wrap them with higher-level semantics.

---

## 15. Implementation Roadmap

| Phase | Focus | Key Deliverables |
|-------|-------|-----------------|
| **1** | StreamType + Room + Presence | Add `StreamType` to transfer-core. `RoomState` CRDT. Invite code generation. Presence heartbeat. `CollabPanel.svelte` (participants list only). |
| **2** | Group messaging | `GroupMailbox` actor. Chat stream. `ChatChannel.svelte`. Typing indicators, reactions, threads, search. |
| **3** | Voice & video | `CallSession` actor. WebRTC SDP via mailbox. `cpal` audio + `nokhwa`/`scrap` capture. `CallPanel.svelte`. SFU for 3+ peers. |
| **4** | CRDT editor | `Sequence<char>` (RGA) in `bonsai-crdt`. Monaco collaborative mode. Cursor presence. Conflict-free file tree. |
| **5** | Shared terminal | `SharedTerminalActor`. PTY broadcast. Input permission control. Ring buffer replay for joiners. |
| **6** | AI + training collab | Distributed training task dispatch. Shared Model Builder CRDT. KDB module broadcast. Post-call summary. |
| **7** | Ecosystem unification | Migrate all inter-device channels to TransferDaemon (see §14.1 table). Bot relay as MailboxHub agent. |
| **8** | Mobile | Android Buddy app collab support. FCM integration. Thermal-aware participation (reduce contrib when battery low). |
| **9** | Polish & audit | Childish simplicity audit. Accessibility. Penetration testing of session key exchange. Performance profiling. |

---

## 16. New Crate: bonsai-collab

All collaboration business logic lives in a new crate that is shared between the desktop
app, the Buddy app, and the Android core library:

```
crates/bonsai-collab/
├── Cargo.toml
└── src/
    ├── lib.rs           — pub re-exports
    ├── room.rs          — RoomState CRDT, ParticipantInfo, PermissionSet
    ├── session.rs       — CollabSession actor, invite code gen/validation
    ├── message.rs       — GroupMailbox, ChatMessage, MessageHistory
    ├── presence.rs      — PresenceHeartbeat, cursor tracking
    ├── media.rs         — MediaSignal, CallSession, SFU routing logic
    ├── workspace.rs     — WorkspaceActor, file tree CRDT, conflict detection
    ├── terminal.rs      — SharedTerminalActor, PTY multiplexer
    ├── training.rs      — distributed task dispatch, progress broadcast
    └── error.rs         — CollabError enum
```

Dependencies:
```toml
[dependencies]
p2p-core   = { path = "../p2p-core" }
p2p-crypto = { path = "../p2p-crypto" }
bonsai-mailbox         = { path = "../bonsai-mailbox" }
bonsai-crdt            = { path = "../bonsai-crdt" }
bonsai-cas             = { path = "../bonsai-cas" }
bonsai-actors          = { path = "../bonsai-actors" }
```

The Tauri `collaboration_commands.rs` module calls into `bonsai-collab` for all logic,
keeping the Tauri layer thin (just command routing and state management).

---

## Appendix A: Why Not Cloud-Based Collaboration?

| Cloud approach | Problem | Bonsai approach |
|---------------|---------|----------------|
| Central server (OT) | Single point of failure; server sees all content | Peer-to-peer; server never sees plaintext |
| Operational Transform (e.g., VS Code Live Share) | Complex convergence algorithm; requires server as tiebreaker | CRDT: deterministic convergence without coordinator |
| WebRTC STUN/TURN only | STUN fails for symmetric NAT; TURN server costs | libp2p + Relay lane; user can self-host relay |
| Firebase/Firestore sync | Vendor lock-in; data leaves the machine | CAS + TransferDaemon; stays on user devices |

## Appendix B: Network Topology Decision Tree

```
Number of participants:
  2 → Full mesh (1 connection each, minimal overhead)
  3–5 → Full mesh (still manageable: 3–10 connections)
  6–12 → Star topology (Owner as SFU/hub)
  13+ → Hierarchical (sub-groups, each with a local SFU)
  
LAN only (mDNS discovery):
  → TCP lane (lowest latency, no relay needed)
  
Behind NAT (STUN success):
  → WebRTC lane with STUN hole-punching
  
Symmetric NAT (STUN fails):
  → Relay lane (TURN-style via Bonsai relay server or self-hosted)
  
Air-gapped / offline:
  → WifiDirect lane (Android) or InProcess lane (same machine)
```
