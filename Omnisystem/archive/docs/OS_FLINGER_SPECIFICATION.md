# OS Flinger – Complete Architectural Specification

**Status:** Design Phase – Production Blueprint Ready  
**Version:** 1.0.0 (Pre-Implementation)  
**Date:** 2026-06-05  
**Scope:** Next-generation, production-grade system migration fabric for Omnisystem  
**Target Systems:** Any Linux distribution → Omnisystem (0%-100% integration)  
**Scale:** Single machines to 100,000+ node fleets  

---

## EXECUTIVE SUMMARY

**OS Flinger** is a **statically-linked, Titan-compiled binary** that performs **safe, auditable, fully-recoverable in-place conversions** of any Linux system to the Omnisystem. It is fundamentally different from `nixos-infect`:

| Aspect | nixos-infect | OS Flinger |
|--------|--------------|-----------|
| **Implementation** | Shell script | Titan binary (compiled, static) |
| **Safety Model** | High-risk, linear | Multi-phase, transactional, atomic |
| **Rollback** | Manual/impossible | Automatic, tested, verified |
| **Integration** | None | Full Omnisystem stack |
| **Fleet Support** | Not supported | Parallel 100,000+ machines |
| **Verification** | None | UVM post-conversion tests |
| **Formal Proofs** | None | Axiom correctness proofs |
| **Deployment Modes** | Single | 5 modes (0%-100% integration) |

---

## 1. ARCHITECTURAL OVERVIEW

### 1.1 Component Architecture

```
┌────────────────────────────────────────────────────────────────────┐
│                    User Workstation / Controller                   │
│                                                                    │
│  Command: os-flinger convert --target 10.0.0.50                  │
│           --config omnisystem.nix --mode hosted-full             │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ OS Flinger Client (Titan binary)                             │ │
│  │  • Parses configuration                                      │ │
│  │  • Initiates conversion workflow                             │ │
│  │  • Monitors progress via TransferDaemon                      │ │
│  │  • Receives status updates and audit logs                    │ │
│  └──────────────────────────────────────────────────────────────┘ │
└────────────────────┬───────────────────────────────────────────────┘
                     │ TransferDaemon (encrypted, multi-path, P2P)
                     │
┌────────────────────▼───────────────────────────────────────────────┐
│              Target Machine (Existing Linux System)                │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ OS Flinger Agent (Aether Actor)                             │ │
│  │  • Pre-flight inventory (async)                             │ │
│  │  • Configuration validation (deterministic solver)          │ │
│  │  • Snapshot creation (atomic)                               │ │
│  │  • Phase transitions (state machine)                        │ │
│  │  • Rollback on failure (automatic)                          │ │
│  │  • UVM integration (post-verification)                      │ │
│  │  • Audit logging (immutable)                                │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ Sanctum Vault (Isolated execution environment)              │ │
│  │  • Agent runs with only required capabilities:              │ │
│  │    - storage: /dev, /sys (block device access)             │ │
│  │    - network: TransferDaemon (artifact download)            │ │
│  │    - none: filesystem (read-only mount of /)               │ │
│  └──────────────────────────────────────────────────────────────┘ │
└────────────────────┬───────────────────────────────────────────────┘
                     │ TransferDaemon (UMS artifact fetching)
                     │
┌────────────────────▼───────────────────────────────────────────────┐
│         Omnisystem Infrastructure (Distributed, Redundant)         │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ UMS Registry (Universal Module System)                      │ │
│  │  • Kernel binaries (UOSC, all architectures)                │ │
│  │  • initrd/bootloaders (verified BLS signatures)             │ │
│  │  • Service modules (transfer-daemon, ums, services)         │ │
│  │  • Content-addressed (BLAKE3 hashes)                        │ │
│  │  • Replicated globally, accessible via P2P                  │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ TransferDaemon Mesh (P2P Distribution)                      │ │
│  │  • Multi-path routing (reliability)                         │ │
│  │  • Post-quantum encryption (security)                       │ │
│  │  • NAT traversal (accessibility)                            │ │
│  │  • Multi-hop peer discovery                                 │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ UVM Agents (Universal Validation Mesh)                      │ │
│  │  • Post-conversion test execution                           │ │
│  │  • Deterministic replay validation                          │ │
│  │  • Performance baseline verification                        │ │
│  │  • Capability enforcement testing                           │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ audit-log Service (Immutable Record)                        │ │
│  │  • Conversion start/success/failure/rollback events         │ │
│  │  • Hardware inventory snapshots                             │ │
│  │  • Configuration applied                                    │ │
│  │  • Test results from UVM                                    │ │
│  │  • Cryptographically signed (council keys)                  │ │
│  └──────────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────────┘
```

### 1.2 Distributed State Model

OS Flinger operates on a **distributed state machine** where the client maintains a view of each target's conversion state. State transitions are **idempotent** – repeating a command at any phase produces the same result.

```
State Enum (Titan):
pub enum ConversionState {
    Idle,           // No conversion in progress
    PreFlight,      // Inventory collection
    Validated,      // Configuration validated, awaiting decision
    Snapshot,       // Snapshot creation in progress
    Staged,         // Artifacts downloaded, ready to deploy
    Deploying,      // Kernel/services being installed
    Finalizing,     // Bootloader/systemd updated
    Verifying,      // UVM test suite running
    Done,           // Conversion successful
    Failed,         // Conversion halted (awaiting rollback decision)
    RollingBack,    // Automatic/manual rollback in progress
    Rolledback,     // System restored to pre-conversion state
}
```

---

## 2. THE TRANSACTIONAL CONVERSION STATE MACHINE

OS Flinger models conversion as a **finite state automaton with explicit, auditable phases**. Each phase is **atomic** – it either succeeds completely or fails, triggering rollback.

### 2.1 Phase Diagram

```
     ┌────────────────────────────────────────────────────────────────┐
     │                      CONVERSION WORKFLOW                       │
     └────────────────────────────────────────────────────────────────┘

[IDLE]
  │
  ├──user cmd: analyze────────────────────────────┐
  │                                                 │
  ▼                                                 │
[PREFLIGHT_INVENTORY]  ◄─────────────────────────┘
  │ (async hardware/OS/storage detection)
  │ (constraint satisfaction check)
  │ (configuration validation)
  │ (generates detailed plan)
  │
  └──if --dry-run: exit [IDLE] (no changes made)
  │
  └──if OK: proceed to snapshot phase
  │
  ▼
[SNAPSHOT]
  │ (local: LVM/Btrfs/ZFS snapshot)
  │ (or remote: stream compressed image to UMS CAS)
  │ (or if unavailable: warn user, require explicit --accept-no-snapshot)
  │
  ├──if snapshot fails & no --force───→ [FAILED] ──→ rollback to [IDLE]
  │
  ▼
[STAGE_ARTIFACTS]
  │ (fetch kernel, initrd, service binaries from UMS)
  │ (verify BLS signatures via Bonsai Council keys)
  │ (store in staging area: /omnisystem-staging or tmpfs)
  │ (log all downloads to audit-log)
  │
  ├──if download fails──→ [FAILED] ──→ rollback
  ├──if signature invalid──→ [FAILED] ──→ rollback (security halt)
  │
  ▼
[DEPLOY]
  │ (mode-specific deployment logic)
  │
  ├──[hosted-light]: install systemd units (non-destructive)
  ├──[hosted-full]: create QEMU domain, virtio config
  ├──[bare-metal]: rewrite bootloader, root filesystem
  ├──[hybrid]: install UOSC kernel, package existing OS into Sanctum vault
  └──[library-os]: compile UOSC + app, stage single binary
  │
  ├──if deploy fails──→ [FAILED] ──→ rollback
  │
  ▼
[FINALIZE]
  │ (apply bootloader changes)
  │ (for destructive modes: mark for reboot, or trigger immediate reboot)
  │ (update /etc/omnisystem-config with final state)
  │
  ├──if finalize fails──→ [FAILED] ──→ rollback
  │
  ▼
[VERIFY] (only for hosted-light, hosted-full, hybrid; skip library-os if offline)
  │ (invoke UVM test suite)
  │ (run capability enforcement tests)
  │ (check core services: TransferDaemon, UMS, VFS, service-manager)
  │ (optional: run user-provided test script)
  │ (measure performance baseline)
  │
  ├──if tests fail──→ [FAILED] ──→ automatic rollback
  │
  ▼
[DONE]
  │ (cleanup: remove agent, staging area)
  │ (log successful conversion to audit-log)
  │ (create recovery snapshot/image for future rollback)
  │
  └──system ready for use

[FAILED]
  │ (phase failure detected)
  │ (emit detailed error message)
  │ (user prompted: automatic rollback or --manual-rollback)
  │
  ├──automatic rollback──→ [ROLLING_BACK]
  │
  ▼
[ROLLING_BACK]
  │ (restore snapshot or image)
  │ (verify system state matches pre-conversion)
  │ (log rollback event with details)
  │
  ▼
[ROLLED_BACK]
  │ (system returned to original state)
  │ (user can retry with different config or --force)
  │
  └──transition to [IDLE]
```

