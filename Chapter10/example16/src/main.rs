extern crate crossbeam;

use std::thread;
use std::sync::Arc;
use std::time::Duration;

use crossbeam::sync::{MsQueue, TreiberStack};

fn main() {
    let queue = Arc::new(MsQueue::new());
    let stack = Arc::new(TreiberStack::new());

    let in_queue = queue.clone();
    let in_stack = stack.clone();
    let in_handle = thread::spawn(move || {
        for i in 0..5 {
            in_queue.push(i);
            in_stack.push(i);
            println!("Pushed :D");
            thread::sleep(Duration::from_millis(50));
        }
    });

    let mut final_queue = Vec::new();
    let mut final_stack = Vec::new();

    let mut last_q_failed = 0;
    let mut last_s_failed = 0;

    loop {
        // Get the queue
        match queue.try_pop() {
            Some(i) => {
                final_queue.push(i);
                last_q_failed = 0;
                println!("Something in the queue! :)");
            }
            None => {
                println!("Nothing in the queue :(");
                last_q_failed += 1;
            }
        }

        // Get the stack
        match stack.try_pop() {
            Some(i) => {
                final_stack.push(i);
                last_s_failed = 0;
                println!("Something in the stack! :)");
            }
            None => {
                println!("Nothing in the stack :(");
                last_s_failed += 1;
            }
        }

        // Check if we finished
        if last_q_failed > 1 && last_s_failed > 1 {
            break;
        } else if last_q_failed > 0 || last_s_failed > 0 {
            thread::sleep(Duration::from_millis(100));
        }
    }

    in_handle.join().unwrap();

    println!("Queue: {:?}", final_queue);
    println!("Stack: {:?}", final_stack);
}
