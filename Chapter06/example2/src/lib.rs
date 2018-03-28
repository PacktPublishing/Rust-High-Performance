//! This library gives a function to calculate Fibonacci numbers.

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
