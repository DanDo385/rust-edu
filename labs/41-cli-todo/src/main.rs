//! # CLI To-Do App - Demo
//!
//! Demonstrates a simple CLI for managing tasks on disk while keeping
//! the business logic inside `cli_todo::solution` for testability.

use cli_todo::solution::TodoList;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("=== CLI To-Do App ===\n");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    let file_path = todo_file_path();
    let mut todo_list = load_or_new(&file_path);

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a task description");
                eprintln!("Usage: {} add <description>", args[0]);
                return;
            }
            let description = args[2..].join(" ");
            todo_list.add_task(description);
            save(&todo_list, &file_path);
        }
        "list" => {
            list_tasks(&todo_list);
        }
        "complete" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a task ID");
                eprintln!("Usage: {} complete <id>", args[0]);
                return;
            }
            match args[2].parse::<usize>() {
                Ok(id) => {
                    if let Err(err) = todo_list.complete_task(id) {
                        eprintln!("Error: {}", err);
                    } else {
                        save(&todo_list, &file_path);
                    }
                }
                Err(_) => eprintln!("Error: Invalid task ID"),
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a task ID");
                eprintln!("Usage: {} remove <id>", args[0]);
                return;
            }
            match args[2].parse::<usize>() {
                Ok(id) => {
                    if let Err(err) = todo_list.remove_task(id) {
                        eprintln!("Error: {}", err);
                    } else {
                        save(&todo_list, &file_path);
                    }
                }
                Err(_) => eprintln!("Error: Invalid task ID"),
            }
        }
        "clear" => {
            todo_list.clear_all();
            save(&todo_list, &file_path);
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

fn todo_file_path() -> PathBuf {
    PathBuf::from("todo.json")
}

fn load_or_new(path: &Path) -> TodoList {
    if path.exists() {
        match fs::read_to_string(path) {
            Ok(contents) => match TodoList::from_json(&contents) {
                Ok(list) => list,
                Err(err) => {
                    eprintln!("Warning: Failed to parse todo file: {}", err);
                    TodoList::new()
                }
            },
            Err(err) => {
                eprintln!("Warning: Failed to read todo file: {}", err);
                TodoList::new()
            }
        }
    } else {
        TodoList::new()
    }
}

fn save(list: &TodoList, path: &Path) {
    match list.to_json() {
        Ok(json) => {
            if let Err(err) = fs::write(path, json) {
                eprintln!("Error: Failed to save todo list: {}", err);
            }
        }
        Err(err) => eprintln!("Error: Failed to serialize tasks: {}", err),
    }
}

fn list_tasks(list: &TodoList) {
    if list.is_empty() {
        println!("No tasks in your list yet. Add one with `todo add <desc>`.");
        return;
    }
    for task in list.get_tasks() {
        println!("{}", task.display_string());
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
