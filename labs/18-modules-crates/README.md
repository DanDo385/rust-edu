# Project 15: Modules and Crates

## Overview
This project teaches Rust's module system, crate organization, visibility rules, and how to structure real-world projects. You'll build a multi-module library with both library and binary components, demonstrating professional Rust project organization. Understanding the module system is crucial for building maintainable, scalable applications.

## Concepts Taught
- **Modules** - Code organization with `mod`
- **Visibility** - `pub` vs private
- **Paths** - Absolute and relative imports
- **use** keyword - Bringing items into scope
- **Crate structure** - lib.rs vs main.rs
- **Submodules** - Nested module hierarchy
- **Re-exports** - Making internal modules public
- **Workspace organization** - Multi-crate projects
- **Module file structure** - mod.rs vs named files
- **Privacy boundaries** - Encapsulation in Rust

## Why Rust Behaves This Way

### Privacy by Default
In Rust, everything is **private by default**. This is opposite to many languages:
- **Python**: Everything is public
- **Go**: Uppercase = public, lowercase = private
- **Java**: Explicit public/private/protected
- **Rust**: Private by default, `pub` to make public

**Why?** This prevents accidental API exposure and encourages deliberate interface design. You explicitly choose what to expose.

### Module System Design
Rust's module system serves multiple purposes:
1. **Code organization** - Logical grouping of related functionality
2. **Namespace management** - Avoid naming conflicts
3. **Privacy control** - Hide implementation details
4. **Compilation optimization** - Modules are compilation units

### Crates: The Compilation Unit
A **crate** is:
- The fundamental compilation unit
- Either a binary (executable) or library
- Has its own namespace
- Published to crates.io

**Key insight**: A project can have ONE library crate but MULTIPLE binary crates.

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Forgetting `pub`
```rust
// lib.rs
mod utils {
    fn helper() {}  // ❌ Private!
}

// main.rs
use mylib::utils::helper;  // ❌ ERROR: helper is private
```
**Fix**: Add `pub`:
```rust
mod utils {
    pub fn helper() {}  // ✅ Now public
}
```

### Pitfall 2: Module File Structure Confusion
```rust
// ❌ WRONG:
// src/network.rs
// src/network/server.rs  // ERROR: Can't have both!

// ✅ CORRECT Option 1 (old style):
// src/network/mod.rs
// src/network/server.rs

// ✅ CORRECT Option 2 (new style, Rust 2018+):
// src/network.rs  (with "mod server;")
// src/network/server.rs
```

### Pitfall 3: Trying to Use Private Fields
```rust
mod mymod {
    pub struct Config {
        value: String,  // ❌ Private field!
    }
}

let cfg = mymod::Config { value: "test".to_string() };  // ❌ ERROR
```
**Fix**: Make fields public or provide constructor:
```rust
pub struct Config {
    pub value: String,  // ✅ Public field
}

// Or better: Keep private, provide constructor
pub struct Config {
    value: String,
}

impl Config {
    pub fn new(value: String) -> Self {
        Config { value }
    }
}
```

### Pitfall 4: Circular Dependencies
```rust
// a.rs
use crate::b::B;

// b.rs
use crate::a::A;  // ❌ Can cause issues
```
**Fix**: Restructure to avoid circular dependencies. Extract shared code to a third module.

### Pitfall 5: Path Confusion (crate vs super vs self)
```rust
// When to use what?
use crate::module;     // Absolute path from crate root
use super::sibling;    // Relative path (parent's child)
use self::child;       // Relative path (my child)
```

## Code Walkthrough

This project demonstrates a real-world structure:
```
15-modules-crates/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Library crate root
│   ├── main.rs          # Binary crate (uses the library)
│   ├── models/
│   │   ├── mod.rs       # Module declaration
│   │   └── user.rs      # User model
│   ├── services/
│   │   ├── mod.rs
│   │   └── auth.rs      # Authentication service
│   └── utils.rs         # Utility functions
```

See `src/lib.rs` and `src/main.rs` for complete implementation.

## Module Organization Patterns

