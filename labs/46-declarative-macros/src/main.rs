//! # Declarative Macros - Demo

use declarative_macros::solution::{calculate_add, calculate_multiply, calculate_power};
use declarative_macros::{
    assert_between, calculate, count_tokens, fancy_sum, greet, hashmap, make_vec, sum, time_it,
};

fn main() {
    println!("=== Declarative Macros Demo ===");

    println!("{}", greet!("Rust"));
    println!("add: {}", calculate!(add 3, 4));
    println!("multiply: {}", calculate!(multiply 6, 7));
    println!("power: {}", calculate!(power 2, 8));

    println!("sum!: {}", sum!(1, 2, 3, 4, 5));
    println!("fancy_sum!: {}", fancy_sum!(10, 20, 30));

    let v = make_vec!["a", "b", "c"];
    println!("make_vec!: {:?}", v);

    let map = hashmap! {
        "alice" => 95,
        "bob" => 87,
    };
    println!("hashmap! size: {}", map.len());

    assert_between!(50, 0, 100);
    println!("count_tokens!: {}", count_tokens!(alpha beta gamma));

    let (result, elapsed) = time_it!({
        let mut total = 0;
        for i in 0..10_000 {
            total += i;
        }
        total
    });
    println!("time_it! result={} elapsed={:?}", result, elapsed);

    // Helper functions from the solution module.
    println!("helper add: {}", calculate_add(10, 20));
    println!("helper multiply: {}", calculate_multiply(6, 7));
    println!("helper power: {}", calculate_power(3.0, 4.0));
}
