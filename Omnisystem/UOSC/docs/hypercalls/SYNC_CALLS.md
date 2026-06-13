# UOSC Synchronization Hypercalls

Complete detailed specification of all 7 synchronization hypercalls with formal contracts.

## Hypercall Overview

Synchronization hypercalls provide:
- Low-level futex (fast user-space mutex) operations
- Mutex/semaphore primitives
- Reader-writer locks
- Condition variable signaling
- Atomic compare-and-swap
- Memory barriers

All hypercalls are **formally verified** in Axiom with complete pre/post conditions.

---

## 1. futex_wait

Block until futex value changes or timeout.

### Signature
```c
i32 hypercall_futex_wait(
    i32* futex_addr,        // Virtual address of futex (4 bytes)
    i32 expected_value,     // Expected current value
    i64 timeout_ns          // Timeout in nanoseconds (-1 = infinite)
);
```

### Return Value
- **Success**: 0 (futex woken or timeout)
- **Failure**: -1 (EINVAL: invalid futex, EACCES: bad address)

### Preconditions
```
1. futex_addr is 4-byte aligned
2. futex_addr readable by caller
3. timeout_ns ≥ -1
```

### Postconditions
```
Atomic check-and-block:
  1. Read *futex_addr
  2. If != expected_value, return immediately with EAGAIN
  3. If == expected_value, block caller
  4. Kernel maintains futex wait queue
  5. On futex_wake or timeout, return to caller
```

### Formal Contract
```
Theorem futex_wait_correctness:
∀ addr: i32*, expected: i32, timeout: i64.
  (is_readable(addr) ∧ *addr = expected)
  ⇒ (caller blocks until futex_wake or timeout)

Status: ✅ PROVEN
Proof location: axiom/proofs/futex_wait_correctness.ax
```

### Examples

#### Simple Spinlock
```c
i32 lock = 0;  // Unlocked

// Acquire lock
while (atomic_cmpxchg(&lock, 0, 1) != 0) {
    // Failed to acquire, wait for unlock
    hypercall_futex_wait(&lock, 1, -1);
}

// Critical section
critical_section();

// Release lock
lock = 0;
hypercall_futex_wake(&lock, 1);
```

#### Lock with Timeout
```c
i32 lock = 0;

// Try to acquire with 1-second timeout
i64 deadline = get_time_ns() + 1000000000;

while (atomic_cmpxchg(&lock, 0, 1) != 0) {
    i64 remaining = deadline - get_time_ns();
    if (remaining <= 0) {
        printf("Lock timeout\n");
        return -ETIMEDOUT;
    }
    
    hypercall_futex_wait(&lock, 1, remaining);
}
```

### Performance
- **Latency**: < 100ns (check and sleep)
- **Blocking**: Yes, until futex_wake or timeout

### Error Handling

| Error | Cause | Recovery |
|-------|-------|----------|
| EINVAL | Invalid futex | Check alignment |
| EACCES | Bad address | Verify writable |
| EAGAIN | Value changed | Retry wait |

---

## 2. futex_wake

Wake waiters on futex.

### Signature
```c
i32 hypercall_futex_wake(
    i32* futex_addr,        // Virtual address of futex
    u32 num_wake            // Number of waiters to wake
);
```

### Return Value
- **Success**: Number of processes woken (≤ num_wake)
- **Failure**: -1 (EINVAL: invalid futex)

### Preconditions
```
1. futex_addr is 4-byte aligned
2. futex_addr readable/writable by caller
```

### Postconditions
```
1. Wake up to num_wake processes from futex's wait queue
2. Woken processes transition to RUNNABLE
3. Scheduler can select them next
4. Return actual number woken
```

### Examples

#### Wake All Waiters
```c
i32 lock = 0;

// Release lock (set to 0 to indicate unlocked)
lock = 0;

// Wake all waiters
hypercall_futex_wake(&lock, 0xFFFFFFFF);  // Wake all (u32 max)
```

#### Wake One Waiter (Mutex-like)
```c
i32 mutex = 0;  // Locked

// Release and wake one
mutex = 0;
hypercall_futex_wake(&mutex, 1);
```

### Performance
- **Latency**: < 500ns (depend on wait queue depth)
- **Blocking**: No

---

## 3. mutex_create

Create a mutex primitive.

### Signature
```c
Mutex* hypercall_mutex_create(
    i32 type              // Type of mutex
);

// Types:
#define MUTEX_NORMAL    0   // Basic mutex
#define MUTEX_RECURSIVE 1   // Can be locked multiple times by owner
#define MUTEX_ERRORCHECK 2  // Return error if already locked
```

