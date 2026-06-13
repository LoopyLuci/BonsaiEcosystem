# UOSC Testing Guide

Complete guide to testing UOSC kernel, drivers, and applications.

## Testing Overview

UOSC testing includes:

- **Unit Tests**: Test individual functions in isolation
- **Integration Tests**: Test kernel subsystems together
- **System Tests**: Test entire kernel on hardware/QEMU
- **Performance Tests**: Benchmark critical operations
- **Formal Verification**: Prove correctness mathematically

## Test Organization

```
tests/
├── unit/                      ← Individual function tests
│   ├── test_process.c
│   ├── test_memory.c
│   ├── test_scheduler.c
│   └── test_hypercalls.c
├── integration/               ← Multi-component tests
│   ├── test_process_lifecycle.c
│   ├── test_memory_protection.c
│   └── test_device_io.c
├── system/                    ← Full kernel tests
│   ├── boot_test.c
│   ├── stress_test.c
│   └── concurrency_test.c
├── performance/               ← Benchmark tests
│   ├── benchmark_context_switch.c
│   ├── benchmark_scheduler.c
│   └── benchmark_memory.c
└── drivers/                   ← Driver tests
    ├── test_console_driver.c
    └── test_timer_driver.c
```

## Building Tests

### Build with Tests

```bash
# Build full system with tests
./build.sh --target=standalone --mode=full

# Includes all test binaries in build/tests/
```

### Build Specific Test

```bash
# Build only kernel tests
./build.sh --tests=kernel

# Build only device tests
./build.sh --tests=device

# Build all tests
./build.sh --tests=all
```

## Running Tests

### Unit Tests

```bash
# Run all unit tests
./build/tests/test_unit

# Output:
[UNIT] process_create ... PASSED
[UNIT] process_exit ... PASSED
[UNIT] mem_alloc ... PASSED
...
[UNIT] All 156 tests PASSED

# Run specific test
./build/tests/test_unit --filter=process_create
```

### Integration Tests

```bash
# Run integration tests in QEMU
./build/tests/integration_test

# Or on hardware
./build/tests/integration_test --device

# With verbose output
./build/tests/integration_test --verbose

# Expected output:
[INT] Process lifecycle ... PASSED
[INT] Memory protection ... PASSED
[INT] Synchronization ... PASSED
...
```

### System Tests

```bash
# Boot kernel with test suite
./build/tests/system_test

# Or in QEMU
qemu-system-x86_64 \
    -kernel build/kernel/uosc-kernel.elf \
    -initrd build/tests/system_test.iso

# Kernel runs all tests on boot
[TEST] Boot sequence ... OK
[TEST] CPU detection ... OK
[TEST] Memory initialization ... OK
[TEST] Process manager ... OK
[TEST] Scheduler ... OK
[TEST] Interrupt handling ... OK
[TEST] Device drivers ... OK
...
```

## Writing Tests

### Unit Test Template

```c
// tests/unit/test_my_feature.c
#include <uosc_test.h>
#include "kernel/my_feature.h"

// Test case
void test_basic_functionality() {
    // Setup
    Object* obj = object_create();
    ASSERT_NE(obj, NULL, "Object creation");
    
    // Test
    int result = object_operation(obj);
    ASSERT_EQ(result, 0, "Operation succeeded");
    
    // Verify state
    ASSERT_EQ(object_get_state(obj), READY, "Object ready");
    
    // Cleanup
    object_destroy(obj);
}

void test_error_condition() {
    // Test error case
    int result = object_operation(NULL);
    ASSERT_NE(result, 0, "Null pointer detected");
    ASSERT_EQ(result, -EINVAL, "Invalid argument error");
}

// Test suite registration
TEST_SUITE(my_feature) {
    RUN_TEST(test_basic_functionality);
    RUN_TEST(test_error_condition);
};
```

### Integration Test Template

```c
// tests/integration/test_subsystem_interaction.c
#include <uosc_test.h>
#include "kernel/subsystem1.h"
#include "kernel/subsystem2.h"

void test_subsystem_interaction() {
    // Setup
    ProcessID p1 = hypercall_process_create(func1, 4096, 128, CAP_ALL);
    ProcessID p2 = hypercall_process_create(func2, 4096, 128, CAP_ALL);
    
    ASSERT_NE(p1, NULL, "P1 created");
    ASSERT_NE(p2, NULL, "P2 created");
    
    // Both processes interact through shared memory
    VirtualAddress shared = hypercall_mem_alloc(4096, 0);
    hypercall_mem_share(p2, shared, 4096, PROT_READ | PROT_WRITE);
    
    // Wait for completion
    int status1, status2;
    hypercall_process_wait(p1, &status1);
    hypercall_process_wait(p2, &status2);
    
    // Verify results
    ASSERT_EQ(status1, 0, "P1 succeeded");
    ASSERT_EQ(status2, 0, "P2 succeeded");
}

TEST_SUITE(subsystem_interaction) {
    RUN_TEST(test_subsystem_interaction);
};
```

