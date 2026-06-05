# Omnisystem Architecture

## Overview

The Omnisystem is a complete rebuilding of the Bonsai Ecosystem using four Omni-languages:

1. **Titan**: Systems language (linear types, effects, self-hosting)
2. **Sylva**: Scripting language (dynamic + gradual static, JIT)
3. **Aether**: Actor language (location-transparent, CRDT-native, supervision)
4. **Axiom**: Proof language (dependent types, code extraction)

All code in the Omnisystem follows strict naming conventions and architectural patterns to ensure consistency, verifiability, and maintainability.

---

## Naming Conventions

### Crate/Service Names
- **Pattern**: `omni-{service-name}`
- **Examples**: `omni-p2p`, `omni-compress`, `omni-ai`, `omni-observability`
- **All lowercase, hyphenated**

### Module Names
- **Pattern**: `omni::{service_name}` (snake_case)
- **Examples**: `omni::p2p`, `omni::compress`, `omni::container`

### USOS Kernel Structures
- **Pattern**: `usos_{component}_{subcomponent}`
- **Examples**: 
  - `usos_process`
  - `usos_memory_region`
  - `usos_capability_token`
  - `usos_ipc_channel`
  - `usos_scheduler`

### Configuration Files
- **Pattern**: `omni.toml` at workspace root
- **Language-specific**: `titan.toml`, `sylva.toml`, `aether.toml`

### Build Artifacts
- **Pattern**: `omni-{name}.{ext}`
- **Examples**: `omni-kernel.bin`, `omni-system.img`, `omni-services.tar`

---

## The Four Omni-Languages

### 1. Titan (Systems Language)

**Purpose**: Core system components, performance-critical code, standard libraries

**Key Features**:
- Linear types with ownership system (all resources: memory, file handles, sockets, GPU buffers)
- Algebraic effect system (`io`, `alloc`, `net`, `fail`, `async`)
- Self-hosting compiler
- Multiple compilation targets:
  - Native (x86-64, ARM64, RISC-V via Cranelift)
  - WebAssembly (WASI)
  - GPU (CUDA, ROCm, SPIR-V)
- Deterministic execution (no garbage collector; all cleanup is compile-time)

**Primary Uses**:
- USOS kernel
- omni-p2p (crypto core)
- omni-compress
- omni-container
- omni-compiler (core)
- omni-observability
- omni-ai (core inference)
- omni-qa (static analysis)
- Standard library

**Standard Library Structure**:
```
titan::core              # No-std: mem, ptr, sync primitives
titan::io                # File, socket, console I/O
titan::sync              # Mutex, RwLock, Condvar, Atomic
titan::fs                # Filesystem abstraction
titan::net               # TCP, UDP, Domain sockets
titan::collections       # Vec, HashMap, BTreeMap, etc.
titan::thread            # Threading primitives
titan::simd              # SIMD operations
titan::alloc             # Memory allocator interface
```

---

### 2. Sylva (Scripting Language)

**Purpose**: Rapid development, dynamic queries, interactive tools

**Key Features**:
- Dynamic typing with optional gradual static typing (`strict` mode)
- JIT compilation to native code
- Time-travel debugging (state snapshots, rewind execution)
- REPL with advanced features (inline docs, auto-complete)
- Interop with Titan libraries (automatic type marshaling)
- JSON/YAML/TOML handling built-in

**Primary Uses**:
- omni-bot scripting
- omni-compiler frontend
- omni-knowledge query language
- Configuration and scripting

**Key Bindings**:
```
sylva::titan              # Marshal between Sylva dynamic and Titan static
sylva::http               # HTTP client/server
sylva::json               # JSON parsing/generation
sylva::async              # Async/await backed by Aether
sylva::repl               # Interactive development
```

---

### 3. Aether (Actor Language)

**Purpose**: Distributed concurrency, message passing, large-scale coordination

**Key Features**:
- Location-transparent actors (local or remote via omni-p2p)
- CRDT-native state (GCounter, PNCounter, ORSet, LWWRegister)
- Supervision trees (Erlang-style with restart strategies)
- Distributed persistence (to omni-observability)
- Automatic message serialization
- Consistent hashing for actor placement (scales to 10,000+ nodes)