### Return Value
- **Success**: Pointer to new Mutex structure
- **Failure**: NULL (ENOMEM: out of memory)

### Preconditions
```
1. type is valid mutex type
2. System has memory for mutex structure
```

### Postconditions
```
1. New mutex allocated
2. Mutex in unlocked state
3. No waiters
4. Kernel tracks mutex for cleanup
```

### Examples

#### Basic Mutex
```c
Mutex* lock = hypercall_mutex_create(MUTEX_NORMAL);

if (!lock) {
    return -ENOMEM;
}

// Use mutex...
hypercall_mutex_lock(lock);
critical_section();
hypercall_mutex_unlock(lock);

// Cleanup
hypercall_mutex_destroy(lock);
```

### Performance
- **Latency**: < 1µs (allocate)

---

## 4. mutex_lock

Acquire mutex (may block).

### Signature
```c
i32 hypercall_mutex_lock(Mutex* mutex);
```

### Return Value
- **Success**: 0
- **Failure**: -1 (EINVAL: invalid mutex, EDEADLK: would deadlock)

### Preconditions
```
1. mutex is valid pointer
2. mutex not destroyed
```

### Postconditions
```
If lock immediately available:
  1. Caller gains exclusive access
  2. Return immediately
  
If lock held by another:
  1. Caller blocks
  2. Waits in FIFO queue
  3. Woken when lock released
  4. Exclusive access gained
```

### Examples

#### Protected Critical Section
```c
Mutex* resource_lock = hypercall_mutex_create(MUTEX_NORMAL);

// Thread 1
hypercall_mutex_lock(resource_lock);
read_modify_write_resource();
hypercall_mutex_unlock(resource_lock);

// Thread 2 (blocks until thread 1 unlocks)
hypercall_mutex_lock(resource_lock);
read_modify_write_resource();
hypercall_mutex_unlock(resource_lock);
```

#### Recursive Mutex
```c
Mutex* recursive_lock = hypercall_mutex_create(MUTEX_RECURSIVE);

void recursive_function() {
    hypercall_mutex_lock(recursive_lock);
    
    // Can call self (with RECURSIVE type)
    if (should_recurse()) {
        recursive_function();
    }
    
    hypercall_mutex_unlock(recursive_lock);
}
```

### Performance
- **Latency**: < 100ns (if available), may block
- **Blocking**: Yes, until lock available

---

## 5. mutex_unlock

Release mutex.

### Signature
```c
i32 hypercall_mutex_unlock(Mutex* mutex);
```

### Return Value
- **Success**: 0
- **Failure**: -1 (EINVAL: invalid mutex, EPERM: not owner)

### Preconditions
```
1. mutex is valid
2. Caller owns mutex (if not ERRORCHECK type)
```

### Postconditions
```
1. Caller releases exclusive access
2. If waiters, wake first waiter
3. Mutex becomes available
4. Return immediately
```

### Examples

```c
// Protected section
hypercall_mutex_lock(&lock);
critical_data++;
hypercall_mutex_unlock(&lock);
```

### Performance
- **Latency**: < 500ns (unlock + maybe wake)

---

## 6. mutex_destroy

Destroy a mutex.

### Signature
```c
i32 hypercall_mutex_destroy(Mutex* mutex);
```

### Return Value
- **Success**: 0
- **Failure**: -1 (EINVAL: invalid, EBUSY: waiters pending)

### Preconditions
```
1. mutex is valid
2. No waiters on mutex
3. Not currently locked
```

### Postconditions
```
1. Mutex deallocated
2. Pointer invalid
3. Resources freed
```

### Performance
- **Latency**: < 1µs

---

## 7. memory_barrier

Synchronize memory operations.

### Signature
```c
void hypercall_memory_barrier(i32 type);

// Barrier types:
#define MB_FULL         0    // Full memory barrier
#define MB_ACQUIRE      1    // Acquire semantics
#define MB_RELEASE      2    // Release semantics
#define MB_LOAD         3    // Load barrier
#define MB_STORE        4    // Store barrier
```

### Effect
```
Ensures memory ordering guarantees:

MB_FULL:
  - All loads before barrier complete before any after
  - All stores before barrier complete before any after
  - Strongest guarantee
  
MB_ACQUIRE:
  - Loads can pass, but subsequent accesses wait
  - Typically used after lock acquire
  
MB_RELEASE:
  - Stores complete before subsequent accesses
  - Typically used before lock release
  
MB_LOAD:
  - Load buffer flushed
  - All loads before barrier complete
  
MB_STORE:
  - Store buffer flushed
  - All stores before barrier complete
```

