# 🌐 Bonsai Polyglot Language Integration System (BPLIS) — Complete Architecture

**Version:** 1.0  
**Status:** Phase 1 Complete, Phase 2-8 Planned  
**Commit:** b7146306  
**Date:** 2026-06-04

---

## Overview

BPLIS is a unified language integration framework that makes every programming language a first-class citizen in the Bonsai Ecosystem. Languages don't compete—they cooperate through:

1. **LAIR** — Language-Agnostic Intermediate Representation
2. **LanguageFrontend Trait** — Single integration point  
3. **Unified Tooling** — `bonsai` CLI, LSP, debugger
4. **Shared Runtime** — BUEB, BACE, BUSH, hot-reload

---

## Architecture Layers

### **Layer 1: Language Frontends**

Each language implements `LanguageFrontend` trait:

```rust
pub trait LanguageFrontend: Send + Sync {
    fn language_name(&self) -> &str;
    fn file_extensions(&self) -> &[&str];
    async fn parse(&self, source: &str, path: &Path) -> Result<LairModule>;
    fn lsp_server(&self) -> Option<Box<dyn LspHandler>> { None }
    fn interpreter(&self) -> Option<Box<dyn LairInterpreter>> { None }
    async fn format(&self, source: &str) -> Result<String> { ... }
    async fn lint(&self, source: &str) -> Result<Vec<Diagnostic>> { ... }
}
```

### **Layer 2: LAIR IR**

Common intermediate representation with:
- Typed SSA form
- Effect annotations
- Hot-reload metadata
- Universal FFI
- 9 term constructors (based on Axiom proof kernel)

```rust
pub struct LairModule {
    pub name: String,
    pub functions: Vec<LairFunction>,
    pub types: Vec<LairTypeDefinition>,
    pub constants: Vec<LairConstant>,
    pub metadata: ModuleMetadata,
}
```

### **Layer 3: BACE Optimizer**

All LAIR modules flow through:
- Type inference
- Effect inference
- Dead code elimination
- Constant folding
- Parallel compilation units

### **Layer 4: Backend**

Multiple backends available:
- **LLVM** — Native code (Titan, Aether, parts of Go)
- **Cranelift** — JIT-friendly (Sylva, hot paths)
- **Interpreter** — Direct execution (Sylva REPL, SQL)
- **WASM** — Browser targets

---

## Language Integration Strategy Matrix

| Language | Integration | Approach | Frontend | Status |
|----------|-------------|----------|----------|--------|
| **Titan** | Native LAIR | Type-checked → LAIR → LLVM | Full compiler | Phase 3 |
| **Sylva** | Native LAIR | Parser → Bytecode → VM/JIT | Full compiler | Phase 2 |
| **Aether** | Native LAIR | Actor model → LAIR message passing | Full compiler | Phase 5 |
| **Axiom** | Proof layer | Dependent types → proof checker | Library integration | Phase 6 |
| **Python** | Wrapper | CPython embed + LAIR bridge | Hybrid | Phase 4 |
| **Go** | Native LAIR | Reuse TinyGo parser → LAIR | Full compiler | Phase 7 |
| **SQL** | Native LAIR | Parser → relational algebra → LAIR | Full compiler | Phase 4 |
| **Ruby** | Wrapper | mruby embed + LSP | Hybrid | Phase 7 |
| **JavaScript** | Wrapper | V8/Deno embed + LSP | Hybrid | Phase 7 |
| **Java** | Wrapper | JDK/GraalVM + LSP | Hybrid | Phase 7 |

---

## Compilation Pipeline

```
Source Code (.titan, .sylva, .py, .sql, etc.)
    ↓
Language-Specific Frontend (implements LanguageFrontend)
    ↓
Parse → AST → LAIR Module
    ↓
BACE Optimizer (type inference, effect analysis, optimizations)
    ↓
Backend Selection:
    ├─ LLVM → Native Code (.o, .exe, .so)
    ├─ Cranelift → JIT Compiled Code
    ├─ Interpreter → Direct Execution
    └─ WASM → Browser/Sandboxed
    ↓
BUEB Execution Engine (CPU/GPU scheduling)
    ↓
Result / Execution
```

