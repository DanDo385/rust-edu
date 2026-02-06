# Project 16: Smart Pointers

## Overview
Learn Rust's smart pointer types for advanced memory management patterns. Smart pointers enable heap allocation, shared ownership, interior mutability, and reference counting. This project explores Box<T>, Rc<T>, Arc<T>, RefCell<T>, and when to use each.

## Concepts Taught
- **Box<T>** for heap allocation and recursive types
- **Rc<T>** for single-threaded shared ownership
- **Arc<T>** for thread-safe shared ownership
- **RefCell<T>** for interior mutability (runtime borrow checking)
- **Deref trait** and automatic dereferencing
- **Drop trait** for custom cleanup
- Building complex data structures (trees, graphs)
- Reference counting and memory leaks

## Why Rust Behaves This Way

### Smart Pointers vs Regular References
Regular references (&T) are "borrowed" - they don't own data. Smart pointers:
- **Own** the data they point to
- Implement **Deref** and **Drop** traits
- Enable patterns impossible with regular references

### Box<T>: Simple Heap Allocation
Most data in Rust lives on the stack (fast, fixed-size). Box<T> moves data to the heap when:
- Size unknown at compile time (recursive types)
- Large data that's expensive to copy
- Want ownership transfer without copying data

**Comparison:**
- **C**: malloc/free (manual, error-prone)
- **C++**: unique_ptr (similar to Box)
- **Go**: Everything is heap-allocated by GC
- **Rust**: Stack by default, Box for explicit heap allocation

### Rc<T> and Arc<T>: Shared Ownership
Sometimes multiple parts of code need to own the same data. Rc (Reference Counted) enables this:
- **Rc<T>**: Single-threaded reference counting
- **Arc<T>**: Thread-safe reference counting (Atomic Rc)

When the last owner drops, data is freed. This is like:
- **C++**: shared_ptr
- **Python**: Default behavior (everything is reference counted)
- **Swift**: ARC (Automatic Reference Counting)

But unlike Python, Rust's reference counting is **explicit** and **opt-in**.

### RefCell<T>: Interior Mutability
Rust's borrowing rules are enforced at compile time. But sometimes you need runtime flexibility:
- **RefCell<T>** moves borrow checking to runtime
- Allows mutation through immutable references
- Panics if you violate borrowing rules at runtime

This is useful for:
- Complex data structures (graphs with cycles)
- Test doubles and mocks
- Observer pattern implementations

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Confusing Box, Rc, and Arc
```rust
let x = Box::new(5);
let y = x;  // ❌ Move! x is no longer valid
```
**Fix**: Use Rc for shared ownership:
```rust
let x = Rc::new(5);
let y = Rc::clone(&x);  // ✅ Both valid, reference count = 2
```

### Pitfall 2: Using Rc in Multithreaded Code
```rust
let data = Rc::new(vec![1, 2, 3]);
thread::spawn(move || {  // ❌ ERROR: Rc is not Send
    println!("{:?}", data);
});
```
**Fix**: Use Arc instead:
```rust
let data = Arc::new(vec![1, 2, 3]);
thread::spawn(move || {  // ✅ Arc is Send + Sync
    println!("{:?}", data);
});
```

### Pitfall 3: Creating Reference Cycles (Memory Leaks)
```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

let a = Rc::new(RefCell::new(Node { next: None }));
let b = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&a)) }));
a.borrow_mut().next = Some(Rc::clone(&b));  // ❌ Cycle! Memory leak
```
**Fix**: Use Weak<T> for one direction of the relationship.

### Pitfall 4: RefCell Runtime Panics
```rust
let x = RefCell::new(5);
let r1 = x.borrow_mut();
let r2 = x.borrow_mut();  // ❌ PANIC: already borrowed mutably
```
**Fix**: Ensure borrows don't overlap:
```rust
let x = RefCell::new(5);
{
    let mut r1 = x.borrow_mut();
    *r1 += 1;
}  // r1 dropped here
let r2 = x.borrow();  // ✅ OK
```

## Code Walkthrough

