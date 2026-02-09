# Project 31 - A-Log-Structured Key-Value Store

## What You're Building (Plain English)

You're building a simple database! Specifically, a "key-value store" that saves data to a file on your disk. Think of it like a dictionary or a `HashMap` that doesn't disappear when your program closes. You'll be able to `set` a value for a given key, `get` the value for a key, and `delete` a key.

The technique you'll use is called "log-structured" storage. Instead of modifying files in place, we'll just append every new command (`set` or `delete`) to the end of a log file. This is a simple, fast, and surprisingly powerful way to build a durable database. When we need to find a value, we'll read the log from beginning to end, and the *last* command for a given key wins.

## New Rust Concepts in This Project

-   **File I/O**: You'll learn how to read from and write to files using `std::fs`. We'll use `OpenOptions` to control how we open files (e.g., for appending).

-   **Serialization with Serde**: Serde is the most popular serialization framework in Rust. You'll use it to convert your Rust structs (representing commands) into a format that can be written to a file (like JSON) and then back again. You'll use `#[derive(Serialize, Deserialize)]`.

-   **Error Handling with `Result`**: File and network operations can fail. You'll use Rust's `Result<T, E>` enum extensively to handle potential errors gracefully, like "file not found" or "permission denied".

-   **`HashMap` for in-memory indexing**: To avoid re-reading the entire file every single time you `get` a key, you'll build an in-memory "index" (a `HashMap`) that maps keys to their locations in the log file on disk.

-   **Log Compaction**: Over time, the log file will get bloated with old, outdated values. You'll implement a `compact()` method to clean up the log, creating a new, smaller log file with only the most recent value for each key.

## Rust Syntax You'll See

```rust
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader};
use serde::{Serialize, Deserialize};

// A command in our log file
#[derive(Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Delete { key: String },
}

// Writing a command to the log
let command = Command::Set { key: "name".into(), value: "Rust".into() };
let json_string = serde_json::to_string(&command)?;
// file.write_all(json_string.as_bytes())?;

// Reading commands from the log
let file = File::open("database.log")?;
let reader = BufReader::new(file);

for line in reader.lines() {
    let line = line?;
    let command: Command = serde_json::from_str(&line)?;
    // ... process the command
}

// Using HashMap for an index
let mut index = std::collections::HashMap::new();
index.insert("name".to_string(), 12345); // Map key to file offset
```

## How to Run

```bash
# Run the main binary (executes src/main.rs for a demo)
cargo run -p key-value-store

# Run the tests (checks your implementation)
cargo test -p key-value-store

# Check if code compiles without running
cargo check -p key-value-store
```

## The Exercises

You will implement the `KvStore`.

1.  **`Command` Enum**: Create an enum that represents the possible operations: `Set { key: String, value: String }` and `Delete { key: String }`. Derive `Serialize` and `Deserialize`.

2.  **`KvStore` Struct**: The struct will hold the path to the log file and the in-memory `HashMap` index.

3.  **`open()`**: A constructor that takes a path. It should open the log file (or create it if it doesn't exist), read all the commands, and build the in-memory index from them.

4.  **`set()`**:
    -   Create a `Command::Set`.
    -   Serialize it to a JSON string.
    -   Append the JSON string as a new line to the log file.
    -   Update the in-memory index with the new file offset for the key.

5.  **`get()`**:
    -   Use the in-memory index to find the file offset for the key.
    -   If the key is in the index, seek to that position in the log file.
    -   Read the command from that line, deserialize it, and return the value.
    -   If the key is not in the index, return `None`.

6.  **`delete()`**:
    -   Create a `Command::Delete`.
    -   Append it to the log file.
    -   Remove the key from the in-memory index.

7.  **`compact()`**: This is the most complex part.
    -   Create a new, temporary log file.
    -   Iterate through the *values* of your in-memory index (which are the file offsets of the most recent values).
    -   For each offset, read the `Command::Set` from the old log file and write it to the new log file.
    -   Atomically replace the old log file with the new one.
    -   Rebuild the index to point to the new offsets in the compacted file.

## Solution Explanation (No Code - Just Ideas)

**Core Idea**: The log is the source of truth. The `HashMap` is just a disposable cache (index) to make reads fast. We can always rebuild the `HashMap` by re-reading the log.

**`set("a", "1")`**:
1.  Append `{"Set":{"key":"a","value":"1"}}` to `db.log`.
2.  Remember that "a" is now at this new position. Store this in the `HashMap`.

**`get("a")`**:
1.  Look up "a" in the `HashMap`.
2.  It tells us "a" is at byte position X in `db.log`.
3.  Go directly to byte X, read the line `{"Set":{"key":"a","value":"1"}}`.
4.  Parse it and return "1". This avoids scanning the whole file.

**`compact()`**:
Imagine your log looks like this:
`{"Set":{"key":"a","value":"1"}}`
`{"Set":{"key":"b","value":"2"}}`
`{"Set":{"key":"a","value":"3"}}` (a is updated)
`{"Delete":{"key":"b"}}` (b is deleted)

Your `HashMap` index only knows about the latest state: `a` points to the 3rd line, and `b` isn't in the index.

The compaction process would:
1.  See that `a`'s latest version is on line 3.
2.  Read that line and write `{"Set":{"key":"a","value":"3"}}` to a *new* file.
3.  See that `b` is deleted, so it does nothing with `b`.
4.  The new file is now much smaller. Swap it with the old one.

## Where Rust Shines

-   **Error Handling**: `?` operator makes handling I/O and parsing errors clean and robust. It's impossible to forget to handle an error.
-   **Serde**: Provides compile-time checks for serialization and deserialization, making your on-disk format reliable.
-   **Ownership and Lifetimes**: The `KvStore` struct will own the `File` handle and the `HashMap`, ensuring they are cleaned up correctly when the store goes out of scope.
-   **Performance**: Rust's performance is on par with C++, making it suitable for building high-performance databases. The log-structured approach is I/O efficient (fast appends).

## Common Beginner Mistakes

1.  **File Locking**: A simple implementation won't handle multiple processes using the same database file. Real databases use a "lock file" to prevent this. We'll ignore this for simplicity.
2.  **Inefficient `get`**: Reading the file line-by-line for every `get` is too slow. The key is to build the in-memory index on startup.
3.  **Forgetting `fsync`**: When you write to a file, the operating system might buffer it. For true durability, you need to call `file.sync_all()` to ensure the data is written to disk, though we may omit this for simplicity in this lab.
4.  **Non-atomic Compaction**: Simply writing to a new file and then renaming it isn't fully atomic on all platforms. `std::fs::rename` is *usually* atomic on POSIX systems if the source and destination are on the same filesystem.

Good luck building your database! 🦀