## Test Assertions

```c
// Basic assertions
ASSERT_EQ(actual, expected, "message");    // Equal
ASSERT_NE(actual, expected, "message");    // Not equal
ASSERT_LT(actual, expected, "message");    // Less than
ASSERT_LE(actual, expected, "message");    // Less or equal
ASSERT_GT(actual, expected, "message");    // Greater than
ASSERT_GE(actual, expected, "message");    // Greater or equal

// Pointer assertions
ASSERT_NULL(ptr, "message");               // Null pointer
ASSERT_NE(ptr, NULL, "message");          // Not null

// Memory assertions
ASSERT_MEM_EQ(actual, expected, size, "message");
ASSERT_MEM_NE(actual, expected, size, "message");

// String assertions
ASSERT_STR_EQ(str1, str2, "message");
ASSERT_STR_NE(str1, str2, "message");
```

## Performance Testing

### Running Benchmarks

```bash
# Run all benchmarks
./build/tests/benchmark_kernel

# Output example:
[BENCH] Context Switch Latency
  Samples:     1000000
  Min:         0.7µs
  Max:         2.3µs
  Avg:         0.85µs
  Stddev:      0.15µs
  Target:      <1µs
  Status:      ✓ PASS

[BENCH] Scheduler Decision
  Samples:     10000000
  Min:         50ns
  Max:         300ns
  Avg:         95ns
  Target:      <100ns
  Status:      ✓ PASS
```

### Creating Benchmark

```c
// tests/performance/benchmark_my_feature.c
#include <uosc_bench.h>

void benchmark_operation() {
    Benchmark bench = benchmark_create("my_operation", 1000000);
    
    for (u64 i = 0; i < benchmark_samples(&bench); i++) {
        benchmark_start(&bench);
        
        // Operation to measure
        my_operation();
        
        benchmark_stop(&bench);
    }
    
    benchmark_set_target(&bench, 1000);  // Target: 1µs
    benchmark_report(&bench);
    
    // Output:
    // my_operation: min=0.8µs max=2.1µs avg=0.95µs (target: 1µs) ✓
}
```

## Stress Testing

### Long-Running Tests

```bash
# Run stress test (1 hour)
./build/tests/stress_test --duration=3600

# Tests:
[STRESS] Creating 1000 processes...
[STRESS] Spawning threads...
[STRESS] Heavy memory allocation...
[STRESS] Context switching 10M times...
[STRESS] Testing synchronization under load...

# Result:
[STRESS] All operations completed successfully
[STRESS] No memory leaks detected
[STRESS] No deadlocks
[STRESS] Stability: ✓ PASS
```

### Concurrency Testing

```bash
// tests/system/test_concurrency.c
void test_concurrent_allocations() {
    ProcessID pids[10];
    
    // Create many processes competing for memory
    for (int i = 0; i < 10; i++) {
        pids[i] = hypercall_process_create(
            allocate_free_loop,  // Allocate and free repeatedly
            4096,
            128,
            CAP_ALL
        );
    }
    
    // Wait for all to complete
    for (int i = 0; i < 10; i++) {
        int status;
        hypercall_process_wait(pids[i], &status);
        ASSERT_EQ(status, 0, "Process succeeded");
    }
}
```

## Device Driver Testing

### Unit Test for Driver

```c
// tests/drivers/test_uart_driver.c
void test_uart_initialization() {
    // Create mock device config
    DeviceConfig config = {
        .base_addr = 0x3F201000,
        .irq_number = 57,
    };
    
    UARTDevice dev = {};
    int result = uart_device_initialize((Device*)&dev, &config);
    
    ASSERT_EQ(result, 0, "UART initialized");
    ASSERT_EQ(dev.state, DEVICE_READY, "Device ready");
}

void test_uart_write() {
    UARTDevice dev = {};
    setup_device(&dev);
    
    const char* msg = "Hello";
    int written = uart_write((Device*)&dev, msg, 5);
    
    ASSERT_EQ(written, 5, "All bytes written");
}
```

### Integration Test with Driver

```c
void test_driver_interrupt_handling() {
    // Open device
    FileDescriptor fd = hypercall_device_open("/dev/uart", O_RDWR);
    ASSERT_NE(fd, -1, "Device opened");
    
    // Write data
    hypercall_device_write(fd, "X", 1);
    
    // Simulate interrupt
    simulate_device_interrupt(UART_IRQ);
    
    // Verify interrupt handled
    u8 buffer[10];
    int n = hypercall_device_read(fd, buffer, 10);
    ASSERT_GT(n, 0, "Data received");
    
    // Cleanup
    hypercall_device_close(fd);
}
```

