# UOSC Documentation Index

**Universal Operating System Core: Complete Self-Contained Documentation**

## Quick Navigation

**Getting Started**: [README.md](README.md) - Start here for overview and use cases

**Architecture**: [Kernel Architecture](kernel/ARCHITECTURE.md) - Complete kernel design and proofs

**Drivers**: [Driver Framework](drivers/FRAMEWORK.md) - Device abstraction and hot-swapping

**Hypercalls**: [Hypercall Specification](hypercalls/SPECIFICATION.md) - 50 proven kernel-mode operations

**Proofs**: [Formal Proofs](proofs/) - Mathematical verification of critical properties

**Guides**: [Implementation Guides](guides/) - Building and extending UOSC

**API**: [API Reference](api/) - Complete function signatures and contracts

**Examples**: [Code Examples](examples/) - Working demonstrations of kernel concepts

## Documentation Structure

```
UOSC/
├── README.md                    ← START HERE
├── docs/
│   ├── INDEX.md                (This file)
│   ├── kernel/
│   │   ├── ARCHITECTURE.md      ← System design
│   │   ├── PROCESS.md           ← Process subsystem
│   │   ├── MEMORY.md            ← Memory management
│   │   ├── SCHEDULER.md         ← Scheduling algorithm
│   │   └── INVARIANTS.md        ← Kernel invariants
│   ├── drivers/
│   │   ├── FRAMEWORK.md         ← Device abstraction
│   │   ├── DRIVER_DEVELOPMENT.md ← Writing drivers
│   │   ├── BUILTIN_DRIVERS.md   ← Standard drivers
│   │   └── EXAMPLES.md          ← Driver samples
│   ├── hypercalls/
│   │   ├── SPECIFICATION.md     ← Complete API
│   │   ├── PROCESS_CALLS.md     ← Process operations
│   │   ├── MEMORY_CALLS.md      ← Memory operations
│   │   ├── DEVICE_CALLS.md      ← I/O operations
│   │   └── SYNC_CALLS.md        ← Synchronization
│   ├── proofs/
│   │   ├── OVERVIEW.md          ← Proof system intro
│   │   ├── PROCESS_ISOLATION.ax ← Process safety
│   │   ├── MEMORY_SAFETY.ax     ← Memory consistency
│   │   ├── SCHEDULING_FAIRNESS.ax ← Fairness proof
│   │   └── NO_DEADLOCK.ax       ← Deadlock-free proof
│   ├── guides/
│   │   ├── BUILDING.md          ← Build instructions
│   │   ├── IMPLEMENTATION.md    ← Implementation guide
│   │   ├── PERFORMANCE.md       ← Performance tuning
│   │   ├── DEPLOYMENT.md        ← Deployment scenarios
│   │   ├── TROUBLESHOOTING.md   ← Debug & diagnosis
│   │   └── CONTRIBUTING.md      ← How to contribute
│   ├── api/
│   │   ├── OVERVIEW.md          ← API summary
│   │   ├── hypercalls.h         ← C header
│   │   ├── process.md           ← Process API
│   │   ├── memory.md            ← Memory API
│   │   ├── devices.md           ← Device API
│   │   └── synchronization.md   ← Sync API
│   ├── examples/
│   │   ├── hello_world.ti       ← Minimal program
│   │   ├── process_creation.ti  ← Process example
│   │   ├── memory_allocation.ti ← Memory example
│   │   ├── device_io.ti         ← I/O example
│   │   └── synchronization.ti   ← Sync example
│   └── reference/
│       ├── GLOSSARY.md          ← Terminology
│       ├── SYSCALL_TABLE.md     ← Hypercall reference
│       ├── ERROR_CODES.md       ← Error documentation
│       └── CONSTANTS.md         ← System constants
│
├── kernel/                      ← Implementation
├── drivers/                     ← Device drivers
├── hypercalls/                  ← Hypercall interface
├── proofs/                      ← Formal proofs
│
└── [Source Code - Self-Contained]
```

## Key Concepts

### Process Management
- **Process**: Isolated execution context with memory space and resources
- **ProcessID**: Unique identifier for each process
- **State**: RUNNABLE, RUNNING, BLOCKED, TERMINATED
- **Isolation**: Processes cannot access each other's memory
- **Capability**: Permission to perform privileged operation

**Learn More**: [Process Subsystem](kernel/PROCESS.md)

### Memory Management
- **Virtual Memory**: Per-process virtual address space
- **Page**: 4KB unit of memory protection
- **Page Table**: Hardware-assisted address translation
- **Segment**: Contiguous memory region (code, data, heap, stack)
- **Protection**: Read, Write, Execute per page

**Learn More**: [Memory Subsystem](kernel/MEMORY.md)

### Scheduling
- **Time Quantum**: CPU time slice per process
- **Priority**: 0-127 real-time, 128-255 normal
- **Fair Queuing**: Equal time to equal priority processes
- **Preemption**: Interrupt-based context switching
- **Load Balancing**: Distribute work across processors (multi-core)

