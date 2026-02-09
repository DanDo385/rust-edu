//! # Lab 17: Closures & Iterators
//!
//! Closures are anonymous functions that capture their environment.
//! Rust gives them three capture modes via traits: Fn, FnMut, FnOnce.
//! These are ZERO-COST abstractions - they compile to the same code as hand-written loops.

/// Applies a closure to a value. Takes Fn (immutable borrow of captures).
///
/// **Teaching: Higher-order functions**
/// - `F: Fn(i32) -> i32` means F is any type implementing Fn trait
/// - Fn can be called multiple times without moving captures
/// - This is the most flexible closure type
///
/// **From the borrow checker's perspective:**
/// - f is borrowed as &F to call it
/// - The closure can only read (not modify) captured variables
/// - We can call f multiple times safely
pub fn apply_closure<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    // **Why we use Fn instead of FnMut:**
    // - f doesn't need to modify captures
    // - Fn is more flexible - doesn't require f to be mutable
    // - Allows both immutable closures and function pointers
    f(x)
}

/// Applies a closure to a value twice. Takes Fn (composable).
///
/// **Teaching: Multiple applications**
/// - Shows that Fn closures can be called multiple times
/// - Demonstrates partial application (nested calls)
pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    // **Why call f twice:**
    // - Fn trait allows multiple invocations
    // - Each call borrows f immutably
    // - This pattern is common in functional programming
    f(f(x))
}

/// Applies a closure multiple times with mutable state. Takes FnMut.
///
/// **Teaching: Mutable closures (FnMut)**
/// - `F: FnMut(i32) -> i32` can modify captured variables
/// - Can still be called multiple times (unlike FnOnce)
/// - Useful for stateful transformations
///
/// **From the borrow checker's perspective:**
/// - f must be mutable to call it as FnMut
/// - Each call mutably borrows f (which may modify captures)
/// - After loop, f's mutable borrow ends
pub fn apply_n_times<F: FnMut(i32) -> i32>(mut f: F, x: i32, n: usize) -> i32 {
    // **Why FnMut is necessary:**
    // - The closure might track state (like call count)
    // - We need to call it multiple times with state changes
    // - FnOnce wouldn't work (can only call once)
    let mut result = x;
    for _ in 0..n {
        result = f(result);
    }
    result
}

/// Filters a slice of numbers using a closure predicate.
///
/// **Teaching: Closures that capture from environment**
/// - The closure can read variables from outer scope
/// - Captures happen by reference (for Fn closures)
/// - Ownership of original slice stays with caller
///
/// **From the borrow checker's perspective:**
/// - `numbers: &[i32]` - caller lends us the slice
/// - `predicate` closure borrows captured variables from outside this function
/// - We return a new Vec (owned), so caller gets fresh ownership
pub fn filter_with_closure<F: Fn(i32) -> bool>(
    numbers: &[i32],
    predicate: F,
) -> Vec<i32> {
    // **Why iter() + filter() + collect():**
    // - iter() gives us &i32 (borrowed references)
    // - filter() keeps numbers where predicate returns true
    // - collect() gathers results into owned Vec<i32>
    // - We don't move original numbers
    numbers.iter().filter(|&&n| predicate(n)).copied().collect()
}

/// Transforms numbers with one closure, then filters with another.
///
/// **Teaching: Closure composition with iterator chains**
/// - Multiple closures work together in a chain
/// - Each operates on the result of the previous
/// - Iterator adapters are lazy (no work until collect())
pub fn transform_and_filter<F, G>(
    numbers: &[i32],
    transform: F,
    predicate: G,
) -> Vec<i32>
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> bool,
{
    // **Why this pattern:**
    // - Separates concerns (transform vs filter)
    // - Each closure is simple and testable
    // - Iterator chain is lazy - only computes what's needed
    numbers
        .iter()
        .map(|&n| transform(n))  // Transform each number
        .filter(|&n| predicate(n))  // Keep only those matching predicate
        .collect()
}

/// Combines two closures into a single composed closure.
///
/// **Teaching: Returning closures**
/// - Must use `impl Fn` (not bare `Fn` which has unknown size)
/// - Must use `move` to transfer ownership into returned closure
/// - The returned closure captures f and g
///
/// **From the borrow checker's perspective:**
/// - f and g are moved into the returned closure
/// - Returned closure owns copies of f and g
/// - Caller gets a new closure that composes them
pub fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    // **Why move is necessary:**
    // - Returned closure outlives this function
    // - Must own f and g (not borrow them)
    // - move transfers ownership into the closure
    move |x| g(f(x))
}

