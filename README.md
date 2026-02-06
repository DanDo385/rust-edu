# Rust-Edu: 61-Lab Comprehensive Curriculum

**Learn Rust from absolute basics to production-ready systems programming!**

A unified, pedagogically structured curriculum with 61 interconnected labs spanning:
- ✅ Fundamentals (variables, ownership, collections, structs, error handling)
- ✅ Intermediate Rust (traits, generics, lifetimes, advanced type system)
- ✅ Systems Programming (concurrency, async, FFI, memory safety)
- ✅ Blockchain & Cryptography (hashing, merkle trees, consensus, digital signatures)
- ✅ Advanced Applications (web servers, GUI, parsers, virtual machines, trading algorithms)

## 🎯 Learning Philosophy

This curriculum teaches Rust through **extreme pedagogical detail** and **spiral progression**:

- **Labs 01-10**: Foundation phase - Core language concepts with exhaustive teaching
- **Labs 11-20**: Intermediate phase - Advanced language features and real-world patterns
- **Labs 21-40**: Systems & Applications - Concurrency, async, web, and specialized domains
- **Labs 41-60**: Advanced Projects - Production-ready implementations
- **Labs 54-60**: Alternative Implementations - Deep dives into specific topics

Every lab includes:
- **Memory Model section**: Conceptual + under-the-hood + memory diagrams
- **Symbol Deep Dive**: Complete explanation of every `&`, `*`, `::`, `->`, `?`, etc.
- **Comprehensive solution.rs**: Step-by-step teaching with ownership implications
- **Integration tests**: Verify invariants and learning outcomes

## 🚀 Quick Start

```bash
# 1. Install Rust (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone or enter this repository
cd rust-edu

# 3. Build all 61 labs
cargo build --workspace

# 4. Start with Lab 01
cd labs/01-variables-types
cargo run

# 5. Read the solution (exhaustively detailed!)
cat src/solution.rs

# 6. Run tests and complete exercises
cargo test

# 7. Progress through the curriculum
cd ../../labs/02-ownership-borrowing
# ... and so on through all 61 labs!
```

## 📚 Curriculum Structure

### 🟢 Foundation Labs (01-10): Core Language Concepts

**01-variables-types** - Variables, types, and type inference
**02-ownership-borrowing** - Ownership, borrowing, references (⭐ Critical)
**03-collections-basics** - Vec, HashMap, iteration patterns
**04-structs-methods** - Custom types, methods, `self` variants
**05-error-handling** - Option, Result, `?` operator, error propagation
**06-traits-basics** - Trait definitions, polymorphism, bounds (⭐ Critical)
**07-sha256-hashing** - Cryptographic hashing, proof-of-work
**08-merkle-tree** - Tree structures, merkle trees, verification
**09-simple-blockchain** - Full blockchain implementation
**10-transaction-validation** - UTXO model, validation rules

### 🟡 Intermediate Labs (11-20): Advanced Language Features

**11-control-flow** - Pattern matching, exhaustiveness, guards
**12-enums-pattern-matching** - ADTs, pattern matching deep dive
**13-generics-bounds** - Generic types, trait bounds, monomorphization
**14-utxo-model** - Blockchain UTXO implementation
**15-lifetimes-borrow-checker** - Lifetime syntax, elision rules (⭐ Critical)
**16-collections-iterators** - Iterator traits, adapters, lazy evaluation
**17-closures-iterators** - Closures, captures, functional patterns
**18-modules-crates** - Module system, visibility, package organization
**19-smart-pointers** - Box, Rc, RefCell, Arc, interior mutability
**20-multithreading-basics** - Threads, channels, synchronization primitives

### 🟠 Systems Labs (21-35): Real-World Systems Programming

**21-async-basics** - Async/await, futures, runtimes
**22-chat-server** - Network programming with async
**23-testing-benchmarking** - Unit tests, integration tests, benchmarking
**24-cargo-features** - Feature gates, conditional compilation
**25-transaction-pool** - Blockchain mempool implementation
**26-consensus-simulation** - Consensus algorithms (PoW, PoS simulation)
**27-gui-egui** - GUI programming with egui
**28-web-server-axum** - HTTP server with web framework
**29-thread-pool** - Thread pool implementation
**30-lock-free-structure** - Atomic operations, lock-free data structures
**31-key-value-store** - In-memory KV store with persistence
**32-basic-vm** - Virtual machine implementation
**33-message-bus** - Event-driven architecture
**34-lru-cache** - Cache implementations with eviction
**35-parallel-processing** - Rayon, data parallelism

### 🔴 Advanced Labs (36-53): Specialized Domains

