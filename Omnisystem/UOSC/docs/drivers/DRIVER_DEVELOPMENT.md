# UOSC Driver Development Guide

Complete guide to writing, testing, and integrating device drivers for UOSC.

## Overview

Writing a UOSC driver involves:

1. Implementing device interface (`Device` trait)
2. Registering with kernel
3. Handling interrupts
4. Managing device state
5. Testing and verification

All drivers run in kernel-mode but are **isolated from kernel** - a crashed driver cannot crash UOSC kernel.

## Driver Architecture

### Device Interface (Required)

Every driver must implement this interface:

```c
struct DeviceOps {
    // Identification
    const char* (*get_name)(Device* dev);
    DeviceClass (*get_class)(Device* dev);
    u32 (*get_vendor_id)(Device* dev);
    u32 (*get_device_id)(Device* dev);
    
    // Lifecycle
    i32 (*initialize)(Device* dev, DeviceConfig* config);
    i32 (*shutdown)(Device* dev);
    i32 (*reset)(Device* dev);
    
    // Configuration
    i32 (*set_config)(Device* dev, const char* key, const void* value);
    
    // State Query
    DeviceState (*get_state)(Device* dev);
    DeviceStats (*get_stats)(Device* dev);
    Capabilities (*get_capabilities)(Device* dev);
    
    // I/O (device-specific)
    // For block devices:
    i32 (*read)(Device* dev, u64 block, u32 count, Buffer* out);
    i32 (*write)(Device* dev, u64 block, const Buffer* data);
    
    // For character devices:
    i32 (*read_bytes)(Device* dev, u32 count, Buffer* out);
    i32 (*write_bytes)(Device* dev, const Buffer* data);
    
    // For network devices:
    i32 (*transmit)(Device* dev, NetworkPacket* packet);
    i32 (*receive)(Device* dev, NetworkPacket* out);
    
    // Control
    i32 (*ioctl)(Device* dev, u32 command, void* args);
    i32 (*poll)(Device* dev, i32 events);
    
    // Interrupt handling
    i32 (*interrupt_handler)(Device* dev, InterruptContext* ctx);
};
```

## Step-by-Step: Writing a Device Driver

### Step 1: Define Driver Data Structure

```c
// drivers/my_device/my_device.h
#ifndef MY_DEVICE_H
#define MY_DEVICE_H

#include <uosc/device.h>
#include <uosc/interrupt.h>

#define MY_DEVICE_NAME "my_device"
#define MY_DEVICE_VENDOR 0x1234
#define MY_DEVICE_ID 0x5678

typedef struct {
    // Hardware registers
    volatile u32* control_reg;
    volatile u32* status_reg;
    volatile u32* data_reg;
    
    // Device state
    DeviceState state;
    u32 interrupt_count;
    
    // Buffers
    RingBuffer* tx_queue;
    RingBuffer* rx_queue;
    
    // Synchronization
    Mutex* state_mutex;
} MyDeviceState;

typedef struct {
    Device base;
    MyDeviceState state;
} MyDevice;

#endif // MY_DEVICE_H
```

### Step 2: Implement Device Interface

