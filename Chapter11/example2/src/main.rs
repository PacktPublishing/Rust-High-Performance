extern crate futures;
extern crate futures_timer;

use std::time::Duration;

use futures::prelude::*;
use futures_timer::Interval;
use futures::future::ok;

fn main() {
    Interval::new(Duration::from_secs(1))
        .take(5)
        .for_each(|_| {
            println!("New interval");
            ok(())
        })
        .wait()
        .unwrap();
}