/// A custom iterator that counts from 1 to max.
///
/// **Teaching: Iterator trait implementation**
/// - Implementing Iterator means providing `next()` method
/// - Returns Option<Item> (Some or None when exhausted)
/// - All iterator methods work on custom iterators!
#[derive(Clone)]
pub struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    /// Creates a new counter from 1 to max (inclusive).
    pub fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // **Why we return Option:**
        // - Iterator protocol uses Option<Item>
        // - Some(item) when there's a next value
        // - None when exhausted
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/// Sums numbers using the fold iterator method.
///
/// **Teaching: fold for accumulation**
/// - fold takes an initial value and a closure
/// - Closure receives (accumulator, current_item)
/// - Returns final accumulated value
/// - No intermediate Vec allocations (efficient!)
pub fn sum_with_fold(numbers: &[i32]) -> i32 {
    // **Why fold instead of a loop:**
    // - Expresses intent clearly (accumulation)
    // - Closure captures the logic
    // - Zero-cost: compiles to equivalent manual loop
    numbers.iter().fold(0, |acc, &n| acc + n)
}

/// Multiplies numbers matching a predicate using fold.
///
/// **Teaching: fold with complex state**
/// - fold can track any accumulator (not just sum)
/// - Here we multiply matching numbers
/// - Combines filtering and reducing in one pass
pub fn product_of_matching<F: Fn(i32) -> bool>(
    numbers: &[i32],
    predicate: F,
) -> i32 {
    // **Why this is efficient:**
    // - Single pass through numbers (no intermediate Vec)
    // - fold handles both filtering and multiplication
    // - No separate filter() call needed
    numbers
        .iter()
        .fold(1, |acc, &n| {
            if predicate(n) {
                acc * n
            } else {
                acc
            }
        })
}

/// Demonstrates lazy evaluation - iterators don't compute until consumed.
///
/// **Teaching: Lazy evaluation**
/// - Creating iterator chains does no work
/// - Work only happens when consumed (collect, sum, etc.)
/// - Zero-cost: compiler optimizes the whole chain together
///
/// **From the borrow checker's perspective:**
/// - The iterator chain holds references to the data
/// - Can't modify/drop the data while chain exists
/// - Consuming (collect) ends the borrow
pub fn demonstrate_lazy_evaluation(numbers: &[i32]) -> Vec<i32> {
    // **This chain doesn't execute yet:**
    // Just creates an iterator adaptor stack
    //
    // **Until we call collect():**
    // Then the whole chain fuses into a single loop:
    //   for n in numbers:
    //     if n > 2:
    //       doubled = n * 2
    //       result.push(doubled)
    numbers
        .iter()
        .filter(|&&n| n > 2)  // Keep only > 2
        .map(|&n| n * 2)      // Double them
        .collect()
}

/// Uses a custom iterator with closures (the real power!).
///
/// **Teaching: Combining custom iterators with closures**
/// - Custom iterators inherit all iterator methods
/// - Can use map, filter, fold, etc. on them
/// - Shows that Iterator trait gives us everything
pub fn sum_evens_from_counter(max: u32) -> u32 {
    // **Why this pattern:**
    // - Counter generates values lazily
    // - filter/map/sum operate on the stream
    // - No intermediate Vec needed
    Counter::new(max)
        .filter(|n| n % 2 == 0)  // Keep evens
        .map(|n| n * n)           // Square them
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_closure() {
        let add_one = |x| x + 1;
        assert_eq!(apply_closure(add_one, 5), 6);
    }

    #[test]
    fn test_apply_twice() {
        let double = |x| x * 2;
        assert_eq!(apply_twice(double, 3), 12); // (3 * 2) * 2
    }

    #[test]
    fn test_apply_n_times() {
        let increment = |x| x + 1;
        assert_eq!(apply_n_times(increment, 0, 5), 5);
    }

    #[test]
    fn test_filter_with_closure() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let evens = filter_with_closure(&nums, |n| n % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
    }

    #[test]
    fn test_transform_and_filter() {
        let nums = vec![1, 2, 3, 4];
        let result = transform_and_filter(&nums, |x| x * 2, |x| x > 4);
        assert_eq!(result, vec![6, 8]); // doubled: [2,4,6,8], filtered: [6,8]
    }

    #[test]
    fn test_counter_iterator() {
        let nums: Vec<_> = Counter::new(5).collect();
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_counter_with_methods() {
        let sum: u32 = Counter::new(5).sum();
        assert_eq!(sum, 15); // 1+2+3+4+5
    }

    #[test]
    fn test_sum_with_fold() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_with_fold(&nums), 15);
    }

    #[test]
    fn test_product_of_matching() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(product_of_matching(&nums, |n| n > 2), 60); // 3*4*5
    }

    #[test]
    fn test_compose() {
        let add_one = |x| x + 1;
        let double = |x| x * 2;
        let f = compose(add_one, double);
        assert_eq!(f(5), 12); // (5 + 1) * 2
    }

    #[test]
    fn test_lazy_evaluation() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = demonstrate_lazy_evaluation(&nums);
        assert_eq!(result, vec![6, 8, 10]); // filtered >2, doubled
    }

    #[test]
    fn test_sum_evens_from_counter() {
        assert_eq!(sum_evens_from_counter(5), 20); // Counter gives 1,2,3,4,5; evens are 2,4; squared: 4+16
    }
}
