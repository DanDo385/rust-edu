//! # A Command Runner - Complete Solution
//!
//! ## What We're Building
//!
//! A convenient and safe builder API around `std::process::Command` to make
//! running external commands ergonomic. We also build a simple `TaskRunner`
//! on top of it to execute a sequence of commands.
//!
//! ## Why Rust Is Perfect For This
//!
//! - **The Builder Pattern**: The standard library's `Command` uses the builder
//!   pattern, which is a common and powerful pattern in Rust for constructing
//!   complex objects. We extend this pattern in our own `CommandBuilder`.
//! - **Error Handling**: `std::io::Error` and our custom `CommandError` enum,
//!   combined with `Result`, provide a robust way to handle the many things
//!   that can go wrong when dealing with external processes.
//! - **Ownership and Lifetimes**: Rust ensures that handles to child processes
//!   (the `Child` struct) are properly managed.
//!
//! ## Key Concepts in this Solution
//!
//! - **`std::process::Command`**: The core API for process creation.
//! - **`spawn()` vs `output()`**: We use `spawn()` to get a `Child` handle, which
//!   gives us more control (like the ability to kill it), instead of the simpler, 
//!   blocking `output()` method.
//! - **Timeout Implementation**: A simple, cross-platform timeout is implemented
//!   by polling `child.try_wait()` in a loop rather than using platform-specific
//!   APIs.

use std::process::{Command, Stdio, Child};
use std::time::{Duration, Instant};
use std::io::{Read};
use thiserror::Error;

/// An error type for our command running operations.
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Command timed out after {0:?}")]
    Timeout(Duration),
}

/// Holds the result of a completed command.
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// A builder for creating and running external commands.
#[derive(Default, Clone)]
pub struct CommandBuilder {
    command: String,
    args: Vec<String>,
    envs: Vec<(String, String)>,
    current_dir: Option<String>,
    timeout: Option<Duration>,
}

impl CommandBuilder {
    /// Creates a new `CommandBuilder`.
    pub fn new(command: impl Into<String>) -> Self {
        CommandBuilder {
            command: command.into(),
            ..Default::default()
        }
    }

    /// Adds an argument.
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Adds an environment variable.
    pub fn env(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.envs.push((key.into(), val.into()));
        self
    }

    /// Sets the working directory.
    pub fn current_dir(mut self, path: impl Into<String>) -> Self {
        self.current_dir = Some(path.into());
        self
    }

    /// Sets a timeout for the command.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Executes the command.
    pub fn run(&self) -> Result<CommandResult, CommandError> {
        let mut cmd = Command::new(&self.command);

        // Configure the command
        cmd.args(&self.args)
            .envs(self.envs.clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(dir) = &self.current_dir {
            cmd.current_dir(dir);
        }

        // Spawn the child process
        let mut child = cmd.spawn()?;

        if let Some(timeout) = self.timeout {
            // --- Timeout Logic ---
            let start = Instant::now();
            loop {
                // Check if the process has finished
                match child.try_wait()? {
                    Some(status) => { // Process finished
                        // Collect output after process has exited
                        let output = child.wait_with_output()?;
                        return Ok(CommandResult {
                            exit_code: status.code().unwrap_or(1),
                            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                        });
                    }
                    None => { // Process still running
                        if start.elapsed() > timeout {
                            // Timeout exceeded, kill the process
                            child.kill()?;
                            return Err(CommandError::Timeout(timeout));
                        }
                        // Sleep for a short duration before checking again
                        std::thread::sleep(Duration::from_millis(50));
                    }
                }
            }
        } else {
            // --- No Timeout Logic ---
            let output = child.wait_with_output()?;
            Ok(CommandResult {
                exit_code: output.status.code().unwrap_or(1),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
    }
}

/// Represents a single named task to be run.
pub struct Task {
    name: String,
    builder: CommandBuilder,
}

impl Task {
    pub fn new(name: String, builder: CommandBuilder) -> Self {
        Task { name, builder }
    }
}

/// Runs a sequence of tasks.
pub struct TaskRunner {
    tasks: Vec<Task>,
}

impl TaskRunner {
    pub fn new(tasks: Vec<Task>) -> Self {
        TaskRunner { tasks }
    }

    /// Runs all tasks in sequence, stopping if one fails.
    pub fn run(&mut self) -> Vec<Result<CommandResult, CommandError>> {
        let mut results = Vec::new();
        for task in &self.tasks {
            println!("Running task: \"{}\"...", task.name);
            let result = task.builder.run();
            match &result {
                Ok(res) if res.exit_code == 0 => {
                    results.push(result);
                }
                Ok(_) => { // Non-zero exit code is considered a failure
                    results.push(result);
                    println!("Task \"{}\" failed, stopping runner.", task.name);
                    break;
                }
                Err(_) => { // I/O or timeout error
                    results.push(result);
                    println!("Task \"{}\" failed, stopping runner.", task.name);
                    break;
                }
            }
        }
        results
    }
}
