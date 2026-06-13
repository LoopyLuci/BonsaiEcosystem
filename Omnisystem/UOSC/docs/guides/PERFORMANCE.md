# UOSC Performance Tuning Guide

Complete guide to understanding, measuring, and optimizing UOSC performance.

## Overview

UOSC performance is determined by:

- **Scheduling latency**: Time until process gets CPU
- **Context switch overhead**: Cost of switching between processes
- **Memory access latency**: Cache misses, TLB misses
- **Device I/O latency**: Hardware communication
- **Interrupt latency**: Time to handle interrupts

This guide covers measurement and optimization techniques.

## Performance Baselines

### Expected Performance

| Operation | Typical | Min | Max | Notes |
|-----------|---------|-----|-----|-------|
| Context Switch | 0.8µs | 0.5µs | 2µs | Hardware-dependent |
| Scheduler Decision | 95ns | 50ns | 200ns | Priority queue lookup |
| Hypercall Latency | <100ns | 50ns | 200ns | No blocking |
| mem_alloc (4KB) | 0.9µs | 0.5µs | 2µs | Page allocator |
| Memory Access (cached) | 5ns | 3ns | 10ns | L1 cache hit |
| Memory Access (L3) | 40ns | 30ns | 60ns | L3 cache hit |
| Memory Access (RAM) | 100ns | 80ns | 150ns | Main memory |
| Interrupt Latency | <1µs | 0.5µs | 3µs | Hardware-dependent |
| Process Creation | <10µs | 5µs | 20µs | Allocate and init |

### System Throughput

| Metric | Value |
|--------|-------|
| Processes | 65,536 max |
| Context Switches/sec | 1M+ (on modern CPU) |
| Hypercalls/sec | 10M+ (sustained) |
| Memory Operations/sec | 100M+ (memcpy throughput) |
| Interrupt Handler Rate | 100k+/sec |

## Measuring Performance

### Using Performance Counters

```bash
# Build with performance monitoring enabled
./build.sh --target=standalone --mode=full --with-profiling

# Run with sampling
./build/kernel/uosc-kernel --profile=cpu --profile-rate=1000

# Profile output: build/profiles/cpu.prof

# Analyze results
./tools/analyze_profile build/profiles/cpu.prof
```

### Built-in Benchmarks

```bash
# Run UOSC benchmark suite
./build/tests/benchmark_kernel

# Output example:
[BENCH] Context Switch Latency
  Min:  0.7µs
  Max:  2.1µs
  Avg:  0.85µs
  Samples: 1000000

[BENCH] Scheduler Decision Latency
  Min:  50ns
  Max:  300ns
  Avg:  95ns
  Samples: 10000000

[BENCH] Memory Allocation
  Min:  0.4µs
  Max:  5.2µs
  Avg:  0.9µs
  Samples: 100000

[BENCH] Page Table Lookup (TLB miss)
  Min:  1.2µs
  Max:  8.5µs
  Avg:  2.3µs
  Samples: 10000
```

### Custom Benchmarking

```c
#include <uosc/benchmark.h>

void benchmark_my_operation() {
    Benchmark bench = benchmark_create("my_operation", 1000000);
    
    for (u64 i = 0; i < benchmark_samples(&bench); i++) {
        benchmark_start(&bench);
        
        // Operation to measure
        my_operation();
        
        benchmark_stop(&bench);
    }
    
    benchmark_report(&bench);
    // Output:
    // my_operation: min=0.5µs max=2.3µs avg=0.8µs (1M samples)
}
```

## Performance Optimization Techniques

### 1. CPU Cache Optimization

#### Principle: Spatial and Temporal Locality

```c
// Good: Sequential access (cache line reuse)
for (i = 0; i < N; i++) {
    array[i] = process(array[i]);  // Sequential, prefetchable
}

// Bad: Random access (cache misses)
for (i = 0; i < N; i++) {
    array[random_index()] = process(...);  // Unpredictable
}
```

#### Data Structure Alignment

