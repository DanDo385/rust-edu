# Project 30 - Lock-Free Stack

## What You're Building (Plain English)

You're going to build a high-performance, thread-safe Stack without using any traditional locks (like Mutexes). A Stack is a "last-in, first-out" (LIFO) data structure, like a stack of plates. You can `push` a new plate onto the top, or `pop` the top plate off.

The "lock-free" part means multiple threads can read and write to the stack at the same time without ever having to wait for each other to release a lock. This is how high-performance systems (like operating systems, databases, and game engines) handle shared data without performance bottlenecks.

## New Rust Concepts in This Project

-   **Atomics**: Special types (like `AtomicUsize`, `AtomicPtr`) that allow you to read and modify a value from multiple threads without causing data races. The CPU guarantees that operations on these types are "atomic" – they happen all at once, without interruption.

-   **Compare-and-Swap (CAS)**: The workhorse of lock-free programming. It's an atomic operation that says: "change this value from A to B, but *only if* the value is still A". This lets you safely modify data even if other threads are also trying to modify it.

-   **Memory Ordering**: Instructions to the compiler and CPU on how to order reads and writes across threads. This is one of the most complex parts of concurrent programming. We'll use:
    -   `Ordering::Relaxed`: Fastest, but provides no guarantees about ordering.
    -   `Ordering::Acquire`: A read operation that ensures no subsequent reads/writes are moved before it.
    -   `Ordering::Release`: A write operation that ensures no preceding reads/writes are moved after it.
    -   `Ordering::AcqRel`: Combines `Acquire` and `Release` for a read-modify-write operation.

-   **Unsafe Code**: You'll dip your toes into `unsafe` Rust. Lock-free programming often requires dereferencing raw pointers, which Rust considers `unsafe` because the compiler can't guarantee they're valid. We take on the responsibility of ensuring safety.

## Rust Syntax You'll See

```rust
use std::sync::atomic::{AtomicPtr, Ordering};

// A pointer that can be safely shared and modified across threads
let atomic_ptr = AtomicPtr::new(std::ptr::null_mut());

// Atomically load the pointer
let current_ptr = atomic_ptr.load(Ordering::Acquire);

// The core lock-free operation
// Tries to change the pointer from `current_ptr` to `new_ptr`
let result = atomic_ptr.compare_exchange(
    current_ptr,
    new_ptr,
    Ordering::AcqRel,
    Ordering::Acquire,
);

match result {
    Ok(ptr) => { /* Exchange succeeded! */ },
    Err(ptr) => { /* Exchange failed, another thread changed it. Retry! */ },
}

// Creating a raw pointer from a Box
let my_box = Box::new(5);
let raw_ptr = Box::into_raw(my_box);

// Getting a Box back from a raw pointer (unsafe!)
let my_box_again = unsafe { Box::from_raw(raw_ptr) };
```

## How to Run

```bash
# Run the main binary (executes src/main.rs for a demo)
cargo run -p lock-free-structure

# Run the tests (checks your implementation)
cargo test -p lock-free-structure

# Run tests with output visible
cargo test -p lock-free-structure -- --nocapture

# Check if code compiles without running
cargo check -p lock-free-structure

# Format your code
cargo fmt -p lock-free-structure
```

## The Exercises

You'll implement a `LockFreeStack`.

1.  **Node Struct**: Define the `Node` that will hold a value and a pointer to the next node.

2.  **LockFreeStack Struct**: Define the main stack struct, which will contain an `AtomicPtr` to the `head` of the stack.

3.  **push()**:
    -   Create a new `Node` on the heap.
    -   In a loop, atomically read the current `head`.
    -   Set the `next` pointer of your new node to the current `head`.
    -   Use `compare_exchange` to swing the `head` pointer to your new node.
    -   If the `compare_exchange` fails, it means another thread pushed a node in the meantime. Retry the loop.

4.  **pop()**:
    -   In a loop, atomically read the current `head`.
    -   If the head is null, the stack is empty; return `None`.
    -   Use `compare_exchange` to set the `head` to the *next* node in the list.
    -   If successful, you now exclusively "own" the old head node. Safely take its value and deallocate the node.
    -   If the `compare_exchange` fails, another thread modified the stack. Retry.

