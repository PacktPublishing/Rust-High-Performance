//! Example benchmark.

#[macro_use]
extern crate criterion;
extern crate example3;

use criterion::Criterion;
use example3::fibonacci;

fn criterion_benchmark(c: &mut Criterion) {
    Criterion::default().bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
