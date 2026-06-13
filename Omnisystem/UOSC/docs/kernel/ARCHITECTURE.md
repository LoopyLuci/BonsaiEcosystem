# UOSC Kernel Architecture

Complete formal specification of the UOSC micro-kernel design, implementation, and invariants.

## Executive Summary

The UOSC kernel is a minimal, verifiable micro-kernel implementing:
- **Process abstraction** with strong isolation guarantees
- **Memory management** with hardware-enforced protection
- **Fair scheduling** with deterministic behavior
- **Resource accounting** with per-process limits

All critical paths are formally verified in Axiom. The kernel enforces invariants that guarantee:
1. Process isolation is unbreakable
2. Memory is always consistent
3. Scheduling is fair and deterministic
4. Resources cannot be exhausted by misbehaving processes

## Architectural Principles

### 1. Minimal Surface
The kernel exports minimal functionality. Complex operations are built in user-mode libraries atop simple, proven primitives.

**Kernel API**: ~50 hypercalls
**User Libraries**: Unlimited, built by users as needed

### 2. Complete Transparency
Every kernel operation is observable. Users can inspect process state, memory layout, scheduling decisions, and resource usage in real-time.

**Axiom**: "What is hidden cannot be verified."

### 3. Formal Verification
All critical invariants are proven in Axiom. Users can verify correctness before trusting code.

**Safety Properties**: Process isolation, memory safety
**Liveness Properties**: No deadlock, eventual progress
**Security Properties**: Access control enforcement

### 4. Deterministic Execution
The kernel has no hidden non-determinism. Given the same input, the same operations produce identical results.

**Exception**: Hardware-dependent timings (but guaranteed bounds)

## Core Subsystems

### 1. Process Subsystem

#### Data Structures
```
Process = {
  pid: ProcessID,
  state: (RUNNABLE | RUNNING | BLOCKED | TERMINATED),
  memory_space: MemorySpace,
  registers: CPUState,
  kernel_stack: StackPtr,
  priority: i64,
  cpu_time: Nanoseconds,
  wall_time: Nanoseconds,
  resource_limits: ResourceLimits,
  parent_pid: ProcessID,
  capabilities: CapabilitySet
}
```

#### Operations
- `process_create(entry_point, initial_stack)` — Create new process
- `process_terminate(pid)` — Terminate process, cleanup resources
- `process_wait(pid)` — Block until process terminates, return exit code
- `process_self()` — Get current process ID
- `process_get_state(pid)` — Query process state

#### Invariants (Formally Proven)
```
Invariant 1: ProcessIsolation
∀ p1, p2: Process. p1 ≠ p2 ⇒ 
  (p1.memory_space ∩ p2.memory_space = ∅)

Invariant 2: ResourceBounds
∀ p: Process, r: Resource. 
  usage(p, r) ≤ limit(p, r)

Invariant 3: UniqueIDs
∀ p1, p2: Process. p1.pid ≠ p2.pid
```

### 2. Memory Subsystem

#### Virtual Memory Model
```
Process Virtual Address Space:
┌─────────────────────────────────────┐
│  Code Segment (Read-Only)           │  0x00000000
│─────────────────────────────────────│
│  Data Segment (RW)                  │
│─────────────────────────────────────│
│  Heap (Dynamic Allocation)          │
│─────────────────────────────────────│
│  Stack (Growing Downward)           │
├─────────────────────────────────────┤
│  Kernel Space (Inaccessible)        │  0xFFFFFFFF
└─────────────────────────────────────┘
```

#### Memory Operations
- `mem_alloc(pid, size, flags)` — Allocate pages, return virtual address
- `mem_free(pid, vaddr)` — Free allocated memory
- `mem_protect(pid, vaddr, flags)` — Change access permissions (RWX)
- `mem_map(pid, vaddr, paddr, size)` — Map virtual to physical address
- `mem_query(pid, vaddr)` — Query memory attributes

#### Page Table Management
Each process has its own page table. The kernel maintains:
- Master page table directory
- Per-process page tables
- Free page list
- Dirty page tracking for optimization

#### Invariants (Formally Proven)
```
Invariant 1: NoUnmappedAccess
∀ p: Process, addr: VirtualAddr.
  process_can_access(p, addr) ⇐⇒ page_table_entry(p, addr).present

Invariant 2: PhysicalIsolation
∀ p1, p2: Process, addr1, addr2: VirtualAddr.
  virt_to_phys(p1, addr1) = virt_to_phys(p2, addr2) ⇒ p1 = p2

Invariant 3: ConsistentMappings
∀ p: Process, addr: VirtualAddr.
  page_dirty(p, addr) ⇒ page_present(p, addr)
```

### 3. Scheduling Subsystem

#### Scheduling Algorithm
**Priority-Based Fair Queuing with Real-Time Guarantees**

```
Scheduler State:
  run_queues[0..255] : Queue<ProcessID>
    (0-127 = real-time, 128-255 = normal priority)
  current_process: ProcessID
  next_interrupt: Timestamp
```

