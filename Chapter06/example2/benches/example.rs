//! Benchmarks

#[macro_use]
extern crate bencher;
extern crate example2;
use example2::*;
use self::bencher::Bencher;

/// Benchmark the 0th sequence number.
fn bench_fibonacci_0(b: &mut Bencher) {
    b.iter(|| (0..1).map(fibonacci).collect::<Vec<u32>>())
}

/// Benchmark the 1st sequence number.
fn bench_fibonacci_1(b: &mut Bencher) {
    b.iter(|| (0..2).map(fibonacci).collect::<Vec<u32>>())
}

/// Benchmark the 2nd sequence number.
fn bench_fibonacci_2(b: &mut Bencher) {
    b.iter(|| (0..3).map(fibonacci).collect::<Vec<u32>>())
}

/// Benchmark the 10th sequence number.
fn bench_fibonacci_10(b: &mut Bencher) {
    b.iter(|| (0..11).map(fibonacci).collect::<Vec<u32>>())
}

/// Benchmark the 20th sequence number.
fn bench_fibonacci_20(b: &mut Bencher) {
    b.iter(|| (0..21).map(fibonacci).collect::<Vec<u32>>())
}

benchmark_group!(
    benches,
    bench_fibonacci_0,
    bench_fibonacci_1,
    bench_fibonacci_2,
    bench_fibonacci_10,
    bench_fibonacci_20
);
benchmark_main!(benches);
