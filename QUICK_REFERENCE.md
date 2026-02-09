# Quick Reference

## Per-lab checks

```bash
cargo fmt
cargo test -p <package-name>
```

## Workspace checks

```bash
cargo test --workspace
```

## Required files per lab

- `Cargo.toml`
- `README.md`
- `src/lib.rs`
- `src/main.rs`
- `src/solution.rs` (or `src/solution/`)
- `tests/integration_test.rs`