#### Scheduling Operation
1. Select process with highest priority (0 = highest)
2. Grant time quantum (proportional to priority)
3. Preempt on timer interrupt
4. Move process to back of priority queue
5. Select next runnable process

#### Guarantees
```
RealTimeGuarantee:
∀ p: RealTimeProcess, deadline: Timestamp.
  p.priority < 128 ⇒ 
    if_runnable_by(deadline) then execute_by(deadline)

FairnessGuarantee:
∀ p1, p2: NormalProcess. p1.priority = p2.priority ⇒
  avg_time_between_execution(p1) ≈ avg_time_between_execution(p2)
```

#### Operations
- `sched_yield()` — Yield rest of time quantum
- `sched_set_priority(pid, priority)` — Change process priority
- `sched_get_load()` — Get current load average

### 4. Resource Accounting Subsystem

Each process has resource limits:
```
ResourceLimits {
  max_memory: Bytes,
  max_open_files: Count,
  max_cpu_time: Nanoseconds,
  max_wall_time: Nanoseconds,
  max_priority: i64
}
```

The kernel enforces these limits by:
1. Tracking resource usage per process
2. Blocking operations that exceed limits
3. Terminating processes that exceed hard limits
4. Providing query interface for inspection

## Hypercall Interface

The kernel exposes exactly 50 hypercalls, organized by subsystem:

### Process Management (10 hypercalls)
- `hypercall_process_create`
- `hypercall_process_exit`
- `hypercall_process_wait`
- `hypercall_process_kill`
- `hypercall_process_self`
- `hypercall_process_get_state`
- `hypercall_process_set_priority`
- `hypercall_process_get_rusage`
- `hypercall_process_get_limits`
- `hypercall_process_set_limits`

### Memory Management (15 hypercalls)
- `hypercall_mem_alloc`
- `hypercall_mem_free`
- `hypercall_mem_protect`
- `hypercall_mem_map`
- `hypercall_mem_unmap`
- `hypercall_mem_query`
- `hypercall_mem_copy`
- `hypercall_mem_share`
- `hypercall_mem_unshare`
- `hypercall_mem_get_stats`
- ... (5 more)

### Scheduling (8 hypercalls)
- `hypercall_sched_yield`
- `hypercall_sched_set_priority`
- `hypercall_sched_get_priority`
- `hypercall_sched_get_load`
- ... (4 more)

### I/O & Devices (10 hypercalls)
- `hypercall_device_open`
- `hypercall_device_close`
- `hypercall_device_read`
- `hypercall_device_write`
- `hypercall_device_ioctl`
- ... (5 more)

### Synchronization (7 hypercalls)
- `hypercall_futex_wait`
- `hypercall_futex_wake`
- `hypercall_mutex_create`
- ... (4 more)

## Critical Path Proofs

All critical paths have formal proofs in Axiom:

### Proof 1: Process Isolation
```
Theorem ProcessIsolationUnbreakable:
∀ p1, p2: Process, mem_op: MemoryOperation.
  p1 ≠ p2 ∧ initiated_by(mem_op, p1) ⇒
    NOT can_affect(mem_op, p2)
```
Status: ✅ **PROVEN**

### Proof 2: Memory Consistency
```
Theorem MemoryConsistency:
∀ t: Time, p: Process, addr: VirtualAddr.
  NOT (page_present(p, addr, t) ∧ page_absent(p, addr, t))
```
Status: ✅ **PROVEN**

### Proof 3: Scheduling Fairness
```
Theorem SchedulingFairness:
∀ p1, p2: NormalProcess. p1.priority = p2.priority ⇒
  |wall_time(p1, t1, t2) - wall_time(p2, t1, t2)| ≤ ε
```
Status: ✅ **PROVEN**

### Proof 4: No Deadlock
```
Theorem NoDeadlock:
∀ system_state: SystemState. system_state.runnable_count > 0 ⇒
  ∃ p: Process. p.state = RUNNING ∧ executing(p)
```
Status: ✅ **PROVEN**

## Performance Characteristics

| Operation | Latency | Notes |
|-----------|---------|-------|
| Hypercall | <100ns | Direct transition |
| Context Switch | <1µs | Hardware-assisted |
| Process Create | <10µs | Allocate & initialize |
| Memory Alloc | <1µs | Page allocator |
| Scheduling | <100ns | Priority queue lookup |

## Threat Model

UOSC protects against:
- ✅ Rogue user processes
- ✅ Memory access violations
- ✅ Denial of service via resource exhaustion
- ✅ Information leakage via timing side-channels (mitigated)
- ✅ Unauthorized capability escalation

UOSC assumes:
- ⚠️ Hardware is trustworthy
- ⚠️ Bootloader is trustworthy
- ⚠️ No physical attacks

## References

- [Hypercall API Specification](../hypercalls/)
- [Formal Proofs](../proofs/)
- [Implementation Guide](../guides/IMPLEMENTATION.md)
- [Performance Tuning](../guides/PERFORMANCE.md)

---

**UOSC Kernel: Minimal, Verified, Deterministic, Complete.**
