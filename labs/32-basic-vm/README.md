# Project 32 - A Basic Stack-Based Virtual Machine

## What You're Building (Plain English)

You're building a simple, simulated computer! This "virtual machine" (VM) won't run on real hardware but will execute a custom "bytecode" language that you define. It will be a "stack machine," which means it performs all its calculations on a stack of values.

Think of it like an old RPN (Reverse Polish Notation) calculator. To add 2 and 3, you would:
1.  `Push 2` onto the stack.
2.  `Push 3` onto the stack.
3.  Execute `Add`. The VM pops 2 and 3, adds them, and pushes the result `5` back onto the stack.

You will define a set of instructions (an "instruction set"), write a program in that instruction set, and then build a VM that can execute it.

## New Rust Concepts in This Project

-   **Enums for Opcodes**: You'll define your VM's instruction set using a Rust `enum`. Each variant will represent an operation (e.g., `Push`, `Add`, `Halt`).

-   **Pattern Matching with `match`**: The heart of your VM's execution loop will be a `match` statement that dispatches on the current instruction and executes the corresponding logic.

-   **Vectors as Stacks**: You'll use a `Vec<i32>` as the VM's data stack, using `push` to add items and `pop` to remove them.

-   **Program Counter**: You'll manage an "instruction pointer" or "program counter" (a `usize` variable) that keeps track of which instruction to execute next.

-   **Error Handling**: You'll implement robust error handling for runtime errors like stack underflow (popping from an empty stack) or division by zero.

## Rust Syntax You'll See

```rust
// The set of all possible instructions for our VM
pub enum Instruction {
    Push(i32),
    Add,
    Sub,
    Mul,
    Div,
    Halt, // Stop execution
}

// The Virtual Machine itself
pub struct VM {
    program: Vec<Instruction>,
    stack: Vec<i32>,
    ip: usize, // Instruction Pointer
}

// The main execution loop
// loop {
//     let instruction = &self.program[self.ip];
//     self.ip += 1;
//
//     match instruction {
//         Instruction::Push(value) => self.stack.push(*value),
//         Instruction::Add => {
//             let b = self.stack.pop().unwrap();
//             let a = self.stack.pop().unwrap();
//             self.stack.push(a + b);
//         },
//         Instruction::Halt => break,
//         // ... other instructions
//     }
// }
```

## How to Run

```bash
# Run the main binary (executes a sample program on your VM)
cargo run -p basic-vm

# Run the tests (checks your VM's correctness)
cargo test -p basic-vm

# Check if code compiles without running
cargo check -p basic-vm
```

## The Exercises

You will implement the `VM` and its instruction set.

1.  **`Instruction` Enum**: Define the opcodes for your VM. Start with:
    -   `Push(i32)`: Pushes a constant value onto the stack.
    -   `Add`, `Sub`, `Mul`, `Div`: Pops two values, performs the operation, and pushes the result.
    -   `Halt`: Stops the VM.

2.  **`VM` Struct**: Create the struct to hold the VM's state:
    -   `program`: A `Vec<Instruction>` containing the bytecode to execute.
    -   `stack`: A `Vec<i32>` for the operand stack.
    -   `ip`: A `usize` for the instruction pointer.

3.  **`new()`**: A constructor that takes a program and creates a new VM instance, ready to run.

4.  **`run()`**: The main execution loop.
    -   It should loop, fetching and executing instructions one by one, advancing the `ip`.
    -   Use a `match` statement to handle each `Instruction` variant.
    -   Implement error handling: if an operation would cause a stack underflow or division by zero, the `run` method should stop and return an `Err`.
    -   The loop terminates when it encounters a `Halt` instruction or an error.

5.  **Stack Manipulation Instructions (Stretch Goal)**: Add more classic stack opcodes:
    -   `Pop`: Discards the top value.
    -   `Dup`: Duplicates the top value (`[a]` -> `[a, a]`).
    -   `Swap`: Swaps the top two values (`[a, b]` -> `[b, a]`).

6.  **Control Flow (Stretch Goal)**: Implement instructions for jumping:
    -   `Jmp(addr)`: Unconditionally sets the `ip` to `addr`.
    -   `JmpIf(addr)`: Pops a value; if it's non-zero, sets `ip` to `addr`.

## Solution Explanation (No Code - Just Ideas)

**The Execution Cycle (Fetch-Decode-Execute)**:
1.  **Fetch**: Get the instruction at the current `ip`. Increment `ip`.
2.  **Decode**: The `match` statement is our decode step. It figures out what the instruction means.
3.  **Execute**: The code inside each `match` arm is the execution step. It manipulates the stack or the `ip`.

**Example: `(5 + 3) * 2`**
This would be written in our bytecode as:
`Push(5)`
`Push(3)`
`Add`
`Push(2)`
`Mul`
`Halt`

Let's trace the stack:
-   `Push(5)` -> `[5]`
-   `Push(3)` -> `[5, 3]`
-   `Add` -> pops 3, pops 5, pushes 8 -> `[8]`
-   `Push(2)` -> `[8, 2]`
-   `Mul` -> pops 2, pops 8, pushes 16 -> `[16]`
-   `Halt` -> Stop. The final result is the value left on the stack.

## Where Rust Shines

-   **`enum` and `match`**: Rust's powerful enums and pattern matching are a perfect fit for defining instruction sets and building interpreters/VMs.
-   **`Result` for Error Handling**: The `run` loop can return a `Result`, cleanly propagating runtime errors without exceptions or error codes.
-   **Safety**: Even though we're building a "low-level" VM, Rust's safety guarantees (like bounds checking on the `program` vector) prevent many common bugs found in C/C++ VM implementations.
-   **`Vec` as a Stack**: The `Vec` type provides an efficient, safe, and easy-to-use implementation of a stack.

## Common Beginner Mistakes

1.  **Stack Underflow**: Calling `pop()` on an empty stack will panic.
    -   **Fix**: Before popping, always check if the stack has enough operands. For `Add`, you need at least 2. If not, return a `StackUnderflow` error.

2.  **Off-by-One `ip` Errors**: Forgetting to increment the instruction pointer, or incrementing it at the wrong time, can lead to infinite loops or skipped instructions.
    -   **Fix**: A common pattern is to fetch the instruction and immediately increment the `ip` *before* executing the instruction.

3.  **Ownership in the `program` Vector**: The `VM` should own its program. A common way to do this is to have the `new()` constructor take ownership of the `Vec<Instruction>`.

4.  **Mutable State**: The `stack` and `ip` are mutable state. The `run` method will need to take `&mut self`.

This project is a fantastic introduction to how programming languages are actually executed under the hood. Good luck! 🦀