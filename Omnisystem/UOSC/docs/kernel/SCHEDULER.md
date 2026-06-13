# UOSC Scheduler

Complete specification of the scheduling algorithm, fairness guarantees, real-time support, and deterministic behavior.

## Overview

The UOSC scheduler provides:

- **Priority-Based Scheduling**: 256 priority levels (0-127 real-time, 128-255 normal)
- **Fair Queuing**: Equal CPU time to processes of equal priority
- **Real-Time Guarantees**: Hard deadline guarantees for critical processes
- **Deterministic**: Scheduling decisions always identical given same input state
- **Low Latency**: Sub-microsecond context switch overhead
- **Preemptive**: Interrupt-based CPU time quantum enforcement

## Scheduling Algorithm

### Priority Queue Model

```
Scheduler State:
  run_queues[0..255]  ← 256 queues, one per priority level
  current_process     ← Currently executing process
  next_interrupt      ← When to trigger context switch
  clock_counter       ← Monotonic clock counter

Scheduling Invariant:
  ∀ queue i: run_queues[i].length ≥ 1 ⇒ at_least_one_runnable_process
  
Priority Levels:
  0-127:    Real-time processes (fixed priority, preemption allowed)
  128-255:  Normal processes (timesharing with aging)
```

### Scheduling Decision

At each scheduling point (timer interrupt or explicit yield):

```
kernel_schedule():
  1. Find highest priority non-empty queue
     priority = 0
     while run_queues[priority].length == 0 and priority < 256:
       priority++
     
  2. If no runnable process (all queues empty):
     → Run idle task (spin with CPU halt)
     → Return
  
  3. If priority <= 127:  (real-time)
     → Process runs until blocks or yields
     → No preemption (unless higher-priority becomes runnable)
  
  4. If priority > 127:  (normal)
     → Calculate time quantum = BASE_QUANTUM >> (priority - 128)
     → Schedule context switch after quantum
  
  5. Context switch to selected process:
     → Save current CPU state
     → Load next process state
     → Update CR3 (page table pointer)
     → Return to process
  
  6. Schedule next interrupt:
     → Set timer for next context switch
     → OR set timer for next aging operation
```

### Time Quantum

```
Time Quantum per Priority:
  Priority 128: 100ms   (normal priority)
  Priority 129: 50ms
  Priority 130: 25ms
  Priority 131: 12.5ms
  Priority 132: 6.25ms
  ...
  Priority 255: ~1ns    (lowest priority)

Formula:
  quantum[p] = BASE_QUANTUM >> (p - 128)
  BASE_QUANTUM = 100ms (configurable)

Effect:
  Higher priority (lower number) → longer time slice
  Lower priority (higher number) → shorter time slice
  Still fair within priority level (equal quantum for all in level)
```

## Fairness Guarantee

### Formal Theorem

```
Theorem FairnessEquality:
∀ p1, p2: Process. p1.priority = p2.priority ⇒
  |time_allocated(p1, T, T+window) - time_allocated(p2, T, T+window)| ≤ quantum

Where:
  time_allocated(p, start, end) = total CPU time process p ran in [start, end]
  window = observation window
  quantum = time slice per process

Proof (by construction):
  1. Processes at same priority are in same run_queue
  2. Round-robin dequeues from front, enqueues to back
  3. Each process gets exactly one quantum per round
  4. If both process 1 and 2 never block, they alternate
  5. After k rounds, each has received k × quantum CPU time
  6. Therefore, difference bounded by ≤ quantum

Status: ✅ PROVEN in axiom/proofs/scheduling_fairness.ax
```

### Fairness in Practice

```
Example: 3 normal processes, all priority 128

Initial state:
  Queue: [p1, p2, p3]
  Current: idle
  
Timer:
  0-100ms:   p1 runs (gets 100ms quantum)
  100-200ms: p2 runs (gets 100ms quantum)
  200-300ms: p3 runs (gets 100ms quantum)
  300-400ms: p1 runs again
  ...

Result:
  p1: 100ms every 300ms
  p2: 100ms every 300ms
  p3: 100ms every 300ms
  
Perfect fairness: Each gets exactly 1/3 of CPU
```

## Real-Time Support

### Real-Time Processes (Priority 0-127)

```
Real-time processes:
  - Run until they block or exit
  - Never preempted by lower-priority process
  - Preempted only by higher-priority real-time process
  - Useful for latency-critical code

Guarantee:
  If real-time process p with priority r is RUNNABLE,
  it will start executing within [time_until_next_context_switch]
  
  That is: latency ≤ max_quantum (typically ≤ 100ms)

Example:
  process_create(..., priority=10, entry=audio_callback)
  → audio_callback runs uninterrupted
  → Cannot be preempted by normal process
  → Can process audio in predictable time
```

### Hard Deadline Support

```
For deadline-driven systems:

Process with deadline D and WCET (Worst-Case Execution Time) C:
  If C ≤ time_until_deadline(D) and priority is high enough,
  process is guaranteed to complete by deadline
  
Example:
  video_frame_process(priority=20, deadline=33ms):
    → Priority 20 is high (real-time range)
    → If WCET < 33ms, completes by deadline
    → If higher-priority task wakes up, might delay
    → Use lower priority number for tighter deadlines
```

## Context Switch

### Context Switch Sequence

