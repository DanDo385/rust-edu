// Project 34: Command Runner
//
// Demonstrates spawning and managing child processes using std::process::Command.
// Shows how to execute shell commands, capture output, run commands in parallel,
// handle timeouts, and build practical automation tools.

use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Command Runner ===\n");

    // ============================================================================
    // BASIC COMMAND EXECUTION
    // ============================================================================
    println!("=== Basic Command Execution ===");

    // Simple command with no arguments
    run_command_simple("echo", &["Hello from command runner!"]);

    // Command with multiple arguments
    run_command_simple("ls", &["-l", "-a", "-h"]);

    println!();

    // ============================================================================
    // CAPTURING OUTPUT
    // ============================================================================
    println!("=== Capturing Command Output ===");

    // Capture stdout and stderr
    match Command::new("echo").arg("Captured output").output() {
        Ok(output) => {
            println!(
                "Stdout: {}",
                String::from_utf8_lossy(&output.stdout).trim()
            );
            println!("Exit code: {}", output.status);
        }
        Err(e) => eprintln!("Failed to execute: {}", e),
    }

    println!();

    // ============================================================================
    // EXIT CODE HANDLING
    // ============================================================================
    println!("=== Exit Code Handling ===");

    // Success
    if let Ok(output) = Command::new("true").output() {
        println!(
            "Command 'true' exited with: {} (success: {})",
            output.status,
            output.status.success()
        );
    }

    // Failure
    if let Ok(output) = Command::new("false").output() {
        println!(
            "Command 'false' exited with: {} (success: {})",
            output.status,
            output.status.success()
        );
    }

    println!();

    // ============================================================================
    // WORKING DIRECTORY
    // ============================================================================
    println!("=== Setting Working Directory ===");

    match Command::new("pwd").current_dir("/tmp").output() {
        Ok(output) => {
            println!(
                "Current directory: {}",
                String::from_utf8_lossy(&output.stdout).trim()
            );
        }
        Err(e) => eprintln!("Failed: {}", e),
    }

    println!();

    // ============================================================================
    // ENVIRONMENT VARIABLES
    // ============================================================================
    println!("=== Environment Variables ===");

    match Command::new("sh")
        .arg("-c")
        .arg("echo $CUSTOM_VAR")
        .env("CUSTOM_VAR", "Hello from environment!")
        .output()
    {
        Ok(output) => {
            println!(
                "Environment variable: {}",
                String::from_utf8_lossy(&output.stdout).trim()
            );
        }
        Err(e) => eprintln!("Failed: {}", e),
    }

    println!();

    // ============================================================================
    // PARALLEL COMMAND EXECUTION
    // ============================================================================
    println!("=== Parallel Command Execution ===");

    let commands = vec![
        ("echo", vec!["Task 1"]),
        ("echo", vec!["Task 2"]),
        ("echo", vec!["Task 3"]),
        ("echo", vec!["Task 4"]),
    ];

    println!("Running {} commands in parallel...", commands.len());
    let start = Instant::now();

    let handles: Vec<_> = commands
        .into_iter()
        .map(|(cmd, args)| {
            thread::spawn(move || {
                let args_str: Vec<&str> = args.iter().map(|s| &s[..]).collect();
                run_command_simple(cmd, &args_str);
            })
        })
        .collect();

    // Wait for all commands to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All commands completed in {:?}", start.elapsed());
    println!();

    // ============================================================================
    // COMMAND WITH TIMEOUT
    // ============================================================================
    println!("=== Command with Timeout ===");

    println!("Running command with 2-second timeout...");
    run_with_timeout("sleep", &["1"], Duration::from_secs(2));

    println!("Running command that will timeout...");
    run_with_timeout("sleep", &["5"], Duration::from_secs(2));

    println!();

    // ============================================================================
    // HANDLING ERRORS
    // ============================================================================
    println!("=== Error Handling ===");

    // Command not found
    match Command::new("nonexistent_command_xyz").output() {
        Ok(_) => println!("Command executed successfully (unexpected)"),
        Err(e) => println!("Expected error - command not found: {}", e),
    }

    // Command exists but fails
    match Command::new("grep")
        .arg("nonexistent_pattern")
        .arg("nonexistent_file.txt")
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("Grep succeeded");
            } else {
                println!(
                    "Grep failed with exit code: {}",
                    output.status.code().unwrap_or(-1)
                );
                println!(
                    "Stderr: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                );
            }
        }
        Err(e) => eprintln!("Failed to run grep: {}", e),
    }

    println!();

    // ============================================================================
    // PRACTICAL EXAMPLES
    // ============================================================================
    println!("=== Practical Examples ===");

    // System information
    println!("\n--- System Information ---");
    run_command_simple("uname", &["-a"]);

    // Date and time
    println!("\n--- Current Date/Time ---");
    run_command_simple("date", &[]);

    // Disk usage
    println!("\n--- Disk Usage ---");
    run_command_simple("df", &["-h", "/"]);

    println!();

    // ============================================================================
    // PIPELINE SIMULATION
    // ============================================================================
    println!("=== Pipeline Simulation ===");

    // Simulate: echo "hello world" | wc -w
    println!("Simulating pipeline: echo 'hello world' | wc -w");

    // First command
    let echo_output = Command::new("echo")
        .arg("hello world")
        .output()
        .expect("Failed to run echo");

    // Second command, using first's output as input
    let mut wc_output = Command::new("wc")
        .arg("-w")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn wc");

    use std::io::Write;
    if let Some(mut stdin) = wc_output.stdin.take() {
        stdin
            .write_all(&echo_output.stdout)
            .expect("Failed to write to stdin");
    }

    let output = wc_output
        .wait_with_output()
        .expect("Failed to wait for wc");

    println!(
        "Word count: {}",
        String::from_utf8_lossy(&output.stdout).trim()
    );

    println!();

    // ============================================================================
    // COMMAND BUILDER
    // ============================================================================
    println!("=== Command Builder Pattern ===");

    let result = CommandBuilder::new("echo")
        .arg("Building commands")
        .env("MY_VAR", "value")
        .timeout(Duration::from_secs(5))
        .run();

    match result {
        Ok(output) => println!("Output: {}", output),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!();

    // ============================================================================
    // TASK RUNNER
    // ============================================================================
    println!("=== Task Runner ===");

    let mut runner = TaskRunner::new();

    runner.add_task(Task {
        name: "Build".to_string(),
        command: "echo".to_string(),
        args: vec!["Building project...".to_string()],
    });

    runner.add_task(Task {
        name: "Test".to_string(),
        command: "echo".to_string(),
        args: vec!["Running tests...".to_string()],
    });

    runner.add_task(Task {
        name: "Deploy".to_string(),
        command: "echo".to_string(),
        args: vec!["Deploying...".to_string()],
    });

    runner.run_all();

    println!();
    println!("=== Command Runner Demo Complete ===");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Runs a command and prints output (simplified version)
fn run_command_simple(cmd: &str, args: &[&str]) {
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
            eprint!("{}", String::from_utf8_lossy(&output.stderr));

            if !output.status.success() {
                eprintln!("Command '{}' failed with: {}", cmd, output.status);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute '{}': {}", cmd, e);
        }
    }
}

