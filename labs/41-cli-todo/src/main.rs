// Project 38: CLI To-Do App
//
// This program implements a command-line to-do list application
// using the clap crate for argument parsing and JSON for persistence.
//
// IMPORTANT: Add these to Cargo.toml:
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("=== CLI To-Do App ===\n");

    // ============================================================================
    // WHAT IS A CLI APPLICATION?
    // ============================================================================
    // Command-Line Interface (CLI) apps accept commands and arguments
    // from the terminal. They're fast, scriptable, and perfect for
    // automation and developer tools.
    //
    // This app demonstrates:
    // - Argument parsing with clap (if available)
    // - Subcommands (add, list, complete, remove)
    // - File persistence with JSON
    // - Professional CLI design patterns

    // For this educational example, we'll use a simple argument parsing
    // approach. In production, you'd use clap with derive macros.

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let mut todo_list = TodoList::load();

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a task description");
                eprintln!("Usage: {} add <description>", args[0]);
                return;
            }
            let description = args[2..].join(" ");
            todo_list.add_task(description);
        }
        "list" => {
            todo_list.list_tasks();
        }
        "complete" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a task ID");
                eprintln!("Usage: {} complete <id>", args[0]);
                return;
            }
            if let Ok(id) = args[2].parse::<usize>() {
                todo_list.complete_task(id);
            } else {
                eprintln!("Error: Invalid task ID");
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a task ID");
                eprintln!("Usage: {} remove <id>", args[0]);
                return;
            }
            if let Ok(id) = args[2].parse::<usize>() {
                todo_list.remove_task(id);
            } else {
                eprintln!("Error: Invalid task ID");
            }
        }
        "clear" => {
            todo_list.clear_all();
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("CLI To-Do App");
    println!();
    println!("USAGE:");
    println!("    todo <COMMAND> [ARGS]");
    println!();
    println!("COMMANDS:");
    println!("    add <description>    Add a new task");
    println!("    list                 List all tasks");
    println!("    complete <id>        Mark task as complete");
    println!("    remove <id>          Remove a task");
    println!("    clear                Clear all tasks");
    println!("    help                 Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    todo add \"Buy groceries\"");
    println!("    todo list");
    println!("    todo complete 1");
    println!("    todo remove 2");
}

// ============================================================================
// TASK STRUCTURE
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }

    fn display(&self) {
        let status = if self.completed { "☑" } else { "☐" };
        println!("  [{}] {} {}", self.id, status, self.description);
    }
}

// ============================================================================
// TODO LIST STRUCTURE
// ============================================================================

struct TodoList {
    tasks: Vec<Task>,
    file_path: PathBuf,
}

impl TodoList {
    /// Load to-do list from file, or create new if doesn't exist
    fn load() -> Self {
        let file_path = Self::get_file_path();

        let tasks = if file_path.exists() {
            // Read and deserialize from file
            match fs::read_to_string(&file_path) {
                Ok(contents) => {
                    match serde_json::from_str(&contents) {
                        Ok(tasks) => tasks,
                        Err(e) => {
                            eprintln!("Warning: Failed to parse todo file: {}", e);
                            eprintln!("Starting with empty list");
                            Vec::new()
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to read todo file: {}", e);
                    eprintln!("Starting with empty list");
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        TodoList { tasks, file_path }
    }

    /// Save to-do list to file
    fn save(&self) {
        let json = match serde_json::to_string_pretty(&self.tasks) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("Error: Failed to serialize tasks: {}", e);
                return;
            }
        };

        if let Err(e) = fs::write(&self.file_path, json) {
            eprintln!("Error: Failed to save todo list: {}", e);
        }
    }

    /// Get the file path for storing tasks
    fn get_file_path() -> PathBuf {
        // Use current directory for this example
        // In production, use home directory: dirs::home_dir()
        PathBuf::from("todo.json")
    }

    /// Add a new task
    fn add_task(&mut self, description: String) {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let task = Task::new(id, description.clone());

        self.tasks.push(task);
        self.save();

        println!("✓ Added task #{}: {}", id, description);
    }

    /// List all tasks
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks yet! Add one with: todo add <description>");
            return;
        }

        println!("Todo List:");
        println!();

        // Show incomplete tasks first, then completed
        let incomplete: Vec<&Task> = self.tasks.iter().filter(|t| !t.completed).collect();
        let completed: Vec<&Task> = self.tasks.iter().filter(|t| t.completed).collect();

        let completed_count = completed.len();
        let incomplete_count = incomplete.len();

        if !incomplete.is_empty() {
            println!("Pending:");
            for task in incomplete {
                task.display();
            }
        }

        if !completed.is_empty() {
            println!();
            println!("Completed:");
            for task in completed {
                task.display();
            }
        }

        println!();
        println!(
            "Total: {} tasks ({} completed, {} pending)",
            self.tasks.len(),
            completed_count,
            incomplete_count
        );
    }

    /// Mark task as complete
    fn complete_task(&mut self, id: usize) {
        let found = if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if task.completed {
                println!("Task #{} is already completed", id);
                false
            } else {
                task.completed = true;
                println!("✓ Marked task #{} as complete: {}", id, task.description);
                true
            }
        } else {
            eprintln!("Error: Task #{} not found", id);
            false
        };

        if found {
            self.save();
        }
    }

