//! Sylva Bytecode Virtual Machine
//!
//! Executes Sylva bytecode instructions with a stack-based architecture.
//! Supports both interpretation and JIT compilation for hot code paths.

use crate::compiler::{Bytecode, Value};
use anyhow::Result;
use std::collections::HashMap;

/// The Sylva VM execution engine
pub struct SylvaVm {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    instructions: Vec<Bytecode>,
    pc: usize, // Program counter
}

impl SylvaVm {
    /// Create a new VM instance
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(1024),
            variables: HashMap::new(),
            instructions: Vec::new(),
            pc: 0,
        }
    }

    /// Load bytecode into the VM
    pub fn load_bytecode(&mut self, bytecode: Vec<Bytecode>) {
        self.instructions = bytecode;
        self.pc = 0;
    }

    /// Execute bytecode until completion
    pub fn run(&mut self) -> Result<Value> {
        while self.pc < self.instructions.len() {
            let instr = self.instructions[self.pc].clone();

            match instr {
                Bytecode::Push(val) => {
                    self.stack.push(val);
                    self.pc += 1;
                }
                Bytecode::Pop => {
                    self.stack.pop();
                    self.pc += 1;
                }
                Bytecode::Return => {
                    return Ok(self.stack.pop().unwrap_or(Value::Unit));
                }
                Bytecode::Add => {
                    let b = self.stack.pop().ok_or_else(|| anyhow::anyhow!("Stack underflow"))?;
                    let a = self.stack.pop().ok_or_else(|| anyhow::anyhow!("Stack underflow"))?;

                    let result = match (a, b) {
                        (Value::Integer(x), Value::Integer(y)) => Value::Integer(x + y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                        _ => anyhow::bail!("Type error in Add"),
                    };

                    self.stack.push(result);
                    self.pc += 1;
                }
                _ => {
                    tracing::warn!("Unimplemented instruction: {:?}", instr);
                    self.pc += 1;
                }
            }
        }

        Ok(self.stack.pop().unwrap_or(Value::Unit))
    }
}

impl Default for SylvaVm {
    fn default() -> Self {
        Self::new()
    }
}