### 2.2 Atomicity Guarantees

Each phase is **atomic** in the sense that:

1. **Pre-transition check**: Before entering a phase, verify all conditions are met.
2. **Transactional action**: Execute the phase's core logic (e.g., write kernel to disk).
3. **Post-transition validation**: Confirm the system is in the expected state.
4. **Rollback on failure**: If validation fails, reverse the transition and halt.

**Example: DEPLOY phase for hosted-light**

```titan
// Pseudocode for atomicity
phase_deploy_hosted_light(config: OmnisystemConfig) -> Result<(), Error> {
    // Pre-transition check
    if !systemd_available() { return Err(SystemdNotFound); }
    if !network_available() { return Err(NetworkDown); }
    
    // Transactional action
    for service in &config.services {
        let unit = generate_systemd_unit(service)?;
        write_to(format!("/etc/systemd/system/{}.service", service.name), unit)?;
    }
    systemctl_daemon_reload()?;
    
    // Post-transition validation
    for service in &config.services {
        if !systemctl_is_enabled(service.name)? {
            return Err(ServiceNotEnabled);
        }
    }
    
    // Log transition
    audit_log_transition(ConversionState::Deploy, ConversionState::Finalize)?;
    
    Ok(())
}
```

---

## 3. DEPLOYMENT MODES – 0% TO 100% INTEGRATION SPECTRUM

OS Flinger supports **five distinct deployment modes**, each with different safety/commitment tradeoffs.

### 3.1 Mode: `hosted-light` (0% → 40% Integration)

**Definition**: Omnisystem userspace services (TransferDaemon, UMS, OmniCloak) are installed as systemd units alongside the existing OS. The host kernel and filesystem are untouched.

**Safety**: ✅ Maximum – fully non-destructive, can be uninstalled like any package

**Rollback**: Remove systemd units and packages. Zero data loss risk.

**Conversion Time**: < 5 minutes (download + unit installation)

**Use Cases**:
- Developer testing Omnisystem tooling
- Safe on-ramp before deeper integration
- Evaluation in production environments

**Configuration Example**:
```nix
services.omnisystem = {
  enable = true;
  mode = "hosted-light";
  services = {
    transfer-daemon = true;
    ums = true;
    omnicloak = true;
  };
  adapter.memory_mb = 256;  # Host handles memory
};
```

**Conversion Steps**:
1. Fetch Omnisystem service binaries from UMS
2. Generate systemd unit files
3. Write to `/etc/systemd/system/`
4. `systemctl daemon-reload && systemctl enable omnisystem-*`
5. Services start immediately

**Verification**: UVM validates that TransferDaemon is listening on port 9000, UMS on 9001, services are responsive.

---

### 3.2 Mode: `hosted-full` (40% → 80% Integration)

**Definition**: A complete UOSC kernel runs inside a KVM/QEMU guest VM. The host filesystem is shared via 9p. All Omnisystem services run in the guest.

**Safety**: ✅ Very High – strong isolation boundary, host completely preserved

**Rollback**: Remove VM definition and disk image. Host filesystem untouched.

**Conversion Time**: 10–20 minutes (VM setup, Omnisystem install in guest)

**Use Cases**:
- Production servers needing full Omnisystem without downtime
- Workstations running both legacy + Omnisystem
- Testing before `bare-metal` migration

**Configuration Example**:
```nix
services.omnisystem = {
  enable = true;
  mode = "hosted-full";
  adapter.memory_mb = 2048;
  adapter.cpus = 4;
  adapter.gpu_passthrough = true;  # Optional
  services = { /* all services enabled */ };
};
```

**Conversion Steps**:
1. Check/install QEMU/KVM
2. Create QEMU disk image (Btrfs or qcow2)
3. Boot UOSC kernel in guest from UMS
4. Mount host filesystem via 9p
5. Install Omnisystem services in guest
6. Configure virtio devices (net, block, console)
7. Update bootloader to boot guest on next reboot

**Special Features**:
- **GPU passthrough**: Via IOMMU groups, if `gpu_passthrough=true`
- **Live migration**: VM can be paused, snapshots, migrated to another host
- **Nested virtualization**: Guest can run additional VMs

**Verification**: UVM boots guest, runs full test suite inside VM, verifies bidirectional filesystem access (host ↔ guest).

---

### 3.3 Mode: `bare-metal` (0% Integration → 100%)

**Definition**: Complete replacement. The host OS is wiped. UOSC becomes the primary kernel, Omnisystem services run natively.

**Safety**: ⚠️ Destructive – requires full snapshot + explicit user confirmation

**Rollback**: Restore snapshot/image from remote storage. Time varies (from minutes to hours depending on size).

**Conversion Time**: 30–60 minutes (depending on disk size, network speed)

**Use Cases**:
- Full commitment to Omnisystem
- Dedicated Omnisystem servers
- IoT devices, edge nodes

**Pre-requisites**:
- User must explicitly confirm `--mode bare-metal` (no default)
- Must have snapshot support OR accept `--accept-no-snapshot`
- Must have ≥ 512 MB RAM for UOSC + services

**Conversion Steps**:
1. Create full system snapshot (local or remote)
2. Download UOSC kernel, initrd, root filesystem image from UMS
3. Reformat root partition with ext4/Btrfs/XFS
4. Extract root filesystem image
5. Install bootloader (GRUB, systemd-boot, EFI)
6. Configure kernel command-line parameters
7. Reboot into UOSC
8. UVM validates all services operational

**Recovery**: In case of failure, restore snapshot and reboot. System returns to pre-conversion state.

---

### 3.4 Mode: `hybrid` (50% → 75% Integration)

**Definition**: UOSC becomes the primary kernel. The original OS is packaged into a **Sanctum vault** and runs as a guest container inside Omnisystem. Legacy applications run unmodified.

**Safety**: ✅ High – original OS preserved, but UOSC takes control

