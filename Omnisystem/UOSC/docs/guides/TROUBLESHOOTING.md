# UOSC Troubleshooting Guide

Comprehensive guide to debugging, diagnosing, and fixing common issues in UOSC.

## Build Issues

### "gcc: command not found"

**Cause**: GCC compiler not installed

**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install build-essential

# CentOS/RHEL
sudo yum groupinstall "Development Tools"

# macOS
brew install gcc
```

### "NASM not found"

**Cause**: NASM assembler not installed

**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install nasm

# CentOS/RHEL
sudo yum install nasm

# macOS
brew install nasm
```

### "undefined reference to `symbol'"

**Cause**: Missing library or object file

**Solution**:
```bash
# Check linker script for all object files
cat linker.ld

# Add missing objects:
ld -T linker.ld -o kernel.elf \
    obj1.o obj2.o ... missing_obj.o

# Rebuild with verbose output
./build.sh --verbose 2>&1 | grep "undefined"
```

### Build hangs or times out

**Cause**: Infinite loop in build script or system issue

**Solution**:
```bash
# Kill build
Ctrl+C

# Clean and retry
./build.sh clean
./build.sh --target=standalone --mode=production

# Check disk space
df -h

# Limit parallel jobs if low memory
./build.sh --jobs=2
```

## Kernel Boot Issues

### "Kernel panic: No init process"

**Cause**: Kernel can't find or execute init task

**Solution**:
```bash
# Verify init task exists in kernel
grep -n "init_task" src/kernel/process.c

# Check if entry point is correct
grep -n "ENTRY_POINT" src/kernel/main.c

# Rebuild with verbose logging
./build.sh --mode=debug --verbose
```

### "CPU doesn't support required features"

**Cause**: Running on incompatible processor

**Solution**:
```bash
# Check CPU features
cat /proc/cpuinfo | grep flags

# Build for older CPU
./build.sh --target=standalone --march=x86-64-v2

# Or use QEMU instead
qemu-system-x86_64 -kernel build/kernel/uosc-kernel.elf
```

### Kernel immediately crashes after boot

**Cause**: Exception in early boot code

**Solution**:
```bash
# Run in QEMU with debugging
qemu-system-x86_64 -kernel kernel.elf -s -S

# In another terminal
gdb kernel.elf
(gdb) target remote localhost:1234
(gdb) c

# Check if exception occurs
(gdb) bt  # Backtrace to see where crash happened
```

## Process Issues

### "EAGAIN: Too many processes"

**Cause**: Hit process limit (default 65536)

**Solution**:
```c
// Terminate some processes first
hypercall_process_kill(old_pid, SIGKILL);

// Or reduce max process limit if custom
// Edit config and rebuild
```

### "ENOMEM: Out of memory" on process_create

**Cause**: Insufficient memory for process structure or stack

**Solution**:
```c
// Reduce stack size
ProcessID child = hypercall_process_create(
    entry,
    4096,      // Reduce from 8192
    priority,
    capabilities
);

// Or free other processes' memory
hypercall_process_exit(0);
```

### Process hangs and never responds

**Cause**: Process blocked on synchronization or I/O

**Solution**:
```c
// Kill unresponsive process
hypercall_process_kill(stuck_pid, SIGKILL);

// Debug with strace (if available)
strace -p $pid

// Check process state
ProcessState state;
hypercall_process_get_state(stuck_pid, &state);
// state.state should be RUNNING or BLOCKED
```

### "Process killed by signal 11 (SIGSEGV)"

**Cause**: Invalid memory access (null pointer, buffer overflow)

**Solution**:
```c
// Run with debugger
gdb ./build/kernel/uosc-kernel

# In debugger
(gdb) run --debug
# ... process crashes ...
(gdb) bt   # Backtrace
(gdb) info registers
(gdb) x/10i $rip  # Show instructions around crash

// Check memory allocations
hypercall_mem_query(bad_addr, &info);
// If EFAULT, address not allocated
```