See `src/main.rs` for detailed examples of:
1. Box<T> for heap allocation and recursive types
2. Rc<T> for shared ownership in single-threaded code
3. Arc<T> for shared ownership across threads
4. RefCell<T> for interior mutability
5. Building a tree structure with smart pointers
6. Demonstrating reference counting behavior
7. Interior mutability patterns

## Performance Considerations

**Box<T>:**
- One indirection (pointer dereference)
- No overhead beyond the heap allocation
- Same as C++ unique_ptr

**Rc<T> / Arc<T>:**
- Two allocations: data + reference count
- Increment/decrement on clone/drop
- Arc uses atomic operations (slower than Rc)
- Overhead: ~16 bytes (pointer + 2 counters)

**RefCell<T>:**
- Runtime borrow checking (small cost)
- No overhead when not borrowed
- Panics instead of compile error if rules violated

**When to use each:**
- **Box**: Default for heap allocation, recursive types
- **Rc**: Shared ownership, single thread, no cycles
- **Arc**: Shared ownership, multiple threads
- **RefCell**: Interior mutability, single thread only
- **Mutex**: Interior mutability, thread-safe

## Comparison: Rust vs C++ vs Go

| Feature | Rust | C++ | Go |
|---------|------|-----|----|
| Heap allocation | Box<T> | unique_ptr | new() (all heap) |
| Shared ownership | Rc<T> / Arc<T> | shared_ptr | Default (GC) |
| Interior mutability | RefCell<T> / Mutex<T> | mutable | Everything mutable |
| Cycle detection | Manual (Weak) | Manual (weak_ptr) | Automatic (GC) |
| Thread safety | Type system (Send/Sync) | Manual | Manual |
| Memory leaks | Possible with cycles | Possible with cycles | GC prevents most |

## Additional Challenges

1. **Doubly-Linked List**: Implement a doubly-linked list using Rc and RefCell.

2. **Observer Pattern**: Create an event system where multiple observers share ownership of an event source.

3. **Graph Structure**: Build a directed graph with cycles using Rc, RefCell, and Weak.

4. **Cache with Shared Data**: Implement a cache where multiple threads can access the same cached values using Arc.

5. **Custom Smart Pointer**: Implement your own smart pointer with custom Drop behavior.

## Key Takeaways

1. **Box<T>** is for simple heap allocation and recursive types
2. **Rc<T>** enables shared ownership in single-threaded code
3. **Arc<T>** is the thread-safe version of Rc
4. **RefCell<T>** allows mutation through shared references
5. Smart pointers implement **Deref** and **Drop** traits
6. Reference cycles can cause memory leaks - use **Weak<T>** to break them
7. Choose based on thread-safety needs: Rc (single) vs Arc (multi)
8. Interior mutability trades compile-time safety for runtime flexibility
9. All smart pointers have performance overhead - use when needed
10. Understanding smart pointers unlocks complex data structures

## Common Mistakes

❌ Using Rc in multithreaded code (not Send/Sync)
❌ Creating reference cycles without Weak<T>
❌ Using RefCell when compile-time borrowing would work
❌ Cloning Arc/Rc when you just need a reference
❌ Forgetting that RefCell panics at runtime
❌ Using interior mutability when not needed (prefer immutability)
❌ Not understanding the difference between Clone (data) and Rc::clone (reference count)
❌ Using Box when stack allocation would work
❌ Mixing RefCell with multithreading (use Mutex instead)
❌ Not considering Weak<T> for parent-child relationships

## Future Directions

- **Next**: Learn multithreading with std::thread (Project 17)
- **Advanced**: Async programming with Arc and async/await (Project 18)
- **Deep Dive**: Implement a lock-free data structure (Project 27)

## Running This Project

```bash
cd 16-smart-pointers
cargo run
```

## Expected Output

You'll see demonstrations of:
- Box heap allocation and recursive types
- Rc reference counting behavior
- Arc sharing across threads
- RefCell interior mutability
- A complete tree structure implementation
- Reference count tracking as values are cloned/dropped
