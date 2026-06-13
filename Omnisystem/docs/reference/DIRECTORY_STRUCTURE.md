# BonsaiWorkspace Directory Structure

## Overview

This document describes the complete organization of the BonsaiWorkspace repository.

## Root Level Files (Essential)

```
Z:\Projects\BonsaiWorkspace\
├── README.md                      # Main project overview and getting started
├── START_HERE.md                  # Quick start guide for new users
├── GETTING_STARTED.md             # Detailed setup and installation
├── SECURITY.md                    # Security policies and vulnerability disclosure
├── CONTRIBUTING.md                # Contribution guidelines
├── CHANGELOG.md                   # Version history and release notes
├── Cargo.toml                     # Rust workspace configuration
├── Cargo.lock                     # Rust dependency lock file
└── Dockerfile.bmcs                # Docker configuration for BMCS
```

## Core Project Directories

### `crates/` - Rust Crates
Main Rust implementation directory. Each subdirectory is a Rust crate in the workspace:

```
crates/
├── bonsai-buir/                   # Bonsai Unified Intermediate Representation
├── bonsai-bco/                    # Bonsai Compiled Objects (serialized functions)
├── bonsai-compile-cache/          # Local compilation cache
├── bonsai-cas-ext/                # CAS extensions for BACE
├── bonsai-hotreload/              # Hot-reload macro and runtime
├── bace-rustc/                    # BACE Rust compiler wrapper
├── bace-rt/                       # BACE runtime and proc-macro
├── cargo-bace/                    # cargo-bace CLI subcommand
├── UOSC-kernel/                   # UOSC Co-OS x86_64 bare-metal kernel
├── compiler-cache/                   # Speculative pre-compilation with AI hints
├── msg-core/               # Core messaging types and encryption
├── msg-smtp/               # SMTP server implementation
├── msg-imap/               # IMAP server implementation
├── msg-p2p/                # P2P delivery via TransferDaemon
├── msg-server/             # Unified BMF server
├── bonsai-backend/                # BUEB (Universal Execution Backend) - Hardware abstraction
│   ├── src/
│   │   ├── lib.rs                 # Public API and initialization
│   │   ├── types.rs               # Data structures
│   │   ├── detect.rs              # Hardware detection module
│   │   ├── allocator.rs           # Device allocation algorithm
│   │   └── cpu.rs                 # CPU optimization operations
│   ├── examples/
│   │   ├── detect_hardware.rs     # Hardware profiling example
│   │   └── octopus_integration.rs # Octopus AI integration demo
│   ├── BUEB.md                    # Comprehensive BUEB documentation
│   └── Cargo.toml
├── bonsai-kdb/                    # Knowledge Database
├── octopus-ai/                    # Octopus AI model training
│   ├── prepare_data.py            # Generate training data
│   ├── train_psychopathy.py       # Model training (DistilGPT-2)
│   └── merge_and_convert.py       # Merge LoRA adapter and convert
└── ...
```

### `bonsai-workspace/` - Tauri IDE
Desktop application workspace:

```
bonsai-workspace/
├── src-tauri/                     # Rust backend (Tauri)
├── src/                           # Svelte frontend
└── package.json
```

### `bonsai-bot/` - Bot Framework
Bot implementation directory

### `bonsai-runtime/` - Runtime
Runtime libraries and utilities

### `android-runtime/` - Android App
Android companion app:

```
android-runtime/
├── app/
│   ├── build.gradle.kts
│   ├── src/main/
│   │   ├── AndroidManifest.xml
│   │   ├── java/ai/bonsai/buddy/
│   │   │   └── MainActivity.kt
│   │   └── res/
│   └── ...
└── ...
```

### Other Project Directories

- `bonsai-native/` - Native bindings and FFI
- `browser-extension/` - Browser extension
- `deploy/` - Deployment configurations

## Documentation Structure

### `docs/` - Complete Documentation

```
docs/
├── DIRECTORY_STRUCTURE.md         # This file - complete directory layout
├── INDEX.md                       # Documentation index (see below)
│
├── specifications/                # Technical specifications
│   ├── BPCF_PRE_SPECIFICATION.md
│   ├── BMF_MESSAGING_SPECIFICATION.md
│   ├── UBSS_SPECIFICATION.md
│   ├── BUEB.md                    # Hardware abstraction layer
│   └── [other specs]
│
├── guides/                        # How-to guides and tutorials
│   ├── BUILD_AND_RUN_GUIDE.md
│   ├── GETTING_STARTED_GUIDE.md
│   ├── QUICKSTART_MOBILE_TRAINING.md
│   ├── UACS_HITL_GUIDE.md
│   └── [other guides]
│
├── status-reports/                # Project status documents
│   ├── BUEB_STATUS.md             # BUEB completion report
│   ├── IMPLEMENTATION_STATUS.md
│   ├── DEPLOYMENT_STATUS.md
│   ├── ANDROID_BRIDGE_STATUS.md
│   └── [other status reports]
│
└── archive/                       # Historical/obsolete documentation
    ├── COMPLETE_IMPLEMENTATION.md
    ├── FINAL_DELIVERY_STATUS.md
    ├── BUG_HUNTER_TRAINING_LOG.md
    └── [historical documents]
```

## Scripts Directory

### `scripts/` - Automation and Utilities

```
scripts/
├── powershell/                    # Windows PowerShell scripts
│   ├── build-and-run.ps1          # Full build and execution
│   ├── build-complete-system.ps1
│   ├── build-launch.ps1
│   ├── fix-and-train.ps1
│   ├── fix-python-deps.ps1
│   ├── install-python.ps1
│   ├── setup-compilation-cache.ps1
│   ├── windows-full-setup.ps1
│   ├── windows-gpu-build.ps1
│   ├── windows-setup-minimal.ps1
│   ├── watch-build-live.ps1
│   ├── auto-build-orchestrator.ps1
│   └── [other scripts]
│
└── shell/                         # Linux/macOS shell scripts
    ├── verify_android_bridge_integration.sh
    ├── START_UACS.sh
    └── [other scripts]
```

