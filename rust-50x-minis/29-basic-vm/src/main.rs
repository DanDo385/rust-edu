// Project 29: Basic VM (Virtual Machine)
//
// A stack-based virtual machine that executes custom bytecode.
// Demonstrates how interpreters work, instruction dispatch, and VM architecture.
// This is the foundation of JVM, Python, WebAssembly, and other interpreted languages.

use std::fmt;

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
    vm.run(true).unwrap();
    println!();
}

fn demo_factorial() {
    println!("=== Factorial Calculation ===\n");

    // Program: factorial(5) using a loop
    // Pseudo-code:
    //   n = 5
    //   acc = 1
    //   while n > 1:
    //     acc = acc * n
    //     n = n - 1
    //   print acc

    let program = vec![
        Instruction::Push(5),      // n
        Instruction::Push(1),      // acc
        // LOOP (ip = 2):
        Instruction::Swap,         // swap n and acc for duplicate
        Instruction::Dup,          // duplicate n
        Instruction::Push(1),
        Instruction::Gt,           // n > 1?
        Instruction::JmpIf(12),    // if true, continue; else jump to END
        // BODY:
        Instruction::Swap,         // get acc on top
        Instruction::Over,         // copy n over acc
        Instruction::Mul,          // acc = acc * n
        Instruction::Swap,         // swap to get n on top
        Instruction::Push(1),
        Instruction::Sub,          // n = n - 1
        Instruction::Jmp(2),       // jump back to LOOP
        // END (ip = 12):
        Instruction::Pop,          // remove n (now 1)
        Instruction::Print,        // print acc
        Instruction::Halt,
    ];

    println!("Program: factorial(5)");
    let mut vm = VirtualMachine::new(program);
    vm.run(false).unwrap();  // Disable trace for cleaner output
    println!();
}

fn demo_function_calls() {
    println!("=== Function Calls ===\n");

    // Program: call a function that adds two numbers
    // main:
    //   push 10
    //   push 20
    //   call add_function
    //   print
    //   halt
    // add_function:
    //   add
    //   ret

    let program = vec![
        // MAIN (ip = 0-5):
        Instruction::Push(10),
        Instruction::Push(20),
        Instruction::Call(6),      // call add_function at ip=6
        Instruction::Print,
        Instruction::Halt,
        Instruction::Halt,         // padding
        // ADD_FUNCTION (ip = 6-7):
        Instruction::Add,
        Instruction::Ret,
    ];

    println!("Program: function call to add two numbers");
    let mut vm = VirtualMachine::new(program);
    vm.run(true).unwrap();
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
    vm.run(true).unwrap();
    println!();
}

// ============================================================================
// INSTRUCTION SET
// ============================================================================
// Each instruction is an enum variant that the VM can execute

#[derive(Debug, Clone)]
enum Instruction {
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
    Print,               // Pop and print top of stack
    Halt,                // Stop execution
}

// ============================================================================
// VM ERROR TYPES
// ============================================================================

#[derive(Debug)]
enum VmError {
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

type VmResult<T> = Result<T, VmError>;

// ============================================================================
// VIRTUAL MACHINE STRUCTURE
// ============================================================================

struct VirtualMachine {
    code: Vec<Instruction>,    // Bytecode instructions
    stack: Vec<i64>,           // Operand stack
    call_stack: Vec<usize>,    // Return addresses for function calls
    ip: usize,                 // Instruction pointer (program counter)
    halted: bool,              // Halt flag
}

impl VirtualMachine {
    /// Creates a new VM with the given bytecode
    fn new(code: Vec<Instruction>) -> Self {
        VirtualMachine {
            code,
            stack: Vec::new(),
            call_stack: Vec::new(),
            ip: 0,
            halted: false,
        }
    }

    /// Runs the VM until halt or error
    fn run(&mut self, trace: bool) -> VmResult<()> {
        while !self.halted {
            self.step(trace)?;
        }
        Ok(())
    }

    /// Executes a single instruction
    fn step(&mut self, trace: bool) -> VmResult<()> {
        // Bounds check
        if self.ip >= self.code.len() {
            return Err(VmError::InvalidJump(self.ip));
        }

        // Fetch instruction
        let instruction = self.code[self.ip].clone();

        if trace {
            println!("IP: {:3} | Instr: {:?} | Stack: {:?}",
                     self.ip, instruction, self.stack);
        }

        // Execute instruction
        self.execute(instruction)?;

        Ok(())
    }

    /// Executes a single instruction (decode + execute)
    fn execute(&mut self, instruction: Instruction) -> VmResult<()> {
        match instruction {
            // ====================================================================
            // STACK OPERATIONS
            // ====================================================================
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
                // Copy second item to top
                if self.stack.len() < 2 {
                    return Err(VmError::StackUnderflow);
                }
                let value = self.stack[self.stack.len() - 2];
                self.stack.push(value);
                self.ip += 1;
            }

            // ====================================================================
            // ARITHMETIC OPERATIONS
            // ====================================================================
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

            // ====================================================================
            // COMPARISON OPERATIONS
            // ====================================================================
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

            // ====================================================================
            // CONTROL FLOW
            // ====================================================================
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
                // Push return address (next instruction)
                self.call_stack.push(self.ip + 1);
                self.ip = target;
            }

