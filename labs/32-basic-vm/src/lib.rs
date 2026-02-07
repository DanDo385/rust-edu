// Lab 32: Basic VM (Virtual Machine) - Library
//
// A stack-based virtual machine that executes custom bytecode.
// Demonstrates how interpreters work, instruction dispatch, and VM architecture.
// This is the foundation of JVM, Python, WebAssembly, and other interpreted languages.
//
// ============================================================================
// OWNERSHIP & MEMORY MODEL
// ============================================================================
// The VM owns its code (Vec<Instruction>), operand stack (Vec<i64>), call stack
// (Vec<usize>), and output buffer (Vec<i64>). All data lives on the heap via Vec.
// Instructions are cloned from the code vector during execution because we cannot
// hold a reference into self.code while mutating self through execute().
//
// The Instruction enum uses Clone (not Copy) because it may be extended with
// heap-allocated variants in the future. Each variant is small enough that
// cloning is essentially free (memcpy of ~16 bytes).

use std::fmt;

// ============================================================================
// INSTRUCTION SET
// ============================================================================
// Each instruction is an enum variant that the VM can execute.
// The enum is sized by its largest variant: tag (1 byte) + data (8 bytes for i64/usize).

/// Represents a single bytecode instruction for the stack-based VM.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Stack operations
    Push(i64),           // Push value onto stack
    Pop,                 // Remove top of stack
    Dup,                 // Duplicate top of stack
    Swap,                // Swap top two values
    Over,                // Copy second value to top

    // Arithmetic operations
    Add,                 // Pop two values, push sum
    Sub,                 // Pop two values, push difference
    Mul,                 // Pop two values, push product
    Div,                 // Pop two values, push quotient

    // Comparison operations
    Eq,                  // Pop two values, push 1 if equal, 0 otherwise
    Lt,                  // Pop two values, push 1 if less than
    Gt,                  // Pop two values, push 1 if greater than

    // Control flow
    Jmp(usize),          // Unconditional jump to instruction
    JmpIf(usize),        // Jump if top of stack is non-zero
    Call(usize),         // Call function (push return address)
    Ret,                 // Return from function

    // I/O
    Print,               // Pop top of stack and store in output buffer
    Halt,                // Stop execution
}

// ============================================================================
// VM ERROR TYPES
// ============================================================================

/// Errors that can occur during VM execution.
#[derive(Debug, PartialEq)]
pub enum VmError {
    StackUnderflow,
    DivisionByZero,
    InvalidJump(usize),
    InvalidInstruction,
    CallStackUnderflow,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VmError::StackUnderflow => write!(f, "Stack underflow"),
            VmError::DivisionByZero => write!(f, "Division by zero"),
            VmError::InvalidJump(ip) => write!(f, "Invalid jump to {}", ip),
            VmError::InvalidInstruction => write!(f, "Invalid instruction"),
            VmError::CallStackUnderflow => write!(f, "Call stack underflow"),
        }
    }
}

/// A convenience type alias for VM operations that can fail.
pub type VmResult<T> = Result<T, VmError>;

// ============================================================================
// VIRTUAL MACHINE STRUCTURE
// ============================================================================
// The VM maintains:
// - code: the program (list of instructions)
// - stack: operand stack for computation
// - call_stack: return addresses for function calls
// - ip: instruction pointer (program counter)
// - halted: whether the VM has stopped
// - output: captured output from Print instructions (for testability)

/// A stack-based virtual machine that executes bytecode instructions.
///
/// Instead of printing to stdout, Print instructions store values in the
/// `output` vector, making the VM fully testable without capturing stdout.
pub struct VirtualMachine {
    code: Vec<Instruction>,
    stack: Vec<i64>,
    call_stack: Vec<usize>,
    ip: usize,
    halted: bool,
    /// Values produced by Print instructions, stored in order.
    output: Vec<i64>,
}

impl VirtualMachine {
    /// Creates a new VM with the given bytecode program.
    ///
    /// The VM starts with an empty stack, empty call stack, instruction pointer
    /// at 0, and an empty output buffer.
    pub fn new(code: Vec<Instruction>) -> Self {
        VirtualMachine {
            code,
            stack: Vec::new(),
            call_stack: Vec::new(),
            ip: 0,
            halted: false,
            output: Vec::new(),
        }
    }

    /// Runs the VM until it halts or encounters an error.
    ///
    /// Returns Ok(()) on successful halt, or Err(VmError) if an error occurs.
    pub fn run(&mut self) -> VmResult<()> {
        while !self.halted {
            self.step()?;
        }
        Ok(())
    }