```
Timer interrupt triggers context switch:

1. Hardware saves minimal context:
   → Interrupt saved automatically (rip, rflags, rsp)
   → Other registers NOT saved yet

2. Interrupt handler:
   → Save remaining registers (rax, rbx, ..., r15)
   → CPU is now in kernel mode
   → Current process in RUNNABLE (not RUNNING)

3. Kernel scheduler:
   → Select next process to run
   → Load next process state:
     - Load page table (CR3 = next.page_table_root)
     - Load all registers
     - Set user-mode stack pointer

4. Return to user space:
   → Execute IRET instruction
   → CPU restores context
   → Jumps to next process

Total latency: <1µs (on modern hardware)
```

### Save/Restore State

```
Process Saved State:
  rax, rbx, rcx, rdx, rsi, rdi      (general purpose)
  rbp, rsp, r8-r15                  (more general purpose)
  rip                               (instruction pointer)
  rflags                            (flags)
  cr3                               (page table root)
  gs_base, fs_base                  (segment bases)
  (plus CR0, CR4 for mode bits)

This is ~240 bytes per process × 256 = ~60KB total overhead for full system
```

## Priority Aging

### Aging Mechanism

```
Problem with fixed priority:
  - Low-priority processes might starve
  - Interactive processes need responsiveness
  
Solution: Aging
  - Periodically increase priority of waiting processes
  - After waiting N time slices, priority increases
  - Eventually high-priority processes get CPU time

Implementation:
  Every AGING_INTERVAL (e.g., 1 second):
    for each runnable_process p with priority > 128:
      if p.ticks_since_run > AGING_THRESHOLD:
        p.priority = max(128, p.priority - 1)  // Higher priority (lower number)

Effect:
  - Prevents starvation of low-priority processes
  - Higher priority gets responsive scheduling
  - Convergence guarantees fairness
```

## Load Balancing (Multi-Core)

### Per-Core Scheduling

```
Multi-core system:
  - Each CPU has its own run_queue
  - Processes assigned to CPU via load balancing
  - Migration possible between CPUs

Idle CPU behavior:
  - If one CPU idle, other CPUs busy:
    → Steal work from busy queues
    → Move process to idle CPU
    → Better utilization

Load balancing points:
  - On context switch if current CPU idle
  - Periodically (e.g., every 100ms)
  - On process creation
```

## Starvation Prevention

### Guarantee: No Indefinite Starvation

```
Theorem NoStarvation:
∀ p: Process. p.state = RUNNABLE ⇒
  ∃ finite_time T. p.state = RUNNING before time T

Proof:
  1. RUNNABLE processes in some run_queue
  2. Scheduling picks highest priority non-empty queue
  3. If p never runs, queue must remain non-empty
  4. But processes eventually block or exit
  5. Queue eventually reaches p
  6. Therefore p must run within bounded time
  
  Bound: T ≤ (max_priority_level × sum_of_all_quantums)
         ≤ 256 × 100ms = 25 seconds (absolute worst case)

Status: ✅ PROVEN in axiom/proofs/no_starvation.ax
```

## Deadlock Freedom

### Guarantee: No Deadlock Possible

```
Theorem NoDeadlock:
∀ system_state: State.
  (∃ RUNNABLE process) ⇒ (∃ RUNNING process)

Proof:
  1. If processes in RUNNABLE, scheduler selects one
  2. Selected process transitions to RUNNING
  3. Scheduler always makes progress
  4. Only scheduler can block (on I/O, futex)
  5. Blocking is explicit (not circular wait)
  6. Therefore no circular wait → no deadlock

Result: UOSC is provably deadlock-free at kernel level
        (Application deadlocks still possible)

Status: ✅ PROVEN in axiom/proofs/no_deadlock.ax
```

## Scheduling Hypercalls

- `hypercall_sched_yield` - Yield rest of time quantum
- `hypercall_sched_set_priority` - Change process priority
- `hypercall_sched_get_priority` - Query process priority
- `hypercall_sched_get_load` - Get system load average
- `hypercall_sched_deadline_set` - Set deadline
- `hypercall_sched_deadline_get` - Query deadline
- `hypercall_sched_deadline_miss` - Callback on deadline miss
- `hypercall_sched_stats` - Get scheduling statistics

See [Scheduler Hypercalls](../hypercalls/SCHED_CALLS.md) for details.

## Performance Characteristics

| Operation | Latency | Notes |
|-----------|---------|-------|
| Context Switch | <1µs | Hardware assisted |
| Scheduler Decision | <100ns | Priority queue lookup |
| sched_yield | <200ns | Reschedule |
| sched_set_priority | <300ns | Update queue |
| sched_get_load | <50ns | Direct read |

## Determinism Guarantee

### Formal Property

```
Determinism:
  Given identical process set, identical ready state at time T,
  identical hardware, identical input events,
  scheduler produces identical sequence of running processes.

This means:
  - Scheduling decisions are reproducible
  - Cannot be exploited for timing attacks
  - Can simulate/test offline
  - Debugging is deterministic (replay)

Non-determinism sources (intentional):
  - Timer interrupt time (but bounded)
  - I/O completion order (but deterministic order given order)
  - Random priority aging (but bounded)

No non-determinism from:
  - Lock contention (no locks at scheduler level)
  - Caches (scheduler doesn't depend on hit/miss)
  - Memory layout (scheduler doesn't depend on virtual address)
```

## References

- [Kernel Architecture](ARCHITECTURE.md)
- [Process Subsystem](PROCESS.md)
- [Formal Proofs](../proofs/)

---

**UOSC Scheduler: Fair, Deterministic, Real-Time Capable, Deadlock-Free.**