**Rollback**: Delete vault, restore original bootloader. Time: minutes.

**Conversion Time**: 45–90 minutes

**Use Cases**:
- Gradual migration: modernize host while preserving legacy apps
- Run proprietary software that requires original OS
- Testing transition before full `bare-metal` migration

**Conversion Steps**:
1. Create snapshot of entire root filesystem
2. Package existing OS into Sanctum vault (tar archive, compressed)
3. Install UOSC kernel and Omnisystem services
4. Create Sanctum config: mount original filesystem read-only at `/compat`
5. Modify original systemd units to run inside vault
6. Reboot into UOSC
7. UVM validates UOSC core + vault functionality

**Container Integration**:
- Original `/` is mounted read-only inside vault at `/compat/`
- User data directories (`/home`, `/var`) are mounted read-write
- System services (sshd, web server) can run inside vault or in Omnisystem
- Network is shared; vault process doesn't have independent NIC

---

### 3.5 Mode: `library-os` (100% Integration → Unikernel)

**Definition**: UOSC is compiled as a static Titan library and linked with a single application. The result is a single bootable binary with no separate userspace.

**Safety**: ✅ Maximum – no external dependencies, pure binary

**Rollback**: Replace binary with previous version. Instant.

**Conversion Time**: 5–10 minutes (download + configure boot)

**Use Cases**:
- IoT devices with limited storage (≤ 100 MB)
- Serverless/container appliances
- Single-purpose microservices
- Reproducible, hermetic deployments

**Configuration Example**:
```nix
services.omnisystem = {
  enable = true;
  mode = "library-os";
  library_os.app = "my-microservice";  # Single Titan app
  adapter.memory_mb = 32;  # Minimal footprint
};
```

**Conversion Steps**:
1. Fetch pre-compiled UOSC library + target application binary from UMS
2. Link binary (or: download source, compile locally if toolchain available)
3. Stage bootloader (UEFI, Multiboot2, or device-specific)
4. Write to `/boot/` or flash to device
5. Update bootloader config

**Special Features**:
- **Reproducible builds**: BLAKE3 hash of binary matches globally
- **Instant rollback**: Keep previous binary in `/boot.bak/`, one grub-set-default away
- **No dependencies**: Everything is compiled in (no glibc, no standard userspace)

---

### 3.6 Mode Selection Matrix

| Mode | Destructive | Risk | Data Loss Risk | Rollback Speed | Use Case |
|------|-------------|------|----------------|----------------|----------|
| hosted-light | ❌ No | Minimal | 0% | Instant | Testing, dev |
| hosted-full | ❌ No | Very low | 0% | Seconds | Production, workstations |
| bare-metal | ✅ Yes | High | ~0% (if snapshot) | Minutes–hours | Full commitment |
| hybrid | ⚠️ Partial | Moderate | ~0% | Seconds | Gradual migration |
| library-os | ❌ No | Minimal | 0% | Instant | IoT, appliances |

---

## 4. PRE-FLIGHT INVENTORY ENGINE

### 4.1 System Manifest Structure

Before any modification, OS Flinger collects a detailed inventory of the target system. This is encoded in a **System Manifest** (Titan struct) that serves as the specification for the conversion.

```titan
// os-flinger/inventory/manifest.ti

pub struct SystemManifest {
    pub timestamp: u64,          // When was this manifest generated?
    pub version: String,         // OS Flinger version
    pub hostname: String,
    pub os: OsInfo,
    pub hardware: HardwareProfile,
    pub storage: StorageLayout,
    pub network: NetworkConfig,
    pub boot: BootConfig,
    pub user_config: OmnisystemConfig,  // Parsed user's omnisystem.nix
    pub constraints: Vec<Constraint>,   // Validation warnings/errors
    pub recommended_mode: DeploymentMode,
    pub conversion_plan: ConversionPlan,
}

pub struct HardwareProfile {
    pub arch: Architecture,      // x86_64, aarch64, riscv64, arm32
    pub cpu_cores: u32,
    pub cpu_model: String,       // E.g., "Intel Core i7-9700K"
    pub total_memory_mb: u64,
    pub memory_layout: Vec<MemoryModule>,
    pub gpu: Option<GpuInfo>,    // Model, VRAM, PCIe ID
    pub tpm_version: Option<TpmVersion>,  // 1.2, 2.0
    pub virtualization_support: VirtualizationSupport,  // KVM, Xen, Hyper-V
    pub storage_controllers: Vec<StorageController>,   // AHCI, NVMe, RAID
}

pub struct OsInfo {
    pub name: String,            // Ubuntu, Debian, CentOS, etc.
    pub version: String,         // 20.04, 11, 8, etc.
    pub kernel_version: String,
    pub package_manager: PackageManager,
    pub init_system: InitSystem,
    pub filesystem_root: FileSystemType,
    pub cloud_provider: Option<CloudProvider>,  // AWS, Azure, GCP, DigitalOcean
}

pub struct StorageLayout {
    pub root_device: String,     // /dev/sda, /dev/nvme0n1, etc.
    pub root_partition: String,  // /dev/sda1, /dev/nvme0n1p3
    pub root_fs_type: FileSystemType,
    pub boot_device: Option<String>,     // May be same as root
    pub boot_fs_type: Option<FileSystemType>,
    pub efi_partition: Option<EfiInfo>,
    pub snapshot_support: SnapshotSupport,  // LVM, Btrfs, ZFS, or None
    pub free_space_mb: u64,      // Available space on root partition
    pub total_size_mb: u64,
    pub partitions: Vec<PartitionInfo>,
    pub lvm_volumes: Option<Vec<LvmVolume>>,
    pub btrfs_subvols: Option<Vec<BtrfsSubvol>>,
    pub zfs_pools: Option<Vec<ZfsPool>>,
}

pub struct NetworConfig {
    pub interfaces: Vec<NetworkInterface>,
    pub hostname_resolvable: bool,
    pub dns_servers: Vec<String>,
    pub default_gateway: Option<String>,
    pub ipv6_support: bool,
    pub mtu: u16,
}

pub struct BootConfig {
    pub bootloader: BootLoader,  // GRUB2, systemd-boot, UEFI, Multiboot2
    pub boot_method: BootMethod, // UEFI, BIOS, Multiboot2
    pub secure_boot: bool,
    pub boot_timeout_seconds: u8,
}

pub struct Constraint {
    pub level: ConstraintLevel,  // Error, Warning, Info
    pub message: String,
    pub recommendation: Option<String>,
}

pub enum ConstraintLevel {
    Error,    // Blocks conversion unless --force
    Warning,  // Conversion will proceed, but with risk
    Info,     // Informational only
}

pub struct ConversionPlan {
    pub mode: DeploymentMode,
    pub phases: Vec<Phase>,
    pub estimated_time_seconds: u32,
    pub estimated_space_needed_mb: u64,
    pub rollback_strategy: RollbackStrategy,
}

pub enum RollbackStrategy {
    LocalSnapshot,       // LVM snapshot, Btrfs snapshot, ZFS snapshot
    RemoteImage,        // Compressed image stored in UMS CAS
    ManualRecovery,     // No automatic rollback possible
}
```

### 4.2 Hardware Detection

The **InventoryAgent** (Aether actor) runs on the target and collects hardware info by inspecting:

