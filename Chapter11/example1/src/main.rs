extern crate futures;

use futures::prelude::*;
use futures::future::{self, FutureResult};
use futures::executor::block_on;

fn main() {
    let final_result = some_complex_computation().map(|res| (res - 10) / 7);

    println!("Doing some other things while our result gets generated");

    match block_on(final_result) {
        Ok(res) => println!("The result is {}", res),
        Err(e) => println!("Error: {}", e),
    }
}

fn some_complex_computation() -> FutureResult<u32, String> {
    use std::thread;
    use std::time::Duration;

    thread::sleep(Duration::from_secs(5));

    future::ok(150)
}