**36-interpreter** - Expression evaluation, AST walking
**37-command-runner** - Process management, subprocess communication
**38-memmap-search** - Memory-mapped file search
**39-order-book** - Trading order book data structure
**40-task-scheduler** - Event-driven scheduling system
**41-cli-todo** - Command-line application
**42-plugin-system** - Runtime plugin loading
**43-file-encryption** - Encryption implementation
**44-web-scraper** - Web scraping patterns
**45-csv-to-json** - Data transformation
**46-declarative-macros** - Macro-by-example (declarative macros)
**47-operator-overloading** - Custom operator implementations
**48-proof-of-work** - PoW algorithms and difficulty
**49-digital-signatures** - Signing and verification
**50-wallet-cli** - Blockchain wallet implementation
**51-concurrent-crawler** - Web crawling with concurrency
**52-blockchain-node** - Full blockchain node
**53-hft-trading-bot** - High-frequency trading simulation

### 💜 Alternative Implementations (54-60): Deep Dives

**54-variables-types-alternative** - Alternative approach to type fundamentals
**55-ownership-borrowing-alternative** - Advanced ownership patterns
**56-structs-methods-alternative** - Struct design patterns
**57-traits-interfaces-alternative** - Trait-based design deep dive
**58-error-handling-alternative** - Error handling patterns
**59-merkle-tree-alternative** - Alternative merkle tree implementation
**60-blockchain-alternative** - Alternative blockchain design


- ✅ Recursive types with Box

**09-simple-blockchain** (3 hours) 🔗
- ✅ Blockchain structure
- ✅ Mining with PoW
- ✅ Chain validation
- ✅ Immutability

**10-transaction-validation** (3 hours) 🔗
- ✅ Digital signatures
- ✅ UTXO model
- ✅ Transaction validation
- ✅ Wallets and keypairs

## 🎓 Key Rust Concepts

### The Big Three
1. **Ownership**: Who owns data
2. **Borrowing**: Temporary access to data
3. **Lifetimes**: How long borrows are valid

### Why This Matters
- **No garbage collector**: Fast, predictable performance
- **No data races**: Thread safety guaranteed at compile time
- **No null pointers**: Option instead of null
- **No dangling pointers**: Borrow checker prevents it

### Rust's Guarantee
> If your code compiles, it's memory-safe and thread-safe.

## 🔗 Blockchain Relevance

Rust is becoming the language of choice for blockchain:

- **Solana**: Written in Rust
- **Polkadot**: Written in Rust
- **Substrate**: Blockchain framework in Rust
- **Near Protocol**: Rust smart contracts
- **Diem** (formerly Libra): Rust implementation

Why?
- **Speed**: No GC pauses, predictable performance
- **Safety**: Can't have memory bugs in consensus code
- **Concurrency**: Handle many nodes safely
- **WebAssembly**: Rust compiles to WASM for smart contracts

## 📖 How to Use This Repository

### For Complete Beginners
1. Read `RUST_BASICS.md` first
2. Do projects 1-5 in order
3. Don't skip ahead!
4. Read every comment in solution.rs
5. Type out the code yourself (don't just read)

### For Experienced Programmers
1. Skim projects 1-2 if you know basics
2. Focus on ownership (project 2) - it's unique to Rust
3. Do projects 3-6 for Rust idioms
4. Jump to blockchain projects (7-10) for fun stuff

### Study Method
For each project:
1. Read the README.md
2. Try implementing lib.rs yourself
3. Run tests: `cargo test`
4. If stuck, peek at solution.rs
5. Read ALL comments in solution.rs
6. Compare your approach to the solution
7. Run the binary: `cargo run`

## 🛠️ Prerequisites

- Basic programming knowledge (any language) or willingness to learn
- Command line familiarity
- Curiosity!

## 🤝 Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

## ❓ FAQ

**Q: I've never programmed before. Can I do this?**
A: Projects 1-3 assume no programming knowledge. Go slow and read every comment!

**Q: I know Python/JavaScript. How long will this take?**
A: About 15-20 hours for all projects if you read everything carefully.

**Q: Why is ownership so confusing?**
A: It's a new concept! No other mainstream language has it. Read project 2's solution.rs multiple times.

**Q: When should I move on from a project?**
A: When you understand every line in solution.rs and all tests pass.

**Q: Can I use this for blockchain development?**
A: Yes! Projects 7-10 teach concepts used in real blockchains like Bitcoin and Ethereum.

## 🎯 Next Steps

After completing this:
- Build a full blockchain with networking
- Learn async Rust for web servers
- Try WebAssembly for smart contracts
- Contribute to open-source Rust blockchain projects
- Explore Solana or Substrate development

Good luck on your Rust journey! 🦀
