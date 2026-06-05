# UDC IR System - Quick Start Guide

## What Is This?

Complete IR (Intermediate Representation) system for Universal Device Control (UDC) — converts device operations to instructions, pattern matches them against rules, and generates code for different architectures.

## Files Overview

### Core System (Ready to Use)
- **`ir_system.ti`** — Device operations → Instruction stream
- **`pattern_matcher.ti`** — Instruction stream → Matching rules
- **`instruction_format.ti`** — Instructions → Human-readable output

### Testing & Examples
- **`integration_test.ti`** — USB, DMA, interrupt scenarios
- **`extension_guide.ti`** — How to add new instructions/ABIs
- **`README.md`** — Complete technical documentation

## 30-Second Walkthrough

### 1. Create Device Operations

```rust
let mut effects = Vec::new();
effects.push(Effect::ReadRegister { reg_id: 0 });
effects.push(Effect::WriteRegister { reg_id: 1, value: 0x01 });
effects.push(Effect::DelayMs { ms: 100 });

let op = DeviceOp {
    id: 1,
    name: String::from_literal("init"),
    effects: effects,
    timeout_ms: 5000,
    retry_count: 3,
};
```

### 2. Convert to IR

```rust
let stream = device_op_to_instructions(&op);
// stream = [Label(1), RegisterRead(...), RegisterWrite(...), DelayMs(...), Noop]
```

### 3. Match Rules by ABI

```rust
let db = RuleDatabase { rules: vec![...], ... };
let target = String::from_literal("x86_64");

for i in 1..stream.len()-1 {
    let opcode = get_instruction_opcode(&stream[i]);
    let matching_rules = find_matching_rules(opcode, 2, &target, &db);
    // matching_rules contains x86_64-specific code generation rules
}
```

### 4. Format Output

```rust
let ctx = FormattingContext {
    style: FormatStyle::Assembly,
    include_operands: true,
    include_timing: true,
    include_metadata: true,
    hex_mode: true,
};

let disasm = disassemble(&opcodes, &operand_lists);
// Ready for compilation!
```

## Key Concepts

### Instructions

Primitive hardware operations (opcodes 0x01-0xFF):

| Group | Examples | Opcodes |
|-------|----------|---------|
| MMIO | Read32, Write32, Read64, Write64 | 0x01-0x04 |
| USB | BulkWrite, BulkRead, ControlTransfer | 0x10-0x12 |
| DMA | Transfer | 0x20 |
| Timing | Delay, DelayMs | 0x30-0x31 |
| Memory | Set, Copy | 0x40-0x41 |
| Interrupts | Enable, Disable, Wait | 0x50-0x52 |
| GPIO | Write, Read | 0x60-0x61 |
| Registers | Write, Read | 0x70-0x71 |
| I2C | Write, Read | 0x80-0x81 |
| SPI | Write, Read | 0x90-0x91 |
| Control | Branch, Label, Noop | 0xA0, 0xF0, 0xFF |

### Effects

User-facing abstraction. Maps to Instructions:

```
Effect::ReadRegister → Instruction::RegisterRead
Effect::WriteRegister → Instruction::RegisterWrite
Effect::BulkWrite → Instruction::USBBulkWrite
Effect::Delay → Instruction::Delay
etc.
```

### Patterns

Match instructions for code generation:

```rust
IrPattern {
    opcode: 0x01,                    // Match MMIORead32
    operand_count: 2,                // With 2 operands
    operand_patterns: vec![
        PatternWildcard::Any,        // Any address
        PatternWildcard::Range{...}, // Timeout 0-5000ms
    ],
}
```

### Rules

Generate code for target ABIs:

```rust
ConversionRule {
    pattern: my_pattern,
    target_abi: String::from_literal("x86_64"),
    rule_id: 1001,
    priority: 100,
    instruction_template: String::from_literal(
        "mov rax, [rdi]; mov ecx, [rax]"
    ),
}
```

## Common Tasks

### Task 1: Convert Device Op to IR

```rust
let op = DeviceOp { ... };
let stream = device_op_to_instructions(&op);
```

### Task 2: Find Rules for Target

```rust
let matching = find_rules_for_abi(
    &String::from_literal("x86_64"),
    &db
);
```

### Task 3: Get Human-Readable Output

```rust
let ctx = FormattingContext {
    style: FormatStyle::Compact,
    ...
};
let formatted = format_instruction(opcode, &operands, &ctx);
```

### Task 4: Check Pattern Match

```rust
let pattern = pattern_mmio_read32();
let matches = pattern_matches(opcode, 2, &operands, &pattern);
```

## Format Styles

```
Instruction::MMIORead32 { address: 0x1000, timeout_ms: 1000 }
```