```c
// Good: Cache-line aligned (typically 64 bytes)
typedef struct __attribute__((aligned(64))) {
    u64 counter;
    u64 timestamp;
    // Padding fills to 64 bytes
} CacheLine;

// Bad: Unaligned, false sharing
typedef struct {
    u64 counter;  // Shares cache line with next field
    u64 other;    // False sharing if accessed by different CPUs
} Unaligned;
```

### 2. Scheduler Optimization

#### Priority Tuning

```c
// Real-time process: high priority (low number)
ProcessID audio = process_create(
    entry_audio,
    8192,
    10,  // priority 10 (real-time range)
    caps
);

// Normal process: lower priority
ProcessID worker = process_create(
    entry_worker,
    16384,
    128,  // priority 128 (normal range)
    caps
);

Result:
  - Audio process gets CPU first
  - Jitter is minimized for audio
  - Worker still gets fair CPU time
```

#### Time Quantum Tuning

```
Default: BASE_QUANTUM = 100ms

For interactive systems: BASE_QUANTUM = 10ms
  - Faster response
  - More context switches (overhead)
  - Better user experience

For batch systems: BASE_QUANTUM = 1000ms
  - Fewer context switches
  - Higher throughput
  - Less interactive response

For real-time: Don't use time quantum (priority 0-127)
```

### 3. Memory Optimization

#### Allocation Strategies

```c
// Good: Pre-allocate, reuse
Buffer* buf = buffer_create(4096);
for (int i = 0; i < 1000; i++) {
    process_with_buffer(buf);  // Reuse
}
buffer_destroy(buf);

// Bad: Allocate per iteration
for (int i = 0; i < 1000; i++) {
    Buffer* buf = buffer_create(4096);  // Allocate
    process_with_buffer(buf);
    buffer_destroy(buf);  // Deallocate
}
```

#### Working Set Optimization

```c
// Minimize working set size
const u64 WORKING_SET_TARGET = 4 * 1024 * 1024;  // 4MB

// Keep frequently accessed data compact
typedef struct {
    u64* hot_data;      // Frequently accessed
    u64* cold_data;     // Rarely accessed
} OptimizedStructure;

// NOT:
typedef struct {
    u64 hot1, cold1, hot2, cold2, hot3, cold3;  // Interleaved
} UnoptimizedStructure;
```

### 4. Interrupt Latency Optimization

#### Interrupt Handler Design

```c
// Good: Minimal work in handler, defer to thread
static i32 interrupt_handler(void* context) {
    Device* dev = (Device*)context;
    
    // Only read hardware state
    u32 status = *dev->status_reg;
    
    // Queue work for later
    queue_work(dev, status);
    
    // Clear interrupt
    *dev->control_reg |= CLEAR_INT;
    
    return 1;  // Handled quickly
}
```

#### Interrupt Coalescing

```c
// For network device: coalesce multiple interrupts
// Instead of: interrupt per packet
// Do: timer-based batching

typedef struct {
    u32 packets_pending;
    Timestamp last_interrupt;
    #define COALESCE_TIMEOUT_US 100
} InterruptCoalescing;

static i32 handle_rx_interrupt(void* context) {
    Device* dev = (Device*)context;
    dev->coalesce.packets_pending++;
    
    // Only signal if timeout expired
    if (now() - dev->coalesce.last_interrupt > COALESCE_TIMEOUT_US) {
        signal_ready();
        dev->coalesce.last_interrupt = now();
    }
    
    return 1;
}
```

### 5. TLB (Translation Lookaside Buffer) Optimization

#### Large Pages

```bash
# Enable 2MB/4MB large pages
./build.sh --target=standalone --with-large-pages

# In code: allocate large pages
Buffer* buf = buffer_create_large(256 * 1024 * 1024);  // 256MB, uses 2MB pages
```

Benefits:
- Fewer TLB entries needed
- Higher hit rate
- Better cache locality
- Especially effective for large working sets (>100MB)

### 6. Lock-Free Optimization

#### Using Atomic Operations