### Pattern 1: Flat Structure (Small Projects)
```
src/
├── lib.rs
├── models.rs
├── services.rs
└── utils.rs
```

### Pattern 2: Nested Structure (Medium Projects)
```
src/
├── lib.rs
├── models/
│   ├── mod.rs
│   ├── user.rs
│   └── post.rs
└── services/
    ├── mod.rs
    └── auth.rs
```

### Pattern 3: Domain-Driven (Large Projects)
```
src/
├── lib.rs
├── domain/
│   ├── user/
│   │   ├── mod.rs
│   │   ├── entity.rs
│   │   └── repository.rs
│   └── post/
└── infrastructure/
```

## Visibility Rules

| Item | Private (default) | Public (`pub`) |
|------|-------------------|----------------|
| Module | Only parent can see | Anyone can see |
| Function | Only module can call | Anyone can call |
| Struct | Only module can create | Anyone can create |
| Struct field | Only module can access | Anyone can access |
| Enum | Only module can use | Anyone can use |
| Enum variant | Same as enum visibility | N/A |

**Special**: `pub(crate)` - Public only within the crate
**Special**: `pub(super)` - Public to parent module

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| Privacy default | Private | By case (upper=pub) | Public |
| Module system | Explicit `mod` | Package/file-based | File/directory-based |
| Namespacing | Hierarchical modules | Package-level | Module/package |
| Visibility levels | pub, pub(crate), pub(super), private | Public, private | Public, _private (convention) |
| Re-exports | `pub use` | Not needed | `__init__.py` imports |
| Library vs binary | lib.rs vs main.rs | Explicit in files | Explicit in files |
| Compile unit | Crate | Package | Module |

## Workspace Structure (Multi-Crate Projects)

For very large projects, use workspaces:
```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "core",
    "api",
    "cli",
]

# core/Cargo.toml
[package]
name = "myproject-core"
version = "0.1.0"

# api/Cargo.toml
[package]
name = "myproject-api"
version = "0.1.0"

[dependencies]
myproject-core = { path = "../core" }
```

## Re-exports and Facade Pattern

Make internal modules easier to use:
```rust
// lib.rs
mod internal {
    pub mod deeply {
        pub mod nested {
            pub fn helper() {}
        }
    }
}

// Instead of: use mylib::internal::deeply::nested::helper;
// Provide:
pub use internal::deeply::nested::helper;
// Now users can: use mylib::helper;
```

## Additional Challenges

1. **Plugin System**: Create a module structure that allows for plugin-like extensions

2. **Feature Flags**: Use Cargo features to conditionally compile modules

3. **Workspace Project**: Build a multi-crate workspace with shared dependencies

4. **API Versioning**: Structure modules to support v1 and v2 APIs simultaneously

5. **Prelude Pattern**: Create a prelude module that re-exports commonly used items

6. **Test Organization**: Organize unit tests, integration tests, and doc tests properly

## Future Directions

- **Next**: Explore smart pointers and advanced ownership (Project 16)
- **Later**: Build complex multi-module systems (Projects 28, 49)
- **Advanced**: Create published crates with proper documentation

## Running This Project

```bash
cd 15-modules-crates

# Run the binary (uses the library)
cargo run

# Test the library
cargo test

# Check documentation
cargo doc --open
```

## Expected Output

You should see:
- Demonstration of module organization
- Public vs private visibility
- Library and binary interaction
- Path resolution (crate, super, self)
- Re-exports and prelude patterns
- Real-world project structure examples
- Best practices for maintainable code organization

The program will demonstrate a complete library + binary structure with proper encapsulation and API design.

## Project Structure Explanation

This project includes:
- **src/lib.rs**: Library crate root (exports public API)
- **src/main.rs**: Binary crate (uses the library)
- **src/models/**: Data models (User, Post, etc.)
- **src/services/**: Business logic (Authentication, etc.)
- **src/utils.rs**: Utility functions
- **Cargo.toml**: Package configuration

The binary demonstrates using the library, showing the separation of concerns between library code (reusable) and application code (executable).
