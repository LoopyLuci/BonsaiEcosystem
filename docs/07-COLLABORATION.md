# Collaboration & TransferDaemon

Bonsai enables real-time collaboration between users — shared code editing, group chat, video calls, and shared terminals — all powered by **TransferDaemon**, a secure peer-to-peer protocol that runs entirely on your devices with no central server.

---

## TransferDaemon: The Universal Protocol

Every inter-device communication in Bonsai — file transfer, messaging, CRDT sync, media streams, knowledge fetch, task distribution — runs over TransferDaemon.

### Architecture

```
Device A                          Device B
┌──────────────────────┐          ┌──────────────────────┐
│  bonsai-workspace    │          │  bonsai-workspace    │
│  ┌────────────────┐  │          │  ┌────────────────┐  │
│  │ CollabCommands │  │          │  │ CollabCommands │  │
│  └───────┬────────┘  │          │  └───────┬────────┘  │
│          │           │          │          │           │
│  ┌───────▼────────┐  │          │  ┌───────▼────────┐  │
│  │ TransferState  │◄─┼──────────┼─►│ TransferState  │  │
│  └───────┬────────┘  │          │  └───────┬────────┘  │
│          │           │          │          │           │
│  ┌───────▼────────┐  │          │  ┌───────▼────────┐  │
│  │ bonsai-p2p     │  │          │  │ bonsai-p2p     │  │
│  │ (WebRTC/libp2p)│  │          │  │ (WebRTC/libp2p)│  │
│  └────────────────┘  │          │  └────────────────┘  │
└──────────────────────┘          └──────────────────────┘
              │                              │
              └──────── Direct P2P ─────────┘
                    (or via bonsai-relay)
```

### Transport Lanes

TransferDaemon automatically selects the best available transport:

| Lane | Use case | Fallback |
|---|---|---|
| **WebRTC** (data channel) | Low-latency control, CRDT ops | Yes |
| **WebRTC** (media track) | Audio/video calls | No |
| **libp2p / QUIC** | File transfer, large payloads | Yes |
| **Tor** | Privacy-sensitive connections | On demand |
| **Bluetooth / Wi-Fi Direct** | Local mesh, no internet | Yes |
| **Relay** (`bonsai-relay`) | NAT traversal fallback | Automatic |
| **In-Process** | Two instances on same machine | Testing only |

The ECF-RG scheduler multiplexes multiple streams over available lanes simultaneously, maximising throughput and minimising latency.

### Security
- **Identity**: each device has an Ed25519 keypair. The public key is your identifier.
- **Session keys**: X25519 Diffie-Hellman establishes a shared secret; AES-256-GCM encrypts all payloads.
- **Signed envelopes** (`bonsai-mailbox`): every message is signed by the sender's Ed25519 key. Recipients verify before processing.
- **Post-quantum**: ML-KEM-768 key encapsulation is planned for a future release.

---

## Collaboration Sessions

### Creating a Session

1. Click the **Collaboration** button in the toolbar (or `Ctrl+Shift+C`).
2. Click **Host Session**.
3. Fill in options:
   - **Your display name**
   - **Allow editing** – participants can edit files
   - **Allow voice** – participants can join audio calls
   - **Allow video** – participants can share video
4. Click **Create**. An **Invitation Code** is generated (e.g., `bonsai-leaf-3721`).
5. Share the code with collaborators.

### Joining a Session

1. Click **Collaboration → Join Session**.
2. Enter the invitation code and your display name.
3. Click **Join**. You are connected directly to the host via TransferDaemon.

### Session Panel

Once connected, the **Session Panel** shows:

```
┌─────────────────────────────────┐
│  Session: bonsai-leaf-3721      │
│                                 │
│  Participants (3)               │
│  🟢 Alice (host) — editing App.svelte
│  🟢 Bob — idle                  │
│  🟡 Carol — away                │
│                                 │
│  [📞 Voice Call]  [📹 Video]    │
│  [💬 Chat]        [🖥 Terminal] │
└─────────────────────────────────┘
```

Green 🟢 = online. Yellow 🟡 = idle. Grey ○ = offline.

---

## Shared File Tree

When you are in a collaboration session, the **File Tree** shows updates from all participants in real time:

- File created by Bob → appears in your tree immediately
- File renamed by Alice → updates in your tree
- File deleted by Carol → greys out (you can restore it from history)

The file tree is synchronised via a **CRDT Map** (`bonsai-crdt`): each entry is a `LwwRegister<FileMetadata>` keyed by path. Concurrent operations (e.g., two users rename the same file) are resolved deterministically — the operation with the higher timestamp wins.

---

## Collaborative Code Editing

When multiple users have the same file open, their edits are merged in real time using CRDTs.

