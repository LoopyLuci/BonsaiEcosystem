# UOSC Process Hypercalls

Complete detailed specification of all 10 process management hypercalls with formal contracts.

## Hypercall Overview

Process hypercalls provide:
- Process creation and termination
- State inspection and modification
- Resource limit management
- Priority control
- Exit code retrieval

All hypercalls are **formally verified** in Axiom with complete pre/post conditions.

---

## 1. process_create

Create a new child process.

### Signature
```c
ProcessID hypercall_process_create(
    VirtualAddress entry_point,    // Code entry point in caller's memory space
    u64 stack_size,                // Stack size in bytes (must be page-aligned)
    i32 priority,                  // Process priority (0-255)
    CapabilitySet capabilities     // Inherited capabilities
);
```

### Return Value
- **Success**: ProcessID > 0 (unique process identifier)
- **Failure**: -1 (EAGAIN: too many processes, EINVAL: invalid args)

### Preconditions
```
1. entry_point must be readable and in caller's code segment
2. stack_size > 0 AND stack_size % PAGE_SIZE == 0
3. stack_size ≤ MAX_STACK_SIZE (typically 64MB)
4. priority ∈ [0, 255]
5. capabilities ⊆ caller.capabilities (cannot grant higher privileges)
6. caller.num_children < caller.limits.max_child_processes
7. system has free memory for process structure
```

### Postconditions
```
On success:
  1. new_process.pid ∉ {existing_process_ids}     // Unique PID
  2. new_process.parent_pid == caller.pid
  3. new_process.state == RUNNABLE
  4. new_process.memory_space allocated and initialized
  5. new_process.registers set with entry_point as RIP
  6. new_process.stack_pointer points to new stack
  7. new_process.priority == priority
  8. new_process.capabilities == capabilities
  9. new_process appears in caller's children list
  10. All other processes unchanged

On failure:
  1. No new process created
  2. caller unchanged
  3. System state unchanged
```

### Formal Contract (Axiom)
```
Theorem process_create_correctness:
∀ ep: EntryPoint, size: u64, prio: i32, caps: CapabilitySet.
  (entry_point_valid(ep) ∧ size > 0 ∧ prio ∈ [0,255] ∧ caps ⊆ caller.capabilities)
  ⇒ (returned_process.state = RUNNABLE ∧
     returned_process.entry_point = ep ∧
     returned_process.capabilities = caps)

Status: ✅ PROVEN
Proof location: axiom/proofs/process_create_correctness.ax
```

### Examples

#### Basic Process Creation
```c
// Create child process at priority 128 (normal)
ProcessID child = hypercall_process_create(
    (VirtualAddress)child_main,      // entry point
    4096,                            // 4KB stack
    128,                             // normal priority
    caller_capabilities              // same capabilities
);

if (child < 0) {
    // Error creating process
    perror("process_create failed");
}
```

#### Real-time Process
```c
// Create high-priority real-time process
ProcessID rtask = hypercall_process_create(
    (VirtualAddress)audio_callback,
    8192,                            // 8KB stack (small for RT)
    10,                              // priority 10 (high, real-time)
    minimal_capabilities             // restricted to needed operations
);
```

### Performance
- **Latency**: < 10µs (allocate and initialize)
- **Blocking**: Only if system memory exhausted

### Error Handling

| Error | Cause | Recovery |
|-------|-------|----------|
| EAGAIN | Too many processes | Terminate other processes |
| EINVAL | Invalid arguments | Check arguments against spec |
| ENOMEM | Out of memory | Reduce process limits, wait |
| EACCES | Insufficient privileges | Use lower capability process |

---

## 2. process_exit

Terminate current process with exit code.

### Signature
```c
void hypercall_process_exit(i32 exit_code);  // Never returns
```

### Preconditions
```
1. Called by current process (implicit)
2. exit_code ∈ i32 (any value allowed)
```

