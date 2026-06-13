# Glossary – Technical Terms in the Bonsai Ecosystem

A reference for all technical terms, acronyms, and concepts used throughout the Bonsai Ecosystem and UOSC.

---

## A

### Arbiter
A safety-clamped orchestrator that coordinates execution tiers in the `SovereignService` trait. The Arbiter decides which tier to invoke (AI → Heuristic → Deterministic Core → Safe Stub) based on confidence, latency, and consistency metrics. See [DETERMINISTIC_BACKBONE.md](DETERMINISTIC_BACKBONE.md).

### Axiom Proofs
Formal verification files (`.ax`) that mathematically prove properties of code (e.g., deadlock-freedom, memory safety). Located in `verified/`. Linked from rustdoc comments in source code.

---

## B

### BACE (Bonsai Atomic Compilation Engine)
Function-level incremental compilation with macro caching and AI hints. Reduces build times to <30 seconds for typical incremental changes. See [BUILD.md](BUILD.md).

### BCF (Bonsai Container Fabric)
OCI-compatible container system with no sidecars. Integrates hardware sandboxing and allows deterministic, air-gapped execution of untrusted code.

### BPLIS (Bonsai Polygon Language Integration System)
The component that generates code in 750+ target languages from a canonical Pong specification. Powered by LAIR (Language-Agnostic Intermediate Representation).

### BMF (Bonsai Messaging Framework)
A sovereign SMTP/IMAP server with post-quantum crypto and spam filtering. Powers email and messaging without cloud dependencies.

### BMN (Media Nexus)
Real-time streaming platform with optional AI enhancement for frame interpolation and quality optimization. Falls back to deterministic streaming when AI is disabled.

### BonsAI V2
The local LLM implementation in the Bonsai Ecosystem. Features deterministic tool calls, fine-tuning via DPO, and RAG integration. Operates with or without AI (optionally deterministic-only).

### BuceClient
The API for interacting with BUCE (Bonsai Universal Compression Engine). Provides streaming compression/decompression with hardware acceleration.

### BUCE (Bonsai Universal Compression Engine)
Content-aware, hardware-accelerated compression with automatic deduplication. Achieves 5–40× compression ratios depending on content type. See [COMPRESSION.md](COMPRESSION.md).

---

## C

### Capability Token
A token issued by the `SovereignService` trait that grants permission to perform specific operations (e.g., file I/O, network access). Part of the security model.

### Changelog
A structured log of all releases, features, and breaking changes. Maintained in [CHANGELOG.md](CHANGELOG.md) using semantic versioning.

---

## D

### DCUtR (Distributed Connectivity Update Request)
A protocol extension in TransferDaemon that allows NAT traversal and hole punching for P2P connections. Part of the Echo fabric.

### Deterministic Core
The portion of any system that produces the same output given identical inputs, without reliance on AI, randomness, or external state. Required for all critical systems in the Bonsai Ecosystem.

### DPO (Direct Preference Optimization)
A fine-tuning technique used to teach BonsAI V2 to follow preferences without RLHF. Used for safety, tool use, and domain adaptation.

---

## E

### Echo Fabric
The P2P mesh network that powers TransferDaemon. Routes messages via multiple paths and automatically selects the best link based on latency and bandwidth.

---

## F

### Feature Gate / Feature Flag
A Rust `#[cfg(feature = "...")]` attribute that conditionally compiles code. Used to enable/disable AI enhancements, optional protocols, and experimental features.

### Fidelity Score
A metric (0.0–1.0) measuring how closely a target-language implementation matches the canonical Pong specification. Calculated by comparing execution traces.

### Fixed-Point Arithmetic (16.16)
A deterministic numeric format used in Polyglot Pong to ensure bit-identical execution across all languages. Represents numbers as signed 32-bit integers: 16 bits for integer part, 16 for fractional.

---

## G

### Governance
The decision-making process for the Bonsai Ecosystem, managed by the Bonsai Council. See [GOVERNANCE.md](GOVERNANCE.md).

### Graceful Degradation
A principle where systems continue to function when higher-tier features fail. In SovereignService: if AI fails, fall back to heuristics; if heuristics fail, use deterministic core; if that fails, use safe stub.

---

## H

### Heuristic Tier
The second execution tier in SovereignService. Uses rule-based algorithms (no AI, no randomness) to make decisions. Faster than AI but less optimal than deterministic core for some workloads.

---

## I

### Identity (Self-Certifying)
In TransferDaemon, a public key that also serves as the peer's unique identifier. No central CA required; cryptographic properties ensure uniqueness.

---

## J

### Job (in Polyglot Pong)
A single test case: one source language + target language + random seed. The orchestrator manages a 750×750 matrix of jobs.

---

## K

### KDB (Knowledge Database)
A vector database + RAG system that stores facts, patterns, and code knowledge. Decouples factual information from the base model, enabling quick updates without retraining.

---

## L

