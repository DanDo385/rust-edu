# Project 29: Basic VM

## Overview
This project implements a stack-based virtual machine that executes custom bytecode. You'll learn how programming languages are executed, how interpreters work, and the fundamentals of CPU architecture. This is the foundation of the JVM, Python interpreter, and WebAssembly.

## Concepts Taught
- **Stack-based architecture** vs register-based
- **Bytecode interpretation** and instruction dispatch
- **Instruction set design** (opcodes)
- **Stack operations** (push, pop, arithmetic)
- **Control flow** (jumps, branches)
- **Memory model** (stack + heap simulation)
- **Pattern matching** for instruction execution
- **Error handling** for runtime errors

## Why Stack-Based VMs?

### Stack-Based vs Register-Based
There are two main VM architectures:

**Stack-based** (JVM, Python, our VM):
- Operations work on implicit stack
- `PUSH 5`, `PUSH 3`, `ADD` → 5 + 3
- Compact bytecode (fewer operands)
- Simpler to generate from compiler
- More instructions executed (push/pop overhead)

**Register-based** (Lua, Android's Dalvik):
- Operations work on explicit registers
- `ADD r1, r2, r3` → r1 = r2 + r3
- Larger bytecode (more operands)
- Fewer instructions (less overhead)
- More complex compiler

**Our choice**: Stack-based for simplicity and educational value.

**Real-world VMs:**
- **JVM**: Stack-based, runs Java/Kotlin/Scala
- **CPython**: Stack-based, runs Python
- **WebAssembly**: Stack-based, runs in browsers
- **Lua**: Register-based (5 registers)
- **BEAM**: Register-based, runs Erlang/Elixir

## Instruction Set Architecture

Our VM supports:

### Stack Operations
- `PUSH <value>`: Push value onto stack
- `POP`: Discard top of stack

### Arithmetic
- `ADD`: Pop two values, push sum
- `SUB`: Pop two values, push difference
- `MUL`: Pop two values, push product
- `DIV`: Pop two values, push quotient

### Comparison
- `EQ`: Pop two values, push 1 if equal, 0 otherwise
- `LT`: Pop two values, push 1 if less than, 0 otherwise
- `GT`: Pop two values, push 1 if greater than, 0 otherwise

### Control Flow
- `JMP <offset>`: Jump to instruction
- `JMPIF <offset>`: Jump if top of stack is non-zero
- `CALL <offset>`: Call function (push return address)
- `RET`: Return from function

### I/O
- `PRINT`: Pop and print value
- `HALT`: Stop execution

## VM Architecture

```
VirtualMachine
├── stack: Vec<i64>        # Operand stack
├── ip: usize              # Instruction pointer
├── call_stack: Vec<usize> # Return addresses
└── code: Vec<Instruction> # Bytecode

Execution Loop:
1. Fetch instruction at IP
2. Decode instruction (pattern match)
3. Execute instruction
4. Update IP
5. Repeat until HALT
```

## Example Program

Calculate factorial(5):

```
Assembly         Stack State
---------------  -------------
PUSH 5           [5]
PUSH 1           [5, 1]        # accumulator
LOOP:
  DUP            [5, 1, 1]
  PUSH 2         [5, 1, 1, 2]
  LT             [5, 1, 0]      # 1 < 2? no
  JMPIF END      [5, 1]
  SWAP           [1, 5]
  DUP            [1, 5, 5]
  ROT            [5, 5, 1]
  MUL            [5, 5]
  SWAP           [5, 5]
  PUSH 1         [5, 5, 1]
  SUB            [5, 4]
  SWAP           [4, 5]
  JMP LOOP
END:
  POP            [120]
  PRINT          []
  HALT
```

## Beginner Pitfalls & VM Notes

### Pitfall 1: Stack Underflow
```rust
// Stack: []
let a = self.stack.pop().unwrap();  // ❌ Panic!

// Fix: Check stack size
if self.stack.is_empty() {
    return Err(VmError::StackUnderflow);
}
```

### Pitfall 2: Instruction Pointer Out of Bounds
```rust
// Jump beyond code length
self.ip = 1000;  // ❌ Will panic on next fetch

// Fix: Bounds checking
if offset >= self.code.len() {
    return Err(VmError::InvalidJump);
}
```

### Pitfall 3: Integer Division by Zero
```rust
let b = self.stack.pop().unwrap();
let a = self.stack.pop().unwrap();
let result = a / b;  // ❌ Panic if b == 0

// Fix: Check for zero
if b == 0 {
    return Err(VmError::DivisionByZero);
}
```

### Pitfall 4: Infinite Loops
```rust
// JMP to self creates infinite loop
Instruction::Jmp(self.ip)  // ❌ Hangs forever

// Fix: Add instruction count limit or timeout
```

## Code Walkthrough

See `src/main.rs` for a detailed implementation that demonstrates:
1. Defining an instruction set with enums
2. Implementing a stack-based VM
3. Fetch-decode-execute loop
4. Example programs (arithmetic, loops, functions)
5. Error handling for runtime errors
6. Debugging output to trace execution

## Performance Considerations

### Bytecode Interpretation Speed
- **Rust pattern matching**: ~5-10 CPU cycles per instruction
- **JVM (HotSpot)**: Uses JIT compilation after warm-up
- **CPython**: ~50-100 ns per bytecode
- **WebAssembly**: Near-native speed with JIT

**Our VM**: Interpreted, ~20-50ns per instruction on modern CPU

### Optimization Techniques
1. **Threaded code**: Replace switch/match with function pointers
2. **JIT compilation**: Compile hot bytecode to native code
3. **Inline caching**: Cache type information
4. **Superinstructions**: Combine common sequences (PUSH + ADD)
5. **Stack caching**: Keep top of stack in registers

### Memory Usage
- **Stack**: 8 bytes per value (i64)
- **Instructions**: ~16 bytes per instruction (enum + data)
- **Small programs**: <1 KB
- **Large programs**: Megabytes (similar to real VMs)

## Comparison: Rust vs Other VMs

| Feature | Our VM | JVM | CPython | WebAssembly |
|---------|--------|-----|---------|-------------|
| Architecture | Stack | Stack | Stack | Stack |
| Bytecode format | Enum | Binary | Binary | Binary |
| Type system | Untyped | Typed | Dynamic | Typed |
| JIT compilation | No | Yes (HotSpot) | No | Yes (V8, SpiderMonkey) |
| Garbage collection | No | Yes | Yes (ref counting) | No (linear memory) |
| Performance | ~50ns/instr | ~1-5ns/instr (JIT) | ~50-100ns/instr | ~1ns/instr (JIT) |

**Rust advantage**: Zero-cost VM implementation, no GC overhead.

## Additional Challenges

1. **Add More Types**: Support strings, floats, booleans

2. **Function Calls**: Implement local variables and parameter passing

3. **Garbage Collection**: Add heap allocation with GC

4. **Debugging**: Add breakpoints, step execution, stack inspection

5. **JIT Compiler**: Compile hot code to native using cranelift

6. **Assembler**: Write a parser to convert text assembly to bytecode

## Real-World VM Features

Production VMs add:
- **Type system**: Static or dynamic typing
- **Garbage collection**: Mark-and-sweep, generational, reference counting
- **JIT compilation**: Compile to native code at runtime
- **Exception handling**: try/catch mechanisms
- **Module system**: Import/export, linking
- **FFI**: Call native code (C, Rust)
- **Concurrency**: Threads, async/await, actors
- **Debugging**: Source maps, profiling, breakpoints

## Future Directions

- **Next**: Message bus (Project 30)
- **Related**: Interpreter (Project 33), macros (Project 43)
- **Advanced**: Build JIT compiler with cranelift, add GC with gc-arena

## Running This Project

```bash
cd 29-basic-vm
cargo run
```

## Expected Output

You should see:
- VM initialization with bytecode
- Step-by-step execution trace (instruction + stack state)
- Results of arithmetic operations
- Factorial calculation result
- Function call demonstration
- Final stack state
