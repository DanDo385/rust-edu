// Lab 32: Basic VM (Virtual Machine) - Demo
//
// A stack-based virtual machine that executes custom bytecode.
// Demonstrates how interpreters work, instruction dispatch, and VM architecture.
// This is the foundation of JVM, Python, WebAssembly, and other interpreted languages.

use basic_vm::{Instruction, VirtualMachine};

fn main() {
    println!("=== Stack-Based Virtual Machine ===\n");

    // ============================================================================
    // SIMPLE ARITHMETIC
    // ============================================================================
    demo_arithmetic();

    // ============================================================================
    // FACTORIAL CALCULATION
    // ============================================================================
    demo_factorial();

    // ============================================================================
    // FUNCTION CALLS
    // ============================================================================
    demo_function_calls();

    // ============================================================================
    // CONDITIONAL EXECUTION
    // ============================================================================
    demo_conditionals();
}

fn demo_arithmetic() {
    println!("=== Arithmetic Operations ===\n");

    // Program: (5 + 3) * 2
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Push(2),
        Instruction::Mul,
        Instruction::Print,
        Instruction::Halt,
    ];

    println!("Program: (5 + 3) * 2");
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    println!("Output: {:?}", vm.output());
    println!();
}

fn demo_factorial() {
    println!("=== Factorial Calculation ===\n");

    // Program: factorial(5) using a loop
    // Uses loop invariant: stack = [n, acc] (acc on top)
    let program = vec![
        Instruction::Push(5),        // 0: n = 5
        Instruction::Push(1),        // 1: acc = 1
        // LOOP (ip = 2): stack = [..., n, acc]
        Instruction::Over,           // 2: [..., n, acc, n]
        Instruction::Push(1),        // 3: [..., n, acc, n, 1]
        Instruction::Eq,             // 4: [..., n, acc, n==1]
        Instruction::JmpIf(13),      // 5: if n==1, jump to END
        // BODY: stack = [..., n, acc]
        Instruction::Over,           // 6: [..., n, acc, n]
        Instruction::Mul,            // 7: [..., n, acc*n]
        Instruction::Swap,           // 8: [..., acc*n, n]
        Instruction::Push(1),        // 9: [..., acc*n, n, 1]
        Instruction::Sub,            // 10: [..., acc*n, n-1]
        Instruction::Swap,           // 11: [..., n-1, acc*n]
        Instruction::Jmp(2),         // 12: back to LOOP
        // END (ip = 13): stack = [..., n, acc] where n==1
        Instruction::Swap,           // 13: [..., acc, n]
        Instruction::Pop,            // 14: [..., acc]
        Instruction::Print,          // 15: print acc = 120
        Instruction::Halt,           // 16
    ];

    println!("Program: factorial(5)");
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    println!("Output: {:?}", vm.output());
    println!();
}

fn demo_function_calls() {
    println!("=== Function Calls ===\n");

    // Program: call a function that adds two numbers
    let program = vec![
        // MAIN (ip = 0-4):
        Instruction::Push(10),
        Instruction::Push(20),
        Instruction::Call(5),      // call add_function at ip=5
        Instruction::Print,
        Instruction::Halt,
        // ADD_FUNCTION (ip = 5-6):
        Instruction::Add,
        Instruction::Ret,
    ];

    println!("Program: function call to add two numbers");
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    println!("Output: {:?}", vm.output());
    println!();
}

fn demo_conditionals() {
    println!("=== Conditional Execution ===\n");

    // Program: if 5 > 3 then print 100 else print 200
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(3),
        Instruction::Gt,           // 5 > 3? pushes 1 (true)
        Instruction::JmpIf(7),     // if true, jump to THEN branch
        // ELSE (ip = 4-6):
        Instruction::Push(200),
        Instruction::Print,
        Instruction::Jmp(9),       // jump past THEN
        // THEN (ip = 7-8):
        Instruction::Push(100),
        Instruction::Print,
        // END (ip = 9):
        Instruction::Halt,
    ];

    println!("Program: if 5 > 3 then print 100 else print 200");
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    println!("Output: {:?}", vm.output());
    println!();
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
//
// 1. INSTRUCTION DISPATCH
//    The match on Instruction compiles to a jump table
//    CPU can predict branches well (modern branch predictors)
//    ~5-10 CPU cycles per instruction
//
// 2. STACK OPERATIONS
//    Vec<i64> grows dynamically on the heap
//    push/pop are O(1) amortized (reallocation when capacity exceeded)
//    Stack values are i64 (8 bytes each)
//
// 3. ENUM SIZE
//    Instruction enum is sized by largest variant
//    Tag (discriminant) + largest data = ~16 bytes per instruction
//    This is larger than real VMs (which use compact binary encoding)
//
// 4. ERROR HANDLING
//    Result<T, VmError> is zero-cost abstraction
//    ? operator compiles to efficient machine code
//    No exceptions - all errors explicit in types
//
// 5. PROGRAM COUNTER (IP)
//    Just a usize (8 bytes on 64-bit)
//    Incremented after each instruction
//    Jumps modify it directly (no pipeline flush in interpreter!)
//
// 6. FUNCTION CALLS
//    call_stack is separate from operand stack
//    Stores return addresses (instruction pointers)
//    Similar to CPU's hardware call stack

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Stack-based VMs are simple to implement
// 2. Fetch-decode-execute loop is the core
// 3. Pattern matching on enums is perfect for instruction dispatch
// 4. Stack operations are the primary way to manipulate data
// 5. Control flow uses jumps (like assembly)
// 6. Function calls need a separate call stack
// 7. Error handling prevents crashes (underflow, division by zero)
// 8. Real VMs add: types, GC, JIT, exceptions, modules
