//! # A Log-Structured Key-Value Store - Your Implementation
//!
//! This project tasks you with building a simple key-value store that persists
//! data to a file using a log-structured approach.
//!
//! ## Your Task
//!
//! Implement the `KvStore` and its associated methods.
//!
//! 1.  **`Command` Enum**: Define the `Command` enum with variants for `Set` and
//!     `Delete`. Remember to derive `Serialize` and `Deserialize` from Serde.
//!
//! 2.  **`KvStore` Struct**: Define the struct to hold the file path, the file
//!     handle, and the in-memory `HashMap` index.
//!
//! 3.  **`open()`**: The constructor. It should:
//!     - Open the log file for reading and appending. Create it if it doesn't exist.
//!     - Populate the in-memory index by reading the entire log file.
//!
//! 4.  **`set()`**: Write a `Command::Set` to the log, then update the index.
//!
//! 5.  **`get()`**: Read the value for a key from the log file using the index.
//!
//! 6.  **`delete()`**: Write a `Command::Delete` to the log, then remove the key
//!     from the index.
//!
//! 7.  **`compact()`**: Perform log compaction to remove redundant entries.
//!
//! ## Running Your Code
//!
//! ```bash
//! cargo test -p key-value-store
//! cargo run -p key-value-store
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

// An error type for the key-value store.
// We've defined this for you to make error handling simpler.
#[derive(Debug)]
pub enum KvError {
    Io(io::Error),
    Serde(serde_json::Error),
    KeyNotFound,
}

// Implement `From` traits to automatically convert common errors into our `KvError`.
impl From<io::Error> for KvError {
    fn from(err: io::Error) -> KvError {
        KvError::Io(err)
    }
}

impl From<serde_json::Error> for KvError {
    fn from(err: serde_json::Error) -> KvError {
        KvError::Serde(err)
    }
}

pub type Result<T> = std::result::Result<T, KvError>;

// TODO: Define the Command enum
// It should have two variants:
// - Set { key: String, value: String }
// - Delete { key: String }
// Derive `Serialize` and `Deserialize`
// #[derive(Serialize, Deserialize, Debug)]
// pub enum Command { ... }


// TODO: Define the KvStore struct
// It needs fields for:
// - The path to the log file (`PathBuf`)
// - A reader and a writer for the file
// - The in-memory index (`HashMap<String, u64>`) where u64 is the file offset
pub struct KvStore {
    // path: PathBuf,
    // reader: BufReader<File>,
    // writer: BufWriter<File>,
    // index: HashMap<String, u64>,
}

impl KvStore {
    /// Opens a `KvStore` at a given path.
    ///
    /// This will create a new log file if one doesn't exist. It populates the
    /// in-memory index by reading the existing log file.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        // TODO: Implement the open method.
        // 1. Use `OpenOptions` to open the file with read, append, and create permissions.
        // 2. Create a `BufReader` and `BufWriter` for the file.
        // 3. Create an empty `HashMap` for the index.
        // 4. Call a helper function `build_index` to populate the map.
        todo!("Open the log file and build the in-memory index")
    }

    /// Sets a key-value pair.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // TODO: Implement the set method.
        // 1. Create a `Command::Set`.
        // 2. Get the current position of the writer (this will be the offset).
        // 3. Serialize the command to a JSON string.
        // 4. Write the JSON string to the file, followed by a newline.
        // 5. `flush()` the writer to ensure it's written to disk (or the OS buffer).
        // 6. Update the index with the key and the offset.
        todo!("Write a Set command to the log and update the index");
    }

    /// Gets a value for a given key.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // TODO: Implement the get method.
        // 1. Look up the key in the index.
        // 2. If it's not found, return `Ok(None)`.
        // 3. If found, use `seek` on the reader to go to the offset.
        // 4. Read the line from the file.
        // 5. Deserialize the line into a `Command`.
        // 6. If it's a `Command::Set`, return its value.
        // 7. If it's anything else, it's an inconsistency; maybe return an error.
        todo!("Read a value from the log using the index");
    }

    /// Deletes a key.
    pub fn delete(&mut self, key: String) -> Result<()> {
        // TODO: Implement the delete method.
        // 1. Check if the key exists in the index. If not, return `Err(KvError::KeyNotFound)`.
        // 2. Create a `Command::Delete`.
        // 3. Serialize and write it to the log file (like in `set`).
        // 4. Remove the key from the in-memory index.
        todo!("Write a Delete command to the log and remove from the index");
    }

    /// Compacts the log file.
    pub fn compact(&mut self) -> Result<()> {
        // TODO: Implement log compaction.
        // 1. Create a new temporary log file.
        // 2. Iterate through the `values()` of your index (the offsets).
        // 3. For each offset, read the command from the old log file.
        // 4. Write that command to the new temporary log file.
        // 5. Replace the old log file with the new one.
        // 6. Re-open the file handle and rebuild the index for the new offsets.
        todo!("Compact the log file to remove redundant entries");
    }
}


// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;
