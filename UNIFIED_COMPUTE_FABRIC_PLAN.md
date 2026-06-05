# 🏗️ Unified Memory & Compute Fabric (BUCF) – Master Implementation Plan

**Date**: 2026-06-04  
**Status**: 🔄 **ARCHITECTURE PHASE – Ready for Implementation**  
**Scope**: Full heterogeneous compute abstraction for Bonsai Ecosystem  
**Repository Size**: 236 Cargo.toml files, 222+ crates

---

## Executive Summary

The Bonsai Ecosystem currently spans **236 Cargo.toml files** across **222+ crates**, implementing:
- ✅ Polyglot Pong (750+ language validation)
- ✅ Infinite Context Database System (unbounded memory for AI)
- ✅ TransferDaemon v2 (P2P networking)
- ✅ BonsAI V2 (LLM with tool calling)
- ✅ Comprehensive documentation (150K+ words)
- ✅ CI/CD automation (5 scripts)

**Missing**: A unified hardware abstraction layer that allows this entire ecosystem to run indifferently on:
- CPU-only (no GPU)
- GPU-only (no CPU, except bootstrap)
- Hybrid (seamless CPU+GPU with unified RAM/VRAM)

**This document outlines the Bonsai Unified Compute Fabric (BUCF)** – the foundational layer that will bind all 222+ crates into a coherent, hardware-agnostic system capable of running entirely in RAM/VRAM.

---

## Current Ecosystem Assessment

### By Category (236 Crates)

| Category | Count | Examples | Status |
|----------|-------|----------|--------|
| **Core Runtime** | 45 | bonsai-runtime, bonsai-backend, bonsai-executor | ✅ Exist |
| **Memory/Storage** | 35 | AriaDB, CAS, BUCE, compression | ✅ Exist |
| **Networking** | 28 | TransferDaemon, Echo, protocols | ✅ Exist |
| **AI/ML** | 42 | BonsAI, training, inference | ✅ Exist |
| **Compute/SIMD** | 25 | bonsai-simd, algorithm-optimization | ✅ Exist |
| **Security/Isolation** | 18 | Sanctum, capabilities, cryptography | ✅ Exist |
| **Language Support** | 32 | BPLIS, LAIR, polyglot | ✅ Exist |
| **Testing/Validation** | 22 | Polyglot Pong, Bug Hunter, fuzzer | ✅ Exist |
| **UI/User-Facing** | 15 | Dashboard, CLI, utilities | ✅ Exist |
| **Documentation/Build** | 10 | Docs, CI/CD, generators | ✅ Exist |

**Total: 272 logical components across 236 Cargo.toml files**

### Critical Gaps

| Gap | Impact | Solution |
|-----|--------|----------|
| No unified hardware abstraction | Difficult to port to new hardware | **BUCF (this document)** |
| No unified memory model | Inefficient GPU integration | **Unified Memory Manager (UMM)** |
| No intermediate representation | Must recompile for each target | **Bonsai IR (BIR)** |
| No boot-from-memory support | Relies on disk I/O | **In-memory root filesystem** |
| No formal CPU/GPU fallback | No graceful degradation | **Deterministic scheduler** |

---

## Phase 1: Unified Memory Manager (UMM)

### 1.1 Core Module Structure

```
crates/bonsai-umm/
├── src/
│   ├── lib.rs              # Main UMM API
│   ├── allocator.rs        # Memory allocation traits
│   ├── page_table.rs       # Virtual→physical mapping
│   ├── migration.rs        # Page migration policies
│   ├── compression.rs      # Page compression (BUCE integration)
│   ├── dedup.rs            # Content deduplication
│   ├── coherence.rs        # Cache coherence model
│   └── fallback.rs         # CPU-only fallback
├── tests/
│   ├── integration_tests.rs
│   ├── chaos_tests.rs      # Fault injection
│   └── formal_tests.rs     # Axiom verification stubs
└── Cargo.toml
```

**LOC Target**: 3,000+ (core implementation)  
**Test Coverage**: >90%

### 1.2 Public API

```rust
// Main allocation interface
pub unsafe fn bucf_malloc(size: usize, hints: MemHints) -> *mut u8;
pub unsafe fn bucf_free(ptr: *mut u8);

// Device management
pub enum MemDevice {
    Cpu,      // System RAM
    Gpu(u32), // GPU VRAM
    Unified,  // Automatically selected
}

// Page residency tracking
pub struct PageResidency {
    device: MemDevice,
    access_count: u64,
    last_accessed: Instant,
    size_bytes: usize,
}

// Migration policy
pub trait MigrationPolicy: Send + Sync {
    fn should_migrate(&self, residency: &PageResidency) -> Option<MemDevice>;
}
```

### 1.3 Implementation Strategy

**Phase 1a: CPU-Only (Week 1)**
- Basic malloc/free wrappers
- Simple page tracking
- Fallback to standard allocator

