# Project 34: Command Runner

## Overview
Build a tool that executes multiple shell commands in parallel, captures their output, and reports results. This project demonstrates process spawning, I/O redirection, concurrent command execution, and practical systems programming with `std::process::Command`.

## Concepts Taught
- **Process spawning**: using `std::process::Command`
- **Child process management**: waiting, killing, status codes
- **I/O redirection**: capturing stdout/stderr
- **Concurrent execution**: running multiple commands in parallel
- **Exit codes**: understanding command success/failure
- **Environment variables**: passing to child processes
- **Working directory**: controlling where commands run
- **Timeouts**: killing long-running commands
- **Error handling**: dealing with command failures

## Why Process Spawning Works

### Operating System Processes
When you spawn a process:
1. OS creates a new process (fork + exec on Unix)
2. Child process has its own memory space
3. Parent can wait for child or run concurrently
4. Child's exit code reports success/failure

### Command vs Shell
```rust
// Direct execution (safer, faster)
Command::new("ls").arg("-la")

// Shell execution (more flexible, less safe)
Command::new("sh").arg("-c").arg("ls -la | grep foo")
```

**Direct**: No shell injection vulnerabilities
**Shell**: Supports pipes, redirection, wildcards

## Why Rust Behaves This Way

### Type-Safe Command Building
```rust
Command::new("git")
    .arg("commit")
    .arg("-m")
    .arg("message")  // Each arg is separate (no shell parsing)
```

Benefits:
- **No shell injection**: arguments aren't interpreted
- **Explicit**: each argument is clear
- **Safe**: compiler ensures correct API usage

**Comparison with other languages:**
- **Python**: `subprocess.run()` - similar safety with list args
- **Go**: `exec.Command()` - similar approach
- **Node.js**: `child_process.spawn()` - can be unsafe with shell: true
- **Bash**: Everything is a string (injection risks)

### Output Capture
```rust
let output = Command::new("ls")
    .output()  // Blocks until command completes
    .expect("Failed to execute");
```

Options:
- `output()`: Capture stdout/stderr, wait for completion
- `spawn()`: Start command, don't wait
- `status()`: Wait for completion, no capture
- `stdout(Stdio::piped())`: Custom I/O redirection

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Not Checking Exit Codes
```rust
let output = Command::new("grep").arg("foo").output()?;
println!("{}", String::from_utf8_lossy(&output.stdout));
// ❌ Might print empty if grep failed!
```
**Fix**: Check `output.status.success()`:
```rust
if output.status.success() {
    println!("{}", String::from_utf8_lossy(&output.stdout));
} else {
    eprintln!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
}
```

### Pitfall 2: Shell Injection
```rust
let user_input = "file.txt; rm -rf /";
Command::new("sh")
    .arg("-c")
    .arg(format!("cat {}", user_input))  // ❌ DANGEROUS!
    .output()?;
```
**Fix**: Use separate arguments, not shell:
```rust
Command::new("cat").arg(user_input).output()?;  // ✅ Safe
```

### Pitfall 3: Not Handling Command Not Found
```rust
Command::new("nonexistent").output()?;  // ❌ Panics if binary doesn't exist
```
**Fix**: Check if command exists first or handle the error:
```rust
match Command::new("nonexistent").output() {
    Ok(output) => { /* ... */ }
    Err(e) => eprintln!("Failed to run command: {}", e),  // ✅ OK
}
```

### Pitfall 4: Zombie Processes
```rust
let mut child = Command::new("sleep").arg("100").spawn()?;
// ❌ If you never wait(), child becomes zombie when it finishes
```
**Fix**: Always wait or detach:
```rust
child.wait()?;  // ✅ OK
```

## Code Walkthrough

See `src/main.rs` for a detailed, commented implementation that demonstrates:
1. Basic command execution and output capture
2. Checking exit codes and handling failures
3. Running multiple commands in parallel
4. Setting environment variables and working directory
5. Implementing command timeouts
6. Piping output between commands
7. Building a practical CI/CD-like task runner

## Performance Considerations

**Process Spawning Overhead:**
- Fork + exec: ~1-2ms on modern systems
- Process cleanup: ~100-500μs
- Worth it for commands that run > 10ms

**Parallel Execution:**
- I/O-bound commands: can run 100s in parallel
- CPU-bound commands: limit to CPU core count
- Network commands: limited by bandwidth/connections

**Memory:**
- Each process: 8MB+ stack + program size
- Pipes: 64KB buffer per pipe
- Be careful spawning 1000s of processes

**Optimization Strategies:**
- Use thread pool to limit concurrent processes
- Reuse processes (like worker pools) for small tasks
- Use async I/O for many concurrent commands
- Batch commands when possible

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| Command execution | `std::process::Command` | `os/exec.Command` | `subprocess.run()` |
| Safety | Type-safe, compile-time | Type-safe, runtime | Runtime errors |
| Parallel execution | Manual threads/async | Goroutines (easy) | `concurrent.futures` |
| Output capture | Built-in | Built-in | Built-in |
| Performance | Fastest | Fast | Slower |
| Error handling | Result types | Error values | Exceptions |

## Additional Challenges

1. **Task Dependency Graph**: Run commands in order based on dependencies.

2. **Command Retry Logic**: Retry failed commands with exponential backoff.

3. **Progress Bar**: Show live progress of running commands.

4. **Log Streaming**: Stream command output to files in real-time.

5. **Docker-like Runner**: Execute commands in isolated environments.

6. **CI/CD Pipeline**: Build a GitHub Actions-like workflow runner.

7. **Interactive Commands**: Handle commands that need user input.

8. **Command History**: Track and save command execution history.

## Real-World Usage

Command runners are everywhere:
- **CI/CD**: GitHub Actions, GitLab CI, Jenkins
- **Build tools**: Make, Cargo, npm, Gradle
- **Task runners**: Task, Just, Invoke
- **Deployment**: Ansible, Fabric, Capistrano
- **Testing**: pytest, cargo test, go test
- **Development**: nodemon, cargo-watch, air
- **System administration**: Automation scripts

## Running This Project

```bash
cd 34-command-runner
cargo run
```

## Expected Output

You should see:
1. Sequential command execution with output
2. Parallel command execution (faster)
3. Exit code handling (success/failure)
4. Command timeouts in action
5. Environment variable passing
6. Working directory changes
7. Error handling for missing commands
8. Practical examples (git status, file operations, system info)