```c
// drivers/my_device/my_device.c
#include "my_device.h"
#include <uosc/kernel.h>

// Identification functions
static const char* my_device_get_name(Device* dev) {
    return MY_DEVICE_NAME;
}

static u32 my_device_get_vendor_id(Device* dev) {
    return MY_DEVICE_VENDOR;
}

static u32 my_device_get_device_id(Device* dev) {
    return MY_DEVICE_ID;
}

// Lifecycle functions
static i32 my_device_initialize(Device* dev, DeviceConfig* config) {
    MyDevice* mydev = (MyDevice*)dev;
    
    // Map hardware registers (if needed)
    mydev->state.control_reg = (volatile u32*)config->base_addr;
    mydev->state.status_reg = mydev->state.control_reg + 1;
    mydev->state.data_reg = mydev->state.control_reg + 2;
    
    // Initialize state
    mydev->state.state = DEVICE_INITIALIZED;
    mydev->state.interrupt_count = 0;
    
    // Create buffers
    mydev->state.tx_queue = ringbuffer_create(1024);
    mydev->state.rx_queue = ringbuffer_create(1024);
    
    // Create synchronization primitive
    mydev->state.state_mutex = mutex_create();
    
    // Reset hardware
    *mydev->state.control_reg = 0x0;
    
    // Register interrupt handler
    kernel_register_interrupt(
        config->irq_number,
        my_device_interrupt_handler,
        (void*)dev
    );
    
    // Enable interrupts
    *mydev->state.control_reg |= 0x1;  // ENABLE_INT
    
    mydev->state.state = DEVICE_READY;
    return 0;  // Success
}

static i32 my_device_shutdown(Device* dev) {
    MyDevice* mydev = (MyDevice*)dev;
    
    // Disable interrupts
    *mydev->state.control_reg = 0x0;
    
    // Cleanup
    ringbuffer_destroy(mydev->state.tx_queue);
    ringbuffer_destroy(mydev->state.rx_queue);
    mutex_destroy(mydev->state.state_mutex);
    
    mydev->state.state = DEVICE_SHUTDOWN;
    return 0;
}

// I/O functions (example: character device)
static i32 my_device_read_bytes(Device* dev, u32 count, Buffer* out) {
    MyDevice* mydev = (MyDevice*)dev;
    
    if (mydev->state.state != DEVICE_READY) {
        return -ENOTREADY;
    }
    
    mutex_lock(mydev->state.state_mutex);
    
    // Read from RX queue (filled by interrupt handler)
    u32 bytes_read = 0;
    while (bytes_read < count && ringbuffer_size(mydev->state.rx_queue) > 0) {
        u8 byte = ringbuffer_pop(mydev->state.rx_queue);
        buffer_append(out, &byte, 1);
        bytes_read++;
    }
    
    mutex_unlock(mydev->state.state_mutex);
    
    return bytes_read;
}

static i32 my_device_write_bytes(Device* dev, const Buffer* data) {
    MyDevice* mydev = (MyDevice*)dev;
    
    if (mydev->state.state != DEVICE_READY) {
        return -ENOTREADY;
    }
    
    mutex_lock(mydev->state.state_mutex);
    
    // Write data to hardware
    u32 written = 0;
    for (u32 i = 0; i < buffer_size(data); i++) {
        *mydev->state.data_reg = buffer_get(data, i);
        written++;
    }
    
    mutex_unlock(mydev->state.state_mutex);
    
    return written;
}

// State query
static DeviceState my_device_get_state(Device* dev) {
    MyDevice* mydev = (MyDevice*)dev;
    return mydev->state.state;
}

// Interrupt handler
static i32 my_device_interrupt_handler(void* context) {
    MyDevice* mydev = (MyDevice*)context;
    
    // Read interrupt status
    u32 status = *mydev->state.status_reg;
    
    if ((status & 0x1) == 0) {
        return 0;  // Not our interrupt
    }
    
    // Handle RX interrupt
    if (status & 0x2) {
        u8 data = *mydev->state.data_reg;
        ringbuffer_push(mydev->state.rx_queue, data);
    }
    
    // Clear interrupt
    *mydev->state.control_reg |= 0x80;  // CLEAR_INT
    
    mydev->state.interrupt_count++;
    
    return 1;  // Handled
}

// Device operations table (required)
static DeviceOps my_device_ops = {
    .get_name = my_device_get_name,
    .get_vendor_id = my_device_get_vendor_id,
    .get_device_id = my_device_get_device_id,
    .initialize = my_device_initialize,
    .shutdown = my_device_shutdown,
    .reset = my_device_reset,
    .get_state = my_device_get_state,
    .read_bytes = my_device_read_bytes,
    .write_bytes = my_device_write_bytes,
    .interrupt_handler = my_device_interrupt_handler,
    // ... other operations ...
};
```

### Step 3: Register Driver

```c
// drivers/my_device/driver_init.c
#include "my_device.h"
#include <uosc/driver.h>

// Driver registration structure
static DriverRegistration my_device_driver = {
    .name = MY_DEVICE_NAME,
    .version = "1.0.0",
    .compatible_devices = {
        { .vendor = MY_DEVICE_VENDOR, .device = MY_DEVICE_ID },
        { 0, 0 }  // Sentinel
    },
    .ops = &my_device_ops,
};

// Export driver (kernel calls this on load)
DriverRegistration* driver_init(void) {
    return &my_device_driver;
}
```

## Driver Lifecycle

### Probing

When kernel discovers hardware matching driver's compatible devices:

```c
kernel_probe_device(device_info) {
    // 1. Find matching driver
    driver = find_driver(device_info.vendor, device_info.device)
    
    // 2. Call driver's probe
    if (driver.ops.probe(device_info) == 0) {
        // 3. Driver claims device
        allocate_device_structure()
        device.ops = driver.ops
        device.state = PROBED
        
        // 4. Initialize device
        device.ops.initialize(device, config)
    }
}
```

### Operation

Once initialized, driver services requests:

```c
user_program calls:
  device_read(fd, buffer, 1024)
  
  ↓
  
kernel_read(fd) {
    device = fd.device
    return device.ops.read(device, ...)
  }
  
  ↓
  
my_device_read_bytes(device, ...) {
    // Acquire state_mutex
    // Read from RX queue (filled by interrupt handler)
    // Release state_mutex
    // Return data
  }
```