## Memory Issues

### "ENOMEM: Out of memory"

**Cause**: Virtual address space or physical memory exhausted

**Solution**:
```bash
# Check memory usage
cat /proc/meminfo | grep MemAvailable

# List large processes
ps aux --sort=-%mem | head

# Kill non-essential processes
killall process_name

# Or increase swap (if possible)
dd if=/dev/zero of=/swapfile bs=1M count=1024
mkswap /swapfile
swapon /swapfile
```

### "Segmentation fault on memory access"

**Cause**: Accessing memory without proper allocation/permission

**Solution**:
```c
// Check allocation before access
MemoryInfo info;
if (hypercall_mem_query(ptr, &info) == 0) {
    // Memory is allocated
    if (info.protection & PROT_READ) {
        // Can read
        value = *ptr;
    }
} else {
    // Memory not allocated
    return -EFAULT;
}
```

### Memory leak detected

**Cause**: Allocation without corresponding deallocation

**Solution**:
```bash
# Check for leaks with valgrind
valgrind --leak-check=full ./program

# Or manually track allocations
./build.sh --with-memory-tracking

# Review allocation sites
grep -n "mem_alloc" src/kernel/*.c
```

## Device Issues

### "ENOENT: No such device" on device_open

**Cause**: Device not registered or driver not loaded

**Solution**:
```bash
# List available devices
ls -la /dev/

# Check driver loaded
grep "device_name" /proc/devices

# Load driver
insmod driver/my_device.ko

# Or rebuild with driver
./build.sh --with-driver=my_device
```

### Device returns "ENOTREADY" on I/O

**Cause**: Device not initialized or hardware not responding

**Solution**:
```bash
# Check device status
lspci | grep device_name

# Check hardware power
# Verify USB power supply or PCI slot power

# Reset device
hypercall_device_ioctl(fd, IOCTL_RESET, NULL);

# Wait for ready
hypercall_device_poll(fd, POLL_READ | POLL_WRITE, 1000);
```

### Device interrupts not working

**Cause**: IRQ not configured or handler not registered

**Solution**:
```bash
# Check IRQ assignment
cat /proc/interrupts | grep device

# Verify IRQ line in device config
grep "IRQ" driver/my_device.c

# Check if interrupt handler registered
kernel_register_interrupt(irq, handler, context);

# Test with polling instead
hypercall_device_poll(fd, POLL_READ, -1);
```

## Synchronization Issues

### "EDEADLK: Would cause deadlock"

**Cause**: Trying to acquire lock already held by caller

**Solution**:
```c
// Use RECURSIVE mutex if need to reacquire
Mutex* lock = hypercall_mutex_create(MUTEX_RECURSIVE);
hypercall_mutex_lock(lock);
// Can now lock again
recursive_function_that_locks();
hypercall_mutex_unlock(lock);
```

### Processes stuck waiting (application deadlock)

**Cause**: Circular lock dependency

**Solution**:
```
Process A: locks L1, waiting for L2
Process B: locks L2, waiting for L1
→ Deadlock

Fix: Use consistent lock ordering:
Process A: locks L1 then L2
Process B: locks L1 then L2
```

### Futex seems not waking processes

**Cause**: futex_wake address mismatch or value changed

**Solution**:
```c
// Ensure same address
i32 futex = 0;
hypercall_futex_wait(&futex, 0, -1);
// Other process:
futex = 1;  // Change value
hypercall_futex_wake(&futex, 1);  // Same address!

// Check futex value before wait
if (*futex_addr == expected_value) {
    hypercall_futex_wait(futex_addr, expected_value, -1);
}
```

## Performance Issues

### Process slower than expected

**Cause**: Bad scheduling priority, page faults, or lock contention

