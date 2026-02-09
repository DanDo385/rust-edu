//! # Lab 17: Closures and Iterators
//!
//! Student-facing API for closure traits (`Fn`, `FnMut`) and iterator composition.

pub fn apply_closure<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

pub fn apply_n_times<F: FnMut(i32) -> i32>(mut f: F, x: i32, n: usize) -> i32 {
    let mut result = x;
    for _ in 0..n {
        result = f(result);
    }
    result
}

pub fn filter_with_closure<F: Fn(i32) -> bool>(numbers: &[i32], predicate: F) -> Vec<i32> {
    numbers.iter().copied().filter(|&n| predicate(n)).collect()
}

pub fn transform_and_filter<F, G>(numbers: &[i32], transform: F, predicate: G) -> Vec<i32>
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> bool,
{
    numbers
        .iter()
        .map(|&n| transform(n))
        .filter(|&n| predicate(n))
        .collect()
}

pub fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| g(f(x))
}

#[derive(Clone)]
pub struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    pub fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn sum_with_fold(numbers: &[i32]) -> i32 {
    numbers.iter().fold(0, |acc, &n| acc + n)
}

pub fn product_of_matching<F: Fn(i32) -> bool>(numbers: &[i32], predicate: F) -> i32 {
    numbers
        .iter()
        .fold(1, |acc, &n| if predicate(n) { acc * n } else { acc })
}

pub fn demonstrate_lazy_evaluation(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&n| n > 2)
        .map(|&n| n * 2)
        .collect()
}

pub fn sum_evens_from_counter(max: u32) -> u32 {
    Counter::new(max)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum()
}

#[doc(hidden)]
pub mod solution;
