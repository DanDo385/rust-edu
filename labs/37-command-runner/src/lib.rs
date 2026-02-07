// Lab 37: Command Runner
//
// Demonstrates spawning and managing child processes using std::process::Command.
// Shows how to execute shell commands, capture output, set environment variables,
// run pipelines, and build practical automation tools.
//
// All public types and functions are pure std -- no external dependencies.

use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

// ============================================================================
// COMMAND RESULT
// ============================================================================

/// The captured result of executing a command.
///
/// # Ownership Model
/// `stdout` and `stderr` own their string data (heap-allocated).
/// `exit_code` is Copy (Option<i32> on the stack).
/// On Unix, a process killed by a signal may have no exit code.
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Standard output captured from the child process.
    pub stdout: String,
    /// Standard error captured from the child process.
    pub stderr: String,
    /// The process exit code (None if terminated by signal on Unix).
    pub exit_code: Option<i32>,
    /// Whether the command exited successfully (exit code 0).
    pub success: bool,
}

// ============================================================================
// COMMAND RUNNER
// ============================================================================

/// Executes external commands and captures their output.
///
/// # Design
/// Each method returns a `Result<CommandResult, String>`.
/// The outer `Result` represents failure to *launch* the process (e.g.,
/// command not found). A non-zero exit code is not an error at the
/// launch level -- it is reflected in `CommandResult::success`.
pub struct CommandRunner;

impl CommandRunner {
    /// Runs a command with arguments and captures stdout/stderr.
    ///
    /// # Examples
    /// ```no_run
    /// use command_runner::CommandRunner;
    /// let result = CommandRunner::run("echo", &["hello"]).unwrap();
    /// assert!(result.success);
    /// assert_eq!(result.stdout.trim(), "hello");
    /// ```
    pub fn run(cmd: &str, args: &[&str]) -> Result<CommandResult, String> {
        let output = Command::new(cmd)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute '{}': {}", cmd, e))?;

        Ok(CommandResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            success: output.status.success(),
        })
    }

    /// Runs a command in a specific working directory.
    ///
    /// # Ownership
    /// `dir` is borrowed (&str) -- we only need it for the duration of the
    /// Command setup. No allocation required.
    pub fn run_in_dir(cmd: &str, args: &[&str], dir: &str) -> Result<CommandResult, String> {
        let output = Command::new(cmd)
            .args(args)
            .current_dir(dir)
            .output()
            .map_err(|e| format!("Failed to execute '{}' in '{}': {}", cmd, dir, e))?;

        Ok(CommandResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            success: output.status.success(),
        })
    }

    /// Runs a command with custom environment variables.
    ///
    /// `envs` is a slice of (key, value) pairs added to the child environment.
    /// The parent environment is inherited; these pairs are additions/overrides.
    pub fn run_with_env(
        cmd: &str,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> Result<CommandResult, String> {
        let mut command = Command::new(cmd);
        command.args(args);

        for (key, value) in envs {
            command.env(key, value);
        }

        let output = command
            .output()
            .map_err(|e| format!("Failed to execute '{}': {}", cmd, e))?;

        Ok(CommandResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            success: output.status.success(),
        })
    }

    /// Runs a command with a timeout. Returns Err if the command exceeds
    /// the timeout (the child process is killed).
    ///
    /// # Implementation
    /// Spawns the child, then polls with `try_wait()` in a loop.
    /// If elapsed time exceeds `timeout`, kills the child.
    pub fn run_with_timeout(
        cmd: &str,
        args: &[&str],
        timeout: Duration,
    ) -> Result<CommandResult, String> {
        let start = Instant::now();

        let mut child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn '{}': {}", cmd, e))?;

        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    let output = child
                        .wait_with_output()
                        .map_err(|e| format!("Failed to read output: {}", e))?;
                    return Ok(CommandResult {
                        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                        exit_code: status.code(),
                        success: status.success(),
                    });
                }
                Ok(None) => {
                    if start.elapsed() > timeout {
                        let _ = child.kill();
                        let _ = child.wait();
                        return Err(format!(
                            "Command '{}' timed out after {:?}",
                            cmd, timeout
                        ));
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(e) => {
                    return Err(format!("Error waiting for '{}': {}", cmd, e));
                }
            }
        }
    }

    /// Pipes the output of one command into another.
    ///
    /// Runs `cmd1 args1 | cmd2 args2` by capturing cmd1's stdout
    /// and feeding it to cmd2's stdin.
    ///
    /// # Ownership
    /// The intermediate output (`first_output.stdout`) is owned by this
    /// function. It is written into cmd2's stdin pipe and then dropped.
    pub fn pipe(
        cmd1: &str,
        args1: &[&str],
        cmd2: &str,
        args2: &[&str],
    ) -> Result<CommandResult, String> {
        // Run first command and capture output
        let first_output = Command::new(cmd1)
            .args(args1)
            .output()
            .map_err(|e| format!("Failed to execute '{}': {}", cmd1, e))?;

        // Spawn second command with piped stdin
        let mut second_child = Command::new(cmd2)
            .args(args2)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn '{}': {}", cmd2, e))?;

        // Write first command's stdout to second command's stdin
        if let Some(mut stdin) = second_child.stdin.take() {
            stdin
                .write_all(&first_output.stdout)
                .map_err(|e| format!("Failed to write to '{}' stdin: {}", cmd2, e))?;
        }

        let output = second_child
            .wait_with_output()
            .map_err(|e| format!("Failed to wait for '{}': {}", cmd2, e))?;

        Ok(CommandResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            success: output.status.success(),
        })
    }
}

