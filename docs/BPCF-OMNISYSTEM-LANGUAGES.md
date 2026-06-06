# 🧬 BPCF – Omnisystem Languages: Titan, Aether, Sylva, Axiom

**Version:** 1.0  
**Status:** Production Specification  

This document provides deep-dive designs for integrating the four Omnisystem languages – **Titan**, **Aether**, **Sylva**, and **Axiom** – into the Bonsai Polyglot Compilation Fabric (BPCF). These languages are first-class citizens, built from the ground up with hot-reloadable, atomic, and verifiable compilation.

---

## 1. Titan – Effect-Tracking Systems Language

Titan is a systems language similar to Rust but with explicit **effect tracking** (e.g., `async`, `unsafe`, `io`, `alloc`). It is used for low-level components (kernel, drivers, real-time systems).

### 1.1 Function-Level Incremental Compilation

- **Dependency tracking** – Titan functions have an effect signature; a change in effect requires re-compiling all callers.
- **Compilation units** – Each function is compiled independently to a `.tco` (Titan Compiled Object) file.
- **CAS key** – Includes effect signature hash.

### 1.2 Hot-Reloading for Titan

- **Effect enforcement at runtime** – The runtime checks that the new function's effect signature matches the old one. If not, the hot-reload transaction is aborted.
- **No unsafe hot-reload** – Functions marked `unsafe` are never hot-reloaded (to prevent memory unsafety). They require a restart or a full AOT re-link.

### 1.3 Axiom Proof Integration

- Each function can have an attached **Axiom proof** (e.g., "this function does not panic", "this function respects the borrow checker").
- The proof is checked before the hot-reload is committed. If verification fails, the update is rejected and the old version remains.

**Example:**
```titan
#[axiom(proof = "never_panics.ax")]
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 { return 0; }  // proof ensures b != 0
    a / b
}
```

### 1.4 State Migration

Titan has a strong type system; changes to struct layout must be accompanied by a **migration function**:
```titan
#[version(old = "1.0", new = "1.1")]
pub fn migrate_state(old: &OldState) -> NewState { ... }
```
The runtime calls this function when a struct layout changes across versions.

---

## 2. Aether – Actor Language

Aether is a language for building concurrent, distributed systems using the actor model. Each actor has its own mailbox and no shared state.

### 2.1 Actor-Level Hot-Reload

- **Isolation** – Each actor can be hot-reloaded independently because actors have no shared state.
- **Suspension** – The runtime sends a `suspend` message to the actor, waits for it to finish processing its current message, then swaps its behaviour code.
- **Resumption** – After the swap, the actor resumes with the same mailbox and internal state (which must be compatible; see below).

### 2.2 State Migration for Actors

Actor state is a value of a type defined by the actor. When the type changes, the runtime needs a migration function:
```aether
actor Counter {
    var count: i32 = 0;
    
    #[version(old = "1.0", new = "1.1")]
    fn migrate(old: CounterV1) -> CounterV2 {
        CounterV2 { count: old.count, history: [] }
    }
}
```

### 2.3 Distributed Hot-Reload Across Nodes

- Actors can be migrated between physical nodes (using Bonsai's Echo fabric). Hot-reloading an actor's code on all nodes simultaneously requires a **two-phase commit**:
  1. Prepare: each node verifies that the new code is compatible (effect signatures, state schemas).
  2. Commit: all nodes swap the code atomically.
- If any node fails, the entire transaction rolls back.

### 2.4 Integration with Survival System

If an actor crashes after hot-reload, the Survival System:
- Restarts the actor (using the previous version of the code).
- Logs the failure and prevents further hot-reloads of that actor until the issue is fixed.

---

## 3. Sylva – Scripting Language

Sylva is a dynamically typed, interpreted language used for configuration, rapid prototyping, and glue code.

### 3.1 Tiered Execution for Sylva

- **Tier 1: Interpreter** – Runs instantly, no compilation overhead.
- **Tier 2: JIT** – Uses `cranelift` to compile hot functions to native code (background).
- **Tier 3: AOT** – Entire scripts can be compiled to WebAssembly or native code for production.

**Promotion:** The runtime tracks invocation count of each function. After 100 calls, it triggers a background JIT compilation; the next call uses the native version.

### 3.2 Hot-Reloading for Sylva

- **Dynamic reload** – Sylva's runtime supports `reload(filename)` that replaces the module's code.
- **State preservation** – Global variables are retained; if the new version adds a global, it is initialised lazily. If it removes a global, the variable is marked as unused and garbage-collected.
- **Backward compatibility** – The interpreter checks that the new version's exports are a superset of the old version's exports (i.e., no removal of public functions).

### 3.3 Integration with Knowledge Database

- The Bug Hunter can analyse Sylva scripts for common issues (typos, missing imports) and store patterns.
- BonsAI can suggest refactors that are hot-reloadable without breaking running systems.

---

## 4. Axiom – Formal Verification Language

Axiom is a dependently typed language used to write formal proofs about programs in other languages (Titan, Rust, etc.).

### 4.1 Proof-Attached Compilation

- **Proof objects** are stored alongside the verified program.
- **Proof checking** – When a program is hot-reloaded, any attached proofs must be re-checked. If a proof fails (e.g., the invariant no longer holds), the hot-reload is rejected.
- **Proof generation** – The Bug Hunter can attempt to generate missing proofs by leveraging the Knowledge Database and machine learning. Generated proofs are stored as candidates and must be manually reviewed.

### 4.2 Incremental Proof Verification

- **Proof dependencies** – A proof may depend on lemmas that are defined in other modules. If a lemma changes, all dependent proofs must be re-checked.
- **Caching** – Verified proofs are stored in CAS; if nothing changes, the proof check is skipped.

### 4.3 Integration with EternalTrainingLoop

- The ETL records which proofs are frequently re-verified (indicating unstable specifications). It can suggest strengthening invariants or refactoring the code to reduce proof churn.

---

## 5. Cross-Language Interactions

Omnisystem languages can call each other seamlessly:

- **Titan → Aether** – Titan code can spawn an actor and send messages. The FFI layer translates Titan types to Aether types (via serialisation).
- **Aether → Sylva** – An actor can evaluate a Sylva script using the embedded interpreter.
- **Sylva → Axiom** – Sylva can call a proof checker to validate properties at runtime (e.g., before executing a critical operation).

All cross-language calls are hot-reloadable, provided the interface version remains compatible (or a migration function is supplied).

---

## 6. Implementation Roadmap for Omnisystem

| Language | Phase 1 (Inc Comp) | Phase 2 (Hot-reload) | Phase 3 (State Migration) | Phase 4 (Distributed) |
|----------|--------------------|----------------------|---------------------------|----------------------|
| **Titan** | ✅ | ✅ | ✅ | ✅ |
| **Aether** | ✅ (actor-level) | ✅ | ✅ (migration functions) | ✅ (two-phase commit) |
| **Sylva** | N/A (interpreted) | ✅ | ✅ (lazy init) | ✅ (via Compute Fabric) |
| **Axiom** | ✅ (proof dependencies) | N/A (verification only) | N/A | N/A |

All phases are ready for implementation using existing Bonsai infrastructure (CAS, Echo, Compute Fabric, Survival).

---

## 7. Conclusion

The Omnisystem languages are the crown jewels of the Bonsai Ecosystem, and the BPCF design ensures they enjoy **first-class hot-reloadable, atomic, verifiable compilation**. Developers can update Titan drivers, Aether actors, Sylva scripts, and Axiom proofs **without stopping the system**, with automatic rollback on failure and global P2P cache sharing. 🚀