```bash
# Architecture
uname -m

# CPU info
/proc/cpuinfo
lscpu
dmidecode -t processor

# Memory
/proc/meminfo
dmidecode -t memory

# GPU
lspci -v | grep -i vga
nvidia-smi (if available)
amdgpu-info (if available)

# TPM
tpm2_getcap properties-fixed (TPM 2.0)
/dev/tpm0 existence (TPM 1.2)

# Virtualization
grep -E 'vmx|svm' /proc/cpuinfo (KVM support)
xen_capabilities (Xen)
dmidecode -s system-product-name (Hyper-V detection)

# Storage
lsblk -o NAME,SIZE,TYPE,FSTYPE,MOUNTPOINT
pvs, lvs, lvdisplay (LVM)
btrfs filesystem show (Btrfs)
zpool list (ZFS)

# Boot
efibootmgr (EFI)
blkid /boot/
/etc/default/grub

# OS Detection
lsb_release -a
/etc/os-release
/etc/issue
```

### 4.3 Configuration Validation (Constraint Solver)

The **ConstraintSolver** (deterministic SAT solver, formally verified with Axiom) takes the hardware manifest + user's omnisystem.nix and determines if the conversion is feasible.

**Example Rules**:

```
Rule 1: hosted-light requires systemd
  IF mode == hosted-light AND not systemd_available() THEN Error

Rule 2: bare-metal requires ≥ 512 MB RAM
  IF mode == bare-metal AND total_memory_mb < 512 THEN Error

Rule 3: hosted-full requires QEMU support
  IF mode == hosted-full AND not (kvm_available OR qemu_available) THEN Warning

Rule 4: GPU passthrough requires IOMMU + IOMMU groups
  IF adapter.gpu_passthrough AND not iommu_enabled THEN Error

Rule 5: If no snapshot support, warn for bare-metal
  IF mode == bare-metal AND snapshot_support == None THEN Warning
    RECOMMENDATION: "No local snapshot available. Full disk image will be stored remotely (slower rollback)."

Rule 6: If OmniCloak requested but no GPU, suggest headless
  IF service.omnicloak AND gpu == None THEN Warning
    RECOMMENDATION: "OmniCloak works in headless mode but lacks GPU acceleration."

Rule 7: library-os requires ≤ 100 MB free space for compilation
  IF mode == library-os AND free_space_mb < 100 THEN Error

Rule 8: Insufficient free space for snapshot
  IF snapshot_strategy == RemoteImage AND free_space_mb < (root_size_mb / 2) THEN Warning
    RECOMMENDATION: "Not enough space to download UMS bootstrap. Will need to stream snapshot to remote."
```

**Solver Output**:

```json
{
  "feasible": true,
  "recommended_mode": "hosted-full",
  "constraints": [
    {
      "level": "Warning",
      "message": "No GPU detected. OmniCloak will run in headless mode.",
      "recommendation": "This is fine for servers, but workstations may want GPU pass-through."
    },
    {
      "level": "Info",
      "message": "IOMMU not enabled. GPU pass-through will not be available.",
      "recommendation": "To enable, add 'intel_iommu=on' to kernel command-line and reboot host."
    }
  ],
  "conversion_plan": {
    "mode": "hosted-full",
    "phases": [
      { "name": "snapshot", "estimated_seconds": 120 },
      { "name": "stage_artifacts", "estimated_seconds": 300 },
      { "name": "deploy", "estimated_seconds": 180 },
      { "name": "finalize", "estimated_seconds": 30 },
      { "name": "verify", "estimated_seconds": 60 }
    ],
    "total_estimated_seconds": 690,
    "rollback_strategy": "RemoteImage"
  }
}
```

---

## 5. SNAPSHOT & ROLLBACK SYSTEM

### 5.1 Snapshot Strategies

OS Flinger supports **hierarchical snapshot creation**, selecting the best available method for the target system.

#### 5.1.1 Local Snapshots (Preferred)

**LVM Thin-Provisioned Snapshots**:
```bash
# Create a thin snapshot of the root volume
# Time: milliseconds
# Space: negligible (copy-on-write)
lvcreate --snapshot --name omnisystem-rollback /dev/mapper/vg0-root

# Restore
lvconvert --merge /dev/mapper/vg0-omnisystem-rollback
reboot
```

**Btrfs Subvolume Snapshot**:
```bash
# Create read-only snapshot
# Time: milliseconds
# Space: negligible (COW)
btrfs subvolume snapshot -r / /.snapshots/omnisystem-rollback-$(date +%s)

# Restore
btrfs subvolume delete /
btrfs subvolume snapshot /.snapshots/omnisystem-rollback-XXXX /
reboot
```

**ZFS Snapshot**:
```bash
# Time: milliseconds
# Space: negligible
zfs snapshot -r rpool/ROOT@omnisystem-rollback

# Restore
zfs rollback rpool/ROOT@omnisystem-rollback
reboot
```

**Advantages**:
- ✅ Instant (milliseconds)
- ✅ Near-zero space overhead (COW)
- ✅ Multiple snapshots can coexist
- ✅ Rollback is atomic

---

#### 5.1.2 Remote Snapshots (Fallback)

If local snapshots are unavailable (ext4, XFS), OS Flinger streams a **compressed image** of the root filesystem to remote storage.

**Process**:
1. User specifies a snapshot target: `--snapshot-target s3://bucket/` or `--snapshot-target omnisystem-node.local`
2. OS Flinger streams the root filesystem to the target:
   ```bash
   # Read root filesystem, compress with zstd
   dd if=/dev/mapper/vg0-root | zstd -19 -T$(nproc) | \
     transfer-daemon push --name root-snapshot-$(date +%s).zst
   ```
3. Image is stored content-addressed in the UMS CAS (BLAKE3 hash)
4. Metadata is logged to audit-log with hash and timestamp
5. On rollback, the image is streamed back and written to disk

**Advantages**:
- ✅ Works on any filesystem
- ✅ Image is deduplicated globally (CAS)
- ✅ Can be verified (hash + signature)

**Disadvantages**:
- ❌ Time: depends on bandwidth (mins–hours)
- ❌ Space: full disk size (compressed)
- ❌ Network dependency

---

#### 5.1.3 No Snapshot (Risky)

If the user runs `os-flinger convert --mode bare-metal --accept-no-snapshot`:

- A **pre-conversion backup** is created (slow, full stream)
- Conversion proceeds with elevated risk
- Rollback requires restore from pre-backup (time-consuming)
- Audit log emits a warning that system was converted without snapshot

---

### 5.2 Rollback Procedure

```
[Failure detected in phase X]
  │
  ├─ Log failure to audit-log with full details
  │
  ├─ If local snapshot exists:
  │   └─ Execute atomic restore (LVM merge, Btrfs rollback, ZFS rollback)
  │   └─ Reboot into restored system
  │   └─ Verify system state matches pre-conversion
  │   └─ Time: seconds
  │
  ├─ Else if remote snapshot exists:
  │   └─ Stream image from UMS CAS back to /dev/mapper/vg0-root
  │   └─ Verify hash matches stored metadata
  │   └─ Reboot
  │   └─ Time: minutes–hours (depends on size, bandwidth)
  │
  └─ Else (no snapshot):
      └─ Halt and prompt user
      └─ User must manually restore from backup or alternate approach
      └─ Log critical error
```

**Verification After Rollback**:

