# Project 37 - A Command Runner

## What You're Building (Plain English)

You're building a flexible tool to run other programs (commands) from your Rust code. Think of it as a super-powered version of running a command in your terminal. You'll create a "Command Runner" that can execute external programs like `ls`, `grep`, or even other Rust binaries you've compiled.

Your tool will be able to:
-   Run a command with arguments.
-   Capture its output (`stdout` and `stderr`).
-   Check its exit status to see if it succeeded or failed.
-   Run the command in a specific directory.
-   Set environment variables for the command.
-   Enforce a timeout, killing the command if it runs for too long.

## New Rust Concepts in This Project

-   **`std::process::Command`**: The core of this project. You'll learn how to use this struct to build and spawn external processes.
-   **Builder Pattern**: `Command` is a classic example of the builder pattern. You chain methods like `.arg()`, `.env()`, and `.current_dir()` to configure the process before running it with `.spawn()` or `.status()`.
-   **Capturing I/O**: You'll configure `Stdio` to capture the output of child processes for logging or further processing.
-   **Error Handling**: Spawning processes and running commands can fail in many ways. You'll handle I/O errors and check the `ExitStatus` of completed processes.
-   **Working with Lifetimes**: If you build a more advanced `CommandBuilder` struct, you will need to manage the lifetimes of references to strings for arguments and environment variables.

## Rust Syntax You'll See

```rust
use std::process::{Command, Stdio, Output};
use std::time::Duration;

// Build and run a command
let output: Result<Output, std::io::Error> = Command::new("echo")
    .arg("Hello, world!")
    .stdout(Stdio::piped()) // Capture stdout
    .stderr(Stdio::piped()) // Capture stderr
    .spawn()? // Spawn the child process
    .wait_with_output(); // Wait for it to finish and collect output

match output {
    Ok(output) => {
        // let status = output.status;
        // let stdout = String::from_utf8_lossy(&output.stdout);
        // let stderr = String::from_utf8_lossy(&output.stderr);
    }
    Err(e) => { /* Handle error */ }
}

// Running a command with a timeout is more complex and often
// requires platform-specific code or async runtimes. We might
// simulate it for this lab.
```

## How to Run

```bash
# Run the main binary (a demo of the command runner)
cargo run -p command-runner

# Run the tests
cargo test -p command-runner

# Check if code compiles
cargo check -p command-runner
```

## The Exercises

You will implement a `run_command` function and a more advanced `CommandBuilder`.

1.  **`CommandResult` Struct**: Create a struct to hold the results of a finished command: `exit_code`, `stdout`, and `stderr`.

2.  **`run_command` Function**:
    -   Takes the command name, arguments, and an optional timeout.
    -   Uses `std::process::Command` to set up and run the command.
    -   Captures the stdout and stderr.
    -   Handles the timeout (this is a stretch goal and can be tricky!).
    -   Returns a `Result<CommandResult, Error>`.

3.  **`CommandBuilder` Struct (Builder Pattern)**:
    -   Create a struct that allows for a more ergonomic way to build a command.
    -   `.new(command)`: Starts building a command.
    -   `.arg(s)`: Adds an argument.
    -   `.env(key, val)`: Sets an environment variable.
    -   `.current_dir(path)`: Sets the working directory.
    -   `.timeout(duration)`: Sets a timeout.
    -   `.run()`: Executes the command and returns a `CommandResult`.

4.  **`TaskRunner` (Stretch Goal)**:
    -   Build a struct that can run a sequence of commands, stopping if one of them fails.

## Solution Explanation (No Code - Just Ideas)

**The `std::process::Command` Builder**:
The key is to understand that `Command::new()` gives you a builder object. You don't run the command immediately. You first call methods on it to configure the `stdin`, `stdout`, `stderr`, working directory, etc. Each of these methods returns `&mut Self`, allowing you to chain the calls together fluently. The command only runs when you call a final method like `spawn()`, `status()`, or `output()`.

**Capturing Output**:
To capture output, you must configure the command's `stdout` and `stderr` to be "piped" using `Stdio::piped()`. This tells the operating system to create a pipe that your Rust program can read from to get the child process's output. The `wait_with_output()` method conveniently waits for the process to finish and reads all the data from these pipes for you.

**Handling Timeouts**:
A simple, cross-platform timeout with `std::process` is not straightforward. A common approach on Unix-like systems involves using process groups and `libc` calls to send a `SIGKILL`. A simpler, but less robust, approach for this lab could be to `spawn` the process, then `try_wait` in a loop for a certain duration. If the process hasn't finished by the end of the duration, you can call `.kill()` on the `Child` object.

## Where Rust Shines

-   **Safety**: Rust's ownership model ensures that handles to child processes are managed correctly. When a `Child` struct is dropped, its process is automatically cleaned up (reaped) by the parent.
-   **Expressive Error Handling**: The `Result` type is perfect for handling the many things that can go wrong: the command might not exist, you might not have permission to run it, it might fail at runtime, etc.
-   **Clear Process Configuration**: The builder pattern used by `Command` makes it very clear and readable to see how a process is being configured before it's launched.

## Common Beginner Mistakes

1.  **Forgetting to Capture Output**: If you don't set `.stdout(Stdio::piped())`, the child process's output will just go to the same place as your parent process's output (usually your terminal), and you won't be able to capture it in a variable.
2.  **Mixing up `spawn()` and `status()`/`output()`**:
    -   `spawn()`: Starts the child process and immediately returns a `Child` handle, allowing your program to continue running while the child works. This is non-blocking.
    -   `status()`/`output()`: These are blocking. They will start the child process and wait for it to complete before returning.
3.  **Error Handling**: Not checking the `ExitStatus` of the child process. A command can run without I/O errors but still fail (e.g., `grep` not finding a match and returning exit code 1). A successful `Result` from `wait_with_output` does not mean the *command* succeeded, only that it *ran*.

This lab is a practical dive into how programs can interact with the wider operating system and run other tools.