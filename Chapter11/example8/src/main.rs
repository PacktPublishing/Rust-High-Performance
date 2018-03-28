#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;

use futures::prelude::*;
use futures::executor::block_on;

#[async]
fn retrieve_data_1() -> Result<i32, i32> {
    Ok(1)
}

#[async]
fn retrieve_data_2() -> Result<i32, i32> {
    Ok(2)
}

#[async_move]
fn add_data() -> Result<i32, i32> {
    Ok(await!(retrieve_data_1())? + await!(retrieve_data_2())?)
}

fn main() {
    println!("Result: {:?}", block_on(add_data()));
}
