//! # Closures and Iterators Demo

use closures_iterators::solution::{self, Counter};

fn main() {
    println!("=== Closures and Iterators Demo ===\n");

    println!("apply_closure: {}", solution::apply_closure(|x| x + 1, 10));
    println!("apply_twice: {}", solution::apply_twice(|x| x * 2, 3));
    println!("apply_n_times: {}", solution::apply_n_times(|x| x + 3, 1, 4));

    let nums = [1, 2, 3, 4, 5, 6];
    println!("filter_with_closure: {:?}", solution::filter_with_closure(&nums, |n| n % 2 == 0));
    println!("transform_and_filter: {:?}", solution::transform_and_filter(&nums, |n| n * 2, |n| n > 6));

    let composed = solution::compose(|x| x + 1, |x| x * 10);
    println!("compose(7): {}", composed(7));

    let counter_vals: Vec<_> = Counter::new(5).collect();
    println!("counter values: {:?}", counter_vals);
    println!("sum_with_fold: {}", solution::sum_with_fold(&nums));
    println!("product_of_matching (>3): {}", solution::product_of_matching(&nums, |n| n > 3));
    println!("lazy eval demo: {:?}", solution::demonstrate_lazy_evaluation(&nums));
    println!("sum_evens_from_counter: {}", solution::sum_evens_from_counter(8));
}
