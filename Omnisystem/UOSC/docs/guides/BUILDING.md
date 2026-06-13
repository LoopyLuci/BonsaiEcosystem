# Building UOSC

Complete guide to building UOSC standalone micro-kernel or integrating with Omnisystem.

## Quick Start

### Standalone Build (Minimal Dependencies)

```bash
# Clone UOSC
cd /path/to/UOSC

# Build standalone kernel
./build.sh --target=standalone --mode=production

# Build complete with tests and examples
./build.sh --target=standalone --mode=full

# Run UOSC
./uosc-kernel --config=default.conf
```

### Integration Build (Omnisystem)

```bash
# Build as Omnisystem component
cd /path/to/Omnisystem
./build.sh --include-uosc --target=omnisystem

# UOSC embedded automatically
```

## Requirements

### Standalone Build

**Mandatory:**
- x86-64 processor (or ARM port)
- GCC 9+ or Clang 10+
- NASM 2.13+ (assembler)
- Make 4.0+
- Bash 4.0+

**Optional:**
- QEMU (for testing without hardware)
- GDB (for debugging)
- Sphinx (for documentation generation)
- Axiom proof checker (for verifying proofs)

### Omnisystem Integration

Same as Omnisystem requirements (includes UOSC).

## Build Directory Structure

```
UOSC/
├── build/                       ← Build output
│   ├── kernel/
│   │   ├── libuosc-kernel.a
│   │   ├── uosc-kernel.exe
│   │   └── uosc-kernel.elf
│   ├── drivers/
│   ├── tests/
│   └── objects/                 ← Object files (.o)
├── src/                         ← Source code
│   ├── kernel/
│   │   ├── process.c
│   │   ├── memory.c
│   │   ├── scheduler.c
│   │   ├── interrupt.c
│   │   └── bootloader/
│   ├── drivers/
│   │   ├── console/
│   │   ├── timer/
│   │   └── ...
│   └── arch/
│       ├── x86_64/
│       ├── arm64/
│       └── common/
├── include/
│   └── uosc/                    ← Public headers
│       ├── kernel.h
│       ├── hypercall.h
│       ├── driver.h
│       └── ...
├── tests/
├── Makefile
├── build.sh                     ← Build script
└── verify.sh                    ← Verification script
```

## Build Targets

### Standalone Targets

```bash
./build.sh --target=standalone --mode=production
  • Minimal kernel
  • No debugging symbols
  • Optimizations enabled (-O3)
  • ~500KB binary
  • Runs on baremetal or QEMU

./build.sh --target=standalone --mode=debug
  • Kernel with debugging symbols
  • Minimal optimizations (-O0)
  • ~2MB binary
  • Suitable for GDB debugging

./build.sh --target=standalone --mode=full
  • Complete kernel
  • Includes all drivers
  • Includes test suite
  • Includes documentation
  • ~10MB total
  • Suitable for development

./build.sh --target=standalone --mode=verify
  • Production kernel
  • Includes formal proof verification
  • Axiom checks all proofs
  • Longer build time
  • Asserts correctness
```

### Omnisystem Integration Targets

```bash
./build.sh --target=omnisystem --include-uosc
  • Build Omnisystem with UOSC as kernel layer
  • UOSC interfaces available to Omnisystem
  • Seamless integration

./build.sh --target=omnisystem --include-uosc --uosc-mode=full
  • Build with full UOSC (all drivers, tests)
  • More comprehensive testing
  • Larger binary
```

### Platform-Specific Targets

```bash
./build.sh --target=x86_64
  • Intel/AMD 64-bit x86
  • Current primary platform
  • Full feature set

./build.sh --target=arm64
  • ARM 64-bit
  • Growing support
  • Some drivers may not be available

./build.sh --target=qemu
  • QEMU-specific optimizations
  • Emulated hardware assumptions
  • Faster test cycles
```

## Build Process

### Step 1: Configuration

```bash
./build.sh --config=custom.conf
```

