# rust-edu

Rust systems programming curriculum organized as labs `01` through `60`.

## Structure

Each lab follows this layout:

```text
labs/NN-topic/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs        # student-facing stubs (`todo!()`)
│   ├── main.rs       # runnable demo
│   └── solution.rs   # reference implementation
└── tests/
    └── integration_test.rs
```

## Quick start

```bash
cargo test --workspace
cargo run -p variables-types
```

Replace `variables-types` with the package name for any lab.

## Notes

- Labs are intended to be completed sequentially.
- Integration tests target reference solutions for deterministic grading.
- Root-level conversion docs are retained as concise maintenance references.