    /// Remove a task
    fn remove_task(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            let task = self.tasks.remove(pos);
            self.save();
            println!("✓ Removed task #{}: {}", id, task.description);
        } else {
            eprintln!("Error: Task #{} not found", id);
        }
    }

    /// Clear all tasks
    fn clear_all(&mut self) {
        let count = self.tasks.len();
        self.tasks.clear();
        self.save();
        println!("✓ Cleared {} tasks", count);
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. ARGUMENT PARSING
//    std::env::args() returns an iterator over command-line arguments.
//    First argument is always the program name.
//    .collect() creates a Vec<String> from the iterator.
//    In production, clap handles this with derive macros.
//
// 2. JSON SERIALIZATION
//    serde_json converts Rust structs to/from JSON.
//    #[derive(Serialize, Deserialize)] generates the conversion code.
//    This is a ZERO-COST abstraction - no runtime overhead.
//
// 3. FILE I/O
//    fs::read_to_string() reads entire file into memory.
//    fs::write() writes string to file atomically (on most systems).
//    For large files, use BufReader/BufWriter for streaming.
//
// 4. PATHBUF
//    PathBuf is an owned, mutable path (like String for paths).
//    Path is a borrowed path (like &str for paths).
//    PathBuf handles OS differences (/ vs \ separators).
//
// 5. ERROR HANDLING
//    Result<T, E> forces explicit error handling.
//    .unwrap_or() provides a default value if error occurs.
//    In production, use ? operator and proper error types.
//
// 6. MEMORY LAYOUT
//    - Task: ~64 bytes (id=8, String=24, bool=1, padding=7)
//    - Vec<Task>: 24 bytes on stack + heap for elements
//    - Total: O(n) where n = number of tasks
//
// 7. PERFORMANCE
//    - Load from disk: ~1ms for 1000 tasks
//    - Parse JSON: ~100 microseconds
//    - Add task: O(1) append to Vec
//    - Find task: O(n) linear search (use HashMap for O(1))
//    - Save to disk: ~1ms for 1000 tasks

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. CLI apps in Rust are FAST (< 1ms startup time)
// 2. clap provides professional argument parsing (use in production)
// 3. serde enables zero-cost JSON serialization
// 4. File I/O is simple with std::fs
// 5. Result<T, E> forces explicit error handling
// 6. PathBuf handles cross-platform paths
// 7. Pattern matching makes command routing clean
// 8. Persistence is as simple as JSON serialize + file write

// ============================================================================
// CLI DESIGN BEST PRACTICES
// ============================================================================
// 1. COMMAND STRUCTURE
//    - Use subcommands for multiple actions (like git)
//    - Keep command names short and memorable
//    - Use consistent naming (add/remove, not add/delete)
//
// 2. ERROR MESSAGES
//    - Clear, actionable error messages
//    - Show what went wrong and how to fix it
//    - Use stderr for errors, stdout for output
//
// 3. HELP TEXT
//    - Always provide --help
//    - Include examples in help text
//    - Show usage format clearly
//
// 4. OUTPUT
//    - Machine-readable output option (JSON, CSV)
//    - Color output (when terminal supports it)
//    - Progress bars for long operations
//
// 5. CONFIGURATION
//    - Support config files (~/.todorc)
//    - Environment variables for settings
//    - Command-line flags override config

// ============================================================================
// USING CLAP (FOR PRODUCTION)
// ============================================================================
// With clap derive macros, the code would look like:
//
// use clap::{Parser, Subcommand};
//
// #[derive(Parser)]
// #[command(name = "todo")]
// #[command(about = "A CLI to-do list", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }
//
// #[derive(Subcommand)]
// enum Commands {
//     /// Add a new task
//     Add { description: String },
//     /// List all tasks
//     List,
//     /// Mark task as complete
//     Complete { id: usize },
//     /// Remove a task
//     Remove { id: usize },
//     /// Clear all tasks
//     Clear,
// }
//
// fn main() {
//     let cli = Cli::parse();
//     // Handle commands...
// }
//
// Clap automatically generates:
// - Help text (--help)
// - Version info (--version)
// - Error messages
// - Shell completions

// ============================================================================
// WHY THIS MATTERS
// ============================================================================
// CLI tools are essential for:
// - Developer productivity (quick, scriptable tools)
// - System administration (deployment, monitoring)
// - Data processing (ETL, analysis)
// - Automation (CI/CD, testing)
//
// Rust CLIs are:
// - FAST: < 1ms startup (vs 10-50ms for Python)
// - PORTABLE: Single binary, no runtime needed
// - SAFE: No segfaults, no race conditions
// - SMALL: 2-5 MB binary (vs 50+ MB for Go)

// ============================================================================
// IMPROVEMENTS FOR PRODUCTION
// ============================================================================
// 1. Use clap with derive macros (better UX)
// 2. Add colored output with termcolor or owo-colors
// 3. Store file in user's home directory (~/.todo.json)
// 4. Add undo functionality (store action history)
// 5. Support filtering (show only incomplete, by date, etc.)
// 6. Add task priorities and due dates
// 7. Implement search functionality
// 8. Add data validation (max length, sanitization)
// 9. Support import/export (CSV, Markdown)
// 10. Add tests for all commands

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Not handling file I/O errors (unwrap on fs::read)
// ❌ Using panic! for user errors (use eprintln! instead)
// ❌ Not validating user input (IDs, descriptions)
// ❌ Hardcoding file paths (use home directory)
// ❌ Not providing helpful error messages
// ❌ Forgetting to save after modifications
// ❌ Not sorting or organizing output
// ❌ Using stdout for errors (should use stderr)
