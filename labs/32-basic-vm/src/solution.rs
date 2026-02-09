//! # A Basic Stack-Based Virtual Machine - Complete Solution
//!
//! ## What We're Building
//!
//! A `VM` that executes a simple, custom bytecode. This is a "stack machine,"
//! meaning it uses a stack to hold temporary values for computations. The VM
//! follows a classic "fetch-decode-execute" cycle.
//!
//! ## Why Rust Is Perfect For This
//!
//! - **`enum` and `match`**: Rust's enums are perfect for defining an instruction
//!   set (opcodes), and `match` provides a clean, exhaustive way to decode and
//!   execute them.
//! - **`Result` and Error Handling**: A VM can have runtime errors (like stack
//!   underflow). Rust's `Result` type allows us to handle these gracefully
//!   without resorting to exceptions or panics.
//! - **`Vec` as a Stack**: `Vec` provides a robust and efficient implementation
//!   of a stack right out of the box, with `push` and `pop`.
//!
//! ## Key Rust Concepts You'll Learn
//!
//! - **Enums as Opcodes**: Defining a state machine or instruction set.
//! - **Pattern Matching**: The core of the execution loop.
//! - **State Management**: Managing the VM's internal state (`stack`, `ip`).
//! - **Custom Error Types**: Creating a dedicated error enum for VM-specific issues.

/// The instruction set for our Virtual Machine.
///
/// Each variant represents a unique operation (opcode).
#[derive(Debug, Clone)]
pub enum Instruction {
    // --- Basic Arithmetic ---
    /// Push a constant value onto the stack.
    Push(i32),
    /// Pop two values, add them, push the result.
    Add,
    /// Pop two values, subtract the top from the second-to-top, push the result.
    Sub,
    /// Pop two values, multiply them, push the result.
    Mul,
    /// Pop two values, divide the second-to-top by the top, push the result.
    Div,

    // --- Stack Manipulation ---
    /// Pop and discard the top value of the stack.
    Pop,
    /// Duplicate the top value of the stack.
    Dup,
    /// Swap the top two values of the stack.
    Swap,
    /// Copy the second value from the top and push it onto the stack.
    Over,

    // --- Comparison ---
    /// Pop two values, push 1 if they are equal, else 0.
    Eq,
    /// Pop two values, push 1 if the second-to-top is greater than the top, else 0.
    Gt,
    /// Pop two values, push 1 if the second-to-top is less than the top, else 0.
    Lt,

    // --- Control Flow ---
    /// Unconditionally jump to the given address (instruction index).
    Jmp(usize),
    /// Pop a value; if it is non-zero, jump to the given address.
    JmpIf(usize),

    // --- Halting ---
    /// Stop program execution.
    Halt,
}

/// Represents all possible runtime errors the VM can encounter.
#[derive(Debug, PartialEq)]
pub enum VmError {
    /// Tried to pop a value from an empty stack.
    StackUnderflow,
    /// Attempted to divide by zero.
    DivisionByZero,
    /// The instruction pointer went out of the program's bounds.
    InvalidInstructionPointer,
}

/// A simple stack-based Virtual Machine.
pub struct VM {
    /// The program (bytecode) to be executed.
    program: Vec<Instruction>,
    /// The data stack for operations.
    stack: Vec<i32>,
    /// The instruction pointer, indicating the index of the next instruction.
    ip: usize,
}

impl VM {
    /// Creates a new VM with a given program.
    pub fn new(program: Vec<Instruction>) -> Self {
        VM {
            program,
            stack: Vec::new(),
            ip: 0,
        }
    }

    /// Runs the VM until it halts or an error occurs.
    ///
    /// The main "fetch-decode-execute" loop happens here.
    pub fn run(&mut self) -> Result<Option<i32>, VmError> {
        while self.ip < self.program.len() {
            // Fetch the instruction. We clone it to avoid borrowing issues with `self`.
            let instruction = self.program[self.ip].clone();
            // Immediately increment the IP for the next cycle.
            self.ip += 1;

            // Decode and Execute the instruction.
            match instruction {
                Instruction::Push(value) => {
                    self.stack.push(value);
                }
                Instruction::Add => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(a + b);
                }
                Instruction::Sub => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(a - b);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(a * b);
                }
                Instruction::Div => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if b == 0 {
                        return Err(VmError::DivisionByZero);
                    }
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(a / b);
                }
                Instruction::Pop => {
                    self.stack.pop().ok_or(VmError::StackUnderflow)?;
                }
                Instruction::Dup => {
                    let val = self.stack.last().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(*val);
                }
                Instruction::Swap => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(b);
                    self.stack.push(a);
                }
                Instruction::Over => {
                    let b = self.stack.get(self.stack.len() - 2).ok_or(VmError::StackUnderflow)?;
                    self.stack.push(*b);
                }
                Instruction::Eq => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(if a == b { 1 } else { 0 });
                }
                Instruction::Gt => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(if a > b { 1 } else { 0 });
                }
                Instruction::Lt => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(if a < b { 1 } else { 0 });
                }
                Instruction::Jmp(addr) => {
                    if addr >= self.program.len() {
                        return Err(VmError::InvalidInstructionPointer);
                    }
                    self.ip = addr;
                }
                Instruction::JmpIf(addr) => {
                    if addr >= self.program.len() {
                        return Err(VmError::InvalidInstructionPointer);
                    }
                    let cond = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if cond != 0 {
                        self.ip = addr;
                    }
                }
                Instruction::Halt => {
                    // Break the loop to stop execution.
                    break;
                }
            }
        }

        // After the loop (due to Halt or end of program), return the top of the stack.
        Ok(self.stack.pop())
    }
}
