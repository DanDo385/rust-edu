# Project 42: CSV to JSON Converter

## Overview
Build a robust CSV to JSON converter using Serde, Rust's serialization framework. This project demonstrates data transformation, file I/O, serialization/deserialization, and working with structured data formats.

## Concepts Taught
- **Serde framework** for serialization/deserialization
- **csv crate** for reading CSV files
- **serde_json** for JSON output
- **Derive macros** (`#[derive(Serialize, Deserialize)]`)
- **Type-safe parsing** with custom structs
- **Error handling** for file I/O and parsing
- **Data transformation** and validation
- **Field mapping** and renaming

## Why Rust for Data Processing

### Type Safety
Rust's type system ensures your data transformations are correct at compile time. Unlike Python's pandas or Node.js CSV parsers, Rust catches schema mismatches before runtime.

### Performance
Serde is **extremely fast** - often 2-10x faster than equivalent Python or JavaScript parsers. The csv crate uses zero-copy parsing when possible.

### Memory Efficiency
Rust processes data without garbage collection pauses, making it ideal for large datasets (GBs of data).

**Comparison with other languages:**
- **Python**: pandas is powerful but slow for large files
- **Go**: Fast but more verbose serialization
- **TypeScript**: Good with json but CSV parsing is slower

## Beginner Pitfalls & Best Practices

### Pitfall 1: Forgetting derive macros
```rust
// ❌ WRONG: Struct can't be serialized without derive
struct Person {
    name: String,
    age: u32,
}
```
**Fix**: Add derive macros:
```rust
// ✅ CORRECT: Add Serialize and Deserialize
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}
```

### Pitfall 2: Field name mismatches
```rust
// ❌ WRONG: CSV has "first_name" but struct has "name"
#[derive(Deserialize)]
struct Person {
    name: String,  // Won't match CSV column "first_name"
}
```
**Fix**: Use rename attribute:
```rust
// ✅ CORRECT: Rename fields to match CSV headers
#[derive(Deserialize)]
struct Person {
    #[serde(rename = "first_name")]
    name: String,
}
```

### Pitfall 3: Not handling missing or invalid data
```rust
// ❌ WRONG: Will panic if age is missing or invalid
age: u32,
```
**Fix**: Use Option or provide defaults:
```rust
// ✅ CORRECT: Handle missing values
age: Option<u32>,

// Or with default
#[serde(default)]
age: u32,  // Defaults to 0 if missing
```

### Pitfall 4: Reading entire file into memory
```rust
// ❌ WRONG: For large files (GB+), this uses too much memory
let records: Vec<Record> = csv::Reader::from_path("huge.csv")?
    .deserialize()
    .collect()?;
```
**Fix**: Process records iteratively:
```rust
// ✅ CORRECT: Stream processing
let mut reader = csv::Reader::from_path("huge.csv")?;
for result in reader.deserialize() {
    let record: Record = result?;
    // Process one record at a time
}
```

## Code Walkthrough

See `src/main.rs` for a complete implementation that demonstrates:
1. Reading CSV files with headers
2. Deserializing into type-safe structs
3. Transforming and validating data
4. Serializing to JSON (pretty-printed and compact)
5. Handling errors gracefully
6. Working with nested structures
7. Custom field mappings and defaults
8. Streaming large files

## Serde Deep Dive

### How Serde Works
Serde is a **framework** for serialization, not just a library. It uses:
1. **Traits**: `Serialize` and `Deserialize` define how types convert to/from data
2. **Derive macros**: Automatically implement these traits
3. **Data formats**: csv, json, toml, yaml, etc. are separate crates

### Derive Macros
```rust
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,

    #[serde(rename = "user_name")]
    name: String,

    #[serde(default)]
    email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
}
```

**Attributes:**
- `rename`: Map field to different name in data
- `default`: Use Default::default() if field is missing
- `skip`: Don't serialize/deserialize this field
- `skip_serializing_if`: Conditionally skip serialization

