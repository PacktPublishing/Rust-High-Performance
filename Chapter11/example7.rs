#![feature(generators, generator_trait)]

use std::ops::Generator;

fn main() {
    let mut generator = || {
        println!("Before yield");
        yield;
        println!("After yield");
    };

    println!("Starting generator...");
    generator.resume();
    println!("Generator started");
    generator.resume();
    println!("Generator finished");
}