### LAIR (Language-Agnostic Intermediate Representation)
An abstraction layer that allows conversion between 750+ programming languages. Powered by BPLIS.

---

## M

### Manifest
A YAML or JSON file listing all languages and their metadata (family, version, compilation flags, etc.). Used by the Polyglot Pong orchestrator to parameterize test runs.

### Multi-Path Bonding
A technique in TransferDaemon where multiple links (WiFi, Bluetooth, LTE, etc.) are simultaneously used to route P2P messages, selecting the fastest available path.

---

## N

### NAT Traversal
Techniques (UPnP, hole punching, relay fallback) that allow P2P connections through network address translators (firewalls, routers). Built into TransferDaemon via DCUtR.

### Nexus Core
An optional blockchain component providing quad-token economics, governance voting, and distributed consensus. Can be disabled for offline use.

---

## O

### OmniBot
A unified control plane for Bonsai across Discord, Telegram, Matrix, and Email. Enables multi-agent swarm orchestration via messaging platforms.

---

## P

### Polyglot Pong
A deterministic distributed game framework used to validate 750+ languages against a canonical specification. Serves as both a test suite and a demonstration of deterministic-first architecture.

### Post-Quantum Cryptography
Algorithms (X25519 + ML-KEM-768 hybrid, SPHINCS+ signatures) that remain secure even against hypothetical quantum computers. Standard in TransferDaemon and all messaging.

---

## R

### RAG (Retrieval-Augmented Generation)
A technique where relevant documents or facts are retrieved and injected into an LLM's context before generating a response. Used in BonsAI V2 + KDB.

### RLHF (Reinforcement Learning from Human Feedback)
A training technique for LLMs. Bonsai prefers DPO as a more efficient alternative.

---

## S

### Safe Stub
The fourth (fallback) execution tier in SovereignService. Returns an empty or default result when all other tiers fail. Guarantees the system never crashes.

### Sanctum Vaults
Hardware-backed secure execution enclaves (Intel SGX, ARM TrustZone) used for high-assurance operations. Integrated into BCF containers.

### SovereignService
A core trait that separates deterministic core from optional AI enhancements. Implements graceful degradation via four tiers: AI → Heuristic → Deterministic Core → Safe Stub.

---

## T

### TEE (Trusted Execution Environment)
A secure, isolated processor or memory region that can execute code with hardware-enforced confidentiality and integrity. Used in Sanctum Vaults.

### TLA+ (Temporal Logic of Actions)
A formal specification language used to model concurrent systems and prove properties like liveness and safety. Bonsai uses TLA+ for protocol verification.

### Trace (Execution Trace)
A record of all state transitions during execution of a program. Polyglot Pong compares traces to measure fidelity and detect divergences.

### TransferDaemon
A peer-to-peer networking daemon with self-certifying identities, multi-path bonding, post-quantum crypto, and NAT traversal. The foundation of all distributed features in Bonsai.

---

## U

### UOSC (Unified Secure Operating System)
The long-term vision of the Bonsai project: a complete, sovereign operating system built from auditable, open-source components with zero external dependencies.

### Universe Logs
A distributed logging system enabling time-travel debugging across all nodes. Captures all system events with deterministic ordering.

---

## V

### Verified (Axiom Proofs & TLA+)
Code or protocols that have been formally proven correct. Located in `verified/` folder; linked from rustdoc comments.

---

## W

### Workspace (Rust)
A collection of interdependent Cargo crates managed by a single `Cargo.toml` at the root. The Bonsai Ecosystem is a single Rust workspace with 30+ crates.

---

## Z

### ZK-STARK (Zero-Knowledge Scalable Transparent Arguments of Knowledge)
A cryptographic proof system that allows proving computation without revealing inputs. Optional enhancement for privacy-critical operations.

---

## Acronyms Quick Reference

| Acronym | Expansion |
|---------|-----------|
| BACE | Bonsai Atomic Compilation Engine |
| BCF | Bonsai Container Fabric |
| BMF | Bonsai Messaging Framework |
| BMN | Media Nexus (Bonsai Media ...) |
| BPLIS | Bonsai Polygon Language Integration System |
| BUCE | Bonsai Universal Compression Engine |
| DCUtR | Distributed Connectivity Update Request |
| DPO | Direct Preference Optimization |
| KDB | Knowledge Database |
| LAIR | Language-Agnostic Intermediate Representation |
| NAT | Network Address Translation |
| RLHF | Reinforcement Learning from Human Feedback |
| RAG | Retrieval-Augmented Generation |
| TLA+ | Temporal Logic of Actions |
| TEE | Trusted Execution Environment |
| UOSC | Unified Secure Operating System |
| ZK-STARK | Zero-Knowledge Scalable Transparent Arguments of Knowledge |

---

**Last Updated**: 2026-06-04  
**Auto-generated section**: Terms are indexed for quick lookup. Add new terms by documenting them in code with `/// Documentation` or updating this file.