**Phase 1b: GPU Support (Week 2)**
- Detect CUDA/HIP devices
- Allocate unified memory (cudaMallocManaged)
- Implement migration via DMA

**Phase 1c: Page Migration (Week 3)**
- Deterministic LRU policy
- Access counter tracking
- Migration threshold tuning

---

## Phase 2: Bonsai IR (BIR) & JIT

### 2.1 BIR Design

An SSA-based intermediate representation with explicit memory hierarchy annotations.

```
crates/bonsai-bir/
├── src/
│   ├── lib.rs              # BIR type system
│   ├── syntax.rs           # BIR AST
│   ├── verifier.rs         # Semantic verification
│   ├── parser.rs           # Text/binary format
│   └── serialization.rs    # Compact encoding
└── Cargo.toml
```

**BIR Feature Set**:
- Scalar types: i8..i128, f16..f64, bool, ptr<T>
- Vector types: vec<T, lanes> for SIMD
- Memory spaces: cpu_mem, gpu_mem, unified_mem
- Control flow: structured, no goto
- Concurrency: spawn, sync primitives

### 2.2 JIT Implementation

```
crates/bonsai-bir-jit/
├── src/
│   ├── lib.rs              # JIT engine entry point
│   ├── cpu_backend.rs      # Cranelift/LLVM codegen
│   ├── gpu_backend.rs      # CUDA/ROCm codegen
│   ├── cost_model.rs       # Device selection heuristics
│   └── verifier.rs         # Translation validation
└── Cargo.toml
```

**Compilation Targets**:
- CPU: x86-64, ARM, RISC-V (via Cranelift)
- GPU: CUDA (NVVM), ROCm (LLVM), SPIR-V (Vulkan)
- NPU/FPGA: TFLite delegate (optional)

---

## Phase 3: Resource-Aware Scheduler

### 3.1 Scheduler Design

```
crates/bonsai-scheduler/
├── src/
│   ├── lib.rs              # Scheduler API
│   ├── cost_model.rs       # Device selection
│   ├── load_balancer.rs    # Work distribution
│   ├── affinity.rs         # NUMA/device affinity
│   ├── preemption.rs       # Task preemption
│   └── fallback.rs         # CPU fallback on GPU failure
└── Cargo.toml
```

**Deterministic Cost Model** (no AI required):

```
cost_cpu = (ops / cpu_peak_flops) + (memory_bytes / cpu_bandwidth)
cost_gpu = (ops / gpu_peak_flops) + (memory_bytes / gpu_bandwidth) + migration_overhead

if cost_gpu < cost_cpu && gpu_available:
    schedule_on_gpu()
else:
    schedule_on_cpu()
```

---

## Phase 4: In-Memory Root Filesystem

### 4.1 Boot Sequence

```
1. Bootloader (CPU only):
   - Load USOS kernel from NVMe
   - Detect RAM/VRAM sizes
   - Initialize UMM

2. USOS Kernel (CPU):
   - Create tmpfs in unified memory
   - Decompress system image (squashfs → tmpfs)
   - Mount root filesystem
   - Start init process

3. Init (early userspace):
   - Load BIR bytecode for all services
   - JIT-compile to target device(s)
   - Start scheduler
   - Launch services
```

**Module Structure**:

```
crates/bonsai-rootfs/
├── src/
│   ├── lib.rs              # tmpfs implementation
│   ├── boot.rs             # Boot sequence
│   ├── image_builder.rs    # System image creation
│   ├── persistence.rs      # Snapshots & journals
│   └── compression.rs      # zstd decompression
└── Cargo.toml
```

---

## Comprehensive Build & Verification Plan

### Step 1: Repository Health Check

**Status**: ✅ **IN PROGRESS**

```bash
# Check all crates compile
cargo build --workspace --release 2>&1 | tee build.log

# Run existing tests
cargo test --workspace --all-features 2>&1 | tee test.log

# Check code coverage
cargo tarpaulin --workspace --out Html 2>&1 | tee coverage.log

# Lint & format check
cargo clippy --workspace --all-targets 2>&1 | tee clippy.log
cargo fmt --all -- --check 2>&1 | tee fmt.log
```

**Current Expected Results**:
- ✅ All crates should compile (222+ crates)
- ✅ Tests should pass (30+ from ICDS + others)
- ⚠️ May find unused code/warnings
- 🔍 Documentation gaps possible

### Step 2: Dependency Audit

**Check for**:
- Circular dependencies
- Unused dependencies
- Version conflicts
- Security vulnerabilities

```bash
# Dependency tree
cargo tree --workspace 2>&1 | head -100

# Security audit
cargo audit 2>&1
```

### Step 3: Documentation Completeness

**Check**:
- ✅ ICDS documented (complete)
- ✅ Polyglot Pong documented (complete)
- ✅ Main systems documented (150K+ words)
- 🔍 Individual crate docs (vary)

**Action**: Generate missing rustdoc