**Solution**:
```bash
# Profile with flamegraph
./build.sh --with-flamegraph
./kernel --flamegraph=cpu
# View build/profiles/flamegraph.html

# Check scheduling priority
ProcessState state;
hypercall_process_get_state(pid, &state);
// state.priority is current priority
// Lower number = higher priority

// Increase priority if too low
hypercall_process_set_priority(pid, 10);
```

### High context switch overhead

**Cause**: Too many processes or too short time quantum

**Solution**:
```bash
# Increase time quantum in config
./build.sh --config=custom.conf

# Reduce number of processes
# Or use real-time priority (0-127) to avoid preemption
```

### Memory allocation slow

**Cause**: Fragmentation or page fault

**Solution**:
```bash
# Pre-commit pages to avoid later faults
hypercall_mem_commit(addr, size);

// Use large pages for huge allocations
hypercall_mem_alloc(size, MEM_HUGE | MEM_ZERO);

// Or pin in memory
hypercall_mem_pin(addr, size);
```

## Testing Issues

### "Test timeout" on integration tests

**Cause**: Process running longer than expected or hang

**Solution**:
```bash
# Increase timeout
./build/tests/integration_test --timeout=10000

# Run single test in debugger
gdb --args ./test_kernel --filter="specific_test"

# Add verbose output
./build/tests/test_kernel --verbose
```

### "Assertion failed" in tests

**Cause**: Test condition not met

**Solution**:
```bash
# Run with detailed output
./build/tests/test_kernel --verbose 2>&1 | head -100

# Run single failing test
./build/tests/test_kernel --filter="failing_test_name"

# Check assertion in source
grep -n "assert" tests/failing_test.c
```

## Verification Issues

### "Proof verification failed"

**Cause**: Formal proof invalid or axiom syntax error

**Solution**:
```bash
# Check specific proof
axiom verify axiom/proofs/failing_proof.ax --verbose

# Look for syntax errors
axiom parse axiom/proofs/failing_proof.ax

# Re-examine theorem statement
grep -n "Theorem" axiom/proofs/failing_proof.ax
```

### Build hangs on "Verifying proofs..."

**Cause**: Proof checker slow or infinite loop

**Solution**:
```bash
# Build without verification
./build.sh --no-verify

# Or with limited verification
./build.sh --verify-proofs --timeout=300
```

## Debugging Techniques

### Using GDB

```bash
# Build with symbols
./build.sh --mode=debug

# Run kernel under GDB
gdb ./build/kernel/uosc-kernel.elf

# Common commands
(gdb) break main
(gdb) run
(gdb) print variable_name
(gdb) step
(gdb) next
(gdb) continue
(gdb) bt  # Backtrace
(gdb) quit
```

### Using QEMU with Debugging

```bash
# Terminal 1: Run kernel in QEMU with debug server
qemu-system-x86_64 \
    -kernel build/kernel/uosc-kernel.elf \
    -s -S \
    -m 1024

# Terminal 2: Connect GDB
gdb build/kernel/uosc-kernel.elf
(gdb) target remote localhost:1234
(gdb) c
(gdb) Ctrl+C  # To break
```

### Adding Debug Output

```c
// Use kernel logging
kernel_log("DEBUG: variable=%d\n", value);

// Or write to serial console
hypercall_device_write(console_fd, "DEBUG\n", 6);

// Build with debug symbols
./build.sh --mode=debug
```

## Getting Help

### Where to Report Bugs

1. Check existing issues: https://github.com/omnisystem/uosc/issues
2. Create new issue with:
   - Error message (complete)
   - Build command used
   - System information
   - Steps to reproduce
   - Expected vs. actual behavior

### Gathering Diagnostic Information

```bash
# System info
uname -a
lsb_release -a

# Build environment
gcc --version
nasm --version
make --version

# Build log
./build.sh --verbose 2>&1 > build.log

# Runtime error info
dmesg | tail -50
```

---

**UOSC Troubleshooting: Systematic, Practical, Comprehensive.**
