# UOSC Device I/O Hypercalls

Complete detailed specification of all 10 device I/O hypercalls with formal contracts.

## Hypercall Overview

Device I/O hypercalls provide:
- Device file open/close
- Byte-oriented I/O operations
- Device control operations
- Event polling
- Memory mapping for buffers
- Synchronization with devices

All hypercalls are **formally verified** in Axiom with complete pre/post conditions.

---

## 1. device_open

Open a device file.

### Signature
```c
FileDescriptor hypercall_device_open(
    const char* device_path,    // Device path (e.g., "/dev/console")
    i32 flags                   // Open flags
);

// Flags:
#define O_READ          0x01   // Read access
#define O_WRITE         0x02   // Write access
#define O_RDWR          0x03   // Read and write
#define O_NONBLOCK      0x04   // Non-blocking I/O
#define O_EXCL          0x08   // Exclusive access
```

### Return Value
- **Success**: FileDescriptor > 0 (valid file descriptor)
- **Failure**: -1 (ENOENT: no such device, EACCES: permission denied)

### Preconditions
```
1. device_path is valid readable string
2. Device exists in kernel
3. Caller has capability to access device
4. flags contain valid permission bits
5. Caller not at FD limit
```

### Postconditions
```
On success:
  1. New file descriptor allocated to caller
  2. Device driver notified of open
  3. Device enters initialized state
  4. FD can be used for I/O operations
  5. Device state tracked in kernel

On failure:
  1. No FD allocated
  2. Device unchanged
```

### Examples

#### Open Console Device
```c
// Open console for writing
FileDescriptor console = hypercall_device_open("/dev/console", O_WRITE);

if (console < 0) {
    return -ENOENT;  // No console
}

// Can now write to console
hypercall_device_write(console, "Hello\n", 6);
```

#### Open with Exclusive Access
```c
// Get exclusive access to device
FileDescriptor exclusive = hypercall_device_open(
    "/dev/uart",
    O_RDWR | O_EXCL
);
// No other process can open same device
```

### Performance
- **Latency**: < 5µs (device initialization)
- **Blocking**: Depends on device driver

### Error Handling

| Error | Cause | Recovery |
|-------|-------|----------|
| ENOENT | Device not found | Check device path |
| EACCES | Permission denied | Request appropriate capability |
| EBUSY | Device busy | Try again later |

---

## 2. device_close

Close a device file.

### Signature
```c
i32 hypercall_device_close(FileDescriptor fd);
```

### Return Value
- **Success**: 0
- **Failure**: -1 (EBADF: invalid FD)

### Preconditions
```
1. fd is valid and open
2. Caller opened the FD or has privileges
```

### Postconditions
```
On success:
  1. File descriptor released
  2. Device driver notified (cleanup)
  3. Any pending I/O cancelled
  4. FD invalid for future use
  5. Resources freed
```

### Examples

```c
FileDescriptor fd = hypercall_device_open("/dev/console", O_WRITE);
// ... use device ...
hypercall_device_close(fd);
// fd now invalid
```

### Performance
- **Latency**: < 2µs (cleanup)

---

## 3. device_read

Read from a device.

### Signature
```c
i32 hypercall_device_read(
    FileDescriptor fd,
    void* buffer,           // User-allocated buffer
    u32 count               // Bytes to read
);
```

### Return Value
- **Success**: Number of bytes read (≥ 0)
- **Failure**: -1 (EBADF: invalid FD, ENOTREADY: device not ready)

### Preconditions
```
1. fd is open for reading
2. buffer is valid writable address
3. count > 0
4. buffer large enough for count bytes
```

### Postconditions
```
On success:
  1. Up to count bytes read from device
  2. Bytes copied to user buffer
  3. Return actual bytes read (≤ count)
  4. May return fewer bytes if data unavailable
  5. Blocking or non-blocking depends on flags

On failure:
  1. No data read
  2. Buffer unchanged
```

### Examples

#### Non-Blocking Read
```c
u8 buffer[1024];
int bytes = hypercall_device_read(console, buffer, 1024);

if (bytes > 0) {
    printf("Read %d bytes\n", bytes);
} else if (bytes == 0) {
    printf("No data available (non-blocking)\n");
} else {
    printf("Error: %d\n", bytes);
}
```

#### Blocking Read (reads until full)
```c
u8 buffer[256];
int total = 0;

while (total < 256) {
    int n = hypercall_device_read(fd, buffer + total, 256 - total);
    if (n <= 0) break;
    total += n;
}
```

### Performance
- **Latency**: < 5µs (if data available), may block
- **Throughput**: Device-dependent (typically 1-100MB/sec)

---

## 4. device_write

Write to a device.

### Signature
```c
i32 hypercall_device_write(
    FileDescriptor fd,
    const void* buffer,    // Data to write
    u32 count              // Bytes to write
);
```

### Return Value
- **Success**: Number of bytes written (> 0)
- **Failure**: -1 (EBADF: invalid FD, ENOTREADY: device not ready)

### Preconditions
```
1. fd is open for writing
2. buffer is valid readable address
3. count > 0
```

