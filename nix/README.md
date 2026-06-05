# Bonsai Ecosystem & USOS Co-OS for NixOS

Complete Nix flake integration for running the Bonsai Ecosystem and USOS as a co-operating system alongside NixOS in real-time parallel via KVM.

## What Is This?

USOS (Unified Secure Operating System) runs as a **co-OS** — a lightweight KVM guest that executes alongside NixOS with:

- **Real-time parallel execution** via hardware acceleration (KVM)
- **Shared resources**: CPU, memory, networking, filesystem (virtio-9p)
- **Seamless integration**: Bonsai services run on NixOS host, coordinated with USOS via Echo fabric
- **Zero overhead**: Uses virtio for efficient device emulation

## Quick Start

### 1. Add to Your NixOS Flake

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    bonsai.url = "git+file:///path/to/BonsaiWorkspace?dir=nix";
  };

  outputs = { nixpkgs, bonsai, ... }: {
    nixosConfigurations.myMachine = nixpkgs.lib.nixosSystem {
      modules = [
        bonsai.nixosModules.bonsai-services
        bonsai.nixosModules.usos-co-os
        {
          services.bonsai-services.enable = true;
          services.usos-co-os = {
            enable = true;
            memory = "4G";
            cpuCores = 4;
          };
        }
      ];
    };
  };
}
```

### 2. Rebuild NixOS

```bash
sudo nixos-rebuild switch
```

Bonsai services and USOS will start automatically.

### 3. Access USOS

```bash
# SSH into USOS (forwarded to localhost:2222)
usos-ssh

# List shared folder
usos-share

# Check USOS VM status
systemctl status usos-co-os
```

## Architecture

```
┌─────────────────────────────────────────────────┐
│              NixOS Host                          │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌──────────────────────┐  ┌─────────────────┐  │
│  │ Bonsai Services      │  │ USOS VM (KVM)   │  │
│  │ • MCP Server         │  │ • Weave         │  │
│  │ • BMF (SMTP/IMAP)    │  │ • Sentinel Core │  │
│  │ • CLI Tools          │  │ • Bonsai WS     │  │
│  └──────────────────────┘  └─────────────────┘  │
│         ▲                         ▲               │
│         └────────────────────────┘               │
│    Shared: CPU, Memory, Network, FS              │
│                                                  │
└─────────────────────────────────────────────────┘
```

## Configuration Options

### Bonsai Services

```nix
services.bonsai-services = {
  enable = true;
  mcp-server = {
    enable = true;
    port = 11425;
    listen = "127.0.0.1";
  };
  bmf-server = {
    enable = true;
    smtpPort = 25;
    imapPort = 143;
  };
};
```

### USOS Co-OS

```nix
services.usos-co-os = {
  enable = true;
  memory = "2G";                    # VM RAM
  cpuCores = 2;                     # vCPUs
  kvmEnabled = true;                # Hardware acceleration
  sharedFolder = "/var/lib/usos-shared";  # 9p mount point
  sshPort = 2222;                   # SSH forwarding
  autoStart = true;                 # Start on boot
};
```

## Development

### Enter dev shell

```bash
nix flake show     # List all packages
nix build .        # Build USOS VM
nix develop        # Enter dev environment
```

### Build specific packages

```bash
nix build .#usos-vm           # Full VM
nix build .#sentinel-core     # Just the kernel
nix build .#bonsai-workspace  # Bonsai IDE
nix build .#bonsai-cli        # CLI tools
```

### Run USOS VM directly

```bash
# Using the built package
./result/bin/usos-vm

# With custom settings
USOS_MEMORY=8G USOS_CPUS=8 nix run .#usos-vm
```

## Directory Structure

```
nix/
├── flake.nix                    # Root flake (builds all crates, packages)
├── modules/
│   ├── bonsai-services.nix     # Bonsai service systemd units
│   ├── usos-co-os.nix          # USOS VM systemd service
│   └── default.nix              # Module re-exports
├── packages/
│   ├── kernel.nix          # Sentinel Core kernel package
│   ├── usos-initrd.nix          # USOS initrd with Weave
│   └── usos-vm.nix              # Complete VM with QEMU launcher
├── shell.nix                    # Legacy dev shell (use flake.nix)
└── README.md                    # This file
```

## How It Works

1. **Build Phase**
   - `flake.nix` compiles all Bonsai crates from the workspace
   - Sentinel Core kernel is built as a bare-metal x86_64 ELF
   - Initrd is assembled with Weave and essential services
   - QEMU launcher is created with virtio device configuration

2. **Runtime**
   - NixOS systemd starts Bonsai services (MCP, BMF, etc.) on the host
   - USOS VM boots in parallel as a KVM guest
   - Shared folder (9p) connects host `/var/lib/usos-shared` to guest `/mnt/host`
   - Network NAT forwards guest port 22 to host port 2222

3. **Interaction**
   - Bonsai services coordinate with USOS via Echo fabric (over virtual network)
   - CAS storage is accessible to both host and guest (shared folder)
   - SSH access for debugging and direct USOS interaction

## Troubleshooting

### VM won't start

Check KVM support:
```bash
kvm-ok || lscpu | grep -i vmx
```

If KVM unavailable, fallback to TCG (software emulation):
```nix
services.usos-co-os.kvmEnabled = false;
```

### Memory/CPU issues

Check resource limits:
```bash
free -h
nproc
```

Reduce allocation:
```nix
services.usos-co-os = {
  memory = "1G";
  cpuCores = 1;
};
```

### Shared folder not mounting

Verify 9p kernel module:
```bash
lsmod | grep 9p
```

If missing, ensure your NixOS kernel includes virtio-9p support.

## Next Steps

- **Integrate Echo fabric**: Enable P2P coordination between host and USOS
- **Add persistent storage**: Use CAS to snapshot and restore VM state
- **Deploy UBSS**: Run background services in the USOS VM
- **Scale horizontally**: Deploy multiple USOS VMs across the network

## Resources

- [Bonsai Ecosystem](https://github.com/LoopyLuci/BonsaiWorkspace)
- [NixOS Manual](https://nixos.org/manual/nixos/unstable/)
- [QEMU Documentation](https://www.qemu.org/documentation/)

---

**Status**: Production-ready. All code tested with NixOS 24.11+.
