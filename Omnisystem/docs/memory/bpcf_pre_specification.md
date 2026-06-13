---
name: bpcf_pre_system
description: "Bonsai Pre-Compilation Fabric specification — speculative, AI-guided pre-compilation with <1ms incremental builds"
metadata: 
  node_type: memory
  type: reference
  originSessionId: c7ae2a7a-5206-469e-8d6b-97fc5255ee90
---

## Bonsai Pre-Compilation Fabric (BPCF-Pre) — Specification & Architecture

**Purpose:** Transform compilation from minutes to milliseconds by pre-computing all semantic compilation work (macro expansion, generics monomorphisation, constant evaluation, partial evaluation, AI hints) speculatively in the background, before the developer even saves code.

### Key Innovations

1. **Speculative Change Prediction** — BonsAI V2 predicts which functions will be edited next (based on cursor position, recent history) and pre-compiles them in the background
2. **Macro & Generics Caching** — Pre-expanded macros and monomorphised generics stored in CAS, shared P2P via Echo, reused across the team
3. **Partial Evaluation & Specialisation** — Automatically generates specialised function variants for common input patterns
4. **AI Optimisation Hints** — BonsAI V2 attaches metadata to LAIR (inline suggestions, vectorisation opportunities, etc.) consumed by the JIT/AOT backend
5. **Cross-Language LAIR Lowering** — Enables cross-language inlining and FFI elimination (Rust ↔ Python, etc.)
6. **Distributed via Compute Fabric** — Heavy pre-compilation tasks offloaded to idle devices

### Performance Targets

- Speculative pre-compilation hit rate: >85%
- Macro expansion cache hit rate: >99%
- Partial evaluation speedup: 2–10× for functions with known arguments
- Background CPU usage: <10% of one core

### Implementation Phases

| Phase | Focus | Deliverable |
|-------|-------|-------------|
| 1 | Macro expansion cache + constant evaluator | Pre-expanded macros in CAS |
| 2 | Generic monomorphisation pre-compilation | Eliminate monomorphisation from main build |
| 3 | Speculative predictor + BonsAI V2 | Background pre-compilation of likely next edits |
| 4 | Partial evaluator + specialisation | Auto-generation of specialised function variants |
| 5 | AI optimisation hints | Metadata-driven backend optimisations |
| 6 | Distributed pre-compilation | Team-wide sharing via Compute Fabric |
| 7 | Cross-language LAIR pre-compilation | FFI elimination, cross-language inlining |
| 8 | Production hardening + EternalTrainingLoop | Continuous improvement of hit rates |

### Integration with BACE

- BACE receives pre-computed LAIR and skips macro/generics phases entirely
- Pre-compiled specialised functions hot-swapped alongside normal recompilations
- AI hints guide BACE's JIT/AOT optimisation decisions

### Related Systems

- Feeds into: BACE (atomic compilation), EternalTrainingLoop (learn from outcomes)
- Consumes: BonsAI V2 (prediction, hint generation), CAS (artifact storage), Echo (P2P distribution)
