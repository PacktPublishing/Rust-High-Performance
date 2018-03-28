// Note: requires nightly Rust.
// Use `cargo clippy` to get warnings.

#![feature(inclusive_range_syntax)]
#![cfg_attr(feature = "cargo-clippy", warn(range_plus_one))]

fn main() {
    // Should warn range plus one.
    let max = 10;
    for i in 0..max + 1 {
        println!("{}", i);
    }

    // Should not warn.
    let max = 10;
    for i in 0..=max {
        println!("{}", i);
    }
}
