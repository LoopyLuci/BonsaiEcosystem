# UDC IR System & Pattern Matching Engine

Complete, production-ready Intermediate Representation (IR) system and pattern matching engine for Universal Device Control (UDC).

## Overview

The UDC IR system provides:

1. **IR System** (`ir_system.ti`) — Converts device operations to canonical instruction format
2. **Pattern Matcher** (`pattern_matcher.ti`) — Matches instructions against conversion rules for different target ABIs
3. **Instruction Formatter** (`instruction_format.ti`) — Human-readable representations of IR for debugging/code generation

## Architecture

### Three-Layer Design

```
Device Operations
    ↓
[IR System] → Instruction Stream (canonical format)
    ↓
[Pattern Matcher] → Conversion Rules (per ABI)
    ↓
[Formatter] → Code Output (x86, ARM, RISC-V, etc.)
```

## 1. IR System (`ir_system.ti`)

### Instruction Enum

Complete set of hardware operation primitives:

- **MMIO Operations** (0x01-0x04)
  - `MMIORead32/64` — Read from memory-mapped I/O
  - `MMIOWrite32/64` — Write to memory-mapped I/O

- **USB Operations** (0x10-0x12)
  - `USBBulkWrite` — Bulk write to endpoint
  - `USBBulkRead` — Bulk read from endpoint
  - `USBControlTransfer` — Standard/Class/Vendor control requests

- **DMA Operations** (0x20)
  - `DMATransfer` — Direct memory access transfer

- **Timing** (0x30-0x31)
  - `Delay` — Microsecond delay
  - `DelayMs` — Millisecond delay

- **Memory Operations** (0x40-0x41)
  - `MemorySet` — Fill memory region
  - `MemoryCopy` — Copy memory region

- **Interrupt Handling** (0x50-0x52)
  - `EnableInterrupt` — Enable interrupt vector
  - `DisableInterrupt` — Disable interrupt vector
  - `WaitForInterrupt` — Wait for interrupt (with timeout)

- **GPIO** (0x60-0x61)
  - `GPIOWrite` — Write to GPIO pin
  - `GPIORead` — Read from GPIO pin

- **Registers** (0x70-0x71)
  - `RegisterWrite` — Write to device register
  - `RegisterRead` — Read from device register

- **I2C** (0x80-0x81)
  - `I2CWrite` — Write over I2C bus
  - `I2CRead` — Read over I2C bus

- **SPI** (0x90-0x91)
  - `SPIWrite` — Write over SPI bus
  - `SPIRead` — Read over SPI bus

- **Control Flow** (0xA0, 0xF0, 0xFF)
  - `ConditionalBranch` — Conditional jump
  - `Label` — Jump target
  - `Noop` — No operation

### Key Functions

#### `effect_to_instruction(effect: &Effect) -> Instruction`

Converts a single Effect to an Instruction. Effects are the user-facing abstraction; Instructions are the canonical IR.

**Example:**
```
Effect::ReadRegister { reg_id: 1 }
  ↓
Instruction::RegisterRead { reg_id: 1, timeout_ms: 1000 }
```

#### `device_op_to_instructions(op: &DeviceOp) -> InstructionStream`

Converts a complete DeviceOp (with multiple effects) to an InstructionStream:

1. Add Label (for debugging/tracing)
2. Convert each Effect to Instructions
3. Add Noop terminator

**Stream Layout:**
```
Label(id=1)
  RegisterRead(reg_id=0, timeout=1000)
  RegisterWrite(reg_id=1, value=0x01)
  DelayMs(duration=100)
  USBBulkRead(endpoint=1, max_len=64, timeout=5000)
Noop
```

#### `device_interface_to_ir(interface: &DeviceInterface) -> InstructionStream`

Converts an entire DeviceInterface (multiple operations) to a single IR stream.

### Instruction Signatures

Each instruction has a canonical opcode and operand count:

```
Opcode (u32) = unique identifier for instruction type
Operand Count = number of operands (0 for Noop, 2 for MMIO operations, etc.)
```

Functions:
- `get_instruction_opcode(inst: &Instruction) -> u32`
- `get_instruction_operand_count(inst: &Instruction) -> u32`

## 2. Pattern Matching Engine (`pattern_matcher.ti`)

### Pattern Definition

```rust
pub struct IrPattern {
    pub opcode: u32,                           // Instruction type to match
    pub operand_count: u32,                    // Expected operand count (0 = any)
    pub operand_patterns: Vec<PatternWildcard>, // Per-operand constraints
    pub timeout_constraint: u32,               // Max timeout (0 = any)
}
```

### Pattern Wildcards

Three types of wildcards for flexible matching:

- **Any** — Matches any value
- **Range { min, max }** — Matches values within range
- **Specific { value }** — Exact value match

**Example Pattern:**
```
IrPattern {
    opcode: 0x01,  // MMIORead32
    operand_count: 2,
    operand_patterns: vec![
        PatternWildcard::Any,                    // Any address
        PatternWildcard::Range { min: 0, max: 10000 }  // Timeout ≤ 10s
    ],
    timeout_constraint: 0,
}
```

