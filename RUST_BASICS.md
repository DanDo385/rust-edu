# Rust Basics Quick Reference

## Ownership

- One owner per value.
- Move transfers ownership.
- Borrow with `&T` (shared) or `&mut T` (exclusive).

## Result/Option

- `Result<T, E>` for recoverable errors.
- `Option<T>` for nullable/absent values.
- Use `?` for propagation in functions that return `Result`/`Option`.

## Collections

- `Vec<T>`: growable contiguous sequence.
- `HashMap<K, V>`: key-value lookup.
- `BTreeMap<K, V>`: ordered map.

## Concurrency

- `std::thread` for native threads.
- `Arc<T>` for shared ownership across threads.
- `Mutex<T>` for synchronized mutable access.

## Useful commands

```bash
cargo check
cargo test
cargo run -p <package-name>
```