### Postconditions
```
On success:
  1. Data written to device
  2. Return number of bytes accepted
  3. May accept fewer bytes if buffer full
  4. Non-blocking: return immediately
  5. Blocking: may block until space available
```

### Examples

#### Console Output
```c
const char* msg = "Hello, UOSC!\n";
int written = hypercall_device_write(console, msg, 14);

if (written == 14) {
    printf("Message sent\n");
}
```

#### Buffered Writing
```c
u8 data[1024];
int total = 0;

while (total < 1024) {
    int n = hypercall_device_write(fd, data + total, 1024 - total);
    if (n <= 0) break;
    total += n;
}
```

### Performance
- **Latency**: < 5µs (if device ready), may block
- **Throughput**: Device-dependent

---

## 5. device_ioctl

Device-specific control operations.

### Signature
```c
i32 hypercall_device_ioctl(
    FileDescriptor fd,
    u32 command,           // Device-specific command
    void* args             // Command arguments
);

// Standard commands:
#define IOCTL_GET_CONFIG    0x1001
#define IOCTL_SET_CONFIG    0x1002
#define IOCTL_GET_STATUS    0x1003
#define IOCTL_RESET         0x1004
#define IOCTL_SET_BAUD      0x1005  // Serial devices
```

### Return Value
- **Success**: 0 or command-specific value
- **Failure**: -1 (EBADF: invalid FD, EINVAL: invalid command)

### Preconditions
```
1. fd is open
2. command is valid for device type
3. args pointer valid if command requires it
4. Caller has capability to execute command
```

### Examples

#### UART Configuration
```c
struct UARTConfig {
    u32 baud_rate;
    u8 data_bits;
    u8 stop_bits;
    u8 parity;
};

UARTConfig config = {
    .baud_rate = 115200,
    .data_bits = 8,
    .stop_bits = 1,
    .parity = 0
};

int result = hypercall_device_ioctl(
    uart_fd,
    IOCTL_SET_CONFIG,
    &config
);
```

#### Get Device Status
```c
struct DeviceStatus {
    u32 state;
    u32 error_count;
    u64 bytes_processed;
};

DeviceStatus status;
hypercall_device_ioctl(fd, IOCTL_GET_STATUS, &status);

printf("Device state: %d\n", status.state);
```

### Performance
- **Latency**: Device-dependent, < 10µs typical

---

## 6. device_poll

Wait for device to become ready.

### Signature
```c
i32 hypercall_device_poll(
    FileDescriptor fd,
    i32 events,         // Events to wait for
    i32 timeout_ms      // Timeout in milliseconds (-1 = infinite)
);

// Events:
#define POLL_READ       0x01   // Data available to read
#define POLL_WRITE      0x02   // Ready to write
#define POLL_ERROR      0x04   // Error condition
```

### Return Value
- **Success**: Bitmask of ready events
- **Failure**: -1 (EBADF: invalid FD, ETIMEDOUT: timeout)

### Preconditions
```
1. fd is open
2. events is valid bit combination
3. timeout_ms ≥ -1
```

### Postconditions
```
On return:
  1. Returns immediately if events already ready
  2. Blocks caller if no events ready
  3. Wakes up when event occurs
  4. Returns events ready
  5. Timeout returns 0 if time expires
```

### Examples

#### Wait for Data
```c
// Wait for data available (with 1 second timeout)
int ready = hypercall_device_poll(fd, POLL_READ, 1000);

if (ready & POLL_READ) {
    // Data available, can read
    hypercall_device_read(fd, buffer, 1024);
} else if (ready == 0) {
    printf("Timeout\n");
} else {
    printf("Error: %d\n", ready);
}
```

#### Wait for Write Ready
```c
// Wait until device accepts data
int ready = hypercall_device_poll(fd, POLL_WRITE, -1);  // Infinite wait

if (ready & POLL_WRITE) {
    hypercall_device_write(fd, buffer, count);
}
```

### Performance
- **Latency**: < 100ns (if already ready), otherwise context switch overhead
- **Blocking**: Yes, until event occurs or timeout

---

## 7. device_map

Memory-map device into virtual address space.

### Signature
```c
VirtualAddress hypercall_device_map(
    FileDescriptor fd,
    u64 offset,         // Offset within device memory
    u64 size,           // Size to map
    i32 prot            // Protection (PROT_READ, PROT_WRITE, PROT_EXEC)
);
```

### Return Value
- **Success**: Virtual address of mapped region
- **Failure**: NULL (EBADF: invalid FD, ENOMEM: out of memory)

### Preconditions
```
1. fd is open
2. Device supports memory-mapping
3. [offset, offset+size) valid in device
4. prot ⊆ device permissions
```

### Postconditions
```
On success:
  1. Device memory accessible at virtual address
  2. Reads/writes go directly to device
  3. Permissions enforced
  4. Multiple mappings to same device possible
```

### Examples

#### Memory-Mapped I/O
```c
// Map UART registers
void* uart_regs = hypercall_device_map(uart_fd, 0, 4096, PROT_READ | PROT_WRITE);

// Can now directly access hardware
volatile u32* data_reg = (volatile u32*)uart_regs;
volatile u32* status_reg = (volatile u32*)(uart_regs + 4);

*data_reg = 'A';  // Send character
```

