# Lab Conversion Progress Tracker

This document tracks the conversion of labs 11-60 from incomplete (main.rs only) to complete teaching labs.

## Legend

- ✅ Complete - Has lib.rs, solution.rs, tests, enhanced README
- 🚧 In Progress - Someone is actively working on it
- ⏳ Ready - Analyzed and ready to convert
- ❓ Pending - Not yet analyzed
- 🔄 Needs Review - Completed but needs peer review

## Labs 11-20: Foundational Advanced Topics

| # | Lab Name | Status | Converter | Priority | Notes |
|---|----------|--------|-----------|----------|-------|
| 11 | control-flow | ❓ | - | 🟡 High | Fundamental concept |
| 12 | enums-pattern-matching | ❓ | - | 🟡 High | Depends on 11 |
| 13 | generics-bounds | ❓ | - | 🟡 High | Core language feature |
| 14 | utxo-model | ❓ | - | 🟠 Medium | Blockchain-specific |
| 15 | lifetimes-borrow-checker | ❓ | - | 🔴 Critical | Most difficult concept |
| 16 | collections-iterators | ❓ | - | 🟡 High | Used everywhere |
| 17 | closures-iterators | ❓ | - | 🟡 High | Functional programming |
| 18 | modules-crates | ❓ | - | 🟠 Medium | Project organization |
| 19 | smart-pointers | ❓ | - | 🟠 Medium | Advanced memory |
| 20 | multithreading-basics | ❓ | - | 🟡 High | Systems programming |

## Labs 21-35: Systems & Applications

| # | Lab Name | Status | Converter | Priority | Notes |
|---|----------|--------|-----------|----------|-------|
| 21 | async-basics | ❓ | - | 🟡 High | Increasingly important |
| 22 | chat-server | ❓ | - | 🟠 Medium | Practical application |
| 23 | testing-benchmarking | ❓ | - | 🟠 Medium | Testing practices |
| 24 | cargo-features | ❓ | - | 🟢 Low | Advanced Cargo |
| 25 | transaction-pool | ❓ | - | 🟠 Medium | Blockchain-specific |
| 26 | consensus-simulation | ❓ | - | 🟠 Medium | Blockchain-specific |
| 27 | gui-egui | ❓ | - | 🟢 Low | Specialized domain |
| 28 | web-server-axum | ❓ | - | 🟠 Medium | Web development |
| 29 | thread-pool | ❓ | - | 🟠 Medium | Concurrency pattern |
| 30 | lock-free-structure | ❓ | - | 🟢 Low | Advanced concurrency |
| 31 | key-value-store | ❓ | - | 🟢 Low | Data structures |
| 32 | basic-vm | ❓ | - | 🟢 Low | Interpreters |
| 33 | message-bus | ❓ | - | 🟢 Low | Architecture pattern |
| 34 | lru-cache | ❓ | - | 🟢 Low | Caching pattern |
| 35 | parallel-processing | ❓ | - | 🟢 Low | Parallelism |

## Labs 36-53: Advanced & Specialized

| # | Lab Name | Status | Converter | Priority | Notes |
|---|----------|--------|-----------|----------|-------|
| 36 | interpreter | ❓ | - | 🟢 Low | Compilers/interpreters |
| 37 | command-runner | ❓ | - | 🟢 Low | Process management |
| 38 | memmap-search | ❓ | - | 🟢 Low | File I/O |
| 39 | order-book | ❓ | - | 🟢 Low | Trading systems |
| 40 | task-scheduler | ❓ | - | 🟢 Low | Scheduling |
| 41 | cli-todo | ❓ | - | 🟢 Low | CLI applications |
| 42 | plugin-system | ❓ | - | 🟢 Low | Plugin architecture |
| 43 | file-encryption | ❓ | - | 🟢 Low | Cryptography |
| 44 | web-scraper | ❓ | - | 🟢 Low | Web scraping |
| 45 | csv-to-json | ❓ | - | 🟢 Low | Data formats |
| 46 | declarative-macros | ❓ | - | 🟢 Low | Metaprogramming |
| 47 | operator-overloading | ❓ | - | 🟢 Low | Traits |
| 48 | proof-of-work | ❓ | - | 🟠 Medium | Blockchain-specific |
| 49 | digital-signatures | ❓ | - | 🟠 Medium | Cryptography |
| 50 | wallet-cli | ❓ | - | 🟠 Medium | Blockchain-specific |
| 51 | concurrent-crawler | ❓ | - | 🟢 Low | Concurrency |
| 52 | blockchain-node | ❓ | - | 🟠 Medium | Blockchain-specific |
| 53 | hft-trading-bot | ❓ | - | 🟢 Low | Trading/algorithms |