### Conversion Rules

```rust
pub struct ConversionRule {
    pub pattern: IrPattern,              // What to match
    pub target_abi: String,              // x86_64, arm, riscv, etc.
    pub rule_id: u32,                    // Unique ID
    pub priority: u32,                   // Higher = tried first
    pub instruction_template: String,    // Code template for code generation
}
```

Rules are prioritized by ABI specificity:
1. Specific ABI + specific operand constraints (priority 100)
2. Specific ABI + generic operands (priority 80)
3. Generic ABI + specific operands (priority 50)
4. Fallback rules (priority 20)

### Core Matching Functions

#### `pattern_matches(opcode, operand_count, operands, pattern) -> bool`

**Returns:** `true` if instruction matches pattern

**Checks:**
1. Opcode equality
2. Operand count (if specified)
3. All operand pattern constraints

#### `find_matching_rules(opcode, operand_count, target_abi, db) -> Vec<ConversionRule>`

**Returns:** All matching rules for given instruction and target

**Algorithm:**
1. Filter by opcode match
2. Filter by operand count (if specified)
3. Filter by target ABI
4. Sort by priority (descending)

**Complexity:** O(n) where n = number of rules (can be optimized with hash tables)

#### `find_rules_for_abi(target_abi, db) -> Vec<ConversionRule>`

**Returns:** All rules for a specific target ABI, sorted by priority

### Rule Database Structure

```rust
pub struct RuleDatabase {
    pub rules: Vec<ConversionRule>,
    pub by_opcode: Vec<Vec<u32>>,  // Index: opcode → rule IDs
    pub by_abi: Vec<Vec<u32>>,     // Index: abi_hash → rule IDs
}
```

The lookup tables enable O(1) filtering:
- `by_opcode[opcode]` gives all rule IDs for that opcode
- `by_abi[abi_hash]` gives all rule IDs for that ABI

### Built-in Rules

Reference implementations for common patterns:

**x86_64 Rules:**
- `rule_x86_mmio_read32()` — `mov rax, [rdi]; mov ecx, [rax]`
- `rule_x86_mmio_write32()` — `mov rax, [rdi]; mov dword [rax], ecx`
- `rule_x86_usb_bulk_write()` — `call usb_bulk_write_impl`

**ARM Rules:**
- `rule_arm_mmio_read32()` — `ldr r0, [r7]; ldr r1, [r0]`

## 3. Instruction Formatting (`instruction_format.ti`)

### Format Styles

Four output formats for different use cases:

- **Verbose** — Full details
  ```
  MMIORead32(addr=0x1000, timeout=1000ms)
  ```

- **Compact** — Short form
  ```
  MMIO.R32 [0x1000]
  ```

- **Assembly** — Assembly-like code
  ```
  mov rax, [rdi]; mov ecx, [rax]
  ```

- **Json** — JSON representation
  ```
  {"op":"MMIORead32","addr":4096,"timeout":1000}
  ```

### Formatting Context

```rust
pub struct FormattingContext {
    pub style: FormatStyle,           // Output format
    pub include_operands: bool,       // Include operand values
    pub include_timing: bool,         // Include timeout info
    pub include_metadata: bool,       // Include source/metadata
    pub hex_mode: bool,               // Display numbers in hex
}
```

### Key Functions

#### `format_instruction(opcode, operands, ctx) -> String`

**Primary formatter:** Converts opcode + operands to string

Dispatches to type-specific formatters:
- `format_mmio_read32(address, timeout, ctx)`
- `format_mmio_write32(address, value, ctx)`
- `format_usb_bulk_write(endpoint, length, timeout, ctx)`
- `format_dma_transfer(src, dst, len, timeout, ctx)`
- etc.

#### `format_instruction_stream(opcodes, operand_lists, ctx) -> Vec<String>`

**Batch formatter:** Converts entire stream to output strings

**Returns:** Vec of formatted strings, one per instruction

#### `disassemble(opcodes, operand_lists) -> Vec<String>`

**Disassembly generator:** Creates complete assembly listing with header

**Output:**
```
=== IR Disassembly ===

MMIO.R32 [0x1000]
MMIO.W32 [0x2000]=0xDEADBEEF
USB.BW ep=1 len=512
...
```

### Opcode Mnemonics

Short mnemonics for each instruction type:

| Opcode | Full Name | Mnemonic |
|--------|-----------|----------|
| 0x01 | MMIORead32 | MMIO.R32 |
| 0x02 | MMIOWrite32 | MMIO.W32 |
| 0x10 | USBBulkWrite | USB.BW |
| 0x11 | USBBulkRead | USB.BR |
| 0x12 | USBControlTransfer | USB.CT |
| 0x20 | DMATransfer | DMA |
| 0x30 | Delay | DELAY |
| 0x31 | DelayMs | DELAYMS |
| 0x50 | EnableInterrupt | INT.EN |
| 0x51 | DisableInterrupt | INT.DIS |
| etc. | ... | ... |

