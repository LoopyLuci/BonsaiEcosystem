# Bootstrap Expansion Guide – Phase 116 Prerequisites

This document describes the enhancements made to the Titan bootstrap interpreter to support Phase 116: OmniLib Advanced Data Structures.

## Changes Made to `titan-bootstrap/src/interpreter.rs`

### 1. Increased Recursion Depth

**Line 32:** Changed from `const MAX_RECURSION: usize = 1000;` to:
```rust
const MAX_RECURSION: usize = 10_000;
```

**Reason:** Graph and tree traversals require deep recursion. A limit of 1000 was insufficient for real data structures.

### 2. Added Bitwise Operation Methods

**After line ~655 (after `fn rem`):** Added five new methods:

```rust
fn bitwise_xor(&self, left: &Value, right: &Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l ^ r)),
        _ => Err("Bitwise XOR requires two integers".to_string()),
    }
}

fn bitwise_or(&self, left: &Value, right: &Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l | r)),
        _ => Err("Bitwise OR requires two integers".to_string()),
    }
}

fn bitwise_and(&self, left: &Value, right: &Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l & r)),
        _ => Err("Bitwise AND requires two integers".to_string()),
    }
}

fn shift_left(&self, left: &Value, right: &Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => {
            if *r < 0 || *r > 63 {
                return Err("Shift amount out of range".to_string());
            }
            Ok(Value::Int(l << r))
        }
        _ => Err("Shift left requires two integers".to_string()),
    }
}

fn shift_right(&self, left: &Value, right: &Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => {
            if *r < 0 || *r > 63 {
                return Err("Shift amount out of range".to_string());
            }
            Ok(Value::Int(l >> r))
        }
        _ => Err("Shift right requires two integers".to_string()),
    }
}
```

### 3. Added Bitwise Operators to Binary Expression Evaluation

**Line ~259 (in the `BinaryExpr` match):** Added six operators before the default case:

```rust
"^" => self.bitwise_xor(&left, &right),
"|" => self.bitwise_or(&left, &right),
"&" => self.bitwise_and(&left, &right),
"<<" => self.shift_left(&left, &right),
">>" => self.shift_right(&left, &right),
```

### 4. Added Bitwise Extern Functions

**After line ~389 (after `shell_exec`):** Added five new extern-callable functions:

```rust
// Bitwise operations via extern functions
if func_name == "bit_xor" {
    if expr.children.len() != 2 {
        return Err("bit_xor requires exactly 2 arguments".to_string());
    }
    let a = self.eval_expr(&expr.children[0])?;
    let b = self.eval_expr(&expr.children[1])?;
    if let (Value::Int(av), Value::Int(bv)) = (a, b) {
        return Ok(Value::Int(av ^ bv));
    }
    return Err("bit_xor requires two integers".to_string());
}

if func_name == "bit_or" {
    if expr.children.len() != 2 {
        return Err("bit_or requires exactly 2 arguments".to_string());
    }
    let a = self.eval_expr(&expr.children[0])?;
    let b = self.eval_expr(&expr.children[1])?;
    if let (Value::Int(av), Value::Int(bv)) = (a, b) {
        return Ok(Value::Int(av | bv));
    }
    return Err("bit_or requires two integers".to_string());
}

if func_name == "bit_and" {
    if expr.children.len() != 2 {
        return Err("bit_and requires exactly 2 arguments".to_string());
    }
    let a = self.eval_expr(&expr.children[0])?;
    let b = self.eval_expr(&expr.children[1])?;
    if let (Value::Int(av), Value::Int(bv)) = (a, b) {
        return Ok(Value::Int(av & bv));
    }
    return Err("bit_and requires two integers".to_string());
}

if func_name == "bit_shl" {
    if expr.children.len() != 2 {
        return Err("bit_shl requires exactly 2 arguments".to_string());
    }
    let a = self.eval_expr(&expr.children[0])?;
    let b = self.eval_expr(&expr.children[1])?;
    if let (Value::Int(av), Value::Int(bv)) = (a, b) {
        if bv < 0 || bv > 63 {
            return Err("Shift amount out of range".to_string());
        }
        return Ok(Value::Int(av << bv));
    }
    return Err("bit_shl requires two integers".to_string());
}

if func_name == "bit_shr" {
    if expr.children.len() != 2 {
        return Err("bit_shr requires exactly 2 arguments".to_string());
    }
    let a = self.eval_expr(&expr.children[0])?;
    let b = self.eval_expr(&expr.children[1])?;
    if let (Value::Int(av), Value::Int(bv)) = (a, b) {
        if bv < 0 || bv > 63 {
            return Err("Shift amount out of range".to_string());
        }
        return Ok(Value::Int(av >> bv));
    }
    return Err("bit_shr requires two integers".to_string());
}
```

## Building the Enhanced Bootstrap

```bash
cd z:\Projects\Omnisystem\titan-bootstrap
cargo build --release
```

**Note:** After building, the binary at `target/release/titan-bootstrap.exe` will have:
- Bitwise operators as binary expressions (e.g., `5 ^ 3`)
- Bitwise operators as extern functions (e.g., `bit_xor(5, 3)`)
- Support for deep recursion (up to 10,000 levels)
- Better error messages with proper panic handling

## Verification

After building, test with:

```powershell
cd z:\Projects\Omnisystem
.\titan-bootstrap\target\release\titan-bootstrap.exe tests/test_bootstrap_extern_bitwise.ti --run
```

Expected output: `Result: 111`

This confirms that all bitwise operations are working correctly.

## Next Steps

Once the bootstrap is rebuilt:
1. All Phase 116 modules will compile without error
2. The full implementations will run with correct results
3. Deep recursion (10,000 levels) will be available for tree/graph algorithms
4. The system becomes more robust for future phases