**config/default.conf**:
```
# Architecture
ARCH=x86_64

# Build mode
MODE=production

# Optimization level
OPT_LEVEL=3

# Debug symbols
DEBUG=0

# Include tests
INCLUDE_TESTS=1

# Verify proofs
VERIFY_PROOFS=0

# Platform
PLATFORM=baremetal

# Bootloader
BOOTLOADER=GRUB

# Memory configuration
KERNEL_BASE_ADDR=0xFFFFFF8000000000
KERNEL_HEAP_SIZE=16MB
```

### Step 2: Compilation

```bash
# Step 2a: Compile bootloader
nasm -f elf64 -o build/objects/bootloader.o src/bootloader/boot.asm

# Step 2b: Compile kernel C code
gcc -c -O3 -nostdlib -fno-builtin \
    -Isrc/include \
    -o build/objects/kernel.o \
    src/kernel/*.c

# Step 2c: Compile architecture-specific code
gcc -c -O3 -nostdlib -fno-builtin \
    -Isrc/include \
    -o build/objects/arch.o \
    src/arch/x86_64/*.c src/arch/x86_64/*.asm

# Step 2d: Compile drivers
gcc -c -O3 -nostdlib -fno-builtin \
    -Isrc/include \
    -o build/objects/drivers.o \
    src/drivers/**/*.c

# Step 2e: Link kernel
ld -T linker.ld -o build/kernel/uosc-kernel.elf \
    build/objects/bootloader.o \
    build/objects/kernel.o \
    build/objects/arch.o \
    build/objects/drivers.o
```

### Step 3: Verification (Optional)

```bash
# Verify all formal proofs
axiom verify axiom/proofs/process_isolation.ax
axiom verify axiom/proofs/memory_safety.ax
axiom verify axiom/proofs/scheduling_fairness.ax
axiom verify axiom/proofs/no_deadlock.ax

# All proofs must verify for --mode=verify
```

### Step 4: Testing

```bash
# Build tests
gcc -o build/tests/test_kernel \
    tests/kernel_tests.c \
    -Ibuild/kernel \
    -Lbuild/kernel -luosc-kernel

# Run tests
./build/tests/test_kernel --verbose
```

### Step 5: Packaging

```bash
# Create bootable ISO (for QEMU/physical hardware)
grub-mkrescue -o build/uosc.iso build/kernel/

# Create Docker image
docker build -f Dockerfile.uosc -t uosc:latest .

# Create tarball
tar czf build/uosc-standalone.tar.gz \
    build/kernel/uosc-kernel.elf \
    build/drivers/ \
    include/uosc/ \
    docs/
```

## Build Flags

### Compiler Flags

```bash
# Mandatory for UOSC
-nostdlib           # Don't link C standard library
-fno-builtin        # Don't use built-in functions
-mcmodel=kernel     # Kernel code model
-fno-stack-protector # No stack protection (not applicable in kernel)
-mno-red-zone       # x86-64: No red zone

# Optimization
-O0 (debug)         # No optimization
-O2 (standard)      # Balanced optimization
-O3 (aggressive)    # Maximum optimization
-march=native       # Target current CPU

# Security
-fPIE               # Position-independent executable (if applicable)
-fstack-check=specific  # Stack overflow checking
```

### Linker Flags

```bash
-T linker.ld        # Use custom linker script
-nostdlib           # No standard library
--no-undefined      # Fail on undefined symbols
```

## Testing Build

### Unit Tests

```bash
./build.sh --target=standalone --mode=full --with-tests

# Run test suite
./build/tests/run_all_tests

# Run specific tests
./build/tests/test_kernel --filter="memory*"
./build/tests/test_scheduler
./build/tests/test_hypercalls
```

### Integration Tests

```bash
# Boot in QEMU and run tests
./verify.sh --qemu

# Expected output:
[UOSC] Kernel 1.0 booting...
[UOSC] CPU: Intel Core i7...
[UOSC] Memory: 8GB
[UOSC] Tests: PASSED (123/123)
[UOSC] System ready
```

### Performance Tests

```bash
./build/tests/benchmark_kernel

# Output:
Context Switch Latency:     0.8µs  (target: <1µs) ✓
Scheduler Decision:       95ns     (target: <100ns) ✓
Memory Allocation (4KB):  0.9µs    (target: <1µs) ✓
Page Table Lookup (TLB):  18ns     (target: <100ns) ✓
```

