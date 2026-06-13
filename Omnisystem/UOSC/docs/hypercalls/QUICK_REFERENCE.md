# UOSC Hypercall Quick Reference

Quick lookup guide for all 50 UOSC hypercalls organized by category.

## Process Management (10 calls)

```c
// Create new process
ProcessID process_create(
    VirtualAddress entry_point,
    u64 stack_size,
    i32 priority,
    CapabilitySet capabilities
);

// Terminate current process
void process_exit(i32 exit_code);

// Wait for child to terminate
i32 process_wait(ProcessID pid, int* status_out);

// Send signal to process
i32 process_kill(ProcessID pid, i32 signal);

// Get current process ID
ProcessID process_self(void);

// Query process state
i32 process_get_state(ProcessID pid, ProcessState* state_out);

// Change process priority
i32 process_set_priority(ProcessID pid, i32 new_priority);

// Get resource limits
i32 process_get_limits(ProcessID pid, ResourceLimits* limits_out);

// Set resource limits
i32 process_set_limits(ProcessID pid, const ResourceLimits* new_limits);

// Get resource usage
i32 process_get_rusage(ProcessID pid, ResourceUsage* usage_out);
```

## Memory Management (15 calls)

```c
// Allocate virtual memory
VirtualAddress mem_alloc(u64 size, i32 flags);

// Deallocate memory
i32 mem_free(VirtualAddress address, u64 size);

// Change memory protection
i32 mem_protect(VirtualAddress address, u64 size, i32 prot);

// Query memory info
i32 mem_query(VirtualAddress address, MemoryInfo* info_out);

// Map physical memory
i32 mem_map(VirtualAddress virt, PhysicalAddress phys, u64 size, i32 flags);

// Unmap physical memory
i32 mem_unmap(VirtualAddress address, u64 size);

// Copy between processes
i32 mem_copy(ProcessID dest_pid, VirtualAddress dest, VirtualAddress src, u64 size);

// Share memory with process
i32 mem_share(ProcessID target_pid, VirtualAddress address, u64 size, i32 prot);

// End memory sharing
i32 mem_unshare(ProcessID target_pid, VirtualAddress address, u64 size);

// Get memory statistics
i32 mem_stats(ProcessID pid, MemoryStats* stats_out);

// Lock memory in RAM
i32 mem_pin(VirtualAddress address, u64 size);

// Unlock memory
i32 mem_unpin(VirtualAddress address, u64 size);

// Allocate physical pages
i32 mem_commit(VirtualAddress address, u64 size);

// Release physical pages
i32 mem_decommit(VirtualAddress address, u64 size);

// Provide memory hints
i32 mem_advise(VirtualAddress address, u64 size, i32 advice);
```

## Device I/O (10 calls)

```c
// Open device file
FileDescriptor device_open(const char* path, i32 flags);

// Close device
i32 device_close(FileDescriptor fd);

// Read from device
i32 device_read(FileDescriptor fd, void* buffer, u32 count);

// Write to device
i32 device_write(FileDescriptor fd, const void* buffer, u32 count);

// Device control operations
i32 device_ioctl(FileDescriptor fd, u32 command, void* args);

// Wait for device event
i32 device_poll(FileDescriptor fd, i32 events, i32 timeout_ms);

// Map device memory
VirtualAddress device_map(FileDescriptor fd, u64 offset, u64 size, i32 prot);

// Unmap device memory
i32 device_unmap(VirtualAddress address, u64 size);

// Synchronize with device
i32 device_sync(FileDescriptor fd, i32 flags);

// Get device status
i32 device_stat(FileDescriptor fd, DeviceStat* stat_out);
```

## Scheduling (8 calls)

```c
// Yield rest of time quantum
i32 sched_yield(void);

// Get current process priority
i32 sched_get_priority(ProcessID pid);

// Get system load average
f64 sched_get_load(void);

// Set process deadline
i32 sched_deadline_set(ProcessID pid, u64 deadline_ns);

// Get process deadline
i32 sched_deadline_get(ProcessID pid);

// Handle deadline miss
i32 sched_deadline_miss(ProcessID pid);

// Get scheduling statistics
i32 sched_stats(SchedStats* stats_out);

// Pre-allocate scheduler resources
i32 sched_prepare(u32 num_processes);
```

## Synchronization (7 calls)

```c
// Block on futex
i32 futex_wait(i32* futex_addr, i32 expected_value, i64 timeout_ns);

// Wake futex waiters
i32 futex_wake(i32* futex_addr, u32 num_wake);

// Create mutex
Mutex* mutex_create(i32 type);

// Acquire mutex
i32 mutex_lock(Mutex* mutex);

// Release mutex
i32 mutex_unlock(Mutex* mutex);

// Destroy mutex
i32 mutex_destroy(Mutex* mutex);

// Memory barrier
void memory_barrier(i32 type);
```

