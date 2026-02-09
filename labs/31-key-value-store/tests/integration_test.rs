//! Integration tests for Lab 31: A Log-Structured Key-Value Store
//!
//! These tests verify the `KvStore`'s functionality:
//! - `set`, `get`, `delete` operations
//! - Persistence across sessions
//! - Log compaction
//! - Error handling

use key_value_store::solution::{KvStore, Result};
use tempfile::TempDir;

// Helper function to get the path for a test database
fn get_test_db_path(temp_dir: &TempDir, name: &str) -> std::path::PathBuf {
    temp_dir.path().join(name)
}

#[test]
fn test_set_and_get_value() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test1.log");
    let mut store = KvStore::open(&path)?;

    store.set("key1".to_string(), "value1".to_string())?;
    assert_eq!(store.get("key1".to_string())?, Some("value1".to_string()));

    Ok(())
}

#[test]
fn test_overwrite_value() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test2.log");
    let mut store = KvStore::open(&path)?;

    store.set("key1".to_string(), "value1".to_string())?;
    store.set("key1".to_string(), "value2".to_string())?;
    assert_eq!(store.get("key1".to_string())?, Some("value2".to_string()));

    Ok(())
}

#[test]
fn test_get_nonexistent_key() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test3.log");
    let mut store = KvStore::open(&path)?;

    assert_eq!(store.get("key1".to_string())?, None);

    Ok(())
}

#[test]
fn test_delete_key() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test4.log");
    let mut store = KvStore::open(&path)?;

    store.set("key1".to_string(), "value1".to_string())?;
    store.delete("key1".to_string())?;
    assert_eq!(store.get("key1".to_string())?, None);

    Ok(())
}

#[test]
fn test_delete_nonexistent_key_errors() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test5.log");
    let mut store = KvStore::open(&path)?;

    let result = store.delete("key1".to_string());
    assert!(matches!(result, Err(key_value_store::solution::KvError::KeyNotFound)));

    Ok(())
}

#[test]
fn test_persistence() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test6.log");

    // First session
    let mut store1 = KvStore::open(&path)?;
    store1.set("key1".to_string(), "value1".to_string())?;
    store1.set("key2".to_string(), "value2".to_string())?;
    drop(store1);

    // Second session
    let mut store2 = KvStore::open(&path)?;
    assert_eq!(store2.get("key1".to_string())?, Some("value1".to_string()));
    assert_eq!(store2.get("key2".to_string())?, Some("value2".to_string()));

    Ok(())
}

#[test]
fn test_compaction() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test7.log");
    let mut store = KvStore::open(&path)?;

    // Create a redundant log
    store.set("key1".to_string(), "value1".to_string())?;
    store.set("key2".to_string(), "value2".to_string())?;
    store.set("key1".to_string(), "updated_value1".to_string())?;
    store.delete("key2".to_string())?;
    store.set("key3".to_string(), "value3".to_string())?;

    let original_size = std::fs::metadata(&path)?.len();

    // Compact the log
    store.compact()?;

    let compacted_size = std::fs::metadata(&path)?.len();

    // The compacted log should be smaller
    assert!(compacted_size < original_size);

    // Verify data integrity after compaction
    assert_eq!(store.get("key1".to_string())?, Some("updated_value1".to_string()));
    assert_eq!(store.get("key2".to_string())?, None);
    assert_eq!(store.get("key3".to_string())?, Some("value3".to_string()));

    // Can still write after compaction
    store.set("key4".to_string(), "value4".to_string())?;
    assert_eq!(store.get("key4".to_string())?, Some("value4".to_string()));

    Ok(())
}

#[test]
fn test_compaction_rebuilds_index_correctly() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test8.log");
    let mut store = KvStore::open(&path)?;

    store.set("a".to_string(), "1".to_string())?;
    store.set("b".to_string(), "2".to_string())?;
    store.compact()?;

    // Getting a value immediately after compaction should work,
    // proving the index was rebuilt correctly.
    assert_eq!(store.get("a".to_string())?, Some("1".to_string()));
    assert_eq!(store.get("b".to_string())?, Some("2".to_string()));

    Ok(())
}

#[test]
fn test_open_on_existing_log() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test9.log");

    {
        let mut store = KvStore::open(&path)?;
        store.set("a".to_string(), "1".to_string())?;
        store.set("b".to_string(), "2".to_string())?;
    } // store is dropped

    {
        let mut store = KvStore::open(&path)?;
        assert_eq!(store.get("a".to_string())?, Some("1".to_string()));
        assert_eq!(store.get("b".to_string())?, Some("2".to_string()));
    }

    Ok(())
}

#[test]
fn test_multiple_set_and_deletes() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let path = get_test_db_path(&temp_dir, "test10.log");
    let mut store = KvStore::open(&path)?;

    store.set("a".to_string(), "1".to_string())?;
    store.set("b".to_string(), "2".to_string())?;
    store.delete("a".to_string())?;
    assert_eq!(store.get("a".to_string())?, None);

    store.set("a".to_string(), "3".to_string())?;
    assert_eq!(store.get("a".to_string())?, Some("3".to_string()));

    Ok(())
}
