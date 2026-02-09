# Lab Conversion Guide (Maintenance)

Use this when adding or reworking a lab while preserving project conventions.

## Required lab template

```text
labs/NN-topic/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── solution.rs
└── tests/
    └── integration_test.rs
```

## Rules

1. Keep `src/lib.rs` student-facing with `todo!()` stubs.
2. Keep full logic in `src/solution.rs` (or `src/solution/*` for multi-file labs).
3. Keep `src/main.rs` as a demo that calls `solution`.
4. Keep integration tests targeting `solution`, not stubs.
5. Match tone and teaching depth from `labs/01-variables-types`.

## Validation commands

```bash
cargo test -p <package-name>
cargo test --workspace
```
