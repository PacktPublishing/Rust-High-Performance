extern crate crossbeam;

use std::thread;
use std::sync::Arc;

use crossbeam::sync::MsQueue;

fn main() {
    let queue = Arc::new(MsQueue::new());

    let handles: Vec<_> = (1..6)
        .map(|_| {
            let t_queue = queue.clone();
            thread::spawn(move || {
                for _ in 0..1_000_000 {
                    t_queue.push(10);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let final_queue = Arc::try_unwrap(queue).unwrap();
    let mut sum = 0;
    while let Some(i) = final_queue.try_pop() {
        sum += i;
    }

    println!("Final sum: {}", sum);
}