## Building with Axiom Proofs

### Proof Verification

```bash
# Enable proof checking during build
./build.sh --target=standalone --verify-proofs

# Build will fail if any proof is invalid
# This adds ~30 minutes to build time
```

### Proof Files

All proofs are in Axiom format:

```
axiom/proofs/
├── process_isolation.ax     → Memory isolation theorem
├── memory_safety.ax         → Memory consistency theorem
├── scheduling_fairness.ax   → CPU fairness theorem
├── no_deadlock.ax           → Deadlock-free proof
└── no_starvation.ax         → Starvation-free proof
```

## Cross-Compilation

### Building for Different Architecture

```bash
# Build for ARM64
./build.sh --arch=arm64 --target=standalone

# Build for different platform
./build.sh --arch=x86_64 --platform=cloud

# Supported architectures
--arch=x86_64       (primary)
--arch=arm64        (secondary)
--arch=riscv64      (experimental)
```

## Customization

### Custom Configuration

```bash
# Create custom config
cp config/default.conf config/my_config.conf
# Edit config/my_config.conf
./build.sh --config=config/my_config.conf
```

### Feature Selection

```bash
# Minimal kernel (no drivers except console)
./build.sh --target=standalone --features=minimal

# Standard kernel
./build.sh --target=standalone --features=standard

# Full kernel (all drivers)
./build.sh --target=standalone --features=full

# Custom features
./build.sh --target=standalone \
    --with-driver=console \
    --with-driver=timer \
    --with-driver=network
```

## Troubleshooting

### Build Failures

```bash
# Clean build
./build.sh clean
./build.sh --target=standalone --mode=production

# Verbose output
./build.sh --target=standalone --verbose 2>&1 | tee build.log

# Check compiler
gcc --version
clang --version
```

### Common Issues

**Issue: "gcc: command not found"**
```bash
# Install GCC
apt-get install build-essential    # Ubuntu/Debian
yum groupinstall "Development Tools"  # RedHat/CentOS
```

**Issue: "NASM not found"**
```bash
# Install NASM
apt-get install nasm               # Ubuntu/Debian
yum install nasm                   # RedHat/CentOS
```

**Issue: Proof verification fails**
```bash
# Disable verification for build
./build.sh --target=standalone --no-verify

# Then investigate failing proof
axiom verify axiom/proofs/[failing_proof].ax --verbose
```

## Installation

### Standalone Installation

```bash
# Copy kernel binary
sudo cp build/kernel/uosc-kernel.elf /boot/

# Copy headers (for driver development)
sudo cp -r include/uosc /usr/include/

# Copy documentation
sudo cp -r docs /usr/share/doc/uosc/
```

### Setting as Boot Kernel (GRUB)

```bash
# Add to /etc/grub.d/40_custom
menuentry "UOSC Kernel" {
    insmod gzio
    insmod part_msdos
    insmod ext2
    set root='(hd0,msdos1)'
    echo 'Loading UOSC kernel...'
    multiboot /boot/uosc-kernel.elf
}

# Update GRUB
sudo grub-mkconfig -o /boot/grub/grub.cfg
```

## Development Workflow

### Iterative Development

```bash
# 1. Modify source
vim src/kernel/process.c

# 2. Quick rebuild
./build.sh --target=standalone --mode=debug

# 3. Test
./build/tests/test_kernel

# 4. Debug if needed
gdb ./build/kernel/uosc-kernel.elf
```

### Documentation Building

```bash
# Build HTML documentation
./build.sh --docs-format=html

# Output in build/docs/html/
# Open: build/docs/html/index.html
```

## Performance Optimization

### Build-Time Optimization

```bash
# Parallel build (speed up)
./build.sh --jobs=8                # Use 8 parallel jobs

# Link-time optimization
./build.sh --lto=full              # Full LTO
./build.sh --lto=thin              # ThinLTO (faster)
```

## References

- [Architecture](../kernel/ARCHITECTURE.md)
- [Kernel Details](../kernel/PROCESS.md)
- [Testing Guide](TESTING.md)
- [Performance Guide](PERFORMANCE.md)

---

**UOSC Build: Simple, Fast, Verifiable, Modular.**