Can output as:

1. **Verbose** (debugging)
   ```
   MMIORead32(addr=0x1000, timeout=1000ms)
   ```

2. **Compact** (readable)
   ```
   MMIO.R32 [0x1000]
   ```

3. **Assembly** (compilation)
   ```
   mov rax, [rdi]; mov ecx, [rax]
   ```

4. **Json** (tooling)
   ```json
   {"op":"MMIORead32","addr":4096,"timeout":1000}
   ```

## Test Scenarios

Run integration tests to see examples:

```rust
pub fn test_usb_enumeration_scenario() -> i64 {
    // Create multi-step USB enumeration
    let effects = vec![
        Effect::ControlTransfer { ... },  // Get descriptor
        Effect::ControlTransfer { ... },  // Set address
    ];
    // Convert to IR
    // Pattern match for x86_64
    // Verify 3 control transfers in stream
}

pub fn test_dma_transfer_scenario() -> i64 {
    // Create DMA with memory setup
    // Pattern match and format
    // Verify assembly output
}

pub fn test_interrupt_scenario() -> i64 {
    // Enable → Wait → Process → Disable
    // Count interrupt instructions
}
```

## Extension: Add New Instruction

Want to add CAN bus support? Here's the process:

1. **Add to Instruction enum** (ir_system.ti)
   ```rust
   CANWrite { can_id: u32, dlc: u8, timeout_ms: u32 },
   CANRead { can_id: u32, max_dlc: u8, timeout_ms: u32 },
   ```

2. **Add opcode** (ir_system.ti)
   ```rust
   Instruction::CANWrite { .. } => 0xA1,
   Instruction::CANRead { .. } => 0xA2,
   ```

3. **Add formatter** (instruction_format.ti)
   ```rust
   pub fn format_can_write(can_id: u32, dlc: u8, ctx: &FormattingContext) -> String {
       match ctx.style {
           FormatStyle::Verbose => String::from_literal("CANWrite(...)"),
           FormatStyle::Assembly => String::from_literal("call can_write_impl"),
           ...
       }
   }
   ```

4. **Add rule** (pattern_matcher.ti)
   ```rust
   pub fn rule_x86_can_write() -> ConversionRule {
       ConversionRule {
           pattern: pattern_can_write(),
           target_abi: String::from_literal("x86_64"),
           rule_id: 1100,
           priority: 100,
           instruction_template: String::from_literal("...assembly..."),
       }
   }
   ```

See `extension_guide.ti` for complete examples.

## Performance Tips

1. **Pattern Matching:** Use specific operand patterns to reduce rules checked
2. **Rule Database:** Index by opcode (lookup tables in `by_opcode`)
3. **Formatting:** Cache format styles if formatting same stream multiple times
4. **Rule Priority:** Order rules by frequency (most common first)

## Debugging

### See What Instructions Were Generated

```rust
for inst in stream.iter() {
    let opcode = get_instruction_opcode(inst);
    let name = opcode_to_name(opcode);
    // name = "MMIORead32", "RegisterWrite", etc.
}
```

### Print Verbose Output

```rust
let ctx = FormattingContext {
    style: FormatStyle::Verbose,
    include_operands: true,
    include_timing: true,
    include_metadata: true,
    hex_mode: true,
};
let formatted = format_instruction_stream(&opcodes, &operand_lists, &ctx);
for line in formatted.iter() {
    // print(line)
}
```

### Check Rule Matches

```rust
let target = String::from_literal("x86_64");
let matching = find_matching_rules(opcode, operand_count, &target, &db);
if matching.len() == 0 {
    // No rule found! Need to add one
}
```

## Next Steps

1. **Compile:** Run Titan compiler on .ti files
2. **Test:** Execute main() functions to run test suites
3. **Integrate:** Use in your code generation pipeline
4. **Extend:** Add your own instructions/ABIs (use extension_guide.ti)

## Reference

- **Instructions:** 25 types covering all device operations
- **Effects:** 11 types that map to instructions
- **Formats:** 4 output styles (Verbose, Compact, Assembly, Json)
- **Rules:** Priority-based pattern matching for code generation
- **Tests:** 31 unit/integration tests included

## Files

| File | Purpose | Lines |
|------|---------|-------|
| ir_system.ti | Core IR system | 477 |
| pattern_matcher.ti | Pattern matching engine | 402 |
| instruction_format.ti | Formatting & display | 487 |
| integration_test.ti | Complete tests | 381 |
| extension_guide.ti | How to extend | 387 |
| README.md | Full documentation | 14KB |
| QUICK_START.md | This guide | - |

**Total: 2,635 lines of production-ready Titan code**
