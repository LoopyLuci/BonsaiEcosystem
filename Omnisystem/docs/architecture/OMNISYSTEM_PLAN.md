# Omnisystem Self-Hosting: Complete Implementation Plan

**Status:** May 18, 2026 — Specifications Complete, Ready for Build

## Executive Summary

The Omnisystem is a fully self-hosting programming environment built entirely on the Omnisystem languages (Titan, Aether, Sylva, Axiom). This document provides the complete blueprint for achieving full self-hosting, from the Rust bootstrap compiler through the complete runtime stack.

**Key Achievement:** By the end of this roadmap, the entire Omnisystem will compile and run itself with zero external dependencies except the initial Rust bootstrap.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    OMNISYSTEM SELF-HOSTING                       │
│                                                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  PHASE 1: Bootstrap (Rust → Titan)                       │   │
│  │  - Rust seed compiler (Cranelift backend)                │   │
│  │  - Compiles Titan code → Native code                     │   │
│  └──────────────────────────────────────────────────────────┘   │
│                           ↓                                       │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  PHASE 2: Runtime Kernel (Rust)                          │   │
│  │  - OmniCore interpreter (UniIR SSA executor)             │   │
│  │  - Capability enforcement                                │   │
│  │  - Task scheduling                                       │   │
│  │  - Telemetry tracking                                    │   │
│  └──────────────────────────────────────────────────────────┘   │
│                           ↓                                       │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  PHASE 3: Compiler Port to Titan (Self-Hosting)          │   │
│  │  - Lexer.ti — Tokenizer written in Titan                │   │
│  │  - Parser.ti — AST builder written in Titan             │   │
│  │  - BorrowChecker.ti — Lifetime checker in Titan         │   │
│  │  - Codegen.ti — Code generator in Titan                 │   │
│  │  - Titan compiler now compiles itself!                   │   │
│  └──────────────────────────────────────────────────────────┘   │
│                           ↓                                       │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  PHASE 4-7: Runtime Layers (Omni Languages)              │   │
│  │  - Aether: Actor runtime, supervision, CRDT             │   │
│  │  - Sylva: REPL, time-travel debugger                    │   │
│  │  - Axiom: Dependent types, proof kernel                 │   │
│  │  - IDE: Studio development environment                  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                           ↓                                       │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  RESULT: FULLY SELF-HOSTING OMNISYSTEM                   │   │
│  │  - No external dependencies                              │   │
│  │  - Omnisystem builds and runs itself                      │   │
│  │  - Compiler written in Titan                             │   │
│  │  - Runtime written in Titan/Aether/Sylva/Axiom          │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Complete File Specification

### Phase 1: Rust Seed Compiler

**Status:** ✅ Code complete, ready to build

```
titan-bootstrap/
├── Cargo.toml                          (Cranelift dependencies)
├── src/
│   ├── main.rs                         (CLI, 4-stage pipeline)
│   ├── lexer.rs                        (Tokenizer)
│   ├── parser.rs                       (Recursive descent)
│   ├── ast.rs                          (AST definitions)
│   ├── borrow_checker.rs               (Lifetime analysis)
│   ├── codegen_cranelift.rs            (Cranelift IR generation)
│   └── error.rs                        (Error handling)
└── target/release/
    └── titan-bootstrap.exe             (Compiled binary)
```

**Build Command:**
```bash
cd titan-bootstrap
cargo build --release
```

**Usage:**
```bash
./target/release/titan-bootstrap source.ti --run --verbose
```

### Phase 2: OmniCore Rust Interpreter

**Status:** ✅ Code complete, ready to build

```
omnicore/
├── Cargo.toml                          (No external deps except blake3)
├── src/
│   ├── lib.rs                          (Main interpreter + tests)
│   │   ├── UniIRModule                 (Module representation)
│   │   ├── Instruction enum            (SSA instructions)
│   │   ├── CapTable                    (Effect enforcement)
│   │   ├── TelemetryEngine             (Event tracking)
│   │   ├── ModuleRegistry              (Module loading)
│   │   ├── UniIRInterpreter            (SSA executor)
│   │   └── OmniCore                    (Kernel struct)
│   └── bin/
│       └── main.rs                     (CLI demo)
└── tests/
    └── integration.rs                  (Full system tests)
```

**Build Command:**
```bash
cd omnicore
cargo build --release
cargo test --release
```

### Phase 3: Titan Compiler (Self-Hosted)

**Status:** 📋 Specifications complete, awaiting implementation

```
titan/compiler/
├── lexer.ti                            (Tokenizer in Titan)
├── parser.ti                           (AST builder in Titan)
├── borrow_checker.ti                   (Lifetime checker in Titan)
├── codegen.ti                          (Codegen in Titan)
└── main.ti                             (Entry point)
```

**Dependencies:** Compiles with Rust seed, produces identical output

