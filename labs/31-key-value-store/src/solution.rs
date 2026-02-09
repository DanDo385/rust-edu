//! # A Log-Structured Key-Value Store - Complete Solution
//!
//! ## What We're Building
//!
//! A `KvStore` that persists key-value data to a file. It uses a log-structured
//! design: all mutations (`set`, `delete`) are appended to a log file. An in-memory
//! `HashMap` acts as an index, mapping keys to their byte offsets in the file,
//! allowing for fast reads without scanning the entire file.
//!
//! ## Why Rust Is Perfect For This
//!
//! - **`Result` and the `?` operator**: Make I/O error handling robust and clean.
//! - **Serde**: Provides best-in-class, compile-time checked serialization.
//! - **Ownership**: `File` handles and other resources are managed automatically,
//!   preventing leaks.
//! - **Performance**: Rust's speed and control over memory make it ideal for
//!   database development.
//!
//! ## Key Rust Concepts You'll Learn
//!
//! - **`std::fs`**: `File` and `OpenOptions` for file manipulation.
//! - **`std::io`**: `BufReader`, `BufWriter`, and traits like `Read`, `Write`, `Seek`.
//! - **Serde**: `#[derive(Serialize, Deserialize)]` and `serde_json`.
//! - **`HashMap`**: For building the in-memory index.
//! - **Custom Error Types**: Creating an enum to represent possible failures.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// An error type for our Key-Value store.
///
/// This enum wraps errors from the underlying I/O and serialization libraries,
/// allowing our functions to return a single, consistent error type.
#[derive(Debug)]
pub enum KvError {
    /// An I/O error occurred (e.g., file not found, permission denied).
    Io(io::Error),
    /// A serialization or deserialization error occurred with Serde.
    Serde(serde_json::Error),
    /// The requested key was not found in the store.
    KeyNotFound,
}

/// Implement the `From` trait to allow easy conversion from `io::Error`
/// into our `KvError`. This lets us use the `?` operator on I/O operations.
impl From<io::Error> for KvError {
    fn from(err: io::Error) -> KvError {
        KvError::Io(err)
    }
}

/// Implement `From` for `serde_json::Error` as well.
impl From<serde_json::Error> for KvError {
    fn from(err: serde_json::Error) -> KvError {
        KvError::Serde(err)
    }
}

/// A specialized `Result` type for our key-value store operations.
pub type Result<T> = std::result::Result<T, KvError>;

/// Represents a command written to the log.
///
/// We derive `Serialize` and `Deserialize` so Serde can automatically
/// convert this enum to and from a format like JSON.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Delete { key: String },
}

/// A log-structured key-value store.
pub struct KvStore {
    path: PathBuf,
    // We use a `BufReader` to efficiently read from the file.
    // The file is wrapped to allow seeking.
    reader: BufReader<File>,
    // We use a `BufWriter` to efficiently write to the file.
    writer: BufWriter<File>,
    // The in-memory index mapping keys to file offsets.
    index: HashMap<String, u64>,
}

impl KvStore {
    /// Opens a `KvStore` at a given path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        // Open the file for both appending and reading.
        // `create(true)` will create it if it doesn't exist.
        let write_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&path)?;

        let read_file = File::open(&path)?;

        let reader = BufReader::new(read_file);
        let writer = BufWriter::new(write_file);

        let mut store = KvStore {
            path,
            reader,
            writer,
            index: HashMap::new(),
        };

        // Build the index from the existing log file.
        store.build_index()?;

        Ok(store)
    }

    /// Builds the in-memory index by reading the log file from the beginning.
    fn build_index(&mut self) -> Result<()> {
        let mut pos = self.reader.seek(SeekFrom::Start(0))?;
        let mut stream = serde_json::Deserializer::from_reader(&mut self.reader).into_iter::<Command>();

        while let Some(cmd_result) = stream.next() {
            let next_pos = stream.byte_offset() as u64;
            match cmd_result? {
                Command::Set { key, .. } => {
                    self.index.insert(key, pos);
                }
                Command::Delete { key } => {
                    self.index.remove(&key);
                }
            }
            pos = next_pos;
        }
        Ok(())
    }


    /// Sets a key-value pair.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set { key: key.clone(), value };

        // Seek to the end of the file to get the correct offset for the new record.
        let pos = self.writer.seek(SeekFrom::End(0))?;

        // Serialize the command to a JSON string.
        serde_json::to_writer(&mut self.writer, &command)?;
        // Write a newline to separate commands.
        self.writer.write_all(b"\n")?;
        // Flush the writer's buffer to ensure the command is written.
        self.writer.flush()?;

        // Update the in-memory index with the new position.
        self.index.insert(key, pos);

        Ok(())
    }

    /// Gets a value for a given key.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // Look up the key in our index.
        match self.index.get(&key) {
            Some(&pos) => {
                // If found, seek the reader to that position.
                self.reader.seek(SeekFrom::Start(pos))?;
                // Create a JSON stream deserializer that reads one object.
                let mut stream = serde_json::Deserializer::from_reader(&mut self.reader).into_iter::<Command>();

                // Get the next command from the stream.
                if let Some(Ok(Command::Set { value, .. })) = stream.next() {
                    Ok(Some(value))
                } else {
                    // This indicates a corrupted file or a bug.
                    Err(KvError::KeyNotFound)
                }
            }
            None => {
                // Key not in index, so it doesn't exist.
                Ok(None)
            }
        }
    }

    /// Deletes a key.
    pub fn delete(&mut self, key: String) -> Result<()> {
        // First, check if the key exists. It's more user-friendly to error
        // if the user tries to delete a non-existent key.
        if !self.index.contains_key(&key) {
            return Err(KvError::KeyNotFound);
        }

        let command = Command::Delete { key: key.clone() };

        // Append the Delete command to the log.
        serde_json::to_writer(&mut self.writer, &command)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;

        // Remove the key from the in-memory index.
        self.index.remove(&key);

        Ok(())
    }

    /// Compacts the log file to remove stale data.
    pub fn compact(&mut self) -> Result<()> {
        // Path for the new, compacted log file.
        let compact_path = self.path.with_extension("compact");
        let mut compact_writer = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&compact_path)?
        );

        let mut new_index = HashMap::new();
        let mut new_pos = 0u64;

        // Iterate through the current index. The values are the offsets
        // to the *latest* version of each key.
        for (key, &pos) in self.index.iter() {
            self.reader.seek(SeekFrom::Start(pos))?;
            let mut stream = serde_json::Deserializer::from_reader(&mut self.reader).into_iter::<Command>();

            if let Some(Ok(Command::Set { value, .. })) = stream.next() {
                // Write the latest version of the command to the new log file.
                let new_cmd = Command::Set { key: key.clone(), value };
                serde_json::to_writer(&mut compact_writer, &new_cmd)?;
                compact_writer.write_all(b"\n")?;
                
                // Add the key and its new offset to our new index.
                new_index.insert(key.clone(), new_pos);
                // Update position for the next record.
                new_pos = compact_writer.seek(SeekFrom::Current(0))?;
            }
        }
        compact_writer.flush()?;

        // Atomically replace the old log with the new one.
        std::fs::rename(&compact_path, &self.path)?;

        // Re-open our file handles and update the store's state.
        // This is simpler than trying to manage the handles in place.
        let write_file = OpenOptions::new().write(true).append(true).open(&self.path)?;
        let read_file = File::open(&self.path)?;

        self.writer = BufWriter::new(write_file);
        self.reader = BufReader::new(read_file);
        self.index = new_index;

        Ok(())
    }
}
