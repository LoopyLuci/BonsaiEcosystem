# Omnisystem Architecture

## Trust Ladder

The Omnisystem is organized around four irreducible trust layers:

```
L0: Titan     — Zero-Trust Systems      — Every instruction justified, no runtime
L1: Aether    — Managed Trust Services  — Runtime guarantees safety, recoverable
L2: Sylva     — Forgiving Trust         — Human-speed development, fail fast
L3: Axiom     — Proven Trust            — Machine-checked mathematical certainty
```

## UniIR — The Universal Intermediate Representation

UniIR is the single semantic pivot for all four languages. It is a typed SSA-based IR with:

- **Explicit effects:** io, alloc, panic, telemetry, and user-defined effects
- **Region annotations:** For Titan's ownership and borrowing model
- **Graded modalities:** 0 (erased proofs), 1 (linear resources), ω (unrestricted)
- **Device annotations:** @cpu, @gpu, @fpga, @tpu
- **Concurrency primitives:** spawn, send, receive, sync

### Lowering Targets
- LLVM IR (native x86-64, ARM64, RISC-V)
- OmniVM bytecode (managed execution for Aether/Sylva)
- WebAssembly with GC, threads, and SIMD
- Hardware synthesis via MLIR/CIRCT for FPGA/ASIC

### Binary Format (TLV)
Tag-Length-Value encoding with sections for metadata, type table, symbol table, SSA code, effect rows (sorted for deterministic hashing), and proof stubs.

## OmniCore — The Runtime Kernel

OmniCore is a ~50K LOC Titan kernel providing:
- **Scheduler:** Work-stealing with affinity support for real-time Titan code
- **Capability Table:** Runtime enforcement of the effect system; violations abort the sandbox, never the kernel
- **Telemetry Engine:** Structured, content-addressed event emission for every effect, allocation, and actor lifecycle
- **Module Loader:** TLV parsing with content hash verification

## Language Specifications

### Titan — L0 Systems Language
- **Type System:** Static, strong, nominal + structural generics, optional dependent types
- **Memory:** Ownership + borrowing with lifetime elision (90% automatic). No GC.
- **Effects:** Explicit row-polymorphic effects enforced as capabilities
- **Concurrency:** Tasks, SIMD annotations, zero-cost async coroutines
- **Performance:** 95-100% of hand-written C. <5s compile for 100K LOC.

### Aether — L1 Application Language
- **Model:** Actor-based with supervision trees, location transparency
- **Consistency:** Consistent<T>, Eventually<CRDT>, Causal<T>, BoundedStaleness<Duration>
- **Memory:** Generational concurrent GC + regions + optional linear types
- **Concurrency:** Millions of actors per node, transparent distribution across nodes
- **Performance:** 80-90% of equivalent Go/Java throughput

### Sylva — L2 Interactive Language
- **Typing:** Gradual + structural + refinement contracts
- **Syntax:** Homoiconic with hygienic macros
- **Features:** Time-travel debugging, live hot-reload, notebook environment
- **Performance:** <50ms REPL startup, hot loops within 2× native after JIT
- **Hardware:** Automatic GPU/FPGA offload via effect inference

### Axiom — L3 Proof Language
- **Type System:** Full dependent types, infinite universe hierarchy
- **Kernel:** ~500 lines of audited Titan code (trusted computing base)
- **Features:** AI-assisted proof synthesis, tactics, SMT-backed automation
- **Output:** Proof-carrying Titan code or runtime contracts

## Cross-Language Interop

All four languages compile to UniIR modules. Cross-language calls go through:
- **Direct linking:** Titan functions called from Aether at zero cost
- **Shared heap:** Sylva and Aether share the OmniVM heap
- **Universal serialization:** Content-addressed CBOR with type schemas
- **Cross-language polymorphism:** Type-class dictionaries passed across boundaries

## Self-Hosting Chain

The Titan Stage 3B compiler consists of 8 modules, all written in Titan:
1. lexer.ti
2. parser.ti
3. borrow_checker.ti
4. lowering.ti
5. codegen.ti
6. xast.ti
7. xast_to_uniir.ti
8. axiom_kernel/typechecker.ti

The native Titan compiler (`titan-bootstrap/output/titan-compiler.exe`) compiles all Omnisystem source files. No Python, Rust, C compiler, or external tool is required. The Omnisystem is fully self-hosting.

## Universal Language Converter Framework (ULCF)

Omni Lingua converts any language to Omni languages through the XAST pivot:

```
Source Language → Frontend Parser → XAST → Universal Backend → UniIR → Titan/Aether/Sylva
```

### Language Engines
| Engine | Languages Covered |
|--------|-------------------|
| C-Family | C, C++, Rust, Go, Swift, Ada, Fortran, COBOL, Dart, Kotlin/Native |
| Pythonic | Python, Ruby, Perl, R, Julia, Scheme, Lisp, Clojure |
| ML-Family | OCaml, F#, Haskell, Elixir, Erlang/OTP |
| Java-VM | Java, Kotlin, Scala, C# |
| Web-Script | JavaScript, TypeScript, PHP |

### Fidelity Levels
- **Certified:** 100% semantic preservation (Axiom proof attached)
- **High-Fidelity:** ≥98% with extensive test suite
- **Exploratory:** Fast conversion with TODOs

## Content-Addressed Package Registry

- **Identity:** Blake3 hash of normalized UniIR module
- **Local:** .build-registry/ directory with modules/ and index/
- **Global:** Kademlia DHT with replication factor k=3
- **Features:** Proof-carrying entries, trust-score aggregation, dependency resolution
- **Commands:** build publish, build import <hash>, build registry list/verify/stats

## Omni Studio IDE

- **LSP Server:** Go-to-definition, hover, diagnostics with UniIR rule citations
- **Cross-language:** References across Titan, Aether, Sylva, and Axiom
- **Dataflow:** Live subscription to OmniCore telemetry
- **Debug Adapter:** DAP bridge for time-travel debugging
- **VS Code Extension:** Full marketplace packaging