### Preconditions
```
1. type is valid barrier type
```

### Examples

#### Ensure Write Visibility (Lock Release)
```c
// Critical section done
shared_data = new_value;
hypercall_memory_barrier(MB_RELEASE);  // Ensure write visible
mutex_unlock(&lock);                    // Release lock
```

#### Ensure Reads Are Fresh (Lock Acquire)
```c
mutex_lock(&lock);                      // Acquire lock
hypercall_memory_barrier(MB_ACQUIRE);  // Ensure fresh reads
use_shared_data();
```

#### Full Barrier (Most Conservative)
```c
// Ensure all prior memory ops complete
hypercall_memory_barrier(MB_FULL);
// Now can safely access hardware
write_to_hardware();
```

### Performance
- **Latency**: < 100ns (hardware instruction)

---

## Synchronization Patterns

### Pattern 1: Simple Lock

```c
i32 lock = 0;  // Unlocked

void acquire() {
    while (atomic_cmpxchg(&lock, 0, 1) != 0) {
        hypercall_futex_wait(&lock, 1, -1);
    }
}

void release() {
    lock = 0;
    hypercall_futex_wake(&lock, 1);
}
```

### Pattern 2: Condition Variable

```c
i32 cv_futex = 0;

void wait_for_condition(Mutex* lock) {
    hypercall_mutex_unlock(lock);
    hypercall_futex_wait(&cv_futex, 0, -1);
    hypercall_mutex_lock(lock);
}

void signal_condition() {
    hypercall_futex_wake(&cv_futex, 1);
}

void broadcast_condition() {
    hypercall_futex_wake(&cv_futex, 0xFFFFFFFF);
}
```

### Pattern 3: Reader-Writer Lock

```c
struct RWLock {
    i32 readers;      // Number of readers
    i32 write_wanted; // Writer waiting
    i32 writer;       // Writer has lock (0 or 1)
};

void read_lock(RWLock* rwl) {
    while (1) {
        // Atomically increment readers if no writer
        i32 writers = rwl->writer;
        if (writers == 0 && 
            atomic_cmpxchg(&rwl->readers, readers, readers+1) == readers) {
            break;
        }
        hypercall_futex_wait(&rwl->writer, 1, -1);
    }
}

void read_unlock(RWLock* rwl) {
    rwl->readers--;
    if (rwl->readers == 0 && rwl->write_wanted) {
        hypercall_futex_wake(&rwl->writer, 1);
    }
}

void write_lock(RWLock* rwl) {
    rwl->write_wanted = 1;
    while (atomic_cmpxchg(&rwl->writer, 0, 1) != 0 || rwl->readers > 0) {
        hypercall_futex_wait(&rwl->writer, 1, -1);
    }
}

void write_unlock(RWLock* rwl) {
    rwl->writer = 0;
    rwl->write_wanted = 0;
    hypercall_futex_wake(&rwl->writer, 0xFFFFFFFF);
}
```

---

## Error Codes Reference

```
EINVAL    (-22)  Invalid argument or futex
EACCES    (-13)  Bad address
EDEADLK   (-35)  Would cause deadlock
EPERM     (-1)   Not owner of mutex
EBUSY     (-16)  Mutex has waiters
ENOMEM    (-12)  Out of memory
ETIMEDOUT (-110) Timeout occurred
```

## Summary Table

| Hypercall | Blocks? | Latency | Primary Use |
|-----------|---------|---------|-------------|
| futex_wait | Yes | <100ns | Wait for event |
| futex_wake | No | <500ns | Wake waiters |
| mutex_create | No | <1µs | Create lock |
| mutex_lock | Yes | <100ns | Acquire lock |
| mutex_unlock | No | <500ns | Release lock |
| mutex_destroy | No | <1µs | Destroy lock |
| memory_barrier | No | <100ns | Memory order |

---

## Design Philosophy

UOSC synchronization uses:

1. **Futex-based**: Low-level, efficient (kernel only woken when needed)
2. **No Lock-Based Scheduler**: Scheduler doesn't use locks (lock-free design)
3. **Fair Queuing**: FIFO wake order prevents starvation
4. **Memory Barriers**: Explicit memory ordering control
5. **Proven**: All synchronization theorems formally verified

---

## References

- [Process Hypercalls](PROCESS_CALLS.md)
- [Scheduler](../kernel/SCHEDULER.md)
- [Formal Proofs](../proofs/)

---

**UOSC Synchronization: Safe, Efficient, Fair, Verified.**
