# CLAUDE.md - Rust-Edu Lab Refactor Rules

This repository is a single Rust learning project made up of many independent lab crates.

## Project Structure Contract

Every lab directory that matches `labs/NN-name` (for example, `labs/01-variables-types`) must include:

- `src/lib.rs`
- `src/main.rs`
- `src/solution.rs`
- `tests/integration_test.rs`

### Purpose of Each File

- `src/lib.rs`
  - Learner-facing implementation file.
  - Keep commentary minimal.
  - Include clear `TODO` guidance for missing steps.
  - Integration tests must target this file's public API.

- `tests/integration_test.rs`
  - Validate behavior of `lib.rs`.
  - Use simple, explicit assertions and useful failure messages.
  - Cover happy path + edge cases.

- `src/main.rs`
  - Demonstrate the `lib.rs` API in runnable form.
  - Keep commentary minimal.
  - Show representative usage and outputs.

- `src/solution.rs`
  - Reference implementation with verbose explanations.
  - Start with top-level context: what the lab builds and why it matters.
  - Explain key implementation decisions in plain language.
  - Include ownership/borrowing notes where relevant.
  - Make it easy for learners to port solution code into `lib.rs` step-by-step.

## Naming Convention

- Lab directories should use two-digit prefixes: `NN-topic-name`.
- Keep crate names and paths consistent with directory purpose.

## Refactor Priorities

1. Correctness (compiles/tests pass)
2. Consistent scaffold across all labs
3. Teaching quality in `solution.rs`
4. Minimal but clear learner guidance in `lib.rs` and `main.rs`

## Commenting Style

- `lib.rs` / `main.rs`: minimal comments + TODO prompts.
- `solution.rs`: detailed teaching commentary.
- Avoid empty or placeholder sections.

## Validation Checklist (Per Lab)

- Required files exist (`lib.rs`, `main.rs`, `solution.rs`, `tests/integration_test.rs`)
- `cargo test -p <crate>` passes
- `integration_test.rs` exercises `lib.rs`
- `main.rs` demonstrates `lib.rs`
- `solution.rs` is complete and instructional