/// Runs a command with a timeout
fn run_with_timeout(cmd: &str, args: &[&str], timeout: Duration) {
    let start = Instant::now();

    match Command::new(cmd).args(args).spawn() {
        Ok(mut child) => {
            // Poll for completion
            loop {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        println!(
                            "  ✓ Command completed in {:?} with status: {}",
                            start.elapsed(),
                            status
                        );
                        return;
                    }
                    Ok(None) => {
                        // Still running
                        if start.elapsed() > timeout {
                            println!("  ✗ Timeout! Killing process...");
                            let _ = child.kill();
                            let _ = child.wait();
                            return;
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(e) => {
                        eprintln!("  ✗ Error waiting for child: {}", e);
                        return;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to spawn command: {}", e);
        }
    }
}

// ============================================================================
// COMMAND BUILDER
// ============================================================================
// Builder pattern for constructing complex commands

struct CommandBuilder {
    command: String,
    args: Vec<String>,
    envs: Vec<(String, String)>,
    timeout: Option<Duration>,
}

impl CommandBuilder {
    fn new(command: &str) -> Self {
        CommandBuilder {
            command: command.to_string(),
            args: Vec::new(),
            envs: Vec::new(),
            timeout: None,
        }
    }

    fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    fn env(mut self, key: &str, value: &str) -> Self {
        self.envs.push((key.to_string(), value.to_string()));
        self
    }

    fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    fn run(self) -> Result<String, String> {
        let mut cmd = Command::new(&self.command);

        for arg in &self.args {
            cmd.arg(arg);
        }

        for (key, value) in &self.envs {
            cmd.env(key, value);
        }

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

// ============================================================================
// TASK RUNNER
// ============================================================================
// Manages and executes multiple tasks

struct Task {
    name: String,
    command: String,
    args: Vec<String>,
}

struct TaskRunner {
    tasks: Vec<Task>,
}

impl TaskRunner {
    fn new() -> Self {
        TaskRunner { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn run_all(&self) {
        println!("Running {} tasks...\n", self.tasks.len());

        for (i, task) in self.tasks.iter().enumerate() {
            println!("Task {}/{}: {}", i + 1, self.tasks.len(), task.name);

            let start = Instant::now();

            let args: Vec<&str> = task.args.iter().map(|s| s.as_str()).collect();

            match Command::new(&task.command).args(&args).output() {
                Ok(output) => {
                    if output.status.success() {
                        println!("  ✓ Success ({:?})", start.elapsed());
                        print!("  {}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        println!("  ✗ Failed ({:?})", start.elapsed());
                        eprint!("  {}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(e) => {
                    println!("  ✗ Error: {}", e);
                }
            }

            println!();
        }
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. PROCESS SPAWNING
//    - On Unix: fork() creates child process, exec() replaces it with new program
//    - On Windows: CreateProcess() creates child directly
//    - Rust abstracts this with Command API
//    - Process creation takes ~1-2ms on modern systems
//
// 2. FILE DESCRIPTORS
//    - stdin (0), stdout (1), stderr (2) are inherited by default
//    - Stdio::piped() creates pipe (OS buffer, typically 64KB)
//    - Stdio::null() redirects to /dev/null (discards output)
//    - Stdio::inherit() uses parent's file descriptors
//
// 3. EXIT CODES
//    - 0 means success, non-zero means failure
//    - Specific codes: 1 (general), 2 (misuse), 127 (not found), etc.
//    - On Unix: process can be killed by signal (no exit code)
//    - Rust exposes both exit codes and signals via ExitStatus
//
// 4. ENVIRONMENT VARIABLES
//    - Child inherits parent's environment by default
//    - env() adds/overrides variables
//    - env_clear() starts with empty environment
//    - Stored as key-value strings in OS
//
// 5. WAITING
//    - wait() blocks until child exits
//    - try_wait() checks if child exited without blocking
//    - On Unix: uses waitpid() system call
//    - Zombie processes if you don't wait (process table leak)
//
// 6. PIPES
//    - Circular buffer in kernel
//    - Writer blocks if buffer full
//    - Reader blocks if buffer empty
//    - Automatically closed when process exits

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Command::new() creates a command builder
// 2. .arg() adds arguments (safe from shell injection)
// 3. .output() runs command and captures stdout/stderr
// 4. .spawn() starts command without waiting
// 5. .status() waits for completion without capturing output
// 6. Check output.status.success() for exit code
// 7. Environment variables and working directory are configurable
// 8. Parallel execution is easy with threads
// 9. Timeouts require manual polling with try_wait()
// 10. Always handle errors - commands can fail in many ways

// ============================================================================
// SECURITY CONSIDERATIONS
// ============================================================================
// SHELL INJECTION:
//   - NEVER pass user input to sh -c
//   - Use Command::new() with separate args
//   - Each arg is passed directly to process (no shell parsing)
//
// PATH INJECTION:
//   - Don't construct paths from user input
//   - Validate/sanitize file paths
//   - Use absolute paths when possible
//
// ENVIRONMENT POLLUTION:
//   - Be careful inheriting environment variables
//   - Use env_clear() for sensitive commands
//   - Validate environment variable values
//
// RESOURCE LIMITS:
//   - Limit number of concurrent processes
//   - Set timeouts for long-running commands
//   - Monitor memory usage (each process uses MB+)

// ============================================================================
// CROSS-PLATFORM CONSIDERATIONS
// ============================================================================
// UNIX (Linux, macOS):
//   - Commands: ls, grep, cat, sh, bash
//   - Path separator: /
//   - Line endings: \n
//
// WINDOWS:
//   - Commands: dir, findstr, type, cmd, powershell
//   - Path separator: \
//   - Line endings: \r\n
//
// PORTABLE CODE:
//   - Use std::env::consts::OS to detect platform
//   - Avoid shell-specific features
//   - Use Rust libraries instead of commands when possible
//   - Test on all target platforms

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Not checking exit codes (command failed silently)
// ❌ Using sh -c with user input (shell injection)
// ❌ Forgetting to wait() on spawned processes (zombies)
// ❌ Not handling command not found errors
// ❌ Assuming commands exist on all systems
// ❌ Not setting working directory for relative paths
// ❌ Deadlocking on pipe buffers (writing without reading)
// ❌ Not using --release for benchmarking command runners

// ============================================================================
// ALTERNATIVES TO COMMAND SPAWNING
// ============================================================================
// When NOT to use Command:
//   - If a Rust crate exists (e.g., use reqwest not curl)
//   - For file operations (use std::fs, not ls/cat)
//   - For text processing (use Rust regex, not grep/sed)
//   - For JSON/TOML/YAML (use serde, not jq/yq)
//
// When TO use Command:
//   - Interfacing with external tools (git, docker, npm)
//   - Running user-provided scripts
//   - System administration tasks
//   - CI/CD pipelines
//   - Build tools and task runners