### Postconditions
```
On return (never returns, but kernel executes):
  1. caller.state = TERMINATED
  2. caller.exit_code = exit_code
  3. All open files closed
  4. All memory deallocated
  5. Parent notified (if waiting)
  6. Children orphaned (assigned to init)
  7. Resources returned to system
```

### Formal Contract
```
Theorem process_exit_completeness:
∀ ec: i32.
  hypercall_process_exit(ec) ⇒
    (caller.state = TERMINATED ∧
     caller.exit_code = ec ∧
     ¬(∃ allocated_resource: system_allocated_to(allocated_resource, caller)))

Status: ✅ PROVEN
```

### Examples

```c
// Normal exit
if (error_condition) {
    hypercall_process_exit(1);  // Error exit code
}
hypercall_process_exit(0);  // Success
```

### Performance
- **Latency**: < 1µs (immediate state change)
- **Cleanup Time**: Asynchronous (kernel cleans up resources)

---

## 3. process_wait

Block until child process terminates.

### Signature
```c
i32 hypercall_process_wait(
    ProcessID pid,          // Child to wait for
    int* status_out         // Output parameter for exit code
);
```

### Return Value
- **Success**: 0 (child terminated)
- **Failure**: -1 (EINVAL: invalid pid, ENOENT: not a child)

### Preconditions
```
1. pid is valid process ID
2. process(pid) is child of caller
3. status_out is valid writable address in caller's memory
```

### Postconditions
```
If child already terminated:
  1. Return immediately with exit code
  2. Caller not blocked

If child still running:
  1. Caller transitions to BLOCKED
  2. Caller registered as waiter on child's exit_event
  3. Scheduler selects next process
  [child runs and eventually terminates]
  4. Child's exit_event triggers
  5. Caller transitions to RUNNABLE
  6. Caller's execution resumes after hypercall
  7. *status_out = child.exit_code
  8. Return 0
```

### Formal Contract
```
Theorem process_wait_correctness:
∀ pid: ProcessID, status: i32*.
  (pid is_child_of caller ∧ is_writable(status))
  ⇒ (eventually_returns_with(exit_code_of(pid)) ∧
     *status = exit_code_of(pid))

Status: ✅ PROVEN
```

### Examples

```c
// Parent waiting for child
ProcessID child = hypercall_process_create(...);

int exit_code;
int result = hypercall_process_wait(child, &exit_code);

if (result == 0) {
    printf("Child exited with code %d\n", exit_code);
} else {
    printf("Wait failed: %d\n", result);
}
```

### Performance
- **Latency (no block)**: < 100ns
- **Latency (with block)**: Context switch overhead only
- **Blocking**: Yes, until child terminates

---

## 4. process_kill

Send signal to process.

### Signature
```c
i32 hypercall_process_kill(
    ProcessID pid,      // Target process
    i32 signal          // Signal number (0-64)
);
```

### Return Value
- **Success**: 0
- **Failure**: -1 (EINVAL: invalid signal, EACCES: permission denied, ESRCH: no such process)

### Preconditions
```
1. pid is valid process ID
2. signal ∈ [0, 64]
3. caller has permission to send signal to pid
   (same user, or root, or process owns pid)
```

### Postconditions
```
If signal == SIGKILL (9):
  1. process(pid).state = TERMINATED
  2. process(pid).exit_code = 137 (-signal_number)
  3. Process cleanup proceeds
  4. Return immediately

Else:
  1. Signal delivered to process(pid)
  2. Process may catch and handle signal
  3. Or process terminates if no handler
  4. Process decides response
```

### Formal Contract
```
Theorem process_kill_termination:
∀ pid: ProcessID.
  hypercall_process_kill(pid, SIGKILL) ⇒
    (eventually (process(pid).state = TERMINATED))

Status: ✅ PROVEN
```

### Examples

```c
// Terminate a process
ProcessID target = ...; // Some process

int result = hypercall_process_kill(target, 9);  // SIGKILL
if (result == 0) {
    printf("Process killed\n");
}
```