**Primary Uses**:
- omni-p2p (P2P mesh)
- omni-media (streaming actors)
- omni-ai (distributed inference)
- omni-observability (log collection)
- Any distributed service

---

### 4. Axiom (Proof Language)

**Purpose**: Formal verification of critical properties, code extraction

**Key Features**:
- Full dependent type system (like Lean 4)
- Inductive types, pattern matching, tactic automation
- Code extraction to Titan and Sylva
- Small kernel (~2000 lines) for trustworthiness
- LSP server with live proof checking
- Integrated into UBVM

**Primary Uses**:
- USOS kernel verification (memory safety, scheduler correctness)
- Protocol verification (omni-p2p handshake, etc.)
- Data structure proofs (no overflow, correct invariants)
- Constraint verification (omni-ai safety properties)

---

## USOS Core Architecture

The USOS kernel is minimal (< 5000 lines of Titan), providing only essential primitives.

### Kernel Components

1. **Memory Manager** (`usos_memory`)
   - Physical memory allocation (frame allocator)
   - Virtual memory with paging
   - Copy-on-write
   - Capability-based regions

2. **Scheduler** (`usos_scheduler`)
   - Preemptive multitasking
   - Priority queues
   - EDF (Earliest Deadline First) for real-time
   - Deterministic scheduling order

3. **IPC** (`usos_ipc`)
   - Synchronous message passing (rendezvous)
   - Asynchronous message passing (buffered)
   - Both use capabilities for authorization

4. **Capabilities** (`usos_capability`)
   - Unforgeable tokens for all resources
   - Fine-grained: separate caps for read/write/execute
   - Revocable and delegable

5. **Boot & Service Manager** (`usos_boot`)
   - Load initial userspace image from CAS
   - Start omni-service-manager
   - Initialize logging and audit

### What Is NOT in USOS Core

- Filesystems (userspace service)
- Networking stack (userspace omni-p2p)
- Device drivers (userspace services)
- GUI / windowing
- Compression, encryption
- Database, blockchain
- Language runtimes

---

## Service Architecture

### Core Service Tiers

**Tier 1: Essential Infrastructure**
- omni-p2p (networking)
- omni-observability (logging/telemetry)
- omni-container (process isolation)
- omni-compress (compression library)

**Tier 2: System Services**
- omni-vfs (filesystem)
- omni-enclave (runtime management)
- omni-compiler (compilation service)

**Tier 3: Business Logic**
- omni-ai (inference)
- omni-knowledge (semantic search)
- omni-media (multimedia)
- omni-bot (chat bridges)
- omni-qa (quality assurance)

**Tier 4: Optional**
- omni-blockchain (Nexus replacement)

---

## Build System (omni command)

The `omni` binary (written in Titan) provides:

```
omni build [target]      # Build specific service or entire system
omni test [suite]        # Run UBVM tests
omni run [service]       # Run service or USOS kernel
omni package [name]      # Create deployment package
omni repl [lang]         # Start REPL (Sylva/Titan)
omni verify [component]  # Run Axiom proofs
omni clean               # Remove build artifacts
omni status              # Show build status
```

---

## Design Principles

1. **Minimalism**: Only essential functionality in the kernel; everything else is userspace
2. **Capability-based**: All access control via unforgeable capabilities
3. **Determinism**: All I/O and randomness is tracked and reproducible
4. **Formal Verification**: Critical code has Axiom proofs
5. **Self-Hosting**: The system builds itself
6. **Content-Addressed**: All dependencies identified by BLAKE3 hash
7. **Distributed by Default**: Aether actors can span 10,000+ nodes
8. **Interop**: Seamless communication across Omni-languages via effects and IPC

---

---

## Implementation Notes

The USOS kernel (`kernel/capability.ti`, `kernel/memory.ti`, `kernel/scheduler.ti`)
and all ten platform services (`services/omni-*/`) are fully implemented in Titan and
pass their test suites. The build tool is `titan-bootstrap/output/titan-compiler.exe`.

**Document Version**: 1.1  
**Last Updated**: 2026-06-04