### How it works
1. Each keystroke in Monaco generates a CRDT operation (insert character at position, or delete).
2. The operation is broadcast over the session's WebRTC data channel.
3. Remote operations are applied to your local document model.
4. Monaco re-renders the affected lines.

### Remote Cursors
Every participant's cursor and selection is visible in your editor:
- Each user has a distinct colour.
- A label with their name appears near the cursor.
- Selections are shown as highlighted regions.

Cursor positions are broadcast as `LwwRegister<CursorPosition>` updates.

### Conflict-Free Merging
Because CRDTs are commutative and associative, concurrent edits never conflict — they merge automatically. There is no "merge conflict" dialog. You can trust that everyone ends up with the same document.

### Offline editing
If you temporarily lose the connection, you keep editing locally. Your operations queue up. When the connection resumes, the queue is replayed and merged.

---

## Integrated Chat

Every collaboration session has a built-in group chat:

- Messages appear in the **Session Chat** panel (click 💬 in the session panel).
- Messages are signed (sender's Ed25519 key) and broadcast to all participants.
- **Threading** – click **Reply** on any message to create a thread.
- **Reactions** – hover a message and click the 😊 icon to add emoji reactions.
- **Typing indicators** – "Alice is typing…" appears while she composes a message.
- **History** – the last 100 messages are kept in memory for the session lifetime. Persistent history can be enabled in Settings.

---

## Voice & Video Calls

### Starting a call
Click **📞 Voice Call** or **📹 Video** in the session panel. All participants are notified and can join.

### One-to-One calls
For private calls: right-click a participant → **Call**. Uses WebRTC directly between the two devices.

### Multi-party calls
- **Mesh** (≤ 4 participants): each device sends and receives directly to all others. Best on LAN.
- **SFU** (> 4 participants): one device (the host's desktop) acts as a Selective Forwarding Unit, receiving all streams and forwarding to each participant. Reduces outbound bandwidth per device.

### Controls during a call

| Button | Action |
|---|---|
| 🎤 / 🔇 | Mute / unmute microphone |
| 📹 / 🚫 | Enable / disable camera |
| 🖥 | Share screen (select window or monitor) |
| ⏺ | Start recording (saved locally as .webm) |
| 📝 | Live transcription (Whisper sidecar) |
| 🌐 | Live translation (BonsAI translates transcript) |
| 📋 | Summarise (generate meeting notes after call) |

---

## Shared Terminal

Click **🖥 Terminal** in the session panel. One participant's terminal is mirrored to all others.

- **All participants see the output** in real time (broadcast as `TerminalEvent::Output`).
- **Write permission** – only participants with write permission can send input. The host can grant/revoke this per-participant.
- **Permission badge** – a coloured lock icon shows who can type.

Useful for: pair debugging, live code demos, guided tutorials.

---

## Permissions

Each participant has a permission set that the **host** (session owner) controls:

| Permission | Effect |
|---|---|
| `CanEdit` | Allowed to edit files in the shared workspace |
| `CanSpeak` | Allowed to speak in voice calls |
| `CanVideo` | Allowed to share video |
| `CanExecute` | Allowed to type in the shared terminal |
| `IsAdmin` | Can change other participants' permissions |

To change permissions:
1. Right-click a participant in the session panel.
2. Select **Edit Permissions**.
3. Toggle the desired capabilities.
4. Click **Apply**. Changes take effect immediately.

---

## How TransferDaemon Powers Every Feature

| Feature | Stream type | Lane |
|---|---|---|
| Group chat messages | `Chat` | WebRTC data channel |
| CRDT file tree sync | `CrdtSync` | WebRTC data channel |
| Collaborative editing ops | `CrdtSync` | WebRTC data channel |
| Cursor presence | `Presence` | WebRTC data channel |
| Voice call | `Media` | WebRTC media track |
| Video call | `Media` | WebRTC media track |
| Screen share | `Media` | WebRTC media track |
| Terminal output | `Terminal` | WebRTC data channel |
| File transfer | `FileTransfer` | libp2p / QUIC |
| Knowledge module fetch | `KnowledgeFetch` | libp2p / QUIC |
| Distributed task dispatch | `TaskDistribute` | libp2p / QUIC |
| Health alerts | `Health` | Any available |

---

## Ending a Session

- **Host closes session** – all participants are notified and disconnected. Session data is cleared.
- **Participant leaves** – click **Leave Session**. Others remain connected.
- **Automatic timeout** – if all participants are disconnected for 10 minutes, the session is automatically closed.

---

*← [Knowledge Database](06-KNOWLEDGE-DATABASE.md) · [Compute Fabric →](08-COMPUTE-FABRIC.md)*