5.  **Drop**: Implement the `Drop` trait to safely deallocate all remaining nodes when the stack goes out of scope, preventing memory leaks.

## Solution Explanation (No Code - Just Ideas)

**push()**:
Imagine trying to add a plate to a stack while other people are doing the same.
1.  You look at the top plate (`head`).
2.  You get your new plate ready, knowing it will go on top of that one.
3.  You try to place your plate on top. But just as you do, you check: "Is the top plate still the one I saw?".
4.  If yes, you've successfully placed your plate. The new top is your plate.
5.  If no, someone else put a plate on top while you were preparing. You can't just put yours there now. You have to start over: look at the *new* top plate and try again. This "retry" loop is the heart of the algorithm.

**pop()**:
1.  You look at the top plate (`head`). If there's nothing, you're done.
2.  You see what the *second* plate in the stack is. This will be the *new* top plate after you take yours.
3.  You try to perform the action: "I'm taking the top plate, so now the official top plate is the second one". You use `compare_exchange` to do this atomically.
4.  If it succeeds, you've successfully claimed the top plate. You can take the value out and discard the plate.
5.  If it fails, someone else either added a new plate or took the one you were looking at. You must start over.

## Where Rust Shines

**Compared to C/C++**:
In C++, implementing a lock-free stack is notoriously difficult and error-prone.
-   **Manual Memory Management**: You have to manually manage memory, leading to risks of memory leaks or use-after-free bugs, especially the infamous "ABA problem".
-   **Undefined Behavior**: A mistake in memory ordering or pointer handling is undefined behavior, which can lead to silent data corruption or crashes that are impossible to debug.
-   **Rust's Ownership**: Rust's ownership model helps manage the lifetime of nodes. `Box` gives us clear ownership, and `unsafe` forces us to be explicit and careful when we're dealing with raw pointers, making it much harder to misuse them. The borrow checker still helps in the safe parts of the code.

**Why this matters**:
-   Rust makes it possible to write low-level, high-performance code with a much higher degree of safety than its predecessors.
-   The compiler guides you away from common concurrency pitfalls. While `unsafe` is a "trust me" block, the rest of the language provides a safe foundation, minimizing the `unsafe` surface area.

## Common Beginner Mistakes & How to Avoid Them

1.  **The ABA Problem**:
    -   Thread 1 reads pointer A.
    -   Thread 2 pops A, pushes B, then pushes a *new* node at the *same memory address* as A.
    -   Thread 1 does a CAS, sees the pointer is still A, and succeeds, corrupting the stack.
    -   **Solution**: Use techniques like hazard pointers or epoch-based reclamation. For this lab, we will simplify and ignore this problem, but it's a critical concept in real-world lock-free code.

2.  **Incorrect Memory Ordering**:
    -   Using `Relaxed` ordering everywhere can cause the CPU to reorder operations in unexpected ways, breaking the logic.
    -   **Fix**: Use `Acquire` on reads that need to synchronize with a `Release` on a write. An `Acquire` load prevents subsequent memory operations from being reordered before it. A `Release` store prevents previous memory operations from being reordered after it. This creates a synchronization point.

3.  **Memory Leaks**:
    -   Forgetting to deallocate a `Node` after it's popped.
    -   `Box::into_raw` gives up ownership. If you don't call `Box::from_raw` later, the memory is leaked.
    -   **Fix**: Ensure every `Box::into_raw` has a corresponding `Box::from_raw` in some code path, or the memory is otherwise managed. Implement `Drop` on the stack to clean up any remaining nodes.

## Stretch Goals

1.  **Implement `len()`**: Add a method to get the number of items in the stack. This is tricky! A simple `AtomicUsize` counter needs to be carefully incremented and decremented with correct memory ordering to be accurate.

2.  **Make it ABA-safe**: Research and implement a simple form of hazard pointers or use the `crossbeam-epoch` crate to manage memory safely and solve the ABA problem.

3.  **Implement a `LockFreeQueue`**: A queue is "first-in, first-out" (FIFO) and is significantly more complex to implement lock-free than a stack. It requires managing both a `head` and a `tail` pointer atomically.

## What's Next?

After this project, you'll have a deep appreciation for the complexities of concurrency and the tools Rust provides to manage it. You'll be ready to tackle other advanced systems programming topics like key-value stores or basic virtual machines. Good luck!