---

## Unified CLI

```bash
# Compile any language
bonsai build app.titan       # → app (native executable)
bonsai build script.sylva    # → script (bytecode or JIT)
bonsai build query.sql       # → query (optimized plan)
bonsai build server.py       # → server (via CPython)

# Run with hot-reload
bonsai run app.titan --watch

# Format, lint, check
bonsai fmt .                 # Format all files
bonsai lint .                # Lint all files
bonsai check .               # Type/effect check

# Language Server
bonsai lsp                   # Start LSP on stdio

# REPL
bonsai repl                  # Interactive shell
bonsai repl --lang sylva     # Sylva-specific REPL
```

---

## Phase Roadmap (8 Phases, ~8-12 Months)

### Phase 1: Foundation ✅ COMPLETE
- ✅ LAIR IR specification and types
- ✅ LanguageFrontend trait & registry
- ✅ Sylva skeleton (parser, VM, compiler stubs)
- ✅ LSP infrastructure
- ✅ Compilation fixes (tokio_rusqlite, bace-rt, etc.)

### Phase 2: Complete Sylva
- ⏳ Full Pest-based parser
- ⏳ Bytecode compiler
- ⏳ Stack-based VM with all operations
- ⏳ JIT compilation (Cranelift)
- ⏳ Time-travel debugging support
- ⏳ REPL with syntax highlighting

### Phase 3: Titan Systems Language
- ⏳ Type checker (Hindley-Milner + effects)
- ⏳ Effect system enforcement
- ⏳ LAIR lowering with optimizations
- ⏳ LLVM code generation
- ⏳ FFI and unsafe blocks
- ⏳ Standard library (core, alloc, std)

### Phase 4: Python & SQL Integration
- ⏳ CPython embedding via Py-O3
- ⏳ LAIR bridge for hot paths
- ⏳ SQL parser (PostgreSQL dialect)
- ⏳ Relational algebra IR
- ⏳ Query optimizer
- ⏳ Multi-dialect support (MySQL, SQLite, etc.)

### Phase 5: Aether Actor Language
- ⏳ Actor syntax & message types
- ⏳ Supervision tree implementation
- ⏳ Message passing primitives
- ⏳ Distributed state (CRDTs)
- ⏳ Location transparency
- ⏳ Actor runtime integration with USOS

### Phase 6: Axiom Proof Layer
- ⏳ Proof kernel integration
- ⏳ Dependent type checking
- ⏳ Proof attributes for Titan functions
- ⏳ Proof verification at compile time
- ⏳ Curry-Howard correspondence
- ⏳ Lean/Coq interop

### Phase 7: Remaining Languages
- ⏳ Go: Full LAIR frontend
- ⏳ Ruby: mruby embedding + LSP
- ⏳ JavaScript: V8/Deno integration
- ⏳ Java: JDK/GraalVM + LSP
- ⏳ C#: OmniSharp + LSP
- ⏳ NoSQL: MongoDB, Neo4j query support

### Phase 8: Performance & Optimization
- ⏳ JIT compilation for all languages
- ⏳ Multi-threaded BACE optimizer
- ⏳ Profile-guided optimization
- ⏳ SIMD vectorization
- ⏳ Memory optimization passes
- ⏳ Benchmarks & performance suite

---

## File Structure

