extern crate rayon;

use rayon::prelude::*;

fn main() {
    let result = (0..1_000_000_u64)
        .into_par_iter()
        .map(|e| e * 2)
        .sum::<u64>();

    println!("Result: {}", result);
}