**Verification:**
```bash
# Compile with seed
titan-bootstrap test.ti -o seed_output.bin

# Compile Titan compiler with seed
titan-bootstrap titan/compiler/main.ti -o titan_compiler.bin

# Use Titan compiler to recompile
./titan_compiler.bin test.ti -o titan_output.bin

# Checksums must match
sha256sum seed_output.bin titan_output.bin
```

### Phase 4: Aether Actor Runtime

**Status:** 📋 Specifications complete, awaiting implementation

```
aether/
├── Cargo.toml                          (Rust crate)
├── src/
│   ├── lib.rs                          (Main runtime)
│   ├── actor.rs                        (Actor spawning + messages)
│   ├── mailbox.rs                      (Message queue)
│   ├── supervisor.rs                   (Supervision tree)
│   ├── crdt.rs                         (GCounter CRDT)
│   └── tests.rs                        (Unit tests)
└── examples/
    └── supervised_actors.rs            (Demo program)
```

**Features:**
- Actor spawning and message passing
- Supervision trees with restart strategies
- Grow-only counter CRDT
- Integration with OmniCore scheduling

### Phase 5: Sylva Interactive Frontend

**Status:** 📋 Specifications complete, awaiting implementation

```
sylva/
├── Cargo.toml                          (Rust crate)
├── src/
│   ├── lib.rs                          (REPL main loop)
│   ├── lexer.rs                        (Expression parser)
│   ├── evaluator.rs                    (Expression evaluation)
│   ├── debugger.rs                     (Time-travel debugging)
│   ├── types.rs                        (Gradual type checker)
│   └── tests.rs                        (Unit tests)
└── examples/
    └── repl_demo.rs                    (Interactive demo)
```

**Features:**
- Expression evaluation in REPL
- Function definition and call
- Time-travel debugging (record, rewind, replay)
- Gradual type checking (dynamic by default)
- Integration with Titan code

### Phase 6: Axiom Proof Kernel

**Status:** 📋 Specifications complete, awaiting implementation

```
axiom/
├── Cargo.toml                          (Rust crate, <50KB)
├── src/
│   ├── lib.rs                          (Kernel entry)
│   ├── terms.rs                        (De Bruijn term representation)
│   ├── types.rs                        (Bidirectional type checker)
│   ├── normalize.rs                    (WHNF normalizer)
│   └── tests.rs                        (Theorems + proofs)
└── examples/
    └── dependent_types.rs              (Demo theorems)
```

**Kernel Size:** <500 LOC (intentionally minimal for trust)

**Features:**
- Dependent types (Π and Σ)
- Universe hierarchy (Type_i : Type_{i+1})
- Beta reduction and let-inlining
- Bidirectional type checking

---

## Next Steps (Getting Started)

### Immediate (Today)

**1. Install Rust**
```powershell
# Windows 10/11 — Install Visual Studio Build Tools first
# Download: https://visualstudio.microsoft.com/downloads/
# Select: "Desktop development with C++"

# Then install Rust:
winget install Rustlang.Rust.MSVC

# Verify:
rustc --version
cargo --version
```

**2. Build the Rust Seed Compiler**
```powershell
cd z:\Projects\Omnisystem\titan-bootstrap
cargo build --release
```

**Expected:** Build completes in 1-2 minutes. Binary: `target/release/titan-bootstrap.exe`

**3. Test the Seed Compiler**
```powershell
# Create test file
echo 'fn main() -> i64 { return 42; }' > test.ti

# Run compiler
cargo run --release -- test.ti --run --verbose

# Expected output:
#   Titan Bootstrap Compiler v0.2.0 (Cranelift backend)
#   Source: test.ti
#     Lex: 12 tokens
#     Parse: 1 functions
#     Borrow Check: ok
#     Codegen: complete
#   Result: 42
```

### Week 1: Verify Bootstrap Chain

**4. Build OmniCore Interpreter**
```powershell
cd z:\Projects\Omnisystem\omnicore
cargo build --release
cargo test --release
```

**5. Run OmniCore Demo**
```powershell
cargo run --release --bin omnicore
```

**Expected Output:**
```
🌲 OmniCore Runtime Kernel
===========================
Module loaded: example
Modules in registry: 1
add(10, 20) = 30
Telemetry events:
  [module_loaded] example:a3f2
Trust score: 74
```

### Weeks 2-12: Implement Phases 3-6

Follow the IMPLEMENTATION_ROADMAP.md for detailed specifications for each phase.

---

## Build Commands Reference

```powershell
# Phase 1: Rust seed compiler
cd titan-bootstrap
cargo build --release

# Phase 2: OmniCore interpreter
cd omnicore
cargo build --release
cargo test --release

# Phase 3: Compile Titan compiler with seed
cd titan-bootstrap
cargo run --release -- ../titan/compiler/main.ti -o titan_compiler.bin

# Phase 4-6: Build runtime layers (Rust crates)
cd aether
cargo build --release

cd ../sylva
cargo build --release

cd ../axiom
cargo build --release

# Integration testing
cargo test --release --all
```

---

## Verification Checkpoints