```titan
fn verify_rollback(manifest: &SystemManifest) -> Result<(), Error> {
    // Compare current system against pre-conversion manifest
    let current = run_inventory_agent().await?;
    
    // Check filesystem
    assert_eq!(current.storage.root_fs_type, manifest.storage.root_fs_type)?;
    assert_eq!(current.storage.free_space_mb, manifest.storage.free_space_mb)?;
    
    // Check OS
    assert_eq!(current.os.name, manifest.os.name)?;
    assert_eq!(current.os.version, manifest.os.version)?;
    
    // Check boot
    assert_eq!(current.boot.bootloader, manifest.boot.bootloader)?;
    
    // Spot-check a few files to ensure data integrity
    assert_hash_match("/etc/hostname", manifest.file_hashes["/etc/hostname"])?;
    assert_hash_match("/etc/fstab", manifest.file_hashes["/etc/fstab"])?;
    
    Ok(())
}
```

---

## 6. ARTIFACT STAGING & VERIFICATION

### 6.1 UMS Integration

All artifacts (kernel, initrd, service binaries) are fetched from the **Universal Module System** registry.

**Artifacts Fetched**:
- `uosc-kernel-x86_64-v1.0.0.bin` (UOSC microkernel)
- `omnisystem-initrd-v1.0.0.cpio.gz` (Initial RAM disk)
- `transfer-daemon-v1.0.0-x86_64` (P2P mesh service)
- `ums-v1.0.0-x86_64` (Module system)
- `omnisystem-services-v1.0.0.tar.zst` (All systemd units)
- `omnisystem-root-fs-v1.0.0.tar.zst` (Root filesystem, for bare-metal)

**Verification**:
```
┌──────────────────────────────────────────┐
│  Artifact from UMS Registry              │
│  Content: binary payload                 │
│  Hash (BLAKE3): XYZABC...               │
│  Signature (BLS): signed by Bonsai      │
│  Metadata: size, timestamp, version      │
└──────────────────────────────────────────┘
        │
        ├─ Compute BLAKE3 hash of downloaded artifact
        ├─ Verify hash matches UMS metadata
        │
        ├─ Verify BLS signature using Bonsai Council keys (5-of-7)
        │ (using Aether signature verification service)
        │
        └─ If all checks pass: artifact is trusted
          └─ Stage to `/omnisystem-staging/`
```

### 6.2 Staging Area

Artifacts are staged in a temporary location:

```
/omnisystem-staging/
├── kernel
├── initrd
├── services.tar.zst
├── root-fs.tar.zst (if bare-metal)
├── manifest.json   (UMS metadata)
└── verification.log
```

**Space Requirements**:
- UOSC kernel: ~5 MB
- initrd: ~20 MB
- Services: ~50 MB
- Root filesystem (bare-metal): ~500 MB–1 GB

**If insufficient space**:
- Stream directly to deployment location (slower, riskier)
- Fetch artifacts one-by-one and deploy incrementally

---

## 7. UVM INTEGRATION – POST-CONVERSION VALIDATION

After the DEPLOY phase, the **Universal Validation Mesh** runs a comprehensive test suite to confirm the Omnisystem instance is fully functional.

### 7.1 UVM Test Categories

#### 7.1.1 Kernel Boot Tests
```
✓ System reaches multi-user target
✓ All hardware detected (CPU, memory, storage)
✓ Network interfaces online
✓ Systemd boot time < 5 seconds
```

#### 7.1.2 Capability Enforcement Tests
```
✓ Process without capability cannot access resource
✓ Capability tokens cannot be forged
✓ Linear type system prevents capability duplication
```

#### 7.1.3 Service Availability Tests
```
✓ TransferDaemon listening on :9000
✓ UMS responding to module queries
✓ VFS service operational
✓ Service manager responding to queries
✓ OmniCloak (if enabled) listening on :8080
✓ Transfer-daemon mesh connected to ≥ 1 peer
```

#### 7.1.4 Deterministic Replay Tests (Optional)
```
✓ Boot sequence recorded and replayed
✓ Replay produces bit-identical state
✓ Memory contents at t=30s are identical
```

#### 7.1.5 Performance Baseline Tests
```
✓ Context switch latency < 100µs
✓ IPC latency < 50µs
✓ Memory bandwidth > 1 GB/s (within 10% of pre-conversion)
✓ Disk I/O latency < 5ms
```

#### 7.1.6 User-Provided Tests (Optional)
```
✓ Run user's test script: /omnisystem-tests/smoke-tests.sh
✓ Exit with status 0
✓ All assertions pass
```

### 7.2 UVM Invocation

```bash
# From within the converted system (or in a test VM)
uvm-runner --config /etc/omnisystem-config \
           --test-suite comprehensive \
           --timeout 300 \
           --output json > /tmp/uvm-results.json

# Results
{
  "tests_total": 47,
  "tests_passed": 47,
  "tests_failed": 0,
  "duration_seconds": 157,
  "performance": {
    "boot_time_ms": 2340,
    "first_service_online_ms": 1240,
    "context_switch_latency_us": 45,
    "ipc_latency_us": 32
  }
}
```

### 7.3 Failure Handling

**If any UVM test fails**:
1. Emit detailed error report (which test, why it failed)
2. Immediately trigger automatic rollback
3. Log failure to audit-log with test results
4. Prompt user: "Conversion failed. System has been rolled back. Details in /var/log/omnisystem/conversion.log"

**User can retry with `--force --skip-uvm` if they trust the conversion**, but this is logged as a risk decision.

---

## 8. TRANSACTIONAL DEPLOYMENT ENGINES

### 8.1 Hosted-Light Deployment

```titan
fn deploy_hosted_light(
    config: &OmnisystemConfig,
    artifacts: &StagingArea,
) -> Result<(), Error> {
    // Phase 1: Generate systemd units
    for service in &config.services {
        let unit = match service {
            Service::TransferDaemon => {
                r#"[Unit]
Description=TransferDaemon P2P Mesh
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/local/bin/transfer-daemon --config /etc/omnisystem/transfer-daemon.conf
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target"#
            },
            // ... other services
        };
        
        fs::write(
            format!("/etc/systemd/system/{}.service", service.name()),
            unit,
        )?;
    }
    
    // Phase 2: Copy binaries
    for (name, binary_path) in &artifacts.binaries {
        fs::copy(binary_path, format!("/usr/local/bin/{}", name))?;
        fs::set_permissions(&format!("/usr/local/bin/{}", name), 0o755)?;
    }
    
    // Phase 3: Reload systemd
    Command::new("systemctl")
        .arg("daemon-reload")
        .status()?;
    
    // Phase 4: Enable and start services
    for service in &config.services {
        Command::new("systemctl")
            .args(&["enable", &format!("{}.service", service.name())])
            .status()?;
        
        Command::new("systemctl")
            .args(&["start", &format!("{}.service", service.name())])
            .status()?;
    }
    
    // Phase 5: Verify all services are active
    for service in &config.services {
        let output = Command::new("systemctl")
            .args(&["is-active", &format!("{}.service", service.name())])
            .output()?;
        
        if !output.status.success() {
            return Err(Error::ServiceFailedToStart(service.name().to_string()));
        }
    }
    
    // Phase 6: Write configuration marker
    fs::write("/etc/omnisystem-config", serde_json::to_string(config)?)?;
    
    // Phase 7: Log to audit-log
    audit_log::log_event(AuditEvent::DeploymentPhaseComplete {
        phase: "hosted-light-deploy",
        duration_ms: 0,
        status: "success",
    }).await?;
    
    Ok(())
}
```

---

### 8.2 Hosted-Full Deployment

