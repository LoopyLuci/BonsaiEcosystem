---
name: transfer_daemon_messaging
description: TransferDaemon integration for P2P email/SMS delivery with multi-path bonding and relay fallback
metadata: 
  node_type: memory
  type: reference
  originSessionId: c7ae2a7a-5206-469e-8d6b-97fc5255ee90
---

## TransferDaemon × BMF — P2P Messaging Mesh

**Purpose:** Augment traditional SMTP/SMS with direct, encrypted, peer-to-peer message delivery via TransferDaemon, enabling sub-100ms latency and full sovereignty.

### Key Features

1. **Content-Addressed P2P Delivery**
   - Messages identified by BLAKE3 content hash (not mutable names)
   - Direct streams between peers via TransferDaemon
   - Encrypted transport (Noise protocol, AES-256-GCM)

2. **Message Envelope Format**
   ```rust
   TransferMailEnvelope {
       sender: email/phone,
       recipient: email/phone,
       subject: Option<String>,
       body_hash: BLAKE3,
       body_ciphertext: Vec<u8>,        // encrypted with recipient's public key
       attachments: Vec<ContentHash>,   // stored in CAS
       delivery_mode: Reliable/FireAndForget/Urgent,
       signature: Vec<u8>,              // Ed25519 signed by sender
   }
   ```

3. **Echo-Based Service Discovery**
   - `bonsai:mail` service announcement for domains/users
   - SRV-like records for load balancing
   - Fallback to SMTP if no Echo record exists

4. **Multi-Path Bonding for Large Messages**
   - Split messages into 64 KiB chunks
   - Send round-robin across WiFi, 5G, Ethernet
   - Reassemble on receipt with sequence numbers
   - Automatic failover if a link drops

5. **Relay Fallback**
   - If recipient offline, store message at a relay node
   - Relay uses capability tokens for authorization
   - Store-and-forward until recipient reconnects
   - 7-day expiry by default

### Routing Decision Tree

```
1. Recipient email address received
2. Query Echo for `bonsai:mail` service
   ├─ Found → Resolve to peer ID
   │  └─ Attempt TransferDaemon delivery
   │     ├─ Success → Done
   │     └─ Fail (3 retries) → Fall back to SMTP
   └─ Not found → Use traditional SMTP
```

### Performance Targets

- P2P delivery latency (LAN): <100ms
- Multi-path overhead: <5%
- Artifact fetch latency (large .bco): <200ms (WAN)
- Concurrent connections: 10,000+ per node

### Security Model

- End-to-end encryption via Noise protocol
- Ed25519 signatures for authenticity
- Capability tokens for relay access control
- Universe audit trail (no plaintext content)

### Implementation Phases

| Phase | Task | Deliverable |
|-------|------|-------------|
| 2.5 | TransferDaemon adapter for BMF | `bmf-transfer` crate, envelope serialisation, Echo integration |
| 3.5 | Multi-path bonding | Chunking, reassembly, control stream |
| 4.5 | Relay fallback | Echo relay discovery, store-and-forward |

### Integration Summary

- **BMF** → Routes via TransferDaemon when Echo record exists, otherwise SMTP
- **TransferDaemon** → Provides encrypted, multi-path, P2P transport
- **Echo** → Service discovery for `bonsai:mail` endpoints
- **CAS** → Message body and attachment storage
- **Universe** → Event logging (without plaintext content)
