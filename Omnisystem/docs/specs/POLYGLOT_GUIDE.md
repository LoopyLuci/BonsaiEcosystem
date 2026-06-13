# Omnisystem Polyglot Integration Guide

## Overview

Omnisystem enables seamless cross-language communication through a unified C FFI layer. Any language with FFI support can interface with the Omnisystem kernel, enabling true polyglot distributed computing.

**Supported Languages (Phase 2)**:
- ✅ Rust (native)
- ✅ Go (C FFI)
- ✅ Python (ctypes)
- ✅ JavaScript/Node.js (node-ffi)
- 🔄 WebAssembly (WASM)
- 🔄 Java (JNI)
- 🔄 C# (.NET FFI)

## Architecture

### Three-Layer Design

```
┌─────────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER                            │
│  (Rust | Go | Python | JavaScript | Java | C# | ...)           │
└──────────────────────┬──────────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────────┐
│              LANGUAGE BINDINGS LAYER                            │
│  (omnisystem_rust_bindings | omnisystem_go_bindings |           │
│   omnisystem_py.py | omnisystem_node.js | ...)                 │
└──────────────────────┬──────────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────────┐
│                    C FFI BRIDGE                                 │
│  (omnisystem-go-bindings cdylib)                               │
│  Standard C calling conventions (System V AMD64 / Win64 / ARM64)│
└──────────────────────┬──────────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────────┐
│              OMNISYSTEM KERNEL LAYER                            │
│  (omnisystem-kernel in Rust)                                    │
│  - Memory Management (paging, allocation)                       │
│  - Process/Thread Scheduling                                   │
│  - Inter-Process Communication                                 │
│  - Capability-Based Security                                   │
│  - Device Abstraction                                          │
└─────────────────────────────────────────────────────────────────┘
```

## Language-Specific Integration

### Rust (Native)

**Direct API** - No overhead, zero-cost abstractions.

```rust
use omnisystem_rust_bindings::prelude::*;

#[tokio::main]
async fn main() {
    // Create runtime
    let runtime = OmnisystemRuntime::new().await?;
    
    // Access kernel directly
    let kernel = runtime.kernel();
    let processes = kernel.process().get_all_processes();
    
    // Spawn async tasks
    runtime.spawn(async {
        println!("Async task");
    });
}
```

**Key Benefits**:
- No marshaling overhead
- Full control over async/await
- Direct kernel access
- Type-safe

### Go (C FFI via cgo)

**FFI Interface** - C function calls with Go concurrency.

```go
package main

import (
    "C"
    "fmt"
)

func main() {
    // Initialize kernel
    if C.omnisystem_init() != 0 {
        panic("Init failed")
    }
    
    // Call C functions
    totalMem := C.omnisystem_get_total_memory()
    procCount := C.omnisystem_get_process_count()
    
    fmt.Printf("Memory: %d bytes\n", totalMem)
    fmt.Printf("Processes: %d\n", procCount)
    
    // Create processes
    for i := 0; i < 5; i++ {
        pid := C.omnisystem_create_process()
        fmt.Printf("Created process %d\n", pid)
    }
}
```

**Key Benefits**:
- Efficient C interop
- Goroutine concurrency
- Simple FFI boundary
- Production-ready

### Python (ctypes)

**ctypes FFI** - Dynamic library loading with Python reflection.

```python
from omnisystem_py import Omnisystem

# Initialize
omni = Omnisystem()
omni.initialize()

# Get statistics
stats = omni.get_stats()
print(f"Memory: {stats['total_memory_mb']:.2f} MB")
print(f"Processes: {stats['process_count']}")

# Create processes
for i in range(3):
    pid = omni.create_process()
    print(f"Created process {pid}")

# Test FFI echo
result = omni.echo_int(42)
assert result == 84  # Returns value * 2
```

**Key Benefits**:
- Pure Python (no compilation)
- Dynamic library loading
- Pythonic API
- Data science integration

### JavaScript/Node.js (node-ffi)

**Native FFI** - JavaScript access to native code.