```titan
fn deploy_hosted_full(
    config: &OmnisystemConfig,
    artifacts: &StagingArea,
    manifest: &SystemManifest,
) -> Result<VmDefinition, Error> {
    // Phase 1: Install QEMU if needed
    ensure_qemu_installed()?;
    
    // Phase 2: Create VM disk image
    let vm_name = format!("omnisystem-{}", manifest.hostname);
    let disk_path = format!("/var/lib/libvirt/images/{}.qcow2", vm_name);
    
    Command::new("qemu-img")
        .args(&["create", "-f", "qcow2", &disk_path, "20G"])
        .status()?;
    
    // Phase 3: Extract root filesystem into VM disk
    let tmp_mount = tempfile::TempDir::new()?;
    mount_loop(&disk_path, tmp_mount.path())?;
    
    extract_tar_zst(&artifacts.root_fs, tmp_mount.path())?;
    
    // Phase 4: Install bootloader in VM
    install_grub_in_vm(tmp_mount.path(), &artifacts.kernel)?;
    
    // Phase 5: Write VM configuration (libvirt XML)
    let vm_xml = generate_libvirt_xml(&vm_name, &disk_path, config)?;
    fs::write(format!("/tmp/{}.xml", vm_name), vm_xml)?;
    
    // Phase 6: Define VM
    Command::new("virsh")
        .args(&["define", &format!("/tmp/{}.xml", vm_name)])
        .status()?;
    
    // Phase 7: Autostart on next reboot
    Command::new("virsh")
        .args(&["autostart", &vm_name])
        .status()?;
    
    // Phase 8: Boot VM
    Command::new("virsh")
        .args(&["start", &vm_name])
        .status()?;
    
    // Phase 9: Wait for VM to become accessible
    for attempt in 0..60 {
        if Command::new("virsh")
            .args(&["domstat", &vm_name])
            .status()
            .is_ok()
        {
            break;
        }
        if attempt > 0 && attempt % 10 == 0 {
            println!("Waiting for VM to boot... ({} seconds)", attempt);
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    
    // Phase 10: Log to audit-log
    audit_log::log_event(AuditEvent::DeploymentPhaseComplete {
        phase: "hosted-full-deploy",
        duration_ms: 0,
        status: "success",
    }).await?;
    
    Ok(VmDefinition { name: vm_name, disk: disk_path })
}
```

---

### 8.3 Bare-Metal Deployment

```titan
fn deploy_bare_metal(
    config: &OmnisystemConfig,
    artifacts: &StagingArea,
    manifest: &SystemManifest,
) -> Result<(), Error> {
    // Phase 1: Verify snapshot exists (safety check)
    if !snapshot_exists() {
        return Err(Error::NoSnapshotBeforeBareMetalConversion);
    }
    
    // Phase 2: Reformat root filesystem
    let root_device = &manifest.storage.root_device;
    Command::new("mkfs.ext4")
        .args(&[root_device, "-F"])
        .status()?;
    
    // Phase 3: Mount and extract root filesystem
    let tmp_mount = tempfile::TempDir::new()?;
    mount_filesystem(root_device, tmp_mount.path())?;
    
    extract_tar_zst(&artifacts.root_fs, tmp_mount.path())?;
    
    // Phase 4: Install bootloader
    install_bootloader(root_device, tmp_mount.path(), &artifacts.kernel)?;
    
    // Phase 5: Update /boot/grub/grub.cfg or /boot/loader/entries/
    update_bootloader_config(&artifacts.kernel, tmp_mount.path())?;
    
    // Phase 6: Unmount
    Command::new("umount")
        .arg(tmp_mount.path())
        .status()?;
    
    // Phase 7: Write marker file for next boot
    // This tells the OS Flinger agent (if running) that the system is being converted
    fs::write("/var/lib/omnisystem/conversion-phase", "finalize")?;
    
    // Phase 8: Log and prepare for reboot
    audit_log::log_event(AuditEvent::DeploymentPhaseComplete {
        phase: "bare-metal-deploy",
        duration_ms: 0,
        status: "success",
        next_action: "reboot",
    }).await?;
    
    // Phase 9: If --reboot-now, reboot. Otherwise, prompt user.
    if config.options.reboot_immediately {
        Command::new("shutdown")
            .args(&["-r", "now", "Omnisystem conversion complete, rebooting..."])
            .status()?;
    } else {
        println!("Conversion complete. To boot into Omnisystem, run: shutdown -r now");
    }
    
    Ok(())
}
```

---

## 9. FLEET ORCHESTRATION – CONVERTING 1,000+ MACHINES IN PARALLEL

OS Flinger supports **fleet-wide conversions** via the `fleet` subcommand. This uses **Aether actors** for distributed orchestration.

### 9.1 Fleet Controller Architecture

```
┌────────────────────────────────────────┐
│   os-flinger fleet convert             │
│   --selector "env=prod,region=us-east" │
│   --config omnisystem.nix              │
│   --concurrency 50                     │
└────────────┬───────────────────────────┘
             │
             ▼
┌────────────────────────────────────────┐
│ Fleet Controller (Aether Actor)        │
│                                        │
│ 1. Query discovery service for nodes  │
│ 2. Group into batches (50 machines)   │
│ 3. Spawn child actors (1 per target)  │
│ 4. Monitor conversion status          │
│ 5. Aggregate results                  │
│ 6. Report failures, trigger rollbacks │
└────────┬───────────────────────────────┘
         │
         ├─ Child Actor (Target 1)
         │  │ invoke: convert(omnisystem.nix, hosted-full, 192.168.1.10)
         │  └─ await: conversion_complete()
         │
         ├─ Child Actor (Target 2)
         │  │ invoke: convert(omnisystem.nix, hosted-full, 192.168.1.11)
         │  └─ await: conversion_complete()
         │
         ├─ Child Actor (Target N)
         │  │ invoke: convert(omnisystem.nix, hosted-full, 192.168.1.N)
         │  └─ await: conversion_complete()
         │
         └─ ...
```

### 9.2 Fleet Command

```bash
# Convert all production nodes in us-east region
os-flinger fleet convert \
  --selector "env=production,region=us-east" \
  --config omnisystem.nix \
  --mode hosted-full \
  --concurrency 50 \
  --batch-size 10 \
  --output json > /tmp/fleet-results.json

# Output
{
  "fleet_id": "fleet-20260605-094532",
  "selector": "env=production,region=us-east",
  "total_nodes": 247,
  "batches": 3,
  "concurrency": 50,
  "results": {
    "success": 245,
    "failed": 2,
    "rolled_back": 2,
    "in_progress": 0
  },
  "failed_nodes": [
    {
      "hostname": "prod-db-14",
      "ip": "192.168.1.114",
      "error": "out_of_memory_during_staging",
      "phase": "stage_artifacts",
      "rolled_back": true
    },
    {
      "hostname": "prod-cache-07",
      "ip": "192.168.1.107",
      "error": "uvm_test_failed",
      "phase": "verify",
      "test_name": "service_availability",
      "rolled_back": true
    }
  ],
  "performance": {
    "total_duration_seconds": 3847,
    "average_per_node_seconds": 156,
    "batch_parallelism_achieved": 0.98,
    "bandwidth_utilization_percent": 87
  }
}
```

### 9.3 Batch Management

**Batching Strategy**:
- User specifies `--batch-size 10` (convert 10 machines, await completion, move to next batch)
- Concurrent conversions within batch: `--concurrency 50` (up to 50 simultaneous)
- If batch fails: automatically roll back entire batch, pause and await user decision

**Dashboard** (live, via `observability` service):