```
crates/
├── bonsai-lair/                    # 80 lines — IR types
├── bonsai-language-frontend/       # 300 lines — Frontend trait + registry
│   ├── src/
│   │   ├── lib.rs
│   │   ├── frontend.rs             # LanguageFrontend trait
│   │   ├── registry.rs             # Language discovery
│   │   ├── lsp.rs                  # LSP server infrastructure
│   │   └── errors.rs
│   └── Cargo.toml
├── omnisystem-sylva/               # 450+ lines — Scripting language
│   ├── src/
│   │   ├── lib.rs
│   │   ├── frontend.rs
│   │   ├── parser.rs               # Pest-based parser
│   │   ├── ast.rs
│   │   ├── compiler.rs             # AST → bytecode
│   │   └── vm.rs                   # Stack-based interpreter
│   ├── src/grammar.pest            # PEG grammar
│   └── Cargo.toml
├── omnisystem-titan/               # Systems language (Phase 3)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── frontend.rs
│   │   ├── parser.rs
│   │   ├── ast.rs
│   │   ├── typeck.rs               # Type checker + effect system
│   │   └── lower.rs                # AST → LAIR
│   └── Cargo.toml
├── omnisystem-aether/              # Actor language (Phase 5)
├── omnisystem-axiom/               # Proof layer (Phase 6)
├── omnisystem-python/              # Python integration (Phase 4)
├── omnisystem-go/                  # Go integration (Phase 7)
├── omnisystem-sql/                 # SQL integration (Phase 4)
├── omnisystem-ruby/                # Ruby integration (Phase 7)
├── omnisystem-javascript/          # JavaScript integration (Phase 7)
└── omnisystem-java/                # Java integration (Phase 7)

scripts/
├── bonsai-cli/                     # Unified CLI entry point
├── language-installer.sh            # Install language dependencies
└── lsp-launcher.sh                 # LSP server startup

docs/
├── LANGUAGE_FRONTENDS.md           # How to add a language
├── LAIR_SPEC.md                    # LAIR IR specification
└── BPLIS_INTEGRATION_GUIDE.md      # Integration guide
```

---

## Key Design Decisions

### 1. LAIR as Common IR
- **Why:** Every language targets the same intermediate representation
- **Benefit:** Single optimizer benefits all languages; cross-language optimization
- **Trade-off:** Must design IR to handle diverse language semantics

### 2. Trait-Based Registration
- **Why:** No central registry needed; languages self-register
- **Benefit:** Languages can be added as optional dependencies
- **Trade-off:** Requires inventory crate for compile-time collection

### 3. Hybrid Integration for Existing Languages
- **Why:** Rewriting production languages (Python, Java) from scratch is impractical
- **Benefit:** Get IDE support, cross-language optimization without full rewrite
- **Trade-off:** Less control over semantics; harder to ensure consistency

### 4. LSP as Primary IDE Integration
- **Why:** Standard protocol; every modern editor supports it
- **Benefit:** Single LSP server handles all 8+ languages
- **Trade-off:** Language-specific IDE features may be limited

---

## Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| Languages integrated | 8+ | ✅ Architecture ready |
| Compilation speed | <30s incremental | ⏳ BACE optimization |
| Performance parity | ±10% vs. native | ⏳ JIT + optimization |
| Cross-language calls | Zero-overhead FFI | ⏳ LAIR FFI design |
| IDE support | 100% features | ⏳ LSP expansion |
| Type safety | Gradual to static | ✅ LAIR supports both |

---

## Next Steps

1. **Complete Sylva** (Phase 2) — Full parser, bytecode VM, JIT
2. **Implement Titan** (Phase 3) — Systems language with effects
3. **Integrate Python/SQL** (Phase 4) — Wrappers + LAIR bridges
4. **Actor runtime** (Phase 5) — Aether with supervision trees
5. **Proof integration** (Phase 6) — Axiom attributes in Titan
6. **Remaining languages** (Phase 7) — Go, Ruby, JavaScript, Java, etc.
7. **Performance** (Phase 8) — JIT, optimization, benchmarks

---

## References

- **Language Design:** Omnisystem repo (Titan, Sylva, Aether, Axiom specs)
- **LAIR IR:** Based on Lean proof kernel + LLVM IR concepts
- **Bonsai Infra:** BACE, BUEB, BUSH, hot-reload systems
- **Tooling:** Unified CLI, LSP, debugger across all languages

---

**Status: Ready for Phase 2 implementation.** 🚀
