# Project 57 - Traits and Interfaces (Alternative Track)

## What You Will Build

You will implement trait-based polymorphism in two styles:

1. Static dispatch via generics (`T: Describe`)
2. Dynamic dispatch via trait objects (`&dyn Describe`, `Box<dyn Animal>`)

You will also implement `Display` and default trait methods.

## Why This Lab Matters (First Principles)

Traits let behavior be decoupled from concrete types.

- Generic functions compile specialized machine code per type (monomorphization).
- Trait objects support heterogeneous collections using vtables.
- Default methods encode reusable behavior contracts.

## Memory Model and Ownership

### High-level ownership flow

```text
Concrete values: Person/Car/Book/Dog/Cat
   |
   | borrowed as &T or &dyn Trait
   v
trait methods produce owned Strings
   |
   v
caller owns results, source values remain valid
```

### Stack vs heap in this lab

- Stack:
  - concrete structs and references
  - trait object fat pointers (data ptr + vtable ptr)
- Heap:
  - `String` fields
  - boxed trait objects (`Box<dyn Animal>`)

### Concrete memory sketch

```text
Vec<Box<dyn Animal>>
  [ Box -> Dog { name: String },
    Box -> Cat { name: String } ]

Each box owns heap allocation; dynamic dispatch uses vtable per concrete type.
```

### Borrow checker behavior

- `get_description<T: Describe>(&T)` uses shared borrow and static dispatch.
- `describe_all(&[&dyn Describe])` borrows heterogeneous values read-only.
- `collect_sounds(&[Box<dyn Animal>])` borrows boxes; ownership of animals remains with vector.

## Rust Mental Models in This Lab

- Immutability by default keeps trait methods side-effect free unless mutation is explicit.
- Mutability is explicit when building collections or editing fields.
- Speed: static dispatch is zero-cost; dynamic dispatch adds one indirection.
- Safety: dynamic polymorphism remains memory-safe and lifetime-checked.

## Symbol Deep Dive

### `&` and `&mut`

- `&T` and `&dyn Trait` are shared borrows.
- `&dyn Trait` is a fat pointer (data + vtable), not a copied object.
- Misconception: trait objects do not require mutable borrowing to call read-only methods.

### `*`

- `*` may appear in arithmetic helpers; not central to trait dispatch.
- Dereference of `Box<T>` is implicit in many method calls via auto-deref.

### Additional symbols used here

- `::` for constructors and associated methods
- `->` explicit return type contracts
- `dyn Trait` for runtime polymorphism

## Exercises

1. `Describe` implementations (`Person`, `Car`, `Book`)
- Goal: implement description and labels.
- Constraints: consistent output formatting.
- Edge cases: empty strings, boundary ages/years.
- Success: describe and label tests pass.

2. Generic helpers (`get_description`, `labeled_description`)
- Goal: use trait bounds with static dispatch.
- Constraints: no type-specific branching.
- Edge cases: all describable types.
- Success: output matches expected label/description composition.

3. Dynamic helpers (`describe_all`, `collect_sounds`)
- Goal: handle mixed concrete types through trait objects.
- Constraints: preserve order of outputs.
- Edge cases: empty input vectors.
- Success: heterogeneous tests pass.

4. `Summary` + `Display`
- Goal: implement default/overridden methods and user-facing formatting.
- Constraints: deterministic string formatting.
- Success: summary/display tests pass.

## What Tests Prove

- Trait implementation tests prove contract conformance.
- Generic tests prove compile-time polymorphism.
- Dynamic tests prove runtime vtable dispatch on mixed collections.
- Display/clone/equality tests prove ergonomic integration traits.

Failure interpretation:

- Type inference or trait-bound errors indicate missing trait impls or signatures.
- Wrong dynamic output ordering often means iteration logic bug.

## Performance Notes

- Static-dispatch helpers are monomorphized and inlinable.
- Dynamic dispatch has small runtime cost per call.
- Boxing introduces heap allocations but enables heterogeneous ownership.

## How to Run

```bash
cargo run -p traits-interfaces-alternative
cargo test -p traits-interfaces-alternative
cargo check -p traits-interfaces-alternative
```