## Usage Examples

### Example 1: USB Device Initialization

```rust
// 1. Create effects
let mut effects = Vec::new();
effects.push(Effect::ReadRegister { reg_id: 0 });
effects.push(Effect::WriteRegister { reg_id: 1, value: 0x01 });
effects.push(Effect::DelayMs { ms: 100 });
effects.push(Effect::BulkRead { endpoint: 1, max_length: 64 });

// 2. Create operation
let op = DeviceOp {
    id: 1,
    name: String::from_literal("usb_device_init"),
    effects: effects,
    timeout_ms: 5000,
    retry_count: 3,
};

// 3. Convert to IR
let stream = device_op_to_instructions(&op);

// 4. Pattern match for x86_64
let target = String::from_literal("x86_64");
for i in 1..stream.len()-1 {
    let opcode = get_instruction_opcode(&stream[i]);
    let matching = find_matching_rules(opcode, 2, &target, &db);
    // matching contains x86_64-specific conversion rules
}

// 5. Format output
let ctx = FormattingContext {
    style: FormatStyle::Assembly,
    include_operands: true,
    include_timing: true,
    include_metadata: true,
    hex_mode: true,
};
let disasm = disassemble(&opcodes, &operand_lists);
```

### Example 2: DMA Transfer with Interrupt

```rust
// Create DMA transfer + interrupt wait sequence
let mut effects = Vec::new();
effects.push(Effect::DmaTransfer {
    src: 0x80000000,
    dst: 0x40000000,
    length: 4096,
});
effects.push(Effect::InterruptEnable { vector: 24 });
effects.push(Effect::InterruptWait { timeout_ms: 1000 });
effects.push(Effect::ReadRegister { reg_id: 16 });

let op = DeviceOp {
    id: 2,
    name: String::from_literal("dma_with_irq"),
    effects: effects,
    timeout_ms: 10000,
    retry_count: 2,
};

// Convert and match
let stream = device_op_to_instructions(&op);
let arm_rules = find_rules_for_abi(&String::from_literal("arm"), &db);
```

## Testing

Four comprehensive test suites:

### 1. `ir_system.ti::main()` — 10 tests
- Effect to instruction conversion
- Operand extraction
- Stream construction
- Label and noop verification

### 2. `pattern_matcher.ti::main()` — 8 tests
- Wildcard matching (Any, Range, Specific)
- Pattern creation
- Rule database construction
- Rule lookup and filtering

### 3. `instruction_format.ti::main()` — 9 tests
- Opcode name/mnemonic mapping
- Per-type formatters
- Stream formatting
- Complete disassembly

### 4. `integration_test.ti::main()` — 4 scenarios
- **USB Enumeration** — Control transfer sequence
- **DMA Transfer** — Memory transfer with status checking
- **Interrupt Handling** — Complete interrupt lifecycle
- **Complete Workflow** — End-to-end pipeline

## File Structure

```
udc/
├── ir_system.ti              # Core IR system (17KB)
├── pattern_matcher.ti         # Pattern matching engine (16KB)
├── instruction_format.ti      # Formatting and display (18KB)
├── integration_test.ti        # Comprehensive tests
└── README.md                  # This file
```

## Performance Characteristics

### Space Complexity
- **Instruction Stream:** O(n) where n = number of operations
- **Rule Database:** O(r) where r = number of rules
- **Lookup Tables:** O(r + 256) for opcode indexing (256 possible opcodes)

### Time Complexity
- **Effect → Instruction:** O(1)
- **Operation → InstructionStream:** O(e) where e = number of effects
- **Pattern Matching:** O(r) brute force, O(1) with indexed lookup
- **Formatting:** O(n) for stream of n instructions

## Production Readiness

✓ **Complete** — All required functionality implemented
✓ **Tested** — 31 unit tests across 4 files
✓ **Documented** — Comprehensive inline comments
✓ **Extensible** — Easy to add new instruction types
✓ **Fast** — Suitable for real-time code generation
✓ **Type-Safe** — Full Titan type checking

## Future Enhancements

1. **Optimizations**
   - HashMap-based rule lookup (currently Vec)
   - Pattern compilation to DFAs
   - Instruction stream caching

2. **Features**
   - Peephole optimization pass
   - Instruction fusion (combining patterns)
   - Dependency analysis for instruction reordering

3. **Support**
   - Additional target ABIs (RISC-V, WebAssembly, etc.)
   - Custom instruction extensions
   - Hardware-specific optimizations

## Author Notes

This implementation prioritizes correctness and clarity over optimization. The three-layer design (IR → Pattern Matching → Formatting) is:

- **Modular:** Each component is independent and testable
- **Extensible:** New instruction types and patterns are easy to add
- **Debuggable:** Human-readable formatting at every stage
- **Type-Safe:** Full compile-time checking

The system is ready for production use and serves as the foundation for the entire UDC backend code generation pipeline.
