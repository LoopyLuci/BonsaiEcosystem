# Bonsai OS Core Blueprint

> A next-generation operating system designed around capability security, live-updatable kernel modules, distributed compute, and a unified agent-native developer experience.

## 1. Purpose and Philosophy

Bonsai OS is not a traditional operating system. It is a distributed, capability-first runtime that treats every machine as a trusted compute participant in a fabric of devices. Its core goals are:

- **Security by capability**: every resource is accessed through derivable, least-privilege capabilities.
- **Live evolution**: kernel and runtime components can be upgraded without a reboot.
- **Composable persistence**: storage is backed by CAS and versioned persistent memory.
- **Distributed fabric integration**: the OS is directly interoperable with the Bonsai Compute Fabric.
- **Developer-first experience**: the runtime exposes deterministic tooling, reproducible artifacts, and self-describing modules.

## 2. Architecture Overview

The Bonsai OS core is layered as follows:

1. **Kernel runtime** (`bonsai-kernel`)
   - Minimal privileged core: capability enforcement, process isolation, IPC, scheduler, storage guard.
2. **Capability manager**
   - Derives and delegates capabilities, enforces authorization, and audits every resource request.
3. **User-space actor runtime** (`bonsai-actor-runtime`)
   - Hosts secure actors, language runtime sandboxes, and UI service agents.
4. **CAS-native storage**
   - Unified artifact store for binaries, configuration, logs, and proof artifacts.
5. **Live update / upgrade fabric**
   - Hot-swap kernel modules and runtime actors using versioned CAS manifests.
6. **Distributed fabric bridge**
   - Native fabric scheduler integration for remote task migration, pooling, and cross-node capabilities.

## 3. Kernel Capabilities and Security Model

### 3.1 Capability Types

The OS defines first-class capabilities for:

- Filesystem access (`FsCap`)
- Network transport (`NetCap`, `TlsCap`, `WireGuardCap`)
- Process and actor creation (`ProcCap`)
- Persistent memory transactions (`PmemCap`)
- Device and GPU access (`DeviceCap`)
- Live upgrade and module swap (`UpgradeCap`)

Each capability is:

- cryptographically derived from a root authority
- bound to a specific resource class and policy
- revocable and auditable

### 3.2 Policy Enforcement

Every system call and host request is validated by the kernel's `TrustGuard`.

- Policies are expressed as immutable rule sets stored in CAS.
- Unsafe actions require an explicit capability derivation step.
- Unauthenticated or unapproved access is denied before execution.

### 3.3 Capability Delegation

Delegation is explicit and proof-based.

- A supervising actor may issue a sub-capability to a worker actor.
- The kernel validates the delegation chain and expiration metadata.
- Delegated capabilities carry provenance information for auditing.

## 4. Process Model and Isolation

Bonsai OS uses an actor-centric isolation model rather than a classic PID-based process tree.

### 4.1 Actors and Sandboxes

- Actors are the primary execution unit.
- Actors can run as:
  - WASM modules in `wasmtime`
  - containerized user-space tasks
  - native actors when explicitly trusted
- Each actor is bound to a capability set and a `SecurityDomain`.

### 4.2 Actor Lifecycle

Lifecycle events flow through `SystemEventBus`:

- `ActorSpawned`
- `ActorPaused`
- `ActorResumed`
- `ActorCrashed`
- `ActorUpgraded`

### 4.3 Resource Accounting

The kernel tracks resource budgets per actor:

- CPU quotas
- memory ceilings
- I/O budgets
- capability-call quotas

Budget exhaustion triggers graceful throttling or reclamation.

## 5. Memory and Storage

### 5.1 Memory Management

Bonsai OS supports a secure memory model with:

- zero-copy shared regions for actor IPC
- capability-limited view into physical or virtual memory
- memory serialization and checkpointing for live migration

### 5.2 Persistent Memory (DAX-style)

Persistent memory is exposed through transactional APIs:

- `sys_pmem_transaction_start`
- `sys_pmem_transaction_commit`
- `sys_pmem_transaction_abort`

Transactions are checkpointed in CAS and can be rolled back by the kernel.

### 5.3 CAS-backed Filesystem

The OS uses an overlay of CAS and a traditional filesystem:

- Immutable artifacts are stored by content key.
- Mutable state is represented as versioned manifests.
- User-space sees a unified namespace with transparent CAS caching.

### 5.4 AuroraFS

AuroraFS is the proposed high-level storage layer:

- snapshots stored as CAS trees
- file-level deduplication
- provenance metadata for every modification
- integration with archival and remote fabric storage

## 6. IPC and Messaging

### 6.1 Capability-aware IPC

IPC is message-based and capability-checked.

