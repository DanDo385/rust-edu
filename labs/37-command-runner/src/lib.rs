//! # A Command Runner - Your Implementation
//!
//! This project is about building a safe and convenient wrapper around
//! `std::process::Command` to run external programs.
//!
//! ## Your Task
//!
//! Implement the `CommandBuilder` and `TaskRunner`.
//!
//! 1.  **`CommandResult` Struct**: Define a struct to hold the results of an
//!     executed command, including exit code, stdout, and stderr.
//!
//! 2.  **`CommandBuilder` Struct**: Create a builder that allows for chaining
//!     methods to configure a command before running it.
//!
//! 3.  **`run()` method**: The final method on the builder that executes the
//!     configured command, handles timeouts, and returns a `CommandResult`.
//!
//! 4.  **`Task` and `TaskRunner` (Stretch Goal)**: Structs to manage and run a
//!     sequence of commands.
//!
//! ## Running Your Code
//!
//! ```bash
//! cargo test -p command-runner
//! cargo run -p command-runner
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

use std::process::{Command, Stdio};
use std::time::Duration;
use thiserror::Error;

// TODO: Define your error type.
// It should be able to represent I/O errors and a timeout error.
// #[derive(Debug, Error)]
// pub enum CommandError { ... }
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Command timed out after {0:?}")]
    Timeout(Duration),
}


// TODO: Define the CommandResult struct.
// It should hold the exit code, stdout, and stderr.
// #[derive(Debug)]
// pub struct CommandResult { ... }
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}


// TODO: Define the CommandBuilder struct.
// It should hold all the configuration for a command.
// - command: String
// - args: Vec<String>
// - envs: Vec<(String, String)>
// - current_dir: Option<String>
// - timeout: Option<Duration>
//
// #[derive(Default, Clone)]
// pub struct CommandBuilder { ... }
#[derive(Default, Clone)]
pub struct CommandBuilder {
    command: String,
    args: Vec<String>,
    envs: Vec<(String, String)>,
    current_dir: Option<String>,
    timeout: Option<Duration>,
}

impl CommandBuilder {
    /// Creates a new `CommandBuilder` for a given command.
    pub fn new(command: impl Into<String>) -> Self {
        todo!("Initialize the CommandBuilder");
    }

    /// Adds an argument to the command.
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        todo!("Add an argument to the args vector");
    }

    /// Sets an environment variable for the command.
    pub fn env(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        todo!("Add a (key, val) tuple to the envs vector");
    }

    /// Sets the working directory for the command.
    pub fn current_dir(mut self, path: impl Into<String>) -> Self {
        todo!("Set the current_dir field");
    }

    /// Sets a timeout for the command.
    pub fn timeout(mut self, duration: Duration) -> Self {
        todo!("Set the timeout field");
    }

    /// Executes the command.
    pub fn run(&self) -> Result<CommandResult, CommandError> {
        // TODO: Implement the run logic.
        // 1. Create a `std::process::Command` from the builder's fields.
        // 2. Configure args, envs, current_dir.
        // 3. Set up stdout and stderr to be `Stdio::piped()`.
        // 4. `spawn()` the process.
        // 5. If there's a timeout, you'll need to manage waiting for the
        //    process. A simple approach is to loop with `try_wait` for the
        //    duration of the timeout. If it doesn't finish in time,
        //    `.kill()` the child process and return a timeout error.
        // 6. If there's no timeout, use `wait_with_output()`.
        // 7. Collect the exit code, stdout, and stderr into your
        //    `CommandResult` struct and return it.
        todo!("Execute the configured command");
    }
}


// --- Stretch Goal: Task Runner ---

// TODO: Define the Task struct.
// It should have a name and a `CommandBuilder`.
// pub struct Task { ... }
pub struct Task {
    pub name: String,
    pub builder: CommandBuilder,
}

// TODO: Define the TaskRunner struct.
// It should hold a vector of `Task`s.
// pub struct TaskRunner { ... }
pub struct TaskRunner {
    tasks: Vec<Task>,
}

impl TaskRunner {
    /// Creates a new `TaskRunner` with a list of tasks.
    pub fn new(tasks: Vec<Task>) -> Self {
        todo!("Initialize the TaskRunner");
    }

    /// Runs all tasks in sequence, stopping if a task fails.
    pub fn run(&mut self) -> Vec<Result<CommandResult, CommandError>> {
        todo!("Loop through tasks, run them, and collect results");
    }
}


// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;
