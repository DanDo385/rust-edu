use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parallel_processing::solution::*;

fn benchmark_sum_of_squares(c: &mut Criterion) {
    let numbers: Vec<i32> = (0..10_000).collect();

    let mut group = c.benchmark_group("Sum of Squares");

    group.bench_function("Sequential", |b| {
        b.iter(|| sum_of_squares_sequential(black_box(&numbers)))
    });

    group.bench_function("Parallel", |b| {
        b.iter(|| sum_of_squares_parallel(black_box(&numbers)))
    });

    group.finish();
}

fn benchmark_find_primes(c: &mut Criterion) {
    let limit = 20_000;

    let mut group = c.benchmark_group("Find Primes");

    group.bench_function("Sequential", |b| {
        b.iter(|| find_primes_sequential(black_box(limit)))
    });

    group.bench_function("Parallel", |b| {
        b.iter(|| find_primes_parallel(black_box(limit)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_sum_of_squares, benchmark_find_primes);
criterion_main!(benches);