## Performance Considerations

**CSV parsing speed**:
- 1 million rows: ~1-2 seconds
- 10 million rows: ~10-20 seconds
- Bottleneck is usually I/O, not parsing

**Memory usage**:
- Streaming (iterator): ~constant memory (few KB)
- Collecting all rows: O(n) memory (can be GBs!)

**Optimization tips**:
1. Use `BufReader` for buffered I/O
2. Process records one at a time for large files
3. Use `serde(flatten)` to reduce allocations
4. Disable pretty-printing for faster JSON serialization

**Benchmarks** (approximate):
- Parsing 100k row CSV: ~200ms
- Converting to JSON: ~100ms
- Writing JSON to disk: ~300ms (depends on disk speed)

## Comparison: Rust vs Python vs JavaScript

| Feature | Rust (serde + csv) | Python (pandas) | JavaScript (csv-parse) |
|---------|-------------------|-----------------|------------------------|
| Speed | Very fast | Slow (pure Python) | Medium |
| Type safety | Compile-time | Runtime | Runtime |
| Memory | Low overhead | High (copies data) | High (GC) |
| Large files | Excellent (streaming) | Good (chunking) | Good (streaming) |
| Error handling | Compile-time checks | Runtime exceptions | Runtime exceptions |

## Additional Challenges

1. **Bidirectional converter**: Add JSON to CSV conversion (reverse direction).

2. **Schema inference**: Automatically detect column types instead of using a fixed struct.

3. **Data validation**: Add custom validation (e.g., email format, age range).

4. **Multiple output formats**: Support XML, YAML, TOML in addition to JSON.

5. **CSV dialect detection**: Handle different delimiters (tab, semicolon, pipe).

6. **Column filtering**: Allow users to select which columns to include in output.

7. **Data aggregation**: Add grouping and summary statistics.

8. **Nested JSON**: Convert CSV with dot notation (e.g., "user.name") to nested JSON.

## Future Directions

- **Data pipelines**: Chain transformations (CSV → filter → aggregate → JSON)
- **ETL systems**: Extract-Transform-Load for data warehouses
- **API integration**: Fetch CSV from URLs, send JSON to APIs
- **Database export**: Convert CSV to SQL INSERT statements

## Running This Project

```bash
cd 42-csv-to-json
cargo run
```

**Note**: Add to `Cargo.toml`:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.3"
```

## Expected Output

The program will:
1. Create sample CSV data
2. Read and parse the CSV file
3. Display parsed records
4. Convert to JSON (both pretty and compact formats)
5. Save JSON to file
6. Demonstrate error handling for malformed data
7. Show advanced features (nested structures, field mapping)

## CSV Format Variations

### Standard CSV
```csv
name,age,city
Alice,30,NYC
Bob,25,SF
```

### CSV with quotes (for commas in data)
```csv
name,description
"Smith, John","Developer, Senior"
```

### CSV with different delimiters
```csv
name;age;city  (semicolon)
name|age|city  (pipe)
name\tage\tcity (tab)
```

### CSV without headers
```csv
Alice,30,NYC
Bob,25,SF
```

The `csv` crate handles all these variations!

## Serde Ecosystem

Popular data formats supported by Serde:
- **serde_json**: JSON
- **toml**: TOML configuration files
- **serde_yaml**: YAML
- **bincode**: Binary format (fastest)
- **csv**: CSV/TSV
- **ron**: Rusty Object Notation
- **messagepack**: Binary JSON alternative
- **xml-rs**: XML (limited serde support)

## Common Data Transformation Patterns

1. **Filtering**: Skip invalid rows
2. **Mapping**: Transform field values
3. **Enrichment**: Add calculated fields
4. **Validation**: Check constraints
5. **Normalization**: Standardize formats
6. **Aggregation**: Group and summarize
7. **Joining**: Combine multiple data sources
8. **Deduplication**: Remove duplicates
