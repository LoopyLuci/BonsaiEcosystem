# UOSC Driver Framework

Complete specification of the universal, modular driver interface for hardware and virtual devices.

## Overview

The UOSC driver framework provides:

- **Unified Interface**: Single API for all device classes (block, character, network, etc.)
- **Dynamic Loading**: Hot-load and unload drivers without kernel restart
- **Fault Isolation**: Misbehaving drivers cannot crash kernel or other drivers
- **Device Discovery**: Automatic enumeration and configuration
- **Resource Management**: Kernel tracks driver resources and enforces limits
- **Security**: Per-driver capability enforcement

## Design Principles

### 1. Hardware Abstraction
Drivers abstract specific hardware details behind a common interface. User-mode libraries build upon this abstraction.

### 2. Modularity
Drivers are independently compilable, loadable, and removable. Zero interdependencies between drivers.

### 3. Isolation
Each driver runs in its own protection domain. Driver crashes cannot affect kernel or other drivers.

### 4. Transparency
All driver operations are logged and observable. Users can inspect driver activity, resource usage, and state.

## Device Model

### Device Hierarchy

Devices form a hierarchical tree:

```
System Bus (Root)
├── CPU (Virtual Device)
├── Memory Controller
│   ├── RAM Device
│   └── Cache Device
├── I/O Bus (PCI, USB, etc.)
│   ├── Network Device (eth0)
│   ├── Storage Device (sda)
│   │   ├── Partition (sda1)
│   │   └── Partition (sda2)
│   └── Graphics Device (gpu0)
└── Interrupt Controller
```

### Device Class Hierarchy

```
Device (Base Interface)
├── BlockDevice (Storage)
│   ├── DiskDevice
│   ├── PartitionDevice
│   └── RaidDevice
├── CharacterDevice (Sequential I/O)
│   ├── SerialPort
│   ├── ParallelPort
│   └── ConsoleDevice
├── NetworkDevice
│   ├── Ethernet
│   ├── WiFi
│   └── VirtualNIC
└── SpecialDevice
    ├── MemoryDevice
    ├── RandomDevice
    └── NullDevice
```

## Device Interface

All devices implement this common interface:

```
interface Device {
  // Identification
  get_name() -> String
  get_class() -> DeviceClass
  get_vendor_id() -> u32
  get_device_id() -> u32
  
  // Configuration
  initialize() -> Status
  shutdown() -> Status
  reset() -> Status
  set_config(key: String, value: Any) -> Status
  
  // State Query
  get_state() -> DeviceState
  get_stats() -> DeviceStats
  get_capabilities() -> Capabilities
  
  // I/O (class-specific)
  // BlockDevice:
  read(block: u64, count: u32) -> Result<Buffer>
  write(block: u64, data: Buffer) -> Status
  
  // CharacterDevice:
  read_bytes(count: u32) -> Result<Buffer>
  write_bytes(data: Buffer) -> Status
  
  // NetworkDevice:
  transmit(packet: NetworkPacket) -> Status
  receive() -> Result<NetworkPacket>
  
  // Control
  ioctl(command: u32, args: Any) -> Result<Any>
  poll(events: EventMask) -> EventMask
}
```

## Driver Lifecycle

### 1. Registration

A driver registers itself with the kernel:

```
driver_register({
  name: "nvme_driver",
  version: "1.0.0",
  compatible_devices: [
    { vendor: 0x8086, device: 0x0953 },
    { vendor: 0x144d, device: 0xa802 }
  ],
  ops: DeviceOps { ... }
})
```

### 2. Discovery

The kernel discovers hardware devices and invokes appropriate drivers:

```
For each physical device:
  1. Enumerate vendor/device ID
  2. Find matching registered driver
  3. Call driver.probe(device_info)
  4. If probe succeeds, driver owns the device
```

### 3. Initialization

Once a driver claims a device, the kernel calls:

```
driver.initialize(device_config) -> Status
  • Driver allocates resources
  • Driver initializes hardware
  • Driver registers device operations
  • Returns success/failure
```

### 4. Runtime

Driver services I/O requests and responds to events:

```
User program issues I/O request
  ↓
Kernel validates request
  ↓
Kernel forwards to driver
  ↓
Driver executes operation
  ↓
Driver returns result
  ↓
Kernel delivers result to user program
```

### 5. Removal

When a driver is removed (unload or shutdown):