### Interrupts

Hardware interrupt triggers handler:

```c
hardware_interrupt(IRQ_N) {
    ↓
  kernel_interrupt_handler(IRQ_N) {
      handler = kernel.interrupt_handlers[IRQ_N]
      return handler(context)
    }
    
    ↓
    
  my_device_interrupt_handler(context) {
      // Read status, process data, clear interrupt
      // Update internal state
      return 1  // Handled
    }
}
```

## Testing a Driver

### Unit Testing

```c
// tests/test_my_device.c
#include <uosc_test.h>
#include "drivers/my_device/my_device.h"

void test_device_initialization() {
    MyDevice dev = {};
    DeviceConfig config = {
        .base_addr = 0x1000,
        .irq_number = 5
    };
    
    ASSERT_EQ(0, my_device_initialize((Device*)&dev, &config));
    ASSERT_EQ(DEVICE_READY, my_device_get_state((Device*)&dev));
}

void test_read_write() {
    MyDevice dev = {};
    DeviceConfig config = {};
    
    my_device_initialize((Device*)&dev, &config);
    
    Buffer write_buf = buffer_create_from_string("Hello");
    ASSERT_EQ(5, my_device_write_bytes((Device*)&dev, &write_buf));
    
    Buffer read_buf = buffer_create(10);
    my_device_read_bytes((Device*)&dev, 5, &read_buf);
    
    ASSERT_EQ(0, buffer_compare(&write_buf, &read_buf));
}

void test_interrupt_handling() {
    MyDevice dev = {};
    // ... setup ...
    
    // Simulate interrupt
    my_device_interrupt_handler(&dev);
    
    // Verify handler executed
    ASSERT_GT(dev.state.interrupt_count, 0);
}

// Register tests
TEST_SUITE(my_device) {
    RUN_TEST(test_device_initialization);
    RUN_TEST(test_read_write);
    RUN_TEST(test_interrupt_handling);
};
```

### Integration Testing

```bash
# Build driver with kernel
./build.sh --with-driver=my_device --mode=full

# Run in QEMU
./build/tests/integration_test_my_device

# Expected output:
[DRIVER] my_device probing...
[DRIVER] my_device initialized
[TEST] read/write test PASSED
[TEST] interrupt test PASSED
```

## Error Handling

### Driver Errors

```c
// Errors driver can return:
#define ENOTREADY  -1   // Device not ready
#define EBUSY      -2   // Device busy
#define EINTR      -3   // Interrupted
#define ENODATA    -4   // No data available
```

### Timeout Handling

```c
static i32 my_device_wait_for_ready(MyDevice* dev, u32 timeout_ms) {
    Timestamp deadline = kernel_get_time() + timeout_ms;
    
    while (kernel_get_time() < deadline) {
        if (dev->state.state == DEVICE_READY) {
            return 0;
        }
        kernel_schedule_yield();
    }
    
    return -ETIMEDOUT;
}
```

## Best Practices

### 1. Synchronization

```c
✓ GOOD: Use mutex for shared state
mutex_lock(state_mutex);
// ... access shared state ...
mutex_unlock(state_mutex);

✗ BAD: Access state without synchronization
state->counter++;  // Race condition!
```

### 2. Error Checking

```c
✓ GOOD: Check every operation
i32 result = my_device_write_bytes(dev, data);
if (result < 0) {
    return result;  // Propagate error
}

✗ BAD: Ignore errors
my_device_write_bytes(dev, data);  // What if it fails?
```

### 3. Resource Management

```c
✓ GOOD: Clean up on error
if (ringbuffer_create(size) == NULL) {
    goto cleanup_error;
}
if (mutex_create() == NULL) {
    ringbuffer_destroy(rb);
    goto cleanup_error;
}

✗ BAD: Leak resources on error
ringbuffer_create(size);
mutex_create();  // If this fails, rb is leaked
```

### 4. Interrupt Safety

```c
✓ GOOD: Minimal work in interrupt handler
static i32 interrupt_handler(void* context) {
    MyDevice* dev = (MyDevice*)context;
    u32 data = *dev->data_reg;
    ringbuffer_push(dev->rx_queue, data);  // Quick
    *dev->control_reg |= 0x80;  // Clear
    return 1;
}

✗ BAD: Heavy work in interrupt
static i32 interrupt_handler(void* context) {
    // ...
    for (i = 0; i < 1000000; i++) {
        process_data();  // Way too slow!
    }
}
```

## References

- [Driver Framework](FRAMEWORK.md)
- [Built-in Drivers](BUILTIN_DRIVERS.md)
- [Hypercall API](../hypercalls/)

---

**UOSC Drivers: Simple, Safe, Isolated, Verified.**