## Common Patterns

### Process Creation and Waiting
```c
ProcessID child = process_create(child_main, 4096, 128, CAP_ALL);
int status;
process_wait(child, &status);
if (status == 0) printf("Child succeeded\n");
```

### Protected Critical Section
```c
Mutex* lock = mutex_create(MUTEX_NORMAL);
mutex_lock(lock);
// ... critical section ...
mutex_unlock(lock);
mutex_destroy(lock);
```

### Shared Memory Between Processes
```c
void* shared = mem_alloc(4096, 0);
mem_share(child_pid, shared, 4096, PROT_READ | PROT_WRITE);
// Both processes can access shared memory
```

### Device I/O
```c
FileDescriptor fd = device_open("/dev/console", O_WRITE);
device_write(fd, "Hello", 5);
device_close(fd);
```

### Wait for Device Ready
```c
int ready = device_poll(fd, POLL_READ, 1000);
if (ready & POLL_READ) {
    device_read(fd, buffer, 1024);
}
```

### Memory-Mapped I/O
```c
void* regs = device_map(fd, 0, 4096, PROT_READ | PROT_WRITE);
volatile u32* control = (volatile u32*)regs;
*control = 0x1234;
```

## Error Codes

```c
EINVAL     (-22)  Invalid argument
EACCES     (-13)  Permission denied
ESRCH      (-3)   No such process
ENOMEM     (-12)  Out of memory
EAGAIN     (-11)  Resource temporarily unavailable
EBADF      (-9)   Bad file descriptor
ENOENT     (-2)   No such file/device
EFAULT     (-14)  Bad address
EBUSY      (-16)  Device/resource busy
ETIMEDOUT  (-110) Operation timed out
EDEADLK    (-35)  Would cause deadlock
EPERM      (-1)   Operation not permitted
```

## Flags and Constants

### Memory Allocation Flags
```c
MEM_ZERO         // Zero-initialize pages
MEM_FIXED        // Allocate at specific address
MEM_HUGE         // Use huge pages (2MB)
MEM_LOCKED       // Lock in physical memory
MEM_EXECUTABLE   // Allocate executable memory
```

### Protection Flags
```c
PROT_NONE   // No access
PROT_READ   // Read access
PROT_WRITE  // Write access
PROT_EXEC   // Execute access
```

### Memory Advice
```c
MEM_WILLNEED    // Will access soon, preload
MEM_DONTNEED    // Won't access, can discard
MEM_SEQUENTIAL  // Sequential access expected
MEM_RANDOM      // Random access expected
```

### Device I/O Flags
```c
O_READ          // Read access
O_WRITE         // Write access
O_RDWR          // Read and write
O_NONBLOCK      // Non-blocking I/O
O_EXCL          // Exclusive access
```

### Poll Events
```c
POLL_READ       // Data available to read
POLL_WRITE      // Ready to write data
POLL_ERROR      // Error condition
```

### Priority Ranges
```
0-127:    Real-time (fixed priority, no preemption)
128-255:  Normal (timesharing)
```

## Performance Targets

| Operation | Target | Typical |
|-----------|--------|---------|
| process_create | <10µs | 5µs |
| process_exit | <1µs | 0.5µs |
| mem_alloc | <5µs | 2µs |
| mem_protect | <2µs | 1µs |
| Context switch | <1µs | 0.8µs |
| Scheduler decision | <100ns | 95ns |
| futex_wait | <100ns | 50ns |
| device_read | <5µs | 2µs |

## Important Notes

1. **Capability-Based Security**: Many operations require specific capabilities
2. **Non-Blocking**: Most hypercalls return immediately; some block (futex_wait, process_wait, device_read)
3. **Formal Verification**: All 50 hypercalls formally proven correct
4. **Memory Safety**: No buffer overflows, use mem_query to verify addresses
5. **Deadlock-Free**: Kernel-level deadlock impossible (user-level still possible)
6. **Deterministic Scheduling**: Identical inputs produce identical scheduling

## See Also

- [Process Calls](PROCESS_CALLS.md) - Detailed process hypercall documentation
- [Memory Calls](MEMORY_CALLS.md) - Detailed memory hypercall documentation
- [Device Calls](DEVICE_CALLS.md) - Detailed device hypercall documentation
- [Sync Calls](SYNC_CALLS.md) - Detailed synchronization hypercall documentation
- [Full Specification](SPECIFICATION.md) - Complete hypercall specification

---

**UOSC Quick Reference: Essential, Concise, Always Accessible.**
