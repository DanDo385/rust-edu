//! # Lab 16: Collections & Iterators
//!
//! Iterators are lazy, composable transformations over collections.
//! They're zero-cost abstractions that compile down to the same code as manual loops.

/// Finds the sum of all even numbers in a slice.
///
/// **Iterator pattern: filter + map + sum**
/// - `filter()`: Keep only items matching a condition
/// - `map()`: Transform each item
/// - `sum()`: Accumulate into a result
///
/// **From the borrow checker's perspective:**
/// - `numbers: &[i32]` - borrowed slice (don't own the data)
/// - `iter()` - creates an iterator over references
/// - Each `&n` in the closure is a reference to each number
/// - We never move the numbers out of the slice
pub fn sum_evens(numbers: &[i32]) -> i32 {
    // **Zero-cost abstraction:**
    // This compiles to the same assembly as:
    //   let mut sum = 0;
    //   for n in numbers {
    //       if n % 2 == 0 { sum += n; }
    //   }
    numbers.iter().filter(|&&n| n % 2 == 0).sum()
}

/// Counts how many items match a predicate.
///
/// **Teaching: Higher-order functions**
/// - Takes a closure (predicate function)
/// - Applies it to each item
/// - Returns count of matches
pub fn count_matching<F>(items: &[i32], predicate: F) -> usize
where
    F: Fn(i32) -> bool,
{
    // **Closure as parameter:**
    // - F is generic over any Fn(i32) -> bool
    // - Could be a lambda, named function, or function pointer
    // - The generic ensures type safety
    items.iter().filter(|&&n| predicate(n)).count()
}

/// Checks if all items in a slice are positive.
///
/// **Iterator method: all()**
/// - Short-circuits on first false
/// - Returns immediately without checking remaining items
/// - Lazy evaluation (stops early when possible)
pub fn all_positive(numbers: &[i32]) -> bool {
    // **Why iter() instead of into_iter()?**
    // - iter() borrows items (&i32) - doesn't move them
    // - We only need to check values, not consume them
    // - Caller keeps ownership after this function
    numbers.iter().all(|&n| n > 0)
}

/// Groups consecutive equal numbers.
///
/// **Teaching: collect() with Vec**
/// - Transforms an iterator into a collection
/// - In this case, creates Vec of Vec
/// - Ownership is transferred to the new Vec
pub fn group_consecutive(numbers: &[i32]) -> Vec<Vec<i32>> {
    // **Ownership note:**
    // - We create new Vecs (allocate on heap)
    // - Return type Vec<Vec<i32>> is owned
    // - Caller becomes responsible for the memory
    let mut groups = Vec::new();
    let mut current_group = Vec::new();

    for &n in numbers {
        if current_group.is_empty() || current_group[0] == n {
            current_group.push(n);
        } else {
            groups.push(current_group);
            current_group = vec![n];
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

/// Finds the first item matching a condition.
///
/// **Iterator method: find()**
/// - Returns Option<&T>
/// - Short-circuits on first match
/// - Returns None if no match found
pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // **Return type Option<i32>:**
    // - find() returns Option<&i32> (reference)
    // - We copy() because i32 is Copy
    // - Result is Option<i32> (owned value)
    numbers.iter().find(|&&n| n % 2 == 0).copied()
}

/// Maps integers to their string representations.
///
/// **Iterator method: map() + collect()**
/// - map() transforms each item with a closure
/// - collect() gathers results into a Vec
/// - Type annotation tells collect() what to build
pub fn to_strings(numbers: &[i32]) -> Vec<String> {
    // **Ownership transformation:**
    // - Input: &[i32] - borrowed
    // - Output: Vec<String> - owned
    // - Each number becomes a new String (heap allocation)
    // - Caller owns the new Vec and all its Strings
    numbers
        .iter()
        .map(|n| n.to_string())
        .collect()
}

/// Finds the maximum in a slice, or None if empty.
///
/// **Iterator method: max()**
/// - Returns Option<T> (or Option<&T> if using references)
/// - Short-circuits after finding max
pub fn find_max(numbers: &[i32]) -> Option<i32> {
    // **Why copied()?**
    // - max() on iter() returns Option<&i32>
    // - copied() converts Option<&i32> to Option<i32>
    // - Works because i32 is Copy
    numbers.iter().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_evens() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(sum_evens(&nums), 12); // 2 + 4 + 6
    }

    #[test]
    fn test_all_positive_true() {
        let nums = vec![1, 2, 3, 4];
        assert!(all_positive(&nums));
    }

    #[test]
    fn test_all_positive_false() {
        let nums = vec![1, 2, -3, 4];
        assert!(!all_positive(&nums));
    }

    #[test]
    fn test_count_matching() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(count_matching(&nums, |n| n % 2 == 0), 3);
    }

    #[test]
    fn test_find_first_even() {
        let nums = vec![1, 3, 4, 5];
        assert_eq!(find_first_even(&nums), Some(4));
    }

    #[test]
    fn test_find_first_even_none() {
        let nums = vec![1, 3, 5, 7];
        assert_eq!(find_first_even(&nums), None);
    }

    #[test]
    fn test_find_max() {
        let nums = vec![3, 1, 4, 1, 5, 9];
        assert_eq!(find_max(&nums), Some(9));
    }

    #[test]
    fn test_find_max_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(find_max(&nums), None);
    }
}
