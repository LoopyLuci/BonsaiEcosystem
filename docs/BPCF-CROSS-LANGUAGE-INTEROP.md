# 🔗 BPCF – Cross-Language Interoperability: LAIR, FFI, State Migration

**Version:** 1.0  
**Status:** Production Specification  

This document details the mechanisms for seamless cross-language hot-reloading, foreign function calls, and state migration across different programming languages within the Bonsai Polyglot Compilation Fabric (BPCF). The core enabler is the **Bonsai Universal IR (BUIR)** and a set of runtime contracts.

---

## 1. Bonsai Universal IR (BUIR) – The Lingua Franca

BUIR is a language-agnostic intermediate representation that all BPCF language frontends produce. It includes:

- **Types** – Primitive types (int, float, bool), structs, enums, tuples, arrays, pointers, functions.
- **Effects** – `async`, `unsafe`, `io`, `alloc`, `noreturn`.
- **Versioning** – Each type and function has a version number.
- **Dependencies** – Edges to other functions/types (cross-language).
- **Provenance** – Source language, file, line.

**Example BUIR for a Rust function and a Python function:**
```
; Rust: fn add(a: i32, b: i32) -> i32
function @add(i32, i32) -> i32 version=1 language=rust {
    %0 = add i32 %a, %b
    ret %0
}

; Python: def add(a, b): return a + b
function @add(i32, i32) -> i32 version=1 language=python {
    %0 = add i32 %a, %b
    ret %0
}
```

BUIR is stored in CAS and shared across devices.

---

## 2. Cross-Language Foreign Function Interface (FFI)

### 2.1 Direct FFI Calls

When a function in language A calls a function in language B, the BPCF compiler:

1. **Generates a stub** in the caller's language that marshals arguments and calls a generic runtime function `cross_call`.
2. **The runtime** uses the target function's BUIR to determine calling convention, argument sizes, and return type.
3. **Data conversion** – The runtime converts values between language representations (e.g., Rust `String` to Python `str`).

**Performance:** Cross-language calls have a small overhead (~50ns + conversion cost). Hot-reloadable cross-language calls use an indirection table as in single-language case.

### 2.2 Interface Definition Language (IDL)

To simplify FFI, BPCF provides an IDL (based on WebAssembly Interface Types) that declares the contract:

```idl
interface math {
    add: (a: i32, b: i32) -> i32 version=1
    subtract: (a: i32, b: i32) -> i32 version=1
}
```

The IDL is compiled into BUIR and used to generate stubs for both sides automatically.

### 2.3 Versioning and Hot-Reload Across Languages

When a function in language B is hot-reloaded, the runtime:

1. Checks that its signature (in BUIR) remains compatible (same parameter types and return type, same version number).
2. Updates the global function pointer table used by language A's stub.
3. If a migration function is provided (e.g., to convert between different data layouts), it is called.

**Atomicity:** The update is atomic across all languages that depend on the function; if any language fails to update, the entire transaction rolls back.

---

## 3. State Migration Across Languages

### 3.1 Problem

When a data structure shared across languages changes its layout (e.g., a Rust struct gains a new field), all languages that use that struct must migrate their instances to the new layout.

### 3.2 Solution: Versioned Schemas

- Each type has a **version number** embedded in its BUIR definition.
- The runtime maintains a **type registry** mapping type name + version to its layout.
- When a new version is loaded, the runtime generates a **migration function** (or uses a user-supplied one) that converts an instance of the old version to the new version.

**Migration function example (Rust → Python):**
```rust
#[version(old = "1.0", new = "1.1")]
pub fn migrate_state(old: &OldState) -> NewState {
    NewState {
        field1: old.field1,
        field2: old.field2.clone(),
        new_field: Default::default(),
    }
}
```

### 3.3 Automatic Migration for Simple Cases

For trivial changes (adding a field, removing a field), the runtime can automatically migrate using a **default value** for new fields or ignoring removed ones.

### 3.4 Cross-Language Migration

When a type is used in multiple languages, the migration function must be supplied in one of the languages (e.g., Rust) and is callable via FFI from others.

The runtime ensures that all instances across all language heaps are migrated atomically before any new-version code runs.

---

## 4. LAIR – Lightweight Atomic IR

LAIR is a simplified subset of BUIR that is used for **fast JIT compilation** and **interpreter fallback**. It strips away high-level language features (e.g., generics) and is close to C-like.

**Properties:**
- No dynamic dispatch (unless explicit)
- No exceptions (errors returned as values)
- Linear memory model (no garbage collection)
- Direct mapping to Cranelift / LLVM.

**Use case:** When a hot-reload fails, the system can fall back to LAIR-based interpreter to keep the program running while the user fixes the issue.

---

## 5. Distributed Cross-Language Compilation

When compiling a cross-language project, the Compute Fabric:

1. **Partitions the dependency graph** – Each node may contain functions from multiple languages.
2. **Assigns each partition to a node** that has the appropriate compilers installed.
3. **Collects BUIR fragments** from all nodes and merges them into a single CAS artifact.
4. **Builds the final executable** (or dynamic library) using a cross-language linker (customised `lld`).

---

## 6. Example End-to-End

**Scenario:** A Rust function `compute` is called from a Python script. The developer changes the Rust function and wants to hot-reload it.

1. Editor saves `compute.rs`. Rust compiler detects change, re-compiles only `compute` to BUIR and object code.
2. BUIR is stored in CAS; the new object code is pushed to Echo.
3. The Python runtime receives a notification that `compute` has a new version.
4. At a safe point, the Python runtime pauses the event loop.
5. It calls the cross-language update handler, which replaces the function pointer in a global table used by the FFI stub.
6. The Rust runtime also updates its own internal pointer.
7. State migration is not needed because the function signature did not change.
8. Both runtimes resume; any subsequent call to `compute` uses the new version.
9. Universe logs the event; Survival System monitors for panics.

If the new `compute` panics, the Survival System rolls back the update to the previous version and alerts the developer.

---

## 7. Security & Sandboxing

- Cross-language FFI calls are **sandboxed** – the runtime verifies that the target function has the required capability token.
- Migration functions are executed in a **Sanctum vault**; if they crash, the entire hot-reload transaction is aborted.
- All cross-language calls are subject to the same rate limiting and circuit breaking as local calls.

---

## 8. Implementation Roadmap

| Phase | Focus | Deliverables |
|-------|-------|--------------|
| **1** | BUIR specification and compiler frontends for Rust, Python, C | BUIR parser, serialisation, CAS storage. |
| **2** | IDL compiler and FFI stubs | Cross-language calls between Rust and Python work. |
| **3** | Versioned types and state migration | Automatic migration for simple structs; user-defined migration functions. |
| **4** | LAIR and interpreter fallback | Tier 1 execution for hot-reload failures. |
| **5** | Distributed compilation across languages | Compute Fabric integration. |

---

## 9. Conclusion

The BPCF cross-language interoperability design provides **seamless, atomic, hot-reloadable FFI** between any languages in the Bonsai Ecosystem. By using BUIR as a common intermediate representation, versioned schemas, and a robust migration framework, the system guarantees that cross-language updates are as safe and transparent as single-language updates. 🚀
