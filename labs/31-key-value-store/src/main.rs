// Project 28: Key-Value Store
//
// A persistent key-value database using the Bitcask storage model.
// Demonstrates append-only logs, file I/O, serialization, and crash recovery.
// This is the same model used by Riak and similar to Redis AOF.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    println!("=== Bitcask-Style Key-Value Store ===\n");

    // ============================================================================
    // INITIALIZATION
    // ============================================================================
    let db_path = "./kvstore_data";
    println!("Creating database at: {}", db_path);

    // Clean up old data for demo
    let _ = fs::remove_dir_all(db_path);

    let mut store = KeyValueStore::open(db_path)?;
    println!("Database initialized\n");

    // ============================================================================
    // BASIC OPERATIONS
    // ============================================================================
    println!("=== Basic Operations ===\n");

    // Put some values
    store.set("name".to_string(), "Alice".to_string())?;
    store.set("age".to_string(), "30".to_string())?;
    store.set("city".to_string(), "San Francisco".to_string())?;

    println!("Inserted: name=Alice, age=30, city=San Francisco");

    // Get values
    println!("Get 'name': {:?}", store.get("name")?);
    println!("Get 'age': {:?}", store.get("age")?);
    println!("Get 'city': {:?}", store.get("city")?);
    println!("Get 'nonexistent': {:?}", store.get("nonexistent")?);

    println!();

    // ============================================================================
    // UPDATE AND DELETE
    // ============================================================================
    println!("=== Update and Delete ===\n");

    // Update existing key
    store.set("age".to_string(), "31".to_string())?;
    println!("Updated age to 31");
    println!("Get 'age': {:?}", store.get("age")?);

    // Delete a key
    store.remove("city".to_string())?;
    println!("Deleted 'city'");
    println!("Get 'city': {:?}", store.get("city")?);

    println!();

    // ============================================================================
    // CRASH RECOVERY SIMULATION
    // ============================================================================
    println!("=== Crash Recovery Simulation ===\n");

    // Add more data
    for i in 0..10 {
        store.set(format!("key{}", i), format!("value{}", i))?;
    }
    println!("Inserted 10 more entries");

    // Drop the store (simulates process exit)
    drop(store);
    println!("Database closed (simulating crash)");

    // Reopen the database - should recover from log
    println!("Reopening database...");
    let mut store = KeyValueStore::open(db_path)?;
    println!("Database recovered from log file");

    // Verify data is still there
    println!("Verify 'name': {:?}", store.get("name")?);
    println!("Verify 'age': {:?}", store.get("age")?);
    println!("Verify 'key5': {:?}", store.get("key5")?);
    println!("Verify 'city' (deleted): {:?}", store.get("city")?);

    println!();

    // ============================================================================
    // COMPACTION
    // ============================================================================
    println!("=== Compaction ===\n");

    // Add more data to create dead space
    for i in 0..20 {
        store.set(format!("temp{}", i), format!("data{}", i))?;
    }
    println!("Added 20 temporary entries");

    // Delete half of them
    for i in 0..10 {
        store.remove(format!("temp{}", i))?;
    }
    println!("Deleted 10 entries (creating dead space)");

    println!("Log file stats before compaction:");
    store.print_stats();

    // Compact the log
    store.compact()?;
    println!("\nCompaction complete!");

    println!("Log file stats after compaction:");
    store.print_stats();

    // Verify data is still accessible
    println!("\nVerifying data after compaction:");
    println!("'name': {:?}", store.get("name")?);
    println!("'temp15': {:?}", store.get("temp15")?);
    println!("'temp5' (deleted): {:?}", store.get("temp5")?);

    println!("\n=== Database Demo Complete ===");

    Ok(())
}

// ============================================================================
// LOG ENTRY FORMAT
// ============================================================================
// Each entry in the log file is serialized as:
// [key_len: u32][value_len: u32][key: bytes][value: bytes]

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug, Clone)]
struct LogPointer {
    file_id: u64,
    offset: u64,
    len: u64,
}

// ============================================================================
// KEY-VALUE STORE STRUCTURE
// ============================================================================

