//! # Collections and Iterators Demo

use collections_iterators::solution;

fn main() {
    println!("=== Collections and Iterators Demo ===\n");

    let nums = [1, 2, 2, 3, 4, 6, 7];

    println!("sum_evens: {}", solution::sum_evens(&nums));
    println!("count > 3: {}", solution::count_matching(&nums, |n| n > 3));
    println!("all_positive: {}", solution::all_positive(&nums));
    println!("group_consecutive: {:?}", solution::group_consecutive(&nums));
    println!("find_first_even: {:?}", solution::find_first_even(&nums));
    println!("to_strings: {:?}", solution::to_strings(&nums));
    println!("find_max: {:?}", solution::find_max(&nums));
}