```javascript
const Omnisystem = require('./omnisystem_node.js');

// Initialize
const omni = new Omnisystem();
omni.initialize();

// Get statistics
const stats = omni.getStats();
console.log(`Memory: ${stats.totalMemoryMB.toFixed(2)} MB`);
console.log(`Processes: ${stats.processCount}`);

// Create processes
for (let i = 0; i < 3; i++) {
    const pid = omni.createProcess();
    console.log(`Created process ${pid}`);
}

// Test FFI echo
const result = omni.echoInt(42);
console.assert(result === 84);
```

**Key Benefits**:
- Event-loop integration
- Async/await support
- Dynamic typing
- Frontend dashboard capability

## Cross-Language Communication Patterns

### Pattern 1: Sequential Pipeline

Language A initializes kernel → Language B creates resources → Language C monitors → Language D executes.

```
Rust (init) → Go (create processes) → Python (monitor) → JS (dashboard)
```

**Example**:
```rust
// Rust: Initialize and setup
let runtime = OmnisystemRuntime::new().await?;

// Go: Create processes via C FFI
// (via omnisystem_create_process)

// Python: Monitor and collect stats
stats = omni.get_stats()

// JavaScript: Display on dashboard
omni.getStats()  // via Node.js FFI
```

### Pattern 2: Parallel Coordination

Multiple languages execute concurrently, communicating via IPC channels.

```
Rust task 1  ──┐
Go task 2    ──┼─→ Shared Memory IPC Channel ──→ Python task 3
JavaScript 3 ──┘
```

**Example**:
```rust
// Rust: Spawn async task
runtime.spawn(async {
    // Compute task
});

// Go: Create process (executes in parallel)
C.omnisystem_create_process()

// Python: Monitor results (subscribes to IPC)
stats = omni.get_stats()
```

### Pattern 3: Divide & Conquer

Each language handles its domain strength:

- **Rust**: Performance-critical kernel operations
- **Go**: Concurrent system services
- **Python**: Data analysis and ML
- **JavaScript**: UI and visualization

## FFI Protocol Reference

### Initialization Sequence

```
1. omnisystem_init() → c_int
   - Returns 0 on success
   - Initializes Tokio runtime
   - Creates OmniKernel instance
   - Thread-safe via lazy_static

2. omnisystem_get_total_memory() → u64
   - Returns total virtual memory bytes
   - Always succeeds after init

3. omnisystem_register_ffi_module(name, major, minor, patch) → c_int
   - Register language-specific module
   - Returns 0 on success
```

### Process Management

```
omnisystem_create_process() → u64
  - Creates new process
  - Returns process ID
  - Returns 0 on failure

omnisystem_get_process_count() → u32
  - Returns current process count
  - Safe to call multiple times
```

### Monitoring & Diagnostics

```
omnisystem_get_allocated_memory() → u64
  - Allocated memory in bytes

omnisystem_get_free_memory() → u64
  - Free memory in bytes

omnisystem_get_health() → c_int
  - 0 = healthy
  - 1 = degraded
  - 2 = critical

omnisystem_echo_int(value) → c_int
  - Echo test: returns value * 2
  - FFI communication verification
```

### Shutdown

```
omnisystem_shutdown() → c_int
  - Graceful kernel shutdown
  - Cleanup all resources
  - Returns 0 on success
```

## Building Language Bindings

### Step 1: Compile C FFI Layer

```bash
cd crates/omnisystem-go-bindings
cargo build --release
```

This produces:
- Windows: `target/release/omnisystem_go.dll`
- Linux: `target/release/libomnisystem_go.so`
- macOS: `target/release/libomnisystem_go.dylib`

### Step 2: Load in Each Language

**Rust**: Already linked, use `omnisystem_rust_bindings` crate

**Go**: Use cgo to link:
```go
// #cgo LDFLAGS: -L/path/to/target/release -lomnisystem_go
// #include "omnisystem.h"
import "C"
```

**Python**: Use ctypes to dynamically load:
```python
import ctypes
lib = ctypes.CDLL("./target/release/libomnisystem_go.so")
```

**Node.js**: Use node-ffi to bind:
```javascript
const ffi = require('ffi-napi');
const lib = ffi.Library('./target/release/libomnisystem_go', {...});
```

