// Lab 35: Parallel Processing - Integration Tests
//
// These tests verify that:
// 1. Core computation functions (is_prime, expensive_computation, apply_filter) are correct
// 2. Parallel results match sequential results exactly
// 3. Aggregate operations (sum, count, histogram, chunk sums) produce correct values

use parallel_processing::*;

// ============================================================================
// is_prime TESTS
// ============================================================================

#[test]
fn test_is_prime_zero_and_one_are_not_prime() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
}

#[test]
fn test_is_prime_two_is_prime() {
    assert!(is_prime(2));
}

#[test]
fn test_is_prime_small_primes() {
    let small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    for &p in &small_primes {
        assert!(is_prime(p), "{} should be prime", p);
    }
}

#[test]
fn test_is_prime_small_composites() {
    let composites = [4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28];
    for &c in &composites {
        assert!(!is_prime(c), "{} should not be prime", c);
    }
}

#[test]
fn test_is_prime_larger_primes() {
    assert!(is_prime(97));
    assert!(is_prime(101));
    assert!(is_prime(997));
    assert!(is_prime(7919));
}

#[test]
fn test_is_prime_larger_composites() {
    assert!(!is_prime(100));
    assert!(!is_prime(1000));
    assert!(!is_prime(7917)); // 3 * 2639
}

#[test]
fn test_is_prime_even_numbers_not_prime() {
    for n in (4..=100).step_by(2) {
        assert!(!is_prime(n), "even number {} should not be prime", n);
    }
}

// ============================================================================
// expensive_computation TESTS
// ============================================================================

#[test]
fn test_expensive_computation_returns_consistent_values() {
    for n in 0..20 {
        let first = expensive_computation(n);
        let second = expensive_computation(n);
        assert_eq!(first, second, "expensive_computation({}) should be deterministic", n);
    }
}

#[test]
fn test_expensive_computation_different_inputs_may_differ() {
    // Not all inputs produce the same result
    let results: Vec<i32> = (0..10).map(expensive_computation).collect();
    let all_same = results.windows(2).all(|w| w[0] == w[1]);
    assert!(!all_same, "different inputs should generally produce different results");
}

// ============================================================================
// apply_filter TESTS
// ============================================================================

#[test]
fn test_apply_filter_deterministic() {
    for pixel in 0..=255u8 {
        let a = apply_filter(pixel);
        let b = apply_filter(pixel);
        assert_eq!(a, b, "apply_filter({}) should be deterministic", pixel);
    }
}

#[test]
fn test_apply_filter_wrapping_behavior() {
    // The filter uses wrapping arithmetic, so it should never panic
    let _ = apply_filter(0);
    let _ = apply_filter(128);
    let _ = apply_filter(255);
}

// ============================================================================
// PARALLEL vs SEQUENTIAL CORRECTNESS TESTS
// ============================================================================

#[test]
fn test_find_primes_parallel_matches_sequential() {
    let seq = find_primes_sequential(1, 10_000);
    let par = find_primes_parallel(1, 10_000);

    // Parallel collect preserves order for ranges
    let mut par_sorted = par.clone();
    par_sorted.sort();

    assert_eq!(seq, par_sorted, "parallel primes should match sequential primes");
}

#[test]
fn test_find_primes_empty_range() {
    let seq = find_primes_sequential(10, 10);
    let par = find_primes_parallel(10, 10);
    assert!(seq.is_empty());
    assert!(par.is_empty());
}

#[test]
fn test_find_primes_small_range() {
    let primes = find_primes_sequential(1, 20);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
}

#[test]
fn test_find_primes_count_under_1000() {
    // There are 168 primes below 1000
    let primes = find_primes_sequential(1, 1000);
    assert_eq!(primes.len(), 168);
}

#[test]
fn test_parallel_map_matches_sequential_map() {
    let data: Vec<i32> = (1..=50).collect();
    let seq = sequential_map(&data);
    let par = parallel_map(&data);
    assert_eq!(seq, par, "parallel map should produce identical results to sequential map");
}

#[test]
fn test_parallel_map_empty_input() {
    let data: Vec<i32> = vec![];
    let result = parallel_map(&data);
    assert!(result.is_empty());
}

#[test]
fn test_parallel_map_single_element() {
    let data = vec![42];
    let seq = sequential_map(&data);
    let par = parallel_map(&data);
    assert_eq!(seq, par);
}

#[test]
fn test_parallel_filter_image_matches_sequential() {
    let image: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    let seq = sequential_filter_image(&image);
    let par = parallel_filter_image(&image);
    assert_eq!(seq, par, "parallel image filter should match sequential filter");
}

#[test]
fn test_parallel_filter_image_empty() {
    let image: Vec<u8> = vec![];
    let result = parallel_filter_image(&image);
    assert!(result.is_empty());
}

#[test]
fn test_parallel_filter_image_preserves_length() {
    let image: Vec<u8> = vec![10, 20, 30, 40, 50];
    let result = parallel_filter_image(&image);
    assert_eq!(result.len(), image.len());
}