```
Fleet Conversion: fleet-20260605-094532
├─ Batch 1/3: [===================================---] 35/50 complete (70%)
│  ├─ Success: 33
│  ├─ Failed: 2 (rolled back)
│  └─ In Progress: 15
│
├─ Batch 2/3: [==================-------------------] 0/50 (queued)
│
└─ Batch 3/3: [-----] 0/50 (pending)

Errors:
  prod-db-14: out_of_memory_during_staging (rollback successful)
  prod-cache-07: uvm_test_failed (rollback successful)
```

---

## 10. AI ADVISOR INTEGRATION (Optional)

OS Flinger can optionally incorporate an **AI advisor** that runs in "shadow mode" – providing recommendations without affecting core logic.

### 10.1 AI Advisor Modes

**Shadow Mode** (default, enabled):
- Analyzes hardware manifest
- Suggests optimal deployment mode
- Predicts conversion time
- Identifies potential conflicts
- All recommendations are **non-binding**; user makes final decisions

**Predictive Mode** (optional, feature-gated):
- Learns from fleet conversion patterns
- Identifies machines likely to fail and pre-mitigates
- Suggests optimal batch ordering (e.g., convert reliable machines first)

**Generative Mode** (optional):
- Generates starter `omnisystem.nix` based on hardware
- Suggests service configuration

### 10.2 Example: AI Advisor Output

```bash
$ os-flinger analyze --target 192.168.1.100

System: ubuntu-20.04, x86_64, 16 GB RAM, 2x SSD, Intel Xeon, KVM capable

=== AI Advisor (Shadow Mode) ===
Recommended Mode: hosted-full
  Reasoning: Your system has sufficient resources (16 GB), supports KVM,
  and multiple SSDs. hosted-full provides optimal balance of isolation
  and performance.

Predicted Conversion Time: 14 minutes
  - Pre-flight: 2 min
  - Snapshot: 3 min (local ZFS snapshot, instant)
  - Stage artifacts: 4 min (100 Mbps network)
  - Deploy: 3 min
  - Verify: 2 min

Potential Issues:
  ⚠ IOMMU not enabled. GPU passthrough unavailable.
    Fix: Add 'intel_iommu=on' to kernel command line and reboot host.

  ⚠ Insufficient free disk space (15 GB available, ~5 GB needed for staging).
    Recommendation: Clear old logs or extend partition. Or, stream artifacts directly (slower).

Suggested Configuration:
---
services.omnisystem = {
  enable = true;
  mode = "hosted-full";
  adapter.memory_mb = 4096;
  adapter.cpus = 4;
  services.transfer-daemon = true;
  services.ums = true;
  services.vfs = true;
  autonomic.enable = true;
};
---

AI Confidence: 0.92 (high)
```

---

## 11. SECURITY & VERIFICATION

### 11.1 Cryptographic Guarantees

**Binary Signing**:
- All OS Flinger releases are signed with Bonsai Council keys (BLS multi-sig)
- Users verify signature before execution: `curl ... | sha256sum -c` + GPG verify

**Artifact Verification**:
- All artifacts from UMS are content-addressed (BLAKE3)
- Signatures verified with 5-of-7 council threshold
- Failed signatures trigger immediate abort

**Audit Trail**:
- Every action logged to immutable audit-log
- Cryptographically signed with host TPM (if available)
- Accessible via `os-flinger audit-log --target 192.168.1.100`

### 11.2 Formal Verification (Axiom)

**Proofs**:
1. **Data Preservation**: User data (≠ OS) is never modified unless explicitly in config
2. **Atomicity**: DEPLOY phase either fully succeeds or rolls back
3. **Constraint Correctness**: Solver never approves infeasible configurations
4. **Signature Verification**: If BLS sig verifies, artifact is unmodified

**Proof Integration**:
```titan
// Formally verified with Axiom
proof preserve_user_data {
    requires: initial_state.files["/home/*"] == final_state.files["/home/*"];
    // Even if DEPLOY fails, rollback restores /home/*
}

proof atomic_deploy {
    requires: deployed_services.all_active() XOR rolled_back_to_initial_state();
    // No partial deployment state is observable
}
```

### 11.3 Sandboxed Execution (Sanctum)

OS Flinger agent runs in a **Sanctum vault** with minimal capabilities:

```
Capabilities Granted:
  - storage: /dev (block device access)
  - storage: /sys (hardware info)
  - network: outbound TransferDaemon only
  
Capabilities Denied:
  - filesystem: read-only access to /
  - compute: limited CPU/memory
  - device: no USB, no GPU (except in mode-specific phases)
```

---

## 12. CLI DESIGN & COMMAND REFERENCE

### 12.1 Core Commands

```bash
# Analyze target system (non-destructive)
os-flinger analyze --target 192.168.1.100 \
                   --output json

# Dry-run conversion (generates plan, no changes)
os-flinger convert --target 192.168.1.100 \
                   --config omnisystem.nix \
                   --mode hosted-full \
                   --dry-run

# Actual conversion
os-flinger convert --target 192.168.1.100 \
                   --config omnisystem.nix \
                   --mode hosted-full \
                   --skip-snapshot      # Don't create snapshot (risky)
                   --no-verify          # Skip UVM tests
                   --reboot-now         # Reboot immediately after

# Rollback failed conversion
os-flinger rollback --target 192.168.1.100

# Show conversion status
os-flinger status --target 192.168.1.100

# View audit trail
os-flinger audit-log --target 192.168.1.100 \
                     --since "2 hours ago"

# Fleet conversion
os-flinger fleet convert --selector "env=prod" \
                         --config omnisystem.nix \
                         --mode hosted-full \
                         --concurrency 50

# Fleet status
os-flinger fleet status --fleet-id fleet-20260605-094532
```

### 12.2 Global Flags

```
--target HOST[:PORT]          Target machine IP/hostname
--config FILE                 Path to omnisystem.nix or URL
--mode {hosted-light,hosted-full,bare-metal,hybrid,library-os}
--dry-run                     Show plan, don't execute
--force                       Ignore warnings, proceed anyway
--output {text,json,yaml}     Output format
--verbose                     Detailed logging
--debug                       Very verbose, includes internals
--help                        Show help
```

### 12.3 Advanced Flags

```
--snapshot-target URL         Where to store remote snapshot (S3, omnisystem-node)
--skip-snapshot              Don't create snapshot (risky)
--skip-uvm                   Skip post-conversion UVM tests
--accept-no-snapshot         Accept conversion without snapshot
--reboot-now                 Automatically reboot after deployment
--ui {cli,web,tui}          User interface (CLI, web dashboard, TUI)
```

---

## 13. IMPLEMENTATION ROADMAP

**Phase 1: Core Infrastructure** (12 weeks)
- [ ] Rust project scaffold + Titan compilation setup
- [ ] SSH transport + TransferDaemon integration
- [ ] Hardware inventory agent (Aether actor)
- [ ] System manifest data structures
- [ ] Constraint solver (SAT, formally verified)
- [ ] Unit tests + Axiom proofs for solver

**Phase 2: Snapshot & Rollback** (8 weeks)
- [ ] LVM snapshot creation + restore
- [ ] Btrfs snapshot support
- [ ] ZFS snapshot support
- [ ] Remote snapshot streaming (to UMS CAS)
- [ ] Rollback verification
- [ ] Comprehensive rollback tests

**Phase 3: Deployment Engines** (10 weeks)
- [ ] hosted-light: systemd unit generation + installation
- [ ] hosted-full: QEMU domain creation + virtio config
- [ ] bare-metal: bootloader update + root filesystem install
- [ ] hybrid: Sanctum vault creation
- [ ] library-os: binary compilation + staging
- [ ] Integration tests for each mode

