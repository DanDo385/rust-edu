# Project 40 - A Task Scheduler

## What You're Building (Plain English)

You're building a simple "to-do" list for your computer that operates on a timeline. This "task scheduler" will let you schedule a piece of work (a "task") to be done at a specific time in the future. You'll also be able to schedule recurring tasks, like "run this clean-up job every 5 minutes."

Your scheduler will always know which task needs to be run next and will be able to "execute" it, which for this lab will mean simply printing a message. This is the fundamental logic behind systems like `cron` on Linux, the Windows Task Scheduler, or timers in a web browser (`setTimeout`, `setInterval`).

## New Rust Concepts in This Project

-   **`std::collections::BinaryHeap`**: A priority queue. This data structure is perfect for a scheduler because it always lets you access the "greatest" item in O(1) time. We can define "greatest" to mean the task that should be run soonest, allowing us to instantly know what to run next.
-   **`std::time::{Instant, Duration}`**: You'll work with Rust's standard library types for handling time, which are essential for calculating when tasks are due.
-   **Wrapper Structs for Ordering**: A `BinaryHeap` is a *max*-heap, meaning it gives you the largest item. To make it behave like a *min*-heap (so we can get the task with the *earliest* execution time), we'll wrap our task struct and implement the `Ord` and `PartialOrd` traits in reverse.
-   **Stateful Systems**: You'll be building a system that manages a complex state (the queue of pending tasks) and evolves it over time as tasks are added and executed.

## Rust Syntax You'll See

```rust
use std::collections::BinaryHeap;
use std::time::{Instant, Duration};

// A task to be run
struct Task {
    execution_time: Instant,
    id: u64,
    // ... other data
}

// We need to implement ordering traits for Task to be used in BinaryHeap
// impl Ord for Task { ... }

// A scheduler holding the priority queue
struct Scheduler {
    tasks: BinaryHeap<Task>,
}

// Peeking at the next task
// if let Some(next_task) = scheduler.tasks.peek() {
//     if next_task.execution_time <= Instant::now() {
//         // It's time to run this task!
//         // let task_to_run = scheduler.tasks.pop();
//     }
// }
```

## How to Run

```bash
# Run the main binary (a demo of the scheduler)
cargo run -p task-scheduler

# Run the tests
cargo test -p task-scheduler

# Check if code compiles
cargo check -p task-scheduler
```

## The Exercises

You will implement the `Task` and `Scheduler` structs.

1.  **`Task` Struct**:
    -   Define a struct to hold all information about a task: a unique ID, a name, the next execution time (`Instant`), and whether it's a one-time or recurring task (perhaps with an `Option<Duration>` for the interval).
    -   Implement `Eq`, `PartialEq`, `Ord`, and `PartialOrd`. The key is to reverse the ordering for `Ord` so that the `BinaryHeap` acts as a min-heap on `execution_time`. A task is "greater" if its execution time is *sooner*.

2.  **`Scheduler` Struct**:
    -   The struct will hold the `BinaryHeap<Task>` and a counter for assigning unique task IDs.

3.  **`new()`**: A constructor for an empty scheduler.

4.  **`schedule_once()`**: Takes a `Duration` from now and a name, and adds a new one-time task to the heap.

5.  **`schedule_recurring()`**: Takes an initial delay, a recurring `Duration` (interval), and a name, and adds a new recurring task.

6.  **`execute_next()`**:
    -   This is the main "tick" function of your scheduler.
    -   It should `peek()` at the top of the heap to see the next task.
    -   It checks if the task's `execution_time` is in the past (i.e., it's due).
    -   If a task is due, it `pop()`s it from the heap.
    -   It "executes" the task (e.g., returns it or some information about it).
    -   If the task was recurring, it calculates the *next* execution time and `push`es a new version of the task back onto the heap.
    -   If no tasks are due, it returns `None`.

## Solution Explanation (No Code - Just Ideas)

**The `BinaryHeap` as a Min-Heap**:
A `BinaryHeap` in Rust is a max-heap. It always gives you the "greatest" element. We want the task with the "smallest" execution time. The solution is to lie to the `BinaryHeap`. We implement `Ord` for our `Task` struct like this:
`other.execution_time.cmp(&self.execution_time)`
By swapping `self` and `other`, we tell the heap that `Task A` is "greater" than `Task B` if `A`'s time is *less than* `B`'s. The heap will then diligently keep the task with the earliest time at the top, ready for us to `peek()` at.

**The Execution Loop**:
A real-world scheduler would run in a loop, possibly on its own thread. It would:
1.  Look at the next task to run.
2.  Calculate how long it needs to sleep until that task is due.
3.  Sleep for that duration.
4.  Wake up, execute the task, and repeat.
For this lab, our `execute_next()` method will just be a single "tick" of this loop that we can call manually in our demo and tests.

## Where Rust Shines

-   **`std::collections`**: `BinaryHeap` provides a powerful and efficient priority queue out of the box.
-   **Trait System**: The ability to implement `Ord` and other traits on our own structs allows us to customize their behavior and integrate them seamlessly with standard library data structures.
-   **Type Safety**: The `std::time` types (`Instant`, `Duration`) provide a safe, platform-agnostic way to handle time calculations, avoiding many common bugs related to time zones or clock adjustments.

## Common Beginner Mistakes

1.  **Getting the `Ord` implementation backward**: This is very common. If your scheduler is running tasks in the wrong order, double-check your `cmp` implementation.
2.  **Mutable vs. Immutable Borrows with the `BinaryHeap`**: `peek()` gives an immutable reference, while `pop()` gives an owned value. You can't `pop` while you're still holding a reference from `peek()`. The API forces you to handle this correctly.
3.  **Drifting in Recurring Tasks**: When rescheduling a recurring task, if you calculate the next time based on `Instant::now()`, any delays in execution will accumulate, causing the task to "drift." A more robust implementation calculates the next time based on the *previous* scheduled time.

This project is a great exercise in data structures, state management, and thinking about how systems evolve over time.