**Learn More**: [Scheduler](kernel/SCHEDULER.md)

### Devices
- **Device Class**: BlockDevice, CharacterDevice, NetworkDevice, etc.
- **Device Tree**: Hierarchical device model
- **Driver**: Software for specific device
- **Interrupt**: Hardware notification
- **Hot-Swap**: Add/remove devices without reboot

**Learn More**: [Driver Framework](drivers/FRAMEWORK.md)

### Hypercalls
- **Hypercall**: Transition from user-mode to kernel-mode
- **Atomicity**: Operation completes or fails completely
- **Verification**: Formal proof of correctness
- **Performance**: Minimal overhead, deterministic latency

**Learn More**: [Hypercall Specification](hypercalls/SPECIFICATION.md)

## Learning Path

### For OS Learners
1. Read [README.md](README.md)
2. Study [Kernel Architecture](kernel/ARCHITECTURE.md)
3. Explore [Memory Management](kernel/MEMORY.md)
4. Understand [Scheduling](kernel/SCHEDULER.md)
5. Review [Code Examples](examples/)

### For Driver Developers
1. Read [Driver Framework](drivers/FRAMEWORK.md)
2. Study [Driver Development Guide](drivers/DRIVER_DEVELOPMENT.md)
3. Review [Example Drivers](drivers/EXAMPLES.md)
4. Build your own driver

### For System Architects
1. Study [Architecture](kernel/ARCHITECTURE.md)
2. Review [Hypercall Interface](hypercalls/SPECIFICATION.md)
3. Understand [Formal Proofs](proofs/)
4. Plan system design

### For Security Researchers
1. Study [Formal Proofs](proofs/)
2. Analyze [Threat Model](kernel/ARCHITECTURE.md#threat-model)
3. Review [Security Considerations](guides/SECURITY.md)
4. Propose improvements

### For Contributors
1. Read [CONTRIBUTING.md](guides/CONTRIBUTING.md)
2. Set up build environment: [BUILDING.md](guides/BUILDING.md)
3. Review [Implementation Guide](guides/IMPLEMENTATION.md)
4. Submit improvements

## Reference Materials

### Complete API Specification
- [Hypercall Specification](hypercalls/SPECIFICATION.md) - All 50 hypercalls with proofs
- [API Reference](api/) - Function signatures and contracts
- [Syscall Table](reference/SYSCALL_TABLE.md) - Complete hypercall listing

### Error Handling
- [Error Codes](reference/ERROR_CODES.md) - All possible error returns
- [Troubleshooting Guide](guides/TROUBLESHOOTING.md) - Common issues and solutions

### Performance
- [Performance Guide](guides/PERFORMANCE.md) - Optimization techniques
- [Benchmarks](guides/BENCHMARKS.md) - Performance measurements
- [Latency Reference](reference/LATENCY.md) - Operation latencies

### Formal Verification
- [Proof Overview](proofs/OVERVIEW.md) - Proof system introduction
- [Process Isolation Proof](proofs/PROCESS_ISOLATION.ax) - Proven in Axiom
- [Memory Safety Proof](proofs/MEMORY_SAFETY.ax) - Proven in Axiom
- [Scheduling Fairness](proofs/SCHEDULING_FAIRNESS.ax) - Proven in Axiom
- [Deadlock-Free Proof](proofs/NO_DEADLOCK.ax) - Proven in Axiom

## Building UOSC

```bash
# Get started
git clone https://github.com/omnisystem/uosc.git
cd uosc

# Build standalone kernel
./build.sh --target=standalone

# Build for Omnisystem
./build.sh --target=omnisystem

# Run tests
./test.sh

# Generate documentation
./docs.sh
```

See [Building Guide](guides/BUILDING.md) for detailed instructions.

## Integration with Omnisystem

UOSC is **fully self-contained** but also integrates with Omnisystem:

```
Omnisystem = Aether + Axiom + Sylva + Titan + [UOSC]

UOSC serves as:
  - Foundational micro-kernel
  - Process isolation mechanism
  - Memory protection layer
  - Device abstraction
  - Low-level driver framework
```

See [Integration Guide](../docs/OMNISYSTEM_INTEGRATION.md) for details.

## Status

**UOSC Phase**: Production Ready (v1.0)

- ✅ Kernel subsystem complete and verified
- ✅ Driver framework fully implemented
- ✅ Hypercall interface proven correct
- ✅ Formal safety proofs completed
- ✅ Production test suite passing
- ✅ Documentation complete

## Community

- **Report Issues**: [UOSC Issue Tracker](https://github.com/omnisystem/uosc/issues)
- **Contribute**: See [CONTRIBUTING.md](guides/CONTRIBUTING.md)
- **Discuss**: UOSC Forum (coming soon)

## License

UOSC is part of Omnisystem. See root LICENSE file.

---

**UOSC: Self-Contained, Modular, Verified, Enterprise-Grade Micro-Kernel**

Start with [README.md](README.md) and navigate from there!
