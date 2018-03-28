//! This library gives a function to calculate Fibonacci numbers.
//! Benchmarks require nightly Rust.
#![feature(test)]

/// Gives the Fibonacci sequence number for the given index.
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// Faster version, to check benchmarks.
// pub fn fibonacci(n: u32) -> u32 {
//     if n == 0 || n == 1 {
//         n
//     } else {
//         let mut previous = 1;
//         let mut current = 1;
//         for _ in 2..n {
//             let new_current = previous + current;
//             previous = current;
//             current = new_current;
//         }
//         current
//     }
// }

/// Tests module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the code gives the correct results.
    #[test]
    fn it_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6_765);
    }
}

/// Benchmarks module
#[cfg(test)]
mod benches {
    extern crate test;
    use super::*;
    use self::test::Bencher;

    /// Benchmark the 0th sequence number.
    #[bench]
    fn bench_fibonacci_0(b: &mut Bencher) {
        b.iter(|| (0..1).map(fibonacci).collect::<Vec<u32>>())
    }

    /// Benchmark the 1st sequence number.
    #[bench]
    fn bench_fibonacci_1(b: &mut Bencher) {
        b.iter(|| (0..2).map(fibonacci).collect::<Vec<u32>>())
    }

    /// Benchmark the 2nd sequence number.
    #[bench]
    fn bench_fibonacci_2(b: &mut Bencher) {
        b.iter(|| (0..3).map(fibonacci).collect::<Vec<u32>>())
    }

    /// Benchmark the 10th sequence number.
    #[bench]
    fn bench_fibonacci_10(b: &mut Bencher) {
        b.iter(|| (0..11).map(fibonacci).collect::<Vec<u32>>())
    }

    /// Benchmark the 20th sequence number.
    #[bench]
    fn bench_fibonacci_20(b: &mut Bencher) {
        b.iter(|| (0..21).map(fibonacci).collect::<Vec<u32>>())
    }
}
