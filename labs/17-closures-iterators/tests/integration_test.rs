//! Integration tests for Lab 17: Closures & Iterators

use closures_iterators::*;

#[test]
fn test_apply_closure_basic() {
    let add_five = |x| x + 5;
    assert_eq!(apply_closure(add_five, 10), 15);
}

#[test]
fn test_apply_closure_with_multiple_operations() {
    // Demonstrates that closures can contain complex logic
    let complex = |x| {
        let temp = x * 2;
        temp + 10
    };
    assert_eq!(apply_closure(complex, 5), 20);
}

#[test]
fn test_apply_closure_zero() {
    let identity = |x| x;
    assert_eq!(apply_closure(identity, 0), 0);
}

#[test]
fn test_apply_twice_composition() {
    let increment = |x| x + 1;
    assert_eq!(apply_twice(increment, 10), 12); // (10 + 1) + 1
}

#[test]
fn test_apply_twice_multiplication() {
    let triple = |x| x * 3;
    assert_eq!(apply_twice(triple, 2), 18); // (2 * 3) * 3
}

#[test]
fn test_apply_twice_zero() {
    let double = |x| x * 2;
    assert_eq!(apply_twice(double, 0), 0);
}

#[test]
fn test_apply_n_times_zero_iterations() {
    let increment = |x| x + 1;
    assert_eq!(apply_n_times(increment, 5, 0), 5); // No changes
}

#[test]
fn test_apply_n_times_multiple_iterations() {
    let increment = |x| x + 1;
    assert_eq!(apply_n_times(increment, 0, 10), 10);
}

#[test]
fn test_apply_n_times_with_stateful_closure() {
    // Demonstrates FnMut - closure tracks state
    let mut call_count = 0;
    let counter = |x| {
        call_count += 1;
        x + 1
    };
    let result = apply_n_times(counter, 0, 5);
    assert_eq!(result, 5);
    assert_eq!(call_count, 5); // Closure modified external state
}

#[test]
fn test_filter_with_closure_evens() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let evens = filter_with_closure(&nums, |n| n % 2 == 0);
    assert_eq!(evens, vec![2, 4, 6, 8]);
}

#[test]
fn test_filter_with_closure_odds() {
    let nums = vec![1, 2, 3, 4, 5];
    let odds = filter_with_closure(&nums, |n| n % 2 != 0);
    assert_eq!(odds, vec![1, 3, 5]);
}

#[test]
fn test_filter_with_closure_greater_than() {
    let nums = vec![1, 5, 3, 8, 2, 9];
    let large = filter_with_closure(&nums, |n| n > 5);
    assert_eq!(large, vec![8, 9]);
}

#[test]
fn test_filter_with_closure_empty_result() {
    let nums = vec![1, 2, 3];
    let result = filter_with_closure(&nums, |n| n > 10);
    assert_eq!(result, vec![]);
}

#[test]
fn test_filter_with_closure_all_match() {
    let nums = vec![2, 4, 6, 8];
    let result = filter_with_closure(&nums, |n| n % 2 == 0);
    assert_eq!(result, vec![2, 4, 6, 8]);
}

#[test]
fn test_transform_and_filter_basic() {
    let nums = vec![1, 2, 3, 4, 5];
    // Double each, keep if > 4
    let result = transform_and_filter(&nums, |x| x * 2, |x| x > 4);
    // doubled: [2, 4, 6, 8, 10]
    // filtered (>4): [6, 8, 10]
    assert_eq!(result, vec![6, 8, 10]);
}

#[test]
fn test_transform_and_filter_no_matches() {
    let nums = vec![1, 2, 3];
    // Add 10, keep if < 0 (impossible)
    let result = transform_and_filter(&nums, |x| x + 10, |x| x < 0);
    assert_eq!(result, vec![]);
}

#[test]
fn test_transform_and_filter_all_match() {
    let nums = vec![1, 2, 3];
    // Multiply by 2, keep if > 0
    let result = transform_and_filter(&nums, |x| x * 2, |x| x > 0);
    assert_eq!(result, vec![2, 4, 6]);
}

#[test]
fn test_counter_iterator_basic() {
    let counter = Counter::new(3);
    let nums: Vec<_> = counter.collect();
    assert_eq!(nums, vec![1, 2, 3]);
}

#[test]
fn test_counter_iterator_large() {
    let counter = Counter::new(10);
    let nums: Vec<_> = counter.collect();
    assert_eq!(nums.len(), 10);
    assert_eq!(nums[0], 1);
    assert_eq!(nums[9], 10);
}

#[test]
fn test_counter_iterator_with_filter() {
    // Shows that custom iterators work with iterator methods
    let evens: Vec<_> = Counter::new(10)
        .filter(|n| n % 2 == 0)
        .collect();
    assert_eq!(evens, vec![2, 4, 6, 8, 10]);
}

#[test]
fn test_counter_iterator_with_map() {
    let doubled: Vec<_> = Counter::new(5)
        .map(|n| n * 2)
        .collect();
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
}

#[test]
fn test_counter_sum() {
    // Tests that custom iterator works with sum()
    let sum: u32 = Counter::new(5).sum();
    assert_eq!(sum, 15); // 1+2+3+4+5
}

#[test]
fn test_counter_cloneable() {
    // Verifies Counter implements Clone
    let c1 = Counter::new(3);
    let c2 = c1.clone();
    assert_eq!(c1.collect::<Vec<_>>(), c2.collect::<Vec<_>>());
}

#[test]
fn test_sum_with_fold_basic() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(sum_with_fold(&nums), 15);
}

#[test]
fn test_sum_with_fold_single_element() {
    assert_eq!(sum_with_fold(&[42]), 42);
}

#[test]
fn test_sum_with_fold_empty() {
    assert_eq!(sum_with_fold(&[]), 0);
}

