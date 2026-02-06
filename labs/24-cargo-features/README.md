# Project 21: Cargo Features

## Overview
Learn how to use Cargo feature flags for conditional compilation, enabling/disabling functionality at compile time. This project demonstrates how to create flexible libraries with optional dependencies and features.

## Concepts Taught
- **Feature flags**: Optional functionality controlled at compile time
- **Conditional compilation**: `cfg` attributes and `#[cfg(feature = "...")]`
- **Optional dependencies**: Dependencies that are only included when features are enabled
- **Cargo profiles**: Debug vs release builds, optimization levels
- **Library vs binary**: Creating a library crate with a binary example
- **Default features**: Features enabled by default

## Why Feature Flags Matter

### Compile-Time Optimization
Feature flags allow you to:
1. Remove unused code completely (zero runtime cost)
2. Reduce binary size by excluding features
3. Create modular libraries where users choose what they need
4. Support different platforms or backends

**Real-world examples:**
- `serde` with `derive` feature for proc macros
- `tokio` with features for different async components
- Database drivers with backend selection (postgres, mysql, sqlite)

### Comparison with Other Languages
- **Python**: No compile-time features (everything is runtime)
- **Go**: Build tags (`// +build linux`) for conditional compilation
- **C/C++**: `#ifdef` preprocessor directives
- **Rust**: Type-safe, Cargo-integrated feature system

## Running This Project

```bash
# Default features (json enabled)
cd 21-cargo-features
cargo run

# Without JSON feature
cargo run --no-default-features

# With XML feature
cargo run --features xml

# With both JSON and XML
cargo run --features "json,xml"

# With all features
cargo run --all-features

# Release mode (optimizations enabled)
cargo run --release
```

## Cargo.toml Configuration

The `Cargo.toml` file defines features:

```toml
[features]
default = ["json"]  # JSON enabled by default
json = ["dep:serde_json"]  # Enables serde_json dependency
xml = ["dep:quick-xml"]    # Enables quick-xml dependency
```

## Performance Considerations

**Binary size impact:**
- Minimal build: ~500 KB
- With JSON: +200 KB
- With XML: +150 KB
- With both: +350 KB

**Compilation time:**
- Features excluded = faster builds
- Each feature adds dependencies to compile

**Zero-cost abstraction:**
- Disabled features = completely removed from binary
- No runtime checks, no function pointers

## Common Use Cases

1. **Optional serialization formats**: JSON, YAML, XML, TOML
2. **Platform-specific code**: Windows, Linux, macOS features
3. **Backend selection**: Different database or network libraries
4. **Development tools**: Logging, debugging features only in debug builds
5. **Crypto algorithms**: Different hashing or encryption backends

## Beginner Pitfalls

### Pitfall 1: Forgetting to Enable Features
```bash
# ❌ Feature not enabled - code won't compile or function is missing
cargo run
```
**Fix**: Enable the feature:
```bash
cargo run --features json
```

### Pitfall 2: Feature Additive Principle
Features in Rust are **additive only**. If any dependency enables a feature, it's enabled for everyone. You cannot disable a feature that another dependency enabled.

### Pitfall 3: Optional Dependencies Not Declared
```toml
# ❌ Missing dep: prefix
json = ["serde_json"]

# ✅ Correct
json = ["dep:serde_json"]
```

## Advanced Topics

1. **Feature combinations**: Test all feature combinations in CI
2. **Weak dependencies**: Dependencies that don't force feature activation
3. **Mutually exclusive features**: Enforcing only one backend
4. **Target-specific features**: Different features per platform

## Additional Challenges

1. **Add YAML support**: Create a new feature for YAML serialization
2. **Compression feature**: Add optional compression with `flate2`
3. **Multiple backends**: Create mutually exclusive database backends
4. **Profile optimization**: Experiment with different `opt-level` settings

## Next Steps

- **Project 22**: Transaction pool with concurrency
- **Project 26**: Thread pool with work stealing
- **Project 39**: Plugin system with dynamic loading

## Expected Output

```
=== Cargo Features Demo ===

Configuration:
- JSON support: enabled
- XML support: disabled
- Build profile: debug

[Rest of output depends on enabled features...]
```
