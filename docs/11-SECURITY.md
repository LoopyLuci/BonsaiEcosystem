# Security & Privacy

Bonsai is designed with security and privacy as first-class concerns, not afterthoughts. This document explains every security mechanism in the ecosystem.

---

## Local-First: No Data Leaves Without Your Consent

**Nothing is sent to any external server unless you explicitly initiate it.** There is no:
- Telemetry or crash reporting
- Cloud model API calls
- Usage analytics
- Centralised identity database

Every model inference, training step, and file operation happens on your hardware. The only network activity is what you choose to do: connect to a peer, download a model, or use the relay for NAT traversal.

You can verify this with a network monitor. On a typical day of heavy Bonsai use, with no peers connected, you will see zero outbound connections.

---

## Encryption at Rest

### Identity Keys
Your Ed25519 keypair is stored in `~/.bonsai/keys/identity.encrypted`. The file is encrypted with AES-256-GCM using a key derived from your passphrase via Argon2id (memory: 64 MB, iterations: 3, parallelism: 4). The passphrase never leaves your machine.

### Training Data and Configuration
All sensitive configuration (API keys you choose to add, model weights, training data) lives in `EncryptedStore` — a custom encrypted key-value store using the same AES-256-GCM + Argon2id scheme.

### Workspace Files
Your code and documents are stored unencrypted by default (for compatibility with other tools). You can enable full-disk or folder-level encryption at the OS level (BitLocker, FileVault, fscrypt) independently.

---

## Encryption in Transit

All peer-to-peer communication uses the **Noise_XX protocol** for session establishment:

```
Initiator                    Responder
    │                             │
    │──── Noise_XX handshake ────►│
    │  (Ed25519 identity, X25519  │
    │   ephemeral, BLAKE3 proof)  │
    │◄────────────────────────────│
    │                             │
    │     Encrypted channel       │
    │ (AES-256-GCM, per-message   │
    │  nonce, 128-bit auth tag)   │
```

1. Both parties verify each other's Ed25519 identity.
2. An ephemeral X25519 keypair generates the session secret (Diffie-Hellman).
3. The session secret is expanded via HKDF into encryption and authentication keys.
4. Each message is encrypted with AES-256-GCM and a monotonically increasing nonce — replay attacks are impossible.

### WebRTC Media
Audio and video calls use DTLS-SRTP, the standard WebRTC encryption. Encryption keys are negotiated in the browser's secure context, not exposed to application code.

---

## Capability-Based Security (`TrustGuard`)

Every operation in Bonsai is an **effect** that must be declared and granted:

```
Effect                What it allows
──────────────────────────────────────────────
FileRead(path)        Read a specific file or directory
FileWrite(path)       Write to a specific file or directory
NetworkConnect(url)   Open a connection to a URL
Execute(cmd)          Run a shell command
GPU                   Use GPU compute
SendMessage(peer)     Send a message to a specific peer
```

When BonsAI proposes a tool call, the capability system checks whether the conversation has been granted that capability. For high-risk effects (FileWrite, Execute), the **Plan Review Gate** prompts you to approve before execution.

### Capability Scoping
You can restrict BonsAI's capabilities per session or project:

```yaml
# BONSAI.md capability overrides
capabilities:
  FileWrite: deny        # BonsAI can never write files in this project
  NetworkConnect: allow  # BonsAI can fetch URLs
  Execute: prompt        # Always ask before running commands
```

---

## Sandboxing: Three-Tier Code Isolation

Any code executed by BonsAI runs in an isolated environment:

| Tier | Technology | Escape risk | Performance |
|---|---|---|---|
| **WASM** | wasmtime or wasmer | Near zero | ~5% overhead |
| **Container** | gVisor (runsc) | Very low | ~10% overhead |
| **Native + TEE** | SGX / TrustZone | None (attestation) | Native speed |

The sandbox tier is selected automatically based on the task. User code is always sandboxed; only signed Bonsai binaries run natively.

---

## Undercover Mode

When Undercover Mode is enabled (`Settings → Privacy → Undercover Mode`):

- **System prompt**: all references to "Bonsai", "BonsAI", and specific model names are removed.
- **Git commits**: the `Co-Authored-By: Claude` attribution line is stripped.
- **Activity log**: events are logged with generic names ("AI assistant" instead of "BonsAI").
- **Screen captures**: the app title bar shows "Code Editor" instead of "Bonsai Workspace".

A 🕵 icon appears in the status bar when active.

---

## Threat Model

### Trust Boundaries

| Boundary | Risk | Mitigation |
|---|---|---|
| Webview (JavaScript) ↔ Tauri IPC | Prompt injection, malicious web content | Input validation, allowlist of IPC commands |
| LLM output → tool executor | Prompt injection, malicious commands | Plan Review Gate, capability checks |
| Sidecar process (llama-server) | Malicious model output | Output parsing, no shell eval |
| Peer connection | Malicious peer, MITM | Ed25519 identity verification, noise protocol |
| `.bkp` package import | Malicious package, code injection | BLAKE3 integrity check, manifest validation |

### Known Risks
1. **Prompt injection via file contents**: if BonsAI reads a file containing adversarial instructions, it might follow them. Mitigation: file contents are quoted in the context; the system prompt explicitly warns against instruction override.
2. **Malicious knowledge modules**: a `.kmod` with carefully crafted vectors could bias retrieval toward harmful outputs. Mitigation: module provenance is tracked; community modules will require code review before the registry lists them.
3. **Relay trust**: the `bonsai-relay` server can see message metadata (sender, recipient, size) but not content (end-to-end encrypted). Run your own relay if you require full metadata privacy.

---

## Audit Logging

Every security-relevant event is recorded in a **tamper-evident Write-Ahead Log (WAL)**:

- File read/write by BonsAI tools
- Tool calls approved or rejected
- Peer connections established or refused
- Identity operations (create, load, export)
- Capability grants and denials
- Training data imports
- Package imports

Each entry is:
1. Serialised to JSON
2. BLAKE3-hashed with the previous entry's hash (chain)
3. Appended to `~/.bonsai/audit.wal`

On startup, the WAL chain is verified. Any tampering breaks the chain and triggers a security alert. You can export the WAL to JSON for external auditing.

---

## Dependency Auditing

On every build (and in CI):

```bash
cargo audit          # checks for known Rust vulnerabilities (RustSec advisory DB)
npm audit            # checks for npm vulnerabilities
```

Critical and high severity vulnerabilities cause the build to fail. Advisories are resolved within 48 hours of publication.

### Secret Scanning
Pre-commit hooks run `detect-secrets` to prevent accidentally committing API keys, passwords, or private keys. CI enforces this for all branches.

---

## Vulnerability Reporting

If you find a security vulnerability, please report it responsibly:

1. **Do not** open a public GitHub issue.
2. Email: see `SECURITY.md` in the repository root.
3. Include: description, reproduction steps, impact assessment.
4. We will acknowledge within 48 hours and aim to patch within 14 days.

We do not have a formal bug bounty programme yet, but responsible disclosures are credited in the changelog.

---

*← [Sovereignty](10-SOVEREIGNTY.md) · [Developer Guide →](12-DEVELOPER.md)*
