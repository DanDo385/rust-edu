//! # A Basic Stack-Based Virtual Machine - Your Implementation
//!
//! This project involves creating a simple virtual machine that executes a
//! custom bytecode format.
//!
//! ## Your Task
//!
//! Implement the `Instruction` enum and the `VM` struct and its methods.
//!
//! 1.  **`Instruction` Enum**: Define the set of operations your VM can perform.
//!     Start with `Push`, `Add`, `Sub`, `Mul`, `Div`, and `Halt`.
//!
//! 2.  **`VM` Struct**: This will hold the VM's state, including the program
//!     (bytecode), the data stack, and the instruction pointer (`ip`).
//!
//! 3.  **`new()`**: A constructor to create a new VM instance with a given program.
//!
//! 4.  **`run()`**: The main execution loop. This is where you will fetch, decode,
//!     and execute instructions. You must handle potential runtime errors like
//!     `StackUnderflow` and `DivisionByZero`.
//!
//! ## Stretch Goals
//!
//! -   Add more stack manipulation instructions like `Pop`, `Dup`, `Swap`, `Over`.
//! -   Add control flow instructions like `Jmp` (unconditional jump) and
//!     `JmpIf` (conditional jump).
//!
//! ## Running Your Code
//!
//! ```bash
//! cargo test -p basic-vm
//! cargo run -p basic-vm
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

// TODO: Define the Instruction enum.
// It should represent all possible operations your VM can execute.
// For example: Push a value, Add, Subtract, Halt, etc.
//
// #[derive(Debug, Clone)]
// pub enum Instruction { ... }
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Push(i32),
    Add,
    Sub,
    Mul,
    Div,
    Halt,
    Pop,
    Dup,
    Swap,
    Over,
    Jmp(usize),
    JmpIf(usize),
}

// TODO: Define VmError enum
// This will represent possible runtime errors.
// e.g., StackUnderflow, DivisionByZero, InvalidInstructionPointer
//
// #[derive(Debug, PartialEq)]
// pub enum VmError { ... }
#[derive(Debug, PartialEq)]
pub enum VmError {
    StackUnderflow,
    DivisionByZero,
    InvalidInstructionPointer,
}

// TODO: Define the VM struct
// It should hold the program, the stack, and the instruction pointer.
//
// pub struct VM { ... }
pub struct VM {
    _program: Vec<Instruction>,
    _stack: Vec<i32>,
    _ip: usize,
}


impl VM {
    /// Creates a new VM with a given program.
    pub fn new(program: Vec<Instruction>) -> Self {
        // TODO: Initialize the VM state.
        // - The program should be stored.
        // - The stack should be empty.
        // - The instruction pointer (`ip`) should start at 0.
        let _ = program;
        todo!("Initialize the VM");
    }

    /// Runs the VM until it halts or an error occurs.
    ///
    /// Returns the last value on the stack if successful, or an error.
    pub fn run(&mut self) -> Result<Option<i32>, VmError> {
        // TODO: Implement the main execution loop.
        // loop {
        //   1. Check if `ip` is within the bounds of the program. If not,
        //      return `Err(VmError::InvalidInstructionPointer)`.
        //
        //   2. Fetch the current instruction.
        //
        //   3. Increment the `ip`.
        //
        //   4. Use a `match` statement to execute the instruction.
        //
        //   5. For arithmetic operations:
        //      - Pop the required number of operands from the stack.
        //      - Check for stack underflow! If `pop()` returns `None`,
        //        return `Err(VmError::StackUnderflow)`.
        //      - Perform the operation.
        //      - Check for division by zero!
        //      - Push the result back onto the stack.
        //
        //   6. For `Halt`, break the loop.
        // }
        //
        // After the loop, return the top value of the stack, if any.
        todo!("Implement the VM's execution loop");
    }
}


// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;