- Messages include sender identity and capability claims.
- The kernel validates each transfer across isolated domains.
- Zero-copy buffers can be shared only if both endpoints hold compatible memory capabilities.

### 6.2 Channels and Streams

Support for:

- synchronous RPC channels
- async event streams
- transfer streams for bulk data

Each channel is labeled with a security policy and resource budget.

## 7. Scheduler and Fabric Integration

### 7.1 Local Scheduler

The local scheduler supports:

- priority queues for interactive vs batch actors
- heterogenous core affinity
- preemption and work-stealing
- capability-based scheduling for security isolation

### 7.2 Fabric Scheduler Bridge

Bonsai OS exposes the local scheduler to the Compute Fabric:

- the coordinator may offload tasks to remote devices
- remote tasks receive derived capabilities for local resource access
- task migration preserves actor state and checkpointed memory

### 7.3 Predictive Scheduling

The engine learns from historical execution:

- execution time per task type
- device reliability and thermal stability
- network latency and bandwidth

This enables AI-driven placement decisions across nodes.

## 8. Networking and Secure Transport

### 8.1 User-space Networking

Network access is mediated by user-space network actors.

- raw packet access is privileged and capability-controlled
- sockets are created via `NetCap` and optional encryption capability
- network stacks can be isolated per actor

### 8.2 Encryption and Policy

The security layer enforces:

- mandatory encryption when required by policy
- authenticated public-key tunnels
- WireGuard-style capabilities for private mesh traffic

### 8.3 TransferDaemon and Fabric Control

The `TransferDaemon` provides:

- WebRTC control channels
- libp2p/QUIC transport for bulk data
- secure rendezvous and capability-aware pairing

## 9. UI and Developer Experience

### 9.1 UI Panels as Actors

UI panels are first-class actor services.

- UI components are generated, versioned, and hot-swappable.
- Panels run in sandboxed webviews or WASM containers.
- UI state is captured as time-travel events for rollback.

### 9.2 Developer APIs

The OS exposes developer-facing contracts:

- `bonsai.sys` host APIs for tools and capabilities
- `sylva_eval` for embedded script execution
- `compile_titan` and `sandbox_executor` for user-defined workloads

### 9.3 Observability

Built-in observability includes:

- capability audit logs
- event bus trace streams
- resource and health dashboards
- formal proof artifact records

## 10. Live Update and Upgrade Strategy

### 10.1 Hot-swap Modules

Kernel modules and runtime components are packaged as CAS artifacts.

- `UpgradeDispatcher` orchestrates safe replacement.
- In-flight actor state can be preserved or migrated.
- Rollback points are stored in CAS manifest history.

### 10.2 Atomic Upgrade Semantics

Upgrades must satisfy:

- no partial enablement of new trusted capabilities
- consistent capability delegation state
- failure-safe rollback on any assertion or health check failure

### 10.3 Upgrade Policies

Users and operators can choose:

- `manual`
- `auto_on_green_ci`
- `canary_24h`
- `shadow` (parallel validation without traffic)

## 11. Formal Verification and Certification

### 11.1 Axiom and Proof Artifacts

The OS will attach proof metadata to critical components:

- IPC isolation invariants
- capability derivation correctness
- scheduler safety properties

Proof artifacts are stored in CAS and surfaced in CI.

### 11.2 Certification Roadmap

Target certification areas:

- EAL7-style assurance
- DO-178C for safety-critical runtime components
- traceability matrices linking requirements → design → implementation → proof

## 12. Implementation Roadmap

### Phase 10 deliverables

- `crates/bonsai-kernel`: minimal capability kernel core
- `crates/bonsai-actor-runtime`: secure actor host
- `crates/bonsai-storage`: CAS-native AuroraFS layer
- `crates/bonsai-fabric-bridge`: scheduler integration with Compute Fabric
- `crates/bonsai-security`: capability derivation and TrustGuard enforcement
- `crates/bonsai-ci`: Axiom attachment and proof validation pipeline

### Success criteria

- simple kernel module boots and accepts authorized actor spawns
- capability policy rejects unauthorized file/socket access
- persistent memory transactions commit and rollback correctly
- live upgrade replaces a kernel module without reboot
- compute fabric schedules a remote task with derived capabilities
- proofs are generated, attached, and verified by CI

## 13. Glossary

- **CAS**: Content-Addressed Storage
- **WASM**: WebAssembly
- **Actor**: isolated execution unit with explicit capabilities
- **Capability**: fine-grained access token for resources
- **UpgradeDispatcher**: live update coordinator
- **TransferDaemon**: cross-device transport and control service

---

*This document is the canonical Bonsai OS Core Blueprint. Use it as the reference for Phase 10 implementation, architecture decisions, and certification planning.*