## Formal Verification

### Verifying Proofs

```bash
# Verify specific proof
axiom verify axiom/proofs/process_isolation.ax

# Verify all proofs
axiom verify axiom/proofs/*.ax

# Verbose output
axiom verify axiom/proofs/process_isolation.ax --verbose

# Expected output:
Checking: process_isolation.ax
Checking theorem ProcessIsolation...
  Lemma 1: memory_separation ... OK
  Lemma 2: capability_isolation ... OK
  Main theorem ... OK
Result: VERIFIED ✓
```

### Building with Verification

```bash
# Build with proof checking (adds ~30 minutes)
./build.sh --verify-proofs --timeout=600

# Build output:
[BUILD] Compiling kernel...
[BUILD] Linking...
[VERIFY] Checking axiom/proofs/process_isolation.ax ... OK
[VERIFY] Checking axiom/proofs/memory_safety.ax ... OK
[VERIFY] Checking axiom/proofs/scheduling_fairness.ax ... OK
[VERIFY] Checking axiom/proofs/no_deadlock.ax ... OK
[VERIFY] All proofs verified ✓
```

## Test Coverage

### Measuring Coverage

```bash
# Build with coverage instrumentation
./build.sh --coverage

# Run tests
./build/tests/test_kernel

# Generate coverage report
gcov src/kernel/*.c

# View report in HTML
open coverage/index.html
```

### Coverage Goals

| Component | Target | Current |
|-----------|--------|---------|
| Kernel Core | 95% | 98% |
| Memory | 90% | 94% |
| Scheduler | 95% | 97% |
| Hypercalls | 90% | 92% |
| Drivers | 80% | 85% |

## Continuous Integration

### CI Test Matrix

```
Test configurations run automatically on every commit:

Architecture:
  - x86_64
  - ARM64 (experimental)

Build modes:
  - Production
  - Debug
  - Full

Verification:
  - With proofs
  - Without proofs

Test suites:
  - Unit tests
  - Integration tests
  - Stress tests
  - Performance tests
```

### Expected CI Results

```
Commit: "feat: Add new scheduler optimization"

CI Results:
  ✓ Build (x86_64, production)
  ✓ Build (x86_64, debug)
  ✓ Build (ARM64, production)
  ✓ Unit Tests (156/156 passed)
  ✓ Integration Tests (42/42 passed)
  ✓ Stress Test (8 hours passed)
  ✓ Performance Tests (all targets met)
  ✓ Proof Verification (5/5 proofs verified)
  ✓ Coverage (98% overall)
  
Status: ALL PASS ✓
```

## Test Reporting

### Test Report Format

```
UOSC Test Report
================
Date: 2026-06-13
Build: v1.0.0
Commit: abc123def456

Summary:
  Unit Tests:       156/156 PASSED
  Integration:      42/42 PASSED
  System Tests:     8/8 PASSED
  Performance:      12/12 PASSED
  Formal Proofs:    5/5 VERIFIED

Details:
  Test Duration:    8h 23m 15s
  Total Coverage:   98%
  Stress Test:      8 hours, no failures
  Memory Leaks:     0 detected

Status: ✓ ALL TESTS PASSED
```

## Debugging Failed Tests

### Finding Failed Test

```bash
# Run with verbose output
./build/tests/test_kernel --verbose 2>&1 | grep -A 5 "FAILED"

# Run specific test
./build/tests/test_kernel --filter=failing_test

# Run with debugger
gdb --args ./build/tests/test_kernel --filter=failing_test
(gdb) run
(gdb) bt  # Backtrace when it fails
```

### Analyzing Test Failure

```
Test output:
  FAILED: test_process_create
  Expected: 0
  Actual: -1 (ENOMEM)
  
Likely cause: Out of memory
Solution: Reduce number of processes or increase heap size

Debug steps:
1. Check memory stats: hypercall_mem_stats(pid, &stats)
2. Check process limit: cat /proc/sys/kernel/pid_max
3. Rebuild with larger heap: ./build.sh --kernel-heap=32MB
```

## Test Best Practices

1. **Isolate Tests**: Each test should be independent
2. **Test One Thing**: One assertion per test case
3. **Clear Names**: Test name describes what is being tested
4. **Setup/Teardown**: Allocate in setup, free in teardown
5. **Deterministic**: Same input → same output every time
6. **Fast**: Unit tests < 100ms, integration < 1s
7. **Comprehensive**: Cover normal, edge, and error cases

---

**UOSC Testing: Comprehensive, Automated, Verified.**
