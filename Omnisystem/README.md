# Omnisystem

A sovereign operating system and service ecosystem written exclusively in the four
Omni-languages: **Titan**, **Sylva**, **Aether**, and **Axiom**.
Zero Rust. Zero external toolchain. 28 tests passing.

---

## Quick Start

```powershell
# Compile any Titan source file
.\titan-bootstrap\output\titan-compiler.exe kernel\capability.ti

# Run the full test suite
.\scripts\test-all.ps1

# Or via GNU make
make test
```

---

## Languages

| Language | Role | Extension |
|----------|------|-----------|
| **Titan** | Systems language — static types, algebraic effects, capability-based I/O | `.ti` |
| **Sylva** | Scripting layer — gradual typing, bytecode VM, tracing JIT | `.ti` |
| **Aether** | Actor concurrency — location-transparent actors, CRDTs, cluster mesh | `.ti` |
| **Axiom** | Formal verification — dependent types, proof extraction | `.ti` |

All source files use `.ti` and are compiled by `titan-compiler.exe`.

---

## Repository Layout

```
Omnisystem/
├── kernel/                 USOS microkernel — pure Titan, no external deps
│   ├── capability.ti       Capability system (8 Axiom-proved invariants)
│   ├── memory.ti           Physical allocator + virtual memory
│   └── scheduler.ti        EDF real-time + CFS scheduler with preemption
│
├── services/               Platform services — one Titan process each
│   ├── p2p/                Peer-to-peer transport
│   ├── compress/           Deterministic compression
│   ├── container/          Process sandboxing (capability-based)
│   ├── observability/      Distributed tracing and metrics
│   ├── storage/            Content-addressed object store
│   ├── cache/              TTL-based distributed cache
│   ├── queue/              Priority message queue
│   ├── rpc/                RPC framework
│   ├── auth/               Authentication and authorization
│   └── crypto/             Cryptographic key management
│
├── titan/                  Titan toolchain
│   ├── compiler/           Self-hosting compiler pipeline + verification
│   ├── std/                Standard library (effects, vec, io, hash, …)
│   └── axlib/              Axiom proofs (ax1–ax7)
│
├── sylva/                  Sylva language runtime
│   ├── interpreter.ti      Stack-based Omni-IR VM
│   ├── compiler.ti         AST → Omni-IR bytecode compiler
│   └── jit.ti              Tracing JIT with constant-folding optimiser
│
├── aether/                 Aether actor runtime
│   ├── actor.ti            Actor system: mailboxes, supervision, work stealing
│   ├── crdt.ti             GCounter, PNCounter, LWWRegister, ORSet
│   ├── crdt_map.ti         CRDTMap, CRDTGraph (add-wins)
│   ├── mesh.ti             Cluster mesh: URI routing, consistent hash, load balancing
│   └── protocol.ti         Session type protocol definitions
│
├── effect/                 Unified effect system
│   └── perform.ti          Effect algebra, handler stack, perform dispatch
│
├── uvm/                    Universal Validation Mesh
│   ├── scheduler.ti        Job scheduler: priority queue, agent dispatch
│   └── agent.ti            Test agent: suites, chaos injection, fidelity checks
│
├── build/                  Build infrastructure (pure Titan)
│   └── ir/ir.ti            Omni-IR specification: opcodes, module format, CAS hash
│
├── titan-bootstrap/        Bootstrap compiler (sole external binary)
│   └── output/
│       └── titan-compiler.exe  Native x86-64 seed binary
│
├── Makefile                GNU make targets: all, kernel, services, proofs, lang, test, clean
├── scripts/test-all.ps1    PowerShell test runner
├── STATUS.md               Component table and gap analysis
├── CHANGELOG.md            Full history
└── README.md               This file
```

---

## Building

The only required tool is `titan-compiler.exe`:

```powershell
# Single file
.\titan-bootstrap\output\titan-compiler.exe <file>.ti

# All kernel files
make kernel

# All 28 tests
make test
```

---

## Test Results

```
28 / 28 passing   (0 failures)
0 Rust files      (zero .rs, zero Cargo.toml)
```

| Category | Count | Status |
|----------|-------|--------|
| USOS Kernel | 3 | ✅ |
| Platform Services | 10 | ✅ |
| Axiom Proofs (kernel + services) | 2 | ✅ (17 theorems) |
| Aether (actors, CRDTs, mesh) | 4 | ✅ |
| Sylva (interpreter, compiler, JIT) | 3 | ✅ |
| Effect system | 2 | ✅ |
| Build infrastructure | 1 | ✅ |
| Compiler verification | 1 | ✅ |
| Universal Validation Mesh | 2 | ✅ |

---

## Design Principles

**Capability-based security.** Every resource access requires a linear, unforgeable
capability token issued by the kernel. No ambient authority. No privilege escalation.

**Algebraic effects.** All I/O, allocation, networking, and randomness are declared
as effect rows in function signatures. The `effect/perform.ti` module provides the
unified dispatch point. Handlers are swappable — Real, Sandbox, Mock, Log — enabling
deterministic testing without modifying application code.

**Content-addressed builds.** All artifacts are identified by a BLAKE3 hash of their
inputs. Builds are deterministic and reproducible across machines and time.

**Actor concurrency with CRDTs.** Aether actors hold state as CRDTs (GCounter,
PNCounter, LWWRegister, ORSet, CRDTMap, CRDTGraph). Concurrent updates converge
automatically — no manual conflict resolution, no coordination protocol needed.

**Formal verification.** Eight kernel safety theorems and nine service correctness
theorems are mechanised in Axiom and compiled into the build. Any violation breaks
the build.

**Universal Validation Mesh.** The UVM scheduler distributes test, proof, fuzz, and
chaos jobs across agents. Fidelity checks verify determinism across repeated runs.

---

## Bootstrap Invariant

`titan-bootstrap/output/titan-compiler.exe` is the sole seed binary. It is a
Windows x86-64 native executable. Once the Titan compiler achieves full self-hosting
on USOS (bare metal), this binary will be replaced by one produced by the
self-hosted compiler — closing the bootstrap chain permanently.
See `titan/compiler/self_host_verify.ti` for the formal proof that this is safe.