**Phase 4: UVM Integration** (8 weeks)
- [ ] UVM test harness integration
- [ ] Kernel boot tests
- [ ] Capability enforcement tests
- [ ] Service availability tests
- [ ] Performance baseline tests
- [ ] Deterministic replay tests (optional)

**Phase 5: Fleet Orchestration** (10 weeks)
- [ ] Fleet controller (Aether actor)
- [ ] Batch management
- [ ] Failure handling + rollback
- [ ] Live dashboard (observability)
- [ ] Parallel conversion tests

**Phase 6: CLI & UX** (6 weeks)
- [ ] Command parser + dispatch
- [ ] JSON/YAML/TUI output
- [ ] Help system
- [ ] Interactive prompts
- [ ] Progress reporting

**Phase 7: AI Advisor & Polish** (8 weeks)
- [ ] AI advisor (shadow mode)
- [ ] Configuration generation
- [ ] Formal verification of core proofs
- [ ] Security audit
- [ ] Documentation
- [ ] Release preparation

**Total Estimated Duration**: 62 weeks (~14 months) for MVP → Production

---

## 14. INTEGRATION WITH OMNISYSTEM ECOSYSTEM

### 14.1 TransferDaemon Integration

- OS Flinger agent communicates with controller via TransferDaemon (not raw SSH)
- All artifact downloads flow through TransferDaemon mesh
- Automatic P2P deduplication (if multiple machines downloading same artifact)
- Multi-path reliability (failover if node unavailable)

### 14.2 UMS Integration

- All artifacts (kernel, services, root filesystem) hosted in UMS registry
- Content-addressed (BLAKE3)
- Signed by Bonsai Council (BLS 5-of-7)
- Accessible globally via mesh

### 14.3 UVM Integration

- Post-conversion validation via UVM test suite
- Tests run in converted system or isolated test VM
- Pass/fail gates automatic rollback

### 14.4 Sanctum Integration

- OS Flinger agent runs in Sanctum vault with minimal capabilities
- Secure isolation from host system

### 14.5 Aether Integration

- Fleet controller is Aether actor
- Each target conversion is child actor
- Supervision tree handles actor failure
- Distributed state management

### 14.6 Axiom Integration

- Core algorithms formally verified
- Constraint solver proven correct
- Safety proofs embedded in CI

### 14.7 Audit-Log Integration

- All conversion events logged immutably
- Cryptographically signed
- Queryable via `os-flinger audit-log`

---

## 15. FAILURE MODES & RECOVERY

### 15.1 Common Failure Scenarios

| Scenario | Detection | Recovery |
|----------|-----------|----------|
| Network timeout during artifact download | Download fails | Retry via TransferDaemon, if persistent, rollback |
| Signature verification fails | BLS check fails | Abort immediately (security), log to audit-log |
| UVM test timeout | Test doesn't complete in 5 min | Auto-rollback, log timeout |
| Out of memory during DEPLOY | malloc fails | Rollback to snapshot, suggest --reboot-before-deploy |
| Disk full during root filesystem extraction | Write fails | Cleanup staging area, rollback, extend partition |
| systemd unit syntax error (hosted-light) | systemctl daemon-reload fails | Rollback unit files, check config |
| QEMU VM won't boot (hosted-full) | VM hangs at boot | Rollback QEMU domain, restore host |

### 15.2 Recovery Procedures

**Automatic Rollback** (default):
```
Failure Detected
  ↓
Log to audit-log
  ↓
Trigger rollback (LVM merge, Btrfs rollback, etc.)
  ↓
System reboots into pre-conversion state
  ↓
User can investigate logs and retry
```

**Manual Rollback** (if automatic fails):
```
User runs: os-flinger rollback --target 192.168.1.100
  ↓
Restore from snapshot/image
  ↓
Reboot
```

---

## 16. TESTING STRATEGY

### 16.1 Unit Tests

- Constraint solver correctness (Axiom proofs)
- System manifest parsing
- Artifact signature verification
- Rollback logic (simulate failures, verify recovery)

### 16.2 Integration Tests

- hosted-light: Deploy systemd units, verify services start
- hosted-full: Boot QEMU, run UVM tests
- bare-metal: Full conversion in VM, snapshot + rollback
- hybrid: Convert to hybrid, verify vault operational
- library-os: Compile unikernel, boot

### 16.3 Fleet Tests

- Deploy 100 machines in parallel
- Simulate failures (network timeout, signature mismatch, etc.)
- Verify batch rollback
- Measure performance (throughput, latency)

### 16.4 Chaos Tests

- Unplug network during conversion → rollback
- Fill disk → rollback
- Corrupt snapshot → fallback to manual recovery
- Reboot during phases → idempotent recovery

---

## 17. DOCUMENTATION & USER GUIDES

### 17.1 User Documentation
- **Quick Start Guide** – "Convert your first machine in 10 minutes"
- **Mode Selection Guide** – "Which mode is right for you?"
- **Troubleshooting** – Common errors and solutions
- **API Reference** – All CLI commands + flags

### 17.2 Operator Documentation
- **Fleet Operations** – Managing 1,000+ machine conversions
- **Monitoring & Alerting** – Integration with observability
- **Backup & Disaster Recovery** – Snapshot management, restore procedures

### 17.3 Developer Documentation
- **Architecture Guide** – Deep dive into design
- **Plugin System** – Custom deployment modes
- **Formal Verification** – How proofs are checked

---

## 18. RELEASE & DEPLOYMENT

### 18.1 Release Process

1. **Build**: `cargo build --release --target x86_64-unknown-linux-musl` (static binary)
2. **Sign**: Sign binary with Bonsai Council keys (BLS)
3. **Test**: Run full integration test suite
4. **Package**: Create release tarball with signatures
5. **Publish**: Upload to github.com/LoopyLuci/OS-Flinger/releases
6. **Announce**: Blog post, email to users

### 18.2 Self-Update

OS Flinger agent can update itself:
```bash
os-flinger self-update --check
# If newer version available:
os-flinger self-update --now
# Agent downloads new binary, verifies signature, atomically replaces itself
```

### 18.3 Deployment Methods

```bash
# 1. Direct download + execute
curl -Ls https://get.omnisystem.org/os-flinger | bash

# 2. Nix package
nix shell github:LoopyLuci/Omnisystem#os-flinger

# 3. Package manager (future)
sudo apt install os-flinger
sudo yum install os-flinger

# 4. Containerized
docker run ghcr.io/LoopyLuci/os-flinger convert --target 192.168.1.100
```

---

## 19. CONCLUSION

**OS Flinger** is a **next-generation, production-grade system migration fabric** that makes converting any Linux system to Omnisystem safe, auditable, and reversible. By leveraging:

- **Statically-linked Titan binary** for portability
- **Transactional state machine** for atomicity
- **Hardware-aware constraint solving** for correctness
- **Automatic snapshot & rollback** for safety
- **Distributed fleet orchestration** for scale
- **Post-conversion UVM validation** for verification
- **Formal proofs (Axiom)** for guarantees
- **Immutable audit trail** for accountability

OS Flinger becomes the ultimate on-ramp to the sovereign Omnisystem, capable of converting a developer's laptop or an entire global data center with equal ease and safety. 🏰

---

**Status**: Ready for Implementation  
**Next Step**: Begin Phase 1 (Core Infrastructure) with full team alignment