## Labs 54-60: Alternative Implementations

| # | Lab Name | Status | Converter | Priority | Notes |
|---|----------|--------|-----------|----------|-------|
| 54 | variables-types-alternative | ❓ | - | 🟢 Low | Alternative approach |
| 55 | ownership-borrowing-alternative | ❓ | - | 🟢 Low | Alternative approach |
| 56 | structs-methods-alternative | ❓ | - | 🟢 Low | Alternative approach |
| 57 | traits-interfaces-alternative | ❓ | - | 🟢 Low | Alternative approach |
| 58 | error-handling-alternative | ❓ | - | 🟢 Low | Alternative approach |
| 59 | merkle-tree-alternative | ❓ | - | 🟢 Low | Alternative approach |
| 60 | blockchain-alternative | ❓ | - | 🟢 Low | Alternative approach |

## Recommended Conversion Order

### Phase 1: Critical Foundation (High Impact)
These are essential for the curriculum:
1. Lab 11 - control-flow (fundamental control flow)
2. Lab 15 - lifetimes-borrow-checker (most difficult concept)
3. Lab 20 - multithreading-basics (important systems)
4. Lab 21 - async-basics (increasingly important)

### Phase 2: Core Advanced (Medium Priority)
These complete the advanced programming topics:
5. Lab 12 - enums-pattern-matching
6. Lab 13 - generics-bounds
7. Lab 16 - collections-iterators
8. Lab 17 - closures-iterators

### Phase 3: Domain-Specific (Lower Priority)
These are specialized topics:
9. Lab 25 - transaction-pool (blockchain)
10. Lab 48 - proof-of-work (blockchain)
11. Lab 28 - web-server-axum (web)

### Phase 4: Everything Else
After above are done, continue with remaining labs in any order.

## How to Update This Tracker

When you convert a lab:

1. **Update Status**:
   - Change from ❓ to 🚧 when you start
   - Change to ✅ when complete
   - Change to 🔄 when done but needs review

2. **Add Your Name**:
   - Put your GitHub username in "Converter" column

3. **Update Your Lab**:
   - Create a PR with your changes
   - Reference this tracker in your PR

Example:
```markdown
| 11 | control-flow | 🚧 | @your-github-name | High | [PR link] |
```

## Statistics

**Total Labs**: 50 (labs 11-60)
**Completed**: 0 (0%)
**In Progress**: 0 (0%)
**Not Started**: 50 (100%)

### Completion by Priority
- 🔴 Critical: 0/1 (0%)
- 🟡 High: 0/10 (0%)
- 🟠 Medium: 0/15 (0%)
- 🟢 Low: 0/24 (0%)

## Getting Started

### Quick Start

1. **Pick a lab** from the "Recommended Conversion Order" above
2. **Run the bootstrap script**:
   ```bash
   ./scripts/convert_lab.sh labs/NN-lab-name
   ```
3. **Follow the 6 steps** in `LAB_CONVERSION_GUIDE.md`
4. **Update this tracker** when done
5. **Create a PR** with your changes

### Resources

- **Full Guide**: `LAB_CONVERSION_GUIDE.md`
- **Example**: `examples/CONVERSION_EXAMPLE.md` (complete walkthrough)
- **Templates**: `templates/` directory
- **Script**: `scripts/convert_lab.sh`

## Communication

- Have questions? Create a GitHub Issue with `[lab-conversion]` tag
- Want to work on a lab? Comment on an Issue claiming it
- Need help? Check existing completed labs (01-10) as examples

## Contributing Guidelines

When converting a lab:

1. **Read the Guide**: Understand all 6 phases before starting
2. **Use Templates**: They ensure consistency
3. **Test Thoroughly**: All tests must pass
4. **Document Everything**: Exhaustive docs are the goal
5. **Create PR**: Reference this tracker
6. **Peer Review**: Get feedback before merging

## Long-Term Vision

**Goal**: All 61 labs (01-60 + alternatives) fully converted with:
- ✅ Complete lib.rs with exercise scaffolding
- ✅ Exhaustive solution.rs with deep explanations
- ✅ 20-40+ comprehensive integration tests
- ✅ Enhanced README with full pedagogical structure
- ✅ All tests passing
- ✅ All code compiling without warnings

**Timeline**: Open-ended community effort
**Effort**: ~7-13 hours per lab × 50 labs = ~350-650 hours total

---

Last updated: 2026-02-06
