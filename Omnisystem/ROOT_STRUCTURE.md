# Omnisystem Root Directory Structure

Clean, organized root directory for the Omnisystem project. This document describes the structure and purpose of each file and directory.

## Root Level Files (Essential Only)

### Configuration & Build Files
- `Cargo.toml` - Main Rust workspace manifest
- `Cargo.lock` - Locked dependency versions
- `Makefile` - GNU Make build configuration
- `build.toml` - Build system configuration
- `.gitignore` - Git ignore rules
- `.gitattributes` - Git attribute rules

### Documentation
- `README.md` - Project overview and quick start

### Containerization
- `Dockerfile` - Standard Docker container definition
- `Dockerfile.bmcs` - Specialized container for BMCS

### Source Code
- `Omnisystem.ti` - Main Omnisystem Titan language source file

## Top-Level Directories

### Core System Components

#### `/UOSC`
Universal Operating System Core - self-contained micro-kernel with complete documentation.
- Fully modular and stable
- Can be used independently of Omnisystem
- Complete documentation in `/UOSC/docs`

#### `/aether`
Aether language implementation and runtime.

#### `/aion`
AION subsystem - advanced processing capabilities.

#### `/axiom`
Axiom formal verification system and theorem prover.

#### `/Conductor`
Conductor orchestration and workflow management system.

### Development & Infrastructure

#### `/scripts`
Build, deployment, testing, and operational scripts.
- `deploy.sh` - Deployment automation
- `generate_sample_crates.sh` - Sample generation
- `build-all.sh` - Complete build orchestration
- `/ci` - Continuous integration scripts
- `/verification` - Verification test scripts
- `/cleanup` - Cleanup and maintenance scripts

#### `/config`
Configuration files for various systems and components.

#### `/crates` & `/crates-root`
Rust workspace crates and root configuration.

#### `/apps`
Application layer and examples.

### Data & Services

#### `/data`
Data files, datasets, and sample data.

#### `/database`
Database schemas, migrations, and data definitions.

#### `/connectors`
Integration connectors for external systems.

#### `/cli`
Command-line interface tools and utilities.

### Documentation & Reference

#### `/docs`
Complete documentation (references to embedded UOSC docs, Omnisystem overview, etc.)

#### `/archive_stale`
**ARCHIVED** Old development documentation and status reports.
- Phase completion documents
- Development status snapshots
- Superseded documentation
- Not for active use - kept for historical reference

### Build & Output

#### `/build`
Build outputs and artifacts.
- Compiled binaries
- Object files
- Generated files

#### `/artifacts`
Build artifacts and releases.

#### `/bindings`
Language bindings and FFI definitions.

### Optional/Specialized

#### `/database-adapters`
Database adapter implementations.

#### `/clojure`
Clojure integration and libraries.

#### `/aether-dns`
DNS system built on Aether.

#### `/coos`
Custom operating system components.

---

## Key Principles

### 1. Clean Root
- **Only essential files at root level**: Build configs, git config, main README
- **Loose documentation archived**: Old status/phase docs moved to `archive_stale/`
- **Scripts organized**: Build and deployment scripts in `scripts/` directory

### 2. Self-Contained Modules
- **UOSC is fully modular**: Complete with its own documentation
- **Each subsystem has clear boundaries**: Separate crates, configs, documentation
- **No dependency on loose files**: Everything in proper directories

### 3. Non-Destructive Organization
- **Historical files preserved**: In `archive_stale/` for reference
- **Git history preserved**: No destructive rewrites
- **Backward compatibility**: Existing build/deploy systems still work

### 4. Production-Ready Structure
- **Clear navigation**: Logical directory organization
- **Professional appearance**: No clutter or loose files
- **Proper isolation**: Development files separate from production code

---

## File Organization Summary

### Removed from Root (Archived)
| Item | Reason | Location |
|------|--------|----------|
| 49 status/consolidation markdown files | Development documentation | `archive_stale/` |
| Cargo.new.toml, Cargo.toml.bak, Cargo_Phase2.toml | Backup/alternate build files | `archive_stale/` |
| analyze_system.py | Development utility | `archive_stale/` |
| test_output.txt | Test artifacts | `archive_stale/` |

### Reorganized
| Item | From | To | Reason |
|------|------|-----|--------|
| deploy.sh | Root | `scripts/` | Organize deployment tools |
| generate_sample_crates.sh | Root | `scripts/` | Organize utility scripts |
| Omnisystem.exe | Root | `build/` | Organize build outputs |

### Preserved at Root (Essential)
| Item | Reason |
|------|--------|
| Cargo.toml, Cargo.lock | Main build configuration |
| Makefile, build.toml | Build system config |
| README.md | Project overview |
| Dockerfiles | Container definitions |
| .gitignore, .gitattributes | Git configuration |
| Omnisystem.ti | Main source file |

---

## Navigation Guide

### For Building Omnisystem
```bash
# Root level
cat Makefile
cat Cargo.toml

# Build scripts
cd scripts/
./build-all.sh
```

### For Deployment
```bash
# Deployment scripts
cd scripts/
./deploy.sh
```

### For UOSC Development
```bash
cd UOSC/
cat docs/README.md         # UOSC overview
cat docs/guides/BUILDING.md # Build UOSC
```

### For Old Documentation
```bash
# Historical reference
cd archive_stale/
# Browse old status/phase documents
```

---

## Maintenance Notes

- **Archive Stale**: For reference only, not updated
- **Scripts**: Keep organized by function (build, deploy, verify)
- **Root**: Keep minimal - only files that must be at root
- **Documentation**: Maintain in proper subdirectories with INDEX.md files

---

**Last Updated**: 2026-06-13  
**Status**: Clean, organized, production-ready
