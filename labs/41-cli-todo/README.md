# Project 38: CLI To-Do App

## Overview
Build a command-line to-do list application with the clap crate. Learn argument parsing, subcommands, file persistence, and how to create professional CLI tools in Rust.

## Concepts Taught
- **clap crate** for argument parsing
- **Subcommands** (add, list, complete, remove)
- **File I/O** for persistence
- **JSON serialization** with serde
- **Error handling** with Result
- **User input validation**
- **CLI design patterns**
- **Derive macros** for command parsing

## Why CLI Applications?

Command-line tools are essential for:
- **Developer productivity**: Quick task management, automation
- **System administration**: Configuration, monitoring
- **DevOps**: CI/CD scripts, deployment tools
- **Data processing**: Parsing, transformation, analysis

### Popular CLI Tools in Rust

- **ripgrep (rg)**: Fast grep alternative
- **fd**: Fast find alternative
- **bat**: Cat with syntax highlighting
- **exa**: Modern ls replacement
- **starship**: Cross-shell prompt
- **cargo**: Rust's package manager

## Why clap?

clap (Command Line Argument Parser) is the most popular Rust CLI framework:

### Advantages
- **Derive macros**: Define CLI structure with attributes
- **Automatic help generation**: --help and usage messages
- **Subcommands**: Like git (git add, git commit)
- **Type safety**: Arguments validated at compile time
- **Completions**: Generate shell completions
- **Colored output**: Beautiful error messages

### Alternatives
- **structopt**: Older, merged into clap v3
- **argh**: Lightweight, smaller compile times
- **pico-args**: Minimal, manual parsing

## Running This Project

```bash
cd 38-cli-todo
cargo build --release

# Add dependencies to Cargo.toml first:
# [dependencies]
# clap = { version = "4.0", features = ["derive"] }
# serde = { version = "1.0", features = ["derive"] }
# serde_json = "1.0"

# Run the app
./target/release/cli-todo add "Buy groceries"
./target/release/cli-todo add "Write Rust code"
./target/release/cli-todo list
./target/release/cli-todo complete 1
./target/release/cli-todo remove 2
```

## CLI Usage

```bash
# Add a task
todo add "Task description"

# List all tasks
todo list

# Mark task as complete
todo complete <id>

# Remove a task
todo remove <id>

# Clear all tasks
todo clear
```

## Data Persistence

Tasks are saved to `~/.todo.json`:
```json
[
  {
    "id": 1,
    "description": "Buy groceries",
    "completed": false,
    "created_at": "2025-11-15T10:30:00Z"
  },
  {
    "id": 2,
    "description": "Write Rust code",
    "completed": true,
    "created_at": "2025-11-15T11:00:00Z"
  }
]
```

## Performance Considerations

**File I/O**:
- Read entire file into memory: O(n) where n = file size
- Deserialize JSON: O(n) where n = number of tasks
- Write file: O(n) to serialize and write

For small to-do lists (< 10,000 tasks), performance is excellent.

**Optimization Strategies**:
1. **Lazy loading**: Only load tasks when needed
2. **Append-only**: Append new tasks instead of rewriting entire file
3. **SQLite**: For large datasets, use a database
4. **Memory mapping**: Use memmap for very large files

## Comparison: Rust vs Other Languages

| Feature | Rust (clap) | Python (argparse) | Go (cobra) |
|---------|-------------|-------------------|------------|
| Performance | Excellent | Moderate | Excellent |
| Binary size | 2-5 MB | Requires Python | 5-10 MB |
| Startup time | < 1ms | 10-50ms (interpreter) | < 1ms |
| Type safety | Compile-time | Runtime | Compile-time |
| Error messages | Excellent | Good | Good |
| Cross-compilation | Excellent | N/A (interpreted) | Excellent |

## Additional Challenges

1. **Task Priorities**: Add high/medium/low priority levels

2. **Due Dates**: Set and display due dates for tasks

3. **Tags**: Add tags to tasks and filter by tag

4. **Search**: Search tasks by keyword

5. **Archive**: Move completed tasks to archive

6. **Undo**: Implement undo for last action

7. **Import/Export**: CSV, Markdown export

8. **Recurring Tasks**: Daily, weekly, monthly tasks

9. **Statistics**: Show completion rate, productivity metrics

10. **Sync**: Sync with cloud storage or Git

## Future Directions

- **Next**: Plugin system with trait objects (Project 39)
- **Later**: Build a web scraper CLI (Project 41)
- **Advanced**: Create a blockchain CLI wallet (Project 47)

## Expected Output

```
$ todo add "Buy groceries"
✓ Added task #1: Buy groceries

$ todo add "Write Rust code"
✓ Added task #2: Write Rust code

$ todo list
Todo List:
  [1] ☐ Buy groceries
  [2] ☐ Write Rust code

$ todo complete 1
✓ Marked task #1 as complete

$ todo list
Todo List:
  [1] ☑ Buy groceries
  [2] ☐ Write Rust code

$ todo remove 2
✓ Removed task #2
```