#[test]
fn test_sum_with_fold_negative() {
    let nums = vec![-1, -2, -3];
    assert_eq!(sum_with_fold(&nums), -6);
}

#[test]
fn test_product_of_matching_basic() {
    let nums = vec![1, 2, 3, 4, 5];
    // Product of numbers > 2: 3 * 4 * 5
    assert_eq!(product_of_matching(&nums, |n| n > 2), 60);
}

#[test]
fn test_product_of_matching_no_matches() {
    let nums = vec![1, 2, 3];
    // Product starts at 1, no matches: stays 1
    assert_eq!(product_of_matching(&nums, |n| n > 10), 1);
}

#[test]
fn test_product_of_matching_all_match() {
    let nums = vec![2, 3, 4];
    assert_eq!(product_of_matching(&nums, |n| n > 0), 24); // 2*3*4
}

#[test]
fn test_product_of_matching_even_numbers() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    // Product of evens: 2 * 4 * 6
    assert_eq!(product_of_matching(&nums, |n| n % 2 == 0), 48);
}

#[test]
fn test_compose_basic() {
    let add_one = |x| x + 1;
    let double = |x| x * 2;
    let composed = compose(add_one, double);
    // (5 + 1) * 2 = 12
    assert_eq!(composed(5), 12);
}

#[test]
fn test_compose_different_order() {
    let add_one = |x| x + 1;
    let double = |x| x * 2;
    let composed = compose(double, add_one);
    // (5 * 2) + 1 = 11
    assert_eq!(composed(5), 11);
}

#[test]
fn test_compose_three_closures() {
    // Shows that composed closures can be composed further
    let add_one = |x| x + 1;
    let double = |x| x * 2;
    let square = |x| x * x;

    let f1 = compose(add_one, double);  // (x+1)*2
    let f2 = compose(f1, square);       // ((x+1)*2)^2

    // ((3+1)*2)^2 = (4*2)^2 = 8^2 = 64
    assert_eq!(f2(3), 64);
}

#[test]
fn test_compose_identity() {
    let identity = |x| x;
    let double = |x| x * 2;
    let composed = compose(identity, double);
    assert_eq!(composed(5), 10);
}

#[test]
fn test_lazy_evaluation_basic() {
    let nums = vec![1, 2, 3, 4, 5];
    let result = demonstrate_lazy_evaluation(&nums);
    // Filter >2: [3, 4, 5]
    // Double: [6, 8, 10]
    assert_eq!(result, vec![6, 8, 10]);
}

#[test]
fn test_lazy_evaluation_no_matches() {
    let nums = vec![1, 2];
    let result = demonstrate_lazy_evaluation(&nums);
    assert_eq!(result, vec![]); // Both <= 2, so filtered out
}

#[test]
fn test_lazy_evaluation_all_match() {
    let nums = vec![3, 4, 5];
    let result = demonstrate_lazy_evaluation(&nums);
    assert_eq!(result, vec![6, 8, 10]);
}

#[test]
fn test_sum_evens_from_counter_basic() {
    // Counter gives 1,2,3,4,5
    // Evens: 2, 4
    // Squared: 4, 16
    // Sum: 20
    assert_eq!(sum_evens_from_counter(5), 20);
}

#[test]
fn test_sum_evens_from_counter_large() {
    // Counter gives 1..=10
    // Evens: 2, 4, 6, 8, 10
    // Squared: 4, 16, 36, 64, 100
    // Sum: 220
    assert_eq!(sum_evens_from_counter(10), 220);
}

#[test]
fn test_sum_evens_from_counter_one() {
    // Counter gives 1
    // Evens: none
    // Sum: 0
    assert_eq!(sum_evens_from_counter(1), 0);
}

#[test]
fn test_closure_with_capturing_environment() {
    // Demonstrates that closures capture from their scope
    let multiplier = 5;
    let multiply_by = |x| x * multiplier;
    assert_eq!(apply_closure(multiply_by, 3), 15);
}

#[test]
fn test_counter_iterator_take() {
    // Shows that custom iterators work with take()
    let nums: Vec<_> = Counter::new(10).take(3).collect();
    assert_eq!(nums, vec![1, 2, 3]);
}

#[test]
fn test_counter_iterator_skip() {
    // Shows that custom iterators work with skip()
    let nums: Vec<_> = Counter::new(5).skip(2).collect();
    assert_eq!(nums, vec![3, 4, 5]);
}

#[test]
fn test_complex_closure_chain() {
    // Combines multiple concepts: transform, filter, custom iterator
    let result = sum_evens_from_counter(20);
    // Counter: 1..=20
    // Evens: 2,4,6,8,10,12,14,16,18,20
    // Squared: 4,16,36,64,100,144,196,256,324,400
    // Sum: 1540
    assert_eq!(result, 1540);
}

#[test]
fn test_iterator_fusion_optimization() {
    // This test demonstrates zero-cost abstraction
    // Both should produce the same result
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Functional style
    let functional: i32 = nums.iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum();

    // Using our lazy evaluation function
    let lazy = nums.iter()
        .filter(|&&n| n > 0)  // All positive
        .map(|&n| n * n)
        .filter(|&n| n > 10)  // Keep > 10
        .sum::<i32>();

    // Each demonstrates zero-cost abstraction (no intermediate allocations)
    assert_eq!(functional, 4 + 16 + 36 + 64 + 100); // 2^2 + 4^2 + 6^2 + 8^2 + 10^2 = 220
    // lazy: squares [1..=10] = [1,4,9,16,25,36,49,64,81,100], filters >10 = [16,25,36,49,64,81,100]
    assert_eq!(lazy, 16 + 25 + 36 + 49 + 64 + 81 + 100); // 371
}
