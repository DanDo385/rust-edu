//! # A Basic Stack-Based Virtual Machine - Interactive Demo
//! 
//! This binary demonstrates the `VM` from our library by executing
//! a few sample programs.
//! Run with: cargo run -p basic-vm

use basic_vm::solution::{Instruction, VM, VmError};

fn main() {
    println!("=== Basic Stack-Based Virtual Machine Demo ===\n");

    // ============================================================================
    // DEMO 1: Simple Arithmetic: (10 + 20) * 2
    // ============================================================================
    println!("1. Program 1: (10 + 20) * 2");
    println!("   --------------------------");
    let program1 = vec![
        Instruction::Push(10),
        Instruction::Push(20),
        Instruction::Add,
        Instruction::Push(2),
        Instruction::Mul,
        Instruction::Halt,
    ];

    println!("   Program: Push(10), Push(20), Add, Push(2), Mul, Halt");
    run_and_print(&program1);

    // ============================================================================
    // DEMO 2: Conditional Logic: if (10 > 5) then 99 else -1
    // ============================================================================
    println!("2. Program 2: if (10 > 5) then 99 else -1");
    println!("   ---------------------------------------");
    let program2 = vec![
        Instruction::Push(10),
        Instruction::Push(5),
        Instruction::Gt,      // Stack: [1 (true)]
        Instruction::JmpIf(5),// Jump to Push(99) if true
        Instruction::Push(-1),// This is the "else" block
        Instruction::Jmp(6),  // Jump past the "then" block
        Instruction::Push(99),// This is the "then" block
        Instruction::Halt,
    ];

    println!("   Program uses conditional jumps (Gt, JmpIf, Jmp)");
    run_and_print(&program2);
    
    // ============================================================================
    // DEMO 3: Loop: Sum numbers from 5 down to 1
    // ============================================================================
    println!("3. Program 3: Sum numbers from 5 down to 1");
    println!("   ---------------------------------------");
    // Pseudo-code:
    // sum = 0
    // n = 5
    // loop {
    //   if n == 0 break
    //   sum = sum + n
    //   n = n - 1
    // }
    let program3 = vec![
        Instruction::Push(0),    // Initialize sum = 0; Stack: [sum]
        Instruction::Push(5),    // Initialize n = 5;   Stack: [sum, n]
        // Loop start (address 2)
        Instruction::Dup,        // Duplicate n;          Stack: [sum, n, n]
        Instruction::Push(0),    // Push 0 for comparison; Stack: [sum, n, n, 0]
        Instruction::Eq,         // n == 0?;              Stack: [sum, n, (1 or 0)]
        Instruction::JmpIf(12),  // If true, jump to Halt
        // Loop body
        Instruction::Over,       // Copy sum to top;      Stack: [sum, n, sum]
        Instruction::Add,        // Add n to sum;         Stack: [sum, new_sum]
        Instruction::Swap,       // Swap;                 Stack: [new_sum, sum]
        Instruction::Pop,        // Pop old sum;          Stack: [new_sum]
        Instruction::Push(1),    // Push 1 for decrement
        Instruction::Sub,        // n = n - 1;            Stack: [sum, n-1]
        Instruction::Jmp(2),     // Jump to loop start
        // Halt (address 12)
        Instruction::Halt,
    ];
    
    println!("   Program uses a loop with Dup, Over, Swap, Pop, and Jmp");
    run_and_print(&program3);


    // ============================================================================
    // DEMO 4: Stack Underflow Error
    // ============================================================================
    println!("4. Program 4: Stack Underflow Error");
    println!("   -------------------------------");
    let program4 = vec![
        Instruction::Push(10),
        Instruction::Add, // Tries to pop 2 values, but only 1 is on the stack
        Instruction::Halt,
    ];

    println!("   Program: Push(10), Add");
    run_and_print(&program4);

    println!("=== Demo Complete! ===");
}

/// Helper function to run a VM and print its result.
fn run_and_print(program: &[Instruction]) {
    let mut vm = VM::new(program.to_vec());
    match vm.run() {
        Ok(Some(result)) => {
            println!("   ✅ Success! Final result: {}", result);
        }
        Ok(None) => {
            println!("   ✅ Success! Program halted with an empty stack.");
        }
        Err(e) => {
            let error_msg = match e {
                VmError::StackUnderflow => "Stack Underflow",
                VmError::DivisionByZero => "Division by Zero",
                VmError::InvalidInstructionPointer => "Invalid Instruction Pointer",
            };
            println!("   ❌ Error: {}", error_msg);
        }
    }
    println!();
}