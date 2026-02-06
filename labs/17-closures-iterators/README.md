# Project 14: Closures and Iterators (Zero-Cost Abstraction)

## Overview
This project dives deep into closures, the three closure traits (Fn, FnMut, FnOnce), and how they enable powerful iterator patterns. You'll learn how Rust provides functional programming features with zero runtime cost, lazy evaluation, and powerful composability. This is where Rust's zero-cost abstraction philosophy truly shines!

## Concepts Taught
- **Closures** - Anonymous functions that capture environment
- **Closure traits**: `Fn`, `FnMut`, `FnOnce`
- **Capture modes**: by reference, by mutable reference, by value
- **Iterator adaptors** and lazy evaluation
- **Custom iterators** - implementing the Iterator trait
- **Functional programming** patterns in Rust
- **Zero-cost abstractions** - performance analysis
- **Combinator patterns** for code composition
- **Move semantics** with closures

## Why Rust Behaves This Way

### Closures and Ownership
Rust closures capture variables from their environment, but how they capture is determined by usage:

1. **Fn** - Borrows values immutably
   - Can be called multiple times
   - Doesn't modify captured variables
   - Most flexible closure type

2. **FnMut** - Borrows values mutably
   - Can be called multiple times
   - Can modify captured variables
   - Requires mutable access

3. **FnOnce** - Takes ownership of values
   - Can only be called once
   - Consumes captured variables
   - Most restrictive but sometimes necessary

The compiler automatically chooses the most flexible trait that works!

### Why Three Traits?
This design ensures:
- **Memory safety**: No data races with captured variables
- **Performance**: No hidden heap allocations or reference counting
- **Flexibility**: Use the right ownership semantics for each case

Compare to other languages:
- **JavaScript**: Closures always capture by reference (can lead to bugs)
- **Python**: Captures by reference, mutable through nonlocal
- **Go**: No closures, only function pointers
- **Rust**: Explicit ownership, compiler-verified safety

### Zero-Cost Abstraction
Closures in Rust are **zero-cost**:
- No heap allocation (closures are stack types)
- No function pointer indirection (monomorphized at compile time)
- Same performance as hand-written structs with methods
- Compiler inlines small closures completely

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Closure Capture Confusion
```rust
let x = 5;
let f = || x + 1;  // Captures x by reference (&x)
let y = x;  // ✅ OK: x is Copy, so it's not moved
println!("{}", f());  // ✅ Works
```
But with non-Copy types:
```rust
let s = String::from("hello");
let f = || println!("{}", s);  // Captures s by reference
drop(s);  // ❌ ERROR: s is borrowed by closure
f();
```
**Fix**: Use `move` to take ownership:
```rust
let s = String::from("hello");
let f = move || println!("{}", s);  // Takes ownership
// drop(s);  // ❌ ERROR: s was moved into closure
f();  // ✅ Works
```

### Pitfall 2: FnOnce Can Only Be Called Once
```rust
let s = String::from("hello");
let f = move || drop(s);  // FnOnce - consumes s
f();
f();  // ❌ ERROR: f already consumed s!
```
**Fix**: Use Fn or FnMut if you need multiple calls:
```rust
let s = String::from("hello");
let f = move || println!("{}", s);  // Fn - just borrows
f();
f();  // ✅ OK: doesn't consume
```

### Pitfall 3: Mutable Closure Borrowing
```rust
let mut x = 5;
let mut f = || x += 1;  // FnMut - needs mutable borrow
let y = x;  // ❌ ERROR: x is mutably borrowed by f
f();
```
**Fix**: Ensure closure is called before other uses:
```rust
let mut x = 5;
{
    let mut f = || x += 1;
    f();
}  // f dropped here, mutable borrow ends
let y = x;  // ✅ OK now
```

### Pitfall 4: Return Closures Without Box
```rust
fn returns_closure() -> impl Fn(i32) -> i32 {  // ✅ Correct way
    |x| x + 1
}

// ❌ WRONG:
// fn returns_closure() -> Fn(i32) -> i32 {  // ERROR: size unknown
//     |x| x + 1
// }
```
**Note**: Use `impl Fn` or `Box<dyn Fn>` for returning closures.