### Quick Script Reference

| Script | Purpose |
|--------|---------|
| `build-and-run.ps1` | Build and launch the IDE |
| `build-complete-system.ps1` | Full system build |
| `windows-full-setup.ps1` | Complete Windows setup |
| `fix-python-deps.ps1` | Fix Python dependency issues |
| `setup-compilation-cache.ps1` | Configure BACE compilation cache |
| `auto-build-orchestrator.ps1` | Automated build orchestration |

## Configuration Files

### `config/` - Configuration and Settings

```
config/
├── bonsai-ci.yaml                 # CI/CD configuration
├── bonsai-ci-complete.yaml
├── bonsai-ecosystem.yaml           # Ecosystem configuration
├── bmcs.config.toml               # BMCS configuration
├── bonsai-ci-config.json
├── docker-compose.bmcs.yml        # Docker Compose for BMCS
└── [other configs]
```

## Data Directory

### `data/` - Data Files and Datasets

```
data/
├── KNOWLEDGE_DATABASE.json        # Knowledge base (if large, may be in separate storage)
├── SURVIVAL_SYSTEM.sqlite.json    # Persistence storage
├── SURVIVAL_SYSTEM_EXTENDED.json
└── [training datasets, models, etc.]
```

## Training Data

### `training-data/` - ML Training Resources

```
training-data/
├── train.jsonl                    # 9,000 training examples
├── validation.jsonl               # 1,000 validation examples
├── train.txt                      # Combined text format
└── [other datasets]
```

## Models

### `psychopathy-octopus-merged/` - Trained Model

```
psychopathy-octopus-merged/
├── pytorch_model.bin              # 312.49 MB merged LoRA+base model
├── config.json
├── tokenizer.json
└── special_tokens_map.json
```

## Logs Directory

### `logs/` - Build and Execution Logs

```
logs/
├── build.log                      # Build output
├── build-rust.log
├── build-launch.log
├── training.log                   # Model training logs
├── phase1.log
├── phase2.log
├── phase3.log
├── phase4.log
├── phase5.log
├── mcp-server.log
├── mcp-server-error.log
└── [other logs]
```

## Python Virtual Environment

### `.venv-ml/` - ML Training Environment

Isolated Python virtual environment for machine learning:

```
.venv-ml/
├── Lib/site-packages/
│   ├── torch==2.0.1+cpu
│   ├── transformers==4.30.0
│   ├── datasets==2.10.1
│   ├── peft==0.7.0 (LoRA support)
│   └── [other packages]
├── Scripts/
│   ├── python.exe
│   └── pip.exe
└── [venv structure]
```

## Key Files by Category

### Getting Started
- **START_HERE.md** - Primary entry point for new users
- **GETTING_STARTED.md** - Detailed setup instructions
- **README.md** - Project overview

### Development
- **Cargo.toml** - Workspace configuration
- **docs/guides/** - Development guides
- **scripts/powershell/** - Build automation

### Documentation
- **docs/specifications/** - Technical specifications
- **docs/status-reports/** - Project status
- **docs/guides/** - How-to guides
- **docs/archive/** - Historical documents

### Configuration
- **config/** - All configuration files
- **Dockerfile.bmcs** - Container definition

### Data & Models
- **data/** - Knowledge bases, datasets
- **training-data/** - ML training data
- **psychopathy-octopus-merged/** - Trained Octopus AI model

## File Organization Principles

1. **Clarity**: Each file is in its logical category
2. **Discoverability**: Use meaningful names and organize by purpose
3. **Scalability**: Structure allows for growth
4. **Separation of Concerns**: Code, docs, config, scripts are separate
5. **Accessibility**: Important files are at root or docs/

## Navigation Guide

### I want to...

**Get started with the project**
→ Read `START_HERE.md` → `GETTING_STARTED.md` → `README.md`

**Build and run the IDE**
→ Run `scripts/powershell/build-and-run.ps1`

**Understand the architecture**
→ Read `docs/specifications/BONSAI_ARCHITECTURE.md`

**See project status**
→ Check `docs/status-reports/`

**Configure the system**
→ Edit files in `config/`

**View build logs**
→ Check `logs/`

**Understand BUEB (Hardware Backend)**
→ Read `crates/bonsai-backend/BUEB.md`

**Find old documentation**
→ Check `docs/archive/`

## Archive Policy

Documentation older than 6 months or marked as "obsolete" is moved to `docs/archive/` but retained for historical reference. This keeps the main documentation directory current and focused.

## Adding New Files

When adding new files:

1. **Code**: Add to appropriate `crates/*/` directory
2. **Documentation**: Add to `docs/{specifications|guides|status-reports}/`
3. **Scripts**: Add to `scripts/{powershell|shell}/`
4. **Configuration**: Add to `config/`
5. **Data**: Add to `data/` or `training-data/`
6. **Logs**: Go to `logs/` automatically

## Summary

The reorganized structure provides:

✅ **Clean root directory** - Only essential files  
✅ **Organized documentation** - By category and purpose  
✅ **Clear scripts location** - All automation in scripts/  
✅ **Centralized configuration** - All configs in config/  
✅ **Separated concerns** - Code, docs, config, scripts distinct  
✅ **Historical archive** - Old docs preserved but not cluttering  
✅ **Easy navigation** - Logical folder hierarchy  

This organization makes the project more maintainable and professional.