### Checkpoint 1: Seed Compiler Functional
- [ ] `cargo build --release` succeeds
- [ ] `test.ti` compiles and outputs `42`
- [ ] Error messages are clear and helpful

### Checkpoint 2: OmniCore Operational
- [ ] All tests pass: `cargo test --release`
- [ ] Demo runs successfully
- [ ] Capability enforcement works

### Checkpoint 3: Titan Self-Hosting
- [ ] Titan compiler compiles with seed
- [ ] Titan compiler compiles itself
- [ ] Bit-identical output verified
- [ ] Rust seed deleted (no longer needed)

### Checkpoint 4: Complete Runtime
- [ ] All 4 language layers build successfully
- [ ] Integration tests pass
- [ ] IDE launches
- [ ] Full self-hosting verified

---

## Troubleshooting

### Build Fails: "cargo: command not found"
**Solution:** Add Rust to PATH or use full rustup path
```powershell
$env:PATH += ";C:\Users\<YourUsername>\.cargo\bin"
```

### Build Fails: "could not compile Cranelift"
**Solution:** Ensure Visual Studio Build Tools are installed
- Download: https://visualstudio.microsoft.com/downloads/
- Select: "Desktop development with C++"
- Restart terminal after installation

### Slow First Compile
**Normal behavior:** First Cranelift compilation can take 30-60 seconds. Subsequent builds are faster.

### Test Failures
**Debugging:**
```powershell
cargo test --release -- --nocapture  # Show println! output
cargo test --release test_name::     # Run specific test
```

---

## Project Structure

```
z:\Projects\Omnisystem\
├── BOOTSTRAP_GUIDE.md                 ← START HERE (installation & build)
├── IMPLEMENTATION_ROADMAP.md          ← Detailed specs for Steps 3-6
├── THIS_FILE: OMNISYSTEM_PLAN.md      ← Complete overview
│
├── titan-bootstrap/                   ← Phase 1: Rust seed (✅ ready)
│   ├── Cargo.toml
│   └── src/
│
├── omnicore/                          ← Phase 2: OmniCore (✅ ready)
│   ├── Cargo.toml
│   └── src/
│
├── titan/                             ← Phase 3: Self-hosted compiler (📋 specs)
│   ├── compiler/
│   │   ├── lexer.ti
│   │   ├── parser.ti
│   │   ├── borrow_checker.ti
│   │   ├── codegen.ti
│   │   └── main.ti
│   └── stdlib/
│
├── aether/                            ← Phase 4: Actor runtime (📋 specs)
│   ├── Cargo.toml
│   └── src/
│
├── sylva/                             ← Phase 5: REPL (📋 specs)
│   ├── Cargo.toml
│   └── src/
│
└── axiom/                             ← Phase 6: Proof kernel (📋 specs)
    ├── Cargo.toml
    └── src/
```

---

## Success Criteria

When complete, the Omnisystem will satisfy:

- ✅ **Compiles from Source:** `cargo build --release` from repository root
- ✅ **Self-Hosting:** Titan compiler compiles itself
- ✅ **No External Deps:** Only dependency is initial Rust bootstrap
- ✅ **Capability-Based Security:** All effects checked at runtime
- ✅ **Fault Tolerant:** Actors supervise and restart on failure
- ✅ **Debuggable:** Time-travel debugging works
- ✅ **Formally Verified:** Axiom proves correctness properties
- ✅ **IDE Available:** Omnisystem Studio ready for development

---

## References

- **Cranelift:** https://docs.rs/cranelift-codegen/
- **De Bruijn Indices:** https://en.wikipedia.org/wiki/De_Bruijn_index
- **Dependent Types:** https://en.wikipedia.org/wiki/Dependent_type
- **CRDT:** https://crdt.tech/
- **Actor Model:** https://en.wikipedia.org/wiki/Actor_model
- **Bootstrap:** https://en.wikipedia.org/wiki/Bootstrapping_(compilers)

---

## Timeline

| Phase | Description | Duration | Status |
|-------|-------------|----------|--------|
| 1 | Rust seed compiler (Cranelift) | 1-2 days | ✅ Ready |
| 2 | OmniCore interpreter | 3-5 days | ✅ Ready |
| 3 | Titan compiler rewrite | 4-6 weeks | 📋 Specs |
| 4 | Aether runtime | 1-2 weeks | 📋 Specs |
| 5 | Sylva REPL | 2-3 weeks | 📋 Specs |
| 6 | Axiom proof kernel | 2-3 weeks | 📋 Specs |
| **Total** | **Full self-hosting** | **~10-13 weeks** | **📋 Ready** |

---

## Contact & Support

For questions or issues:
1. Check BOOTSTRAP_GUIDE.md for common problems
2. Review IMPLEMENTATION_ROADMAP.md for detailed specifications
3. Run `cargo test --release` to verify components
4. Check git history: `git log --oneline` for recent changes

---

**Current Date:** May 18, 2026  
**Last Updated:** Today  
**Status:** Complete implementation plan ready for execution  
**Next Action:** Install Rust and build Phase 1 (Rust seed compiler)