// ============================================================================
// AGGREGATE OPERATION TESTS
// ============================================================================

#[test]
fn test_parallel_sum_correct() {
    let data: Vec<i32> = (1..=1000).collect();
    let expected: i64 = (1..=1000i64).sum();
    assert_eq!(parallel_sum(&data), expected);
}

#[test]
fn test_parallel_sum_empty() {
    let data: Vec<i32> = vec![];
    assert_eq!(parallel_sum(&data), 0);
}

#[test]
fn test_parallel_sum_single_element() {
    let data = vec![42];
    assert_eq!(parallel_sum(&data), 42);
}

#[test]
fn test_parallel_sum_negative_numbers() {
    let data = vec![-10, -20, 30, 40];
    assert_eq!(parallel_sum(&data), 40);
}

#[test]
fn test_parallel_count_even_numbers() {
    let data: Vec<i32> = (1..=100).collect();
    let count = parallel_count(&data, |x| x % 2 == 0);
    assert_eq!(count, 50);
}

#[test]
fn test_parallel_count_all_match() {
    let data: Vec<i32> = (2..=100).step_by(2).collect();
    let count = parallel_count(&data, |x| x % 2 == 0);
    assert_eq!(count, data.len());
}

#[test]
fn test_parallel_count_none_match() {
    let data: Vec<i32> = (1..=100).collect();
    let count = parallel_count(&data, |x| *x > 200);
    assert_eq!(count, 0);
}

#[test]
fn test_parallel_count_empty_slice() {
    let data: Vec<i32> = vec![];
    let count = parallel_count(&data, |_| true);
    assert_eq!(count, 0);
}

// ============================================================================
// CHUNK SUM TESTS
// ============================================================================

#[test]
fn test_parallel_chunk_sums_even_division() {
    let data: Vec<i32> = (1..=100).collect();
    let sums = parallel_chunk_sums(&data, 10);
    assert_eq!(sums.len(), 10);

    // First chunk: 1+2+...+10 = 55
    assert_eq!(sums[0], 55);

    // Total of all chunks should equal total sum
    let total: i32 = sums.iter().sum();
    let expected: i32 = (1..=100).sum();
    assert_eq!(total, expected);
}

#[test]
fn test_parallel_chunk_sums_uneven_division() {
    let data: Vec<i32> = (1..=7).collect();
    let sums = parallel_chunk_sums(&data, 3);
    // Chunks: [1,2,3], [4,5,6], [7]
    assert_eq!(sums.len(), 3);
    assert_eq!(sums[0], 6);
    assert_eq!(sums[1], 15);
    assert_eq!(sums[2], 7);
}

#[test]
fn test_parallel_chunk_sums_single_chunk() {
    let data: Vec<i32> = (1..=5).collect();
    let sums = parallel_chunk_sums(&data, 100);
    assert_eq!(sums.len(), 1);
    assert_eq!(sums[0], 15);
}

#[test]
fn test_parallel_chunk_sums_chunk_size_one() {
    let data = vec![10, 20, 30];
    let sums = parallel_chunk_sums(&data, 1);
    assert_eq!(sums, vec![10, 20, 30]);
}

// ============================================================================
// HISTOGRAM TESTS
// ============================================================================

#[test]
fn test_parallel_digit_histogram_basic() {
    let numbers: Vec<i32> = (1..=100).collect();
    let hist = parallel_digit_histogram(&numbers);
    assert_eq!(hist.len(), 10);

    // Total count should equal input length
    let total: usize = hist.iter().sum();
    assert_eq!(total, 100);
}

#[test]
fn test_parallel_digit_histogram_all_same_digit() {
    // All numbers end in 5
    let numbers = vec![5, 15, 25, 35, 45];
    let hist = parallel_digit_histogram(&numbers);
    assert_eq!(hist[5], 5);
    // All other buckets should be 0
    for (i, &count) in hist.iter().enumerate() {
        if i != 5 {
            assert_eq!(count, 0, "bucket {} should be 0", i);
        }
    }
}

#[test]
fn test_parallel_digit_histogram_empty_input() {
    let numbers: Vec<i32> = vec![];
    let hist = parallel_digit_histogram(&numbers);
    assert_eq!(hist.len(), 10);
    assert!(hist.iter().all(|&c| c == 0));
}

#[test]
fn test_parallel_digit_histogram_single_element() {
    let numbers = vec![7];
    let hist = parallel_digit_histogram(&numbers);
    assert_eq!(hist[7], 1);
    let total: usize = hist.iter().sum();
    assert_eq!(total, 1);
}

#[test]
fn test_parallel_digit_histogram_uniform_distribution() {
    // 0..10 has one of each digit
    let numbers: Vec<i32> = (0..10).collect();
    let hist = parallel_digit_histogram(&numbers);
    for &count in &hist {
        assert_eq!(count, 1);
    }
}