### Pitfall 5: Forgetting Iterators Are Lazy
```rust
let v = vec![1, 2, 3];
v.iter().map(|x| x * 2);  // ❌ Does nothing! No collect() or consume
```
**Fix**: Consume the iterator:
```rust
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();  // ✅
```

## Code Walkthrough

See `src/main.rs` for comprehensive examples demonstrating:
1. Basic closures and capture modes
2. Fn, FnMut, FnOnce traits
3. Closures with iterators
4. Custom iterator implementation
5. Lazy evaluation and performance
6. Functional patterns (map, filter, fold)
7. Real-world examples comparing imperative vs functional style

## Performance Considerations

### Closure Performance
- **Zero allocation**: Closures are stack types (structs with captured variables)
- **Static dispatch**: Monomorphized at compile time, no function pointers
- **Inlining**: Small closures inline completely, eliminating call overhead
- **Same as manual code**: A closure is literally a struct with a method

Example - what the compiler does:
```rust
let x = 5;
let f = |y| x + y;

// Compiles to something like:
struct Closure { x: i32 }
impl Closure {
    fn call(&self, y: i32) -> i32 {
        self.x + y
    }
}
let f = Closure { x: 5 };
```

### Iterator Performance
- **Lazy evaluation**: No work until consumed (collect, sum, etc.)
- **Fusion**: Multiple iterator adaptors merge into single loop
- **No intermediate allocations**: Chains don't create temporary vectors
- **LLVM optimizations**: Often faster than hand-written loops!

Benchmark results (typical):
- Iterator chains: **Same speed** as manual loops
- Sometimes **faster** due to better optimization opportunities
- Always **more composable** and maintainable

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| Closures | Yes (Fn/FnMut/FnOnce) | Limited (func literals) | Yes (full support) |
| Capture semantics | Explicit (move/borrow) | Always by reference | Always by reference |
| Performance | Zero-cost (inlined) | Function pointers | Slower (interpreted) |
| Ownership tracking | Compile-time | Runtime | Runtime |
| Iterator chains | Yes (zero-cost) | Manual loops | Yes (slower) |
| Lazy evaluation | Yes (iterators) | No | Yes (generators) |
| Type inference | Full | Partial | Dynamic |
| Generic closures | Yes (impl Fn<T>) | No (interface{}) | Yes (duck typing) |

## Functional Programming Patterns

### Map-Reduce
```rust
let sum: i32 = data.iter()
    .map(|x| x * 2)     // Transform
    .sum();             // Reduce
```

### Filter-Map
```rust
let result: Vec<_> = data.iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

### Fold (Reduce with accumulator)
```rust
let product = data.iter()
    .fold(1, |acc, x| acc * x);
```

### Chain Combinators
```rust
let result = data.iter()
    .filter(predicate)
    .map(transform)
    .flat_map(expand)
    .take(10)
    .collect();
```

## Additional Challenges

1. **Custom Iterator**: Implement an iterator for the Fibonacci sequence with lazy evaluation

2. **Closure Composition**: Write a function that takes two closures and returns their composition

3. **Memoization**: Implement a caching mechanism using closures and HashMap

4. **Lazy Filtering**: Create a lazy filter that only computes values when needed

5. **Parser Combinators**: Build a simple parser using closure composition

6. **Performance Benchmark**: Compare iterator chains vs manual loops vs Python equivalents

## Future Directions

- **Next**: Learn about modules and project structure (Project 15)
- **Later**: Async closures and async iterators (Project 18)
- **Advanced**: Parallel iterators with Rayon (Project 32)

## Running This Project

```bash
cd 14-closures-iterators
cargo run
```

## Expected Output

You should see:
- Closure capture demonstrations (Fn, FnMut, FnOnce)
- Iterator chain examples
- Custom iterator implementation
- Performance comparisons
- Lazy evaluation demonstrations
- Functional vs imperative code comparisons
- Real-world examples (data processing, transformations)

The program will show both the elegant functional style and explain the zero-cost compilation underneath.