    /// Executes a single instruction (fetch-decode-execute cycle).
    ///
    /// This is the core of the interpreter loop:
    /// 1. Bounds-check the instruction pointer
    /// 2. Fetch (clone) the instruction at ip
    /// 3. Execute the instruction
    pub fn step(&mut self) -> VmResult<()> {
        if self.ip >= self.code.len() {
            return Err(VmError::InvalidJump(self.ip));
        }

        let instruction = self.code[self.ip].clone();
        self.execute(instruction)?;

        Ok(())
    }

    /// Decodes and executes a single instruction.
    ///
    /// Each instruction manipulates the stack, control flow, or output buffer.
    /// The instruction pointer is advanced after each instruction (except jumps
    /// and halt, which set it directly).
    pub fn execute(&mut self, instruction: Instruction) -> VmResult<()> {
        match instruction {
            // ================================================================
            // STACK OPERATIONS
            // ================================================================
            Instruction::Push(value) => {
                self.stack.push(value);
                self.ip += 1;
            }

            Instruction::Pop => {
                self.pop()?;
                self.ip += 1;
            }

            Instruction::Dup => {
                let value = self.peek()?;
                self.stack.push(value);
                self.ip += 1;
            }

            Instruction::Swap => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.stack.push(a);
                self.stack.push(b);
                self.ip += 1;
            }

            Instruction::Over => {
                if self.stack.len() < 2 {
                    return Err(VmError::StackUnderflow);
                }
                let value = self.stack[self.stack.len() - 2];
                self.stack.push(value);
                self.ip += 1;
            }

            // ================================================================
            // ARITHMETIC OPERATIONS
            // ================================================================
            Instruction::Add => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(a + b);
                self.ip += 1;
            }

            Instruction::Sub => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(a - b);
                self.ip += 1;
            }

            Instruction::Mul => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(a * b);
                self.ip += 1;
            }

            Instruction::Div => {
                let b = self.pop()?;
                if b == 0 {
                    return Err(VmError::DivisionByZero);
                }
                let a = self.pop()?;
                self.stack.push(a / b);
                self.ip += 1;
            }

            // ================================================================
            // COMPARISON OPERATIONS
            // ================================================================
            Instruction::Eq => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(if a == b { 1 } else { 0 });
                self.ip += 1;
            }

            Instruction::Lt => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(if a < b { 1 } else { 0 });
                self.ip += 1;
            }

            Instruction::Gt => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(if a > b { 1 } else { 0 });
                self.ip += 1;
            }

            // ================================================================
            // CONTROL FLOW
            // ================================================================
            Instruction::Jmp(target) => {
                if target >= self.code.len() {
                    return Err(VmError::InvalidJump(target));
                }
                self.ip = target;
            }

            Instruction::JmpIf(target) => {
                let condition = self.pop()?;
                if condition != 0 {
                    if target >= self.code.len() {
                        return Err(VmError::InvalidJump(target));
                    }
                    self.ip = target;
                } else {
                    self.ip += 1;
                }
            }

            Instruction::Call(target) => {
                if target >= self.code.len() {
                    return Err(VmError::InvalidJump(target));
                }
                self.call_stack.push(self.ip + 1);
                self.ip = target;
            }

            Instruction::Ret => {
                let return_addr = self.call_stack.pop()
                    .ok_or(VmError::CallStackUnderflow)?;
                self.ip = return_addr;
            }

            // ================================================================
            // I/O OPERATIONS
            // ================================================================
            Instruction::Print => {
                let value = self.pop()?;
                self.output.push(value);
                self.ip += 1;
            }

            Instruction::Halt => {
                self.halted = true;
            }
        }

        Ok(())
    }

    // ====================================================================
    // ACCESSOR METHODS
    // ====================================================================

    /// Returns the values produced by Print instructions, in order.
    pub fn output(&self) -> &[i64] {
        &self.output
    }

    /// Returns the current operand stack contents.
    pub fn stack(&self) -> &[i64] {
        &self.stack
    }

    /// Returns the current instruction pointer.
    pub fn ip(&self) -> usize {
        self.ip
    }

    /// Returns whether the VM has halted.
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    // ====================================================================
    // HELPER METHODS
    // ====================================================================

    fn pop(&mut self) -> VmResult<i64> {
        self.stack.pop().ok_or(VmError::StackUnderflow)
    }

    fn peek(&self) -> VmResult<i64> {
        self.stack.last().copied().ok_or(VmError::StackUnderflow)
    }
}