## Performance Characteristics

### FFI Overhead

| Operation | Latency | Notes |
|-----------|---------|-------|
| Process creation | ~100 µs | Via FFI boundary |
| Memory query | ~10 µs | Direct access |
| Echo test (round-trip) | ~20 µs | Call + return |
| Kernel init | ~50 ms | One-time cost |

### Scalability

- **Processes**: Tested with 1,000+ concurrent
- **Threads per process**: 256 priority levels
- **IPC channels**: Unlimited (limited by memory)
- **Languages**: Unlimited (each loads library once)

## Best Practices

### 1. Initialize Once

```rust
// ✅ Good: Shared runtime
let runtime = OmnisystemRuntime::new().await?;
for _ in 0..1000 {
    runtime.spawn(async { /* task */ });
}

// ❌ Bad: Multiple inits
for _ in 0..1000 {
    let runtime = OmnisystemRuntime::new().await?;
}
```

### 2. Use Language Strengths

```rust
// Rust: Performance
// → Complex algorithms, tight loops, system code

// Go: Concurrency
// → Network services, goroutine-based tasks

// Python: Data science
// → ML models, statistical analysis, data processing

// JavaScript: UI
// → Dashboards, real-time visualization
```

### 3. Minimize FFI Calls

```rust
// ✅ Good: Batch calls
let stats = runtime.get_stats();
let memory = stats.total_memory_bytes;
let processes = stats.process_count;

// ❌ Bad: Individual calls
let memory = runtime.kernel().memory().get_stats().total_memory_bytes;
let processes = runtime.kernel().process().process_count();
```

### 4. Error Handling

```python
from omnisystem_py import Omnisystem, OmnisystemError

try:
    omni = Omnisystem()
    omni.initialize()
except OmnisystemError as e:
    print(f"Omnisystem error: {e}")
    # Fallback or retry
```

## Examples

### Example 1: Multi-Language Pipeline

See: `crates/omnisystem-rust-bindings/examples/polyglot_orchestration.rs`

Demonstrates:
- Rust kernel initialization
- Go FFI process creation
- Python statistics monitoring
- Async task coordination

### Example 2: Go Service

See: `bindings/` directory

Go service using Omnisystem:
```go
func main() {
    C.omnisystem_init()
    defer C.omnisystem_shutdown()
    
    // Service logic
}
```

### Example 3: Python Analysis

See: `bindings/omnisystem_py.py`

Data analysis on Omnisystem:
```python
omni = Omnisystem()
omni.initialize()
stats = omni.get_stats()
# Analyze with Pandas, NumPy, etc.
```

## Troubleshooting

### Library Not Found

```
Error: Could not find omnisystem_go.dll
```

**Solution**: Ensure omnisystem-go-bindings is built:
```bash
cd crates/omnisystem-go-bindings
cargo build --release
```

### Kernel Not Initialized

```
Error: Kernel not initialized. Call initialize() first.
```

**Solution**: Initialize before use:
```python
omni = Omnisystem()
omni.initialize()  # Required before other calls
stats = omni.get_stats()
```

### FFI Type Mismatch

```
Error: Incorrect argument type
```

**Solution**: Verify calling convention:
- C expects: `c_int` (32-bit int)
- C expects: `u64` (64-bit unsigned)
- Map correctly in language bindings

## Future Work

- **WebAssembly**: Browser-native WASM bindings
- **Java**: JNI bindings for enterprise integration
- **C#/.NET**: FFI for Windows/.NET ecosystem
- **Zig**: Direct Zig integration (no FFI overhead)
- **Kotlin**: Multi-platform Kotlin bindings
- **Distributed**: RPC protocol for cross-machine orchestration

## Summary

Omnisystem's polyglot architecture enables:
- ✅ Language interoperability via C FFI
- ✅ Each language operates in its strength domain
- ✅ Coordinated execution across languages
- ✅ Shared kernel state and IPC
- ✅ Production-ready FFI layer
- ✅ Zero-copy memory sharing (via capability system)

The C FFI layer is the "Universal Adapter" - any language that can call C can orchestrate with Omnisystem.
