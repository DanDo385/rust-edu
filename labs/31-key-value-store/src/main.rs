//! # Key-Value Store Demo

use key_value_store::solution::KvStore;

fn main() {
    println!("=== Key-Value Store Demo ===\n");

    let path = std::env::temp_dir().join("kv_demo.log");
    let mut store = KvStore::open(&path).expect("open store");

    store.set("name".to_string(), "rust".to_string()).expect("set");
    let value = store.get("name".to_string()).expect("get");

    println!("path: {}", path.display());
    println!("name => {:?}", value);
}
