//! # A Command Runner - Interactive Demo
//! 
//! This binary demonstrates the command runner library by executing
//! a few sample external commands.
//! Run with: cargo run -p command-runner

use command_runner::solution::{CommandBuilder, Task, TaskRunner};
use std::time::Duration;

fn main() {
    println!("=== Command Runner Demo ===\n");

    // ============================================================================
    // DEMO 1: Simple successful command
    // ============================================================================
    println!("1. Running a simple 'echo' command...");
    let result1 = CommandBuilder::new("echo")
        .arg("Hello from the command runner!")
        .run()
        .unwrap();

    println!("   -> Exit Code: {}", result1.exit_code);
    println!("   -> Stdout: {}", result1.stdout.trim());
    println!("   -> Stderr: {}", result1.stderr.trim());
    println!();

    // ============================================================================
    // DEMO 2: A command that fails
    // ============================================================================
    println!("2. Running a command that will fail ('ls' on a nonexistent file)...");
    let result2 = CommandBuilder::new("ls")
        .arg("nonexistent_file_xyz")
        .run()
        .unwrap();

    println!("   -> Exit Code: {}", result2.exit_code);
    println!("   -> Stdout: {}", result2.stdout.trim());
    // The error message from `ls` goes to stderr.
    println!("   -> Stderr: {}", result2.stderr.trim());
    println!();

    // ============================================================================
    // DEMO 3: A command with a timeout
    // ============================================================================
    println!("3. Running a command ('sleep 3') with a 1-second timeout...");
    let result3 = CommandBuilder::new("sleep")
        .arg("3")
        .timeout(Duration::from_secs(1))
        .run();

    match result3 {
        Ok(_) => println!("   -> This should not happen!"),
        Err(e) => println!("   -> Correctly failed with error: {}", e),
    }
    println!();

    // ============================================================================
    // DEMO 4: Task Runner with a sequence of tasks
    // ============================================================================
    println!("4. Running a sequence of tasks with TaskRunner...");
    let tasks = vec![
        Task::new("Task 1: List current directory".to_string(), CommandBuilder::new("ls").arg("-l")),
        Task::new("Task 2: Echo something".to_string(), CommandBuilder::new("echo").arg("Task 2 reporting in!")),
        Task::new("Task 3: This one will fail".to_string(), CommandBuilder::new("false")),
        Task::new("Task 4: This will be skipped".to_string(), CommandBuilder::new("echo").arg("Should not see this")),
    ];

    let mut runner = TaskRunner::new(tasks);
    let results = runner.run();

    println!("\n   --- Task Runner Results ---");
    for (i, res) in results.iter().enumerate() {
        if res.is_ok() {
            println!("   ✅ Task {} succeeded", i + 1);
        } else {
            println!("   ❌ Task {} failed", i + 1);
        }
    }
    println!("   -------------------------\n");


    println!("=== Demo Complete! ===");
}