# Contributing to BonsaiWorkspace

Thank you for your interest in contributing to BonsaiWorkspace! This document provides guidelines for contributing to the project.

## Overview

BonsaiWorkspace is organized as a three-layer Co-Operating System:

1. **UOSC (Layer 1)** - Microkernel with 9 kernel subsystems
2. **Omnisystem (Layer 2)** - OS services and polyglot languages
3. **BonsaiEcosystem (Layer 3)** - Applications and user interface

For detailed contribution guidelines specific to development, see [DOCS_CONTRIBUTING.md](BonsaiEcosystem/docs/production-docs/DOCS_CONTRIBUTING.md).

## Quick Start

### Prerequisites
- Git
- Rust toolchain (for Omnisystem services)
- Titan compiler (self-hosting, included in Omnisystem)
- PowerShell (for Windows) or Bash (for Linux/macOS)

### Setup

1. Clone the repository:
```bash
git clone https://github.com/your-org/bonsai-workspace.git
cd BonsaiWorkspace
```

2. Install dependencies:
```bash
# For Omnisystem (Rust services)
cd Omnisystem
cargo build --release

# For BonsaiEcosystem (GUI)
cd ../BonsaiEcosystem
# Build instructions vary by component
```

3. Run tests:
```bash
cd Omnisystem
cargo test --all
```

### Build Instructions

See [DOCS_OMNISYSTEM_BUILD.md](DOCS_OMNISYSTEM_BUILD.md) for detailed build instructions for all platforms.

## Repository Structure

```
BonsaiWorkspace/
├── README.md                              # Main documentation
├── CHANGELOG.md                           # Version history
├── CONTRIBUTING.md                        # This file
├── Omnisystem/                            # Layer 2: OS services & languages
│   ├── UOSC/                              # Layer 1: Microkernel
│   │   ├── kernel/                        # 9 kernel subsystems
│   │   ├── drivers/                       # Device drivers
│   │   ├── proofs/                        # Formal verification
│   │   └── UOSC_KERNEL_COMPLETE.md
│   ├── languages/                         # Titan, Sylva, Aether, Axiom
│   ├── services/                          # TransferDaemon, UMS, SLM, BMF, AI Shim, etc.
│   ├── Cargo.toml                         # Rust workspace
│   └── scripts/                           # Build and test scripts
└── BonsaiEcosystem/                       # Layer 3: Applications
    ├── workspace/                         # Bonsai Workspace (IDE)
    ├── buddy/                             # Bonsai Buddy (assistant)
    ├── control-panel/                     # System control
    ├── installer/                         # Platform installers
    └── docs/                              # Documentation
```

## Code Organization

### UOSC Kernel (Omnisystem/UOSC/)

All kernel code is in **Titan language**. Key files:

- `kernel/boot.ti` - Bootloader and initialization
- `kernel/memory.ti` - Memory management
- `kernel/scheduler.ti` - Process scheduling
- `kernel/ipc.ti` - Inter-process communication
- `kernel/sanctum.ti` - Hardware isolation
- `kernel/hypercall.ti` - Hypervisor interface
- `drivers/console.ti` - Serial/framebuffer output
- `drivers/timer.ti` - System timers
- `proofs/kernel_security.ax` - Formal verification (Axiom)

**Guidelines**:
- All code must be production-grade (no placeholders)
- All subsystems must be fully integrated
- All changes must maintain kernel invariants
- Add formal verification theorems for new critical code

### Omnisystem Services (Omnisystem/services/)

Core services in Rust or Titan:

- `TransferDaemon` - P2P networking
- `UMS` - Module system
- `SLM` - Service lifecycle
- `BMF` - Messaging
- `Container` - Container runtime
- `AI Shim` - Unified AI API

**Guidelines**:
- Follow Rust idioms and style guidelines
- Write tests for new functionality
- Document public APIs
- Maintain backwards compatibility when possible

### BonsaiEcosystem Applications (BonsaiEcosystem/)

User-facing applications in Sylva, Rust, or platform-native languages.

**Guidelines**:
- Follow language-specific conventions
- Write clear, maintainable code
- Test on target platforms before submitting
- Document user-facing features

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bugfix-name
```

### 2. Make Your Changes

- Keep commits atomic and well-documented
- One feature or fix per branch
- Follow the code style of the file/module you're editing

### 3. Test Your Changes

```bash
# For kernel changes
cd Omnisystem/UOSC
# Verify it compiles without errors

# For service changes
cd Omnisystem
cargo test --all

# For application changes
# Test on target platform
```

### 4. Commit with Clear Messages

```bash
git commit -m "type: Brief description

Longer explanation of the change, why it was made, and any relevant details.

Co-Authored-By: Your Name <your.email@example.com>"
```

**Commit types**:
- `feat:` - New feature
- `fix:` - Bug fix
- `refactor:` - Code reorganization
- `docs:` - Documentation
- `test:` - Test additions/fixes
- `chore:` - Build, CI, or tooling

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:
- Clear title describing the change
- Detailed description of what was changed and why
- List of any breaking changes
- Test results or verification steps

## Pull Request Requirements

All pull requests must:

- ✅ Have a clear, descriptive title
- ✅ Include detailed description of changes
- ✅ Pass all existing tests
- ✅ Add tests for new functionality
- ✅ Update documentation if needed
- ✅ Follow code style guidelines
- ✅ Have no merge conflicts
- ✅ Be reviewed by at least one maintainer

## Code Style

### Titan (Kernel)

- Use snake_case for function and variable names
- Use PascalCase for type names
- Document public functions with comments
- Keep functions focused and concise
- Use Result types for error handling

### Rust (Services)

- Follow `rustfmt` formatting
- Use clippy to check for common issues
- Document public APIs with doc comments
- Write tests using `#[test]` or similar frameworks

### Sylva (UI)

- Follow language conventions
- Clear, readable variable and function names
- Consistent indentation and formatting

## Reporting Issues

Found a bug? Have a feature request? Please open an issue on GitHub with:

- **Bug reports**: Clear reproduction steps, expected vs actual behavior, error logs
- **Feature requests**: Use case, proposed API/design, any relevant examples
- **Documentation issues**: What's unclear or missing

## Questions?

- **Technical questions**: See [DOCS_CONTRIBUTING.md](BonsaiEcosystem/docs/production-docs/DOCS_CONTRIBUTING.md)
- **Architecture questions**: See [FACTUAL_REPOSITORY_DOCUMENTATION.md](FACTUAL_REPOSITORY_DOCUMENTATION.md)
- **Build issues**: See [DOCS_OMNISYSTEM_BUILD.md](DOCS_OMNISYSTEM_BUILD.md)

## License

By contributing to BonsaiWorkspace, you agree that your contributions will be licensed under the same license as the project.

---

**Thank you for contributing to BonsaiWorkspace!** 🚀