// ============================================================================
// COMMAND BUILDER
// ============================================================================
// Builder pattern for constructing commands with many options.

/// A fluent builder for constructing and running commands.
///
/// # Ownership
/// All strings are owned (String). This allows the builder to be moved
/// freely and even sent across threads if needed.
pub struct CommandBuilder {
    command: String,
    args: Vec<String>,
    envs: Vec<(String, String)>,
    working_dir: Option<String>,
    timeout: Option<Duration>,
}

impl CommandBuilder {
    /// Creates a new builder for the given command.
    pub fn new(command: &str) -> Self {
        CommandBuilder {
            command: command.to_string(),
            args: Vec::new(),
            envs: Vec::new(),
            working_dir: None,
            timeout: None,
        }
    }

    /// Adds a single argument.
    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    /// Adds multiple arguments at once.
    pub fn args(mut self, args: &[&str]) -> Self {
        for a in args {
            self.args.push(a.to_string());
        }
        self
    }

    /// Adds an environment variable.
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.envs.push((key.to_string(), value.to_string()));
        self
    }

    /// Sets the working directory.
    pub fn working_dir(mut self, dir: &str) -> Self {
        self.working_dir = Some(dir.to_string());
        self
    }

    /// Sets a timeout for the command.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Executes the command and returns the result.
    ///
    /// If a timeout is set, uses the timeout variant. Otherwise,
    /// runs to completion.
    pub fn run(self) -> Result<CommandResult, String> {
        if let Some(timeout) = self.timeout {
            // Build args as &str slices
            let arg_refs: Vec<&str> = self.args.iter().map(|s| s.as_str()).collect();
            return CommandRunner::run_with_timeout(&self.command, &arg_refs, timeout);
        }

        let mut cmd = Command::new(&self.command);

        for arg in &self.args {
            cmd.arg(arg);
        }

        for (key, value) in &self.envs {
            cmd.env(key, value);
        }

        if let Some(ref dir) = self.working_dir {
            cmd.current_dir(dir);
        }

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to execute '{}': {}", self.command, e))?;

        Ok(CommandResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            success: output.status.success(),
        })
    }
}

// ============================================================================
// TASK RUNNER
// ============================================================================

/// A single task to be executed by the TaskRunner.
#[derive(Debug, Clone)]
pub struct Task {
    /// Human-readable name for the task.
    pub name: String,
    /// The command to execute.
    pub command: String,
    /// Arguments to pass to the command.
    pub args: Vec<String>,
}

/// The result of running a single task.
#[derive(Debug, Clone)]
pub struct TaskResult {
    /// The task name.
    pub name: String,
    /// The command result (None if the command failed to launch).
    pub result: Result<CommandResult, String>,
    /// How long the task took.
    pub duration: Duration,
}

/// Manages and executes a sequence of tasks.
///
/// # Ownership
/// TaskRunner owns its `Vec<Task>`. Each task is consumed when run
/// (we iterate over references, but the runner retains ownership).
pub struct TaskRunner {
    tasks: Vec<Task>,
}

impl TaskRunner {
    /// Creates an empty task runner.
    pub fn new() -> Self {
        TaskRunner { tasks: Vec::new() }
    }

    /// Adds a task to the runner.
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Returns the number of tasks.
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    /// Runs all tasks sequentially and returns their results.
    ///
    /// Unlike the main.rs version, this does not print anything --
    /// it returns structured results that the caller can inspect.
    pub fn run_all(&self) -> Vec<TaskResult> {
        self.tasks
            .iter()
            .map(|task| {
                let start = Instant::now();
                let arg_refs: Vec<&str> = task.args.iter().map(|s| s.as_str()).collect();
                let result = CommandRunner::run(&task.command, &arg_refs);
                let duration = start.elapsed();

                TaskResult {
                    name: task.name.clone(),
                    result,
                    duration,
                }
            })
            .collect()
    }
}

impl Default for TaskRunner {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_result_fields() {
        let result = CommandResult {
            stdout: "hello".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
            success: true,
        };
        assert!(result.success);
        assert_eq!(result.exit_code, Some(0));
    }

    #[test]
    fn test_command_builder_new() {
        let builder = CommandBuilder::new("echo");
        assert_eq!(builder.command, "echo");
        assert!(builder.args.is_empty());
    }

    #[test]
    fn test_task_runner_empty() {
        let runner = TaskRunner::new();
        assert_eq!(runner.task_count(), 0);
    }

    #[test]
    fn test_task_runner_add() {
        let mut runner = TaskRunner::new();
        runner.add_task(Task {
            name: "test".to_string(),
            command: "echo".to_string(),
            args: vec!["hi".to_string()],
        });
        assert_eq!(runner.task_count(), 1);
    }
}
