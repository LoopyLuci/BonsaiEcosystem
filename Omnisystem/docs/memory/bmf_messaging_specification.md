---
name: bmf_messaging_system
description: "Bonsai Messaging Fabric specification — sovereign, encrypted email/SMS with AI filtering and P2P federation"
metadata: 
  node_type: memory
  type: reference
  originSessionId: c7ae2a7a-5206-469e-8d6b-97fc5255ee90
---

## Bonsai Messaging Fabric (BMF) — Complete Specification

**Purpose:** Replace traditional email/SMS infrastructure (Postfix, Dovecot, Twilio) with a sovereign, self-hosted, AI-augmented messaging platform that stays within the Bonsai mesh.

### Core Principles

- **Sovereignty** — No reliance on Gmail, Outlook, or cloud SMS gateways. All data on your hardware.
- **Privacy by default** — End-to-end encryption (OpenPGP), no plaintext logs
- **AI-driven** — BonsAI V2 for spam filtering, phishing detection, auto-replies, translation
- **Federated** — Echo-based P2P federation with other BMF instances
- **Resilient** — Survival System ensures 99.99% availability
- **Observable** — Universe events for every message (subject, sender, recipients, timestamp)

### Components

1. **Protocol Gateways**
   - SMTP server (ingress/egress, RFC-compliant)
   - IMAP/POP3 servers (client sync)
   - SMS/MMS gateway (SMPP 3.4, REST API)

2. **Unified Message Queue (SMQ)**
   - Persistent storage in CAS (content-addressed)
   - Deduplication via BLAKE3 hashing
   - Priority queues (urgent, normal, bulk)
   - Exponential backoff retry logic

3. **Routing & Filtering**
   - Spam detection (BonsAI V2, >90% accuracy)
   - Phishing detection (URL scanning, DKIM/DMARC/SPF)
   - Content filtering (profanity, PII redaction)
   - Greylisting and rate limiting

4. **Storage Backend**
   - Raw messages in CAS
   - Metadata in KDB (sender, recipients, date, subject, tags)
   - Full-text search via Tantivy

### AI Features

- Automatic replies (suggest templates)
- Priority sorting (Urgent, Important, Newsletter, Spam)
- Language translation
- Summarisation of long threads
- Anomaly detection (unusual sending patterns)
- Continuous training via EternalTrainingLoop

### Performance Targets

- SMTP throughput: 10,000 messages/second
- IMAP sync latency: <200ms for 1,000 messages
- Spam detection: <10ms/msg (GPU) / <50ms (CPU)
- Availability: 99.99% (Survival System)

### Integration with Bonsai Ecosystem

- **USOS** — Capability tokens for email/SMS access
- **BonsAI V2** — Spam detection, auto-reply, translation
- **Universe** — Log every message event (without body content)
- **Echo** — P2P federation with other BMF instances
- **TransferDaemon** — Encrypted P2P message delivery (optional, fallback to SMTP)
- **OmniBot** — Send/receive SMS via Discord/Telegram

### Implementation Roadmap

| Phase | Focus |
|-------|-------|
| 1 | Core SMTP server, message storage in CAS |
| 2 | IMAP server, Bonsai Workspace integration |
| 3 | Spam filtering (BonsAI V2) |
| 4 | SMS/MMS gateway (SMPP) |
| 5 | P2P federation (Echo + TransferDaemon) |
| 6 | Security hardening (DKIM, DMARC, SPF, TLS) |
| 7 | AI features (auto-reply, translation) |
| 8 | Production hardening |