struct KeyValueStore {
    path: PathBuf,
    index: HashMap<String, LogPointer>,
    writer: BufWriter<File>,
    readers: HashMap<u64, BufReader<File>>,
    current_file_id: u64,
    current_offset: u64,
}

impl KeyValueStore {
    /// Opens or creates a key-value store at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        let mut store = KeyValueStore {
            path: path.to_path_buf(),
            index: HashMap::new(),
            writer: BufWriter::new(File::create(path.join("data_0.log"))?),
            readers: HashMap::new(),
            current_file_id: 0,
            current_offset: 0,
        };

        // Try to recover from existing log files
        store.recover()?;

        Ok(store)
    }

    /// Recovers the index by replaying all log files
    fn recover(&mut self) -> io::Result<()> {
        // Find all log files
        let mut log_files: Vec<_> = fs::read_dir(&self.path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "log" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        if log_files.is_empty() {
            return Ok(());
        }

        log_files.sort();

        println!("Recovering from {} log file(s)...", log_files.len());

        // Replay each log file
        for log_file in &log_files {
            self.replay_log_file(log_file)?;
        }

        // Setup writer to append to the latest log file
        let latest_log = log_files.last().unwrap();
        let file_id = Self::extract_file_id(latest_log);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(latest_log)?;

        self.current_offset = file.metadata()?.len();
        self.current_file_id = file_id;
        self.writer = BufWriter::new(file);

        println!("Recovered {} keys", self.index.len());

        Ok(())
    }

    /// Replays a single log file to rebuild the index
    fn replay_log_file(&mut self, path: &Path) -> io::Result<()> {
        let file_id = Self::extract_file_id(path);
        let mut reader = BufReader::new(File::open(path)?);
        let mut offset = 0u64;

        loop {
            // Try to read the next entry
            let start_offset = offset;

            // Read length prefix
            let mut len_buf = [0u8; 8];
            match reader.read_exact(&mut len_buf) {
                Ok(_) => {}
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }

            let entry_len = u64::from_le_bytes(len_buf);
            offset += 8;

            // Read entry data
            let mut entry_buf = vec![0u8; entry_len as usize];
            reader.read_exact(&mut entry_buf)?;
            offset += entry_len;

            // Deserialize command
            let command: Command = bincode::deserialize(&entry_buf)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            // Update index based on command
            match command {
                Command::Set { key, .. } => {
                    self.index.insert(
                        key,
                        LogPointer {
                            file_id,
                            offset: start_offset,
                            len: 8 + entry_len,
                        },
                    );
                }
                Command::Remove { key } => {
                    self.index.remove(&key);
                }
            }
        }

        Ok(())
    }

    /// Sets a key-value pair (insert or update)
    pub fn set(&mut self, key: String, value: String) -> io::Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value,
        };

        let offset = self.write_command(&command)?;

        // Update index
        self.index.insert(
            key,
            LogPointer {
                file_id: self.current_file_id,
                offset,
                len: self.current_offset - offset,
            },
        );

        Ok(())
    }

    /// Gets a value by key
    pub fn get(&mut self, key: &str) -> io::Result<Option<String>> {
        // Lookup in index
        let pointer = match self.index.get(key) {
            Some(p) => p,
            None => return Ok(None),
        };

        // Get or create reader for this file
        if !self.readers.contains_key(&pointer.file_id) {
            let log_path = self.log_path(pointer.file_id);
            let reader = BufReader::new(File::open(log_path)?);
            self.readers.insert(pointer.file_id, reader);
        }

        let reader = self.readers.get_mut(&pointer.file_id).unwrap();

        // Seek to the entry
        reader.seek(SeekFrom::Start(pointer.offset))?;

        // Read length
        let mut len_buf = [0u8; 8];
        reader.read_exact(&mut len_buf)?;
        let entry_len = u64::from_le_bytes(len_buf);

        // Read entry
        let mut entry_buf = vec![0u8; entry_len as usize];
        reader.read_exact(&mut entry_buf)?;

        // Deserialize
        let command: Command = bincode::deserialize(&entry_buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        match command {
            Command::Set { value, .. } => Ok(Some(value)),
            Command::Remove { .. } => Ok(None),
        }
    }

    /// Removes a key
    pub fn remove(&mut self, key: String) -> io::Result<()> {
        if !self.index.contains_key(&key) {
            return Ok(());
        }

        let command = Command::Remove { key: key.clone() };
        self.write_command(&command)?;

        // Remove from index
        self.index.remove(&key);

        Ok(())
    }

    /// Writes a command to the log file
    fn write_command(&mut self, command: &Command) -> io::Result<u64> {
        let offset = self.current_offset;

        // Serialize command
        let data = bincode::serialize(command)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Write length prefix
        let len = data.len() as u64;
        self.writer.write_all(&len.to_le_bytes())?;

        // Write data
        self.writer.write_all(&data)?;

        // Flush to ensure durability (for demo; production might batch)
        self.writer.flush()?;

        self.current_offset += 8 + len;

        Ok(offset)
    }

    /// Compacts the log by removing deleted/outdated entries
    pub fn compact(&mut self) -> io::Result<()> {
        // Create a new compacted log file
        let new_file_id = self.current_file_id + 1;
        let new_log_path = self.log_path(new_file_id);

        let mut new_writer = BufWriter::new(File::create(&new_log_path)?);
        let mut new_offset = 0u64;
        let mut new_index = HashMap::new();

        // Collect keys first to avoid borrow checker issues
        let keys: Vec<_> = self.index.keys().cloned().collect();

        // Write all current key-value pairs to new file
        for key in keys {
            // Read the value
            let value = self.get(&key)?.unwrap();

            // Write to new log
            let command = Command::Set {
                key: key.clone(),
                value,
            };

            let data = bincode::serialize(&command)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            let len = data.len() as u64;
            new_writer.write_all(&len.to_le_bytes())?;
            new_writer.write_all(&data)?;

            let entry_len = 8 + len;

            new_index.insert(
                key.clone(),
                LogPointer {
                    file_id: new_file_id,
                    offset: new_offset,
                    len: entry_len,
                },
            );

            new_offset += entry_len;
        }

        new_writer.flush()?;

        // Switch to new log file
        self.writer = new_writer;
        self.index = new_index;
        self.current_file_id = new_file_id;
        self.current_offset = new_offset;
        self.readers.clear();

        // Delete old log files (in production, keep for backup)
        for file_id in 0..new_file_id {
            let old_log = self.log_path(file_id);
            let _ = fs::remove_file(old_log);
        }

        Ok(())
    }

    /// Prints statistics about the log file
    pub fn print_stats(&self) {
        println!("  Keys in index: {}", self.index.len());
        println!("  Current file: data_{}.log", self.current_file_id);
        println!("  Current offset: {} bytes", self.current_offset);
    }

    fn log_path(&self, file_id: u64) -> PathBuf {
        self.path.join(format!("data_{}.log", file_id))
    }

    fn extract_file_id(path: &Path) -> u64 {
        path.file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.strip_prefix("data_"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
//
// 1. FILE I/O
//    File::open() -> syscall open()
//    write_all() -> multiple write() syscalls if needed
//    BufWriter batches writes to reduce syscalls (10-100x faster!)
//
// 2. SERIALIZATION (BINCODE)
//    Zero-copy where possible (no intermediate allocations)
//    Compact binary format (~50% smaller than JSON)
//    Fast: ~1-2 GB/s serialization throughput
//
// 3. HASHMAP INDEX
//    HashMap uses SipHash (cryptographic, DoS-resistant)
//    O(1) average lookup, worst case O(n) on collision
//    Grows by 2x when >75% full
//
// 4. BUFWRITER/BUFREADER
//    Internal 8KB buffer (default)
//    Reduces syscalls: 1 syscall per 8KB instead of per write
//    Must call flush() to ensure data is written!
//
// 5. ERROR HANDLING
//    io::Result<T> = Result<T, io::Error>
//    ? operator propagates errors up the call stack
//    No exceptions - all errors are explicit in type signatures
//
// 6. DROP CLEANUP
//    When KeyValueStore is dropped:
//    - BufWriter flushes automatically
//    - Files are closed
//    - No memory leaks (all Vecs/HashMaps freed)

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Append-only logs are simple and fast
// 2. In-memory index enables O(1) lookups
// 3. Crash recovery by replaying log files
// 4. Compaction reclaims space from deleted/updated keys
// 5. BufWriter is essential for good write performance
// 6. Serialization format matters (bincode > JSON for size/speed)
// 7. Must balance: write throughput vs durability (fsync cost)
// 8. All keys must fit in RAM (limitation of this model)

// ============================================================================
// BITCASK MODEL EXPLAINED
// ============================================================================
//
// WHY APPEND-ONLY?
// - Sequential writes are 100x faster than random writes on HDD
// - Even on SSD, appending is optimal (no read-modify-write)
// - Immutable files are easy to backup/replicate
// - Simple crash recovery (just replay)
//
// WHY IN-MEMORY INDEX?
// - Disk seeks are slow (~10ms HDD, ~100μs SSD)
// - Hash lookup is ~50ns (200,000x faster than disk seek!)
// - Trade-off: RAM usage vs read performance
//
// WHEN TO COMPACT?
// - When log file exceeds size threshold (e.g., 1GB)
// - When too much dead space (e.g., >50% deleted entries)
// - During low-traffic periods (off-peak hours)
//
// LIMITS OF THIS MODEL:
// - All keys must fit in RAM
// - No range queries (would need B-tree or LSM-tree)
// - Compaction blocks writes (can be mitigated)

// ============================================================================
// PERFORMANCE ANALYSIS
// ============================================================================
//
// WRITE PERFORMANCE:
// - Append-only: ~500 MB/s on SSD (sequential write)
// - With fsync: ~100-1000 writes/sec (fsync bottleneck)
// - Without fsync: ~100,000 writes/sec (buffered)
//
// READ PERFORMANCE:
// - Hash lookup: ~50-100ns
// - File read: ~100-500μs on SSD
// - Total: ~500μs per read (2000 reads/sec per thread)
//
// MEMORY:
// - Index: ~100 bytes per key (depends on key length)
// - 1 million keys: ~100 MB
// - 10 million keys: ~1 GB
//
// COMPACTION:
// - I/O: Read all live data + write new file
// - 1 GB of live data: ~2 seconds on SSD
// - During compaction: Reads served from old files

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting to flush BufWriter
//    Data sits in buffer, not on disk!
//    Fix: Call flush() or drop writer
//
// ❌ Not handling partial writes
//    write() might write less than requested
//    Fix: Use write_all() instead
//
// ❌ Assuming fsync is cheap
//    fsync() takes 1-10ms, limits throughput
//    Fix: Batch writes, use group commit
//
// ❌ Not versioning serialization format
//    Changing Entry struct breaks old log files
//    Fix: Add version field, support migration
//
// ❌ Forgetting to close files
//    File descriptor leak
//    Fix: Rust's Drop trait handles this automatically!

// ============================================================================
// EXTENDING THIS IMPLEMENTATION
// ============================================================================
//
// PRODUCTION IMPROVEMENTS:
//
// 1. CONCURRENT ACCESS
//    Use RwLock<HashMap> for index
//    Multiple readers, single writer
//
// 2. WRITE-AHEAD LOG (WAL)
//    Write to WAL before updating index
//    Enables atomic multi-key updates (transactions)
//
// 3. SNAPSHOTS
//    Periodically save index to disk
//    Faster recovery (don't replay entire log)
//
// 4. REPLICATION
//    Ship log entries to replica servers
//    Master-slave or multi-master
//
// 5. COMPRESSION
//    Compress log entries with Snappy/LZ4
//    ~50-80% size reduction with minimal CPU
//
// 6. BLOOM FILTERS
//    Avoid disk reads for missing keys
//    99% accurate, tiny memory footprint
//
// 7. TTL (TIME-TO-LIVE)
//    Auto-delete keys after expiration
//    Useful for cache use cases