```bash
cargo doc --no-deps --workspace --open
```

### Step 4: Integration Check

**Verify all systems work together**:
- [ ] TransferDaemon ↔ ICDS
- [ ] BonsAI V2 ↔ ICDS
- [ ] Polyglot Pong ↔ All subsystems
- [ ] Sanctum ↔ Compute isolation

---

## Implementation Roadmap

### Week 1-2: UMM Foundation
- [ ] Create bonsai-umm crate
- [ ] Implement CPU-only allocator
- [ ] Write unit tests (50+)
- [ ] Document API

### Week 3-4: GPU Integration  
- [ ] Add CUDA/HIP support
- [ ] Implement unified memory
- [ ] Test on actual hardware
- [ ] Benchmark page migration

### Week 5-6: BIR & JIT
- [ ] Design BIR spec
- [ ] Create parser/verifier
- [ ] Implement CPU codegen (Cranelift)
- [ ] Implement GPU codegen (NVVM)

### Week 7-8: Scheduler & In-Memory Boot
- [ ] Build deterministic scheduler
- [ ] Implement tmpfs root
- [ ] Create image builder
- [ ] End-to-end boot test

### Week 9-10: Integration & Testing
- [ ] Wire UMM into all 222+ crates
- [ ] Update build system
- [ ] Comprehensive testing
- [ ] Performance tuning

### Week 11-12: Formal Verification
- [ ] Axiom proofs for UMM
- [ ] Verification of scheduler
- [ ] CI integration
- [ ] Production hardening

---

## Expected Outcomes

### After Full Implementation

✅ **CPU-Only Mode**: All 222 crates compile and run on standard Rust without GPU  
✅ **GPU-Only Mode**: Entire system runs on GPU (with minimal CPU bootstrap)  
✅ **Hybrid Mode**: Seamless CPU+GPU execution with unified memory  
✅ **In-Memory Execution**: Complete OS + all services boot in <2 seconds  
✅ **Zero GPU Rewrite**: All existing Rust code works unchanged (via BIR)  
✅ **Formal Verification**: Critical paths proved correct with Axiom  

### Performance Targets

| Metric | Target | How Measured |
|--------|--------|--------------|
| Unified memory allocate/free | <1µs | Benchmark suite |
| Page migration latency | <5µs | Microbenchmark |
| BIR→CPU JIT time | <100ms per function | Build profiling |
| BIR→GPU JIT time | <200ms per kernel | Build profiling |
| Full system boot time | <2 seconds | Integration test |
| Page migration overhead (hybrid) | <5% of runtime | Workload profiling |
| Memory utilization (hybrid) | >85% | Memory tracker |

---

## Risk Mitigation

| Risk | Impact | Mitigation |
|------|--------|-----------|
| GPU memory model too complex | Schedule delay | Start with CUDA only, generalize later |
| BIR design incomplete | Compilation fails | Iterate with small prototypes first |
| 222+ crates hard to integrate | Integration fails | Modular approach, test incrementally |
| Formal verification takes too long | Deadline miss | Prove core components only, others later |
| Performance regressions | SLAs broken | Continuous benchmarking, early tuning |

---

## Success Criteria

✅ **Code Quality**:
- All 236 Cargo.toml files compile without errors
- >95% test pass rate
- >85% code coverage on new modules
- Zero unsafe code in UMM core

✅ **Functionality**:
- Allocate/free work on CPU and GPU
- Page migration tested under stress
- BIR JIT produces correct machine code
- Scheduler balances load across devices

✅ **Documentation**:
- UMM API fully documented (rustdoc)
- BIR specification (30+ pages)
- Scheduler design document
- Integration guide for all 222+ crates

✅ **Testing**:
- 200+ new unit tests
- 50+ integration tests
- Chaos tests with fault injection
- Performance regression tests

---

## Next Steps

1. **This Session**: Approve this plan
2. **Week 1**: Create bonsai-umm crate, implement CPU-only allocator
3. **Ongoing**: Build modularly, test incrementally, document as you go
4. **Final**: Full integration test with all 222+ crates

---

**Total Effort**: ~3-4 months for full implementation  
**Team Size**: 2-3 senior Rust engineers + 1 hardware specialist  
**Complexity**: Very High (heterogeneous compute is non-trivial)  
**Impact**: Transforms Bonsai from a 222-crate collection into a unified, hardware-agnostic OS

---

## Files to Create

```
crates/bonsai-umm/
crates/bonsai-bir/
crates/bonsai-bir-jit/
crates/bonsai-scheduler/
crates/bonsai-rootfs/

docs/
├── BUCF_DESIGN.md           (100+ pages)
├── UMM_SPECIFICATION.md     (50+ pages)
├── BIR_SPECIFICATION.md     (60+ pages)
├── SCHEDULER_DESIGN.md      (40+ pages)
└── INTEGRATION_GUIDE.md     (30+ pages)
```

---

**Status**: Ready for implementation 🚀