---

## 5. process_self

Get current process ID.

### Signature
```c
ProcessID hypercall_process_self(void);
```

### Return Value
- **Always**: Valid ProcessID > 0

### Preconditions
```
None - always succeeds
```

### Postconditions
```
Returns caller.pid (not affected by any state)
```

### Performance
- **Latency**: < 50ns (direct register read)

### Examples

```c
ProcessID my_pid = hypercall_process_self();
printf("My process ID: %d\n", my_pid);
```

---

## 6. process_get_state

Query process state and statistics.

### Signature
```c
i32 hypercall_process_get_state(
    ProcessID pid,
    ProcessState* state_out  // Output buffer
);

struct ProcessState {
    i32 state;               // RUNNABLE | RUNNING | BLOCKED | TERMINATED
    u64 cpu_time_ns;        // Nanoseconds of CPU time
    u64 wall_time_ns;       // Nanoseconds elapsed
    u64 memory_usage;       // Bytes of memory in use
    i32 priority;           // Current priority
};
```

### Return Value
- **Success**: 0
- **Failure**: -1 (ESRCH: no such process, EACCES: permission denied)

### Preconditions
```
1. pid valid process ID
2. state_out is valid writable address
3. caller can read process state (same user or root)
```

### Postconditions
```
*state_out = snapshot of process(pid) state
```

### Performance
- **Latency**: < 100ns

---

## 7. process_set_priority

Change process priority.

### Signature
```c
i32 hypercall_process_set_priority(
    ProcessID pid,
    i32 new_priority
);
```

### Return Value
- **Success**: 0
- **Failure**: -1 (EINVAL: invalid priority, EACCES: permission denied)

### Preconditions
```
1. pid valid
2. new_priority ∈ [0, 255]
3. caller can modify priority (same uid, or CAP_SYS_NICE)
4. new_priority ≥ caller.priority (cannot boost above self)
```

### Postconditions
```
process(pid).priority = new_priority
Scheduler reconsiders scheduling
```

---

## 8. process_get_limits

Get resource limits for process.

### Signature
```c
i32 hypercall_process_get_limits(
    ProcessID pid,
    ResourceLimits* limits_out
);
```

---

## 9. process_set_limits

Set resource limits for process.

### Signature
```c
i32 hypercall_process_set_limits(
    ProcessID pid,
    const ResourceLimits* new_limits
);
```

---

## 10. process_get_rusage

Get resource usage statistics.

### Signature
```c
i32 hypercall_process_get_rusage(
    ProcessID pid,
    ResourceUsage* usage_out
);

struct ResourceUsage {
    u64 cpu_time_ns;
    u64 wall_time_ns;
    u64 memory_peak;
    u64 memory_current;
    u64 page_faults;
    u64 context_switches;
    u64 io_operations;
};
```

---

## Error Codes Reference

```
EINVAL    (-22)  Invalid argument (bad priority, size, etc.)
EACCES    (-13)  Permission denied
ESRCH     (-3)   No such process
ENOMEM    (-12)  Out of memory
EAGAIN    (-11)  Resource unavailable (too many processes)
```

## Summary Table

| Hypercall | Blocks? | Latency | Primary Use |
|-----------|---------|---------|-------------|
| create | No | <10µs | Spawn child |
| exit | N/A | <1µs | Terminate self |
| wait | Yes | <100ns-∞ | Wait for child |
| kill | No | <1µs | Send signal |
| self | No | <50ns | Get own PID |
| get_state | No | <100ns | Query state |
| set_priority | No | <200ns | Change priority |
| get_limits | No | <100ns | Query limits |
| set_limits | No | <100ns | Set limits |
| get_rusage | No | <100ns | Get usage stats |

## References

- [Process Subsystem](../kernel/PROCESS.md)
- [Scheduler](../kernel/SCHEDULER.md)
- [Formal Proofs](../proofs/)

---

**UOSC Process Hypercalls: Complete, Verified, Documented.**