            Instruction::Ret => {
                let return_addr = self.call_stack.pop()
                    .ok_or(VmError::CallStackUnderflow)?;
                self.ip = return_addr;
            }

            // ====================================================================
            // I/O OPERATIONS
            // ====================================================================
            Instruction::Print => {
                let value = self.pop()?;
                println!("Output: {}", value);
                self.ip += 1;
            }

            Instruction::Halt => {
                self.halted = true;
            }
        }

        Ok(())
    }

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    fn pop(&mut self) -> VmResult<i64> {
        self.stack.pop().ok_or(VmError::StackUnderflow)
    }

    fn peek(&self) -> VmResult<i64> {
        self.stack.last().copied().ok_or(VmError::StackUnderflow)
    }
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

// ============================================================================
// STACK-BASED VS REGISTER-BASED
// ============================================================================
//
// STACK-BASED (our VM, JVM, Python):
//   PUSH 5
//   PUSH 3
//   ADD          # Stack: [8]
//   PUSH 2
//   MUL          # Stack: [16]
//
//   Pros: Compact bytecode, simple to generate
//   Cons: More instructions (push/pop overhead)
//
// REGISTER-BASED (Lua, Dalvik):
//   LOAD r1, 5
//   LOAD r2, 3
//   ADD r3, r1, r2   # r3 = r1 + r2
//   LOAD r4, 2
//   MUL r5, r3, r4   # r5 = r3 * r4
//
//   Pros: Fewer instructions, less memory traffic
//   Cons: Larger bytecode, more complex compiler
//
// In practice, JIT compilers make both equally fast!

// ============================================================================
// HOW REAL VMs OPTIMIZE
// ============================================================================
//
// 1. JIT COMPILATION
//    - Detect hot code (frequently executed)
//    - Compile bytecode to native machine code
//    - 10-100x speedup over interpretation
//    - Used by: JVM (HotSpot), V8 (JavaScript), PyPy
//
// 2. INLINE CACHING
//    - Cache type information at call sites
//    - Avoid type checks on every operation
//    - Critical for dynamic languages (Python, JavaScript)
//
// 3. THREADED CODE
//    - Replace switch/match with array of function pointers
//    - Each instruction is a function
//    - Better branch prediction, less overhead
//
// 4. STACK CACHING
//    - Keep top N stack values in CPU registers
//    - Reduce memory loads/stores
//    - Significant speedup (2-3x)
//
// 5. SUPERINSTRUCTIONS
//    - Combine frequent instruction sequences
//    - e.g., PUSH_ADD (push + add in one instruction)
//    - Reduces instruction dispatch overhead
//
// 6. GARBAGE COLLECTION
//    - Generational GC (young generation, old generation)
//    - Concurrent GC (GC runs in parallel with execution)
//    - Incremental GC (small pauses)

// ============================================================================
// PERFORMANCE CHARACTERISTICS
// ============================================================================
//
// OUR VM (interpreted):
// - ~20-50 ns per instruction
// - No JIT, no optimization
// - Educational, not production-ready
//
// CPYTHON (interpreted):
// - ~50-100 ns per bytecode
// - Similar to our VM
// - No JIT (but PyPy has JIT)
//
// JVM HOTSPOT (JIT):
// - Starts interpreted (~20 ns/instr)
// - After warm-up, JIT compiles to native
// - Hot code: ~1-5 ns per operation (near C speed!)
//
// WEBASSEMBLY (JIT):
// - Compiled to native on load
// - ~1 ns per operation (native speed)
// - Used in browsers, very fast
//
// Lesson: JIT compilation is essential for production VMs!

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting stack underflow checks
//    let a = self.stack.pop().unwrap();  // PANIC!
//    Fix: Return Err(StackUnderflow) instead
//
// ❌ Not incrementing IP
//    Instruction executes but IP doesn't advance
//    Fix: Always increment IP (except for jumps)
//
// ❌ Invalid jump targets
//    JMP to offset beyond code length
//    Fix: Bounds check all jump targets
//
// ❌ Division by zero
//    CPU exception, program crashes
//    Fix: Check denominator before dividing
//
// ❌ Infinite loops without halt
//    JMP to self or loop without exit condition
//    Fix: Add instruction count limit or timeout

// ============================================================================
// EXTENDING THIS VM
// ============================================================================
//
// NEXT STEPS:
//
// 1. ADD TYPES
//    - String, Float, Boolean, Null
//    - Tagged unions or separate stacks
//
// 2. ADD HEAP
//    - Allocate objects/arrays
//    - Reference by pointer/index
//
// 3. GARBAGE COLLECTION
//    - Mark-and-sweep or reference counting
//    - Reclaim unused heap objects
//
// 4. FUNCTION LOCALS
//    - Stack frames for local variables
//    - Frame pointer + base pointer
//
// 5. EXCEPTION HANDLING
//    - try/catch mechanism
//    - Exception handler stack
//
// 6. JIT COMPILER
//    - Use cranelift or LLVM
//    - Compile hot code to native
//
// 7. ASSEMBLER/DISASSEMBLER
//    - Parse text assembly to bytecode
//    - Print bytecode as readable assembly
//
// 8. DEBUGGER
//    - Breakpoints, step execution
//    - Stack inspection, variable watching