```
kernel.driver_unload(driver_name) -> Status
  1. Kernel notifies driver: shutdown pending
  2. Driver completes pending operations
  3. Driver releases all resources
  4. Driver is removed from kernel
  5. Device becomes unavailable
```

## Device State Machine

```
┌─────────────┐
│  DISCOVERY  │
│  (Hardware  │
│   detected) │
└─────┬───────┘
      │
      ↓
┌─────────────┐
│ REGISTERED  │
│ (Driver     │
│  claims it) │
└─────┬───────┘
      │
      ↓
┌─────────────┐         ┌─────────────┐
│INITIALIZED │◄────────│   SUSPENDED │
│ (Driver    │         │ (S-state    │
│  setup OK) │────────►│  power save) │
└─────┬───────┘         └─────────────┘
      │
      ├─────────────────────────────┐
      │                             │
      ↓                             ↓
┌──────────────┐          ┌──────────────┐
│  RUNNING     │          │   SUSPENDED  │
│ (Device      │          │  (Idle/power)│
│  operational)│          └──────────────┘
└──────┬───────┘
       │
       ↓
┌─────────────┐
│  REMOVING   │
│ (Shutdown   │
│  in progress)│
└─────┬───────┘
      │
      ↓
┌─────────────┐
│  REMOVED    │
│ (Device     │
│  unavailable)│
└─────────────┘
```

## Interrupt Handling

Drivers respond to hardware interrupts through registered handlers:

```
device.register_interrupt_handler(
  irq: u32,
  handler: fn(context: Context) -> InterruptStatus
)

Interrupt Flow:
  1. Hardware raises interrupt
  2. Kernel saves context
  3. Kernel calls registered handler
  4. Handler processes interrupt
  5. Handler returns status
  6. Kernel restores context
  7. Execution resumes
```

## Resource Management

The kernel tracks and limits driver resources:

```
DriverResources {
  memory_limit: Bytes,
  max_interrupts: Count,
  max_dma_transfers: Count,
  max_open_files: Count,
  max_io_operations: Count
}

Kernel Enforcement:
  • Deny allocation if limit exceeded
  • Terminate driver if violations detected
  • Log all resource violations
```

## Error Handling

Drivers must handle errors gracefully:

```
interface DriverErrorHandler {
  // Called when driver operation fails
  on_error(operation: Operation, error: ErrorCode) -> RecoveryAction
  
  // Called when device is in error state
  on_device_error(device: Device, severity: Severity) -> RecoveryAction
  
  // Called when driver detects inconsistency
  on_invariant_violation(invariant: String) -> RecoveryAction
}

RecoveryAction = RETRY | FAIL | ESCALATE | SHUTDOWN
```

## Hot-Swapping

Drivers support dynamic addition and removal:

```
API:
  kernel.driver_load(driver_path: String) -> Status
  kernel.driver_unload(driver_name: String) -> Status
  kernel.driver_reload(driver_name: String) -> Status

Sequence:
  1. Load driver binary
  2. Verify digital signature
  3. Register with kernel
  4. Kernel discovers devices
  5. Driver probes and initializes
```

## Built-In Drivers

UOSC includes minimal built-in drivers:

1. **Console Driver** - Serial console output
2. **RTC Driver** - Real-time clock
3. **Timer Driver** - System timer and scheduling
4. **Memory Driver** - Physical memory access
5. **Interrupt Controller** - Interrupt routing

All other drivers are optional and loaded dynamically.

## Writing a Driver

See [DRIVER_DEVELOPMENT.md](DRIVER_DEVELOPMENT.md) for complete guide to writing UOSC drivers.

## API Reference

See [API Reference](../api/drivers.md) for complete driver API specification.

## Performance

| Operation | Latency | Notes |
|-----------|---------|-------|
| Device Open | <10µs | Lookup + verify permissions |
| Device I/O | Hardware-dependent | Minimal kernel overhead |
| Interrupt | <1µs | Handler latency |
| Driver Load | <100ms | Including verification |

## Security Considerations

- **Capability Checking**: Only authorized processes can access devices
- **Resource Limits**: Drivers cannot monopolize resources
- **Isolation**: Driver crashes don't affect kernel
- **Audit Trail**: All driver operations logged
- **Signed Drivers**: Optional digital signature verification

---

**UOSC Drivers: Unified, Modular, Isolated, Observable.**