```c
// Good: Lock-free counter for high contention
typedef struct {
    AtomicU64 counter;
} LockFreeCounter;

void increment(LockFreeCounter* c) {
    atomic_increment(&c->counter);  // No lock
}

// vs.

typedef struct {
    Mutex mutex;
    u64 counter;
} LockedCounter;

void increment(LockedCounter* c) {
    mutex_lock(&c->mutex);
    c->counter++;
    mutex_unlock(&c->mutex);  // Lock overhead
}
```

Performance impact:
- Lock-free: ~5ns per operation
- Locked: ~100-500ns per operation
- 100x difference under contention!

## Real-World Optimization Example

### Problem: Slow I/O Path

```c
// Baseline: 5µs per operation
static i32 device_write(Device* dev, Buffer* data) {
    mutex_lock(dev->state_mutex);          // 200ns
    for (i = 0; i < size; i++) {
        *dev->data_reg = data[i];          // 100ns each
        while (!(*dev->status_reg & READY)); // Spin wait
    }
    mutex_unlock(dev->state_mutex);        // 50ns
    return size;
}
// Per-byte: 300ns + spin wait = SLOW
```

### Optimization 1: Reduce Lock Contention

```c
// Without per-operation synchronization (in interrupt handler)
static i32 interrupt_handler(void* ctx) {
    Device* dev = ctx;
    u8 byte = *dev->data_reg;
    ringbuffer_push(&dev->rx_queue, byte);  // Lock-free ring buffer
    return 1;
}

// Main write path (no lock needed for RX):
static i32 device_write(Device* dev, Buffer* data) {
    // No lock for write-only path
    for (i = 0; i < size; i++) {
        *dev->data_reg = data[i];
        while (!(*dev->status_reg & READY));
    }
    return size;
}
```

### Optimization 2: Batching

```c
// Before: Write byte-by-byte
device_write(dev, byte1); // 300ns
device_write(dev, byte2); // 300ns
device_write(dev, byte3); // 300ns
// Total: 900ns for 3 bytes

// After: Batch write
Buffer buf = buffer_create(3);
buffer_append(buf, byte1, 1);
buffer_append(buf, byte2, 1);
buffer_append(buf, byte3, 1);
device_write_batch(dev, &buf);  // 600ns for 3 bytes (200ns/byte)
// 33% faster!
```

### Optimization 3: DMA (for larger transfers)

```c
// For blocks > 64 bytes, use DMA instead of CPU write
static i32 device_write_large(Device* dev, Buffer* data) {
    if (data->size <= 64) {
        return device_write(dev, data);  // CPU write
    }
    
    // DMA write
    *dev->dma_src_addr = buffer_physical_addr(data);
    *dev->dma_size = data->size;
    *dev->dma_ctrl = START_DMA;
    
    while (!(*dev->status_reg & DMA_DONE));
    
    return data->size;
}

// Result: 10MB/sec throughput (vs 100KB/sec with CPU)
```

### Result

```
Before optimization:    300ns per byte
After lock removal:     250ns per byte (17% improvement)
After batching:         200ns per byte (33% improvement)
After DMA:              10MB/sec (2000x for large buffers)
```

## Profiling Tools

### flamegraph

```bash
# Generate CPU flame graph
./build.sh --with-flamegraph
./build/kernel/uosc-kernel --flamegraph=cpu
# View: build/profiles/flamegraph.html
```

### perf (Linux)

```bash
# Record performance data
perf record -F 1000 ./build/kernel/uosc-kernel
perf report
```

### Intel VTune (Windows/Linux)

```bash
# Integrated with UOSC
./build.sh --with-vtune
# GUI profiler interface
```

## Performance Debugging Checklist

```
□ Establish baseline with benchmark_kernel
□ Profile with flamegraph to find hotspots
□ Check cache hit rates (perf/VTune)
□ Check TLB miss rates
□ Check interrupt frequency
□ Verify no lock contention (mutex hold times)
□ Measure context switch overhead
□ Check memory allocation patterns
□ Verify CPU cache line utilization
□ Profile with real workload (not just microbenchmarks)
```

## References

- [Scheduler](../kernel/SCHEDULER.md)
- [Memory](../kernel/MEMORY.md)
- [Building](BUILDING.md)

---

**UOSC Performance: Measured, Optimized, Validated.**