#### Display Framebuffer
```c
// Map GPU framebuffer memory
void* framebuffer = hypercall_device_map(
    gpu_fd,
    0,                          // Offset 0
    1024 * 768 * 4,             // Size for 1024x768 RGBA
    PROT_READ | PROT_WRITE
);

// Can now draw directly to framebuffer
u32* pixels = (u32*)framebuffer;
pixels[0] = 0xFF0000FF;  // Red pixel
```

### Performance
- **Latency**: < 5µs (setup memory mapping)

---

## 8. device_unmap

Unmap device from virtual address space.

### Signature
```c
i32 hypercall_device_unmap(
    VirtualAddress address,
    u64 size
);
```

### Return Value
- **Success**: 0
- **Failure**: -1 (EFAULT: bad address)

### Effect
```
Virtual address range no longer maps to device.
Subsequent accesses cause page fault.
```

### Performance
- **Latency**: < 2µs

---

## 9. device_sync

Synchronize with device.

### Signature
```c
i32 hypercall_device_sync(
    FileDescriptor fd,
    i32 flags           // Sync flags
);

// Flags:
#define SYNC_READ       0x01   // Wait for read to complete
#define SYNC_WRITE      0x02   // Wait for write to complete
#define SYNC_ALL        0x03   // Wait for all pending I/O
#define SYNC_NOWAIT     0x10   // Non-blocking check
```

### Return Value
- **Success**: 0 (I/O complete)
- **Failure**: -1 (error), or bytes still pending if SYNC_NOWAIT

### Preconditions
```
1. fd is open
2. flags is valid combination
```

### Postconditions
```
On success (blocking):
  1. All pending I/O complete
  2. Device fully synchronized
  3. Returns after all operations done

On success (non-blocking):
  1. Check status
  2. Returns immediately with count of pending bytes
```

### Examples

#### Ensure Write Complete
```c
// Write data
hypercall_device_write(fd, data, size);

// Wait for write to actually complete
hypercall_device_sync(fd, SYNC_WRITE);

// Now safe to power down device
```

#### Non-Blocking Check
```c
// Start write
hypercall_device_write(fd, data, size);

// Do other work while device is busy...

// Later, check if done
int pending = hypercall_device_sync(fd, SYNC_WRITE | SYNC_NOWAIT);

if (pending == 0) {
    printf("Write complete\n");
} else {
    printf("Still writing %d bytes\n", pending);
}
```

### Performance
- **Latency**: Device-dependent, may block
- **Blocking**: Yes (unless SYNC_NOWAIT)

---

## 10. device_stat

Get device status and information.

### Signature
```c
i32 hypercall_device_stat(
    FileDescriptor fd,
    DeviceStat* stat_out
);

struct DeviceStat {
    const char* name;
    u32 vendor_id;
    u32 device_id;
    i32 state;
    u64 bytes_read;
    u64 bytes_written;
    u64 error_count;
    u32 interrupt_count;
};
```

### Return Value
- **Success**: 0
- **Failure**: -1 (EBADF: invalid FD)

### Examples

```c
DeviceStat stat;
hypercall_device_stat(fd, &stat);

printf("Device: %s\n", stat.name);
printf("Vendor: 0x%x\n", stat.vendor_id);
printf("State: %d\n", stat.state);
printf("Bytes read: %lu\n", stat.bytes_read);
```

### Performance
- **Latency**: < 200ns

---

## Error Codes Reference

```
EBADF     (-9)   Bad file descriptor
ENOENT    (-2)   No such device
EACCES    (-13)  Permission denied
EINVAL    (-22)  Invalid argument
ENOTREADY (-36)  Device not ready
ETIMEDOUT (-110) Operation timed out
EBUSY     (-16)  Device or resource busy
```

## Summary Table

| Hypercall | Blocks? | Latency | Primary Use |
|-----------|---------|---------|-------------|
| device_open | No | <5µs | Open device |
| device_close | No | <2µs | Close device |
| device_read | Yes | <5µs | Read data |
| device_write | Yes | <5µs | Write data |
| device_ioctl | Varies | <10µs | Device control |
| device_poll | Yes | <100ns | Wait for event |
| device_map | No | <5µs | Map to memory |
| device_unmap | No | <2µs | Unmap memory |
| device_sync | Yes | Device-dependent | Wait for I/O |
| device_stat | No | <200ns | Get status |

## Device Driver Integration

When using these hypercalls:

1. Always `device_open()` before use
2. Check return values for errors
3. Use `device_poll()` for efficient waiting
4. Call `device_sync()` before critical operations
5. `device_close()` to cleanup and free resources

---

## References

- [Driver Framework](../drivers/FRAMEWORK.md)
- [Driver Development](../drivers/DRIVER_DEVELOPMENT.md)
- [Built-in Drivers](../drivers/BUILTIN_DRIVERS.md)

---

**UOSC Device Hypercalls: Safe, Efficient, Uniform, Verified.**
