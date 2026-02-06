# Project 28: Key-Value Store

## Overview
This project implements a persistent key-value store using the Bitcask storage model. You'll learn about append-only logs, file I/O, serialization, and how databases like Redis and Riak work under the hood. This is the foundation of many production databases.

## Concepts Taught
- **Append-only log** structure
- **File I/O** with `std::fs`
- **Serialization** with serde and bincode
- **In-memory indexing** for fast lookups
- **Write-ahead logging** (WAL)
- **Compaction** to reclaim space
- **Crash recovery** from log files
- **Error handling** with Result types

## Why Bitcask?

### The Bitcask Model (Riak's storage engine)
Bitcask is a simple but powerful storage model:
1. **Append-only log**: All writes go to the end of a file (fast!)
2. **In-memory index**: HashMap pointing to file offsets
3. **Immutable files**: Never modify existing data (safe!)
4. **Compaction**: Periodically merge old files, removing deleted/outdated keys

**Advantages:**
- **Fast writes**: O(1) append to file
- **Fast reads**: O(1) hash lookup + O(1) file read
- **Crash recovery**: Rebuild index by replaying log
- **Simple**: Easy to implement correctly
- **Predictable**: No B-tree complexity

**Disadvantages:**
- **Memory**: Must fit all keys in RAM
- **Slow iteration**: Must scan log file
- **Compaction overhead**: Periodic GC pauses

**Real-world usage:**
- **Riak**: Distributed database (uses Bitcask)
- **Redis**: Similar model (AOF persistence)
- **LevelDB/RocksDB**: LSM-tree (evolution of Bitcask)
- **FoundationDB**: Uses similar append-only logs

## Architecture

```
KeyValueStore
├── index: HashMap<String, LogPointer>
│   └── LogPointer { file_id, offset, len }
├── writer: BufWriter<File>
├── readers: HashMap<u64, BufReader<File>>
└── log_files: Vec<PathBuf>

Disk Layout:
data_0.log: [entry1][entry2][entry3]...
data_1.log: [entry4][entry5]...
            ↑
            Append-only writes
```

## Write Path

```
put("key", "value")
  ↓
Serialize to bytes
  ↓
Append to current log file
  ↓
Update in-memory index
  ↓
Optional: fsync() for durability
```

## Read Path

```
get("key")
  ↓
Lookup in HashMap
  ↓
Found: Read from file at offset
  ↓
Deserialize and return
```

## Compaction

Over time, the log accumulates deleted/outdated entries:

```
Before compaction:
[set k1=v1][set k2=v2][del k1][set k2=v3][set k3=v4]

After compaction:
[set k2=v3][set k3=v4]
```

Benefits:
- Reclaims disk space
- Faster recovery (fewer entries to replay)
- Better read performance (less data to skip)

## Beginner Pitfalls & File I/O Notes

### Pitfall 1: Not Flushing Writes
```rust
writer.write_all(&bytes)?;
// ❌ Data might still be in buffer!

writer.write_all(&bytes)?;
writer.flush()?;  // ✅ Force write to OS
```

### Pitfall 2: Losing Data on Crash
```rust
writer.write_all(&bytes)?;
// ❌ Data in OS buffer, not on disk

writer.write_all(&bytes)?;
file.sync_all()?;  // ✅ Force write to disk (fsync)
```

### Pitfall 3: Serialization Format Changes
```rust
// If you change Entry struct, old log files become unreadable!
// Solution: Version your format, support migration
```

### Pitfall 4: File Descriptor Leaks
```rust
for _ in 0..10000 {
    let file = File::open("log")?;  // ❌ Leaks file descriptors!
}
// Fix: Reuse readers or close files explicitly
```

## Code Walkthrough

See `src/main.rs` for a detailed implementation that demonstrates:
1. Creating a persistent key-value store
2. Writing and reading data
3. Crash recovery by replaying log
4. Compaction to reclaim space
5. Proper error handling with Result types
6. File I/O best practices

## Performance Considerations

### Write Performance
- **Sequential writes**: ~500 MB/s on SSD, ~100 MB/s on HDD
- **Append-only**: Optimal for disks (no seeking!)
- **BufWriter**: Batches writes (10-100x faster than unbuffered)
- **fsync() cost**: ~1-10ms per call (limits throughput to 100-1000 ops/sec)

### Read Performance
- **Index lookup**: O(1), ~50-100ns
- **File read**: ~100-500μs on SSD, ~10ms on HDD
- **BufReader**: Reduces syscalls (faster for small reads)
- **Memory mapping**: Alternative to read() (see mmap crate)

### Memory Usage
- **Index**: ~100 bytes per key (depends on key length)
- **1 million keys**: ~100 MB RAM
- **Limit**: All keys must fit in memory!

### Compaction Cost
- **I/O**: Must read old files, write new file
- **Time**: Proportional to data size (seconds to minutes)
- **During compaction**: Higher latency for concurrent operations

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|-----|--------|
| File I/O | std::fs, explicit errors | os package, similar | open(), exceptions |
| Serialization | serde (zero-copy) | encoding/gob | pickle, json |
| Error handling | Result<T, E> | (val, err) | Exceptions |
| Buffer control | BufReader/Writer | bufio | Automatic |
| Memory safety | Compile-time | Runtime panics | Runtime errors |

**Rust advantage**: Zero-copy serialization with serde, no buffer overruns.

## Additional Challenges

1. **Transactions**: Implement multi-key atomic updates using write-ahead log

2. **Range Queries**: Add a B-tree index for range scans (not just point lookups)

3. **Replication**: Implement master-slave replication by shipping log entries

4. **Snapshots**: Save index to disk for faster recovery

5. **Compression**: Compress log entries with LZ4 or Snappy

6. **TTL (Time-to-Live)**: Auto-delete keys after expiration

## Real-World Database Features

Production databases add:
- **Concurrent access**: RwLock or MVCC
- **Transactions**: ACID guarantees
- **Replication**: Master-slave, multi-master
- **Sharding**: Partition data across nodes
- **Query language**: SQL, key-value, document
- **Backup/restore**: Point-in-time recovery
- **Monitoring**: Metrics, logging, health checks

## Future Directions

- **Next**: Basic VM (Project 29)
- **Related**: Error handling (Project 8), file I/O
- **Advanced**: Build LSM-tree (LevelDB-style), add WAL, MVCC

## Running This Project

```bash
cd 28-key-value-store
cargo run
```

**Note**: Add to `Cargo.toml`:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
```

## Expected Output

You should see:
- Database initialization (creating log files)
- Writing key-value pairs to disk
- Reading values back from disk
- Crash recovery demonstration (restart and verify data)
- Compaction demonstration (merging old log files)
- Statistics (file sizes, entry counts)
